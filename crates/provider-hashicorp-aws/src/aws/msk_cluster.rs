use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct MskClusterData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    cluster_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enhanced_monitoring: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    kafka_version: PrimField<String>,
    number_of_broker_nodes: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    broker_node_group_info: Option<Vec<MskClusterBrokerNodeGroupInfoEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_authentication: Option<Vec<MskClusterClientAuthenticationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    configuration_info: Option<Vec<MskClusterConfigurationInfoEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_info: Option<Vec<MskClusterEncryptionInfoEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logging_info: Option<Vec<MskClusterLoggingInfoEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    open_monitoring: Option<Vec<MskClusterOpenMonitoringEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rebalancing: Option<Vec<MskClusterRebalancingEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<MskClusterTimeoutsEl>,
    dynamic: MskClusterDynamic,
}

struct MskCluster_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<MskClusterData>,
}

#[derive(Clone)]
pub struct MskCluster(Rc<MskCluster_>);

impl MskCluster {
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

    #[doc = "Set the field `enhanced_monitoring`.\n"]
    pub fn set_enhanced_monitoring(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().enhanced_monitoring = Some(v.into());
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

    #[doc = "Set the field `storage_mode`.\n"]
    pub fn set_storage_mode(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().storage_mode = Some(v.into());
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

    #[doc = "Set the field `broker_node_group_info`.\n"]
    pub fn set_broker_node_group_info(
        self,
        v: impl Into<BlockAssignable<MskClusterBrokerNodeGroupInfoEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().broker_node_group_info = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.broker_node_group_info = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `client_authentication`.\n"]
    pub fn set_client_authentication(
        self,
        v: impl Into<BlockAssignable<MskClusterClientAuthenticationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().client_authentication = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.client_authentication = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `configuration_info`.\n"]
    pub fn set_configuration_info(
        self,
        v: impl Into<BlockAssignable<MskClusterConfigurationInfoEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().configuration_info = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.configuration_info = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `encryption_info`.\n"]
    pub fn set_encryption_info(
        self,
        v: impl Into<BlockAssignable<MskClusterEncryptionInfoEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().encryption_info = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.encryption_info = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `logging_info`.\n"]
    pub fn set_logging_info(self, v: impl Into<BlockAssignable<MskClusterLoggingInfoEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().logging_info = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.logging_info = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `open_monitoring`.\n"]
    pub fn set_open_monitoring(
        self,
        v: impl Into<BlockAssignable<MskClusterOpenMonitoringEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().open_monitoring = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.open_monitoring = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `rebalancing`.\n"]
    pub fn set_rebalancing(self, v: impl Into<BlockAssignable<MskClusterRebalancingEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rebalancing = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rebalancing = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<MskClusterTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers` after provisioning.\n"]
    pub fn bootstrap_brokers(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bootstrap_brokers", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_public_sasl_iam` after provisioning.\n"]
    pub fn bootstrap_brokers_public_sasl_iam(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bootstrap_brokers_public_sasl_iam", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_public_sasl_scram` after provisioning.\n"]
    pub fn bootstrap_brokers_public_sasl_scram(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bootstrap_brokers_public_sasl_scram", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_public_tls` after provisioning.\n"]
    pub fn bootstrap_brokers_public_tls(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bootstrap_brokers_public_tls", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_sasl_iam` after provisioning.\n"]
    pub fn bootstrap_brokers_sasl_iam(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bootstrap_brokers_sasl_iam", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_sasl_scram` after provisioning.\n"]
    pub fn bootstrap_brokers_sasl_scram(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bootstrap_brokers_sasl_scram", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_tls` after provisioning.\n"]
    pub fn bootstrap_brokers_tls(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bootstrap_brokers_tls", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_vpc_connectivity_sasl_iam` after provisioning.\n"]
    pub fn bootstrap_brokers_vpc_connectivity_sasl_iam(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.bootstrap_brokers_vpc_connectivity_sasl_iam",
                self.extract_ref()
            ),
        )
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_vpc_connectivity_sasl_scram` after provisioning.\n"]
    pub fn bootstrap_brokers_vpc_connectivity_sasl_scram(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.bootstrap_brokers_vpc_connectivity_sasl_scram",
                self.extract_ref()
            ),
        )
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_vpc_connectivity_tls` after provisioning.\n"]
    pub fn bootstrap_brokers_vpc_connectivity_tls(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.bootstrap_brokers_vpc_connectivity_tls",
                self.extract_ref()
            ),
        )
    }

    #[doc = "Get a reference to the value of field `cluster_name` after provisioning.\n"]
    pub fn cluster_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `cluster_uuid` after provisioning.\n"]
    pub fn cluster_uuid(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster_uuid", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `current_version` after provisioning.\n"]
    pub fn current_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.current_version", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `enhanced_monitoring` after provisioning.\n"]
    pub fn enhanced_monitoring(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enhanced_monitoring", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kafka_version` after provisioning.\n"]
    pub fn kafka_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kafka_version", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `number_of_broker_nodes` after provisioning.\n"]
    pub fn number_of_broker_nodes(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.number_of_broker_nodes", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `storage_mode` after provisioning.\n"]
    pub fn storage_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.storage_mode", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `zookeeper_connect_string` after provisioning.\n"]
    pub fn zookeeper_connect_string(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.zookeeper_connect_string", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `zookeeper_connect_string_tls` after provisioning.\n"]
    pub fn zookeeper_connect_string_tls(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.zookeeper_connect_string_tls", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `broker_node_group_info` after provisioning.\n"]
    pub fn broker_node_group_info(&self) -> ListRef<MskClusterBrokerNodeGroupInfoElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.broker_node_group_info", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `client_authentication` after provisioning.\n"]
    pub fn client_authentication(&self) -> ListRef<MskClusterClientAuthenticationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.client_authentication", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `configuration_info` after provisioning.\n"]
    pub fn configuration_info(&self) -> ListRef<MskClusterConfigurationInfoElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.configuration_info", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `encryption_info` after provisioning.\n"]
    pub fn encryption_info(&self) -> ListRef<MskClusterEncryptionInfoElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.encryption_info", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `logging_info` after provisioning.\n"]
    pub fn logging_info(&self) -> ListRef<MskClusterLoggingInfoElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.logging_info", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `open_monitoring` after provisioning.\n"]
    pub fn open_monitoring(&self) -> ListRef<MskClusterOpenMonitoringElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.open_monitoring", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `rebalancing` after provisioning.\n"]
    pub fn rebalancing(&self) -> ListRef<MskClusterRebalancingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rebalancing", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MskClusterTimeoutsElRef {
        MskClusterTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for MskCluster {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for MskCluster {}

impl ToListMappable for MskCluster {
    type O = ListRef<MskClusterRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for MskCluster_ {
    fn extract_resource_type(&self) -> String {
        "aws_msk_cluster".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildMskCluster {
    pub tf_id: String,
    #[doc = ""]
    pub cluster_name: PrimField<String>,
    #[doc = ""]
    pub kafka_version: PrimField<String>,
    #[doc = ""]
    pub number_of_broker_nodes: PrimField<f64>,
}

impl BuildMskCluster {
    pub fn build(self, stack: &mut Stack) -> MskCluster {
        let out = MskCluster(Rc::new(MskCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(MskClusterData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                cluster_name: self.cluster_name,
                enhanced_monitoring: core::default::Default::default(),
                id: core::default::Default::default(),
                kafka_version: self.kafka_version,
                number_of_broker_nodes: self.number_of_broker_nodes,
                region: core::default::Default::default(),
                storage_mode: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                broker_node_group_info: core::default::Default::default(),
                client_authentication: core::default::Default::default(),
                configuration_info: core::default::Default::default(),
                encryption_info: core::default::Default::default(),
                logging_info: core::default::Default::default(),
                open_monitoring: core::default::Default::default(),
                rebalancing: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct MskClusterRef {
    shared: StackShared,
    base: String,
}

impl Ref for MskClusterRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl MskClusterRef {
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

    #[doc = "Get a reference to the value of field `bootstrap_brokers` after provisioning.\n"]
    pub fn bootstrap_brokers(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bootstrap_brokers", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_public_sasl_iam` after provisioning.\n"]
    pub fn bootstrap_brokers_public_sasl_iam(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bootstrap_brokers_public_sasl_iam", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_public_sasl_scram` after provisioning.\n"]
    pub fn bootstrap_brokers_public_sasl_scram(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bootstrap_brokers_public_sasl_scram", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_public_tls` after provisioning.\n"]
    pub fn bootstrap_brokers_public_tls(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bootstrap_brokers_public_tls", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_sasl_iam` after provisioning.\n"]
    pub fn bootstrap_brokers_sasl_iam(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bootstrap_brokers_sasl_iam", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_sasl_scram` after provisioning.\n"]
    pub fn bootstrap_brokers_sasl_scram(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bootstrap_brokers_sasl_scram", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_tls` after provisioning.\n"]
    pub fn bootstrap_brokers_tls(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bootstrap_brokers_tls", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_vpc_connectivity_sasl_iam` after provisioning.\n"]
    pub fn bootstrap_brokers_vpc_connectivity_sasl_iam(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.bootstrap_brokers_vpc_connectivity_sasl_iam",
                self.extract_ref()
            ),
        )
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_vpc_connectivity_sasl_scram` after provisioning.\n"]
    pub fn bootstrap_brokers_vpc_connectivity_sasl_scram(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.bootstrap_brokers_vpc_connectivity_sasl_scram",
                self.extract_ref()
            ),
        )
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_vpc_connectivity_tls` after provisioning.\n"]
    pub fn bootstrap_brokers_vpc_connectivity_tls(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.bootstrap_brokers_vpc_connectivity_tls",
                self.extract_ref()
            ),
        )
    }

    #[doc = "Get a reference to the value of field `cluster_name` after provisioning.\n"]
    pub fn cluster_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `cluster_uuid` after provisioning.\n"]
    pub fn cluster_uuid(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster_uuid", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `current_version` after provisioning.\n"]
    pub fn current_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.current_version", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `enhanced_monitoring` after provisioning.\n"]
    pub fn enhanced_monitoring(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enhanced_monitoring", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kafka_version` after provisioning.\n"]
    pub fn kafka_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kafka_version", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `number_of_broker_nodes` after provisioning.\n"]
    pub fn number_of_broker_nodes(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.number_of_broker_nodes", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `storage_mode` after provisioning.\n"]
    pub fn storage_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.storage_mode", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `zookeeper_connect_string` after provisioning.\n"]
    pub fn zookeeper_connect_string(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.zookeeper_connect_string", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `zookeeper_connect_string_tls` after provisioning.\n"]
    pub fn zookeeper_connect_string_tls(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.zookeeper_connect_string_tls", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `broker_node_group_info` after provisioning.\n"]
    pub fn broker_node_group_info(&self) -> ListRef<MskClusterBrokerNodeGroupInfoElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.broker_node_group_info", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `client_authentication` after provisioning.\n"]
    pub fn client_authentication(&self) -> ListRef<MskClusterClientAuthenticationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.client_authentication", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `configuration_info` after provisioning.\n"]
    pub fn configuration_info(&self) -> ListRef<MskClusterConfigurationInfoElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.configuration_info", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `encryption_info` after provisioning.\n"]
    pub fn encryption_info(&self) -> ListRef<MskClusterEncryptionInfoElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.encryption_info", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `logging_info` after provisioning.\n"]
    pub fn logging_info(&self) -> ListRef<MskClusterLoggingInfoElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.logging_info", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `open_monitoring` after provisioning.\n"]
    pub fn open_monitoring(&self) -> ListRef<MskClusterOpenMonitoringElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.open_monitoring", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `rebalancing` after provisioning.\n"]
    pub fn rebalancing(&self) -> ListRef<MskClusterRebalancingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rebalancing", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MskClusterTimeoutsElRef {
        MskClusterTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct MskClusterBrokerNodeGroupInfoElConnectivityInfoElPublicAccessEl {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}

