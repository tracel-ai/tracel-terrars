use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct MemcacheInstanceData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authorized_network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_protection: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memcache_version: Option<PrimField<String>>,
    name: PrimField<String>,
    node_count: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reserved_ip_range_id: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zones: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maintenance_policy: Option<Vec<MemcacheInstanceMaintenancePolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memcache_parameters: Option<Vec<MemcacheInstanceMemcacheParametersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_config: Option<Vec<MemcacheInstanceNodeConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<MemcacheInstanceTimeoutsEl>,
    dynamic: MemcacheInstanceDynamic,
}
struct MemcacheInstance_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<MemcacheInstanceData>,
}
#[derive(Clone)]
pub struct MemcacheInstance(Rc<MemcacheInstance_>);
impl MemcacheInstance {
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
    #[doc = "Set the field `authorized_network`.\nThe full name of the GCE network to connect the instance to.  If not provided,\n'default' will be used."]
    pub fn set_authorized_network(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().authorized_network = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_protection`.\nWhether Terraform will be prevented from destroying the instance.\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is not set to false in Terraform state.\nWhen the field is set to true or unset in Terraform state, a 'terraform apply'\nor 'terraform destroy' that would delete the instance will fail.\nWhen the field is set to false, deleting the instance is allowed."]
    pub fn set_deletion_protection(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().deletion_protection = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\nA user-visible name for the instance."]
    pub fn set_display_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nResource labels to represent user-provided metadata.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `memcache_version`.\nThe major version of Memcached software. If not provided, latest supported version will be used.\nCurrently the latest supported major version is MEMCACHE_1_5. The minor version will be automatically\ndetermined by our system based on the latest supported minor version. Default value: \"MEMCACHE_1_5\" Possible values: [\"MEMCACHE_1_5\", \"MEMCACHE_1_6_15\"]"]
    pub fn set_memcache_version(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().memcache_version = Some(v.into());
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
    #[doc = "Set the field `reserved_ip_range_id`.\nContains the name of allocated IP address ranges associated with\nthe private service access connection for example, \"test-default\"\nassociated with IP range 10.0.0.0/29."]
    pub fn set_reserved_ip_range_id(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().reserved_ip_range_id = Some(v.into());
        self
    }
    #[doc = "Set the field `zones`.\nZones where memcache nodes should be provisioned.  If not\nprovided, all zones will be used."]
    pub fn set_zones(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().zones = Some(v.into());
        self
    }
    #[doc = "Set the field `maintenance_policy`.\n"]
    pub fn set_maintenance_policy(
        self,
        v: impl Into<BlockAssignable<MemcacheInstanceMaintenancePolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().maintenance_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.maintenance_policy = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `memcache_parameters`.\n"]
    pub fn set_memcache_parameters(
        self,
        v: impl Into<BlockAssignable<MemcacheInstanceMemcacheParametersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().memcache_parameters = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.memcache_parameters = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `node_config`.\n"]
    pub fn set_node_config(
        self,
        v: impl Into<BlockAssignable<MemcacheInstanceNodeConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().node_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.node_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<MemcacheInstanceTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
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
    #[doc = "Get a reference to the value of field `maintenance_schedule` after provisioning.\nOutput only. Published maintenance schedule."]
    pub fn maintenance_schedule(&self) -> ListRef<MemcacheInstanceMaintenanceScheduleElRef> {
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
    pub fn memcache_nodes(&self) -> ListRef<MemcacheInstanceMemcacheNodesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.memcache_nodes", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `maintenance_policy` after provisioning.\n"]
    pub fn maintenance_policy(&self) -> ListRef<MemcacheInstanceMaintenancePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.maintenance_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `memcache_parameters` after provisioning.\n"]
    pub fn memcache_parameters(&self) -> ListRef<MemcacheInstanceMemcacheParametersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.memcache_parameters", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `node_config` after provisioning.\n"]
    pub fn node_config(&self) -> ListRef<MemcacheInstanceNodeConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.node_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MemcacheInstanceTimeoutsElRef {
        MemcacheInstanceTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for MemcacheInstance {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for MemcacheInstance {}
impl ToListMappable for MemcacheInstance {
    type O = ListRef<MemcacheInstanceRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for MemcacheInstance_ {
    fn extract_resource_type(&self) -> String {
        "google_memcache_instance".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildMemcacheInstance {
    pub tf_id: String,
    #[doc = "The resource name of the instance."]
    pub name: PrimField<String>,
    #[doc = "Number of nodes in the memcache instance."]
    pub node_count: PrimField<f64>,
}
impl BuildMemcacheInstance {
    pub fn build(self, stack: &mut Stack) -> MemcacheInstance {
        let out = MemcacheInstance(Rc::new(MemcacheInstance_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(MemcacheInstanceData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                authorized_network: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                deletion_protection: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                memcache_version: core::default::Default::default(),
                name: self.name,
                node_count: self.node_count,
                project: core::default::Default::default(),
                region: core::default::Default::default(),
                reserved_ip_range_id: core::default::Default::default(),
                zones: core::default::Default::default(),
                maintenance_policy: core::default::Default::default(),
                memcache_parameters: core::default::Default::default(),
                node_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct MemcacheInstanceRef {
    shared: StackShared,
    base: String,
}
impl Ref for MemcacheInstanceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl MemcacheInstanceRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
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
    #[doc = "Get a reference to the value of field `maintenance_schedule` after provisioning.\nOutput only. Published maintenance schedule."]
    pub fn maintenance_schedule(&self) -> ListRef<MemcacheInstanceMaintenanceScheduleElRef> {
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
    pub fn memcache_nodes(&self) -> ListRef<MemcacheInstanceMemcacheNodesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.memcache_nodes", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `maintenance_policy` after provisioning.\n"]
    pub fn maintenance_policy(&self) -> ListRef<MemcacheInstanceMaintenancePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.maintenance_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `memcache_parameters` after provisioning.\n"]
    pub fn memcache_parameters(&self) -> ListRef<MemcacheInstanceMemcacheParametersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.memcache_parameters", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `node_config` after provisioning.\n"]
    pub fn node_config(&self) -> ListRef<MemcacheInstanceNodeConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.node_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MemcacheInstanceTimeoutsElRef {
        MemcacheInstanceTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct MemcacheInstanceMaintenanceScheduleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    end_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schedule_deadline_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time: Option<PrimField<String>>,
}
impl MemcacheInstanceMaintenanceScheduleEl {
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
impl ToListMappable for MemcacheInstanceMaintenanceScheduleEl {
    type O = BlockAssignable<MemcacheInstanceMaintenanceScheduleEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMemcacheInstanceMaintenanceScheduleEl {}
impl BuildMemcacheInstanceMaintenanceScheduleEl {
    pub fn build(self) -> MemcacheInstanceMaintenanceScheduleEl {
        MemcacheInstanceMaintenanceScheduleEl {
            end_time: core::default::Default::default(),
            schedule_deadline_time: core::default::Default::default(),
            start_time: core::default::Default::default(),
        }
    }
}
pub struct MemcacheInstanceMaintenanceScheduleElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MemcacheInstanceMaintenanceScheduleElRef {
    fn new(shared: StackShared, base: String) -> MemcacheInstanceMaintenanceScheduleElRef {
        MemcacheInstanceMaintenanceScheduleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MemcacheInstanceMaintenanceScheduleElRef {
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
pub struct MemcacheInstanceMemcacheNodesEl {
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
impl MemcacheInstanceMemcacheNodesEl {
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
impl ToListMappable for MemcacheInstanceMemcacheNodesEl {
    type O = BlockAssignable<MemcacheInstanceMemcacheNodesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMemcacheInstanceMemcacheNodesEl {}
impl BuildMemcacheInstanceMemcacheNodesEl {
    pub fn build(self) -> MemcacheInstanceMemcacheNodesEl {
        MemcacheInstanceMemcacheNodesEl {
            host: core::default::Default::default(),
            node_id: core::default::Default::default(),
            port: core::default::Default::default(),
            state: core::default::Default::default(),
            zone: core::default::Default::default(),
        }
    }
}
pub struct MemcacheInstanceMemcacheNodesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MemcacheInstanceMemcacheNodesElRef {
    fn new(shared: StackShared, base: String) -> MemcacheInstanceMemcacheNodesElRef {
        MemcacheInstanceMemcacheNodesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MemcacheInstanceMemcacheNodesElRef {
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
pub struct MemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    hours: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minutes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nanos: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    seconds: Option<PrimField<f64>>,
}
impl MemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl {
    #[doc = "Set the field `hours`.\nHours of day in 24 hour format. Should be from 0 to 23.\nAn API may choose to allow the value \"24:00:00\" for scenarios like business closing time."]
    pub fn set_hours(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.hours = Some(v.into());
        self
    }
    #[doc = "Set the field `minutes`.\nMinutes of hour of day. Must be from 0 to 59."]
    pub fn set_minutes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.minutes = Some(v.into());
        self
    }
    #[doc = "Set the field `nanos`.\nFractions of seconds in nanoseconds. Must be from 0 to 999,999,999."]
    pub fn set_nanos(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.nanos = Some(v.into());
        self
    }
    #[doc = "Set the field `seconds`.\nSeconds of minutes of the time. Must normally be from 0 to 59.\nAn API may allow the value 60 if it allows leap-seconds."]
    pub fn set_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.seconds = Some(v.into());
        self
    }
}
impl ToListMappable for MemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl {
    type O =
        BlockAssignable<MemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl {}
impl BuildMemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl {
    pub fn build(self) -> MemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl {
        MemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl {
            hours: core::default::Default::default(),
            minutes: core::default::Default::default(),
            nanos: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}
pub struct MemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeElRef {
        MemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `hours` after provisioning.\nHours of day in 24 hour format. Should be from 0 to 23.\nAn API may choose to allow the value \"24:00:00\" for scenarios like business closing time."]
    pub fn hours(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.hours", self.base))
    }
    #[doc = "Get a reference to the value of field `minutes` after provisioning.\nMinutes of hour of day. Must be from 0 to 59."]
    pub fn minutes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.minutes", self.base))
    }
    #[doc = "Get a reference to the value of field `nanos` after provisioning.\nFractions of seconds in nanoseconds. Must be from 0 to 999,999,999."]
    pub fn nanos(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }
    #[doc = "Get a reference to the value of field `seconds` after provisioning.\nSeconds of minutes of the time. Must normally be from 0 to 59.\nAn API may allow the value 60 if it allows leap-seconds."]
    pub fn seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}
#[derive(Serialize, Default)]
struct MemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowElDynamic {
    start_time: Option<
        DynamicBlock<MemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl>,
    >,
}
#[derive(Serialize)]
pub struct MemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowEl {
    day: PrimField<String>,
    duration: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time:
        Option<Vec<MemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl>>,
    dynamic: MemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowElDynamic,
}
impl MemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowEl {
    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(
        mut self,
        v: impl Into<
            BlockAssignable<
                MemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.start_time = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.start_time = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for MemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowEl {
    type O = BlockAssignable<MemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowEl {
    #[doc = "Required. The day of week that maintenance updates occur.\n- DAY_OF_WEEK_UNSPECIFIED: The day of the week is unspecified.\n- MONDAY: Monday\n- TUESDAY: Tuesday\n- WEDNESDAY: Wednesday\n- THURSDAY: Thursday\n- FRIDAY: Friday\n- SATURDAY: Saturday\n- SUNDAY: Sunday Possible values: [\"DAY_OF_WEEK_UNSPECIFIED\", \"MONDAY\", \"TUESDAY\", \"WEDNESDAY\", \"THURSDAY\", \"FRIDAY\", \"SATURDAY\", \"SUNDAY\"]"]
    pub day: PrimField<String>,
    #[doc = "Required. The length of the maintenance window, ranging from 3 hours to 8 hours.\nA duration in seconds with up to nine fractional digits,\nterminated by 's'. Example: \"3.5s\"."]
    pub duration: PrimField<String>,
}
impl BuildMemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowEl {
    pub fn build(self) -> MemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowEl {
        MemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowEl {
            day: self.day,
            duration: self.duration,
            start_time: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct MemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowElRef {
        MemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `day` after provisioning.\nRequired. The day of week that maintenance updates occur.\n- DAY_OF_WEEK_UNSPECIFIED: The day of the week is unspecified.\n- MONDAY: Monday\n- TUESDAY: Tuesday\n- WEDNESDAY: Wednesday\n- THURSDAY: Thursday\n- FRIDAY: Friday\n- SATURDAY: Saturday\n- SUNDAY: Sunday Possible values: [\"DAY_OF_WEEK_UNSPECIFIED\", \"MONDAY\", \"TUESDAY\", \"WEDNESDAY\", \"THURSDAY\", \"FRIDAY\", \"SATURDAY\", \"SUNDAY\"]"]
    pub fn day(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.day", self.base))
    }
    #[doc = "Get a reference to the value of field `duration` after provisioning.\nRequired. The length of the maintenance window, ranging from 3 hours to 8 hours.\nA duration in seconds with up to nine fractional digits,\nterminated by 's'. Example: \"3.5s\"."]
    pub fn duration(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.duration", self.base))
    }
    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(
        &self,
    ) -> ListRef<MemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}
#[derive(Serialize, Default)]
struct MemcacheInstanceMaintenancePolicyElDynamic {
    weekly_maintenance_window:
        Option<DynamicBlock<MemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowEl>>,
}
#[derive(Serialize)]
pub struct MemcacheInstanceMaintenancePolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weekly_maintenance_window:
        Option<Vec<MemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowEl>>,
    dynamic: MemcacheInstanceMaintenancePolicyElDynamic,
}
impl MemcacheInstanceMaintenancePolicyEl {
    #[doc = "Set the field `description`.\nOptional. Description of what this policy is for.\nCreate/Update methods return INVALID_ARGUMENT if the\nlength is greater than 512."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `weekly_maintenance_window`.\n"]
    pub fn set_weekly_maintenance_window(
        mut self,
        v: impl Into<BlockAssignable<MemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.weekly_maintenance_window = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.weekly_maintenance_window = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for MemcacheInstanceMaintenancePolicyEl {
    type O = BlockAssignable<MemcacheInstanceMaintenancePolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMemcacheInstanceMaintenancePolicyEl {}
impl BuildMemcacheInstanceMaintenancePolicyEl {
    pub fn build(self) -> MemcacheInstanceMaintenancePolicyEl {
        MemcacheInstanceMaintenancePolicyEl {
            description: core::default::Default::default(),
            weekly_maintenance_window: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct MemcacheInstanceMaintenancePolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MemcacheInstanceMaintenancePolicyElRef {
    fn new(shared: StackShared, base: String) -> MemcacheInstanceMaintenancePolicyElRef {
        MemcacheInstanceMaintenancePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MemcacheInstanceMaintenancePolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The time when the policy was created.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond\nresolution and up to nine fractional digits"]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.base))
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nOptional. Description of what this policy is for.\nCreate/Update methods return INVALID_ARGUMENT if the\nlength is greater than 512."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The time when the policy was updated.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond\nresolution and up to nine fractional digits."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }
    #[doc = "Get a reference to the value of field `weekly_maintenance_window` after provisioning.\n"]
    pub fn weekly_maintenance_window(
        &self,
    ) -> ListRef<MemcacheInstanceMaintenancePolicyElWeeklyMaintenanceWindowElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.weekly_maintenance_window", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct MemcacheInstanceMemcacheParametersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    params: Option<RecField<PrimField<String>>>,
}
impl MemcacheInstanceMemcacheParametersEl {
    #[doc = "Set the field `params`.\nUser-defined set of parameters to use in the memcache process."]
    pub fn set_params(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.params = Some(v.into());
        self
    }
}
impl ToListMappable for MemcacheInstanceMemcacheParametersEl {
    type O = BlockAssignable<MemcacheInstanceMemcacheParametersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMemcacheInstanceMemcacheParametersEl {}
impl BuildMemcacheInstanceMemcacheParametersEl {
    pub fn build(self) -> MemcacheInstanceMemcacheParametersEl {
        MemcacheInstanceMemcacheParametersEl {
            params: core::default::Default::default(),
        }
    }
}
pub struct MemcacheInstanceMemcacheParametersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MemcacheInstanceMemcacheParametersElRef {
    fn new(shared: StackShared, base: String) -> MemcacheInstanceMemcacheParametersElRef {
        MemcacheInstanceMemcacheParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MemcacheInstanceMemcacheParametersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\nThis is a unique ID associated with this set of parameters."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
    #[doc = "Get a reference to the value of field `params` after provisioning.\nUser-defined set of parameters to use in the memcache process."]
    pub fn params(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.params", self.base))
    }
}
#[derive(Serialize)]
pub struct MemcacheInstanceNodeConfigEl {
    cpu_count: PrimField<f64>,
    memory_size_mb: PrimField<f64>,
}
impl MemcacheInstanceNodeConfigEl {}
impl ToListMappable for MemcacheInstanceNodeConfigEl {
    type O = BlockAssignable<MemcacheInstanceNodeConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMemcacheInstanceNodeConfigEl {
    #[doc = "Number of CPUs per node."]
    pub cpu_count: PrimField<f64>,
    #[doc = "Memory size in Mebibytes for each memcache node."]
    pub memory_size_mb: PrimField<f64>,
}
impl BuildMemcacheInstanceNodeConfigEl {
    pub fn build(self) -> MemcacheInstanceNodeConfigEl {
        MemcacheInstanceNodeConfigEl {
            cpu_count: self.cpu_count,
            memory_size_mb: self.memory_size_mb,
        }
    }
}
pub struct MemcacheInstanceNodeConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MemcacheInstanceNodeConfigElRef {
    fn new(shared: StackShared, base: String) -> MemcacheInstanceNodeConfigElRef {
        MemcacheInstanceNodeConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MemcacheInstanceNodeConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cpu_count` after provisioning.\nNumber of CPUs per node."]
    pub fn cpu_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.cpu_count", self.base))
    }
    #[doc = "Get a reference to the value of field `memory_size_mb` after provisioning.\nMemory size in Mebibytes for each memcache node."]
    pub fn memory_size_mb(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.memory_size_mb", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct MemcacheInstanceTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl MemcacheInstanceTimeoutsEl {
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
impl ToListMappable for MemcacheInstanceTimeoutsEl {
    type O = BlockAssignable<MemcacheInstanceTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMemcacheInstanceTimeoutsEl {}
impl BuildMemcacheInstanceTimeoutsEl {
    pub fn build(self) -> MemcacheInstanceTimeoutsEl {
        MemcacheInstanceTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct MemcacheInstanceTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MemcacheInstanceTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> MemcacheInstanceTimeoutsElRef {
        MemcacheInstanceTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MemcacheInstanceTimeoutsElRef {
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
struct MemcacheInstanceDynamic {
    maintenance_policy: Option<DynamicBlock<MemcacheInstanceMaintenancePolicyEl>>,
    memcache_parameters: Option<DynamicBlock<MemcacheInstanceMemcacheParametersEl>>,
    node_config: Option<DynamicBlock<MemcacheInstanceNodeConfigEl>>,
}
