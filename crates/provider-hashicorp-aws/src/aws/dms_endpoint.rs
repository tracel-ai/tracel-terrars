use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DmsEndpointData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    database_name: Option<PrimField<String>>,
    endpoint_id: PrimField<String>,
    endpoint_type: PrimField<String>,
    engine_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    extra_connection_attributes: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    password: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pause_replication_tasks: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secrets_manager_access_role_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secrets_manager_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    server_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_access_role: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ssl_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    username: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    elasticsearch_settings: Option<Vec<DmsEndpointElasticsearchSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kafka_settings: Option<Vec<DmsEndpointKafkaSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kinesis_settings: Option<Vec<DmsEndpointKinesisSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mongodb_settings: Option<Vec<DmsEndpointMongodbSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mysql_settings: Option<Vec<DmsEndpointMysqlSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oracle_settings: Option<Vec<DmsEndpointOracleSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    postgres_settings: Option<Vec<DmsEndpointPostgresSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    redis_settings: Option<Vec<DmsEndpointRedisSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    redshift_settings: Option<Vec<DmsEndpointRedshiftSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DmsEndpointTimeoutsEl>,
    dynamic: DmsEndpointDynamic,
}
struct DmsEndpoint_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DmsEndpointData>,
}
#[derive(Clone)]
pub struct DmsEndpoint(Rc<DmsEndpoint_>);
impl DmsEndpoint {
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
    #[doc = "Set the field `certificate_arn`.\n"]
    pub fn set_certificate_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().certificate_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `database_name`.\n"]
    pub fn set_database_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().database_name = Some(v.into());
        self
    }
    #[doc = "Set the field `extra_connection_attributes`.\n"]
    pub fn set_extra_connection_attributes(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().extra_connection_attributes = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `kms_key_arn`.\n"]
    pub fn set_kms_key_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().kms_key_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `password`.\n"]
    pub fn set_password(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().password = Some(v.into());
        self
    }
    #[doc = "Set the field `pause_replication_tasks`.\n"]
    pub fn set_pause_replication_tasks(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().pause_replication_tasks = Some(v.into());
        self
    }
    #[doc = "Set the field `port`.\n"]
    pub fn set_port(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().port = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `secrets_manager_access_role_arn`.\n"]
    pub fn set_secrets_manager_access_role_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().secrets_manager_access_role_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `secrets_manager_arn`.\n"]
    pub fn set_secrets_manager_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().secrets_manager_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `server_name`.\n"]
    pub fn set_server_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().server_name = Some(v.into());
        self
    }
    #[doc = "Set the field `service_access_role`.\n"]
    pub fn set_service_access_role(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().service_access_role = Some(v.into());
        self
    }
    #[doc = "Set the field `ssl_mode`.\n"]
    pub fn set_ssl_mode(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().ssl_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }
    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags_all = Some(v.into());
        self
    }
    #[doc = "Set the field `username`.\n"]
    pub fn set_username(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().username = Some(v.into());
        self
    }
    #[doc = "Set the field `elasticsearch_settings`.\n"]
    pub fn set_elasticsearch_settings(
        self,
        v: impl Into<BlockAssignable<DmsEndpointElasticsearchSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().elasticsearch_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.elasticsearch_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `kafka_settings`.\n"]
    pub fn set_kafka_settings(
        self,
        v: impl Into<BlockAssignable<DmsEndpointKafkaSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().kafka_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.kafka_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `kinesis_settings`.\n"]
    pub fn set_kinesis_settings(
        self,
        v: impl Into<BlockAssignable<DmsEndpointKinesisSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().kinesis_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.kinesis_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `mongodb_settings`.\n"]
    pub fn set_mongodb_settings(
        self,
        v: impl Into<BlockAssignable<DmsEndpointMongodbSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().mongodb_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.mongodb_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `mysql_settings`.\n"]
    pub fn set_mysql_settings(
        self,
        v: impl Into<BlockAssignable<DmsEndpointMysqlSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().mysql_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.mysql_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `oracle_settings`.\n"]
    pub fn set_oracle_settings(
        self,
        v: impl Into<BlockAssignable<DmsEndpointOracleSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().oracle_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.oracle_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `postgres_settings`.\n"]
    pub fn set_postgres_settings(
        self,
        v: impl Into<BlockAssignable<DmsEndpointPostgresSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().postgres_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.postgres_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `redis_settings`.\n"]
    pub fn set_redis_settings(
        self,
        v: impl Into<BlockAssignable<DmsEndpointRedisSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().redis_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.redis_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `redshift_settings`.\n"]
    pub fn set_redshift_settings(
        self,
        v: impl Into<BlockAssignable<DmsEndpointRedshiftSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().redshift_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.redshift_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DmsEndpointTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `certificate_arn` after provisioning.\n"]
    pub fn certificate_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.certificate_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `database_name` after provisioning.\n"]
    pub fn database_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.database_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `endpoint_arn` after provisioning.\n"]
    pub fn endpoint_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.endpoint_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `endpoint_id` after provisioning.\n"]
    pub fn endpoint_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.endpoint_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `endpoint_type` after provisioning.\n"]
    pub fn endpoint_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.endpoint_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `engine_name` after provisioning.\n"]
    pub fn engine_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.engine_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `extra_connection_attributes` after provisioning.\n"]
    pub fn extra_connection_attributes(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.extra_connection_attributes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `password` after provisioning.\n"]
    pub fn password(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.password", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `pause_replication_tasks` after provisioning.\n"]
    pub fn pause_replication_tasks(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pause_replication_tasks", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.port", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `secrets_manager_access_role_arn` after provisioning.\n"]
    pub fn secrets_manager_access_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.secrets_manager_access_role_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `secrets_manager_arn` after provisioning.\n"]
    pub fn secrets_manager_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.secrets_manager_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `server_name` after provisioning.\n"]
    pub fn server_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.server_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_access_role` after provisioning.\n"]
    pub fn service_access_role(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_access_role", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ssl_mode` after provisioning.\n"]
    pub fn ssl_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ssl_mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `username` after provisioning.\n"]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.username", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `elasticsearch_settings` after provisioning.\n"]
    pub fn elasticsearch_settings(&self) -> ListRef<DmsEndpointElasticsearchSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.elasticsearch_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `kafka_settings` after provisioning.\n"]
    pub fn kafka_settings(&self) -> ListRef<DmsEndpointKafkaSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.kafka_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `kinesis_settings` after provisioning.\n"]
    pub fn kinesis_settings(&self) -> ListRef<DmsEndpointKinesisSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.kinesis_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `mongodb_settings` after provisioning.\n"]
    pub fn mongodb_settings(&self) -> ListRef<DmsEndpointMongodbSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.mongodb_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `mysql_settings` after provisioning.\n"]
    pub fn mysql_settings(&self) -> ListRef<DmsEndpointMysqlSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.mysql_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `oracle_settings` after provisioning.\n"]
    pub fn oracle_settings(&self) -> ListRef<DmsEndpointOracleSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.oracle_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `postgres_settings` after provisioning.\n"]
    pub fn postgres_settings(&self) -> ListRef<DmsEndpointPostgresSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.postgres_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `redis_settings` after provisioning.\n"]
    pub fn redis_settings(&self) -> ListRef<DmsEndpointRedisSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.redis_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `redshift_settings` after provisioning.\n"]
    pub fn redshift_settings(&self) -> ListRef<DmsEndpointRedshiftSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.redshift_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DmsEndpointTimeoutsElRef {
        DmsEndpointTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for DmsEndpoint {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for DmsEndpoint {}
impl ToListMappable for DmsEndpoint {
    type O = ListRef<DmsEndpointRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for DmsEndpoint_ {
    fn extract_resource_type(&self) -> String {
        "aws_dms_endpoint".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDmsEndpoint {
    pub tf_id: String,
    #[doc = ""]
    pub endpoint_id: PrimField<String>,
    #[doc = ""]
    pub endpoint_type: PrimField<String>,
    #[doc = ""]
    pub engine_name: PrimField<String>,
}
impl BuildDmsEndpoint {
    pub fn build(self, stack: &mut Stack) -> DmsEndpoint {
        let out = DmsEndpoint(Rc::new(DmsEndpoint_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DmsEndpointData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                certificate_arn: core::default::Default::default(),
                database_name: core::default::Default::default(),
                endpoint_id: self.endpoint_id,
                endpoint_type: self.endpoint_type,
                engine_name: self.engine_name,
                extra_connection_attributes: core::default::Default::default(),
                id: core::default::Default::default(),
                kms_key_arn: core::default::Default::default(),
                password: core::default::Default::default(),
                pause_replication_tasks: core::default::Default::default(),
                port: core::default::Default::default(),
                region: core::default::Default::default(),
                secrets_manager_access_role_arn: core::default::Default::default(),
                secrets_manager_arn: core::default::Default::default(),
                server_name: core::default::Default::default(),
                service_access_role: core::default::Default::default(),
                ssl_mode: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                username: core::default::Default::default(),
                elasticsearch_settings: core::default::Default::default(),
                kafka_settings: core::default::Default::default(),
                kinesis_settings: core::default::Default::default(),
                mongodb_settings: core::default::Default::default(),
                mysql_settings: core::default::Default::default(),
                oracle_settings: core::default::Default::default(),
                postgres_settings: core::default::Default::default(),
                redis_settings: core::default::Default::default(),
                redshift_settings: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct DmsEndpointRef {
    shared: StackShared,
    base: String,
}
impl Ref for DmsEndpointRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DmsEndpointRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `certificate_arn` after provisioning.\n"]
    pub fn certificate_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.certificate_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `database_name` after provisioning.\n"]
    pub fn database_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.database_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `endpoint_arn` after provisioning.\n"]
    pub fn endpoint_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.endpoint_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `endpoint_id` after provisioning.\n"]
    pub fn endpoint_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.endpoint_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `endpoint_type` after provisioning.\n"]
    pub fn endpoint_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.endpoint_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `engine_name` after provisioning.\n"]
    pub fn engine_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.engine_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `extra_connection_attributes` after provisioning.\n"]
    pub fn extra_connection_attributes(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.extra_connection_attributes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `password` after provisioning.\n"]
    pub fn password(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.password", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `pause_replication_tasks` after provisioning.\n"]
    pub fn pause_replication_tasks(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pause_replication_tasks", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.port", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `secrets_manager_access_role_arn` after provisioning.\n"]
    pub fn secrets_manager_access_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.secrets_manager_access_role_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `secrets_manager_arn` after provisioning.\n"]
    pub fn secrets_manager_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.secrets_manager_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `server_name` after provisioning.\n"]
    pub fn server_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.server_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_access_role` after provisioning.\n"]
    pub fn service_access_role(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_access_role", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ssl_mode` after provisioning.\n"]
    pub fn ssl_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ssl_mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `username` after provisioning.\n"]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.username", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `elasticsearch_settings` after provisioning.\n"]
    pub fn elasticsearch_settings(&self) -> ListRef<DmsEndpointElasticsearchSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.elasticsearch_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `kafka_settings` after provisioning.\n"]
    pub fn kafka_settings(&self) -> ListRef<DmsEndpointKafkaSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.kafka_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `kinesis_settings` after provisioning.\n"]
    pub fn kinesis_settings(&self) -> ListRef<DmsEndpointKinesisSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.kinesis_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `mongodb_settings` after provisioning.\n"]
    pub fn mongodb_settings(&self) -> ListRef<DmsEndpointMongodbSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.mongodb_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `mysql_settings` after provisioning.\n"]
    pub fn mysql_settings(&self) -> ListRef<DmsEndpointMysqlSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.mysql_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `oracle_settings` after provisioning.\n"]
    pub fn oracle_settings(&self) -> ListRef<DmsEndpointOracleSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.oracle_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `postgres_settings` after provisioning.\n"]
    pub fn postgres_settings(&self) -> ListRef<DmsEndpointPostgresSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.postgres_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `redis_settings` after provisioning.\n"]
    pub fn redis_settings(&self) -> ListRef<DmsEndpointRedisSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.redis_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `redshift_settings` after provisioning.\n"]
    pub fn redshift_settings(&self) -> ListRef<DmsEndpointRedshiftSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.redshift_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DmsEndpointTimeoutsElRef {
        DmsEndpointTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DmsEndpointElasticsearchSettingsEl {
    endpoint_uri: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_retry_duration: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    full_load_error_percentage: Option<PrimField<f64>>,
    service_access_role_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_new_mapping_type: Option<PrimField<bool>>,
}
impl DmsEndpointElasticsearchSettingsEl {
    #[doc = "Set the field `error_retry_duration`.\n"]
    pub fn set_error_retry_duration(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.error_retry_duration = Some(v.into());
        self
    }
    #[doc = "Set the field `full_load_error_percentage`.\n"]
    pub fn set_full_load_error_percentage(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.full_load_error_percentage = Some(v.into());
        self
    }
    #[doc = "Set the field `use_new_mapping_type`.\n"]
    pub fn set_use_new_mapping_type(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.use_new_mapping_type = Some(v.into());
        self
    }
}
impl ToListMappable for DmsEndpointElasticsearchSettingsEl {
    type O = BlockAssignable<DmsEndpointElasticsearchSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDmsEndpointElasticsearchSettingsEl {
    #[doc = ""]
    pub endpoint_uri: PrimField<String>,
    #[doc = ""]
    pub service_access_role_arn: PrimField<String>,
}
impl BuildDmsEndpointElasticsearchSettingsEl {
    pub fn build(self) -> DmsEndpointElasticsearchSettingsEl {
        DmsEndpointElasticsearchSettingsEl {
            endpoint_uri: self.endpoint_uri,
            error_retry_duration: core::default::Default::default(),
            full_load_error_percentage: core::default::Default::default(),
            service_access_role_arn: self.service_access_role_arn,
            use_new_mapping_type: core::default::Default::default(),
        }
    }
}
pub struct DmsEndpointElasticsearchSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DmsEndpointElasticsearchSettingsElRef {
    fn new(shared: StackShared, base: String) -> DmsEndpointElasticsearchSettingsElRef {
        DmsEndpointElasticsearchSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DmsEndpointElasticsearchSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `endpoint_uri` after provisioning.\n"]
    pub fn endpoint_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint_uri", self.base))
    }
    #[doc = "Get a reference to the value of field `error_retry_duration` after provisioning.\n"]
    pub fn error_retry_duration(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.error_retry_duration", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `full_load_error_percentage` after provisioning.\n"]
    pub fn full_load_error_percentage(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.full_load_error_percentage", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `service_access_role_arn` after provisioning.\n"]
    pub fn service_access_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_access_role_arn", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `use_new_mapping_type` after provisioning.\n"]
    pub fn use_new_mapping_type(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.use_new_mapping_type", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DmsEndpointKafkaSettingsEl {
    broker: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_control_details: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_null_and_empty: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_partition_value: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_table_alter_operations: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_transaction_details: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_format: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_max_bytes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    no_hex_prefix: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    partition_include_schema_table: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sasl_mechanism: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sasl_password: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sasl_username: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_protocol: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ssl_ca_certificate_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ssl_client_certificate_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ssl_client_key_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ssl_client_key_password: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    topic: Option<PrimField<String>>,
}
impl DmsEndpointKafkaSettingsEl {
    #[doc = "Set the field `include_control_details`.\n"]
    pub fn set_include_control_details(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.include_control_details = Some(v.into());
        self
    }
    #[doc = "Set the field `include_null_and_empty`.\n"]
    pub fn set_include_null_and_empty(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.include_null_and_empty = Some(v.into());
        self
    }
    #[doc = "Set the field `include_partition_value`.\n"]
    pub fn set_include_partition_value(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.include_partition_value = Some(v.into());
        self
    }
    #[doc = "Set the field `include_table_alter_operations`.\n"]
    pub fn set_include_table_alter_operations(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.include_table_alter_operations = Some(v.into());
        self
    }
    #[doc = "Set the field `include_transaction_details`.\n"]
    pub fn set_include_transaction_details(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.include_transaction_details = Some(v.into());
        self
    }
    #[doc = "Set the field `message_format`.\n"]
    pub fn set_message_format(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message_format = Some(v.into());
        self
    }
    #[doc = "Set the field `message_max_bytes`.\n"]
    pub fn set_message_max_bytes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.message_max_bytes = Some(v.into());
        self
    }
    #[doc = "Set the field `no_hex_prefix`.\n"]
    pub fn set_no_hex_prefix(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.no_hex_prefix = Some(v.into());
        self
    }
    #[doc = "Set the field `partition_include_schema_table`.\n"]
    pub fn set_partition_include_schema_table(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.partition_include_schema_table = Some(v.into());
        self
    }
    #[doc = "Set the field `sasl_mechanism`.\n"]
    pub fn set_sasl_mechanism(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sasl_mechanism = Some(v.into());
        self
    }
    #[doc = "Set the field `sasl_password`.\n"]
    pub fn set_sasl_password(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sasl_password = Some(v.into());
        self
    }
    #[doc = "Set the field `sasl_username`.\n"]
    pub fn set_sasl_username(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sasl_username = Some(v.into());
        self
    }
    #[doc = "Set the field `security_protocol`.\n"]
    pub fn set_security_protocol(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.security_protocol = Some(v.into());
        self
    }
    #[doc = "Set the field `ssl_ca_certificate_arn`.\n"]
    pub fn set_ssl_ca_certificate_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ssl_ca_certificate_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `ssl_client_certificate_arn`.\n"]
    pub fn set_ssl_client_certificate_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ssl_client_certificate_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `ssl_client_key_arn`.\n"]
    pub fn set_ssl_client_key_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ssl_client_key_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `ssl_client_key_password`.\n"]
    pub fn set_ssl_client_key_password(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ssl_client_key_password = Some(v.into());
        self
    }
    #[doc = "Set the field `topic`.\n"]
    pub fn set_topic(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.topic = Some(v.into());
        self
    }
}
impl ToListMappable for DmsEndpointKafkaSettingsEl {
    type O = BlockAssignable<DmsEndpointKafkaSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDmsEndpointKafkaSettingsEl {
    #[doc = ""]
    pub broker: PrimField<String>,
}
impl BuildDmsEndpointKafkaSettingsEl {
    pub fn build(self) -> DmsEndpointKafkaSettingsEl {
        DmsEndpointKafkaSettingsEl {
            broker: self.broker,
            include_control_details: core::default::Default::default(),
            include_null_and_empty: core::default::Default::default(),
            include_partition_value: core::default::Default::default(),
            include_table_alter_operations: core::default::Default::default(),
            include_transaction_details: core::default::Default::default(),
            message_format: core::default::Default::default(),
            message_max_bytes: core::default::Default::default(),
            no_hex_prefix: core::default::Default::default(),
            partition_include_schema_table: core::default::Default::default(),
            sasl_mechanism: core::default::Default::default(),
            sasl_password: core::default::Default::default(),
            sasl_username: core::default::Default::default(),
            security_protocol: core::default::Default::default(),
            ssl_ca_certificate_arn: core::default::Default::default(),
            ssl_client_certificate_arn: core::default::Default::default(),
            ssl_client_key_arn: core::default::Default::default(),
            ssl_client_key_password: core::default::Default::default(),
            topic: core::default::Default::default(),
        }
    }
}
pub struct DmsEndpointKafkaSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DmsEndpointKafkaSettingsElRef {
    fn new(shared: StackShared, base: String) -> DmsEndpointKafkaSettingsElRef {
        DmsEndpointKafkaSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DmsEndpointKafkaSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `broker` after provisioning.\n"]
    pub fn broker(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.broker", self.base))
    }
    #[doc = "Get a reference to the value of field `include_control_details` after provisioning.\n"]
    pub fn include_control_details(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.include_control_details", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `include_null_and_empty` after provisioning.\n"]
    pub fn include_null_and_empty(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.include_null_and_empty", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `include_partition_value` after provisioning.\n"]
    pub fn include_partition_value(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.include_partition_value", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `include_table_alter_operations` after provisioning.\n"]
    pub fn include_table_alter_operations(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.include_table_alter_operations", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `include_transaction_details` after provisioning.\n"]
    pub fn include_transaction_details(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.include_transaction_details", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `message_format` after provisioning.\n"]
    pub fn message_format(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.message_format", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `message_max_bytes` after provisioning.\n"]
    pub fn message_max_bytes(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.message_max_bytes", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `no_hex_prefix` after provisioning.\n"]
    pub fn no_hex_prefix(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.no_hex_prefix", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `partition_include_schema_table` after provisioning.\n"]
    pub fn partition_include_schema_table(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.partition_include_schema_table", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `sasl_mechanism` after provisioning.\n"]
    pub fn sasl_mechanism(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sasl_mechanism", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `sasl_password` after provisioning.\n"]
    pub fn sasl_password(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sasl_password", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `sasl_username` after provisioning.\n"]
    pub fn sasl_username(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sasl_username", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `security_protocol` after provisioning.\n"]
    pub fn security_protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.security_protocol", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ssl_ca_certificate_arn` after provisioning.\n"]
    pub fn ssl_ca_certificate_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ssl_ca_certificate_arn", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ssl_client_certificate_arn` after provisioning.\n"]
    pub fn ssl_client_certificate_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ssl_client_certificate_arn", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ssl_client_key_arn` after provisioning.\n"]
    pub fn ssl_client_key_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ssl_client_key_arn", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ssl_client_key_password` after provisioning.\n"]
    pub fn ssl_client_key_password(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ssl_client_key_password", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `topic` after provisioning.\n"]
    pub fn topic(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.topic", self.base))
    }
}
#[derive(Serialize)]
pub struct DmsEndpointKinesisSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    include_control_details: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_null_and_empty: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_partition_value: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_table_alter_operations: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_transaction_details: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_format: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    partition_include_schema_table: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_access_role_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stream_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_large_integer_value: Option<PrimField<bool>>,
}
impl DmsEndpointKinesisSettingsEl {
    #[doc = "Set the field `include_control_details`.\n"]
    pub fn set_include_control_details(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.include_control_details = Some(v.into());
        self
    }
    #[doc = "Set the field `include_null_and_empty`.\n"]
    pub fn set_include_null_and_empty(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.include_null_and_empty = Some(v.into());
        self
    }
    #[doc = "Set the field `include_partition_value`.\n"]
    pub fn set_include_partition_value(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.include_partition_value = Some(v.into());
        self
    }
    #[doc = "Set the field `include_table_alter_operations`.\n"]
    pub fn set_include_table_alter_operations(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.include_table_alter_operations = Some(v.into());
        self
    }
    #[doc = "Set the field `include_transaction_details`.\n"]
    pub fn set_include_transaction_details(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.include_transaction_details = Some(v.into());
        self
    }
    #[doc = "Set the field `message_format`.\n"]
    pub fn set_message_format(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message_format = Some(v.into());
        self
    }
    #[doc = "Set the field `partition_include_schema_table`.\n"]
    pub fn set_partition_include_schema_table(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.partition_include_schema_table = Some(v.into());
        self
    }
    #[doc = "Set the field `service_access_role_arn`.\n"]
    pub fn set_service_access_role_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_access_role_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `stream_arn`.\n"]
    pub fn set_stream_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.stream_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `use_large_integer_value`.\n"]
    pub fn set_use_large_integer_value(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.use_large_integer_value = Some(v.into());
        self
    }
}
impl ToListMappable for DmsEndpointKinesisSettingsEl {
    type O = BlockAssignable<DmsEndpointKinesisSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDmsEndpointKinesisSettingsEl {}
impl BuildDmsEndpointKinesisSettingsEl {
    pub fn build(self) -> DmsEndpointKinesisSettingsEl {
        DmsEndpointKinesisSettingsEl {
            include_control_details: core::default::Default::default(),
            include_null_and_empty: core::default::Default::default(),
            include_partition_value: core::default::Default::default(),
            include_table_alter_operations: core::default::Default::default(),
            include_transaction_details: core::default::Default::default(),
            message_format: core::default::Default::default(),
            partition_include_schema_table: core::default::Default::default(),
            service_access_role_arn: core::default::Default::default(),
            stream_arn: core::default::Default::default(),
            use_large_integer_value: core::default::Default::default(),
        }
    }
}
pub struct DmsEndpointKinesisSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DmsEndpointKinesisSettingsElRef {
    fn new(shared: StackShared, base: String) -> DmsEndpointKinesisSettingsElRef {
        DmsEndpointKinesisSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DmsEndpointKinesisSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `include_control_details` after provisioning.\n"]
    pub fn include_control_details(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.include_control_details", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `include_null_and_empty` after provisioning.\n"]
    pub fn include_null_and_empty(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.include_null_and_empty", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `include_partition_value` after provisioning.\n"]
    pub fn include_partition_value(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.include_partition_value", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `include_table_alter_operations` after provisioning.\n"]
    pub fn include_table_alter_operations(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.include_table_alter_operations", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `include_transaction_details` after provisioning.\n"]
    pub fn include_transaction_details(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.include_transaction_details", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `message_format` after provisioning.\n"]
    pub fn message_format(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.message_format", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `partition_include_schema_table` after provisioning.\n"]
    pub fn partition_include_schema_table(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.partition_include_schema_table", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `service_access_role_arn` after provisioning.\n"]
    pub fn service_access_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_access_role_arn", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `stream_arn` after provisioning.\n"]
    pub fn stream_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.stream_arn", self.base))
    }
    #[doc = "Get a reference to the value of field `use_large_integer_value` after provisioning.\n"]
    pub fn use_large_integer_value(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.use_large_integer_value", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DmsEndpointMongodbSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    auth_mechanism: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auth_source: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auth_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    docs_to_investigate: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    extract_doc_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nesting_level: Option<PrimField<String>>,
}
impl DmsEndpointMongodbSettingsEl {
    #[doc = "Set the field `auth_mechanism`.\n"]
    pub fn set_auth_mechanism(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.auth_mechanism = Some(v.into());
        self
    }
    #[doc = "Set the field `auth_source`.\n"]
    pub fn set_auth_source(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.auth_source = Some(v.into());
        self
    }
    #[doc = "Set the field `auth_type`.\n"]
    pub fn set_auth_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.auth_type = Some(v.into());
        self
    }
    #[doc = "Set the field `docs_to_investigate`.\n"]
    pub fn set_docs_to_investigate(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.docs_to_investigate = Some(v.into());
        self
    }
    #[doc = "Set the field `extract_doc_id`.\n"]
    pub fn set_extract_doc_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.extract_doc_id = Some(v.into());
        self
    }
    #[doc = "Set the field `nesting_level`.\n"]
    pub fn set_nesting_level(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.nesting_level = Some(v.into());
        self
    }
}
impl ToListMappable for DmsEndpointMongodbSettingsEl {
    type O = BlockAssignable<DmsEndpointMongodbSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDmsEndpointMongodbSettingsEl {}
impl BuildDmsEndpointMongodbSettingsEl {
    pub fn build(self) -> DmsEndpointMongodbSettingsEl {
        DmsEndpointMongodbSettingsEl {
            auth_mechanism: core::default::Default::default(),
            auth_source: core::default::Default::default(),
            auth_type: core::default::Default::default(),
            docs_to_investigate: core::default::Default::default(),
            extract_doc_id: core::default::Default::default(),
            nesting_level: core::default::Default::default(),
        }
    }
}
pub struct DmsEndpointMongodbSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DmsEndpointMongodbSettingsElRef {
    fn new(shared: StackShared, base: String) -> DmsEndpointMongodbSettingsElRef {
        DmsEndpointMongodbSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DmsEndpointMongodbSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `auth_mechanism` after provisioning.\n"]
    pub fn auth_mechanism(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.auth_mechanism", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `auth_source` after provisioning.\n"]
    pub fn auth_source(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.auth_source", self.base))
    }
    #[doc = "Get a reference to the value of field `auth_type` after provisioning.\n"]
    pub fn auth_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.auth_type", self.base))
    }
    #[doc = "Get a reference to the value of field `docs_to_investigate` after provisioning.\n"]
    pub fn docs_to_investigate(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.docs_to_investigate", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `extract_doc_id` after provisioning.\n"]
    pub fn extract_doc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.extract_doc_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `nesting_level` after provisioning.\n"]
    pub fn nesting_level(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.nesting_level", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DmsEndpointMysqlSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    after_connect_script: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authentication_method: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    clean_source_metadata_on_mismatch: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    events_poll_interval: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    execute_timeout: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_file_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parallel_load_threads: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    server_timezone: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_access_role_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_db_type: Option<PrimField<String>>,
}
impl DmsEndpointMysqlSettingsEl {
    #[doc = "Set the field `after_connect_script`.\n"]
    pub fn set_after_connect_script(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.after_connect_script = Some(v.into());
        self
    }
    #[doc = "Set the field `authentication_method`.\n"]
    pub fn set_authentication_method(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.authentication_method = Some(v.into());
        self
    }
    #[doc = "Set the field `clean_source_metadata_on_mismatch`.\n"]
    pub fn set_clean_source_metadata_on_mismatch(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.clean_source_metadata_on_mismatch = Some(v.into());
        self
    }
    #[doc = "Set the field `events_poll_interval`.\n"]
    pub fn set_events_poll_interval(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.events_poll_interval = Some(v.into());
        self
    }
    #[doc = "Set the field `execute_timeout`.\n"]
    pub fn set_execute_timeout(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.execute_timeout = Some(v.into());
        self
    }
    #[doc = "Set the field `max_file_size`.\n"]
    pub fn set_max_file_size(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_file_size = Some(v.into());
        self
    }
    #[doc = "Set the field `parallel_load_threads`.\n"]
    pub fn set_parallel_load_threads(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.parallel_load_threads = Some(v.into());
        self
    }
    #[doc = "Set the field `server_timezone`.\n"]
    pub fn set_server_timezone(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.server_timezone = Some(v.into());
        self
    }
    #[doc = "Set the field `service_access_role_arn`.\n"]
    pub fn set_service_access_role_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_access_role_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `target_db_type`.\n"]
    pub fn set_target_db_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.target_db_type = Some(v.into());
        self
    }
}
impl ToListMappable for DmsEndpointMysqlSettingsEl {
    type O = BlockAssignable<DmsEndpointMysqlSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDmsEndpointMysqlSettingsEl {}
impl BuildDmsEndpointMysqlSettingsEl {
    pub fn build(self) -> DmsEndpointMysqlSettingsEl {
        DmsEndpointMysqlSettingsEl {
            after_connect_script: core::default::Default::default(),
            authentication_method: core::default::Default::default(),
            clean_source_metadata_on_mismatch: core::default::Default::default(),
            events_poll_interval: core::default::Default::default(),
            execute_timeout: core::default::Default::default(),
            max_file_size: core::default::Default::default(),
            parallel_load_threads: core::default::Default::default(),
            server_timezone: core::default::Default::default(),
            service_access_role_arn: core::default::Default::default(),
            target_db_type: core::default::Default::default(),
        }
    }
}
pub struct DmsEndpointMysqlSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DmsEndpointMysqlSettingsElRef {
    fn new(shared: StackShared, base: String) -> DmsEndpointMysqlSettingsElRef {
        DmsEndpointMysqlSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DmsEndpointMysqlSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `after_connect_script` after provisioning.\n"]
    pub fn after_connect_script(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.after_connect_script", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `authentication_method` after provisioning.\n"]
    pub fn authentication_method(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.authentication_method", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `clean_source_metadata_on_mismatch` after provisioning.\n"]
    pub fn clean_source_metadata_on_mismatch(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.clean_source_metadata_on_mismatch", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `events_poll_interval` after provisioning.\n"]
    pub fn events_poll_interval(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.events_poll_interval", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `execute_timeout` after provisioning.\n"]
    pub fn execute_timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.execute_timeout", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_file_size` after provisioning.\n"]
    pub fn max_file_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_file_size", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `parallel_load_threads` after provisioning.\n"]
    pub fn parallel_load_threads(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parallel_load_threads", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `server_timezone` after provisioning.\n"]
    pub fn server_timezone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.server_timezone", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `service_access_role_arn` after provisioning.\n"]
    pub fn service_access_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_access_role_arn", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `target_db_type` after provisioning.\n"]
    pub fn target_db_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_db_type", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DmsEndpointOracleSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    authentication_method: Option<PrimField<String>>,
}
impl DmsEndpointOracleSettingsEl {
    #[doc = "Set the field `authentication_method`.\n"]
    pub fn set_authentication_method(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.authentication_method = Some(v.into());
        self
    }
}
impl ToListMappable for DmsEndpointOracleSettingsEl {
    type O = BlockAssignable<DmsEndpointOracleSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDmsEndpointOracleSettingsEl {}
impl BuildDmsEndpointOracleSettingsEl {
    pub fn build(self) -> DmsEndpointOracleSettingsEl {
        DmsEndpointOracleSettingsEl {
            authentication_method: core::default::Default::default(),
        }
    }
}
pub struct DmsEndpointOracleSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DmsEndpointOracleSettingsElRef {
    fn new(shared: StackShared, base: String) -> DmsEndpointOracleSettingsElRef {
        DmsEndpointOracleSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DmsEndpointOracleSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `authentication_method` after provisioning.\n"]
    pub fn authentication_method(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.authentication_method", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DmsEndpointPostgresSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    after_connect_script: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authentication_method: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    babelfish_database_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    capture_ddls: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    database_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ddl_artifacts_schema: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    execute_timeout: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fail_tasks_on_lob_truncation: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    heartbeat_enable: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    heartbeat_frequency: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    heartbeat_schema: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    map_boolean_as_boolean: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    map_jsonb_as_clob: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    map_long_varchar_as: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_file_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    plugin_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_access_role_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    slot_name: Option<PrimField<String>>,
}
impl DmsEndpointPostgresSettingsEl {
    #[doc = "Set the field `after_connect_script`.\n"]
    pub fn set_after_connect_script(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.after_connect_script = Some(v.into());
        self
    }
    #[doc = "Set the field `authentication_method`.\n"]
    pub fn set_authentication_method(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.authentication_method = Some(v.into());
        self
    }
    #[doc = "Set the field `babelfish_database_name`.\n"]
    pub fn set_babelfish_database_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.babelfish_database_name = Some(v.into());
        self
    }
    #[doc = "Set the field `capture_ddls`.\n"]
    pub fn set_capture_ddls(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.capture_ddls = Some(v.into());
        self
    }
    #[doc = "Set the field `database_mode`.\n"]
    pub fn set_database_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.database_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `ddl_artifacts_schema`.\n"]
    pub fn set_ddl_artifacts_schema(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ddl_artifacts_schema = Some(v.into());
        self
    }
    #[doc = "Set the field `execute_timeout`.\n"]
    pub fn set_execute_timeout(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.execute_timeout = Some(v.into());
        self
    }
    #[doc = "Set the field `fail_tasks_on_lob_truncation`.\n"]
    pub fn set_fail_tasks_on_lob_truncation(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.fail_tasks_on_lob_truncation = Some(v.into());
        self
    }
    #[doc = "Set the field `heartbeat_enable`.\n"]
    pub fn set_heartbeat_enable(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.heartbeat_enable = Some(v.into());
        self
    }
    #[doc = "Set the field `heartbeat_frequency`.\n"]
    pub fn set_heartbeat_frequency(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.heartbeat_frequency = Some(v.into());
        self
    }
    #[doc = "Set the field `heartbeat_schema`.\n"]
    pub fn set_heartbeat_schema(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.heartbeat_schema = Some(v.into());
        self
    }
    #[doc = "Set the field `map_boolean_as_boolean`.\n"]
    pub fn set_map_boolean_as_boolean(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.map_boolean_as_boolean = Some(v.into());
        self
    }
    #[doc = "Set the field `map_jsonb_as_clob`.\n"]
    pub fn set_map_jsonb_as_clob(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.map_jsonb_as_clob = Some(v.into());
        self
    }
    #[doc = "Set the field `map_long_varchar_as`.\n"]
    pub fn set_map_long_varchar_as(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.map_long_varchar_as = Some(v.into());
        self
    }
    #[doc = "Set the field `max_file_size`.\n"]
    pub fn set_max_file_size(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_file_size = Some(v.into());
        self
    }
    #[doc = "Set the field `plugin_name`.\n"]
    pub fn set_plugin_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.plugin_name = Some(v.into());
        self
    }
    #[doc = "Set the field `service_access_role_arn`.\n"]
    pub fn set_service_access_role_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_access_role_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `slot_name`.\n"]
    pub fn set_slot_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.slot_name = Some(v.into());
        self
    }
}
impl ToListMappable for DmsEndpointPostgresSettingsEl {
    type O = BlockAssignable<DmsEndpointPostgresSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDmsEndpointPostgresSettingsEl {}
impl BuildDmsEndpointPostgresSettingsEl {
    pub fn build(self) -> DmsEndpointPostgresSettingsEl {
        DmsEndpointPostgresSettingsEl {
            after_connect_script: core::default::Default::default(),
            authentication_method: core::default::Default::default(),
            babelfish_database_name: core::default::Default::default(),
            capture_ddls: core::default::Default::default(),
            database_mode: core::default::Default::default(),
            ddl_artifacts_schema: core::default::Default::default(),
            execute_timeout: core::default::Default::default(),
            fail_tasks_on_lob_truncation: core::default::Default::default(),
            heartbeat_enable: core::default::Default::default(),
            heartbeat_frequency: core::default::Default::default(),
            heartbeat_schema: core::default::Default::default(),
            map_boolean_as_boolean: core::default::Default::default(),
            map_jsonb_as_clob: core::default::Default::default(),
            map_long_varchar_as: core::default::Default::default(),
            max_file_size: core::default::Default::default(),
            plugin_name: core::default::Default::default(),
            service_access_role_arn: core::default::Default::default(),
            slot_name: core::default::Default::default(),
        }
    }
}
pub struct DmsEndpointPostgresSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DmsEndpointPostgresSettingsElRef {
    fn new(shared: StackShared, base: String) -> DmsEndpointPostgresSettingsElRef {
        DmsEndpointPostgresSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DmsEndpointPostgresSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `after_connect_script` after provisioning.\n"]
    pub fn after_connect_script(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.after_connect_script", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `authentication_method` after provisioning.\n"]
    pub fn authentication_method(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.authentication_method", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `babelfish_database_name` after provisioning.\n"]
    pub fn babelfish_database_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.babelfish_database_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `capture_ddls` after provisioning.\n"]
    pub fn capture_ddls(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.capture_ddls", self.base))
    }
    #[doc = "Get a reference to the value of field `database_mode` after provisioning.\n"]
    pub fn database_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.database_mode", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ddl_artifacts_schema` after provisioning.\n"]
    pub fn ddl_artifacts_schema(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ddl_artifacts_schema", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `execute_timeout` after provisioning.\n"]
    pub fn execute_timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.execute_timeout", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `fail_tasks_on_lob_truncation` after provisioning.\n"]
    pub fn fail_tasks_on_lob_truncation(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.fail_tasks_on_lob_truncation", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `heartbeat_enable` after provisioning.\n"]
    pub fn heartbeat_enable(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.heartbeat_enable", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `heartbeat_frequency` after provisioning.\n"]
    pub fn heartbeat_frequency(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.heartbeat_frequency", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `heartbeat_schema` after provisioning.\n"]
    pub fn heartbeat_schema(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.heartbeat_schema", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `map_boolean_as_boolean` after provisioning.\n"]
    pub fn map_boolean_as_boolean(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.map_boolean_as_boolean", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `map_jsonb_as_clob` after provisioning.\n"]
    pub fn map_jsonb_as_clob(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.map_jsonb_as_clob", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `map_long_varchar_as` after provisioning.\n"]
    pub fn map_long_varchar_as(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.map_long_varchar_as", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_file_size` after provisioning.\n"]
    pub fn max_file_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_file_size", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `plugin_name` after provisioning.\n"]
    pub fn plugin_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.plugin_name", self.base))
    }
    #[doc = "Get a reference to the value of field `service_access_role_arn` after provisioning.\n"]
    pub fn service_access_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_access_role_arn", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `slot_name` after provisioning.\n"]
    pub fn slot_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.slot_name", self.base))
    }
}
#[derive(Serialize)]
pub struct DmsEndpointRedisSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    auth_password: Option<PrimField<String>>,
    auth_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auth_user_name: Option<PrimField<String>>,
    port: PrimField<f64>,
    server_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ssl_ca_certificate_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ssl_security_protocol: Option<PrimField<String>>,
}
impl DmsEndpointRedisSettingsEl {
    #[doc = "Set the field `auth_password`.\n"]
    pub fn set_auth_password(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.auth_password = Some(v.into());
        self
    }
    #[doc = "Set the field `auth_user_name`.\n"]
    pub fn set_auth_user_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.auth_user_name = Some(v.into());
        self
    }
    #[doc = "Set the field `ssl_ca_certificate_arn`.\n"]
    pub fn set_ssl_ca_certificate_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ssl_ca_certificate_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `ssl_security_protocol`.\n"]
    pub fn set_ssl_security_protocol(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ssl_security_protocol = Some(v.into());
        self
    }
}
impl ToListMappable for DmsEndpointRedisSettingsEl {
    type O = BlockAssignable<DmsEndpointRedisSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDmsEndpointRedisSettingsEl {
    #[doc = ""]
    pub auth_type: PrimField<String>,
    #[doc = ""]
    pub port: PrimField<f64>,
    #[doc = ""]
    pub server_name: PrimField<String>,
}
impl BuildDmsEndpointRedisSettingsEl {
    pub fn build(self) -> DmsEndpointRedisSettingsEl {
        DmsEndpointRedisSettingsEl {
            auth_password: core::default::Default::default(),
            auth_type: self.auth_type,
            auth_user_name: core::default::Default::default(),
            port: self.port,
            server_name: self.server_name,
            ssl_ca_certificate_arn: core::default::Default::default(),
            ssl_security_protocol: core::default::Default::default(),
        }
    }
}
pub struct DmsEndpointRedisSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DmsEndpointRedisSettingsElRef {
    fn new(shared: StackShared, base: String) -> DmsEndpointRedisSettingsElRef {
        DmsEndpointRedisSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DmsEndpointRedisSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `auth_password` after provisioning.\n"]
    pub fn auth_password(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.auth_password", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `auth_type` after provisioning.\n"]
    pub fn auth_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.auth_type", self.base))
    }
    #[doc = "Get a reference to the value of field `auth_user_name` after provisioning.\n"]
    pub fn auth_user_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.auth_user_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
    #[doc = "Get a reference to the value of field `server_name` after provisioning.\n"]
    pub fn server_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.server_name", self.base))
    }
    #[doc = "Get a reference to the value of field `ssl_ca_certificate_arn` after provisioning.\n"]
    pub fn ssl_ca_certificate_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ssl_ca_certificate_arn", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ssl_security_protocol` after provisioning.\n"]
    pub fn ssl_security_protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ssl_security_protocol", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DmsEndpointRedshiftSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_folder: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    server_side_encryption_kms_key_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_access_role_arn: Option<PrimField<String>>,
}
impl DmsEndpointRedshiftSettingsEl {
    #[doc = "Set the field `bucket_folder`.\n"]
    pub fn set_bucket_folder(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket_folder = Some(v.into());
        self
    }
    #[doc = "Set the field `bucket_name`.\n"]
    pub fn set_bucket_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket_name = Some(v.into());
        self
    }
    #[doc = "Set the field `encryption_mode`.\n"]
    pub fn set_encryption_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.encryption_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `server_side_encryption_kms_key_id`.\n"]
    pub fn set_server_side_encryption_kms_key_id(
        mut self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.server_side_encryption_kms_key_id = Some(v.into());
        self
    }
    #[doc = "Set the field `service_access_role_arn`.\n"]
    pub fn set_service_access_role_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_access_role_arn = Some(v.into());
        self
    }
}
impl ToListMappable for DmsEndpointRedshiftSettingsEl {
    type O = BlockAssignable<DmsEndpointRedshiftSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDmsEndpointRedshiftSettingsEl {}
impl BuildDmsEndpointRedshiftSettingsEl {
    pub fn build(self) -> DmsEndpointRedshiftSettingsEl {
        DmsEndpointRedshiftSettingsEl {
            bucket_folder: core::default::Default::default(),
            bucket_name: core::default::Default::default(),
            encryption_mode: core::default::Default::default(),
            server_side_encryption_kms_key_id: core::default::Default::default(),
            service_access_role_arn: core::default::Default::default(),
        }
    }
}
pub struct DmsEndpointRedshiftSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DmsEndpointRedshiftSettingsElRef {
    fn new(shared: StackShared, base: String) -> DmsEndpointRedshiftSettingsElRef {
        DmsEndpointRedshiftSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DmsEndpointRedshiftSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bucket_folder` after provisioning.\n"]
    pub fn bucket_folder(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bucket_folder", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }
    #[doc = "Get a reference to the value of field `encryption_mode` after provisioning.\n"]
    pub fn encryption_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.encryption_mode", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `server_side_encryption_kms_key_id` after provisioning.\n"]
    pub fn server_side_encryption_kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.server_side_encryption_kms_key_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `service_access_role_arn` after provisioning.\n"]
    pub fn service_access_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_access_role_arn", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DmsEndpointTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}
impl DmsEndpointTimeoutsEl {
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
}
impl ToListMappable for DmsEndpointTimeoutsEl {
    type O = BlockAssignable<DmsEndpointTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDmsEndpointTimeoutsEl {}
impl BuildDmsEndpointTimeoutsEl {
    pub fn build(self) -> DmsEndpointTimeoutsEl {
        DmsEndpointTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}
pub struct DmsEndpointTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DmsEndpointTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DmsEndpointTimeoutsElRef {
        DmsEndpointTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DmsEndpointTimeoutsElRef {
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
}
#[derive(Serialize, Default)]
struct DmsEndpointDynamic {
    elasticsearch_settings: Option<DynamicBlock<DmsEndpointElasticsearchSettingsEl>>,
    kafka_settings: Option<DynamicBlock<DmsEndpointKafkaSettingsEl>>,
    kinesis_settings: Option<DynamicBlock<DmsEndpointKinesisSettingsEl>>,
    mongodb_settings: Option<DynamicBlock<DmsEndpointMongodbSettingsEl>>,
    mysql_settings: Option<DynamicBlock<DmsEndpointMysqlSettingsEl>>,
    oracle_settings: Option<DynamicBlock<DmsEndpointOracleSettingsEl>>,
    postgres_settings: Option<DynamicBlock<DmsEndpointPostgresSettingsEl>>,
    redis_settings: Option<DynamicBlock<DmsEndpointRedisSettingsEl>>,
    redshift_settings: Option<DynamicBlock<DmsEndpointRedshiftSettingsEl>>,
}
