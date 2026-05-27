use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct RedisInstanceData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    alternative_location_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auth_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authorized_network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    connect_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_managed_key: Option<PrimField<String>>,
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
    location_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maintenance_version: Option<PrimField<String>>,
    memory_size_gb: PrimField<f64>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    read_replicas_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    redis_configs: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    redis_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    replica_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reserved_ip_range: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secondary_ip_range: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tier: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transit_encryption_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maintenance_policy: Option<Vec<RedisInstanceMaintenancePolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    persistence_config: Option<Vec<RedisInstancePersistenceConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<RedisInstanceTimeoutsEl>,
    dynamic: RedisInstanceDynamic,
}
struct RedisInstance_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<RedisInstanceData>,
}
#[derive(Clone)]
pub struct RedisInstance(Rc<RedisInstance_>);
impl RedisInstance {
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
    #[doc = "Set the field `alternative_location_id`.\nOnly applicable to STANDARD_HA tier which protects the instance\nagainst zonal failures by provisioning it across two zones.\nIf provided, it must be a different zone from the one provided in\n[locationId]."]
    pub fn set_alternative_location_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().alternative_location_id = Some(v.into());
        self
    }
    #[doc = "Set the field `auth_enabled`.\nOptional. Indicates whether OSS Redis AUTH is enabled for the\ninstance. If set to \"true\" AUTH is enabled on the instance.\nDefault value is \"false\" meaning AUTH is disabled."]
    pub fn set_auth_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().auth_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `authorized_network`.\nThe full name of the Google Compute Engine network to which the\ninstance is connected. If left unspecified, the default network\nwill be used."]
    pub fn set_authorized_network(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().authorized_network = Some(v.into());
        self
    }
    #[doc = "Set the field `connect_mode`.\nThe connection mode of the Redis instance. Default value: \"DIRECT_PEERING\" Possible values: [\"DIRECT_PEERING\", \"PRIVATE_SERVICE_ACCESS\"]"]
    pub fn set_connect_mode(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().connect_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `customer_managed_key`.\nOptional. The KMS key reference that you want to use to encrypt the data at rest for this Redis\ninstance. If this is provided, CMEK is enabled."]
    pub fn set_customer_managed_key(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().customer_managed_key = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_protection`.\nWhether Terraform will be prevented from destroying the instance.\nWhen a'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is not set to false in Terraform state.\nWhen the field is set to true or unset in Terraform state, a 'terraform apply'\nor 'terraform destroy' that would delete the instance will fail.\nWhen the field is set to false, deleting the instance is allowed."]
    pub fn set_deletion_protection(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().deletion_protection = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\nAn arbitrary and optional user-provided name for the instance."]
    pub fn set_display_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nResource labels to represent user provided metadata.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `location_id`.\nThe zone where the instance will be provisioned. If not provided,\nthe service will choose a zone for the instance. For STANDARD_HA tier,\ninstances will be created across two zones for protection against\nzonal failures. If [alternativeLocationId] is also provided, it must\nbe different from [locationId]."]
    pub fn set_location_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().location_id = Some(v.into());
        self
    }
    #[doc = "Set the field `maintenance_version`.\nThe self service update maintenance version."]
    pub fn set_maintenance_version(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().maintenance_version = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `read_replicas_mode`.\nOptional. Read replica mode. Can only be specified when trying to create the instance.\nIf not set, Memorystore Redis backend will default to READ_REPLICAS_DISABLED.\n- READ_REPLICAS_DISABLED: If disabled, read endpoint will not be provided and the\ninstance cannot scale up or down the number of replicas.\n- READ_REPLICAS_ENABLED: If enabled, read endpoint will be provided and the instance\ncan scale up and down the number of replicas. Possible values: [\"READ_REPLICAS_DISABLED\", \"READ_REPLICAS_ENABLED\"]"]
    pub fn set_read_replicas_mode(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().read_replicas_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `redis_configs`.\nRedis configuration parameters, according to http://redis.io/topics/config.\nPlease check Memorystore documentation for the list of supported parameters:\nhttps://cloud.google.com/memorystore/docs/redis/reference/rest/v1/projects.locations.instances#Instance.FIELDS.redis_configs"]
    pub fn set_redis_configs(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().redis_configs = Some(v.into());
        self
    }
    #[doc = "Set the field `redis_version`.\nThe version of Redis software. If not provided, latest supported\nversion will be used. Please check the API documentation linked\nat the top for the latest valid values."]
    pub fn set_redis_version(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().redis_version = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nThe name of the Redis region of the instance."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `replica_count`.\nOptional. The number of replica nodes. The valid range for the Standard Tier with\nread replicas enabled is [1-5] and defaults to 2. If read replicas are not enabled\nfor a Standard Tier instance, the only valid value is 1 and the default is 1.\nThe valid value for basic tier is 0 and the default is also 0."]
    pub fn set_replica_count(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().replica_count = Some(v.into());
        self
    }
    #[doc = "Set the field `reserved_ip_range`.\nThe CIDR range of internal addresses that are reserved for this\ninstance. If not provided, the service will choose an unused /29\nblock, for example, 10.0.0.0/29 or 192.168.0.0/29. Ranges must be\nunique and non-overlapping with existing subnets in an authorized\nnetwork."]
    pub fn set_reserved_ip_range(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().reserved_ip_range = Some(v.into());
        self
    }
    #[doc = "Set the field `secondary_ip_range`.\nOptional. Additional IP range for node placement. Required when enabling read replicas on\nan existing instance. For DIRECT_PEERING mode value must be a CIDR range of size /28, or\n\"auto\". For PRIVATE_SERVICE_ACCESS mode value must be the name of an allocated address\nrange associated with the private service access connection, or \"auto\"."]
    pub fn set_secondary_ip_range(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().secondary_ip_range = Some(v.into());
        self
    }
    #[doc = "Set the field `tier`.\nThe service tier of the instance. Must be one of these values:\n\n- BASIC: standalone instance\n- STANDARD_HA: highly available primary/replica instances Default value: \"BASIC\" Possible values: [\"BASIC\", \"STANDARD_HA\"]"]
    pub fn set_tier(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().tier = Some(v.into());
        self
    }
    #[doc = "Set the field `transit_encryption_mode`.\nThe TLS mode of the Redis instance, If not provided, TLS is disabled for the instance.\n\n- SERVER_AUTHENTICATION: Client to Server traffic encryption enabled with server authentication Default value: \"DISABLED\" Possible values: [\"SERVER_AUTHENTICATION\", \"DISABLED\"]"]
    pub fn set_transit_encryption_mode(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().transit_encryption_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `maintenance_policy`.\n"]
    pub fn set_maintenance_policy(
        self,
        v: impl Into<BlockAssignable<RedisInstanceMaintenancePolicyEl>>,
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
    #[doc = "Set the field `persistence_config`.\n"]
    pub fn set_persistence_config(
        self,
        v: impl Into<BlockAssignable<RedisInstancePersistenceConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().persistence_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.persistence_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<RedisInstanceTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
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
    #[doc = "Get a reference to the value of field `maintenance_schedule` after provisioning.\nUpcoming maintenance schedule."]
    pub fn maintenance_schedule(&self) -> ListRef<RedisInstanceMaintenanceScheduleElRef> {
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
    pub fn nodes(&self) -> ListRef<RedisInstanceNodesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.nodes", self.extract_ref()),
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
    pub fn server_ca_certs(&self) -> ListRef<RedisInstanceServerCaCertsElRef> {
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
    #[doc = "Get a reference to the value of field `maintenance_policy` after provisioning.\n"]
    pub fn maintenance_policy(&self) -> ListRef<RedisInstanceMaintenancePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.maintenance_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `persistence_config` after provisioning.\n"]
    pub fn persistence_config(&self) -> ListRef<RedisInstancePersistenceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.persistence_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> RedisInstanceTimeoutsElRef {
        RedisInstanceTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for RedisInstance {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for RedisInstance {}
impl ToListMappable for RedisInstance {
    type O = ListRef<RedisInstanceRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for RedisInstance_ {
    fn extract_resource_type(&self) -> String {
        "google_redis_instance".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildRedisInstance {
    pub tf_id: String,
    #[doc = "Redis memory size in GiB."]
    pub memory_size_gb: PrimField<f64>,
    #[doc = "The ID of the instance or a fully qualified identifier for the instance."]
    pub name: PrimField<String>,
}
impl BuildRedisInstance {
    pub fn build(self, stack: &mut Stack) -> RedisInstance {
        let out = RedisInstance(Rc::new(RedisInstance_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(RedisInstanceData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                alternative_location_id: core::default::Default::default(),
                auth_enabled: core::default::Default::default(),
                authorized_network: core::default::Default::default(),
                connect_mode: core::default::Default::default(),
                customer_managed_key: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                deletion_protection: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location_id: core::default::Default::default(),
                maintenance_version: core::default::Default::default(),
                memory_size_gb: self.memory_size_gb,
                name: self.name,
                project: core::default::Default::default(),
                read_replicas_mode: core::default::Default::default(),
                redis_configs: core::default::Default::default(),
                redis_version: core::default::Default::default(),
                region: core::default::Default::default(),
                replica_count: core::default::Default::default(),
                reserved_ip_range: core::default::Default::default(),
                secondary_ip_range: core::default::Default::default(),
                tier: core::default::Default::default(),
                transit_encryption_mode: core::default::Default::default(),
                maintenance_policy: core::default::Default::default(),
                persistence_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct RedisInstanceRef {
    shared: StackShared,
    base: String,
}
impl Ref for RedisInstanceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl RedisInstanceRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
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
    #[doc = "Get a reference to the value of field `maintenance_schedule` after provisioning.\nUpcoming maintenance schedule."]
    pub fn maintenance_schedule(&self) -> ListRef<RedisInstanceMaintenanceScheduleElRef> {
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
    pub fn nodes(&self) -> ListRef<RedisInstanceNodesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.nodes", self.extract_ref()),
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
    pub fn server_ca_certs(&self) -> ListRef<RedisInstanceServerCaCertsElRef> {
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
    #[doc = "Get a reference to the value of field `maintenance_policy` after provisioning.\n"]
    pub fn maintenance_policy(&self) -> ListRef<RedisInstanceMaintenancePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.maintenance_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `persistence_config` after provisioning.\n"]
    pub fn persistence_config(&self) -> ListRef<RedisInstancePersistenceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.persistence_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> RedisInstanceTimeoutsElRef {
        RedisInstanceTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct RedisInstanceMaintenanceScheduleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    end_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schedule_deadline_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time: Option<PrimField<String>>,
}
impl RedisInstanceMaintenanceScheduleEl {
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
impl ToListMappable for RedisInstanceMaintenanceScheduleEl {
    type O = BlockAssignable<RedisInstanceMaintenanceScheduleEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildRedisInstanceMaintenanceScheduleEl {}
impl BuildRedisInstanceMaintenanceScheduleEl {
    pub fn build(self) -> RedisInstanceMaintenanceScheduleEl {
        RedisInstanceMaintenanceScheduleEl {
            end_time: core::default::Default::default(),
            schedule_deadline_time: core::default::Default::default(),
            start_time: core::default::Default::default(),
        }
    }
}
pub struct RedisInstanceMaintenanceScheduleElRef {
    shared: StackShared,
    base: String,
}
impl Ref for RedisInstanceMaintenanceScheduleElRef {
    fn new(shared: StackShared, base: String) -> RedisInstanceMaintenanceScheduleElRef {
        RedisInstanceMaintenanceScheduleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl RedisInstanceMaintenanceScheduleElRef {
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
pub struct RedisInstanceNodesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone: Option<PrimField<String>>,
}
impl RedisInstanceNodesEl {
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
impl ToListMappable for RedisInstanceNodesEl {
    type O = BlockAssignable<RedisInstanceNodesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildRedisInstanceNodesEl {}
impl BuildRedisInstanceNodesEl {
    pub fn build(self) -> RedisInstanceNodesEl {
        RedisInstanceNodesEl {
            id: core::default::Default::default(),
            zone: core::default::Default::default(),
        }
    }
}
pub struct RedisInstanceNodesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for RedisInstanceNodesElRef {
    fn new(shared: StackShared, base: String) -> RedisInstanceNodesElRef {
        RedisInstanceNodesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl RedisInstanceNodesElRef {
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
pub struct RedisInstanceServerCaCertsEl {
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
impl RedisInstanceServerCaCertsEl {
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
impl ToListMappable for RedisInstanceServerCaCertsEl {
    type O = BlockAssignable<RedisInstanceServerCaCertsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildRedisInstanceServerCaCertsEl {}
impl BuildRedisInstanceServerCaCertsEl {
    pub fn build(self) -> RedisInstanceServerCaCertsEl {
        RedisInstanceServerCaCertsEl {
            cert: core::default::Default::default(),
            create_time: core::default::Default::default(),
            expire_time: core::default::Default::default(),
            serial_number: core::default::Default::default(),
            sha1_fingerprint: core::default::Default::default(),
        }
    }
}
pub struct RedisInstanceServerCaCertsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for RedisInstanceServerCaCertsElRef {
    fn new(shared: StackShared, base: String) -> RedisInstanceServerCaCertsElRef {
        RedisInstanceServerCaCertsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl RedisInstanceServerCaCertsElRef {
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
#[derive(Serialize)]
pub struct RedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    hours: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minutes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nanos: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    seconds: Option<PrimField<f64>>,
}
impl RedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl {
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
impl ToListMappable for RedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl {
    type O = BlockAssignable<RedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildRedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl {}
impl BuildRedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl {
    pub fn build(self) -> RedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl {
        RedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl {
            hours: core::default::Default::default(),
            minutes: core::default::Default::default(),
            nanos: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}
pub struct RedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for RedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> RedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeElRef {
        RedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl RedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeElRef {
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
struct RedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowElDynamic {
    start_time:
        Option<DynamicBlock<RedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl>>,
}
#[derive(Serialize)]
pub struct RedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowEl {
    day: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time: Option<Vec<RedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl>>,
    dynamic: RedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowElDynamic,
}
impl RedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowEl {
    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(
        mut self,
        v: impl Into<
            BlockAssignable<RedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeEl>,
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
impl ToListMappable for RedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowEl {
    type O = BlockAssignable<RedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildRedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowEl {
    #[doc = "Required. The day of week that maintenance updates occur.\n\n- DAY_OF_WEEK_UNSPECIFIED: The day of the week is unspecified.\n- MONDAY: Monday\n- TUESDAY: Tuesday\n- WEDNESDAY: Wednesday\n- THURSDAY: Thursday\n- FRIDAY: Friday\n- SATURDAY: Saturday\n- SUNDAY: Sunday Possible values: [\"DAY_OF_WEEK_UNSPECIFIED\", \"MONDAY\", \"TUESDAY\", \"WEDNESDAY\", \"THURSDAY\", \"FRIDAY\", \"SATURDAY\", \"SUNDAY\"]"]
    pub day: PrimField<String>,
}
impl BuildRedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowEl {
    pub fn build(self) -> RedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowEl {
        RedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowEl {
            day: self.day,
            start_time: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct RedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowElRef {
    shared: StackShared,
    base: String,
}
impl Ref for RedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> RedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowElRef {
        RedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl RedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `day` after provisioning.\nRequired. The day of week that maintenance updates occur.\n\n- DAY_OF_WEEK_UNSPECIFIED: The day of the week is unspecified.\n- MONDAY: Monday\n- TUESDAY: Tuesday\n- WEDNESDAY: Wednesday\n- THURSDAY: Thursday\n- FRIDAY: Friday\n- SATURDAY: Saturday\n- SUNDAY: Sunday Possible values: [\"DAY_OF_WEEK_UNSPECIFIED\", \"MONDAY\", \"TUESDAY\", \"WEDNESDAY\", \"THURSDAY\", \"FRIDAY\", \"SATURDAY\", \"SUNDAY\"]"]
    pub fn day(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.day", self.base))
    }
    #[doc = "Get a reference to the value of field `duration` after provisioning.\nOutput only. Duration of the maintenance window.\nThe current window is fixed at 1 hour.\nA duration in seconds with up to nine fractional digits,\nterminated by 's'. Example: \"3.5s\"."]
    pub fn duration(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.duration", self.base))
    }
    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(
        &self,
    ) -> ListRef<RedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowElStartTimeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}
#[derive(Serialize, Default)]
struct RedisInstanceMaintenancePolicyElDynamic {
    weekly_maintenance_window:
        Option<DynamicBlock<RedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowEl>>,
}
#[derive(Serialize)]
pub struct RedisInstanceMaintenancePolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weekly_maintenance_window:
        Option<Vec<RedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowEl>>,
    dynamic: RedisInstanceMaintenancePolicyElDynamic,
}
impl RedisInstanceMaintenancePolicyEl {
    #[doc = "Set the field `description`.\nOptional. Description of what this policy is for.\nCreate/Update methods return INVALID_ARGUMENT if the\nlength is greater than 512."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `weekly_maintenance_window`.\n"]
    pub fn set_weekly_maintenance_window(
        mut self,
        v: impl Into<BlockAssignable<RedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowEl>>,
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
impl ToListMappable for RedisInstanceMaintenancePolicyEl {
    type O = BlockAssignable<RedisInstanceMaintenancePolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildRedisInstanceMaintenancePolicyEl {}
impl BuildRedisInstanceMaintenancePolicyEl {
    pub fn build(self) -> RedisInstanceMaintenancePolicyEl {
        RedisInstanceMaintenancePolicyEl {
            description: core::default::Default::default(),
            weekly_maintenance_window: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct RedisInstanceMaintenancePolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for RedisInstanceMaintenancePolicyElRef {
    fn new(shared: StackShared, base: String) -> RedisInstanceMaintenancePolicyElRef {
        RedisInstanceMaintenancePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl RedisInstanceMaintenancePolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The time when the policy was created.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond\nresolution and up to nine fractional digits."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.base))
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nOptional. Description of what this policy is for.\nCreate/Update methods return INVALID_ARGUMENT if the\nlength is greater than 512."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The time when the policy was last updated.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond\nresolution and up to nine fractional digits."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }
    #[doc = "Get a reference to the value of field `weekly_maintenance_window` after provisioning.\n"]
    pub fn weekly_maintenance_window(
        &self,
    ) -> ListRef<RedisInstanceMaintenancePolicyElWeeklyMaintenanceWindowElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.weekly_maintenance_window", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct RedisInstancePersistenceConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    persistence_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rdb_snapshot_period: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rdb_snapshot_start_time: Option<PrimField<String>>,
}
impl RedisInstancePersistenceConfigEl {
    #[doc = "Set the field `persistence_mode`.\nOptional. Controls whether Persistence features are enabled. If not provided, the existing value will be used.\n\n- DISABLED: \tPersistence is disabled for the instance, and any existing snapshots are deleted.\n- RDB: RDB based Persistence is enabled. Possible values: [\"DISABLED\", \"RDB\"]"]
    pub fn set_persistence_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.persistence_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `rdb_snapshot_period`.\nOptional. Available snapshot periods for scheduling.\n\n- ONE_HOUR:\tSnapshot every 1 hour.\n- SIX_HOURS:\tSnapshot every 6 hours.\n- TWELVE_HOURS:\tSnapshot every 12 hours.\n- TWENTY_FOUR_HOURS:\tSnapshot every 24 hours. Possible values: [\"ONE_HOUR\", \"SIX_HOURS\", \"TWELVE_HOURS\", \"TWENTY_FOUR_HOURS\"]"]
    pub fn set_rdb_snapshot_period(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.rdb_snapshot_period = Some(v.into());
        self
    }
    #[doc = "Set the field `rdb_snapshot_start_time`.\nOptional. Date and time that the first snapshot was/will be attempted,\nand to which future snapshots will be aligned. If not provided,\nthe current time will be used.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution\nand up to nine fractional digits.\nExamples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn set_rdb_snapshot_start_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.rdb_snapshot_start_time = Some(v.into());
        self
    }
}
impl ToListMappable for RedisInstancePersistenceConfigEl {
    type O = BlockAssignable<RedisInstancePersistenceConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildRedisInstancePersistenceConfigEl {}
impl BuildRedisInstancePersistenceConfigEl {
    pub fn build(self) -> RedisInstancePersistenceConfigEl {
        RedisInstancePersistenceConfigEl {
            persistence_mode: core::default::Default::default(),
            rdb_snapshot_period: core::default::Default::default(),
            rdb_snapshot_start_time: core::default::Default::default(),
        }
    }
}
pub struct RedisInstancePersistenceConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for RedisInstancePersistenceConfigElRef {
    fn new(shared: StackShared, base: String) -> RedisInstancePersistenceConfigElRef {
        RedisInstancePersistenceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl RedisInstancePersistenceConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `persistence_mode` after provisioning.\nOptional. Controls whether Persistence features are enabled. If not provided, the existing value will be used.\n\n- DISABLED: \tPersistence is disabled for the instance, and any existing snapshots are deleted.\n- RDB: RDB based Persistence is enabled. Possible values: [\"DISABLED\", \"RDB\"]"]
    pub fn persistence_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.persistence_mode", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `rdb_next_snapshot_time` after provisioning.\nOutput only. The next time that a snapshot attempt is scheduled to occur.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up\nto nine fractional digits.\nExamples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn rdb_next_snapshot_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rdb_next_snapshot_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `rdb_snapshot_period` after provisioning.\nOptional. Available snapshot periods for scheduling.\n\n- ONE_HOUR:\tSnapshot every 1 hour.\n- SIX_HOURS:\tSnapshot every 6 hours.\n- TWELVE_HOURS:\tSnapshot every 12 hours.\n- TWENTY_FOUR_HOURS:\tSnapshot every 24 hours. Possible values: [\"ONE_HOUR\", \"SIX_HOURS\", \"TWELVE_HOURS\", \"TWENTY_FOUR_HOURS\"]"]
    pub fn rdb_snapshot_period(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rdb_snapshot_period", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `rdb_snapshot_start_time` after provisioning.\nOptional. Date and time that the first snapshot was/will be attempted,\nand to which future snapshots will be aligned. If not provided,\nthe current time will be used.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution\nand up to nine fractional digits.\nExamples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn rdb_snapshot_start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rdb_snapshot_start_time", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct RedisInstanceTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl RedisInstanceTimeoutsEl {
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
impl ToListMappable for RedisInstanceTimeoutsEl {
    type O = BlockAssignable<RedisInstanceTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildRedisInstanceTimeoutsEl {}
impl BuildRedisInstanceTimeoutsEl {
    pub fn build(self) -> RedisInstanceTimeoutsEl {
        RedisInstanceTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct RedisInstanceTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for RedisInstanceTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> RedisInstanceTimeoutsElRef {
        RedisInstanceTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl RedisInstanceTimeoutsElRef {
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
struct RedisInstanceDynamic {
    maintenance_policy: Option<DynamicBlock<RedisInstanceMaintenancePolicyEl>>,
    persistence_config: Option<DynamicBlock<RedisInstancePersistenceConfigEl>>,
}
