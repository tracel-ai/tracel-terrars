use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ManagedKafkaConnectorData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    configs: Option<RecField<PrimField<String>>>,
    connect_cluster: PrimField<String>,
    connector_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    task_restart_policy: Option<Vec<ManagedKafkaConnectorTaskRestartPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ManagedKafkaConnectorTimeoutsEl>,
    dynamic: ManagedKafkaConnectorDynamic,
}
struct ManagedKafkaConnector_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ManagedKafkaConnectorData>,
}
#[derive(Clone)]
pub struct ManagedKafkaConnector(Rc<ManagedKafkaConnector_>);
impl ManagedKafkaConnector {
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
    #[doc = "Set the field `configs`.\nConnector config as keys/values. The keys of the map are connector property names, for example: 'connector.class', 'tasks.max', 'key.converter'."]
    pub fn set_configs(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().configs = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
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
    #[doc = "Set the field `task_restart_policy`.\n"]
    pub fn set_task_restart_policy(
        self,
        v: impl Into<BlockAssignable<ManagedKafkaConnectorTaskRestartPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().task_restart_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.task_restart_policy = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ManagedKafkaConnectorTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `configs` after provisioning.\nConnector config as keys/values. The keys of the map are connector property names, for example: 'connector.class', 'tasks.max', 'key.converter'."]
    pub fn configs(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.configs", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `connect_cluster` after provisioning.\nThe connect cluster name."]
    pub fn connect_cluster(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.connect_cluster", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `connector_id` after provisioning.\nThe ID to use for the connector, which will become the final component of the connector's name. This value is structured like: 'my-connector-id'."]
    pub fn connector_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.connector_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nID of the location of the Kafka Connect resource. See https://cloud.google.com/managed-kafka/docs/locations for a list of supported locations."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the connector. The 'connector' segment is used when connecting directly to the connect cluster. Structured like: 'projects/PROJECT_ID/locations/LOCATION/connectClusters/CONNECT_CLUSTER/connectors/CONNECTOR_ID'."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe current state of the connect. Possible values: 'STATE_UNSPECIFIED', 'UNASSIGNED', 'RUNNING', 'PAUSED', 'FAILED', 'RESTARTING', and 'STOPPED'."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `task_restart_policy` after provisioning.\n"]
    pub fn task_restart_policy(&self) -> ListRef<ManagedKafkaConnectorTaskRestartPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.task_restart_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ManagedKafkaConnectorTimeoutsElRef {
        ManagedKafkaConnectorTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ManagedKafkaConnector {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ManagedKafkaConnector {}
impl ToListMappable for ManagedKafkaConnector {
    type O = ListRef<ManagedKafkaConnectorRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ManagedKafkaConnector_ {
    fn extract_resource_type(&self) -> String {
        "google_managed_kafka_connector".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildManagedKafkaConnector {
    pub tf_id: String,
    #[doc = "The connect cluster name."]
    pub connect_cluster: PrimField<String>,
    #[doc = "The ID to use for the connector, which will become the final component of the connector's name. This value is structured like: 'my-connector-id'."]
    pub connector_id: PrimField<String>,
    #[doc = "ID of the location of the Kafka Connect resource. See https://cloud.google.com/managed-kafka/docs/locations for a list of supported locations."]
    pub location: PrimField<String>,
}
impl BuildManagedKafkaConnector {
    pub fn build(self, stack: &mut Stack) -> ManagedKafkaConnector {
        let out = ManagedKafkaConnector(Rc::new(ManagedKafkaConnector_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ManagedKafkaConnectorData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                configs: core::default::Default::default(),
                connect_cluster: self.connect_cluster,
                connector_id: self.connector_id,
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                task_restart_policy: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ManagedKafkaConnectorRef {
    shared: StackShared,
    base: String,
}
impl Ref for ManagedKafkaConnectorRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ManagedKafkaConnectorRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `configs` after provisioning.\nConnector config as keys/values. The keys of the map are connector property names, for example: 'connector.class', 'tasks.max', 'key.converter'."]
    pub fn configs(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.configs", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `connect_cluster` after provisioning.\nThe connect cluster name."]
    pub fn connect_cluster(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.connect_cluster", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `connector_id` after provisioning.\nThe ID to use for the connector, which will become the final component of the connector's name. This value is structured like: 'my-connector-id'."]
    pub fn connector_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.connector_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nID of the location of the Kafka Connect resource. See https://cloud.google.com/managed-kafka/docs/locations for a list of supported locations."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the connector. The 'connector' segment is used when connecting directly to the connect cluster. Structured like: 'projects/PROJECT_ID/locations/LOCATION/connectClusters/CONNECT_CLUSTER/connectors/CONNECTOR_ID'."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe current state of the connect. Possible values: 'STATE_UNSPECIFIED', 'UNASSIGNED', 'RUNNING', 'PAUSED', 'FAILED', 'RESTARTING', and 'STOPPED'."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `task_restart_policy` after provisioning.\n"]
    pub fn task_restart_policy(&self) -> ListRef<ManagedKafkaConnectorTaskRestartPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.task_restart_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ManagedKafkaConnectorTimeoutsElRef {
        ManagedKafkaConnectorTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ManagedKafkaConnectorTaskRestartPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    maximum_backoff: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minimum_backoff: Option<PrimField<String>>,
}
impl ManagedKafkaConnectorTaskRestartPolicyEl {
    #[doc = "Set the field `maximum_backoff`.\nThe maximum amount of time to wait before retrying a failed task. This sets an upper bound for the backoff delay.\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\"."]
    pub fn set_maximum_backoff(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.maximum_backoff = Some(v.into());
        self
    }
    #[doc = "Set the field `minimum_backoff`.\nThe minimum amount of time to wait before retrying a failed task. This sets a lower bound for the backoff delay.\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\"."]
    pub fn set_minimum_backoff(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.minimum_backoff = Some(v.into());
        self
    }
}
impl ToListMappable for ManagedKafkaConnectorTaskRestartPolicyEl {
    type O = BlockAssignable<ManagedKafkaConnectorTaskRestartPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildManagedKafkaConnectorTaskRestartPolicyEl {}
impl BuildManagedKafkaConnectorTaskRestartPolicyEl {
    pub fn build(self) -> ManagedKafkaConnectorTaskRestartPolicyEl {
        ManagedKafkaConnectorTaskRestartPolicyEl {
            maximum_backoff: core::default::Default::default(),
            minimum_backoff: core::default::Default::default(),
        }
    }
}
pub struct ManagedKafkaConnectorTaskRestartPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ManagedKafkaConnectorTaskRestartPolicyElRef {
    fn new(shared: StackShared, base: String) -> ManagedKafkaConnectorTaskRestartPolicyElRef {
        ManagedKafkaConnectorTaskRestartPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ManagedKafkaConnectorTaskRestartPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `maximum_backoff` after provisioning.\nThe maximum amount of time to wait before retrying a failed task. This sets an upper bound for the backoff delay.\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\"."]
    pub fn maximum_backoff(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.maximum_backoff", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `minimum_backoff` after provisioning.\nThe minimum amount of time to wait before retrying a failed task. This sets a lower bound for the backoff delay.\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\"."]
    pub fn minimum_backoff(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.minimum_backoff", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ManagedKafkaConnectorTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ManagedKafkaConnectorTimeoutsEl {
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
impl ToListMappable for ManagedKafkaConnectorTimeoutsEl {
    type O = BlockAssignable<ManagedKafkaConnectorTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildManagedKafkaConnectorTimeoutsEl {}
impl BuildManagedKafkaConnectorTimeoutsEl {
    pub fn build(self) -> ManagedKafkaConnectorTimeoutsEl {
        ManagedKafkaConnectorTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ManagedKafkaConnectorTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ManagedKafkaConnectorTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ManagedKafkaConnectorTimeoutsElRef {
        ManagedKafkaConnectorTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ManagedKafkaConnectorTimeoutsElRef {
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
struct ManagedKafkaConnectorDynamic {
    task_restart_policy: Option<DynamicBlock<ManagedKafkaConnectorTaskRestartPolicyEl>>,
}
