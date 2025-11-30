use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct AppsyncDatasourceData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    api_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_role_arn: Option<PrimField<String>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dynamodb_config: Option<Vec<AppsyncDatasourceDynamodbConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    elasticsearch_config: Option<Vec<AppsyncDatasourceElasticsearchConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    event_bridge_config: Option<Vec<AppsyncDatasourceEventBridgeConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http_config: Option<Vec<AppsyncDatasourceHttpConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lambda_config: Option<Vec<AppsyncDatasourceLambdaConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    opensearchservice_config: Option<Vec<AppsyncDatasourceOpensearchserviceConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    relational_database_config: Option<Vec<AppsyncDatasourceRelationalDatabaseConfigEl>>,
    dynamic: AppsyncDatasourceDynamic,
}

struct AppsyncDatasource_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<AppsyncDatasourceData>,
}

#[derive(Clone)]
pub struct AppsyncDatasource(Rc<AppsyncDatasource_>);

impl AppsyncDatasource {
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

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `service_role_arn`.\n"]
    pub fn set_service_role_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().service_role_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `dynamodb_config`.\n"]
    pub fn set_dynamodb_config(
        self,
        v: impl Into<BlockAssignable<AppsyncDatasourceDynamodbConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().dynamodb_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.dynamodb_config = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `elasticsearch_config`.\n"]
    pub fn set_elasticsearch_config(
        self,
        v: impl Into<BlockAssignable<AppsyncDatasourceElasticsearchConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().elasticsearch_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.elasticsearch_config = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `event_bridge_config`.\n"]
    pub fn set_event_bridge_config(
        self,
        v: impl Into<BlockAssignable<AppsyncDatasourceEventBridgeConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().event_bridge_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.event_bridge_config = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `http_config`.\n"]
    pub fn set_http_config(
        self,
        v: impl Into<BlockAssignable<AppsyncDatasourceHttpConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().http_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.http_config = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `lambda_config`.\n"]
    pub fn set_lambda_config(
        self,
        v: impl Into<BlockAssignable<AppsyncDatasourceLambdaConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().lambda_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.lambda_config = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `opensearchservice_config`.\n"]
    pub fn set_opensearchservice_config(
        self,
        v: impl Into<BlockAssignable<AppsyncDatasourceOpensearchserviceConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().opensearchservice_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.opensearchservice_config = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `relational_database_config`.\n"]
    pub fn set_relational_database_config(
        self,
        v: impl Into<BlockAssignable<AppsyncDatasourceRelationalDatabaseConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().relational_database_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.relational_database_config = Some(d);
            }
        }
        self
    }

    #[doc = "Get a reference to the value of field `api_id` after provisioning.\n"]
    pub fn api_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.api_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `service_role_arn` after provisioning.\n"]
    pub fn service_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_role_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `dynamodb_config` after provisioning.\n"]
    pub fn dynamodb_config(&self) -> ListRef<AppsyncDatasourceDynamodbConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dynamodb_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `elasticsearch_config` after provisioning.\n"]
    pub fn elasticsearch_config(&self) -> ListRef<AppsyncDatasourceElasticsearchConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.elasticsearch_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `event_bridge_config` after provisioning.\n"]
    pub fn event_bridge_config(&self) -> ListRef<AppsyncDatasourceEventBridgeConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.event_bridge_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `http_config` after provisioning.\n"]
    pub fn http_config(&self) -> ListRef<AppsyncDatasourceHttpConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.http_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `lambda_config` after provisioning.\n"]
    pub fn lambda_config(&self) -> ListRef<AppsyncDatasourceLambdaConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.lambda_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `opensearchservice_config` after provisioning.\n"]
    pub fn opensearchservice_config(
        &self,
    ) -> ListRef<AppsyncDatasourceOpensearchserviceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.opensearchservice_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `relational_database_config` after provisioning.\n"]
    pub fn relational_database_config(
        &self,
    ) -> ListRef<AppsyncDatasourceRelationalDatabaseConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.relational_database_config", self.extract_ref()),
        )
    }
}

impl Referable for AppsyncDatasource {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for AppsyncDatasource {}

impl ToListMappable for AppsyncDatasource {
    type O = ListRef<AppsyncDatasourceRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for AppsyncDatasource_ {
    fn extract_resource_type(&self) -> String {
        "aws_appsync_datasource".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAppsyncDatasource {
    pub tf_id: String,
    #[doc = ""]
    pub api_id: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl BuildAppsyncDatasource {
    pub fn build(self, stack: &mut Stack) -> AppsyncDatasource {
        let out = AppsyncDatasource(Rc::new(AppsyncDatasource_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(AppsyncDatasourceData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                api_id: self.api_id,
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                service_role_arn: core::default::Default::default(),
                type_: self.type_,
                dynamodb_config: core::default::Default::default(),
                elasticsearch_config: core::default::Default::default(),
                event_bridge_config: core::default::Default::default(),
                http_config: core::default::Default::default(),
                lambda_config: core::default::Default::default(),
                opensearchservice_config: core::default::Default::default(),
                relational_database_config: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AppsyncDatasourceRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppsyncDatasourceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl AppsyncDatasourceRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `api_id` after provisioning.\n"]
    pub fn api_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.api_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `service_role_arn` after provisioning.\n"]
    pub fn service_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_role_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `dynamodb_config` after provisioning.\n"]
    pub fn dynamodb_config(&self) -> ListRef<AppsyncDatasourceDynamodbConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dynamodb_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `elasticsearch_config` after provisioning.\n"]
    pub fn elasticsearch_config(&self) -> ListRef<AppsyncDatasourceElasticsearchConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.elasticsearch_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `event_bridge_config` after provisioning.\n"]
    pub fn event_bridge_config(&self) -> ListRef<AppsyncDatasourceEventBridgeConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.event_bridge_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `http_config` after provisioning.\n"]
    pub fn http_config(&self) -> ListRef<AppsyncDatasourceHttpConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.http_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `lambda_config` after provisioning.\n"]
    pub fn lambda_config(&self) -> ListRef<AppsyncDatasourceLambdaConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.lambda_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `opensearchservice_config` after provisioning.\n"]
    pub fn opensearchservice_config(
        &self,
    ) -> ListRef<AppsyncDatasourceOpensearchserviceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.opensearchservice_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `relational_database_config` after provisioning.\n"]
    pub fn relational_database_config(
        &self,
    ) -> ListRef<AppsyncDatasourceRelationalDatabaseConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.relational_database_config", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct AppsyncDatasourceDynamodbConfigElDeltaSyncConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    base_table_ttl: Option<PrimField<f64>>,
    delta_sync_table_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delta_sync_table_ttl: Option<PrimField<f64>>,
}

impl AppsyncDatasourceDynamodbConfigElDeltaSyncConfigEl {
    #[doc = "Set the field `base_table_ttl`.\n"]
    pub fn set_base_table_ttl(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.base_table_ttl = Some(v.into());
        self
    }

    #[doc = "Set the field `delta_sync_table_ttl`.\n"]
    pub fn set_delta_sync_table_ttl(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.delta_sync_table_ttl = Some(v.into());
        self
    }
}

impl ToListMappable for AppsyncDatasourceDynamodbConfigElDeltaSyncConfigEl {
    type O = BlockAssignable<AppsyncDatasourceDynamodbConfigElDeltaSyncConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppsyncDatasourceDynamodbConfigElDeltaSyncConfigEl {
    #[doc = ""]
    pub delta_sync_table_name: PrimField<String>,
}

impl BuildAppsyncDatasourceDynamodbConfigElDeltaSyncConfigEl {
    pub fn build(self) -> AppsyncDatasourceDynamodbConfigElDeltaSyncConfigEl {
        AppsyncDatasourceDynamodbConfigElDeltaSyncConfigEl {
            base_table_ttl: core::default::Default::default(),
            delta_sync_table_name: self.delta_sync_table_name,
            delta_sync_table_ttl: core::default::Default::default(),
        }
    }
}

pub struct AppsyncDatasourceDynamodbConfigElDeltaSyncConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppsyncDatasourceDynamodbConfigElDeltaSyncConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppsyncDatasourceDynamodbConfigElDeltaSyncConfigElRef {
        AppsyncDatasourceDynamodbConfigElDeltaSyncConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppsyncDatasourceDynamodbConfigElDeltaSyncConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `base_table_ttl` after provisioning.\n"]
    pub fn base_table_ttl(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.base_table_ttl", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `delta_sync_table_name` after provisioning.\n"]
    pub fn delta_sync_table_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delta_sync_table_name", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `delta_sync_table_ttl` after provisioning.\n"]
    pub fn delta_sync_table_ttl(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delta_sync_table_ttl", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct AppsyncDatasourceDynamodbConfigElDynamic {
    delta_sync_config: Option<DynamicBlock<AppsyncDatasourceDynamodbConfigElDeltaSyncConfigEl>>,
}

#[derive(Serialize)]
pub struct AppsyncDatasourceDynamodbConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    table_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_caller_credentials: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    versioned: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delta_sync_config: Option<Vec<AppsyncDatasourceDynamodbConfigElDeltaSyncConfigEl>>,
    dynamic: AppsyncDatasourceDynamodbConfigElDynamic,
}

impl AppsyncDatasourceDynamodbConfigEl {
    #[doc = "Set the field `region`.\n"]
    pub fn set_region(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.region = Some(v.into());
        self
    }

    #[doc = "Set the field `use_caller_credentials`.\n"]
    pub fn set_use_caller_credentials(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.use_caller_credentials = Some(v.into());
        self
    }

    #[doc = "Set the field `versioned`.\n"]
    pub fn set_versioned(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.versioned = Some(v.into());
        self
    }

    #[doc = "Set the field `delta_sync_config`.\n"]
    pub fn set_delta_sync_config(
        mut self,
        v: impl Into<BlockAssignable<AppsyncDatasourceDynamodbConfigElDeltaSyncConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.delta_sync_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.delta_sync_config = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for AppsyncDatasourceDynamodbConfigEl {
    type O = BlockAssignable<AppsyncDatasourceDynamodbConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppsyncDatasourceDynamodbConfigEl {
    #[doc = ""]
    pub table_name: PrimField<String>,
}

impl BuildAppsyncDatasourceDynamodbConfigEl {
    pub fn build(self) -> AppsyncDatasourceDynamodbConfigEl {
        AppsyncDatasourceDynamodbConfigEl {
            region: core::default::Default::default(),
            table_name: self.table_name,
            use_caller_credentials: core::default::Default::default(),
            versioned: core::default::Default::default(),
            delta_sync_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppsyncDatasourceDynamodbConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppsyncDatasourceDynamodbConfigElRef {
    fn new(shared: StackShared, base: String) -> AppsyncDatasourceDynamodbConfigElRef {
        AppsyncDatasourceDynamodbConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppsyncDatasourceDynamodbConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }

    #[doc = "Get a reference to the value of field `table_name` after provisioning.\n"]
    pub fn table_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.table_name", self.base))
    }

    #[doc = "Get a reference to the value of field `use_caller_credentials` after provisioning.\n"]
    pub fn use_caller_credentials(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.use_caller_credentials", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `versioned` after provisioning.\n"]
    pub fn versioned(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.versioned", self.base))
    }

    #[doc = "Get a reference to the value of field `delta_sync_config` after provisioning.\n"]
    pub fn delta_sync_config(
        &self,
    ) -> ListRef<AppsyncDatasourceDynamodbConfigElDeltaSyncConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.delta_sync_config", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct AppsyncDatasourceElasticsearchConfigEl {
    endpoint: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}

impl AppsyncDatasourceElasticsearchConfigEl {
    #[doc = "Set the field `region`.\n"]
    pub fn set_region(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.region = Some(v.into());
        self
    }
}

impl ToListMappable for AppsyncDatasourceElasticsearchConfigEl {
    type O = BlockAssignable<AppsyncDatasourceElasticsearchConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppsyncDatasourceElasticsearchConfigEl {
    #[doc = ""]
    pub endpoint: PrimField<String>,
}

impl BuildAppsyncDatasourceElasticsearchConfigEl {
    pub fn build(self) -> AppsyncDatasourceElasticsearchConfigEl {
        AppsyncDatasourceElasticsearchConfigEl {
            endpoint: self.endpoint,
            region: core::default::Default::default(),
        }
    }
}

pub struct AppsyncDatasourceElasticsearchConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppsyncDatasourceElasticsearchConfigElRef {
    fn new(shared: StackShared, base: String) -> AppsyncDatasourceElasticsearchConfigElRef {
        AppsyncDatasourceElasticsearchConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppsyncDatasourceElasticsearchConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }
}

#[derive(Serialize)]
pub struct AppsyncDatasourceEventBridgeConfigEl {
    event_bus_arn: PrimField<String>,
}

impl AppsyncDatasourceEventBridgeConfigEl {}

impl ToListMappable for AppsyncDatasourceEventBridgeConfigEl {
    type O = BlockAssignable<AppsyncDatasourceEventBridgeConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppsyncDatasourceEventBridgeConfigEl {
    #[doc = ""]
    pub event_bus_arn: PrimField<String>,
}

impl BuildAppsyncDatasourceEventBridgeConfigEl {
    pub fn build(self) -> AppsyncDatasourceEventBridgeConfigEl {
        AppsyncDatasourceEventBridgeConfigEl {
            event_bus_arn: self.event_bus_arn,
        }
    }
}

pub struct AppsyncDatasourceEventBridgeConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppsyncDatasourceEventBridgeConfigElRef {
    fn new(shared: StackShared, base: String) -> AppsyncDatasourceEventBridgeConfigElRef {
        AppsyncDatasourceEventBridgeConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppsyncDatasourceEventBridgeConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `event_bus_arn` after provisioning.\n"]
    pub fn event_bus_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.event_bus_arn", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct AppsyncDatasourceHttpConfigElAuthorizationConfigElAwsIamConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    signing_region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    signing_service_name: Option<PrimField<String>>,
}

impl AppsyncDatasourceHttpConfigElAuthorizationConfigElAwsIamConfigEl {
    #[doc = "Set the field `signing_region`.\n"]
    pub fn set_signing_region(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.signing_region = Some(v.into());
        self
    }

    #[doc = "Set the field `signing_service_name`.\n"]
    pub fn set_signing_service_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.signing_service_name = Some(v.into());
        self
    }
}

impl ToListMappable for AppsyncDatasourceHttpConfigElAuthorizationConfigElAwsIamConfigEl {
    type O = BlockAssignable<AppsyncDatasourceHttpConfigElAuthorizationConfigElAwsIamConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppsyncDatasourceHttpConfigElAuthorizationConfigElAwsIamConfigEl {}

impl BuildAppsyncDatasourceHttpConfigElAuthorizationConfigElAwsIamConfigEl {
    pub fn build(self) -> AppsyncDatasourceHttpConfigElAuthorizationConfigElAwsIamConfigEl {
        AppsyncDatasourceHttpConfigElAuthorizationConfigElAwsIamConfigEl {
            signing_region: core::default::Default::default(),
            signing_service_name: core::default::Default::default(),
        }
    }
}

pub struct AppsyncDatasourceHttpConfigElAuthorizationConfigElAwsIamConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppsyncDatasourceHttpConfigElAuthorizationConfigElAwsIamConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppsyncDatasourceHttpConfigElAuthorizationConfigElAwsIamConfigElRef {
        AppsyncDatasourceHttpConfigElAuthorizationConfigElAwsIamConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppsyncDatasourceHttpConfigElAuthorizationConfigElAwsIamConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `signing_region` after provisioning.\n"]
    pub fn signing_region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.signing_region", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `signing_service_name` after provisioning.\n"]
    pub fn signing_service_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.signing_service_name", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct AppsyncDatasourceHttpConfigElAuthorizationConfigElDynamic {
    aws_iam_config:
        Option<DynamicBlock<AppsyncDatasourceHttpConfigElAuthorizationConfigElAwsIamConfigEl>>,
}

#[derive(Serialize)]
pub struct AppsyncDatasourceHttpConfigElAuthorizationConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    authorization_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    aws_iam_config: Option<Vec<AppsyncDatasourceHttpConfigElAuthorizationConfigElAwsIamConfigEl>>,
    dynamic: AppsyncDatasourceHttpConfigElAuthorizationConfigElDynamic,
}

impl AppsyncDatasourceHttpConfigElAuthorizationConfigEl {
    #[doc = "Set the field `authorization_type`.\n"]
    pub fn set_authorization_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.authorization_type = Some(v.into());
        self
    }

    #[doc = "Set the field `aws_iam_config`.\n"]
    pub fn set_aws_iam_config(
        mut self,
        v: impl Into<BlockAssignable<AppsyncDatasourceHttpConfigElAuthorizationConfigElAwsIamConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.aws_iam_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.aws_iam_config = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for AppsyncDatasourceHttpConfigElAuthorizationConfigEl {
    type O = BlockAssignable<AppsyncDatasourceHttpConfigElAuthorizationConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppsyncDatasourceHttpConfigElAuthorizationConfigEl {}

impl BuildAppsyncDatasourceHttpConfigElAuthorizationConfigEl {
    pub fn build(self) -> AppsyncDatasourceHttpConfigElAuthorizationConfigEl {
        AppsyncDatasourceHttpConfigElAuthorizationConfigEl {
            authorization_type: core::default::Default::default(),
            aws_iam_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppsyncDatasourceHttpConfigElAuthorizationConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppsyncDatasourceHttpConfigElAuthorizationConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppsyncDatasourceHttpConfigElAuthorizationConfigElRef {
        AppsyncDatasourceHttpConfigElAuthorizationConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppsyncDatasourceHttpConfigElAuthorizationConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authorization_type` after provisioning.\n"]
    pub fn authorization_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.authorization_type", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `aws_iam_config` after provisioning.\n"]
    pub fn aws_iam_config(
        &self,
    ) -> ListRef<AppsyncDatasourceHttpConfigElAuthorizationConfigElAwsIamConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.aws_iam_config", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct AppsyncDatasourceHttpConfigElDynamic {
    authorization_config: Option<DynamicBlock<AppsyncDatasourceHttpConfigElAuthorizationConfigEl>>,
}

#[derive(Serialize)]
pub struct AppsyncDatasourceHttpConfigEl {
    endpoint: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authorization_config: Option<Vec<AppsyncDatasourceHttpConfigElAuthorizationConfigEl>>,
    dynamic: AppsyncDatasourceHttpConfigElDynamic,
}

impl AppsyncDatasourceHttpConfigEl {
    #[doc = "Set the field `authorization_config`.\n"]
    pub fn set_authorization_config(
        mut self,
        v: impl Into<BlockAssignable<AppsyncDatasourceHttpConfigElAuthorizationConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.authorization_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.authorization_config = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for AppsyncDatasourceHttpConfigEl {
    type O = BlockAssignable<AppsyncDatasourceHttpConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppsyncDatasourceHttpConfigEl {
    #[doc = ""]
    pub endpoint: PrimField<String>,
}

impl BuildAppsyncDatasourceHttpConfigEl {
    pub fn build(self) -> AppsyncDatasourceHttpConfigEl {
        AppsyncDatasourceHttpConfigEl {
            endpoint: self.endpoint,
            authorization_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppsyncDatasourceHttpConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppsyncDatasourceHttpConfigElRef {
    fn new(shared: StackShared, base: String) -> AppsyncDatasourceHttpConfigElRef {
        AppsyncDatasourceHttpConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppsyncDatasourceHttpConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `authorization_config` after provisioning.\n"]
    pub fn authorization_config(
        &self,
    ) -> ListRef<AppsyncDatasourceHttpConfigElAuthorizationConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.authorization_config", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct AppsyncDatasourceLambdaConfigEl {
    function_arn: PrimField<String>,
}

impl AppsyncDatasourceLambdaConfigEl {}

impl ToListMappable for AppsyncDatasourceLambdaConfigEl {
    type O = BlockAssignable<AppsyncDatasourceLambdaConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppsyncDatasourceLambdaConfigEl {
    #[doc = ""]
    pub function_arn: PrimField<String>,
}

impl BuildAppsyncDatasourceLambdaConfigEl {
    pub fn build(self) -> AppsyncDatasourceLambdaConfigEl {
        AppsyncDatasourceLambdaConfigEl {
            function_arn: self.function_arn,
        }
    }
}

pub struct AppsyncDatasourceLambdaConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppsyncDatasourceLambdaConfigElRef {
    fn new(shared: StackShared, base: String) -> AppsyncDatasourceLambdaConfigElRef {
        AppsyncDatasourceLambdaConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppsyncDatasourceLambdaConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `function_arn` after provisioning.\n"]
    pub fn function_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.function_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct AppsyncDatasourceOpensearchserviceConfigEl {
    endpoint: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}

impl AppsyncDatasourceOpensearchserviceConfigEl {
    #[doc = "Set the field `region`.\n"]
    pub fn set_region(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.region = Some(v.into());
        self
    }
}

impl ToListMappable for AppsyncDatasourceOpensearchserviceConfigEl {
    type O = BlockAssignable<AppsyncDatasourceOpensearchserviceConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppsyncDatasourceOpensearchserviceConfigEl {
    #[doc = ""]
    pub endpoint: PrimField<String>,
}

impl BuildAppsyncDatasourceOpensearchserviceConfigEl {
    pub fn build(self) -> AppsyncDatasourceOpensearchserviceConfigEl {
        AppsyncDatasourceOpensearchserviceConfigEl {
            endpoint: self.endpoint,
            region: core::default::Default::default(),
        }
    }
}

pub struct AppsyncDatasourceOpensearchserviceConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppsyncDatasourceOpensearchserviceConfigElRef {
    fn new(shared: StackShared, base: String) -> AppsyncDatasourceOpensearchserviceConfigElRef {
        AppsyncDatasourceOpensearchserviceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppsyncDatasourceOpensearchserviceConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }
}

#[derive(Serialize)]
pub struct AppsyncDatasourceRelationalDatabaseConfigElHttpEndpointConfigEl {
    aws_secret_store_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    database_name: Option<PrimField<String>>,
    db_cluster_identifier: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schema: Option<PrimField<String>>,
}

impl AppsyncDatasourceRelationalDatabaseConfigElHttpEndpointConfigEl {
    #[doc = "Set the field `database_name`.\n"]
    pub fn set_database_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.database_name = Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\n"]
    pub fn set_region(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.region = Some(v.into());
        self
    }

    #[doc = "Set the field `schema`.\n"]
    pub fn set_schema(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.schema = Some(v.into());
        self
    }
}

impl ToListMappable for AppsyncDatasourceRelationalDatabaseConfigElHttpEndpointConfigEl {
    type O = BlockAssignable<AppsyncDatasourceRelationalDatabaseConfigElHttpEndpointConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppsyncDatasourceRelationalDatabaseConfigElHttpEndpointConfigEl {
    #[doc = ""]
    pub aws_secret_store_arn: PrimField<String>,
    #[doc = ""]
    pub db_cluster_identifier: PrimField<String>,
}

impl BuildAppsyncDatasourceRelationalDatabaseConfigElHttpEndpointConfigEl {
    pub fn build(self) -> AppsyncDatasourceRelationalDatabaseConfigElHttpEndpointConfigEl {
        AppsyncDatasourceRelationalDatabaseConfigElHttpEndpointConfigEl {
            aws_secret_store_arn: self.aws_secret_store_arn,
            database_name: core::default::Default::default(),
            db_cluster_identifier: self.db_cluster_identifier,
            region: core::default::Default::default(),
            schema: core::default::Default::default(),
        }
    }
}

pub struct AppsyncDatasourceRelationalDatabaseConfigElHttpEndpointConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppsyncDatasourceRelationalDatabaseConfigElHttpEndpointConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppsyncDatasourceRelationalDatabaseConfigElHttpEndpointConfigElRef {
        AppsyncDatasourceRelationalDatabaseConfigElHttpEndpointConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppsyncDatasourceRelationalDatabaseConfigElHttpEndpointConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `aws_secret_store_arn` after provisioning.\n"]
    pub fn aws_secret_store_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.aws_secret_store_arn", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `database_name` after provisioning.\n"]
    pub fn database_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.database_name", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `db_cluster_identifier` after provisioning.\n"]
    pub fn db_cluster_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.db_cluster_identifier", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }

    #[doc = "Get a reference to the value of field `schema` after provisioning.\n"]
    pub fn schema(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.schema", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppsyncDatasourceRelationalDatabaseConfigElDynamic {
    http_endpoint_config:
        Option<DynamicBlock<AppsyncDatasourceRelationalDatabaseConfigElHttpEndpointConfigEl>>,
}

#[derive(Serialize)]
pub struct AppsyncDatasourceRelationalDatabaseConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    source_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http_endpoint_config:
        Option<Vec<AppsyncDatasourceRelationalDatabaseConfigElHttpEndpointConfigEl>>,
    dynamic: AppsyncDatasourceRelationalDatabaseConfigElDynamic,
}

impl AppsyncDatasourceRelationalDatabaseConfigEl {
    #[doc = "Set the field `source_type`.\n"]
    pub fn set_source_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_type = Some(v.into());
        self
    }

    #[doc = "Set the field `http_endpoint_config`.\n"]
    pub fn set_http_endpoint_config(
        mut self,
        v: impl Into<BlockAssignable<AppsyncDatasourceRelationalDatabaseConfigElHttpEndpointConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.http_endpoint_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.http_endpoint_config = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for AppsyncDatasourceRelationalDatabaseConfigEl {
    type O = BlockAssignable<AppsyncDatasourceRelationalDatabaseConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppsyncDatasourceRelationalDatabaseConfigEl {}

impl BuildAppsyncDatasourceRelationalDatabaseConfigEl {
    pub fn build(self) -> AppsyncDatasourceRelationalDatabaseConfigEl {
        AppsyncDatasourceRelationalDatabaseConfigEl {
            source_type: core::default::Default::default(),
            http_endpoint_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppsyncDatasourceRelationalDatabaseConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppsyncDatasourceRelationalDatabaseConfigElRef {
    fn new(shared: StackShared, base: String) -> AppsyncDatasourceRelationalDatabaseConfigElRef {
        AppsyncDatasourceRelationalDatabaseConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppsyncDatasourceRelationalDatabaseConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `source_type` after provisioning.\n"]
    pub fn source_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_type", self.base))
    }

    #[doc = "Get a reference to the value of field `http_endpoint_config` after provisioning.\n"]
    pub fn http_endpoint_config(
        &self,
    ) -> ListRef<AppsyncDatasourceRelationalDatabaseConfigElHttpEndpointConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.http_endpoint_config", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct AppsyncDatasourceDynamic {
    dynamodb_config: Option<DynamicBlock<AppsyncDatasourceDynamodbConfigEl>>,
    elasticsearch_config: Option<DynamicBlock<AppsyncDatasourceElasticsearchConfigEl>>,
    event_bridge_config: Option<DynamicBlock<AppsyncDatasourceEventBridgeConfigEl>>,
    http_config: Option<DynamicBlock<AppsyncDatasourceHttpConfigEl>>,
    lambda_config: Option<DynamicBlock<AppsyncDatasourceLambdaConfigEl>>,
    opensearchservice_config: Option<DynamicBlock<AppsyncDatasourceOpensearchserviceConfigEl>>,
    relational_database_config: Option<DynamicBlock<AppsyncDatasourceRelationalDatabaseConfigEl>>,
}
