use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataElasticacheServerlessCacheData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}
struct DataElasticacheServerlessCache_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataElasticacheServerlessCacheData>,
}
#[derive(Clone)]
pub struct DataElasticacheServerlessCache(Rc<DataElasticacheServerlessCache_>);
impl DataElasticacheServerlessCache {
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
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `cache_usage_limits` after provisioning.\n"]
    pub fn cache_usage_limits(&self) -> DataElasticacheServerlessCacheCacheUsageLimitsRef {
        DataElasticacheServerlessCacheCacheUsageLimitsRef::new(
            self.shared().clone(),
            format!("{}.cache_usage_limits", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `daily_snapshot_time` after provisioning.\n"]
    pub fn daily_snapshot_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.daily_snapshot_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> DataElasticacheServerlessCacheEndpointRef {
        DataElasticacheServerlessCacheEndpointRef::new(
            self.shared().clone(),
            format!("{}.endpoint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `engine` after provisioning.\n"]
    pub fn engine(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.engine", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `full_engine_version` after provisioning.\n"]
    pub fn full_engine_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.full_engine_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `major_engine_version` after provisioning.\n"]
    pub fn major_engine_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.major_engine_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reader_endpoint` after provisioning.\n"]
    pub fn reader_endpoint(&self) -> DataElasticacheServerlessCacheReaderEndpointRef {
        DataElasticacheServerlessCacheReaderEndpointRef::new(
            self.shared().clone(),
            format!("{}.reader_endpoint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.security_group_ids", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `snapshot_retention_limit` after provisioning.\n"]
    pub fn snapshot_retention_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.snapshot_retention_limit", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.subnet_ids", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `user_group_id` after provisioning.\n"]
    pub fn user_group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_group_id", self.extract_ref()),
        )
    }
}
impl Referable for DataElasticacheServerlessCache {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataElasticacheServerlessCache {}
impl ToListMappable for DataElasticacheServerlessCache {
    type O = ListRef<DataElasticacheServerlessCacheRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataElasticacheServerlessCache_ {
    fn extract_datasource_type(&self) -> String {
        "aws_elasticache_serverless_cache".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataElasticacheServerlessCache {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
}
impl BuildDataElasticacheServerlessCache {
    pub fn build(self, stack: &mut Stack) -> DataElasticacheServerlessCache {
        let out = DataElasticacheServerlessCache(Rc::new(DataElasticacheServerlessCache_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataElasticacheServerlessCacheData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                name: self.name,
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataElasticacheServerlessCacheRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataElasticacheServerlessCacheRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataElasticacheServerlessCacheRef {
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
    #[doc = "Get a reference to the value of field `cache_usage_limits` after provisioning.\n"]
    pub fn cache_usage_limits(&self) -> DataElasticacheServerlessCacheCacheUsageLimitsRef {
        DataElasticacheServerlessCacheCacheUsageLimitsRef::new(
            self.shared().clone(),
            format!("{}.cache_usage_limits", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `daily_snapshot_time` after provisioning.\n"]
    pub fn daily_snapshot_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.daily_snapshot_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> DataElasticacheServerlessCacheEndpointRef {
        DataElasticacheServerlessCacheEndpointRef::new(
            self.shared().clone(),
            format!("{}.endpoint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `engine` after provisioning.\n"]
    pub fn engine(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.engine", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `full_engine_version` after provisioning.\n"]
    pub fn full_engine_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.full_engine_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `major_engine_version` after provisioning.\n"]
    pub fn major_engine_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.major_engine_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reader_endpoint` after provisioning.\n"]
    pub fn reader_endpoint(&self) -> DataElasticacheServerlessCacheReaderEndpointRef {
        DataElasticacheServerlessCacheReaderEndpointRef::new(
            self.shared().clone(),
            format!("{}.reader_endpoint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.security_group_ids", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `snapshot_retention_limit` after provisioning.\n"]
    pub fn snapshot_retention_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.snapshot_retention_limit", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.subnet_ids", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `user_group_id` after provisioning.\n"]
    pub fn user_group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_group_id", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataElasticacheServerlessCacheCacheUsageLimitsDataStorage {
    #[serde(skip_serializing_if = "Option::is_none")]
    maximum: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minimum: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unit: Option<PrimField<String>>,
}
impl DataElasticacheServerlessCacheCacheUsageLimitsDataStorage {
    #[doc = "Set the field `maximum`.\n"]
    pub fn set_maximum(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.maximum = Some(v.into());
        self
    }
    #[doc = "Set the field `minimum`.\n"]
    pub fn set_minimum(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.minimum = Some(v.into());
        self
    }
    #[doc = "Set the field `unit`.\n"]
    pub fn set_unit(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.unit = Some(v.into());
        self
    }
}
impl ToListMappable for DataElasticacheServerlessCacheCacheUsageLimitsDataStorage {
    type O = BlockAssignable<DataElasticacheServerlessCacheCacheUsageLimitsDataStorage>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataElasticacheServerlessCacheCacheUsageLimitsDataStorage {}
impl BuildDataElasticacheServerlessCacheCacheUsageLimitsDataStorage {
    pub fn build(self) -> DataElasticacheServerlessCacheCacheUsageLimitsDataStorage {
        DataElasticacheServerlessCacheCacheUsageLimitsDataStorage {
            maximum: core::default::Default::default(),
            minimum: core::default::Default::default(),
            unit: core::default::Default::default(),
        }
    }
}
pub struct DataElasticacheServerlessCacheCacheUsageLimitsDataStorageRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataElasticacheServerlessCacheCacheUsageLimitsDataStorageRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataElasticacheServerlessCacheCacheUsageLimitsDataStorageRef {
        DataElasticacheServerlessCacheCacheUsageLimitsDataStorageRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataElasticacheServerlessCacheCacheUsageLimitsDataStorageRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `maximum` after provisioning.\n"]
    pub fn maximum(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.maximum", self.base))
    }
    #[doc = "Get a reference to the value of field `minimum` after provisioning.\n"]
    pub fn minimum(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.minimum", self.base))
    }
    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }
}
#[derive(Serialize)]
pub struct DataElasticacheServerlessCacheCacheUsageLimitsEcpuPerSecond {
    #[serde(skip_serializing_if = "Option::is_none")]
    maximum: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minimum: Option<PrimField<f64>>,
}
impl DataElasticacheServerlessCacheCacheUsageLimitsEcpuPerSecond {
    #[doc = "Set the field `maximum`.\n"]
    pub fn set_maximum(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.maximum = Some(v.into());
        self
    }
    #[doc = "Set the field `minimum`.\n"]
    pub fn set_minimum(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.minimum = Some(v.into());
        self
    }
}
impl ToListMappable for DataElasticacheServerlessCacheCacheUsageLimitsEcpuPerSecond {
    type O = BlockAssignable<DataElasticacheServerlessCacheCacheUsageLimitsEcpuPerSecond>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataElasticacheServerlessCacheCacheUsageLimitsEcpuPerSecond {}
impl BuildDataElasticacheServerlessCacheCacheUsageLimitsEcpuPerSecond {
    pub fn build(self) -> DataElasticacheServerlessCacheCacheUsageLimitsEcpuPerSecond {
        DataElasticacheServerlessCacheCacheUsageLimitsEcpuPerSecond {
            maximum: core::default::Default::default(),
            minimum: core::default::Default::default(),
        }
    }
}
pub struct DataElasticacheServerlessCacheCacheUsageLimitsEcpuPerSecondRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataElasticacheServerlessCacheCacheUsageLimitsEcpuPerSecondRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataElasticacheServerlessCacheCacheUsageLimitsEcpuPerSecondRef {
        DataElasticacheServerlessCacheCacheUsageLimitsEcpuPerSecondRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataElasticacheServerlessCacheCacheUsageLimitsEcpuPerSecondRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `maximum` after provisioning.\n"]
    pub fn maximum(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.maximum", self.base))
    }
    #[doc = "Get a reference to the value of field `minimum` after provisioning.\n"]
    pub fn minimum(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.minimum", self.base))
    }
}
#[derive(Serialize)]
pub struct DataElasticacheServerlessCacheCacheUsageLimits {
    #[serde(skip_serializing_if = "Option::is_none")]
    data_storage: Option<DataElasticacheServerlessCacheCacheUsageLimitsDataStorage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ecpu_per_second: Option<DataElasticacheServerlessCacheCacheUsageLimitsEcpuPerSecond>,
}
impl DataElasticacheServerlessCacheCacheUsageLimits {
    #[doc = "Set the field `data_storage`.\n"]
    pub fn set_data_storage(
        mut self,
        v: impl Into<DataElasticacheServerlessCacheCacheUsageLimitsDataStorage>,
    ) -> Self {
        self.data_storage = Some(v.into());
        self
    }
    #[doc = "Set the field `ecpu_per_second`.\n"]
    pub fn set_ecpu_per_second(
        mut self,
        v: impl Into<DataElasticacheServerlessCacheCacheUsageLimitsEcpuPerSecond>,
    ) -> Self {
        self.ecpu_per_second = Some(v.into());
        self
    }
}
impl ToListMappable for DataElasticacheServerlessCacheCacheUsageLimits {
    type O = BlockAssignable<DataElasticacheServerlessCacheCacheUsageLimits>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataElasticacheServerlessCacheCacheUsageLimits {}
impl BuildDataElasticacheServerlessCacheCacheUsageLimits {
    pub fn build(self) -> DataElasticacheServerlessCacheCacheUsageLimits {
        DataElasticacheServerlessCacheCacheUsageLimits {
            data_storage: core::default::Default::default(),
            ecpu_per_second: core::default::Default::default(),
        }
    }
}
pub struct DataElasticacheServerlessCacheCacheUsageLimitsRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataElasticacheServerlessCacheCacheUsageLimitsRef {
    fn new(shared: StackShared, base: String) -> DataElasticacheServerlessCacheCacheUsageLimitsRef {
        DataElasticacheServerlessCacheCacheUsageLimitsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataElasticacheServerlessCacheCacheUsageLimitsRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `data_storage` after provisioning.\n"]
    pub fn data_storage(&self) -> DataElasticacheServerlessCacheCacheUsageLimitsDataStorageRef {
        DataElasticacheServerlessCacheCacheUsageLimitsDataStorageRef::new(
            self.shared().clone(),
            format!("{}.data_storage", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ecpu_per_second` after provisioning.\n"]
    pub fn ecpu_per_second(
        &self,
    ) -> DataElasticacheServerlessCacheCacheUsageLimitsEcpuPerSecondRef {
        DataElasticacheServerlessCacheCacheUsageLimitsEcpuPerSecondRef::new(
            self.shared().clone(),
            format!("{}.ecpu_per_second", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataElasticacheServerlessCacheEndpoint {
    #[serde(skip_serializing_if = "Option::is_none")]
    address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
}
impl DataElasticacheServerlessCacheEndpoint {
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
impl ToListMappable for DataElasticacheServerlessCacheEndpoint {
    type O = BlockAssignable<DataElasticacheServerlessCacheEndpoint>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataElasticacheServerlessCacheEndpoint {}
impl BuildDataElasticacheServerlessCacheEndpoint {
    pub fn build(self) -> DataElasticacheServerlessCacheEndpoint {
        DataElasticacheServerlessCacheEndpoint {
            address: core::default::Default::default(),
            port: core::default::Default::default(),
        }
    }
}
pub struct DataElasticacheServerlessCacheEndpointRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataElasticacheServerlessCacheEndpointRef {
    fn new(shared: StackShared, base: String) -> DataElasticacheServerlessCacheEndpointRef {
        DataElasticacheServerlessCacheEndpointRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataElasticacheServerlessCacheEndpointRef {
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
pub struct DataElasticacheServerlessCacheReaderEndpoint {
    #[serde(skip_serializing_if = "Option::is_none")]
    address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
}
impl DataElasticacheServerlessCacheReaderEndpoint {
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
impl ToListMappable for DataElasticacheServerlessCacheReaderEndpoint {
    type O = BlockAssignable<DataElasticacheServerlessCacheReaderEndpoint>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataElasticacheServerlessCacheReaderEndpoint {}
impl BuildDataElasticacheServerlessCacheReaderEndpoint {
    pub fn build(self) -> DataElasticacheServerlessCacheReaderEndpoint {
        DataElasticacheServerlessCacheReaderEndpoint {
            address: core::default::Default::default(),
            port: core::default::Default::default(),
        }
    }
}
pub struct DataElasticacheServerlessCacheReaderEndpointRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataElasticacheServerlessCacheReaderEndpointRef {
    fn new(shared: StackShared, base: String) -> DataElasticacheServerlessCacheReaderEndpointRef {
        DataElasticacheServerlessCacheReaderEndpointRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataElasticacheServerlessCacheReaderEndpointRef {
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
