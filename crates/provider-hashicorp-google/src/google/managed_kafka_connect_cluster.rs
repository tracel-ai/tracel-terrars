use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ManagedKafkaConnectClusterData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    connect_cluster_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    kafka_cluster: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    capacity_config: Option<Vec<ManagedKafkaConnectClusterCapacityConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gcp_config: Option<Vec<ManagedKafkaConnectClusterGcpConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ManagedKafkaConnectClusterTimeoutsEl>,
    dynamic: ManagedKafkaConnectClusterDynamic,
}
struct ManagedKafkaConnectCluster_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ManagedKafkaConnectClusterData>,
}
#[derive(Clone)]
pub struct ManagedKafkaConnectCluster(Rc<ManagedKafkaConnectCluster_>);
impl ManagedKafkaConnectCluster {
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
    #[doc = "Set the field `labels`.\nList of label KEY=VALUE pairs to add. Keys must start with a lowercase character and contain only hyphens (-), underscores (\u{a0}), lowercase characters, and numbers. Values must contain only hyphens (-), underscores (\u{a0}), lowercase characters, and numbers.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `capacity_config`.\n"]
    pub fn set_capacity_config(
        self,
        v: impl Into<BlockAssignable<ManagedKafkaConnectClusterCapacityConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().capacity_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.capacity_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `gcp_config`.\n"]
    pub fn set_gcp_config(
        self,
        v: impl Into<BlockAssignable<ManagedKafkaConnectClusterGcpConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().gcp_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.gcp_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ManagedKafkaConnectClusterTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `connect_cluster_id` after provisioning.\nThe ID to use for the Connect Cluster, which will become the final component of the connect cluster's name. This value is structured like: 'my-connect-cluster-id'."]
    pub fn connect_cluster_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.connect_cluster_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe time when the cluster was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `kafka_cluster` after provisioning.\nThe name of the Kafka cluster this Kafka Connect cluster is attached to. Structured like: 'projects/PROJECT_ID/locations/LOCATION/clusters/CLUSTER_ID'."]
    pub fn kafka_cluster(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kafka_cluster", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nList of label KEY=VALUE pairs to add. Keys must start with a lowercase character and contain only hyphens (-), underscores (\u{a0}), lowercase characters, and numbers. Values must contain only hyphens (-), underscores (\u{a0}), lowercase characters, and numbers.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nID of the location of the Kafka Connect resource. See https://cloud.google.com/managed-kafka/docs/locations for a list of supported locations."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the connect cluster. Structured like: 'projects/PROJECT_ID/locations/LOCATION/connectClusters/CONNECT_CLUSTER_ID'."]
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
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe current state of the connect cluster. Possible values: 'STATE_UNSPECIFIED', 'CREATING', 'ACTIVE', 'DELETING'."]
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
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe time when the cluster was last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `capacity_config` after provisioning.\n"]
    pub fn capacity_config(&self) -> ListRef<ManagedKafkaConnectClusterCapacityConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.capacity_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `gcp_config` after provisioning.\n"]
    pub fn gcp_config(&self) -> ListRef<ManagedKafkaConnectClusterGcpConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.gcp_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ManagedKafkaConnectClusterTimeoutsElRef {
        ManagedKafkaConnectClusterTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ManagedKafkaConnectCluster {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ManagedKafkaConnectCluster {}
impl ToListMappable for ManagedKafkaConnectCluster {
    type O = ListRef<ManagedKafkaConnectClusterRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ManagedKafkaConnectCluster_ {
    fn extract_resource_type(&self) -> String {
        "google_managed_kafka_connect_cluster".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildManagedKafkaConnectCluster {
    pub tf_id: String,
    #[doc = "The ID to use for the Connect Cluster, which will become the final component of the connect cluster's name. This value is structured like: 'my-connect-cluster-id'."]
    pub connect_cluster_id: PrimField<String>,
    #[doc = "The name of the Kafka cluster this Kafka Connect cluster is attached to. Structured like: 'projects/PROJECT_ID/locations/LOCATION/clusters/CLUSTER_ID'."]
    pub kafka_cluster: PrimField<String>,
    #[doc = "ID of the location of the Kafka Connect resource. See https://cloud.google.com/managed-kafka/docs/locations for a list of supported locations."]
    pub location: PrimField<String>,
}
impl BuildManagedKafkaConnectCluster {
    pub fn build(self, stack: &mut Stack) -> ManagedKafkaConnectCluster {
        let out = ManagedKafkaConnectCluster(Rc::new(ManagedKafkaConnectCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ManagedKafkaConnectClusterData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                connect_cluster_id: self.connect_cluster_id,
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                kafka_cluster: self.kafka_cluster,
                labels: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                capacity_config: core::default::Default::default(),
                gcp_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ManagedKafkaConnectClusterRef {
    shared: StackShared,
    base: String,
}
impl Ref for ManagedKafkaConnectClusterRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ManagedKafkaConnectClusterRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `connect_cluster_id` after provisioning.\nThe ID to use for the Connect Cluster, which will become the final component of the connect cluster's name. This value is structured like: 'my-connect-cluster-id'."]
    pub fn connect_cluster_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.connect_cluster_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe time when the cluster was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `kafka_cluster` after provisioning.\nThe name of the Kafka cluster this Kafka Connect cluster is attached to. Structured like: 'projects/PROJECT_ID/locations/LOCATION/clusters/CLUSTER_ID'."]
    pub fn kafka_cluster(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kafka_cluster", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nList of label KEY=VALUE pairs to add. Keys must start with a lowercase character and contain only hyphens (-), underscores (\u{a0}), lowercase characters, and numbers. Values must contain only hyphens (-), underscores (\u{a0}), lowercase characters, and numbers.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nID of the location of the Kafka Connect resource. See https://cloud.google.com/managed-kafka/docs/locations for a list of supported locations."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the connect cluster. Structured like: 'projects/PROJECT_ID/locations/LOCATION/connectClusters/CONNECT_CLUSTER_ID'."]
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
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe current state of the connect cluster. Possible values: 'STATE_UNSPECIFIED', 'CREATING', 'ACTIVE', 'DELETING'."]
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
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe time when the cluster was last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `capacity_config` after provisioning.\n"]
    pub fn capacity_config(&self) -> ListRef<ManagedKafkaConnectClusterCapacityConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.capacity_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `gcp_config` after provisioning.\n"]
    pub fn gcp_config(&self) -> ListRef<ManagedKafkaConnectClusterGcpConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.gcp_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ManagedKafkaConnectClusterTimeoutsElRef {
        ManagedKafkaConnectClusterTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ManagedKafkaConnectClusterCapacityConfigEl {
    memory_bytes: PrimField<String>,
    vcpu_count: PrimField<String>,
}
impl ManagedKafkaConnectClusterCapacityConfigEl {}
impl ToListMappable for ManagedKafkaConnectClusterCapacityConfigEl {
    type O = BlockAssignable<ManagedKafkaConnectClusterCapacityConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildManagedKafkaConnectClusterCapacityConfigEl {
    #[doc = "The memory to provision for the cluster in bytes. The CPU:memory ratio (vCPU:GiB) must be between 1:1 and 1:8. Minimum: 3221225472 (3 GiB)."]
    pub memory_bytes: PrimField<String>,
    #[doc = "The number of vCPUs to provision for the cluster. The minimum is 3."]
    pub vcpu_count: PrimField<String>,
}
impl BuildManagedKafkaConnectClusterCapacityConfigEl {
    pub fn build(self) -> ManagedKafkaConnectClusterCapacityConfigEl {
        ManagedKafkaConnectClusterCapacityConfigEl {
            memory_bytes: self.memory_bytes,
            vcpu_count: self.vcpu_count,
        }
    }
}
pub struct ManagedKafkaConnectClusterCapacityConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ManagedKafkaConnectClusterCapacityConfigElRef {
    fn new(shared: StackShared, base: String) -> ManagedKafkaConnectClusterCapacityConfigElRef {
        ManagedKafkaConnectClusterCapacityConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ManagedKafkaConnectClusterCapacityConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `memory_bytes` after provisioning.\nThe memory to provision for the cluster in bytes. The CPU:memory ratio (vCPU:GiB) must be between 1:1 and 1:8. Minimum: 3221225472 (3 GiB)."]
    pub fn memory_bytes(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.memory_bytes", self.base))
    }
    #[doc = "Get a reference to the value of field `vcpu_count` after provisioning.\nThe number of vCPUs to provision for the cluster. The minimum is 3."]
    pub fn vcpu_count(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vcpu_count", self.base))
    }
}
#[derive(Serialize)]
pub struct ManagedKafkaConnectClusterGcpConfigElAccessConfigElNetworkConfigsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    additional_subnets: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dns_domain_names: Option<ListField<PrimField<String>>>,
    primary_subnet: PrimField<String>,
}
impl ManagedKafkaConnectClusterGcpConfigElAccessConfigElNetworkConfigsEl {
    #[doc = "Set the field `additional_subnets`.\nAdditional subnets may be specified. They may be in another region, but must be in the same VPC network. The Connect workers can communicate with network endpoints in either the primary or additional subnets."]
    pub fn set_additional_subnets(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.additional_subnets = Some(v.into());
        self
    }
    #[doc = "Set the field `dns_domain_names`.\nAdditional DNS domain names from the subnet's network to be made visible to the Connect Cluster. When using MirrorMaker2, it's necessary to add the bootstrap address's dns domain name of the target cluster to make it visible to the connector. For example: my-kafka-cluster.us-central1.managedkafka.my-project.cloud.goog"]
    pub fn set_dns_domain_names(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.dns_domain_names = Some(v.into());
        self
    }
}
impl ToListMappable for ManagedKafkaConnectClusterGcpConfigElAccessConfigElNetworkConfigsEl {
    type O = BlockAssignable<ManagedKafkaConnectClusterGcpConfigElAccessConfigElNetworkConfigsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildManagedKafkaConnectClusterGcpConfigElAccessConfigElNetworkConfigsEl {
    #[doc = "VPC subnet to make available to the Kafka Connect cluster. Structured like: projects/{project}/regions/{region}/subnetworks/{subnet_id}. It is used to create a Private Service Connect (PSC) interface for the Kafka Connect workers. It must be located in the same region as the Kafka Connect cluster. The CIDR range of the subnet must be within the IPv4 address ranges for private networks, as specified in RFC 1918. The primary subnet CIDR range must have a minimum size of /22 (1024 addresses)."]
    pub primary_subnet: PrimField<String>,
}
impl BuildManagedKafkaConnectClusterGcpConfigElAccessConfigElNetworkConfigsEl {
    pub fn build(self) -> ManagedKafkaConnectClusterGcpConfigElAccessConfigElNetworkConfigsEl {
        ManagedKafkaConnectClusterGcpConfigElAccessConfigElNetworkConfigsEl {
            additional_subnets: core::default::Default::default(),
            dns_domain_names: core::default::Default::default(),
            primary_subnet: self.primary_subnet,
        }
    }
}
pub struct ManagedKafkaConnectClusterGcpConfigElAccessConfigElNetworkConfigsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ManagedKafkaConnectClusterGcpConfigElAccessConfigElNetworkConfigsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ManagedKafkaConnectClusterGcpConfigElAccessConfigElNetworkConfigsElRef {
        ManagedKafkaConnectClusterGcpConfigElAccessConfigElNetworkConfigsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ManagedKafkaConnectClusterGcpConfigElAccessConfigElNetworkConfigsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `additional_subnets` after provisioning.\nAdditional subnets may be specified. They may be in another region, but must be in the same VPC network. The Connect workers can communicate with network endpoints in either the primary or additional subnets."]
    pub fn additional_subnets(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.additional_subnets", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `dns_domain_names` after provisioning.\nAdditional DNS domain names from the subnet's network to be made visible to the Connect Cluster. When using MirrorMaker2, it's necessary to add the bootstrap address's dns domain name of the target cluster to make it visible to the connector. For example: my-kafka-cluster.us-central1.managedkafka.my-project.cloud.goog"]
    pub fn dns_domain_names(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dns_domain_names", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `primary_subnet` after provisioning.\nVPC subnet to make available to the Kafka Connect cluster. Structured like: projects/{project}/regions/{region}/subnetworks/{subnet_id}. It is used to create a Private Service Connect (PSC) interface for the Kafka Connect workers. It must be located in the same region as the Kafka Connect cluster. The CIDR range of the subnet must be within the IPv4 address ranges for private networks, as specified in RFC 1918. The primary subnet CIDR range must have a minimum size of /22 (1024 addresses)."]
    pub fn primary_subnet(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.primary_subnet", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ManagedKafkaConnectClusterGcpConfigElAccessConfigElDynamic {
    network_configs:
        Option<DynamicBlock<ManagedKafkaConnectClusterGcpConfigElAccessConfigElNetworkConfigsEl>>,
}
#[derive(Serialize)]
pub struct ManagedKafkaConnectClusterGcpConfigElAccessConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    network_configs:
        Option<Vec<ManagedKafkaConnectClusterGcpConfigElAccessConfigElNetworkConfigsEl>>,
    dynamic: ManagedKafkaConnectClusterGcpConfigElAccessConfigElDynamic,
}
impl ManagedKafkaConnectClusterGcpConfigElAccessConfigEl {
    #[doc = "Set the field `network_configs`.\n"]
    pub fn set_network_configs(
        mut self,
        v: impl Into<
            BlockAssignable<ManagedKafkaConnectClusterGcpConfigElAccessConfigElNetworkConfigsEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.network_configs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.network_configs = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ManagedKafkaConnectClusterGcpConfigElAccessConfigEl {
    type O = BlockAssignable<ManagedKafkaConnectClusterGcpConfigElAccessConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildManagedKafkaConnectClusterGcpConfigElAccessConfigEl {}
impl BuildManagedKafkaConnectClusterGcpConfigElAccessConfigEl {
    pub fn build(self) -> ManagedKafkaConnectClusterGcpConfigElAccessConfigEl {
        ManagedKafkaConnectClusterGcpConfigElAccessConfigEl {
            network_configs: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ManagedKafkaConnectClusterGcpConfigElAccessConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ManagedKafkaConnectClusterGcpConfigElAccessConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ManagedKafkaConnectClusterGcpConfigElAccessConfigElRef {
        ManagedKafkaConnectClusterGcpConfigElAccessConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ManagedKafkaConnectClusterGcpConfigElAccessConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `network_configs` after provisioning.\n"]
    pub fn network_configs(
        &self,
    ) -> ListRef<ManagedKafkaConnectClusterGcpConfigElAccessConfigElNetworkConfigsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_configs", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ManagedKafkaConnectClusterGcpConfigElDynamic {
    access_config: Option<DynamicBlock<ManagedKafkaConnectClusterGcpConfigElAccessConfigEl>>,
}
#[derive(Serialize)]
pub struct ManagedKafkaConnectClusterGcpConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    access_config: Option<Vec<ManagedKafkaConnectClusterGcpConfigElAccessConfigEl>>,
    dynamic: ManagedKafkaConnectClusterGcpConfigElDynamic,
}
impl ManagedKafkaConnectClusterGcpConfigEl {
    #[doc = "Set the field `access_config`.\n"]
    pub fn set_access_config(
        mut self,
        v: impl Into<BlockAssignable<ManagedKafkaConnectClusterGcpConfigElAccessConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.access_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.access_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ManagedKafkaConnectClusterGcpConfigEl {
    type O = BlockAssignable<ManagedKafkaConnectClusterGcpConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildManagedKafkaConnectClusterGcpConfigEl {}
impl BuildManagedKafkaConnectClusterGcpConfigEl {
    pub fn build(self) -> ManagedKafkaConnectClusterGcpConfigEl {
        ManagedKafkaConnectClusterGcpConfigEl {
            access_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ManagedKafkaConnectClusterGcpConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ManagedKafkaConnectClusterGcpConfigElRef {
    fn new(shared: StackShared, base: String) -> ManagedKafkaConnectClusterGcpConfigElRef {
        ManagedKafkaConnectClusterGcpConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ManagedKafkaConnectClusterGcpConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `access_config` after provisioning.\n"]
    pub fn access_config(&self) -> ListRef<ManagedKafkaConnectClusterGcpConfigElAccessConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.access_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ManagedKafkaConnectClusterTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ManagedKafkaConnectClusterTimeoutsEl {
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
impl ToListMappable for ManagedKafkaConnectClusterTimeoutsEl {
    type O = BlockAssignable<ManagedKafkaConnectClusterTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildManagedKafkaConnectClusterTimeoutsEl {}
impl BuildManagedKafkaConnectClusterTimeoutsEl {
    pub fn build(self) -> ManagedKafkaConnectClusterTimeoutsEl {
        ManagedKafkaConnectClusterTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ManagedKafkaConnectClusterTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ManagedKafkaConnectClusterTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ManagedKafkaConnectClusterTimeoutsElRef {
        ManagedKafkaConnectClusterTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ManagedKafkaConnectClusterTimeoutsElRef {
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
struct ManagedKafkaConnectClusterDynamic {
    capacity_config: Option<DynamicBlock<ManagedKafkaConnectClusterCapacityConfigEl>>,
    gcp_config: Option<DynamicBlock<ManagedKafkaConnectClusterGcpConfigEl>>,
}