impl MskClusterBrokerNodeGroupInfoElConnectivityInfoElPublicAccessEl {
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}

impl ToListMappable for MskClusterBrokerNodeGroupInfoElConnectivityInfoElPublicAccessEl {
    type O = BlockAssignable<MskClusterBrokerNodeGroupInfoElConnectivityInfoElPublicAccessEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskClusterBrokerNodeGroupInfoElConnectivityInfoElPublicAccessEl {}

impl BuildMskClusterBrokerNodeGroupInfoElConnectivityInfoElPublicAccessEl {
    pub fn build(self) -> MskClusterBrokerNodeGroupInfoElConnectivityInfoElPublicAccessEl {
        MskClusterBrokerNodeGroupInfoElConnectivityInfoElPublicAccessEl {
            type_: core::default::Default::default(),
        }
    }
}

pub struct MskClusterBrokerNodeGroupInfoElConnectivityInfoElPublicAccessElRef {
    shared: StackShared,
    base: String,
}

impl Ref for MskClusterBrokerNodeGroupInfoElConnectivityInfoElPublicAccessElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MskClusterBrokerNodeGroupInfoElConnectivityInfoElPublicAccessElRef {
        MskClusterBrokerNodeGroupInfoElConnectivityInfoElPublicAccessElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskClusterBrokerNodeGroupInfoElConnectivityInfoElPublicAccessElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct MskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityElClientAuthenticationElSaslEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    iam: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scram: Option<PrimField<bool>>,
}

impl
    MskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityElClientAuthenticationElSaslEl
{
    #[doc = "Set the field `iam`.\n"]
    pub fn set_iam(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.iam = Some(v.into());
        self
    }

    #[doc = "Set the field `scram`.\n"]
    pub fn set_scram(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.scram = Some(v.into());
        self
    }
}

impl ToListMappable for MskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityElClientAuthenticationElSaslEl {
    type O =
        BlockAssignable<
            MskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityElClientAuthenticationElSaslEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityElClientAuthenticationElSaslEl
{}

impl BuildMskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityElClientAuthenticationElSaslEl {
    pub fn build(
        self,
    ) -> MskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityElClientAuthenticationElSaslEl {
        MskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityElClientAuthenticationElSaslEl {
            iam: core::default::Default::default(),
            scram: core::default::Default::default(),
        }
    }
}

pub struct MskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityElClientAuthenticationElSaslElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for MskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityElClientAuthenticationElSaslElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityElClientAuthenticationElSaslElRef {
        MskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityElClientAuthenticationElSaslElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityElClientAuthenticationElSaslElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `iam` after provisioning.\n"]
    pub fn iam(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.iam", self.base))
    }

