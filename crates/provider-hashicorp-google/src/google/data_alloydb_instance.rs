use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataAlloydbInstanceData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    cluster_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    instance_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
}
struct DataAlloydbInstance_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataAlloydbInstanceData>,
}
#[derive(Clone)]
pub struct DataAlloydbInstance(Rc<DataAlloydbInstance_>);
impl DataAlloydbInstance {
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
    #[doc = "Set the field `location`.\nThe canonical ID for the location. For example: \"us-east1\"."]
    pub fn set_location(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().location = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\nProject ID of the project."]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `activation_policy` after provisioning.\n'Specifies whether an instance needs to spin up. Once the instance is\nactive, the activation policy can be updated to the 'NEVER' to stop the\ninstance. Likewise, the activation policy can be updated to 'ALWAYS' to\nstart the instance.\nThere are restrictions around when an instance can/cannot be activated (for\nexample, a read pool instance should be stopped before stopping primary\netc.). Please refer to the API documentation for more details.\nPossible values are: 'ACTIVATION_POLICY_UNSPECIFIED', 'ALWAYS', 'NEVER'.' Possible values: [\"ACTIVATION_POLICY_UNSPECIFIED\", \"ALWAYS\", \"NEVER\"]"]
    pub fn activation_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.activation_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `annotations` after provisioning.\nAnnotations to allow client tools to store small amount of arbitrary data. This is distinct from labels.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `availability_type` after provisioning.\n'Availability type of an Instance. Defaults to REGIONAL for both primary and read instances.\nNote that primary and read instances can have different availability types.\nPrimary instances can be either ZONAL or REGIONAL. Read Pool instances can also be either ZONAL or REGIONAL.\nRead pools of size 1 can only have zonal availability. Read pools with a node count of 2 or more\ncan have regional availability (nodes are present in 2 or more zones in a region).\nPossible values are: 'AVAILABILITY_TYPE_UNSPECIFIED', 'ZONAL', 'REGIONAL'.' Possible values: [\"AVAILABILITY_TYPE_UNSPECIFIED\", \"ZONAL\", \"REGIONAL\"]"]
    pub fn availability_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.availability_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `client_connection_config` after provisioning.\nClient connection specific configurations."]
    pub fn client_connection_config(
        &self,
    ) -> ListRef<DataAlloydbInstanceClientConnectionConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.client_connection_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cluster` after provisioning.\nIdentifies the alloydb cluster. Must be in the format\n'projects/{project}/locations/{location}/clusters/{cluster_id}'"]
    pub fn cluster(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cluster_id` after provisioning.\nThe ID of the alloydb cluster that the instance belongs to.'alloydb_cluster_id'"]
    pub fn cluster_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `connection_pool_config` after provisioning.\nConfiguration for Managed Connection Pool."]
    pub fn connection_pool_config(&self) -> ListRef<DataAlloydbInstanceConnectionPoolConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.connection_pool_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTime the Instance was created in UTC."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `database_flags` after provisioning.\nDatabase flags. Set at instance level. * They are copied from primary instance on read instance creation. * Read instances can set new or override existing flags that are relevant for reads, e.g. for enabling columnar cache on a read instance. Flags set on read instance may or may not be present on primary."]
    pub fn database_flags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.database_flags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nUser-settable and human-readable display name for the Instance."]
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
    #[doc = "Get a reference to the value of field `gce_zone` after provisioning.\nThe Compute Engine zone that the instance should serve from, per https://cloud.google.com/compute/docs/regions-zones This can ONLY be specified for ZONAL instances. If present for a REGIONAL instance, an error will be thrown. If this is absent for a ZONAL instance, instance is created in a random zone with available capacity."]
    pub fn gce_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gce_zone", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `instance_id` after provisioning.\nThe ID of the alloydb instance."]
    pub fn instance_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\nThe type of the instance.\nIf the instance type is READ_POOL, provide the associated PRIMARY/SECONDARY instance in the 'depends_on' meta-data attribute.\nIf the instance type is SECONDARY, point to the cluster_type of the associated secondary cluster instead of mentioning SECONDARY.\nExample: {instance_type = google_alloydb_cluster.<secondary_cluster_name>.cluster_type} instead of {instance_type = SECONDARY}\nIf the instance type is SECONDARY, the terraform delete instance operation does not delete the secondary instance but abandons it instead.\nUse deletion_policy = \"FORCE\" in the associated secondary cluster and delete the cluster forcefully to delete the secondary cluster as well its associated secondary instance.\nUsers can undo the delete secondary instance action by importing the deleted secondary instance by calling terraform import. Possible values: [\"PRIMARY\", \"READ_POOL\", \"SECONDARY\"]"]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ip_address` after provisioning.\nThe IP address for the Instance. This is the connection endpoint for an end-user application."]
    pub fn ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ip_address", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nUser-defined labels for the alloydb instance.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe canonical ID for the location. For example: \"us-east1\"."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `machine_config` after provisioning.\nConfigurations for the machines that host the underlying database engine."]
    pub fn machine_config(&self) -> ListRef<DataAlloydbInstanceMachineConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.machine_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the instance resource."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network_config` after provisioning.\nInstance level network configuration."]
    pub fn network_config(&self) -> ListRef<DataAlloydbInstanceNetworkConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `outbound_public_ip_addresses` after provisioning.\nThe outbound public IP addresses for the instance. This is available ONLY when\nnetworkConfig.enableOutboundPublicIp is set to true. These IP addresses are used\nfor outbound connections."]
    pub fn outbound_public_ip_addresses(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.outbound_public_ip_addresses", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nProject ID of the project."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `psc_instance_config` after provisioning.\nConfiguration for Private Service Connect (PSC) for the instance."]
    pub fn psc_instance_config(&self) -> ListRef<DataAlloydbInstancePscInstanceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.psc_instance_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `public_ip_address` after provisioning.\nThe public IP addresses for the Instance. This is available ONLY when\nnetworkConfig.enablePublicIp is set to true. This is the connection\nendpoint for an end-user application."]
    pub fn public_ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.public_ip_address", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `query_insights_config` after provisioning.\nConfiguration for query insights."]
    pub fn query_insights_config(&self) -> ListRef<DataAlloydbInstanceQueryInsightsConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.query_insights_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `read_pool_config` after provisioning.\nRead pool specific config. If the instance type is READ_POOL, this configuration must be provided."]
    pub fn read_pool_config(&self) -> ListRef<DataAlloydbInstanceReadPoolConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.read_pool_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reconciling` after provisioning.\nSet to true if the current state of Instance does not match the user's intended state, and the service is actively updating the resource to reconcile them. This can happen due to user-triggered updates or system actions like failover or maintenance."]
    pub fn reconciling(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reconciling", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe current state of the alloydb instance."]
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
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nThe system-generated UID of the resource."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nTime the Instance was updated in UTC."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
}
impl Referable for DataAlloydbInstance {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataAlloydbInstance {}
impl ToListMappable for DataAlloydbInstance {
    type O = ListRef<DataAlloydbInstanceRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataAlloydbInstance_ {
    fn extract_datasource_type(&self) -> String {
        "google_alloydb_instance".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataAlloydbInstance {
    pub tf_id: String,
    #[doc = "The ID of the alloydb cluster that the instance belongs to.'alloydb_cluster_id'"]
    pub cluster_id: PrimField<String>,
    #[doc = "The ID of the alloydb instance."]
    pub instance_id: PrimField<String>,
}
impl BuildDataAlloydbInstance {
    pub fn build(self, stack: &mut Stack) -> DataAlloydbInstance {
        let out = DataAlloydbInstance(Rc::new(DataAlloydbInstance_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataAlloydbInstanceData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                cluster_id: self.cluster_id,
                id: core::default::Default::default(),
                instance_id: self.instance_id,
                location: core::default::Default::default(),
                project: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataAlloydbInstanceRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAlloydbInstanceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataAlloydbInstanceRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `activation_policy` after provisioning.\n'Specifies whether an instance needs to spin up. Once the instance is\nactive, the activation policy can be updated to the 'NEVER' to stop the\ninstance. Likewise, the activation policy can be updated to 'ALWAYS' to\nstart the instance.\nThere are restrictions around when an instance can/cannot be activated (for\nexample, a read pool instance should be stopped before stopping primary\netc.). Please refer to the API documentation for more details.\nPossible values are: 'ACTIVATION_POLICY_UNSPECIFIED', 'ALWAYS', 'NEVER'.' Possible values: [\"ACTIVATION_POLICY_UNSPECIFIED\", \"ALWAYS\", \"NEVER\"]"]
    pub fn activation_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.activation_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `annotations` after provisioning.\nAnnotations to allow client tools to store small amount of arbitrary data. This is distinct from labels.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `availability_type` after provisioning.\n'Availability type of an Instance. Defaults to REGIONAL for both primary and read instances.\nNote that primary and read instances can have different availability types.\nPrimary instances can be either ZONAL or REGIONAL. Read Pool instances can also be either ZONAL or REGIONAL.\nRead pools of size 1 can only have zonal availability. Read pools with a node count of 2 or more\ncan have regional availability (nodes are present in 2 or more zones in a region).\nPossible values are: 'AVAILABILITY_TYPE_UNSPECIFIED', 'ZONAL', 'REGIONAL'.' Possible values: [\"AVAILABILITY_TYPE_UNSPECIFIED\", \"ZONAL\", \"REGIONAL\"]"]
    pub fn availability_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.availability_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `client_connection_config` after provisioning.\nClient connection specific configurations."]
    pub fn client_connection_config(
        &self,
    ) -> ListRef<DataAlloydbInstanceClientConnectionConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.client_connection_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cluster` after provisioning.\nIdentifies the alloydb cluster. Must be in the format\n'projects/{project}/locations/{location}/clusters/{cluster_id}'"]
    pub fn cluster(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cluster_id` after provisioning.\nThe ID of the alloydb cluster that the instance belongs to.'alloydb_cluster_id'"]
    pub fn cluster_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `connection_pool_config` after provisioning.\nConfiguration for Managed Connection Pool."]
    pub fn connection_pool_config(&self) -> ListRef<DataAlloydbInstanceConnectionPoolConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.connection_pool_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTime the Instance was created in UTC."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `database_flags` after provisioning.\nDatabase flags. Set at instance level. * They are copied from primary instance on read instance creation. * Read instances can set new or override existing flags that are relevant for reads, e.g. for enabling columnar cache on a read instance. Flags set on read instance may or may not be present on primary."]
    pub fn database_flags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.database_flags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nUser-settable and human-readable display name for the Instance."]
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
    #[doc = "Get a reference to the value of field `gce_zone` after provisioning.\nThe Compute Engine zone that the instance should serve from, per https://cloud.google.com/compute/docs/regions-zones This can ONLY be specified for ZONAL instances. If present for a REGIONAL instance, an error will be thrown. If this is absent for a ZONAL instance, instance is created in a random zone with available capacity."]
    pub fn gce_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gce_zone", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `instance_id` after provisioning.\nThe ID of the alloydb instance."]
    pub fn instance_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\nThe type of the instance.\nIf the instance type is READ_POOL, provide the associated PRIMARY/SECONDARY instance in the 'depends_on' meta-data attribute.\nIf the instance type is SECONDARY, point to the cluster_type of the associated secondary cluster instead of mentioning SECONDARY.\nExample: {instance_type = google_alloydb_cluster.<secondary_cluster_name>.cluster_type} instead of {instance_type = SECONDARY}\nIf the instance type is SECONDARY, the terraform delete instance operation does not delete the secondary instance but abandons it instead.\nUse deletion_policy = \"FORCE\" in the associated secondary cluster and delete the cluster forcefully to delete the secondary cluster as well its associated secondary instance.\nUsers can undo the delete secondary instance action by importing the deleted secondary instance by calling terraform import. Possible values: [\"PRIMARY\", \"READ_POOL\", \"SECONDARY\"]"]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ip_address` after provisioning.\nThe IP address for the Instance. This is the connection endpoint for an end-user application."]
    pub fn ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ip_address", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nUser-defined labels for the alloydb instance.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe canonical ID for the location. For example: \"us-east1\"."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `machine_config` after provisioning.\nConfigurations for the machines that host the underlying database engine."]
    pub fn machine_config(&self) -> ListRef<DataAlloydbInstanceMachineConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.machine_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the instance resource."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network_config` after provisioning.\nInstance level network configuration."]
    pub fn network_config(&self) -> ListRef<DataAlloydbInstanceNetworkConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `outbound_public_ip_addresses` after provisioning.\nThe outbound public IP addresses for the instance. This is available ONLY when\nnetworkConfig.enableOutboundPublicIp is set to true. These IP addresses are used\nfor outbound connections."]
    pub fn outbound_public_ip_addresses(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.outbound_public_ip_addresses", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nProject ID of the project."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `psc_instance_config` after provisioning.\nConfiguration for Private Service Connect (PSC) for the instance."]
    pub fn psc_instance_config(&self) -> ListRef<DataAlloydbInstancePscInstanceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.psc_instance_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `public_ip_address` after provisioning.\nThe public IP addresses for the Instance. This is available ONLY when\nnetworkConfig.enablePublicIp is set to true. This is the connection\nendpoint for an end-user application."]
    pub fn public_ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.public_ip_address", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `query_insights_config` after provisioning.\nConfiguration for query insights."]
    pub fn query_insights_config(&self) -> ListRef<DataAlloydbInstanceQueryInsightsConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.query_insights_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `read_pool_config` after provisioning.\nRead pool specific config. If the instance type is READ_POOL, this configuration must be provided."]
    pub fn read_pool_config(&self) -> ListRef<DataAlloydbInstanceReadPoolConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.read_pool_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reconciling` after provisioning.\nSet to true if the current state of Instance does not match the user's intended state, and the service is actively updating the resource to reconcile them. This can happen due to user-triggered updates or system actions like failover or maintenance."]
    pub fn reconciling(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reconciling", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe current state of the alloydb instance."]
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
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nThe system-generated UID of the resource."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nTime the Instance was updated in UTC."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataAlloydbInstanceClientConnectionConfigElSslConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ssl_mode: Option<PrimField<String>>,
}
impl DataAlloydbInstanceClientConnectionConfigElSslConfigEl {
    #[doc = "Set the field `ssl_mode`.\n"]
    pub fn set_ssl_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ssl_mode = Some(v.into());
        self
    }
}
impl ToListMappable for DataAlloydbInstanceClientConnectionConfigElSslConfigEl {
    type O = BlockAssignable<DataAlloydbInstanceClientConnectionConfigElSslConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataAlloydbInstanceClientConnectionConfigElSslConfigEl {}
impl BuildDataAlloydbInstanceClientConnectionConfigElSslConfigEl {
    pub fn build(self) -> DataAlloydbInstanceClientConnectionConfigElSslConfigEl {
        DataAlloydbInstanceClientConnectionConfigElSslConfigEl {
            ssl_mode: core::default::Default::default(),
        }
    }
}
pub struct DataAlloydbInstanceClientConnectionConfigElSslConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAlloydbInstanceClientConnectionConfigElSslConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAlloydbInstanceClientConnectionConfigElSslConfigElRef {
        DataAlloydbInstanceClientConnectionConfigElSslConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataAlloydbInstanceClientConnectionConfigElSslConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ssl_mode` after provisioning.\n"]
    pub fn ssl_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ssl_mode", self.base))
    }
}
#[derive(Serialize)]
pub struct DataAlloydbInstanceClientConnectionConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    require_connectors: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ssl_config: Option<ListField<DataAlloydbInstanceClientConnectionConfigElSslConfigEl>>,
}
impl DataAlloydbInstanceClientConnectionConfigEl {
    #[doc = "Set the field `require_connectors`.\n"]
    pub fn set_require_connectors(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.require_connectors = Some(v.into());
        self
    }
    #[doc = "Set the field `ssl_config`.\n"]
    pub fn set_ssl_config(
        mut self,
        v: impl Into<ListField<DataAlloydbInstanceClientConnectionConfigElSslConfigEl>>,
    ) -> Self {
        self.ssl_config = Some(v.into());
        self
    }
}
impl ToListMappable for DataAlloydbInstanceClientConnectionConfigEl {
    type O = BlockAssignable<DataAlloydbInstanceClientConnectionConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataAlloydbInstanceClientConnectionConfigEl {}
impl BuildDataAlloydbInstanceClientConnectionConfigEl {
    pub fn build(self) -> DataAlloydbInstanceClientConnectionConfigEl {
        DataAlloydbInstanceClientConnectionConfigEl {
            require_connectors: core::default::Default::default(),
            ssl_config: core::default::Default::default(),
        }
    }
}
pub struct DataAlloydbInstanceClientConnectionConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAlloydbInstanceClientConnectionConfigElRef {
    fn new(shared: StackShared, base: String) -> DataAlloydbInstanceClientConnectionConfigElRef {
        DataAlloydbInstanceClientConnectionConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataAlloydbInstanceClientConnectionConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `require_connectors` after provisioning.\n"]
    pub fn require_connectors(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.require_connectors", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ssl_config` after provisioning.\n"]
    pub fn ssl_config(&self) -> ListRef<DataAlloydbInstanceClientConnectionConfigElSslConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ssl_config", self.base))
    }
}
#[derive(Serialize)]
pub struct DataAlloydbInstanceConnectionPoolConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    flags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pooler_count: Option<PrimField<f64>>,
}
impl DataAlloydbInstanceConnectionPoolConfigEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `flags`.\n"]
    pub fn set_flags(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.flags = Some(v.into());
        self
    }
    #[doc = "Set the field `pooler_count`.\n"]
    pub fn set_pooler_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.pooler_count = Some(v.into());
        self
    }
}
impl ToListMappable for DataAlloydbInstanceConnectionPoolConfigEl {
    type O = BlockAssignable<DataAlloydbInstanceConnectionPoolConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataAlloydbInstanceConnectionPoolConfigEl {}
impl BuildDataAlloydbInstanceConnectionPoolConfigEl {
    pub fn build(self) -> DataAlloydbInstanceConnectionPoolConfigEl {
        DataAlloydbInstanceConnectionPoolConfigEl {
            enabled: core::default::Default::default(),
            flags: core::default::Default::default(),
            pooler_count: core::default::Default::default(),
        }
    }
}
pub struct DataAlloydbInstanceConnectionPoolConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAlloydbInstanceConnectionPoolConfigElRef {
    fn new(shared: StackShared, base: String) -> DataAlloydbInstanceConnectionPoolConfigElRef {
        DataAlloydbInstanceConnectionPoolConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataAlloydbInstanceConnectionPoolConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
    #[doc = "Get a reference to the value of field `flags` after provisioning.\n"]
    pub fn flags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.flags", self.base))
    }
    #[doc = "Get a reference to the value of field `pooler_count` after provisioning.\n"]
    pub fn pooler_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.pooler_count", self.base))
    }
}
#[derive(Serialize)]
pub struct DataAlloydbInstanceMachineConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cpu_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    machine_type: Option<PrimField<String>>,
}
impl DataAlloydbInstanceMachineConfigEl {
    #[doc = "Set the field `cpu_count`.\n"]
    pub fn set_cpu_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.cpu_count = Some(v.into());
        self
    }
    #[doc = "Set the field `machine_type`.\n"]
    pub fn set_machine_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.machine_type = Some(v.into());
        self
    }
}
impl ToListMappable for DataAlloydbInstanceMachineConfigEl {
    type O = BlockAssignable<DataAlloydbInstanceMachineConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataAlloydbInstanceMachineConfigEl {}
impl BuildDataAlloydbInstanceMachineConfigEl {
    pub fn build(self) -> DataAlloydbInstanceMachineConfigEl {
        DataAlloydbInstanceMachineConfigEl {
            cpu_count: core::default::Default::default(),
            machine_type: core::default::Default::default(),
        }
    }
}
pub struct DataAlloydbInstanceMachineConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAlloydbInstanceMachineConfigElRef {
    fn new(shared: StackShared, base: String) -> DataAlloydbInstanceMachineConfigElRef {
        DataAlloydbInstanceMachineConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataAlloydbInstanceMachineConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cpu_count` after provisioning.\n"]
    pub fn cpu_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.cpu_count", self.base))
    }
    #[doc = "Get a reference to the value of field `machine_type` after provisioning.\n"]
    pub fn machine_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.base))
    }
}
#[derive(Serialize)]
pub struct DataAlloydbInstanceNetworkConfigElAuthorizedExternalNetworksEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cidr_range: Option<PrimField<String>>,
}
impl DataAlloydbInstanceNetworkConfigElAuthorizedExternalNetworksEl {
    #[doc = "Set the field `cidr_range`.\n"]
    pub fn set_cidr_range(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cidr_range = Some(v.into());
        self
    }
}
impl ToListMappable for DataAlloydbInstanceNetworkConfigElAuthorizedExternalNetworksEl {
    type O = BlockAssignable<DataAlloydbInstanceNetworkConfigElAuthorizedExternalNetworksEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataAlloydbInstanceNetworkConfigElAuthorizedExternalNetworksEl {}
impl BuildDataAlloydbInstanceNetworkConfigElAuthorizedExternalNetworksEl {
    pub fn build(self) -> DataAlloydbInstanceNetworkConfigElAuthorizedExternalNetworksEl {
        DataAlloydbInstanceNetworkConfigElAuthorizedExternalNetworksEl {
            cidr_range: core::default::Default::default(),
        }
    }
}
pub struct DataAlloydbInstanceNetworkConfigElAuthorizedExternalNetworksElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAlloydbInstanceNetworkConfigElAuthorizedExternalNetworksElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAlloydbInstanceNetworkConfigElAuthorizedExternalNetworksElRef {
        DataAlloydbInstanceNetworkConfigElAuthorizedExternalNetworksElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataAlloydbInstanceNetworkConfigElAuthorizedExternalNetworksElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cidr_range` after provisioning.\n"]
    pub fn cidr_range(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cidr_range", self.base))
    }
}
#[derive(Serialize)]
pub struct DataAlloydbInstanceNetworkConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allocated_ip_range_override: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authorized_external_networks:
        Option<ListField<DataAlloydbInstanceNetworkConfigElAuthorizedExternalNetworksEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_outbound_public_ip: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_public_ip: Option<PrimField<bool>>,
}
impl DataAlloydbInstanceNetworkConfigEl {
    #[doc = "Set the field `allocated_ip_range_override`.\n"]
    pub fn set_allocated_ip_range_override(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.allocated_ip_range_override = Some(v.into());
        self
    }
    #[doc = "Set the field `authorized_external_networks`.\n"]
    pub fn set_authorized_external_networks(
        mut self,
        v: impl Into<ListField<DataAlloydbInstanceNetworkConfigElAuthorizedExternalNetworksEl>>,
    ) -> Self {
        self.authorized_external_networks = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_outbound_public_ip`.\n"]
    pub fn set_enable_outbound_public_ip(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_outbound_public_ip = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_public_ip`.\n"]
    pub fn set_enable_public_ip(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_public_ip = Some(v.into());
        self
    }
}
impl ToListMappable for DataAlloydbInstanceNetworkConfigEl {
    type O = BlockAssignable<DataAlloydbInstanceNetworkConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataAlloydbInstanceNetworkConfigEl {}
impl BuildDataAlloydbInstanceNetworkConfigEl {
    pub fn build(self) -> DataAlloydbInstanceNetworkConfigEl {
        DataAlloydbInstanceNetworkConfigEl {
            allocated_ip_range_override: core::default::Default::default(),
            authorized_external_networks: core::default::Default::default(),
            enable_outbound_public_ip: core::default::Default::default(),
            enable_public_ip: core::default::Default::default(),
        }
    }
}
pub struct DataAlloydbInstanceNetworkConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAlloydbInstanceNetworkConfigElRef {
    fn new(shared: StackShared, base: String) -> DataAlloydbInstanceNetworkConfigElRef {
        DataAlloydbInstanceNetworkConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataAlloydbInstanceNetworkConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allocated_ip_range_override` after provisioning.\n"]
    pub fn allocated_ip_range_override(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allocated_ip_range_override", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `authorized_external_networks` after provisioning.\n"]
    pub fn authorized_external_networks(
        &self,
    ) -> ListRef<DataAlloydbInstanceNetworkConfigElAuthorizedExternalNetworksElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.authorized_external_networks", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_outbound_public_ip` after provisioning.\n"]
    pub fn enable_outbound_public_ip(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_outbound_public_ip", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_public_ip` after provisioning.\n"]
    pub fn enable_public_ip(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_public_ip", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataAlloydbInstancePscInstanceConfigElPscAutoConnectionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    consumer_network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    consumer_network_status: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    consumer_project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
}
impl DataAlloydbInstancePscInstanceConfigElPscAutoConnectionsEl {
    #[doc = "Set the field `consumer_network`.\n"]
    pub fn set_consumer_network(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.consumer_network = Some(v.into());
        self
    }
    #[doc = "Set the field `consumer_network_status`.\n"]
    pub fn set_consumer_network_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.consumer_network_status = Some(v.into());
        self
    }
    #[doc = "Set the field `consumer_project`.\n"]
    pub fn set_consumer_project(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.consumer_project = Some(v.into());
        self
    }
    #[doc = "Set the field `ip_address`.\n"]
    pub fn set_ip_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ip_address = Some(v.into());
        self
    }
    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status = Some(v.into());
        self
    }
}
impl ToListMappable for DataAlloydbInstancePscInstanceConfigElPscAutoConnectionsEl {
    type O = BlockAssignable<DataAlloydbInstancePscInstanceConfigElPscAutoConnectionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataAlloydbInstancePscInstanceConfigElPscAutoConnectionsEl {}
impl BuildDataAlloydbInstancePscInstanceConfigElPscAutoConnectionsEl {
    pub fn build(self) -> DataAlloydbInstancePscInstanceConfigElPscAutoConnectionsEl {
        DataAlloydbInstancePscInstanceConfigElPscAutoConnectionsEl {
            consumer_network: core::default::Default::default(),
            consumer_network_status: core::default::Default::default(),
            consumer_project: core::default::Default::default(),
            ip_address: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}
pub struct DataAlloydbInstancePscInstanceConfigElPscAutoConnectionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAlloydbInstancePscInstanceConfigElPscAutoConnectionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAlloydbInstancePscInstanceConfigElPscAutoConnectionsElRef {
        DataAlloydbInstancePscInstanceConfigElPscAutoConnectionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataAlloydbInstancePscInstanceConfigElPscAutoConnectionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `consumer_network` after provisioning.\n"]
    pub fn consumer_network(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.consumer_network", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `consumer_network_status` after provisioning.\n"]
    pub fn consumer_network_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.consumer_network_status", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `consumer_project` after provisioning.\n"]
    pub fn consumer_project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.consumer_project", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ip_address` after provisioning.\n"]
    pub fn ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip_address", self.base))
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}
#[derive(Serialize)]
pub struct DataAlloydbInstancePscInstanceConfigElPscInterfaceConfigsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    network_attachment_resource: Option<PrimField<String>>,
}
impl DataAlloydbInstancePscInstanceConfigElPscInterfaceConfigsEl {
    #[doc = "Set the field `network_attachment_resource`.\n"]
    pub fn set_network_attachment_resource(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network_attachment_resource = Some(v.into());
        self
    }
}
impl ToListMappable for DataAlloydbInstancePscInstanceConfigElPscInterfaceConfigsEl {
    type O = BlockAssignable<DataAlloydbInstancePscInstanceConfigElPscInterfaceConfigsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataAlloydbInstancePscInstanceConfigElPscInterfaceConfigsEl {}
impl BuildDataAlloydbInstancePscInstanceConfigElPscInterfaceConfigsEl {
    pub fn build(self) -> DataAlloydbInstancePscInstanceConfigElPscInterfaceConfigsEl {
        DataAlloydbInstancePscInstanceConfigElPscInterfaceConfigsEl {
            network_attachment_resource: core::default::Default::default(),
        }
    }
}
pub struct DataAlloydbInstancePscInstanceConfigElPscInterfaceConfigsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAlloydbInstancePscInstanceConfigElPscInterfaceConfigsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAlloydbInstancePscInstanceConfigElPscInterfaceConfigsElRef {
        DataAlloydbInstancePscInstanceConfigElPscInterfaceConfigsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataAlloydbInstancePscInstanceConfigElPscInterfaceConfigsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `network_attachment_resource` after provisioning.\n"]
    pub fn network_attachment_resource(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network_attachment_resource", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataAlloydbInstancePscInstanceConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_consumer_projects: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    psc_auto_connections:
        Option<ListField<DataAlloydbInstancePscInstanceConfigElPscAutoConnectionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    psc_dns_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    psc_interface_configs:
        Option<ListField<DataAlloydbInstancePscInstanceConfigElPscInterfaceConfigsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_attachment_link: Option<PrimField<String>>,
}
impl DataAlloydbInstancePscInstanceConfigEl {
    #[doc = "Set the field `allowed_consumer_projects`.\n"]
    pub fn set_allowed_consumer_projects(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.allowed_consumer_projects = Some(v.into());
        self
    }
    #[doc = "Set the field `psc_auto_connections`.\n"]
    pub fn set_psc_auto_connections(
        mut self,
        v: impl Into<ListField<DataAlloydbInstancePscInstanceConfigElPscAutoConnectionsEl>>,
    ) -> Self {
        self.psc_auto_connections = Some(v.into());
        self
    }
    #[doc = "Set the field `psc_dns_name`.\n"]
    pub fn set_psc_dns_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.psc_dns_name = Some(v.into());
        self
    }
    #[doc = "Set the field `psc_interface_configs`.\n"]
    pub fn set_psc_interface_configs(
        mut self,
        v: impl Into<ListField<DataAlloydbInstancePscInstanceConfigElPscInterfaceConfigsEl>>,
    ) -> Self {
        self.psc_interface_configs = Some(v.into());
        self
    }
    #[doc = "Set the field `service_attachment_link`.\n"]
    pub fn set_service_attachment_link(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_attachment_link = Some(v.into());
        self
    }
}
impl ToListMappable for DataAlloydbInstancePscInstanceConfigEl {
    type O = BlockAssignable<DataAlloydbInstancePscInstanceConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataAlloydbInstancePscInstanceConfigEl {}
impl BuildDataAlloydbInstancePscInstanceConfigEl {
    pub fn build(self) -> DataAlloydbInstancePscInstanceConfigEl {
        DataAlloydbInstancePscInstanceConfigEl {
            allowed_consumer_projects: core::default::Default::default(),
            psc_auto_connections: core::default::Default::default(),
            psc_dns_name: core::default::Default::default(),
            psc_interface_configs: core::default::Default::default(),
            service_attachment_link: core::default::Default::default(),
        }
    }
}
pub struct DataAlloydbInstancePscInstanceConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAlloydbInstancePscInstanceConfigElRef {
    fn new(shared: StackShared, base: String) -> DataAlloydbInstancePscInstanceConfigElRef {
        DataAlloydbInstancePscInstanceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataAlloydbInstancePscInstanceConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allowed_consumer_projects` after provisioning.\n"]
    pub fn allowed_consumer_projects(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allowed_consumer_projects", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `psc_auto_connections` after provisioning.\n"]
    pub fn psc_auto_connections(
        &self,
    ) -> ListRef<DataAlloydbInstancePscInstanceConfigElPscAutoConnectionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.psc_auto_connections", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `psc_dns_name` after provisioning.\n"]
    pub fn psc_dns_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.psc_dns_name", self.base))
    }
    #[doc = "Get a reference to the value of field `psc_interface_configs` after provisioning.\n"]
    pub fn psc_interface_configs(
        &self,
    ) -> ListRef<DataAlloydbInstancePscInstanceConfigElPscInterfaceConfigsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.psc_interface_configs", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `service_attachment_link` after provisioning.\n"]
    pub fn service_attachment_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_attachment_link", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataAlloydbInstanceQueryInsightsConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    query_plans_per_minute: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    query_string_length: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    record_application_tags: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    record_client_address: Option<PrimField<bool>>,
}
impl DataAlloydbInstanceQueryInsightsConfigEl {
    #[doc = "Set the field `query_plans_per_minute`.\n"]
    pub fn set_query_plans_per_minute(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.query_plans_per_minute = Some(v.into());
        self
    }
    #[doc = "Set the field `query_string_length`.\n"]
    pub fn set_query_string_length(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.query_string_length = Some(v.into());
        self
    }
    #[doc = "Set the field `record_application_tags`.\n"]
    pub fn set_record_application_tags(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.record_application_tags = Some(v.into());
        self
    }
    #[doc = "Set the field `record_client_address`.\n"]
    pub fn set_record_client_address(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.record_client_address = Some(v.into());
        self
    }
}
impl ToListMappable for DataAlloydbInstanceQueryInsightsConfigEl {
    type O = BlockAssignable<DataAlloydbInstanceQueryInsightsConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataAlloydbInstanceQueryInsightsConfigEl {}
impl BuildDataAlloydbInstanceQueryInsightsConfigEl {
    pub fn build(self) -> DataAlloydbInstanceQueryInsightsConfigEl {
        DataAlloydbInstanceQueryInsightsConfigEl {
            query_plans_per_minute: core::default::Default::default(),
            query_string_length: core::default::Default::default(),
            record_application_tags: core::default::Default::default(),
            record_client_address: core::default::Default::default(),
        }
    }
}
pub struct DataAlloydbInstanceQueryInsightsConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAlloydbInstanceQueryInsightsConfigElRef {
    fn new(shared: StackShared, base: String) -> DataAlloydbInstanceQueryInsightsConfigElRef {
        DataAlloydbInstanceQueryInsightsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataAlloydbInstanceQueryInsightsConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `query_plans_per_minute` after provisioning.\n"]
    pub fn query_plans_per_minute(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.query_plans_per_minute", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `query_string_length` after provisioning.\n"]
    pub fn query_string_length(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.query_string_length", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `record_application_tags` after provisioning.\n"]
    pub fn record_application_tags(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.record_application_tags", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `record_client_address` after provisioning.\n"]
    pub fn record_client_address(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.record_client_address", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataAlloydbInstanceReadPoolConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    node_count: Option<PrimField<f64>>,
}
impl DataAlloydbInstanceReadPoolConfigEl {
    #[doc = "Set the field `node_count`.\n"]
    pub fn set_node_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.node_count = Some(v.into());
        self
    }
}
impl ToListMappable for DataAlloydbInstanceReadPoolConfigEl {
    type O = BlockAssignable<DataAlloydbInstanceReadPoolConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataAlloydbInstanceReadPoolConfigEl {}
impl BuildDataAlloydbInstanceReadPoolConfigEl {
    pub fn build(self) -> DataAlloydbInstanceReadPoolConfigEl {
        DataAlloydbInstanceReadPoolConfigEl {
            node_count: core::default::Default::default(),
        }
    }
}
pub struct DataAlloydbInstanceReadPoolConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAlloydbInstanceReadPoolConfigElRef {
    fn new(shared: StackShared, base: String) -> DataAlloydbInstanceReadPoolConfigElRef {
        DataAlloydbInstanceReadPoolConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataAlloydbInstanceReadPoolConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `node_count` after provisioning.\n"]
    pub fn node_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.node_count", self.base))
    }
}
