use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct ElasticacheServerlessCacheData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    daily_snapshot_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    engine: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    major_engine_version: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_group_ids: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    snapshot_arns_to_restore: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    snapshot_retention_limit: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnet_ids: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_group_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_usage_limits: Option<Vec<ElasticacheServerlessCacheCacheUsageLimitsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ElasticacheServerlessCacheTimeoutsEl>,
    dynamic: ElasticacheServerlessCacheDynamic,
}

struct ElasticacheServerlessCache_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ElasticacheServerlessCacheData>,
}

#[derive(Clone)]
pub struct ElasticacheServerlessCache(Rc<ElasticacheServerlessCache_>);

impl ElasticacheServerlessCache {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderAws) -> Self {
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
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
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc = "Set the field `daily_snapshot_time`.\n"]
    pub fn set_daily_snapshot_time(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().daily_snapshot_time = Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().kms_key_id = Some(v.into());
        self
    }

    #[doc = "Set the field `major_engine_version`.\n"]
    pub fn set_major_engine_version(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().major_engine_version = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `security_group_ids`.\n"]
    pub fn set_security_group_ids(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().security_group_ids = Some(v.into());
        self
    }

    #[doc = "Set the field `snapshot_arns_to_restore`.\n"]
    pub fn set_snapshot_arns_to_restore(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().snapshot_arns_to_restore = Some(v.into());
        self
    }

    #[doc = "Set the field `snapshot_retention_limit`.\n"]
    pub fn set_snapshot_retention_limit(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().snapshot_retention_limit = Some(v.into());
        self
    }

    #[doc = "Set the field `subnet_ids`.\n"]
    pub fn set_subnet_ids(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().subnet_ids = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc = "Set the field `user_group_id`.\n"]
    pub fn set_user_group_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().user_group_id = Some(v.into());
        self
    }

    #[doc = "Set the field `cache_usage_limits`.\n"]
    pub fn set_cache_usage_limits(
        self,
        v: impl Into<BlockAssignable<ElasticacheServerlessCacheCacheUsageLimitsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cache_usage_limits = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cache_usage_limits = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ElasticacheServerlessCacheTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `daily_snapshot_time` after provisioning.\n"]
    pub fn daily_snapshot_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.daily_snapshot_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> ListRef<ElasticacheServerlessCacheEndpointElRef> {
        ListRef::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `engine` after provisioning.\n"]
    pub fn engine(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.engine", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `full_engine_version` after provisioning.\n"]
    pub fn full_engine_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.full_engine_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `major_engine_version` after provisioning.\n"]
    pub fn major_engine_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.major_engine_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `reader_endpoint` after provisioning.\n"]
    pub fn reader_endpoint(&self) -> ListRef<ElasticacheServerlessCacheReaderEndpointElRef> {
        ListRef::new(self.shared().clone(), format!("{}.reader_endpoint", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `snapshot_arns_to_restore` after provisioning.\n"]
    pub fn snapshot_arns_to_restore(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.snapshot_arns_to_restore", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `snapshot_retention_limit` after provisioning.\n"]
    pub fn snapshot_retention_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.snapshot_retention_limit", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_group_id` after provisioning.\n"]
    pub fn user_group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_group_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cache_usage_limits` after provisioning.\n"]
    pub fn cache_usage_limits(&self) -> ListRef<ElasticacheServerlessCacheCacheUsageLimitsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cache_usage_limits", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ElasticacheServerlessCacheTimeoutsElRef {
        ElasticacheServerlessCacheTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for ElasticacheServerlessCache {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for ElasticacheServerlessCache { }

impl ToListMappable for ElasticacheServerlessCache {
    type O = ListRef<ElasticacheServerlessCacheRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ElasticacheServerlessCache_ {
    fn extract_resource_type(&self) -> String {
        "aws_elasticache_serverless_cache".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildElasticacheServerlessCache {
    pub tf_id: String,
    #[doc = ""]
    pub engine: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildElasticacheServerlessCache {
    pub fn build(self, stack: &mut Stack) -> ElasticacheServerlessCache {
        let out = ElasticacheServerlessCache(Rc::new(ElasticacheServerlessCache_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ElasticacheServerlessCacheData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                daily_snapshot_time: core::default::Default::default(),
                description: core::default::Default::default(),
                engine: self.engine,
                kms_key_id: core::default::Default::default(),
                major_engine_version: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                security_group_ids: core::default::Default::default(),
                snapshot_arns_to_restore: core::default::Default::default(),
                snapshot_retention_limit: core::default::Default::default(),
                subnet_ids: core::default::Default::default(),
                tags: core::default::Default::default(),
                user_group_id: core::default::Default::default(),
                cache_usage_limits: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ElasticacheServerlessCacheRef {
    shared: StackShared,
    base: String,
}

impl Ref for ElasticacheServerlessCacheRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl ElasticacheServerlessCacheRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `daily_snapshot_time` after provisioning.\n"]
    pub fn daily_snapshot_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.daily_snapshot_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> ListRef<ElasticacheServerlessCacheEndpointElRef> {
        ListRef::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `engine` after provisioning.\n"]
    pub fn engine(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.engine", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `full_engine_version` after provisioning.\n"]
    pub fn full_engine_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.full_engine_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `major_engine_version` after provisioning.\n"]
    pub fn major_engine_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.major_engine_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `reader_endpoint` after provisioning.\n"]
    pub fn reader_endpoint(&self) -> ListRef<ElasticacheServerlessCacheReaderEndpointElRef> {
        ListRef::new(self.shared().clone(), format!("{}.reader_endpoint", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `snapshot_arns_to_restore` after provisioning.\n"]
    pub fn snapshot_arns_to_restore(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.snapshot_arns_to_restore", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `snapshot_retention_limit` after provisioning.\n"]
    pub fn snapshot_retention_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.snapshot_retention_limit", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_group_id` after provisioning.\n"]
    pub fn user_group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_group_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cache_usage_limits` after provisioning.\n"]
    pub fn cache_usage_limits(&self) -> ListRef<ElasticacheServerlessCacheCacheUsageLimitsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cache_usage_limits", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ElasticacheServerlessCacheTimeoutsElRef {
        ElasticacheServerlessCacheTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct ElasticacheServerlessCacheEndpointEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
}

impl ElasticacheServerlessCacheEndpointEl {
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

impl ToListMappable for ElasticacheServerlessCacheEndpointEl {
    type O = BlockAssignable<ElasticacheServerlessCacheEndpointEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildElasticacheServerlessCacheEndpointEl {}

impl BuildElasticacheServerlessCacheEndpointEl {
    pub fn build(self) -> ElasticacheServerlessCacheEndpointEl {
        ElasticacheServerlessCacheEndpointEl {
            address: core::default::Default::default(),
            port: core::default::Default::default(),
        }
    }
}

pub struct ElasticacheServerlessCacheEndpointElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ElasticacheServerlessCacheEndpointElRef {
    fn new(shared: StackShared, base: String) -> ElasticacheServerlessCacheEndpointElRef {
        ElasticacheServerlessCacheEndpointElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ElasticacheServerlessCacheEndpointElRef {
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
pub struct ElasticacheServerlessCacheReaderEndpointEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
}

impl ElasticacheServerlessCacheReaderEndpointEl {
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

impl ToListMappable for ElasticacheServerlessCacheReaderEndpointEl {
    type O = BlockAssignable<ElasticacheServerlessCacheReaderEndpointEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildElasticacheServerlessCacheReaderEndpointEl {}

impl BuildElasticacheServerlessCacheReaderEndpointEl {
    pub fn build(self) -> ElasticacheServerlessCacheReaderEndpointEl {
        ElasticacheServerlessCacheReaderEndpointEl {
            address: core::default::Default::default(),
            port: core::default::Default::default(),
        }
    }
}

pub struct ElasticacheServerlessCacheReaderEndpointElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ElasticacheServerlessCacheReaderEndpointElRef {
    fn new(shared: StackShared, base: String) -> ElasticacheServerlessCacheReaderEndpointElRef {
        ElasticacheServerlessCacheReaderEndpointElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ElasticacheServerlessCacheReaderEndpointElRef {
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
pub struct ElasticacheServerlessCacheCacheUsageLimitsElDataStorageEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    maximum: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minimum: Option<PrimField<f64>>,
    unit: PrimField<String>,
}

impl ElasticacheServerlessCacheCacheUsageLimitsElDataStorageEl {
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

impl ToListMappable for ElasticacheServerlessCacheCacheUsageLimitsElDataStorageEl {
    type O = BlockAssignable<ElasticacheServerlessCacheCacheUsageLimitsElDataStorageEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildElasticacheServerlessCacheCacheUsageLimitsElDataStorageEl {
    #[doc = ""]
    pub unit: PrimField<String>,
}

impl BuildElasticacheServerlessCacheCacheUsageLimitsElDataStorageEl {
    pub fn build(self) -> ElasticacheServerlessCacheCacheUsageLimitsElDataStorageEl {
        ElasticacheServerlessCacheCacheUsageLimitsElDataStorageEl {
            maximum: core::default::Default::default(),
            minimum: core::default::Default::default(),
            unit: self.unit,
        }
    }
}

pub struct ElasticacheServerlessCacheCacheUsageLimitsElDataStorageElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ElasticacheServerlessCacheCacheUsageLimitsElDataStorageElRef {
    fn new(shared: StackShared, base: String) -> ElasticacheServerlessCacheCacheUsageLimitsElDataStorageElRef {
        ElasticacheServerlessCacheCacheUsageLimitsElDataStorageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ElasticacheServerlessCacheCacheUsageLimitsElDataStorageElRef {
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
pub struct ElasticacheServerlessCacheCacheUsageLimitsElEcpuPerSecondEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    maximum: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minimum: Option<PrimField<f64>>,
}

impl ElasticacheServerlessCacheCacheUsageLimitsElEcpuPerSecondEl {
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

impl ToListMappable for ElasticacheServerlessCacheCacheUsageLimitsElEcpuPerSecondEl {
    type O = BlockAssignable<ElasticacheServerlessCacheCacheUsageLimitsElEcpuPerSecondEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildElasticacheServerlessCacheCacheUsageLimitsElEcpuPerSecondEl {}

impl BuildElasticacheServerlessCacheCacheUsageLimitsElEcpuPerSecondEl {
    pub fn build(self) -> ElasticacheServerlessCacheCacheUsageLimitsElEcpuPerSecondEl {
        ElasticacheServerlessCacheCacheUsageLimitsElEcpuPerSecondEl {
            maximum: core::default::Default::default(),
            minimum: core::default::Default::default(),
        }
    }
}

pub struct ElasticacheServerlessCacheCacheUsageLimitsElEcpuPerSecondElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ElasticacheServerlessCacheCacheUsageLimitsElEcpuPerSecondElRef {
    fn new(shared: StackShared, base: String) -> ElasticacheServerlessCacheCacheUsageLimitsElEcpuPerSecondElRef {
        ElasticacheServerlessCacheCacheUsageLimitsElEcpuPerSecondElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ElasticacheServerlessCacheCacheUsageLimitsElEcpuPerSecondElRef {
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

#[derive(Serialize, Default)]
struct ElasticacheServerlessCacheCacheUsageLimitsElDynamic {
    data_storage: Option<DynamicBlock<ElasticacheServerlessCacheCacheUsageLimitsElDataStorageEl>>,
    ecpu_per_second: Option<DynamicBlock<ElasticacheServerlessCacheCacheUsageLimitsElEcpuPerSecondEl>>,
}

#[derive(Serialize)]
pub struct ElasticacheServerlessCacheCacheUsageLimitsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    data_storage: Option<Vec<ElasticacheServerlessCacheCacheUsageLimitsElDataStorageEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ecpu_per_second: Option<Vec<ElasticacheServerlessCacheCacheUsageLimitsElEcpuPerSecondEl>>,
    dynamic: ElasticacheServerlessCacheCacheUsageLimitsElDynamic,
}

impl ElasticacheServerlessCacheCacheUsageLimitsEl {
    #[doc = "Set the field `data_storage`.\n"]
    pub fn set_data_storage(
        mut self,
        v: impl Into<BlockAssignable<ElasticacheServerlessCacheCacheUsageLimitsElDataStorageEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.data_storage = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.data_storage = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ecpu_per_second`.\n"]
    pub fn set_ecpu_per_second(
        mut self,
        v: impl Into<BlockAssignable<ElasticacheServerlessCacheCacheUsageLimitsElEcpuPerSecondEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ecpu_per_second = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ecpu_per_second = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for ElasticacheServerlessCacheCacheUsageLimitsEl {
    type O = BlockAssignable<ElasticacheServerlessCacheCacheUsageLimitsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildElasticacheServerlessCacheCacheUsageLimitsEl {}

impl BuildElasticacheServerlessCacheCacheUsageLimitsEl {
    pub fn build(self) -> ElasticacheServerlessCacheCacheUsageLimitsEl {
        ElasticacheServerlessCacheCacheUsageLimitsEl {
            data_storage: core::default::Default::default(),
            ecpu_per_second: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct ElasticacheServerlessCacheCacheUsageLimitsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ElasticacheServerlessCacheCacheUsageLimitsElRef {
    fn new(shared: StackShared, base: String) -> ElasticacheServerlessCacheCacheUsageLimitsElRef {
        ElasticacheServerlessCacheCacheUsageLimitsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ElasticacheServerlessCacheCacheUsageLimitsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data_storage` after provisioning.\n"]
    pub fn data_storage(&self) -> ListRef<ElasticacheServerlessCacheCacheUsageLimitsElDataStorageElRef> {
        ListRef::new(self.shared().clone(), format!("{}.data_storage", self.base))
    }

    #[doc = "Get a reference to the value of field `ecpu_per_second` after provisioning.\n"]
    pub fn ecpu_per_second(&self) -> ListRef<ElasticacheServerlessCacheCacheUsageLimitsElEcpuPerSecondElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ecpu_per_second", self.base))
    }
}

#[derive(Serialize)]
pub struct ElasticacheServerlessCacheTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl ElasticacheServerlessCacheTimeoutsEl {
    #[doc =
        "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }

    #[doc =
        "Set the field `delete`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }

    #[doc =
        "Set the field `update`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}

impl ToListMappable for ElasticacheServerlessCacheTimeoutsEl {
    type O = BlockAssignable<ElasticacheServerlessCacheTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildElasticacheServerlessCacheTimeoutsEl {}

impl BuildElasticacheServerlessCacheTimeoutsEl {
    pub fn build(self) -> ElasticacheServerlessCacheTimeoutsEl {
        ElasticacheServerlessCacheTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct ElasticacheServerlessCacheTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ElasticacheServerlessCacheTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ElasticacheServerlessCacheTimeoutsElRef {
        ElasticacheServerlessCacheTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ElasticacheServerlessCacheTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc =
        "Get a reference to the value of field `delete` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc =
        "Get a reference to the value of field `update` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize, Default)]
struct ElasticacheServerlessCacheDynamic {
    cache_usage_limits: Option<DynamicBlock<ElasticacheServerlessCacheCacheUsageLimitsEl>>,
}