    #[doc = "Get a reference to the value of field `scram` after provisioning.\n"]
    pub fn scram(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.scram", self.base))
    }
}

#[derive(Serialize, Default)]
struct MskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityElClientAuthenticationElDynamic {
    sasl: Option<
        DynamicBlock<MskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityElClientAuthenticationElSaslEl>,
    >,
}

#[derive(Serialize)]
pub struct MskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityElClientAuthenticationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    tls: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sasl: Option<Vec<MskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityElClientAuthenticationElSaslEl>>,
    dynamic: MskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityElClientAuthenticationElDynamic,
}

impl MskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityElClientAuthenticationEl {
    #[doc = "Set the field `tls`.\n"]
    pub fn set_tls(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.tls = Some(v.into());
        self
    }

    #[doc = "Set the field `sasl`.\n"]
    pub fn set_sasl(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            MskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityElClientAuthenticationElSaslEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.sasl = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.sasl = Some(d);
            }
        }
        self
    }
}

impl ToListMappable
    for MskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityElClientAuthenticationEl
{
    type O = BlockAssignable<
        MskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityElClientAuthenticationEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityElClientAuthenticationEl
{}

impl BuildMskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityElClientAuthenticationEl {
    pub fn build(
        self,
    ) -> MskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityElClientAuthenticationEl
    {
        MskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityElClientAuthenticationEl {
            tls: core::default::Default::default(),
            sasl: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct MskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityElClientAuthenticationElRef
{
    shared: StackShared,
    base: String,
}

impl Ref
    for MskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityElClientAuthenticationElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> MskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityElClientAuthenticationElRef
    {
        MskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityElClientAuthenticationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityElClientAuthenticationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `tls` after provisioning.\n"]
    pub fn tls(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.tls", self.base))
    }

    #[doc = "Get a reference to the value of field `sasl` after provisioning.\n"]
    pub fn sasl(
        &self,
    ) -> ListRef<MskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityElClientAuthenticationElSaslElRef>{
        ListRef::new(self.shared().clone(), format!("{}.sasl", self.base))
    }
}

#[derive(Serialize, Default)]
struct MskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityElDynamic {
    client_authentication: Option<
        DynamicBlock<MskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityElClientAuthenticationEl>,
    >,
}

#[derive(Serialize)]
pub struct MskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    client_authentication: Option<
        Vec<MskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityElClientAuthenticationEl>,
    >,
    dynamic: MskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityElDynamic,
}

impl MskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityEl {
    #[doc = "Set the field `client_authentication`.\n"]
    pub fn set_client_authentication(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            MskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityElClientAuthenticationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.client_authentication = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.client_authentication = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for MskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityEl {
    type O = BlockAssignable<MskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityEl {}

impl BuildMskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityEl {
    pub fn build(self) -> MskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityEl {
        MskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityEl {
            client_authentication: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct MskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityElRef {
    shared: StackShared,
    base: String,
}

impl Ref for MskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityElRef {
        MskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `client_authentication` after provisioning.\n"]
    pub fn client_authentication(
        &self,
    ) -> ListRef<
        MskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityElClientAuthenticationElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.client_authentication", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct MskClusterBrokerNodeGroupInfoElConnectivityInfoElDynamic {
    public_access:
        Option<DynamicBlock<MskClusterBrokerNodeGroupInfoElConnectivityInfoElPublicAccessEl>>,
    vpc_connectivity:
        Option<DynamicBlock<MskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityEl>>,
}

#[derive(Serialize)]
pub struct MskClusterBrokerNodeGroupInfoElConnectivityInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    public_access: Option<Vec<MskClusterBrokerNodeGroupInfoElConnectivityInfoElPublicAccessEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_connectivity:
        Option<Vec<MskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityEl>>,
    dynamic: MskClusterBrokerNodeGroupInfoElConnectivityInfoElDynamic,
}

impl MskClusterBrokerNodeGroupInfoElConnectivityInfoEl {
    #[doc = "Set the field `public_access`.\n"]
    pub fn set_public_access(
        mut self,
        v: impl Into<BlockAssignable<MskClusterBrokerNodeGroupInfoElConnectivityInfoElPublicAccessEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.public_access = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.public_access = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `vpc_connectivity`.\n"]
    pub fn set_vpc_connectivity(
        mut self,
        v: impl Into<
            BlockAssignable<MskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.vpc_connectivity = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.vpc_connectivity = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for MskClusterBrokerNodeGroupInfoElConnectivityInfoEl {
    type O = BlockAssignable<MskClusterBrokerNodeGroupInfoElConnectivityInfoEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskClusterBrokerNodeGroupInfoElConnectivityInfoEl {}

impl BuildMskClusterBrokerNodeGroupInfoElConnectivityInfoEl {
    pub fn build(self) -> MskClusterBrokerNodeGroupInfoElConnectivityInfoEl {
        MskClusterBrokerNodeGroupInfoElConnectivityInfoEl {
            public_access: core::default::Default::default(),
            vpc_connectivity: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct MskClusterBrokerNodeGroupInfoElConnectivityInfoElRef {
    shared: StackShared,
    base: String,
}

impl Ref for MskClusterBrokerNodeGroupInfoElConnectivityInfoElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MskClusterBrokerNodeGroupInfoElConnectivityInfoElRef {
        MskClusterBrokerNodeGroupInfoElConnectivityInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskClusterBrokerNodeGroupInfoElConnectivityInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `public_access` after provisioning.\n"]
    pub fn public_access(
        &self,
    ) -> ListRef<MskClusterBrokerNodeGroupInfoElConnectivityInfoElPublicAccessElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.public_access", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `vpc_connectivity` after provisioning.\n"]
    pub fn vpc_connectivity(
        &self,
    ) -> ListRef<MskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vpc_connectivity", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct MskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoElProvisionedThroughputEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volume_throughput: Option<PrimField<f64>>,
}

impl MskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoElProvisionedThroughputEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `volume_throughput`.\n"]
    pub fn set_volume_throughput(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.volume_throughput = Some(v.into());
        self
    }
}

impl ToListMappable
    for MskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoElProvisionedThroughputEl
{
    type O = BlockAssignable<
        MskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoElProvisionedThroughputEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoElProvisionedThroughputEl
{}

impl BuildMskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoElProvisionedThroughputEl {
    pub fn build(
        self,
    ) -> MskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoElProvisionedThroughputEl {
        MskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoElProvisionedThroughputEl {
            enabled: core::default::Default::default(),
            volume_throughput: core::default::Default::default(),
        }
    }
}

pub struct MskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoElProvisionedThroughputElRef {
    shared: StackShared,
    base: String,
}

impl Ref
    for MskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoElProvisionedThroughputElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> MskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoElProvisionedThroughputElRef
    {
        MskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoElProvisionedThroughputElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoElProvisionedThroughputElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `volume_throughput` after provisioning.\n"]
    pub fn volume_throughput(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.volume_throughput", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct MskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoElDynamic {
    provisioned_throughput: Option<
        DynamicBlock<
            MskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoElProvisionedThroughputEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct MskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    volume_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provisioned_throughput: Option<
        Vec<MskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoElProvisionedThroughputEl>,
    >,
    dynamic: MskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoElDynamic,
}

impl MskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoEl {
    #[doc = "Set the field `volume_size`.\n"]
    pub fn set_volume_size(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.volume_size = Some(v.into());
        self
    }

    #[doc = "Set the field `provisioned_throughput`.\n"]
    pub fn set_provisioned_throughput(
        mut self,
        v: impl Into<
            BlockAssignable<
                MskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoElProvisionedThroughputEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.provisioned_throughput = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.provisioned_throughput = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for MskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoEl {
    type O = BlockAssignable<MskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoEl {}

impl BuildMskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoEl {
    pub fn build(self) -> MskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoEl {
        MskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoEl {
            volume_size: core::default::Default::default(),
            provisioned_throughput: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct MskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoElRef {
    shared: StackShared,
    base: String,
}

impl Ref for MskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoElRef {
        MskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `volume_size` after provisioning.\n"]
    pub fn volume_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.volume_size", self.base))
    }

    #[doc = "Get a reference to the value of field `provisioned_throughput` after provisioning.\n"]
    pub fn provisioned_throughput(
        &self,
    ) -> ListRef<
        MskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoElProvisionedThroughputElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.provisioned_throughput", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct MskClusterBrokerNodeGroupInfoElStorageInfoElDynamic {
    ebs_storage_info:
        Option<DynamicBlock<MskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoEl>>,
}

#[derive(Serialize)]
pub struct MskClusterBrokerNodeGroupInfoElStorageInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ebs_storage_info: Option<Vec<MskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoEl>>,
    dynamic: MskClusterBrokerNodeGroupInfoElStorageInfoElDynamic,
}

impl MskClusterBrokerNodeGroupInfoElStorageInfoEl {
    #[doc = "Set the field `ebs_storage_info`.\n"]
    pub fn set_ebs_storage_info(
        mut self,
        v: impl Into<BlockAssignable<MskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ebs_storage_info = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ebs_storage_info = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for MskClusterBrokerNodeGroupInfoElStorageInfoEl {
    type O = BlockAssignable<MskClusterBrokerNodeGroupInfoElStorageInfoEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskClusterBrokerNodeGroupInfoElStorageInfoEl {}

impl BuildMskClusterBrokerNodeGroupInfoElStorageInfoEl {
    pub fn build(self) -> MskClusterBrokerNodeGroupInfoElStorageInfoEl {
        MskClusterBrokerNodeGroupInfoElStorageInfoEl {
            ebs_storage_info: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct MskClusterBrokerNodeGroupInfoElStorageInfoElRef {
    shared: StackShared,
    base: String,
}

impl Ref for MskClusterBrokerNodeGroupInfoElStorageInfoElRef {
    fn new(shared: StackShared, base: String) -> MskClusterBrokerNodeGroupInfoElStorageInfoElRef {
        MskClusterBrokerNodeGroupInfoElStorageInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskClusterBrokerNodeGroupInfoElStorageInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ebs_storage_info` after provisioning.\n"]
    pub fn ebs_storage_info(
        &self,
    ) -> ListRef<MskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ebs_storage_info", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct MskClusterBrokerNodeGroupInfoElDynamic {
    connectivity_info: Option<DynamicBlock<MskClusterBrokerNodeGroupInfoElConnectivityInfoEl>>,
    storage_info: Option<DynamicBlock<MskClusterBrokerNodeGroupInfoElStorageInfoEl>>,
}

#[derive(Serialize)]
pub struct MskClusterBrokerNodeGroupInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    az_distribution: Option<PrimField<String>>,
    client_subnets: SetField<PrimField<String>>,
    instance_type: PrimField<String>,
    security_groups: SetField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    connectivity_info: Option<Vec<MskClusterBrokerNodeGroupInfoElConnectivityInfoEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_info: Option<Vec<MskClusterBrokerNodeGroupInfoElStorageInfoEl>>,
    dynamic: MskClusterBrokerNodeGroupInfoElDynamic,
}

impl MskClusterBrokerNodeGroupInfoEl {
    #[doc = "Set the field `az_distribution`.\n"]
    pub fn set_az_distribution(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.az_distribution = Some(v.into());
        self
    }

    #[doc = "Set the field `connectivity_info`.\n"]
    pub fn set_connectivity_info(
        mut self,
        v: impl Into<BlockAssignable<MskClusterBrokerNodeGroupInfoElConnectivityInfoEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.connectivity_info = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.connectivity_info = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `storage_info`.\n"]
    pub fn set_storage_info(
        mut self,
        v: impl Into<BlockAssignable<MskClusterBrokerNodeGroupInfoElStorageInfoEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.storage_info = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.storage_info = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for MskClusterBrokerNodeGroupInfoEl {
    type O = BlockAssignable<MskClusterBrokerNodeGroupInfoEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskClusterBrokerNodeGroupInfoEl {
    #[doc = ""]
    pub client_subnets: SetField<PrimField<String>>,
    #[doc = ""]
    pub instance_type: PrimField<String>,
    #[doc = ""]
    pub security_groups: SetField<PrimField<String>>,
}

impl BuildMskClusterBrokerNodeGroupInfoEl {
    pub fn build(self) -> MskClusterBrokerNodeGroupInfoEl {
        MskClusterBrokerNodeGroupInfoEl {
            az_distribution: core::default::Default::default(),
            client_subnets: self.client_subnets,
            instance_type: self.instance_type,
            security_groups: self.security_groups,
            connectivity_info: core::default::Default::default(),
            storage_info: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct MskClusterBrokerNodeGroupInfoElRef {
    shared: StackShared,
    base: String,
}

impl Ref for MskClusterBrokerNodeGroupInfoElRef {
    fn new(shared: StackShared, base: String) -> MskClusterBrokerNodeGroupInfoElRef {
        MskClusterBrokerNodeGroupInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskClusterBrokerNodeGroupInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `az_distribution` after provisioning.\n"]
    pub fn az_distribution(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.az_distribution", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `client_subnets` after provisioning.\n"]
    pub fn client_subnets(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.client_subnets", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_type", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `security_groups` after provisioning.\n"]
    pub fn security_groups(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.security_groups", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `connectivity_info` after provisioning.\n"]
    pub fn connectivity_info(
        &self,
    ) -> ListRef<MskClusterBrokerNodeGroupInfoElConnectivityInfoElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.connectivity_info", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `storage_info` after provisioning.\n"]
    pub fn storage_info(&self) -> ListRef<MskClusterBrokerNodeGroupInfoElStorageInfoElRef> {
        ListRef::new(self.shared().clone(), format!("{}.storage_info", self.base))
    }
}

#[derive(Serialize)]
pub struct MskClusterClientAuthenticationElSaslEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    iam: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scram: Option<PrimField<bool>>,
}

impl MskClusterClientAuthenticationElSaslEl {
    #[doc = "Set the field `iam`.\n"]
    pub fn set_iam(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.iam = Some(v.into());
        self
    }

    #[doc = "Set the field `scram`.\n"]
    pub fn set_scram(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.scram = Some(v.into());
        self
    }
}

impl ToListMappable for MskClusterClientAuthenticationElSaslEl {
    type O = BlockAssignable<MskClusterClientAuthenticationElSaslEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskClusterClientAuthenticationElSaslEl {}

impl BuildMskClusterClientAuthenticationElSaslEl {
    pub fn build(self) -> MskClusterClientAuthenticationElSaslEl {
        MskClusterClientAuthenticationElSaslEl {
            iam: core::default::Default::default(),
            scram: core::default::Default::default(),
        }
    }
}

pub struct MskClusterClientAuthenticationElSaslElRef {
    shared: StackShared,
    base: String,
}

impl Ref for MskClusterClientAuthenticationElSaslElRef {
    fn new(shared: StackShared, base: String) -> MskClusterClientAuthenticationElSaslElRef {
        MskClusterClientAuthenticationElSaslElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskClusterClientAuthenticationElSaslElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `iam` after provisioning.\n"]
    pub fn iam(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.iam", self.base))
    }

    #[doc = "Get a reference to the value of field `scram` after provisioning.\n"]
    pub fn scram(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.scram", self.base))
    }
}

#[derive(Serialize)]
pub struct MskClusterClientAuthenticationElTlsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate_authority_arns: Option<SetField<PrimField<String>>>,
}

impl MskClusterClientAuthenticationElTlsEl {
    #[doc = "Set the field `certificate_authority_arns`.\n"]
    pub fn set_certificate_authority_arns(
        mut self,
        v: impl Into<SetField<PrimField<String>>>,
    ) -> Self {
        self.certificate_authority_arns = Some(v.into());
        self
    }
}

impl ToListMappable for MskClusterClientAuthenticationElTlsEl {
    type O = BlockAssignable<MskClusterClientAuthenticationElTlsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskClusterClientAuthenticationElTlsEl {}

impl BuildMskClusterClientAuthenticationElTlsEl {
    pub fn build(self) -> MskClusterClientAuthenticationElTlsEl {
        MskClusterClientAuthenticationElTlsEl {
            certificate_authority_arns: core::default::Default::default(),
        }
    }
}

pub struct MskClusterClientAuthenticationElTlsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for MskClusterClientAuthenticationElTlsElRef {
    fn new(shared: StackShared, base: String) -> MskClusterClientAuthenticationElTlsElRef {
        MskClusterClientAuthenticationElTlsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskClusterClientAuthenticationElTlsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificate_authority_arns` after provisioning.\n"]
    pub fn certificate_authority_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.certificate_authority_arns", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct MskClusterClientAuthenticationElDynamic {
    sasl: Option<DynamicBlock<MskClusterClientAuthenticationElSaslEl>>,
    tls: Option<DynamicBlock<MskClusterClientAuthenticationElTlsEl>>,
}

#[derive(Serialize)]
pub struct MskClusterClientAuthenticationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    unauthenticated: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sasl: Option<Vec<MskClusterClientAuthenticationElSaslEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tls: Option<Vec<MskClusterClientAuthenticationElTlsEl>>,
    dynamic: MskClusterClientAuthenticationElDynamic,
}

impl MskClusterClientAuthenticationEl {
    #[doc = "Set the field `unauthenticated`.\n"]
    pub fn set_unauthenticated(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.unauthenticated = Some(v.into());
        self
    }

    #[doc = "Set the field `sasl`.\n"]
    pub fn set_sasl(
        mut self,
        v: impl Into<BlockAssignable<MskClusterClientAuthenticationElSaslEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.sasl = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.sasl = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `tls`.\n"]
    pub fn set_tls(
        mut self,
        v: impl Into<BlockAssignable<MskClusterClientAuthenticationElTlsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.tls = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.tls = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for MskClusterClientAuthenticationEl {
    type O = BlockAssignable<MskClusterClientAuthenticationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskClusterClientAuthenticationEl {}

impl BuildMskClusterClientAuthenticationEl {
    pub fn build(self) -> MskClusterClientAuthenticationEl {
        MskClusterClientAuthenticationEl {
            unauthenticated: core::default::Default::default(),
            sasl: core::default::Default::default(),
            tls: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct MskClusterClientAuthenticationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for MskClusterClientAuthenticationElRef {
    fn new(shared: StackShared, base: String) -> MskClusterClientAuthenticationElRef {
        MskClusterClientAuthenticationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskClusterClientAuthenticationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `unauthenticated` after provisioning.\n"]
    pub fn unauthenticated(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.unauthenticated", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `sasl` after provisioning.\n"]
    pub fn sasl(&self) -> ListRef<MskClusterClientAuthenticationElSaslElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sasl", self.base))
    }

    #[doc = "Get a reference to the value of field `tls` after provisioning.\n"]
    pub fn tls(&self) -> ListRef<MskClusterClientAuthenticationElTlsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tls", self.base))
    }
}

#[derive(Serialize)]
pub struct MskClusterConfigurationInfoEl {
    arn: PrimField<String>,
    revision: PrimField<f64>,
}

impl MskClusterConfigurationInfoEl {}

impl ToListMappable for MskClusterConfigurationInfoEl {
    type O = BlockAssignable<MskClusterConfigurationInfoEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskClusterConfigurationInfoEl {
    #[doc = ""]
    pub arn: PrimField<String>,
    #[doc = ""]
    pub revision: PrimField<f64>,
}

impl BuildMskClusterConfigurationInfoEl {
    pub fn build(self) -> MskClusterConfigurationInfoEl {
        MskClusterConfigurationInfoEl {
            arn: self.arn,
            revision: self.revision,
        }
    }
}

pub struct MskClusterConfigurationInfoElRef {
    shared: StackShared,
    base: String,
}

impl Ref for MskClusterConfigurationInfoElRef {
    fn new(shared: StackShared, base: String) -> MskClusterConfigurationInfoElRef {
        MskClusterConfigurationInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskClusterConfigurationInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `revision` after provisioning.\n"]
    pub fn revision(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.revision", self.base))
    }
}

#[derive(Serialize)]
pub struct MskClusterEncryptionInfoElEncryptionInTransitEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    client_broker: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    in_cluster: Option<PrimField<bool>>,
}

impl MskClusterEncryptionInfoElEncryptionInTransitEl {
    #[doc = "Set the field `client_broker`.\n"]
    pub fn set_client_broker(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_broker = Some(v.into());
        self
    }

    #[doc = "Set the field `in_cluster`.\n"]
    pub fn set_in_cluster(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.in_cluster = Some(v.into());
        self
    }
}

impl ToListMappable for MskClusterEncryptionInfoElEncryptionInTransitEl {
    type O = BlockAssignable<MskClusterEncryptionInfoElEncryptionInTransitEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskClusterEncryptionInfoElEncryptionInTransitEl {}

impl BuildMskClusterEncryptionInfoElEncryptionInTransitEl {
    pub fn build(self) -> MskClusterEncryptionInfoElEncryptionInTransitEl {
        MskClusterEncryptionInfoElEncryptionInTransitEl {
            client_broker: core::default::Default::default(),
            in_cluster: core::default::Default::default(),
        }
    }
}

pub struct MskClusterEncryptionInfoElEncryptionInTransitElRef {
    shared: StackShared,
    base: String,
}

impl Ref for MskClusterEncryptionInfoElEncryptionInTransitElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MskClusterEncryptionInfoElEncryptionInTransitElRef {
        MskClusterEncryptionInfoElEncryptionInTransitElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskClusterEncryptionInfoElEncryptionInTransitElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `client_broker` after provisioning.\n"]
    pub fn client_broker(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_broker", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `in_cluster` after provisioning.\n"]
    pub fn in_cluster(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.in_cluster", self.base))
    }
}

#[derive(Serialize, Default)]
struct MskClusterEncryptionInfoElDynamic {
    encryption_in_transit: Option<DynamicBlock<MskClusterEncryptionInfoElEncryptionInTransitEl>>,
}

#[derive(Serialize)]
pub struct MskClusterEncryptionInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_at_rest_kms_key_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_in_transit: Option<Vec<MskClusterEncryptionInfoElEncryptionInTransitEl>>,
    dynamic: MskClusterEncryptionInfoElDynamic,
}

impl MskClusterEncryptionInfoEl {
    #[doc = "Set the field `encryption_at_rest_kms_key_arn`.\n"]
    pub fn set_encryption_at_rest_kms_key_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.encryption_at_rest_kms_key_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `encryption_in_transit`.\n"]
    pub fn set_encryption_in_transit(
        mut self,
        v: impl Into<BlockAssignable<MskClusterEncryptionInfoElEncryptionInTransitEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.encryption_in_transit = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.encryption_in_transit = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for MskClusterEncryptionInfoEl {
    type O = BlockAssignable<MskClusterEncryptionInfoEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskClusterEncryptionInfoEl {}

impl BuildMskClusterEncryptionInfoEl {
    pub fn build(self) -> MskClusterEncryptionInfoEl {
        MskClusterEncryptionInfoEl {
            encryption_at_rest_kms_key_arn: core::default::Default::default(),
            encryption_in_transit: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct MskClusterEncryptionInfoElRef {
    shared: StackShared,
    base: String,
}

impl Ref for MskClusterEncryptionInfoElRef {
    fn new(shared: StackShared, base: String) -> MskClusterEncryptionInfoElRef {
        MskClusterEncryptionInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskClusterEncryptionInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `encryption_at_rest_kms_key_arn` after provisioning.\n"]
    pub fn encryption_at_rest_kms_key_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.encryption_at_rest_kms_key_arn", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `encryption_in_transit` after provisioning.\n"]
    pub fn encryption_in_transit(
        &self,
    ) -> ListRef<MskClusterEncryptionInfoElEncryptionInTransitElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.encryption_in_transit", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct MskClusterLoggingInfoElBrokerLogsElCloudwatchLogsEl {
    enabled: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_group: Option<PrimField<String>>,
}

impl MskClusterLoggingInfoElBrokerLogsElCloudwatchLogsEl {
    #[doc = "Set the field `log_group`.\n"]
    pub fn set_log_group(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.log_group = Some(v.into());
        self
    }
}

impl ToListMappable for MskClusterLoggingInfoElBrokerLogsElCloudwatchLogsEl {
    type O = BlockAssignable<MskClusterLoggingInfoElBrokerLogsElCloudwatchLogsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskClusterLoggingInfoElBrokerLogsElCloudwatchLogsEl {
    #[doc = ""]
    pub enabled: PrimField<bool>,
}

impl BuildMskClusterLoggingInfoElBrokerLogsElCloudwatchLogsEl {
    pub fn build(self) -> MskClusterLoggingInfoElBrokerLogsElCloudwatchLogsEl {
        MskClusterLoggingInfoElBrokerLogsElCloudwatchLogsEl {
            enabled: self.enabled,
            log_group: core::default::Default::default(),
        }
    }
}

pub struct MskClusterLoggingInfoElBrokerLogsElCloudwatchLogsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for MskClusterLoggingInfoElBrokerLogsElCloudwatchLogsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MskClusterLoggingInfoElBrokerLogsElCloudwatchLogsElRef {
        MskClusterLoggingInfoElBrokerLogsElCloudwatchLogsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskClusterLoggingInfoElBrokerLogsElCloudwatchLogsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `log_group` after provisioning.\n"]
    pub fn log_group(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_group", self.base))
    }
}

#[derive(Serialize)]
pub struct MskClusterLoggingInfoElBrokerLogsElFirehoseEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    delivery_stream: Option<PrimField<String>>,
    enabled: PrimField<bool>,
}

impl MskClusterLoggingInfoElBrokerLogsElFirehoseEl {
    #[doc = "Set the field `delivery_stream`.\n"]
    pub fn set_delivery_stream(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delivery_stream = Some(v.into());
        self
    }
}

impl ToListMappable for MskClusterLoggingInfoElBrokerLogsElFirehoseEl {
    type O = BlockAssignable<MskClusterLoggingInfoElBrokerLogsElFirehoseEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskClusterLoggingInfoElBrokerLogsElFirehoseEl {
    #[doc = ""]
    pub enabled: PrimField<bool>,
}

impl BuildMskClusterLoggingInfoElBrokerLogsElFirehoseEl {
    pub fn build(self) -> MskClusterLoggingInfoElBrokerLogsElFirehoseEl {
        MskClusterLoggingInfoElBrokerLogsElFirehoseEl {
            delivery_stream: core::default::Default::default(),
            enabled: self.enabled,
        }
    }
}

pub struct MskClusterLoggingInfoElBrokerLogsElFirehoseElRef {
    shared: StackShared,
    base: String,
}

impl Ref for MskClusterLoggingInfoElBrokerLogsElFirehoseElRef {
    fn new(shared: StackShared, base: String) -> MskClusterLoggingInfoElBrokerLogsElFirehoseElRef {
        MskClusterLoggingInfoElBrokerLogsElFirehoseElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskClusterLoggingInfoElBrokerLogsElFirehoseElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `delivery_stream` after provisioning.\n"]
    pub fn delivery_stream(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delivery_stream", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct MskClusterLoggingInfoElBrokerLogsElS3El {
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket: Option<PrimField<String>>,
    enabled: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<PrimField<String>>,
}

impl MskClusterLoggingInfoElBrokerLogsElS3El {
    #[doc = "Set the field `bucket`.\n"]
    pub fn set_bucket(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket = Some(v.into());
        self
    }

    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix = Some(v.into());
        self
    }
}

impl ToListMappable for MskClusterLoggingInfoElBrokerLogsElS3El {
    type O = BlockAssignable<MskClusterLoggingInfoElBrokerLogsElS3El>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskClusterLoggingInfoElBrokerLogsElS3El {
    #[doc = ""]
    pub enabled: PrimField<bool>,
}

impl BuildMskClusterLoggingInfoElBrokerLogsElS3El {
    pub fn build(self) -> MskClusterLoggingInfoElBrokerLogsElS3El {
        MskClusterLoggingInfoElBrokerLogsElS3El {
            bucket: core::default::Default::default(),
            enabled: self.enabled,
            prefix: core::default::Default::default(),
        }
    }
}

pub struct MskClusterLoggingInfoElBrokerLogsElS3ElRef {
    shared: StackShared,
    base: String,
}

impl Ref for MskClusterLoggingInfoElBrokerLogsElS3ElRef {
    fn new(shared: StackShared, base: String) -> MskClusterLoggingInfoElBrokerLogsElS3ElRef {
        MskClusterLoggingInfoElBrokerLogsElS3ElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskClusterLoggingInfoElBrokerLogsElS3ElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }
}

#[derive(Serialize, Default)]
struct MskClusterLoggingInfoElBrokerLogsElDynamic {
    cloudwatch_logs: Option<DynamicBlock<MskClusterLoggingInfoElBrokerLogsElCloudwatchLogsEl>>,
    firehose: Option<DynamicBlock<MskClusterLoggingInfoElBrokerLogsElFirehoseEl>>,
    s3: Option<DynamicBlock<MskClusterLoggingInfoElBrokerLogsElS3El>>,
}

#[derive(Serialize)]
pub struct MskClusterLoggingInfoElBrokerLogsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cloudwatch_logs: Option<Vec<MskClusterLoggingInfoElBrokerLogsElCloudwatchLogsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    firehose: Option<Vec<MskClusterLoggingInfoElBrokerLogsElFirehoseEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3: Option<Vec<MskClusterLoggingInfoElBrokerLogsElS3El>>,
    dynamic: MskClusterLoggingInfoElBrokerLogsElDynamic,
}

impl MskClusterLoggingInfoElBrokerLogsEl {
    #[doc = "Set the field `cloudwatch_logs`.\n"]
    pub fn set_cloudwatch_logs(
        mut self,
        v: impl Into<BlockAssignable<MskClusterLoggingInfoElBrokerLogsElCloudwatchLogsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cloudwatch_logs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cloudwatch_logs = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `firehose`.\n"]
    pub fn set_firehose(
        mut self,
        v: impl Into<BlockAssignable<MskClusterLoggingInfoElBrokerLogsElFirehoseEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.firehose = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.firehose = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `s3`.\n"]
    pub fn set_s3(
        mut self,
        v: impl Into<BlockAssignable<MskClusterLoggingInfoElBrokerLogsElS3El>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.s3 = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.s3 = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for MskClusterLoggingInfoElBrokerLogsEl {
    type O = BlockAssignable<MskClusterLoggingInfoElBrokerLogsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskClusterLoggingInfoElBrokerLogsEl {}

impl BuildMskClusterLoggingInfoElBrokerLogsEl {
    pub fn build(self) -> MskClusterLoggingInfoElBrokerLogsEl {
        MskClusterLoggingInfoElBrokerLogsEl {
            cloudwatch_logs: core::default::Default::default(),
            firehose: core::default::Default::default(),
            s3: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct MskClusterLoggingInfoElBrokerLogsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for MskClusterLoggingInfoElBrokerLogsElRef {
    fn new(shared: StackShared, base: String) -> MskClusterLoggingInfoElBrokerLogsElRef {
        MskClusterLoggingInfoElBrokerLogsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskClusterLoggingInfoElBrokerLogsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloudwatch_logs` after provisioning.\n"]
    pub fn cloudwatch_logs(
        &self,
    ) -> ListRef<MskClusterLoggingInfoElBrokerLogsElCloudwatchLogsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cloudwatch_logs", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `firehose` after provisioning.\n"]
    pub fn firehose(&self) -> ListRef<MskClusterLoggingInfoElBrokerLogsElFirehoseElRef> {
        ListRef::new(self.shared().clone(), format!("{}.firehose", self.base))
    }

    #[doc = "Get a reference to the value of field `s3` after provisioning.\n"]
    pub fn s3(&self) -> ListRef<MskClusterLoggingInfoElBrokerLogsElS3ElRef> {
        ListRef::new(self.shared().clone(), format!("{}.s3", self.base))
    }
}

#[derive(Serialize, Default)]
struct MskClusterLoggingInfoElDynamic {
    broker_logs: Option<DynamicBlock<MskClusterLoggingInfoElBrokerLogsEl>>,
}

#[derive(Serialize)]
pub struct MskClusterLoggingInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    broker_logs: Option<Vec<MskClusterLoggingInfoElBrokerLogsEl>>,
    dynamic: MskClusterLoggingInfoElDynamic,
}

impl MskClusterLoggingInfoEl {
    #[doc = "Set the field `broker_logs`.\n"]
    pub fn set_broker_logs(
        mut self,
        v: impl Into<BlockAssignable<MskClusterLoggingInfoElBrokerLogsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.broker_logs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.broker_logs = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for MskClusterLoggingInfoEl {
    type O = BlockAssignable<MskClusterLoggingInfoEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskClusterLoggingInfoEl {}

impl BuildMskClusterLoggingInfoEl {
    pub fn build(self) -> MskClusterLoggingInfoEl {
        MskClusterLoggingInfoEl {
            broker_logs: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct MskClusterLoggingInfoElRef {
    shared: StackShared,
    base: String,
}

impl Ref for MskClusterLoggingInfoElRef {
    fn new(shared: StackShared, base: String) -> MskClusterLoggingInfoElRef {
        MskClusterLoggingInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskClusterLoggingInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `broker_logs` after provisioning.\n"]
    pub fn broker_logs(&self) -> ListRef<MskClusterLoggingInfoElBrokerLogsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.broker_logs", self.base))
    }
}

#[derive(Serialize)]
pub struct MskClusterOpenMonitoringElPrometheusElJmxExporterEl {
    enabled_in_broker: PrimField<bool>,
}

impl MskClusterOpenMonitoringElPrometheusElJmxExporterEl {}

impl ToListMappable for MskClusterOpenMonitoringElPrometheusElJmxExporterEl {
    type O = BlockAssignable<MskClusterOpenMonitoringElPrometheusElJmxExporterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskClusterOpenMonitoringElPrometheusElJmxExporterEl {
    #[doc = ""]
    pub enabled_in_broker: PrimField<bool>,
}

impl BuildMskClusterOpenMonitoringElPrometheusElJmxExporterEl {
    pub fn build(self) -> MskClusterOpenMonitoringElPrometheusElJmxExporterEl {
        MskClusterOpenMonitoringElPrometheusElJmxExporterEl {
            enabled_in_broker: self.enabled_in_broker,
        }
    }
}

pub struct MskClusterOpenMonitoringElPrometheusElJmxExporterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for MskClusterOpenMonitoringElPrometheusElJmxExporterElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MskClusterOpenMonitoringElPrometheusElJmxExporterElRef {
        MskClusterOpenMonitoringElPrometheusElJmxExporterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskClusterOpenMonitoringElPrometheusElJmxExporterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled_in_broker` after provisioning.\n"]
    pub fn enabled_in_broker(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enabled_in_broker", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct MskClusterOpenMonitoringElPrometheusElNodeExporterEl {
    enabled_in_broker: PrimField<bool>,
}

impl MskClusterOpenMonitoringElPrometheusElNodeExporterEl {}

impl ToListMappable for MskClusterOpenMonitoringElPrometheusElNodeExporterEl {
    type O = BlockAssignable<MskClusterOpenMonitoringElPrometheusElNodeExporterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskClusterOpenMonitoringElPrometheusElNodeExporterEl {
    #[doc = ""]
    pub enabled_in_broker: PrimField<bool>,
}

impl BuildMskClusterOpenMonitoringElPrometheusElNodeExporterEl {
    pub fn build(self) -> MskClusterOpenMonitoringElPrometheusElNodeExporterEl {
        MskClusterOpenMonitoringElPrometheusElNodeExporterEl {
            enabled_in_broker: self.enabled_in_broker,
        }
    }
}

pub struct MskClusterOpenMonitoringElPrometheusElNodeExporterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for MskClusterOpenMonitoringElPrometheusElNodeExporterElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MskClusterOpenMonitoringElPrometheusElNodeExporterElRef {
        MskClusterOpenMonitoringElPrometheusElNodeExporterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskClusterOpenMonitoringElPrometheusElNodeExporterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled_in_broker` after provisioning.\n"]
    pub fn enabled_in_broker(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enabled_in_broker", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct MskClusterOpenMonitoringElPrometheusElDynamic {
    jmx_exporter: Option<DynamicBlock<MskClusterOpenMonitoringElPrometheusElJmxExporterEl>>,
    node_exporter: Option<DynamicBlock<MskClusterOpenMonitoringElPrometheusElNodeExporterEl>>,
}

#[derive(Serialize)]
pub struct MskClusterOpenMonitoringElPrometheusEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    jmx_exporter: Option<Vec<MskClusterOpenMonitoringElPrometheusElJmxExporterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_exporter: Option<Vec<MskClusterOpenMonitoringElPrometheusElNodeExporterEl>>,
    dynamic: MskClusterOpenMonitoringElPrometheusElDynamic,
}

impl MskClusterOpenMonitoringElPrometheusEl {
    #[doc = "Set the field `jmx_exporter`.\n"]
    pub fn set_jmx_exporter(
        mut self,
        v: impl Into<BlockAssignable<MskClusterOpenMonitoringElPrometheusElJmxExporterEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.jmx_exporter = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.jmx_exporter = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `node_exporter`.\n"]
    pub fn set_node_exporter(
        mut self,
        v: impl Into<BlockAssignable<MskClusterOpenMonitoringElPrometheusElNodeExporterEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.node_exporter = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.node_exporter = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for MskClusterOpenMonitoringElPrometheusEl {
    type O = BlockAssignable<MskClusterOpenMonitoringElPrometheusEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskClusterOpenMonitoringElPrometheusEl {}

impl BuildMskClusterOpenMonitoringElPrometheusEl {
    pub fn build(self) -> MskClusterOpenMonitoringElPrometheusEl {
        MskClusterOpenMonitoringElPrometheusEl {
            jmx_exporter: core::default::Default::default(),
            node_exporter: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct MskClusterOpenMonitoringElPrometheusElRef {
    shared: StackShared,
    base: String,
}

impl Ref for MskClusterOpenMonitoringElPrometheusElRef {
    fn new(shared: StackShared, base: String) -> MskClusterOpenMonitoringElPrometheusElRef {
        MskClusterOpenMonitoringElPrometheusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskClusterOpenMonitoringElPrometheusElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `jmx_exporter` after provisioning.\n"]
    pub fn jmx_exporter(&self) -> ListRef<MskClusterOpenMonitoringElPrometheusElJmxExporterElRef> {
        ListRef::new(self.shared().clone(), format!("{}.jmx_exporter", self.base))
    }

    #[doc = "Get a reference to the value of field `node_exporter` after provisioning.\n"]
    pub fn node_exporter(
        &self,
    ) -> ListRef<MskClusterOpenMonitoringElPrometheusElNodeExporterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.node_exporter", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct MskClusterOpenMonitoringElDynamic {
    prometheus: Option<DynamicBlock<MskClusterOpenMonitoringElPrometheusEl>>,
}

#[derive(Serialize)]
pub struct MskClusterOpenMonitoringEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    prometheus: Option<Vec<MskClusterOpenMonitoringElPrometheusEl>>,
    dynamic: MskClusterOpenMonitoringElDynamic,
}

impl MskClusterOpenMonitoringEl {
    #[doc = "Set the field `prometheus`.\n"]
    pub fn set_prometheus(
        mut self,
        v: impl Into<BlockAssignable<MskClusterOpenMonitoringElPrometheusEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.prometheus = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.prometheus = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for MskClusterOpenMonitoringEl {
    type O = BlockAssignable<MskClusterOpenMonitoringEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskClusterOpenMonitoringEl {}

impl BuildMskClusterOpenMonitoringEl {
    pub fn build(self) -> MskClusterOpenMonitoringEl {
        MskClusterOpenMonitoringEl {
            prometheus: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct MskClusterOpenMonitoringElRef {
    shared: StackShared,
    base: String,
}

impl Ref for MskClusterOpenMonitoringElRef {
    fn new(shared: StackShared, base: String) -> MskClusterOpenMonitoringElRef {
        MskClusterOpenMonitoringElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskClusterOpenMonitoringElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `prometheus` after provisioning.\n"]
    pub fn prometheus(&self) -> ListRef<MskClusterOpenMonitoringElPrometheusElRef> {
        ListRef::new(self.shared().clone(), format!("{}.prometheus", self.base))
    }
}

#[derive(Serialize)]
pub struct MskClusterRebalancingEl {
    status: PrimField<String>,
}

impl MskClusterRebalancingEl {}

impl ToListMappable for MskClusterRebalancingEl {
    type O = BlockAssignable<MskClusterRebalancingEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskClusterRebalancingEl {
    #[doc = ""]
    pub status: PrimField<String>,
}

impl BuildMskClusterRebalancingEl {
    pub fn build(self) -> MskClusterRebalancingEl {
        MskClusterRebalancingEl {
            status: self.status,
        }
    }
}

pub struct MskClusterRebalancingElRef {
    shared: StackShared,
    base: String,
}

impl Ref for MskClusterRebalancingElRef {
    fn new(shared: StackShared, base: String) -> MskClusterRebalancingElRef {
        MskClusterRebalancingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskClusterRebalancingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct MskClusterTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl MskClusterTimeoutsEl {
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

impl ToListMappable for MskClusterTimeoutsEl {
    type O = BlockAssignable<MskClusterTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskClusterTimeoutsEl {}

impl BuildMskClusterTimeoutsEl {
    pub fn build(self) -> MskClusterTimeoutsEl {
        MskClusterTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct MskClusterTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for MskClusterTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> MskClusterTimeoutsElRef {
        MskClusterTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskClusterTimeoutsElRef {
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
struct MskClusterDynamic {
    broker_node_group_info: Option<DynamicBlock<MskClusterBrokerNodeGroupInfoEl>>,
    client_authentication: Option<DynamicBlock<MskClusterClientAuthenticationEl>>,
    configuration_info: Option<DynamicBlock<MskClusterConfigurationInfoEl>>,
    encryption_info: Option<DynamicBlock<MskClusterEncryptionInfoEl>>,
    logging_info: Option<DynamicBlock<MskClusterLoggingInfoEl>>,
    open_monitoring: Option<DynamicBlock<MskClusterOpenMonitoringEl>>,
    rebalancing: Option<DynamicBlock<MskClusterRebalancingEl>>,
}
