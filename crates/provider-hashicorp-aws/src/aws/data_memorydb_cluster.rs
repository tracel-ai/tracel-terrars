use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataMemorydbClusterData {
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
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
}

struct DataMemorydbCluster_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataMemorydbClusterData>,
}

#[derive(Clone)]
pub struct DataMemorydbCluster(Rc<DataMemorydbCluster_>);

impl DataMemorydbCluster {
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

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `acl_name` after provisioning.\n"]
    pub fn acl_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.acl_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_minor_version_upgrade` after provisioning.\n"]
    pub fn auto_minor_version_upgrade(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_minor_version_upgrade", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_endpoint` after provisioning.\n"]
    pub fn cluster_endpoint(&self) -> ListRef<DataMemorydbClusterClusterEndpointElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cluster_endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_tiering` after provisioning.\n"]
    pub fn data_tiering(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.data_tiering", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `engine` after provisioning.\n"]
    pub fn engine(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.engine", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `engine_patch_version` after provisioning.\n"]
    pub fn engine_patch_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.engine_patch_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `engine_version` after provisioning.\n"]
    pub fn engine_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.engine_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `final_snapshot_name` after provisioning.\n"]
    pub fn final_snapshot_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.final_snapshot_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_window` after provisioning.\n"]
    pub fn maintenance_window(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.maintenance_window", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_type` after provisioning.\n"]
    pub fn node_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.node_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `num_replicas_per_shard` after provisioning.\n"]
    pub fn num_replicas_per_shard(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.num_replicas_per_shard", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `num_shards` after provisioning.\n"]
    pub fn num_shards(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.num_shards", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parameter_group_name` after provisioning.\n"]
    pub fn parameter_group_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.parameter_group_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.security_group_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `shards` after provisioning.\n"]
    pub fn shards(&self) -> SetRef<DataMemorydbClusterShardsElRef> {
        SetRef::new(self.shared().clone(), format!("{}.shards", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `snapshot_retention_limit` after provisioning.\n"]
    pub fn snapshot_retention_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.snapshot_retention_limit", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `snapshot_window` after provisioning.\n"]
    pub fn snapshot_window(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.snapshot_window", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sns_topic_arn` after provisioning.\n"]
    pub fn sns_topic_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sns_topic_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subnet_group_name` after provisioning.\n"]
    pub fn subnet_group_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subnet_group_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tls_enabled` after provisioning.\n"]
    pub fn tls_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.tls_enabled", self.extract_ref()))
    }
}

impl Referable for DataMemorydbCluster {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataMemorydbCluster { }

impl ToListMappable for DataMemorydbCluster {
    type O = ListRef<DataMemorydbClusterRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataMemorydbCluster_ {
    fn extract_datasource_type(&self) -> String {
        "aws_memorydb_cluster".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataMemorydbCluster {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildDataMemorydbCluster {
    pub fn build(self, stack: &mut Stack) -> DataMemorydbCluster {
        let out = DataMemorydbCluster(Rc::new(DataMemorydbCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataMemorydbClusterData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataMemorydbClusterRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataMemorydbClusterRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataMemorydbClusterRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `acl_name` after provisioning.\n"]
    pub fn acl_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.acl_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_minor_version_upgrade` after provisioning.\n"]
    pub fn auto_minor_version_upgrade(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_minor_version_upgrade", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_endpoint` after provisioning.\n"]
    pub fn cluster_endpoint(&self) -> ListRef<DataMemorydbClusterClusterEndpointElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cluster_endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_tiering` after provisioning.\n"]
    pub fn data_tiering(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.data_tiering", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `engine` after provisioning.\n"]
    pub fn engine(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.engine", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `engine_patch_version` after provisioning.\n"]
    pub fn engine_patch_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.engine_patch_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `engine_version` after provisioning.\n"]
    pub fn engine_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.engine_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `final_snapshot_name` after provisioning.\n"]
    pub fn final_snapshot_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.final_snapshot_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_window` after provisioning.\n"]
    pub fn maintenance_window(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.maintenance_window", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_type` after provisioning.\n"]
    pub fn node_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.node_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `num_replicas_per_shard` after provisioning.\n"]
    pub fn num_replicas_per_shard(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.num_replicas_per_shard", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `num_shards` after provisioning.\n"]
    pub fn num_shards(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.num_shards", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parameter_group_name` after provisioning.\n"]
    pub fn parameter_group_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.parameter_group_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.security_group_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `shards` after provisioning.\n"]
    pub fn shards(&self) -> SetRef<DataMemorydbClusterShardsElRef> {
        SetRef::new(self.shared().clone(), format!("{}.shards", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `snapshot_retention_limit` after provisioning.\n"]
    pub fn snapshot_retention_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.snapshot_retention_limit", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `snapshot_window` after provisioning.\n"]
    pub fn snapshot_window(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.snapshot_window", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sns_topic_arn` after provisioning.\n"]
    pub fn sns_topic_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sns_topic_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subnet_group_name` after provisioning.\n"]
    pub fn subnet_group_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subnet_group_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tls_enabled` after provisioning.\n"]
    pub fn tls_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.tls_enabled", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataMemorydbClusterClusterEndpointEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
}

impl DataMemorydbClusterClusterEndpointEl {
    #[doc = "Set the field `address`.\n"]
    pub fn set_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.address = Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
}

impl ToListMappable for DataMemorydbClusterClusterEndpointEl {
    type O = BlockAssignable<DataMemorydbClusterClusterEndpointEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMemorydbClusterClusterEndpointEl {}

impl BuildDataMemorydbClusterClusterEndpointEl {
    pub fn build(self) -> DataMemorydbClusterClusterEndpointEl {
        DataMemorydbClusterClusterEndpointEl {
            address: core::default::Default::default(),
            port: core::default::Default::default(),
        }
    }
}

pub struct DataMemorydbClusterClusterEndpointElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataMemorydbClusterClusterEndpointElRef {
    fn new(shared: StackShared, base: String) -> DataMemorydbClusterClusterEndpointElRef {
        DataMemorydbClusterClusterEndpointElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMemorydbClusterClusterEndpointElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `address` after provisioning.\n"]
    pub fn address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.address", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}

#[derive(Serialize)]
pub struct DataMemorydbClusterShardsElNodesElEndpointEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
}

impl DataMemorydbClusterShardsElNodesElEndpointEl {
    #[doc = "Set the field `address`.\n"]
    pub fn set_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.address = Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
}

impl ToListMappable for DataMemorydbClusterShardsElNodesElEndpointEl {
    type O = BlockAssignable<DataMemorydbClusterShardsElNodesElEndpointEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMemorydbClusterShardsElNodesElEndpointEl {}

impl BuildDataMemorydbClusterShardsElNodesElEndpointEl {
    pub fn build(self) -> DataMemorydbClusterShardsElNodesElEndpointEl {
        DataMemorydbClusterShardsElNodesElEndpointEl {
            address: core::default::Default::default(),
            port: core::default::Default::default(),
        }
    }
}

pub struct DataMemorydbClusterShardsElNodesElEndpointElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataMemorydbClusterShardsElNodesElEndpointElRef {
    fn new(shared: StackShared, base: String) -> DataMemorydbClusterShardsElNodesElEndpointElRef {
        DataMemorydbClusterShardsElNodesElEndpointElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMemorydbClusterShardsElNodesElEndpointElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `address` after provisioning.\n"]
    pub fn address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.address", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}

#[derive(Serialize)]
pub struct DataMemorydbClusterShardsElNodesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    availability_zone: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    create_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    endpoint: Option<ListField<DataMemorydbClusterShardsElNodesElEndpointEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl DataMemorydbClusterShardsElNodesEl {
    #[doc = "Set the field `availability_zone`.\n"]
    pub fn set_availability_zone(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.availability_zone = Some(v.into());
        self
    }

    #[doc = "Set the field `create_time`.\n"]
    pub fn set_create_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create_time = Some(v.into());
        self
    }

    #[doc = "Set the field `endpoint`.\n"]
    pub fn set_endpoint(mut self, v: impl Into<ListField<DataMemorydbClusterShardsElNodesElEndpointEl>>) -> Self {
        self.endpoint = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}

impl ToListMappable for DataMemorydbClusterShardsElNodesEl {
    type O = BlockAssignable<DataMemorydbClusterShardsElNodesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMemorydbClusterShardsElNodesEl {}

impl BuildDataMemorydbClusterShardsElNodesEl {
    pub fn build(self) -> DataMemorydbClusterShardsElNodesEl {
        DataMemorydbClusterShardsElNodesEl {
            availability_zone: core::default::Default::default(),
            create_time: core::default::Default::default(),
            endpoint: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct DataMemorydbClusterShardsElNodesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataMemorydbClusterShardsElNodesElRef {
    fn new(shared: StackShared, base: String) -> DataMemorydbClusterShardsElNodesElRef {
        DataMemorydbClusterShardsElNodesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMemorydbClusterShardsElNodesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `availability_zone` after provisioning.\n"]
    pub fn availability_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.availability_zone", self.base))
    }

    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.base))
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> ListRef<DataMemorydbClusterShardsElNodesElEndpointElRef> {
        ListRef::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataMemorydbClusterShardsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nodes: Option<SetField<DataMemorydbClusterShardsElNodesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    num_nodes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    slots: Option<PrimField<String>>,
}

impl DataMemorydbClusterShardsEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc = "Set the field `nodes`.\n"]
    pub fn set_nodes(mut self, v: impl Into<SetField<DataMemorydbClusterShardsElNodesEl>>) -> Self {
        self.nodes = Some(v.into());
        self
    }

    #[doc = "Set the field `num_nodes`.\n"]
    pub fn set_num_nodes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.num_nodes = Some(v.into());
        self
    }

    #[doc = "Set the field `slots`.\n"]
    pub fn set_slots(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.slots = Some(v.into());
        self
    }
}

impl ToListMappable for DataMemorydbClusterShardsEl {
    type O = BlockAssignable<DataMemorydbClusterShardsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMemorydbClusterShardsEl {}

impl BuildDataMemorydbClusterShardsEl {
    pub fn build(self) -> DataMemorydbClusterShardsEl {
        DataMemorydbClusterShardsEl {
            name: core::default::Default::default(),
            nodes: core::default::Default::default(),
            num_nodes: core::default::Default::default(),
            slots: core::default::Default::default(),
        }
    }
}

pub struct DataMemorydbClusterShardsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataMemorydbClusterShardsElRef {
    fn new(shared: StackShared, base: String) -> DataMemorydbClusterShardsElRef {
        DataMemorydbClusterShardsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMemorydbClusterShardsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `nodes` after provisioning.\n"]
    pub fn nodes(&self) -> SetRef<DataMemorydbClusterShardsElNodesElRef> {
        SetRef::new(self.shared().clone(), format!("{}.nodes", self.base))
    }

    #[doc = "Get a reference to the value of field `num_nodes` after provisioning.\n"]
    pub fn num_nodes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.num_nodes", self.base))
    }

    #[doc = "Get a reference to the value of field `slots` after provisioning.\n"]
    pub fn slots(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.slots", self.base))
    }
}
