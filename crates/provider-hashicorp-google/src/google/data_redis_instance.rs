use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataRedisInstanceData {
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
struct DataRedisInstance_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataRedisInstanceData>,
}
#[derive(Clone)]
pub struct DataRedisInstance(Rc<DataRedisInstance_>);
impl DataRedisInstance {
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
    #[doc = "Set the field `region`.\nThe name of the Redis region of the instance."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `alternative_location_id` after provisioning.\nOnly applicable to STANDARD_HA tier which protects the instance\nagainst zonal failures by provisioning it across two zones.\nIf provided, it must be a different zone from the one provided in\n[locationId]."]
    pub fn alternative_location_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.alternative_location_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `auth_enabled` after provisioning.\nOptional. Indicates whether OSS Redis AUTH is enabled for the\ninstance. If set to \"true\" AUTH is enabled on the instance.\nDefault value is \"false\" meaning AUTH is disabled."]
    pub fn auth_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.auth_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `auth_string` after provisioning.\nAUTH String set on the instance. This field will only be populated if auth_enabled is true."]
    pub fn auth_string(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.auth_string", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `authorized_network` after provisioning.\nThe full name of the Google Compute Engine network to which the\ninstance is connected. If left unspecified, the default network\nwill be used."]
    pub fn authorized_network(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.authorized_network", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `connect_mode` after provisioning.\nThe connection mode of the Redis instance. Default value: \"DIRECT_PEERING\" Possible values: [\"DIRECT_PEERING\", \"PRIVATE_SERVICE_ACCESS\"]"]
    pub fn connect_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.connect_mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe time the instance was created in RFC3339 UTC \"Zulu\" format,\naccurate to nanoseconds."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `current_location_id` after provisioning.\nThe current zone where the Redis endpoint is placed.\nFor Basic Tier instances, this will always be the same as the\n[locationId] provided by the user at creation time. For Standard Tier\ninstances, this can be either [locationId] or [alternativeLocationId]\nand can change after a failover event."]
    pub fn current_location_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.current_location_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `customer_managed_key` after provisioning.\nOptional. The KMS key reference that you want to use to encrypt the data at rest for this Redis\ninstance. If this is provided, CMEK is enabled."]
    pub fn customer_managed_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.customer_managed_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_protection` after provisioning.\nWhether Terraform will be prevented from destroying the instance.\nWhen a'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is not set to false in Terraform state.\nWhen the field is set to true or unset in Terraform state, a 'terraform apply'\nor 'terraform destroy' that would delete the instance will fail.\nWhen the field is set to false, deleting the instance is allowed."]
    pub fn deletion_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_protection", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nAn arbitrary and optional user-provided name for the instance."]
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
    #[doc = "Get a reference to the value of field `effective_reserved_ip_range` after provisioning.\nThe CIDR range of internal addresses that are reserved for this\ninstance. If not provided, the service will choose an unused /29\nblock, for example, 10.0.0.0/29 or 192.168.0.0/29. Ranges must be\nunique and non-overlapping with existing subnets in an authorized\nnetwork."]
    pub fn effective_reserved_ip_range(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.effective_reserved_ip_range", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `host` after provisioning.\nHostname or IP address of the exposed Redis endpoint used by clients\nto connect to the service."]
    pub fn host(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.host", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nResource labels to represent user provided metadata.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location_id` after provisioning.\nThe zone where the instance will be provisioned. If not provided,\nthe service will choose a zone for the instance. For STANDARD_HA tier,\ninstances will be created across two zones for protection against\nzonal failures. If [alternativeLocationId] is also provided, it must\nbe different from [locationId]."]
    pub fn location_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `maintenance_policy` after provisioning.\nMaintenance policy for an instance."]
    pub fn maintenance_policy(&self) -> ListRef<DataRedisInstanceMaintenancePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.maintenance_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `maintenance_schedule` after provisioning.\nUpcoming maintenance schedule."]
    pub fn maintenance_schedule(&self) -> ListRef<DataRedisInstanceMaintenanceScheduleElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.maintenance_schedule", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `maintenance_version` after provisioning.\nThe self service update maintenance version."]
    pub fn maintenance_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.maintenance_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `memory_size_gb` after provisioning.\nRedis memory size in GiB."]
    pub fn memory_size_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.memory_size_gb", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe ID of the instance or a fully qualified identifier for the instance."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `nodes` after provisioning.\nOutput only. Info per node."]
    pub fn nodes(&self) -> ListRef<DataRedisInstanceNodesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.nodes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `persistence_config` after provisioning.\nPersistence configuration for an instance."]
    pub fn persistence_config(&self) -> ListRef<DataRedisInstancePersistenceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.persistence_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `persistence_iam_identity` after provisioning.\nOutput only. Cloud IAM identity used by import / export operations\nto transfer data to/from Cloud Storage. Format is \"serviceAccount:\".\nThe value may change over time for a given instance so should be\nchecked before each import/export operation."]
    pub fn persistence_iam_identity(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.persistence_iam_identity", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\nThe port number of the exposed Redis endpoint."]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.port", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `read_endpoint` after provisioning.\nOutput only. Hostname or IP address of the exposed readonly Redis endpoint. Standard tier only.\nTargets all healthy replica nodes in instance. Replication is asynchronous and replica nodes\nwill exhibit some lag behind the primary. Write requests must target 'host'."]
    pub fn read_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.read_endpoint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `read_endpoint_port` after provisioning.\nOutput only. The port number of the exposed readonly redis endpoint. Standard tier only.\nWrite requests should target 'port'."]
    pub fn read_endpoint_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.read_endpoint_port", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `read_replicas_mode` after provisioning.\nOptional. Read replica mode. Can only be specified when trying to create the instance.\nIf not set, Memorystore Redis backend will default to READ_REPLICAS_DISABLED.\n- READ_REPLICAS_DISABLED: If disabled, read endpoint will not be provided and the\ninstance cannot scale up or down the number of replicas.\n- READ_REPLICAS_ENABLED: If enabled, read endpoint will be provided and the instance\ncan scale up and down the number of replicas. Possible values: [\"READ_REPLICAS_DISABLED\", \"READ_REPLICAS_ENABLED\"]"]
    pub fn read_replicas_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.read_replicas_mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `redis_configs` after provisioning.\nRedis configuration parameters, according to http://redis.io/topics/config.\nPlease check Memorystore documentation for the list of supported parameters:\nhttps://cloud.google.com/memorystore/docs/redis/reference/rest/v1/projects.locations.instances#Instance.FIELDS.redis_configs"]
    pub fn redis_configs(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.redis_configs", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `redis_version` after provisioning.\nThe version of Redis software. If not provided, latest supported\nversion will be used. Please check the API documentation linked\nat the top for the latest valid values."]
    pub fn redis_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.redis_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nThe name of the Redis region of the instance."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `replica_count` after provisioning.\nOptional. The number of replica nodes. The valid range for the Standard Tier with\nread replicas enabled is [1-5] and defaults to 2. If read replicas are not enabled\nfor a Standard Tier instance, the only valid value is 1 and the default is 1.\nThe valid value for basic tier is 0 and the default is also 0."]
    pub fn replica_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.replica_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reserved_ip_range` after provisioning.\nThe CIDR range of internal addresses that are reserved for this\ninstance. If not provided, the service will choose an unused /29\nblock, for example, 10.0.0.0/29 or 192.168.0.0/29. Ranges must be\nunique and non-overlapping with existing subnets in an authorized\nnetwork."]
    pub fn reserved_ip_range(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reserved_ip_range", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `secondary_ip_range` after provisioning.\nOptional. Additional IP range for node placement. Required when enabling read replicas on\nan existing instance. For DIRECT_PEERING mode value must be a CIDR range of size /28, or\n\"auto\". For PRIVATE_SERVICE_ACCESS mode value must be the name of an allocated address\nrange associated with the private service access connection, or \"auto\"."]
    pub fn secondary_ip_range(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.secondary_ip_range", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `server_ca_certs` after provisioning.\nList of server CA certificates for the instance."]
    pub fn server_ca_certs(&self) -> ListRef<DataRedisInstanceServerCaCertsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.server_ca_certs", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tier` after provisioning.\nThe service tier of the instance. Must be one of these values:\n\n- BASIC: standalone instance\n- STANDARD_HA: highly available primary/replica instances Default value: \"BASIC\" Possible values: [\"BASIC\", \"STANDARD_HA\"]"]
    pub fn tier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.tier", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `transit_encryption_mode` after provisioning.\nThe TLS mode of the Redis instance, If not provided, TLS is disabled for the instance.\n\n- SERVER_AUTHENTICATION: Client to Server traffic encryption enabled with server authentication Default value: \"DISABLED\" Possible values: [\"SERVER_AUTHENTICATION\", \"DISABLED\"]"]
    pub fn transit_encryption_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.transit_encryption_mode", self.extract_ref()),
        )
    }
}
impl Referable for DataRedisInstance {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataRedisInstance {}
impl ToListMappable for DataRedisInstance {
    type O = ListRef<DataRedisInstanceRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataRedisInstance_ {
    fn extract_datasource_type(&self) -> String {
        "google_redis_instance".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataRedisInstance {
    pub tf_id: String,
    #[doc = "The ID of the instance or a fully qualified identifier for the instance."]
    pub name: PrimField<String>,
}
impl BuildDataRedisInstance {
    pub fn build(self, stack: &mut Stack) -> DataRedisInstance {
        let out = DataRedisInstance(Rc::new(DataRedisInstance_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataRedisInstanceData {
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
pub struct DataRedisInstanceRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataRedisInstanceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataRedisInstanceRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `alternative_location_id` after provisioning.\nOnly applicable to STANDARD_HA tier which protects the instance\nagainst zonal failures by provisioning it across two zones.\nIf provided, it must be a different zone from the one provided in\n[locationId]."]
    pub fn alternative_location_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.alternative_location_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `auth_enabled` after provisioning.\nOptional. Indicates whether OSS Redis AUTH is enabled for the\ninstance. If set to \"true\" AUTH is enabled on the instance.\nDefault value is \"false\" meaning AUTH is disabled."]
    pub fn auth_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.auth_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `auth_string` after provisioning.\nAUTH String set on the instance. This field will only be populated if auth_enabled is true."]
    pub fn auth_string(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.auth_string", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `authorized_network` after provisioning.\nThe full name of the Google Compute Engine network to which the\ninstance is connected. If left unspecified, the default network\nwill be used."]
    pub fn authorized_network(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.authorized_network", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `connect_mode` after provisioning.\nThe connection mode of the Redis instance. Default value: \"DIRECT_PEERING\" Possible values: [\"DIRECT_PEERING\", \"PRIVATE_SERVICE_ACCESS\"]"]
    pub fn connect_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.connect_mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe time the instance was created in RFC3339 UTC \"Zulu\" format,\naccurate to nanoseconds."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `current_location_id` after provisioning.\nThe current zone where the Redis endpoint is placed.\nFor Basic Tier instances, this will always be the same as the\n[locationId] provided by the user at creation time. For Standard Tier\ninstances, this can be either [locationId] or [alternativeLocationId]\nand can change after a failover event."]
    pub fn current_location_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.current_location_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `customer_managed_key` after provisioning.\nOptional. The KMS key reference that you want to use to encrypt the data at rest for this Redis\ninstance. If this is provided, CMEK is enabled."]
    pub fn customer_managed_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.customer_managed_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_protection` after provisioning.\nWhether Terraform will be prevented from destroying the instance.\nWhen a'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is not set to false in Terraform state.\nWhen the field is set to true or unset in Terraform state, a 'terraform apply'\nor 'terraform destroy' that would delete the instance will fail.\nWhen the field is set to false, deleting the instance is allowed."]
    pub fn deletion_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_protection", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nAn arbitrary and optional user-provided name for the instance."]
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
    #[doc = "Get a reference to the value of field `effective_reserved_ip_range` after provisioning.\nThe CIDR range of internal addresses that are reserved for this\ninstance. If not provided, the service will choose an unused /29\nblock, for example, 10.0.0.0/29 or 192.168.0.0/29. Ranges must be\nunique and non-overlapping with existing subnets in an authorized\nnetwork."]
    pub fn effective_reserved_ip_range(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.effective_reserved_ip_range", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `host` after provisioning.\nHostname or IP address of the exposed Redis endpoint used by clients\nto connect to the service."]
    pub fn host(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.host", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nResource labels to represent user provided metadata.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location_id` after provisioning.\nThe zone where the instance will be provisioned. If not provided,\nthe service will choose a zone for the instance. For STANDARD_HA tier,\ninstances will be created across two zones for protection against\nzonal failures. If [alternativeLocationId] is also provided, it must\nbe different from [locationId]."]
    pub fn location_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `maintenance_policy` after provisioning.\nMaintenance policy for an instance."]
    pub fn maintenance_policy(&self) -> ListRef<DataRedisInstanceMaintenancePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.maintenance_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `maintenance_schedule` after provisioning.\nUpcoming maintenance schedule."]
    pub fn maintenance_schedule(&self) -> ListRef<DataRedisInstanceMaintenanceScheduleElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.maintenance_schedule", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `maintenance_version` after provisioning.\nThe self service update maintenance version."]
    pub fn maintenance_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.maintenance_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `memory_size_gb` after provisioning.\nRedis memory size in GiB."]
    pub fn memory_size_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.memory_size_gb", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe ID of the instance or a fully qualified identifier for the instance."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `nodes` after provisioning.\nOutput only. Info per node."]
    pub fn nodes(&self) -> ListRef<DataRedisInstanceNodesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.nodes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `persistence_config` after provisioning.\nPersistence configuration for an instance."]
    pub fn persistence_config(&self) -> ListRef<DataRedisInstancePersistenceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.persistence_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `persistence_iam_identity` after provisioning.\nOutput only. Cloud IAM identity used by import / export operations\nto transfer data to/from Cloud Storage. Format is \"serviceAccount:\".\nThe value may change over time for a given instance so should be\nchecked before each import/export operation."]
    pub fn persistence_iam_identity(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.persistence_iam_identity", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\nThe port number of the exposed Redis endpoint."]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.port", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `read_endpoint` after provisioning.\nOutput only. Hostname or IP address of the exposed readonly Redis endpoint. Standard tier only.\nTargets all healthy replica nodes in instance. Replication is asynchronous and replica nodes\nwill exhibit some lag behind the primary. Write requests must target 'host'."]
    pub fn read_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.read_endpoint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `read_endpoint_port` after provisioning.\nOutput only. The port number of the exposed readonly redis endpoint. Standard tier only.\nWrite requests should target 'port'."]
    pub fn read_endpoint_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.read_endpoint_port", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `read_replicas_mode` after provisioning.\nOptional. Read replica mode. Can only be specified when trying to create the instance.\nIf not set, Memorystore Redis backend will default to READ_REPLICAS_DISABLED.\n- READ_REPLICAS_DISABLED: If disabled, read endpoint will not be provided and the\ninstance cannot scale up or down the number of replicas.\n- READ_REPLICAS_ENABLED: If enabled, read endpoint will be provided and the instance\ncan scale up and down the number of replicas. Possible values: [\"READ_REPLICAS_DISABLED\", \"READ_REPLICAS_ENABLED\"]"]
    pub fn read_replicas_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.read_replicas_mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `redis_configs` after provisioning.\nRedis configuration parameters, according to http://redis.io/topics/config.\nPlease check Memorystore documentation for the list of supported parameters:\nhttps://cloud.google.com/memorystore/docs/redis/reference/rest/v1/projects.locations.instances#Instance.FIELDS.redis_configs"]
    pub fn redis_configs(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.redis_configs", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `redis_version` after provisioning.\nThe version of Redis software. If not provided, latest supported\nversion will be used. Please check the API documentation linked\nat the top for the latest valid values."]
    pub fn redis_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.redis_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nThe name of the Redis region of the instance."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `replica_count` after provisioning.\nOptional. The number of replica nodes. The valid range for the Standard Tier with\nread replicas enabled is [1-5] and defaults to 2. If read replicas are not enabled\nfor a Standard Tier instance, the only valid value is 1 and the default is 1.\nThe valid value for basic tier is 0 and the default is also 0."]
    pub fn replica_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.replica_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reserved_ip_range` after provisioning.\nThe CIDR range of internal addresses that are reserved for this\ninstance. If not provided, the service will choose an unused /29\nblock, for example, 10.0.0.0/29 or 192.168.0.0/29. Ranges must be\nunique and non-overlapping with existing subnets in an authorized\nnetwork."]
    pub fn reserved_ip_range(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reserved_ip_range", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `secondary_ip_range` after provisioning.\nOptional. Additional IP range for node placement. Required when enabling read replicas on\nan existing instance. For DIRECT_PEERING mode value must be a CIDR range of size /28, or\n\"auto\". For PRIVATE_SERVICE_ACCESS mode value must be the name of an allocated address\nrange associated with the private service access connection, or \"auto\"."]
    pub fn secondary_ip_range(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.secondary_ip_range", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `server_ca_certs` after provisioning.\nList of server CA certificates for the instance."]
    pub fn server_ca_certs(&self) -> ListRef<DataRedisInstanceServerCaCertsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.server_ca_certs", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tier` after provisioning.\nThe service tier of the instance. Must be one of these values:\n\n- BASIC: standalone instance\n- STANDARD_HA: highly available primary/replica instances Default value: \"BASIC\" Possible values: [\"BASIC\", \"STANDARD_HA\"]"]
    pub fn tier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.tier", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `transit_encryption_mode` after provisioning.\nThe TLS mode of the Redis instance, If not provided, TLS is disabled for the instance.\n\n- SERVER_AUTHENTICATION: Client to Server traffic encryption enabled with server authentication Default value: \"DISABLED\" Possible values: [\"SERVER_AUTHENTICATION\", \"DISABLED\"]"]
    pub fn transit_encryption_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.transit_encryption_mode", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataRedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    hours: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minutes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nanos: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    seconds: Option<PrimField<f64>>,
}
impl DataRedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl {
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
impl ToListMappable for DataRedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl {
    type O =
        BlockAssignable<DataRedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataRedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl {}
impl BuildDataRedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl {
    pub fn build(self) -> DataRedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl {
        DataRedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl {
            hours: core::default::Default::default(),
            minutes: core::default::Default::default(),
            nanos: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}
pub struct DataRedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataRedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataRedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeElRef {
        DataRedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataRedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeElRef {
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
pub struct DataRedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    day: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time:
        Option<ListField<DataRedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl>>,
}
impl DataRedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowEl {
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
            ListField<DataRedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl>,
        >,
    ) -> Self {
        self.start_time = Some(v.into());
        self
    }
}
impl ToListMappable for DataRedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowEl {
    type O = BlockAssignable<DataRedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataRedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowEl {}
impl BuildDataRedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowEl {
    pub fn build(self) -> DataRedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowEl {
        DataRedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowEl {
            day: core::default::Default::default(),
            duration: core::default::Default::default(),
            start_time: core::default::Default::default(),
        }
    }
}
pub struct DataRedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataRedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataRedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowElRef {
        DataRedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataRedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowElRef {
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
    ) -> ListRef<DataRedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}
#[derive(Serialize)]
pub struct DataRedisInstanceMaintenancePolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weekly_maintenance_window:
        Option<ListField<DataRedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowEl>>,
}
impl DataRedisInstanceMaintenancePolicyEl {
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
        v: impl Into<ListField<DataRedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowEl>>,
    ) -> Self {
        self.weekly_maintenance_window = Some(v.into());
        self
    }
}
impl ToListMappable for DataRedisInstanceMaintenancePolicyEl {
    type O = BlockAssignable<DataRedisInstanceMaintenancePolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataRedisInstanceMaintenancePolicyEl {}
impl BuildDataRedisInstanceMaintenancePolicyEl {
    pub fn build(self) -> DataRedisInstanceMaintenancePolicyEl {
        DataRedisInstanceMaintenancePolicyEl {
            create_time: core::default::Default::default(),
            description: core::default::Default::default(),
            update_time: core::default::Default::default(),
            weekly_maintenance_window: core::default::Default::default(),
        }
    }
}
pub struct DataRedisInstanceMaintenancePolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataRedisInstanceMaintenancePolicyElRef {
    fn new(shared: StackShared, base: String) -> DataRedisInstanceMaintenancePolicyElRef {
        DataRedisInstanceMaintenancePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataRedisInstanceMaintenancePolicyElRef {
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
    ) -> ListRef<DataRedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.weekly_maintenance_window", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataRedisInstanceMaintenanceScheduleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    end_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schedule_deadline_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time: Option<PrimField<String>>,
}
impl DataRedisInstanceMaintenanceScheduleEl {
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
impl ToListMappable for DataRedisInstanceMaintenanceScheduleEl {
    type O = BlockAssignable<DataRedisInstanceMaintenanceScheduleEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataRedisInstanceMaintenanceScheduleEl {}
impl BuildDataRedisInstanceMaintenanceScheduleEl {
    pub fn build(self) -> DataRedisInstanceMaintenanceScheduleEl {
        DataRedisInstanceMaintenanceScheduleEl {
            end_time: core::default::Default::default(),
            schedule_deadline_time: core::default::Default::default(),
            start_time: core::default::Default::default(),
        }
    }
}
pub struct DataRedisInstanceMaintenanceScheduleElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataRedisInstanceMaintenanceScheduleElRef {
    fn new(shared: StackShared, base: String) -> DataRedisInstanceMaintenanceScheduleElRef {
        DataRedisInstanceMaintenanceScheduleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataRedisInstanceMaintenanceScheduleElRef {
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
pub struct DataRedisInstanceNodesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone: Option<PrimField<String>>,
}
impl DataRedisInstanceNodesEl {
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }
    #[doc = "Set the field `zone`.\n"]
    pub fn set_zone(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.zone = Some(v.into());
        self
    }
}
impl ToListMappable for DataRedisInstanceNodesEl {
    type O = BlockAssignable<DataRedisInstanceNodesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataRedisInstanceNodesEl {}
impl BuildDataRedisInstanceNodesEl {
    pub fn build(self) -> DataRedisInstanceNodesEl {
        DataRedisInstanceNodesEl {
            id: core::default::Default::default(),
            zone: core::default::Default::default(),
        }
    }
}
pub struct DataRedisInstanceNodesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataRedisInstanceNodesElRef {
    fn new(shared: StackShared, base: String) -> DataRedisInstanceNodesElRef {
        DataRedisInstanceNodesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataRedisInstanceNodesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
    #[doc = "Get a reference to the value of field `zone` after provisioning.\n"]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone", self.base))
    }
}
#[derive(Serialize)]
pub struct DataRedisInstancePersistenceConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    persistence_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rdb_next_snapshot_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rdb_snapshot_period: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rdb_snapshot_start_time: Option<PrimField<String>>,
}
impl DataRedisInstancePersistenceConfigEl {
    #[doc = "Set the field `persistence_mode`.\n"]
    pub fn set_persistence_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.persistence_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `rdb_next_snapshot_time`.\n"]
    pub fn set_rdb_next_snapshot_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.rdb_next_snapshot_time = Some(v.into());
        self
    }
    #[doc = "Set the field `rdb_snapshot_period`.\n"]
    pub fn set_rdb_snapshot_period(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.rdb_snapshot_period = Some(v.into());
        self
    }
    #[doc = "Set the field `rdb_snapshot_start_time`.\n"]
    pub fn set_rdb_snapshot_start_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.rdb_snapshot_start_time = Some(v.into());
        self
    }
}
impl ToListMappable for DataRedisInstancePersistenceConfigEl {
    type O = BlockAssignable<DataRedisInstancePersistenceConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataRedisInstancePersistenceConfigEl {}
impl BuildDataRedisInstancePersistenceConfigEl {
    pub fn build(self) -> DataRedisInstancePersistenceConfigEl {
        DataRedisInstancePersistenceConfigEl {
            persistence_mode: core::default::Default::default(),
            rdb_next_snapshot_time: core::default::Default::default(),
            rdb_snapshot_period: core::default::Default::default(),
            rdb_snapshot_start_time: core::default::Default::default(),
        }
    }
}
pub struct DataRedisInstancePersistenceConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataRedisInstancePersistenceConfigElRef {
    fn new(shared: StackShared, base: String) -> DataRedisInstancePersistenceConfigElRef {
        DataRedisInstancePersistenceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataRedisInstancePersistenceConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `persistence_mode` after provisioning.\n"]
    pub fn persistence_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.persistence_mode", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `rdb_next_snapshot_time` after provisioning.\n"]
    pub fn rdb_next_snapshot_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rdb_next_snapshot_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `rdb_snapshot_period` after provisioning.\n"]
    pub fn rdb_snapshot_period(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rdb_snapshot_period", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `rdb_snapshot_start_time` after provisioning.\n"]
    pub fn rdb_snapshot_start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rdb_snapshot_start_time", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataRedisInstanceServerCaCertsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cert: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    create_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expire_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    serial_number: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sha1_fingerprint: Option<PrimField<String>>,
}
impl DataRedisInstanceServerCaCertsEl {
    #[doc = "Set the field `cert`.\n"]
    pub fn set_cert(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cert = Some(v.into());
        self
    }
    #[doc = "Set the field `create_time`.\n"]
    pub fn set_create_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create_time = Some(v.into());
        self
    }
    #[doc = "Set the field `expire_time`.\n"]
    pub fn set_expire_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.expire_time = Some(v.into());
        self
    }
    #[doc = "Set the field `serial_number`.\n"]
    pub fn set_serial_number(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.serial_number = Some(v.into());
        self
    }
    #[doc = "Set the field `sha1_fingerprint`.\n"]
    pub fn set_sha1_fingerprint(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sha1_fingerprint = Some(v.into());
        self
    }
}
impl ToListMappable for DataRedisInstanceServerCaCertsEl {
    type O = BlockAssignable<DataRedisInstanceServerCaCertsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataRedisInstanceServerCaCertsEl {}
impl BuildDataRedisInstanceServerCaCertsEl {
    pub fn build(self) -> DataRedisInstanceServerCaCertsEl {
        DataRedisInstanceServerCaCertsEl {
            cert: core::default::Default::default(),
            create_time: core::default::Default::default(),
            expire_time: core::default::Default::default(),
            serial_number: core::default::Default::default(),
            sha1_fingerprint: core::default::Default::default(),
        }
    }
}
pub struct DataRedisInstanceServerCaCertsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataRedisInstanceServerCaCertsElRef {
    fn new(shared: StackShared, base: String) -> DataRedisInstanceServerCaCertsElRef {
        DataRedisInstanceServerCaCertsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataRedisInstanceServerCaCertsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cert` after provisioning.\n"]
    pub fn cert(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cert", self.base))
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.base))
    }
    #[doc = "Get a reference to the value of field `expire_time` after provisioning.\n"]
    pub fn expire_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expire_time", self.base))
    }
    #[doc = "Get a reference to the value of field `serial_number` after provisioning.\n"]
    pub fn serial_number(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.serial_number", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `sha1_fingerprint` after provisioning.\n"]
    pub fn sha1_fingerprint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sha1_fingerprint", self.base),
        )
    }
}
