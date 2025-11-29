use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataElasticacheReplicationGroupData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    replication_group_id: PrimField<String>,
}

struct DataElasticacheReplicationGroup_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataElasticacheReplicationGroupData>,
}

#[derive(Clone)]
pub struct DataElasticacheReplicationGroup(Rc<DataElasticacheReplicationGroup_>);

impl DataElasticacheReplicationGroup {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderAws) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auth_token_enabled` after provisioning.\n"]
    pub fn auth_token_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.auth_token_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `automatic_failover_enabled` after provisioning.\n"]
    pub fn automatic_failover_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.automatic_failover_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_mode` after provisioning.\n"]
    pub fn cluster_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configuration_endpoint_address` after provisioning.\n"]
    pub fn configuration_endpoint_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.configuration_endpoint_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_delivery_configuration` after provisioning.\n"]
    pub fn log_delivery_configuration(&self) -> SetRef<DataElasticacheReplicationGroupLogDeliveryConfigurationElRef> {
        SetRef::new(self.shared().clone(), format!("{}.log_delivery_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `member_clusters` after provisioning.\n"]
    pub fn member_clusters(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.member_clusters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `multi_az_enabled` after provisioning.\n"]
    pub fn multi_az_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.multi_az_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_group_configuration` after provisioning.\n"]
    pub fn node_group_configuration(&self) -> SetRef<DataElasticacheReplicationGroupNodeGroupConfigurationElRef> {
        SetRef::new(self.shared().clone(), format!("{}.node_group_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_type` after provisioning.\n"]
    pub fn node_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.node_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `num_cache_clusters` after provisioning.\n"]
    pub fn num_cache_clusters(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.num_cache_clusters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `num_node_groups` after provisioning.\n"]
    pub fn num_node_groups(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.num_node_groups", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `primary_endpoint_address` after provisioning.\n"]
    pub fn primary_endpoint_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.primary_endpoint_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `reader_endpoint_address` after provisioning.\n"]
    pub fn reader_endpoint_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.reader_endpoint_address", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `replicas_per_node_group` after provisioning.\n"]
    pub fn replicas_per_node_group(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.replicas_per_node_group", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `replication_group_id` after provisioning.\n"]
    pub fn replication_group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.replication_group_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `snapshot_retention_limit` after provisioning.\n"]
    pub fn snapshot_retention_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.snapshot_retention_limit", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `snapshot_window` after provisioning.\n"]
    pub fn snapshot_window(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.snapshot_window", self.extract_ref()))
    }
}

impl Referable for DataElasticacheReplicationGroup {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataElasticacheReplicationGroup { }

impl ToListMappable for DataElasticacheReplicationGroup {
    type O = ListRef<DataElasticacheReplicationGroupRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataElasticacheReplicationGroup_ {
    fn extract_datasource_type(&self) -> String {
        "aws_elasticache_replication_group".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataElasticacheReplicationGroup {
    pub tf_id: String,
    #[doc = ""]
    pub replication_group_id: PrimField<String>,
}

impl BuildDataElasticacheReplicationGroup {
    pub fn build(self, stack: &mut Stack) -> DataElasticacheReplicationGroup {
        let out = DataElasticacheReplicationGroup(Rc::new(DataElasticacheReplicationGroup_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataElasticacheReplicationGroupData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                replication_group_id: self.replication_group_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataElasticacheReplicationGroupRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataElasticacheReplicationGroupRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataElasticacheReplicationGroupRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auth_token_enabled` after provisioning.\n"]
    pub fn auth_token_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.auth_token_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `automatic_failover_enabled` after provisioning.\n"]
    pub fn automatic_failover_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.automatic_failover_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_mode` after provisioning.\n"]
    pub fn cluster_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configuration_endpoint_address` after provisioning.\n"]
    pub fn configuration_endpoint_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.configuration_endpoint_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_delivery_configuration` after provisioning.\n"]
    pub fn log_delivery_configuration(&self) -> SetRef<DataElasticacheReplicationGroupLogDeliveryConfigurationElRef> {
        SetRef::new(self.shared().clone(), format!("{}.log_delivery_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `member_clusters` after provisioning.\n"]
    pub fn member_clusters(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.member_clusters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `multi_az_enabled` after provisioning.\n"]
    pub fn multi_az_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.multi_az_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_group_configuration` after provisioning.\n"]
    pub fn node_group_configuration(&self) -> SetRef<DataElasticacheReplicationGroupNodeGroupConfigurationElRef> {
        SetRef::new(self.shared().clone(), format!("{}.node_group_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_type` after provisioning.\n"]
    pub fn node_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.node_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `num_cache_clusters` after provisioning.\n"]
    pub fn num_cache_clusters(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.num_cache_clusters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `num_node_groups` after provisioning.\n"]
    pub fn num_node_groups(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.num_node_groups", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `primary_endpoint_address` after provisioning.\n"]
    pub fn primary_endpoint_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.primary_endpoint_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `reader_endpoint_address` after provisioning.\n"]
    pub fn reader_endpoint_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.reader_endpoint_address", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `replicas_per_node_group` after provisioning.\n"]
    pub fn replicas_per_node_group(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.replicas_per_node_group", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `replication_group_id` after provisioning.\n"]
    pub fn replication_group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.replication_group_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `snapshot_retention_limit` after provisioning.\n"]
    pub fn snapshot_retention_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.snapshot_retention_limit", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `snapshot_window` after provisioning.\n"]
    pub fn snapshot_window(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.snapshot_window", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataElasticacheReplicationGroupLogDeliveryConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    destination: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_format: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_type: Option<PrimField<String>>,
}

impl DataElasticacheReplicationGroupLogDeliveryConfigurationEl {
    #[doc = "Set the field `destination`.\n"]
    pub fn set_destination(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.destination = Some(v.into());
        self
    }

    #[doc = "Set the field `destination_type`.\n"]
    pub fn set_destination_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.destination_type = Some(v.into());
        self
    }

    #[doc = "Set the field `log_format`.\n"]
    pub fn set_log_format(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.log_format = Some(v.into());
        self
    }

    #[doc = "Set the field `log_type`.\n"]
    pub fn set_log_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.log_type = Some(v.into());
        self
    }
}

impl ToListMappable for DataElasticacheReplicationGroupLogDeliveryConfigurationEl {
    type O = BlockAssignable<DataElasticacheReplicationGroupLogDeliveryConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataElasticacheReplicationGroupLogDeliveryConfigurationEl {}

impl BuildDataElasticacheReplicationGroupLogDeliveryConfigurationEl {
    pub fn build(self) -> DataElasticacheReplicationGroupLogDeliveryConfigurationEl {
        DataElasticacheReplicationGroupLogDeliveryConfigurationEl {
            destination: core::default::Default::default(),
            destination_type: core::default::Default::default(),
            log_format: core::default::Default::default(),
            log_type: core::default::Default::default(),
        }
    }
}

pub struct DataElasticacheReplicationGroupLogDeliveryConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataElasticacheReplicationGroupLogDeliveryConfigurationElRef {
    fn new(shared: StackShared, base: String) -> DataElasticacheReplicationGroupLogDeliveryConfigurationElRef {
        DataElasticacheReplicationGroupLogDeliveryConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataElasticacheReplicationGroupLogDeliveryConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.destination", self.base))
    }

    #[doc = "Get a reference to the value of field `destination_type` after provisioning.\n"]
    pub fn destination_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.destination_type", self.base))
    }

    #[doc = "Get a reference to the value of field `log_format` after provisioning.\n"]
    pub fn log_format(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_format", self.base))
    }

    #[doc = "Get a reference to the value of field `log_type` after provisioning.\n"]
    pub fn log_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataElasticacheReplicationGroupNodeGroupConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    node_group_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    primary_availability_zone: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    primary_outpost_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    replica_availability_zones: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    replica_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    replica_outpost_arns: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    slots: Option<PrimField<String>>,
}

impl DataElasticacheReplicationGroupNodeGroupConfigurationEl {
    #[doc = "Set the field `node_group_id`.\n"]
    pub fn set_node_group_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.node_group_id = Some(v.into());
        self
    }

    #[doc = "Set the field `primary_availability_zone`.\n"]
    pub fn set_primary_availability_zone(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.primary_availability_zone = Some(v.into());
        self
    }

    #[doc = "Set the field `primary_outpost_arn`.\n"]
    pub fn set_primary_outpost_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.primary_outpost_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `replica_availability_zones`.\n"]
    pub fn set_replica_availability_zones(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.replica_availability_zones = Some(v.into());
        self
    }

    #[doc = "Set the field `replica_count`.\n"]
    pub fn set_replica_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.replica_count = Some(v.into());
        self
    }

    #[doc = "Set the field `replica_outpost_arns`.\n"]
    pub fn set_replica_outpost_arns(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.replica_outpost_arns = Some(v.into());
        self
    }

    #[doc = "Set the field `slots`.\n"]
    pub fn set_slots(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.slots = Some(v.into());
        self
    }
}

impl ToListMappable for DataElasticacheReplicationGroupNodeGroupConfigurationEl {
    type O = BlockAssignable<DataElasticacheReplicationGroupNodeGroupConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataElasticacheReplicationGroupNodeGroupConfigurationEl {}

impl BuildDataElasticacheReplicationGroupNodeGroupConfigurationEl {
    pub fn build(self) -> DataElasticacheReplicationGroupNodeGroupConfigurationEl {
        DataElasticacheReplicationGroupNodeGroupConfigurationEl {
            node_group_id: core::default::Default::default(),
            primary_availability_zone: core::default::Default::default(),
            primary_outpost_arn: core::default::Default::default(),
            replica_availability_zones: core::default::Default::default(),
            replica_count: core::default::Default::default(),
            replica_outpost_arns: core::default::Default::default(),
            slots: core::default::Default::default(),
        }
    }
}

pub struct DataElasticacheReplicationGroupNodeGroupConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataElasticacheReplicationGroupNodeGroupConfigurationElRef {
    fn new(shared: StackShared, base: String) -> DataElasticacheReplicationGroupNodeGroupConfigurationElRef {
        DataElasticacheReplicationGroupNodeGroupConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataElasticacheReplicationGroupNodeGroupConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `node_group_id` after provisioning.\n"]
    pub fn node_group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.node_group_id", self.base))
    }

    #[doc = "Get a reference to the value of field `primary_availability_zone` after provisioning.\n"]
    pub fn primary_availability_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.primary_availability_zone", self.base))
    }

    #[doc = "Get a reference to the value of field `primary_outpost_arn` after provisioning.\n"]
    pub fn primary_outpost_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.primary_outpost_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `replica_availability_zones` after provisioning.\n"]
    pub fn replica_availability_zones(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.replica_availability_zones", self.base))
    }

    #[doc = "Get a reference to the value of field `replica_count` after provisioning.\n"]
    pub fn replica_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.replica_count", self.base))
    }

    #[doc = "Get a reference to the value of field `replica_outpost_arns` after provisioning.\n"]
    pub fn replica_outpost_arns(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.replica_outpost_arns", self.base))
    }

    #[doc = "Get a reference to the value of field `slots` after provisioning.\n"]
    pub fn slots(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.slots", self.base))
    }
}
