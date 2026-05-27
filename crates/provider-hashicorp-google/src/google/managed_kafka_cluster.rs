use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ManagedKafkaClusterData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    cluster_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    broker_capacity_config: Option<Vec<ManagedKafkaClusterBrokerCapacityConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    capacity_config: Option<Vec<ManagedKafkaClusterCapacityConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gcp_config: Option<Vec<ManagedKafkaClusterGcpConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rebalance_config: Option<Vec<ManagedKafkaClusterRebalanceConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ManagedKafkaClusterTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tls_config: Option<Vec<ManagedKafkaClusterTlsConfigEl>>,
    dynamic: ManagedKafkaClusterDynamic,
}
struct ManagedKafkaCluster_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ManagedKafkaClusterData>,
}
#[derive(Clone)]
pub struct ManagedKafkaCluster(Rc<ManagedKafkaCluster_>);
impl ManagedKafkaCluster {
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
    #[doc = "Set the field `broker_capacity_config`.\n"]
    pub fn set_broker_capacity_config(
        self,
        v: impl Into<BlockAssignable<ManagedKafkaClusterBrokerCapacityConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().broker_capacity_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.broker_capacity_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `capacity_config`.\n"]
    pub fn set_capacity_config(
        self,
        v: impl Into<BlockAssignable<ManagedKafkaClusterCapacityConfigEl>>,
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
        v: impl Into<BlockAssignable<ManagedKafkaClusterGcpConfigEl>>,
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
    #[doc = "Set the field `rebalance_config`.\n"]
    pub fn set_rebalance_config(
        self,
        v: impl Into<BlockAssignable<ManagedKafkaClusterRebalanceConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rebalance_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rebalance_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ManagedKafkaClusterTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Set the field `tls_config`.\n"]
    pub fn set_tls_config(
        self,
        v: impl Into<BlockAssignable<ManagedKafkaClusterTlsConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().tls_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.tls_config = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `cluster_id` after provisioning.\nThe ID to use for the cluster, which will become the final component of the cluster's name. The ID must be 1-63 characters long, and match the regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' to comply with RFC 1035. This value is structured like: 'my-cluster-id'."]
    pub fn cluster_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster_id", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nList of label KEY=VALUE pairs to add. Keys must start with a lowercase character and contain only hyphens (-), underscores (\u{a0}), lowercase characters, and numbers. Values must contain only hyphens (-), underscores (\u{a0}), lowercase characters, and numbers.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nID of the location of the Kafka resource. See https://cloud.google.com/managed-kafka/docs/locations for a list of supported locations."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the cluster. Structured like: 'projects/PROJECT_ID/locations/LOCATION/clusters/CLUSTER_ID'."]
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
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe current state of the cluster. Possible values: 'STATE_UNSPECIFIED', 'CREATING', 'ACTIVE', 'DELETING'."]
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
    #[doc = "Get a reference to the value of field `broker_capacity_config` after provisioning.\n"]
    pub fn broker_capacity_config(&self) -> ListRef<ManagedKafkaClusterBrokerCapacityConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.broker_capacity_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `capacity_config` after provisioning.\n"]
    pub fn capacity_config(&self) -> ListRef<ManagedKafkaClusterCapacityConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.capacity_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `gcp_config` after provisioning.\n"]
    pub fn gcp_config(&self) -> ListRef<ManagedKafkaClusterGcpConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.gcp_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rebalance_config` after provisioning.\n"]
    pub fn rebalance_config(&self) -> ListRef<ManagedKafkaClusterRebalanceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rebalance_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ManagedKafkaClusterTimeoutsElRef {
        ManagedKafkaClusterTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tls_config` after provisioning.\n"]
    pub fn tls_config(&self) -> ListRef<ManagedKafkaClusterTlsConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tls_config", self.extract_ref()),
        )
    }
}
impl Referable for ManagedKafkaCluster {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ManagedKafkaCluster {}
impl ToListMappable for ManagedKafkaCluster {
    type O = ListRef<ManagedKafkaClusterRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ManagedKafkaCluster_ {
    fn extract_resource_type(&self) -> String {
        "google_managed_kafka_cluster".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildManagedKafkaCluster {
    pub tf_id: String,
    #[doc = "The ID to use for the cluster, which will become the final component of the cluster's name. The ID must be 1-63 characters long, and match the regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' to comply with RFC 1035. This value is structured like: 'my-cluster-id'."]
    pub cluster_id: PrimField<String>,
    #[doc = "ID of the location of the Kafka resource. See https://cloud.google.com/managed-kafka/docs/locations for a list of supported locations."]
    pub location: PrimField<String>,
}
impl BuildManagedKafkaCluster {
    pub fn build(self, stack: &mut Stack) -> ManagedKafkaCluster {
        let out = ManagedKafkaCluster(Rc::new(ManagedKafkaCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ManagedKafkaClusterData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                cluster_id: self.cluster_id,
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                broker_capacity_config: core::default::Default::default(),
                capacity_config: core::default::Default::default(),
                gcp_config: core::default::Default::default(),
                rebalance_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                tls_config: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ManagedKafkaClusterRef {
    shared: StackShared,
    base: String,
}
impl Ref for ManagedKafkaClusterRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ManagedKafkaClusterRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cluster_id` after provisioning.\nThe ID to use for the cluster, which will become the final component of the cluster's name. The ID must be 1-63 characters long, and match the regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' to comply with RFC 1035. This value is structured like: 'my-cluster-id'."]
    pub fn cluster_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster_id", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nList of label KEY=VALUE pairs to add. Keys must start with a lowercase character and contain only hyphens (-), underscores (\u{a0}), lowercase characters, and numbers. Values must contain only hyphens (-), underscores (\u{a0}), lowercase characters, and numbers.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nID of the location of the Kafka resource. See https://cloud.google.com/managed-kafka/docs/locations for a list of supported locations."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the cluster. Structured like: 'projects/PROJECT_ID/locations/LOCATION/clusters/CLUSTER_ID'."]
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
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe current state of the cluster. Possible values: 'STATE_UNSPECIFIED', 'CREATING', 'ACTIVE', 'DELETING'."]
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
    #[doc = "Get a reference to the value of field `broker_capacity_config` after provisioning.\n"]
    pub fn broker_capacity_config(&self) -> ListRef<ManagedKafkaClusterBrokerCapacityConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.broker_capacity_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `capacity_config` after provisioning.\n"]
    pub fn capacity_config(&self) -> ListRef<ManagedKafkaClusterCapacityConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.capacity_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `gcp_config` after provisioning.\n"]
    pub fn gcp_config(&self) -> ListRef<ManagedKafkaClusterGcpConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.gcp_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rebalance_config` after provisioning.\n"]
    pub fn rebalance_config(&self) -> ListRef<ManagedKafkaClusterRebalanceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rebalance_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ManagedKafkaClusterTimeoutsElRef {
        ManagedKafkaClusterTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tls_config` after provisioning.\n"]
    pub fn tls_config(&self) -> ListRef<ManagedKafkaClusterTlsConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tls_config", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ManagedKafkaClusterBrokerCapacityConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_size_gib: Option<PrimField<String>>,
}
impl ManagedKafkaClusterBrokerCapacityConfigEl {
    #[doc = "Set the field `disk_size_gib`.\nThe disk to provision for each broker in Gibibytes. Minimum: 100 GiB."]
    pub fn set_disk_size_gib(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.disk_size_gib = Some(v.into());
        self
    }
}
impl ToListMappable for ManagedKafkaClusterBrokerCapacityConfigEl {
    type O = BlockAssignable<ManagedKafkaClusterBrokerCapacityConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildManagedKafkaClusterBrokerCapacityConfigEl {}
impl BuildManagedKafkaClusterBrokerCapacityConfigEl {
    pub fn build(self) -> ManagedKafkaClusterBrokerCapacityConfigEl {
        ManagedKafkaClusterBrokerCapacityConfigEl {
            disk_size_gib: core::default::Default::default(),
        }
    }
}
pub struct ManagedKafkaClusterBrokerCapacityConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ManagedKafkaClusterBrokerCapacityConfigElRef {
    fn new(shared: StackShared, base: String) -> ManagedKafkaClusterBrokerCapacityConfigElRef {
        ManagedKafkaClusterBrokerCapacityConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ManagedKafkaClusterBrokerCapacityConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `disk_size_gib` after provisioning.\nThe disk to provision for each broker in Gibibytes. Minimum: 100 GiB."]
    pub fn disk_size_gib(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disk_size_gib", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ManagedKafkaClusterCapacityConfigEl {
    memory_bytes: PrimField<String>,
    vcpu_count: PrimField<String>,
}
impl ManagedKafkaClusterCapacityConfigEl {}
impl ToListMappable for ManagedKafkaClusterCapacityConfigEl {
    type O = BlockAssignable<ManagedKafkaClusterCapacityConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildManagedKafkaClusterCapacityConfigEl {
    #[doc = "The memory to provision for the cluster in bytes. The value must be between 1 GiB and 8 GiB per vCPU. Ex. 1024Mi, 4Gi."]
    pub memory_bytes: PrimField<String>,
    #[doc = "The number of vCPUs to provision for the cluster. The minimum is 3."]
    pub vcpu_count: PrimField<String>,
}
impl BuildManagedKafkaClusterCapacityConfigEl {
    pub fn build(self) -> ManagedKafkaClusterCapacityConfigEl {
        ManagedKafkaClusterCapacityConfigEl {
            memory_bytes: self.memory_bytes,
            vcpu_count: self.vcpu_count,
        }
    }
}
pub struct ManagedKafkaClusterCapacityConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ManagedKafkaClusterCapacityConfigElRef {
    fn new(shared: StackShared, base: String) -> ManagedKafkaClusterCapacityConfigElRef {
        ManagedKafkaClusterCapacityConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ManagedKafkaClusterCapacityConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `memory_bytes` after provisioning.\nThe memory to provision for the cluster in bytes. The value must be between 1 GiB and 8 GiB per vCPU. Ex. 1024Mi, 4Gi."]
    pub fn memory_bytes(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.memory_bytes", self.base))
    }
    #[doc = "Get a reference to the value of field `vcpu_count` after provisioning.\nThe number of vCPUs to provision for the cluster. The minimum is 3."]
    pub fn vcpu_count(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vcpu_count", self.base))
    }
}
#[derive(Serialize)]
pub struct ManagedKafkaClusterGcpConfigElAccessConfigElNetworkConfigsEl {
    subnet: PrimField<String>,
}
impl ManagedKafkaClusterGcpConfigElAccessConfigElNetworkConfigsEl {}
impl ToListMappable for ManagedKafkaClusterGcpConfigElAccessConfigElNetworkConfigsEl {
    type O = BlockAssignable<ManagedKafkaClusterGcpConfigElAccessConfigElNetworkConfigsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildManagedKafkaClusterGcpConfigElAccessConfigElNetworkConfigsEl {
    #[doc = "Name of the VPC subnet from which the cluster is accessible. Both broker and bootstrap server IP addresses and DNS entries are automatically created in the subnet. There can only be one subnet per network, and the subnet must be located in the same region as the cluster. The project may differ. The name of the subnet must be in the format 'projects/PROJECT_ID/regions/REGION/subnetworks/SUBNET'."]
    pub subnet: PrimField<String>,
}
impl BuildManagedKafkaClusterGcpConfigElAccessConfigElNetworkConfigsEl {
    pub fn build(self) -> ManagedKafkaClusterGcpConfigElAccessConfigElNetworkConfigsEl {
        ManagedKafkaClusterGcpConfigElAccessConfigElNetworkConfigsEl {
            subnet: self.subnet,
        }
    }
}
pub struct ManagedKafkaClusterGcpConfigElAccessConfigElNetworkConfigsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ManagedKafkaClusterGcpConfigElAccessConfigElNetworkConfigsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ManagedKafkaClusterGcpConfigElAccessConfigElNetworkConfigsElRef {
        ManagedKafkaClusterGcpConfigElAccessConfigElNetworkConfigsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ManagedKafkaClusterGcpConfigElAccessConfigElNetworkConfigsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `subnet` after provisioning.\nName of the VPC subnet from which the cluster is accessible. Both broker and bootstrap server IP addresses and DNS entries are automatically created in the subnet. There can only be one subnet per network, and the subnet must be located in the same region as the cluster. The project may differ. The name of the subnet must be in the format 'projects/PROJECT_ID/regions/REGION/subnetworks/SUBNET'."]
    pub fn subnet(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subnet", self.base))
    }
}
#[derive(Serialize, Default)]
struct ManagedKafkaClusterGcpConfigElAccessConfigElDynamic {
    network_configs:
        Option<DynamicBlock<ManagedKafkaClusterGcpConfigElAccessConfigElNetworkConfigsEl>>,
}
#[derive(Serialize)]
pub struct ManagedKafkaClusterGcpConfigElAccessConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    network_configs: Option<Vec<ManagedKafkaClusterGcpConfigElAccessConfigElNetworkConfigsEl>>,
    dynamic: ManagedKafkaClusterGcpConfigElAccessConfigElDynamic,
}
impl ManagedKafkaClusterGcpConfigElAccessConfigEl {
    #[doc = "Set the field `network_configs`.\n"]
    pub fn set_network_configs(
        mut self,
        v: impl Into<BlockAssignable<ManagedKafkaClusterGcpConfigElAccessConfigElNetworkConfigsEl>>,
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
impl ToListMappable for ManagedKafkaClusterGcpConfigElAccessConfigEl {
    type O = BlockAssignable<ManagedKafkaClusterGcpConfigElAccessConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildManagedKafkaClusterGcpConfigElAccessConfigEl {}
impl BuildManagedKafkaClusterGcpConfigElAccessConfigEl {
    pub fn build(self) -> ManagedKafkaClusterGcpConfigElAccessConfigEl {
        ManagedKafkaClusterGcpConfigElAccessConfigEl {
            network_configs: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ManagedKafkaClusterGcpConfigElAccessConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ManagedKafkaClusterGcpConfigElAccessConfigElRef {
    fn new(shared: StackShared, base: String) -> ManagedKafkaClusterGcpConfigElAccessConfigElRef {
        ManagedKafkaClusterGcpConfigElAccessConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ManagedKafkaClusterGcpConfigElAccessConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `network_configs` after provisioning.\n"]
    pub fn network_configs(
        &self,
    ) -> ListRef<ManagedKafkaClusterGcpConfigElAccessConfigElNetworkConfigsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_configs", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ManagedKafkaClusterGcpConfigElDynamic {
    access_config: Option<DynamicBlock<ManagedKafkaClusterGcpConfigElAccessConfigEl>>,
}
#[derive(Serialize)]
pub struct ManagedKafkaClusterGcpConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    access_config: Option<Vec<ManagedKafkaClusterGcpConfigElAccessConfigEl>>,
    dynamic: ManagedKafkaClusterGcpConfigElDynamic,
}
impl ManagedKafkaClusterGcpConfigEl {
    #[doc = "Set the field `kms_key`.\nThe Cloud KMS Key name to use for encryption. The key must be located in the same region as the cluster and cannot be changed. Must be in the format 'projects/PROJECT_ID/locations/LOCATION/keyRings/KEY_RING/cryptoKeys/KEY'."]
    pub fn set_kms_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key = Some(v.into());
        self
    }
    #[doc = "Set the field `access_config`.\n"]
    pub fn set_access_config(
        mut self,
        v: impl Into<BlockAssignable<ManagedKafkaClusterGcpConfigElAccessConfigEl>>,
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
impl ToListMappable for ManagedKafkaClusterGcpConfigEl {
    type O = BlockAssignable<ManagedKafkaClusterGcpConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildManagedKafkaClusterGcpConfigEl {}
impl BuildManagedKafkaClusterGcpConfigEl {
    pub fn build(self) -> ManagedKafkaClusterGcpConfigEl {
        ManagedKafkaClusterGcpConfigEl {
            kms_key: core::default::Default::default(),
            access_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ManagedKafkaClusterGcpConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ManagedKafkaClusterGcpConfigElRef {
    fn new(shared: StackShared, base: String) -> ManagedKafkaClusterGcpConfigElRef {
        ManagedKafkaClusterGcpConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ManagedKafkaClusterGcpConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `kms_key` after provisioning.\nThe Cloud KMS Key name to use for encryption. The key must be located in the same region as the cluster and cannot be changed. Must be in the format 'projects/PROJECT_ID/locations/LOCATION/keyRings/KEY_RING/cryptoKeys/KEY'."]
    pub fn kms_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key", self.base))
    }
    #[doc = "Get a reference to the value of field `access_config` after provisioning.\n"]
    pub fn access_config(&self) -> ListRef<ManagedKafkaClusterGcpConfigElAccessConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.access_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ManagedKafkaClusterRebalanceConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<String>>,
}
impl ManagedKafkaClusterRebalanceConfigEl {
    #[doc = "Set the field `mode`.\nThe rebalance behavior for the cluster. When not specified, defaults to 'NO_REBALANCE'. Possible values: 'MODE_UNSPECIFIED', 'NO_REBALANCE', 'AUTO_REBALANCE_ON_SCALE_UP'."]
    pub fn set_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mode = Some(v.into());
        self
    }
}
impl ToListMappable for ManagedKafkaClusterRebalanceConfigEl {
    type O = BlockAssignable<ManagedKafkaClusterRebalanceConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildManagedKafkaClusterRebalanceConfigEl {}
impl BuildManagedKafkaClusterRebalanceConfigEl {
    pub fn build(self) -> ManagedKafkaClusterRebalanceConfigEl {
        ManagedKafkaClusterRebalanceConfigEl {
            mode: core::default::Default::default(),
        }
    }
}
pub struct ManagedKafkaClusterRebalanceConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ManagedKafkaClusterRebalanceConfigElRef {
    fn new(shared: StackShared, base: String) -> ManagedKafkaClusterRebalanceConfigElRef {
        ManagedKafkaClusterRebalanceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ManagedKafkaClusterRebalanceConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `mode` after provisioning.\nThe rebalance behavior for the cluster. When not specified, defaults to 'NO_REBALANCE'. Possible values: 'MODE_UNSPECIFIED', 'NO_REBALANCE', 'AUTO_REBALANCE_ON_SCALE_UP'."]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
}
#[derive(Serialize)]
pub struct ManagedKafkaClusterTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ManagedKafkaClusterTimeoutsEl {
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
impl ToListMappable for ManagedKafkaClusterTimeoutsEl {
    type O = BlockAssignable<ManagedKafkaClusterTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildManagedKafkaClusterTimeoutsEl {}
impl BuildManagedKafkaClusterTimeoutsEl {
    pub fn build(self) -> ManagedKafkaClusterTimeoutsEl {
        ManagedKafkaClusterTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ManagedKafkaClusterTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ManagedKafkaClusterTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ManagedKafkaClusterTimeoutsElRef {
        ManagedKafkaClusterTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ManagedKafkaClusterTimeoutsElRef {
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
#[derive(Serialize)]
pub struct ManagedKafkaClusterTlsConfigElTrustConfigElCasConfigsEl {
    ca_pool: PrimField<String>,
}
impl ManagedKafkaClusterTlsConfigElTrustConfigElCasConfigsEl {}
impl ToListMappable for ManagedKafkaClusterTlsConfigElTrustConfigElCasConfigsEl {
    type O = BlockAssignable<ManagedKafkaClusterTlsConfigElTrustConfigElCasConfigsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildManagedKafkaClusterTlsConfigElTrustConfigElCasConfigsEl {
    #[doc = "The name of the CA pool to pull CA certificates from. The CA pool does not need to be in the same project or location as the Kafka cluster. Must be in the format 'projects/PROJECT_ID/locations/LOCATION/caPools/CA_POOL_ID."]
    pub ca_pool: PrimField<String>,
}
impl BuildManagedKafkaClusterTlsConfigElTrustConfigElCasConfigsEl {
    pub fn build(self) -> ManagedKafkaClusterTlsConfigElTrustConfigElCasConfigsEl {
        ManagedKafkaClusterTlsConfigElTrustConfigElCasConfigsEl {
            ca_pool: self.ca_pool,
        }
    }
}
pub struct ManagedKafkaClusterTlsConfigElTrustConfigElCasConfigsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ManagedKafkaClusterTlsConfigElTrustConfigElCasConfigsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ManagedKafkaClusterTlsConfigElTrustConfigElCasConfigsElRef {
        ManagedKafkaClusterTlsConfigElTrustConfigElCasConfigsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ManagedKafkaClusterTlsConfigElTrustConfigElCasConfigsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ca_pool` after provisioning.\nThe name of the CA pool to pull CA certificates from. The CA pool does not need to be in the same project or location as the Kafka cluster. Must be in the format 'projects/PROJECT_ID/locations/LOCATION/caPools/CA_POOL_ID."]
    pub fn ca_pool(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ca_pool", self.base))
    }
}
#[derive(Serialize, Default)]
struct ManagedKafkaClusterTlsConfigElTrustConfigElDynamic {
    cas_configs: Option<DynamicBlock<ManagedKafkaClusterTlsConfigElTrustConfigElCasConfigsEl>>,
}
#[derive(Serialize)]
pub struct ManagedKafkaClusterTlsConfigElTrustConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cas_configs: Option<Vec<ManagedKafkaClusterTlsConfigElTrustConfigElCasConfigsEl>>,
    dynamic: ManagedKafkaClusterTlsConfigElTrustConfigElDynamic,
}
impl ManagedKafkaClusterTlsConfigElTrustConfigEl {
    #[doc = "Set the field `cas_configs`.\n"]
    pub fn set_cas_configs(
        mut self,
        v: impl Into<BlockAssignable<ManagedKafkaClusterTlsConfigElTrustConfigElCasConfigsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cas_configs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cas_configs = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ManagedKafkaClusterTlsConfigElTrustConfigEl {
    type O = BlockAssignable<ManagedKafkaClusterTlsConfigElTrustConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildManagedKafkaClusterTlsConfigElTrustConfigEl {}
impl BuildManagedKafkaClusterTlsConfigElTrustConfigEl {
    pub fn build(self) -> ManagedKafkaClusterTlsConfigElTrustConfigEl {
        ManagedKafkaClusterTlsConfigElTrustConfigEl {
            cas_configs: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ManagedKafkaClusterTlsConfigElTrustConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ManagedKafkaClusterTlsConfigElTrustConfigElRef {
    fn new(shared: StackShared, base: String) -> ManagedKafkaClusterTlsConfigElTrustConfigElRef {
        ManagedKafkaClusterTlsConfigElTrustConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ManagedKafkaClusterTlsConfigElTrustConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cas_configs` after provisioning.\n"]
    pub fn cas_configs(
        &self,
    ) -> ListRef<ManagedKafkaClusterTlsConfigElTrustConfigElCasConfigsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cas_configs", self.base))
    }
}
#[derive(Serialize, Default)]
struct ManagedKafkaClusterTlsConfigElDynamic {
    trust_config: Option<DynamicBlock<ManagedKafkaClusterTlsConfigElTrustConfigEl>>,
}
#[derive(Serialize)]
pub struct ManagedKafkaClusterTlsConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ssl_principal_mapping_rules: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    trust_config: Option<Vec<ManagedKafkaClusterTlsConfigElTrustConfigEl>>,
    dynamic: ManagedKafkaClusterTlsConfigElDynamic,
}
impl ManagedKafkaClusterTlsConfigEl {
    #[doc = "Set the field `ssl_principal_mapping_rules`.\nThe rules for mapping mTLS certificate Distinguished Names (DNs) to shortened principal names for Kafka ACLs. This field corresponds exactly to the ssl.principal.mapping.rules broker config and matches the format and syntax defined in the Apache Kafka documentation. Setting or modifying this field will trigger a rolling restart of the Kafka brokers to apply the change. An empty string means that the default Kafka behavior is used. Example: 'RULE:^CN=(.?),OU=ServiceUsers.$/$1@example.com/,DEFAULT'"]
    pub fn set_ssl_principal_mapping_rules(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ssl_principal_mapping_rules = Some(v.into());
        self
    }
    #[doc = "Set the field `trust_config`.\n"]
    pub fn set_trust_config(
        mut self,
        v: impl Into<BlockAssignable<ManagedKafkaClusterTlsConfigElTrustConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.trust_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.trust_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ManagedKafkaClusterTlsConfigEl {
    type O = BlockAssignable<ManagedKafkaClusterTlsConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildManagedKafkaClusterTlsConfigEl {}
impl BuildManagedKafkaClusterTlsConfigEl {
    pub fn build(self) -> ManagedKafkaClusterTlsConfigEl {
        ManagedKafkaClusterTlsConfigEl {
            ssl_principal_mapping_rules: core::default::Default::default(),
            trust_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ManagedKafkaClusterTlsConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ManagedKafkaClusterTlsConfigElRef {
    fn new(shared: StackShared, base: String) -> ManagedKafkaClusterTlsConfigElRef {
        ManagedKafkaClusterTlsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ManagedKafkaClusterTlsConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ssl_principal_mapping_rules` after provisioning.\nThe rules for mapping mTLS certificate Distinguished Names (DNs) to shortened principal names for Kafka ACLs. This field corresponds exactly to the ssl.principal.mapping.rules broker config and matches the format and syntax defined in the Apache Kafka documentation. Setting or modifying this field will trigger a rolling restart of the Kafka brokers to apply the change. An empty string means that the default Kafka behavior is used. Example: 'RULE:^CN=(.?),OU=ServiceUsers.$/$1@example.com/,DEFAULT'"]
    pub fn ssl_principal_mapping_rules(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ssl_principal_mapping_rules", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `trust_config` after provisioning.\n"]
    pub fn trust_config(&self) -> ListRef<ManagedKafkaClusterTlsConfigElTrustConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.trust_config", self.base))
    }
}
#[derive(Serialize, Default)]
struct ManagedKafkaClusterDynamic {
    broker_capacity_config: Option<DynamicBlock<ManagedKafkaClusterBrokerCapacityConfigEl>>,
    capacity_config: Option<DynamicBlock<ManagedKafkaClusterCapacityConfigEl>>,
    gcp_config: Option<DynamicBlock<ManagedKafkaClusterGcpConfigEl>>,
    rebalance_config: Option<DynamicBlock<ManagedKafkaClusterRebalanceConfigEl>>,
    tls_config: Option<DynamicBlock<ManagedKafkaClusterTlsConfigEl>>,
}
