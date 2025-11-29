use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataMskClusterData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    cluster_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
}

struct DataMskCluster_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataMskClusterData>,
}

#[derive(Clone)]
pub struct DataMskCluster(Rc<DataMskCluster_>);

impl DataMskCluster {
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

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers` after provisioning.\n"]
    pub fn bootstrap_brokers(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bootstrap_brokers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_public_sasl_iam` after provisioning.\n"]
    pub fn bootstrap_brokers_public_sasl_iam(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bootstrap_brokers_public_sasl_iam", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_public_sasl_scram` after provisioning.\n"]
    pub fn bootstrap_brokers_public_sasl_scram(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bootstrap_brokers_public_sasl_scram", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_public_tls` after provisioning.\n"]
    pub fn bootstrap_brokers_public_tls(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bootstrap_brokers_public_tls", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_sasl_iam` after provisioning.\n"]
    pub fn bootstrap_brokers_sasl_iam(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bootstrap_brokers_sasl_iam", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_sasl_scram` after provisioning.\n"]
    pub fn bootstrap_brokers_sasl_scram(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bootstrap_brokers_sasl_scram", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_tls` after provisioning.\n"]
    pub fn bootstrap_brokers_tls(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bootstrap_brokers_tls", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `broker_node_group_info` after provisioning.\n"]
    pub fn broker_node_group_info(&self) -> ListRef<DataMskClusterBrokerNodeGroupInfoElRef> {
        ListRef::new(self.shared().clone(), format!("{}.broker_node_group_info", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_name` after provisioning.\n"]
    pub fn cluster_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_uuid` after provisioning.\n"]
    pub fn cluster_uuid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_uuid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kafka_version` after provisioning.\n"]
    pub fn kafka_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kafka_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `number_of_broker_nodes` after provisioning.\n"]
    pub fn number_of_broker_nodes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.number_of_broker_nodes", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zookeeper_connect_string` after provisioning.\n"]
    pub fn zookeeper_connect_string(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zookeeper_connect_string", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zookeeper_connect_string_tls` after provisioning.\n"]
    pub fn zookeeper_connect_string_tls(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zookeeper_connect_string_tls", self.extract_ref()))
    }
}

impl Referable for DataMskCluster {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataMskCluster { }

impl ToListMappable for DataMskCluster {
    type O = ListRef<DataMskClusterRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataMskCluster_ {
    fn extract_datasource_type(&self) -> String {
        "aws_msk_cluster".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataMskCluster {
    pub tf_id: String,
    #[doc = ""]
    pub cluster_name: PrimField<String>,
}

impl BuildDataMskCluster {
    pub fn build(self, stack: &mut Stack) -> DataMskCluster {
        let out = DataMskCluster(Rc::new(DataMskCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataMskClusterData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                cluster_name: self.cluster_name,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataMskClusterRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataMskClusterRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataMskClusterRef {
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

    #[doc = "Get a reference to the value of field `bootstrap_brokers` after provisioning.\n"]
    pub fn bootstrap_brokers(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bootstrap_brokers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_public_sasl_iam` after provisioning.\n"]
    pub fn bootstrap_brokers_public_sasl_iam(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bootstrap_brokers_public_sasl_iam", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_public_sasl_scram` after provisioning.\n"]
    pub fn bootstrap_brokers_public_sasl_scram(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bootstrap_brokers_public_sasl_scram", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_public_tls` after provisioning.\n"]
    pub fn bootstrap_brokers_public_tls(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bootstrap_brokers_public_tls", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_sasl_iam` after provisioning.\n"]
    pub fn bootstrap_brokers_sasl_iam(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bootstrap_brokers_sasl_iam", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_sasl_scram` after provisioning.\n"]
    pub fn bootstrap_brokers_sasl_scram(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bootstrap_brokers_sasl_scram", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_tls` after provisioning.\n"]
    pub fn bootstrap_brokers_tls(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bootstrap_brokers_tls", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `broker_node_group_info` after provisioning.\n"]
    pub fn broker_node_group_info(&self) -> ListRef<DataMskClusterBrokerNodeGroupInfoElRef> {
        ListRef::new(self.shared().clone(), format!("{}.broker_node_group_info", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_name` after provisioning.\n"]
    pub fn cluster_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_uuid` after provisioning.\n"]
    pub fn cluster_uuid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_uuid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kafka_version` after provisioning.\n"]
    pub fn kafka_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kafka_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `number_of_broker_nodes` after provisioning.\n"]
    pub fn number_of_broker_nodes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.number_of_broker_nodes", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zookeeper_connect_string` after provisioning.\n"]
    pub fn zookeeper_connect_string(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zookeeper_connect_string", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zookeeper_connect_string_tls` after provisioning.\n"]
    pub fn zookeeper_connect_string_tls(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zookeeper_connect_string_tls", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataMskClusterBrokerNodeGroupInfoElConnectivityInfoElPublicAccessEl {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}

impl DataMskClusterBrokerNodeGroupInfoElConnectivityInfoElPublicAccessEl {
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}

impl ToListMappable for DataMskClusterBrokerNodeGroupInfoElConnectivityInfoElPublicAccessEl {
    type O = BlockAssignable<DataMskClusterBrokerNodeGroupInfoElConnectivityInfoElPublicAccessEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMskClusterBrokerNodeGroupInfoElConnectivityInfoElPublicAccessEl {}

impl BuildDataMskClusterBrokerNodeGroupInfoElConnectivityInfoElPublicAccessEl {
    pub fn build(self) -> DataMskClusterBrokerNodeGroupInfoElConnectivityInfoElPublicAccessEl {
        DataMskClusterBrokerNodeGroupInfoElConnectivityInfoElPublicAccessEl {
            type_: core::default::Default::default(),
        }
    }
}

pub struct DataMskClusterBrokerNodeGroupInfoElConnectivityInfoElPublicAccessElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataMskClusterBrokerNodeGroupInfoElConnectivityInfoElPublicAccessElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataMskClusterBrokerNodeGroupInfoElConnectivityInfoElPublicAccessElRef {
        DataMskClusterBrokerNodeGroupInfoElConnectivityInfoElPublicAccessElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMskClusterBrokerNodeGroupInfoElConnectivityInfoElPublicAccessElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataMskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityElClientAuthenticationElSaslEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    iam: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scram: Option<PrimField<bool>>,
}

impl DataMskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityElClientAuthenticationElSaslEl {
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

impl ToListMappable for DataMskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityElClientAuthenticationElSaslEl {
    type O =
        BlockAssignable<
            DataMskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityElClientAuthenticationElSaslEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityElClientAuthenticationElSaslEl {}

impl BuildDataMskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityElClientAuthenticationElSaslEl {
    pub fn build(
        self,
    ) -> DataMskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityElClientAuthenticationElSaslEl {
        DataMskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityElClientAuthenticationElSaslEl {
            iam: core::default::Default::default(),
            scram: core::default::Default::default(),
        }
    }
}

pub struct DataMskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityElClientAuthenticationElSaslElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataMskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityElClientAuthenticationElSaslElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataMskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityElClientAuthenticationElSaslElRef {
        DataMskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityElClientAuthenticationElSaslElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityElClientAuthenticationElSaslElRef {
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
pub struct DataMskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityElClientAuthenticationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    sasl: Option<
        ListField<DataMskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityElClientAuthenticationElSaslEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    tls: Option<PrimField<bool>>,
}

impl DataMskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityElClientAuthenticationEl {
    #[doc = "Set the field `sasl`.\n"]
    pub fn set_sasl(
        mut self,
        v:
            impl

                    Into<
                        ListField<
                            DataMskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityElClientAuthenticationElSaslEl,
                        >,
                    >,
    ) -> Self {
        self.sasl = Some(v.into());
        self
    }

    #[doc = "Set the field `tls`.\n"]
    pub fn set_tls(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.tls = Some(v.into());
        self
    }
}

impl ToListMappable for DataMskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityElClientAuthenticationEl {
    type O =
        BlockAssignable<
            DataMskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityElClientAuthenticationEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityElClientAuthenticationEl {}

impl BuildDataMskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityElClientAuthenticationEl {
    pub fn build(
        self,
    ) -> DataMskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityElClientAuthenticationEl {
        DataMskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityElClientAuthenticationEl {
            sasl: core::default::Default::default(),
            tls: core::default::Default::default(),
        }
    }
}

pub struct DataMskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityElClientAuthenticationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataMskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityElClientAuthenticationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataMskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityElClientAuthenticationElRef {
        DataMskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityElClientAuthenticationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityElClientAuthenticationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `sasl` after provisioning.\n"]
    pub fn sasl(
        &self,
    ) -> ListRef<
        DataMskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityElClientAuthenticationElSaslElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.sasl", self.base))
    }

    #[doc = "Get a reference to the value of field `tls` after provisioning.\n"]
    pub fn tls(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.tls", self.base))
    }
}

#[derive(Serialize)]
pub struct DataMskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    client_authentication: Option<
        ListField<DataMskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityElClientAuthenticationEl>,
    >,
}

impl DataMskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityEl {
    #[doc = "Set the field `client_authentication`.\n"]
    pub fn set_client_authentication(
        mut self,
        v:
            impl

                    Into<
                        ListField<
                            DataMskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityElClientAuthenticationEl,
                        >,
                    >,
    ) -> Self {
        self.client_authentication = Some(v.into());
        self
    }
}

impl ToListMappable for DataMskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityEl {
    type O = BlockAssignable<DataMskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityEl {}

impl BuildDataMskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityEl {
    pub fn build(self) -> DataMskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityEl {
        DataMskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityEl {
            client_authentication: core::default::Default::default(),
        }
    }
}

pub struct DataMskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataMskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataMskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityElRef {
        DataMskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `client_authentication` after provisioning.\n"]
    pub fn client_authentication(
        &self,
    ) -> ListRef<DataMskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityElClientAuthenticationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.client_authentication", self.base))
    }
}

#[derive(Serialize)]
pub struct DataMskClusterBrokerNodeGroupInfoElConnectivityInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    public_access: Option<ListField<DataMskClusterBrokerNodeGroupInfoElConnectivityInfoElPublicAccessEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_connectivity: Option<ListField<DataMskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityEl>>,
}

impl DataMskClusterBrokerNodeGroupInfoElConnectivityInfoEl {
    #[doc = "Set the field `public_access`.\n"]
    pub fn set_public_access(
        mut self,
        v: impl Into<ListField<DataMskClusterBrokerNodeGroupInfoElConnectivityInfoElPublicAccessEl>>,
    ) -> Self {
        self.public_access = Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_connectivity`.\n"]
    pub fn set_vpc_connectivity(
        mut self,
        v: impl Into<ListField<DataMskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityEl>>,
    ) -> Self {
        self.vpc_connectivity = Some(v.into());
        self
    }
}

impl ToListMappable for DataMskClusterBrokerNodeGroupInfoElConnectivityInfoEl {
    type O = BlockAssignable<DataMskClusterBrokerNodeGroupInfoElConnectivityInfoEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMskClusterBrokerNodeGroupInfoElConnectivityInfoEl {}

impl BuildDataMskClusterBrokerNodeGroupInfoElConnectivityInfoEl {
    pub fn build(self) -> DataMskClusterBrokerNodeGroupInfoElConnectivityInfoEl {
        DataMskClusterBrokerNodeGroupInfoElConnectivityInfoEl {
            public_access: core::default::Default::default(),
            vpc_connectivity: core::default::Default::default(),
        }
    }
}

pub struct DataMskClusterBrokerNodeGroupInfoElConnectivityInfoElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataMskClusterBrokerNodeGroupInfoElConnectivityInfoElRef {
    fn new(shared: StackShared, base: String) -> DataMskClusterBrokerNodeGroupInfoElConnectivityInfoElRef {
        DataMskClusterBrokerNodeGroupInfoElConnectivityInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMskClusterBrokerNodeGroupInfoElConnectivityInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `public_access` after provisioning.\n"]
    pub fn public_access(&self) -> ListRef<DataMskClusterBrokerNodeGroupInfoElConnectivityInfoElPublicAccessElRef> {
        ListRef::new(self.shared().clone(), format!("{}.public_access", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_connectivity` after provisioning.\n"]
    pub fn vpc_connectivity(&self) -> ListRef<DataMskClusterBrokerNodeGroupInfoElConnectivityInfoElVpcConnectivityElRef> {
        ListRef::new(self.shared().clone(), format!("{}.vpc_connectivity", self.base))
    }
}

#[derive(Serialize)]
pub struct DataMskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoElProvisionedThroughputEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volume_throughput: Option<PrimField<f64>>,
}

impl DataMskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoElProvisionedThroughputEl {
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

impl ToListMappable for DataMskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoElProvisionedThroughputEl {
    type O =
        BlockAssignable<DataMskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoElProvisionedThroughputEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoElProvisionedThroughputEl {}

impl BuildDataMskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoElProvisionedThroughputEl {
    pub fn build(self) -> DataMskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoElProvisionedThroughputEl {
        DataMskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoElProvisionedThroughputEl {
            enabled: core::default::Default::default(),
            volume_throughput: core::default::Default::default(),
        }
    }
}

pub struct DataMskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoElProvisionedThroughputElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataMskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoElProvisionedThroughputElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataMskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoElProvisionedThroughputElRef {
        DataMskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoElProvisionedThroughputElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoElProvisionedThroughputElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `volume_throughput` after provisioning.\n"]
    pub fn volume_throughput(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.volume_throughput", self.base))
    }
}

#[derive(Serialize)]
pub struct DataMskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    provisioned_throughput: Option<
        ListField<DataMskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoElProvisionedThroughputEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    volume_size: Option<PrimField<f64>>,
}

impl DataMskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoEl {
    #[doc = "Set the field `provisioned_throughput`.\n"]
    pub fn set_provisioned_throughput(
        mut self,
        v:
            impl

                    Into<
                        ListField<
                            DataMskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoElProvisionedThroughputEl,
                        >,
                    >,
    ) -> Self {
        self.provisioned_throughput = Some(v.into());
        self
    }

    #[doc = "Set the field `volume_size`.\n"]
    pub fn set_volume_size(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.volume_size = Some(v.into());
        self
    }
}

impl ToListMappable for DataMskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoEl {
    type O = BlockAssignable<DataMskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoEl {}

impl BuildDataMskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoEl {
    pub fn build(self) -> DataMskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoEl {
        DataMskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoEl {
            provisioned_throughput: core::default::Default::default(),
            volume_size: core::default::Default::default(),
        }
    }
}

pub struct DataMskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataMskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoElRef {
    fn new(shared: StackShared, base: String) -> DataMskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoElRef {
        DataMskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `provisioned_throughput` after provisioning.\n"]
    pub fn provisioned_throughput(
        &self,
    ) -> ListRef<DataMskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoElProvisionedThroughputElRef> {
        ListRef::new(self.shared().clone(), format!("{}.provisioned_throughput", self.base))
    }

    #[doc = "Get a reference to the value of field `volume_size` after provisioning.\n"]
    pub fn volume_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.volume_size", self.base))
    }
}

#[derive(Serialize)]
pub struct DataMskClusterBrokerNodeGroupInfoElStorageInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ebs_storage_info: Option<ListField<DataMskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoEl>>,
}

impl DataMskClusterBrokerNodeGroupInfoElStorageInfoEl {
    #[doc = "Set the field `ebs_storage_info`.\n"]
    pub fn set_ebs_storage_info(
        mut self,
        v: impl Into<ListField<DataMskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoEl>>,
    ) -> Self {
        self.ebs_storage_info = Some(v.into());
        self
    }
}

impl ToListMappable for DataMskClusterBrokerNodeGroupInfoElStorageInfoEl {
    type O = BlockAssignable<DataMskClusterBrokerNodeGroupInfoElStorageInfoEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMskClusterBrokerNodeGroupInfoElStorageInfoEl {}

impl BuildDataMskClusterBrokerNodeGroupInfoElStorageInfoEl {
    pub fn build(self) -> DataMskClusterBrokerNodeGroupInfoElStorageInfoEl {
        DataMskClusterBrokerNodeGroupInfoElStorageInfoEl { ebs_storage_info: core::default::Default::default() }
    }
}

pub struct DataMskClusterBrokerNodeGroupInfoElStorageInfoElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataMskClusterBrokerNodeGroupInfoElStorageInfoElRef {
    fn new(shared: StackShared, base: String) -> DataMskClusterBrokerNodeGroupInfoElStorageInfoElRef {
        DataMskClusterBrokerNodeGroupInfoElStorageInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMskClusterBrokerNodeGroupInfoElStorageInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ebs_storage_info` after provisioning.\n"]
    pub fn ebs_storage_info(&self) -> ListRef<DataMskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ebs_storage_info", self.base))
    }
}

#[derive(Serialize)]
pub struct DataMskClusterBrokerNodeGroupInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    az_distribution: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_subnets: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    connectivity_info: Option<ListField<DataMskClusterBrokerNodeGroupInfoElConnectivityInfoEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_groups: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_info: Option<ListField<DataMskClusterBrokerNodeGroupInfoElStorageInfoEl>>,
}

impl DataMskClusterBrokerNodeGroupInfoEl {
    #[doc = "Set the field `az_distribution`.\n"]
    pub fn set_az_distribution(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.az_distribution = Some(v.into());
        self
    }

    #[doc = "Set the field `client_subnets`.\n"]
    pub fn set_client_subnets(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.client_subnets = Some(v.into());
        self
    }

    #[doc = "Set the field `connectivity_info`.\n"]
    pub fn set_connectivity_info(
        mut self,
        v: impl Into<ListField<DataMskClusterBrokerNodeGroupInfoElConnectivityInfoEl>>,
    ) -> Self {
        self.connectivity_info = Some(v.into());
        self
    }

    #[doc = "Set the field `instance_type`.\n"]
    pub fn set_instance_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance_type = Some(v.into());
        self
    }

    #[doc = "Set the field `security_groups`.\n"]
    pub fn set_security_groups(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.security_groups = Some(v.into());
        self
    }

    #[doc = "Set the field `storage_info`.\n"]
    pub fn set_storage_info(
        mut self,
        v: impl Into<ListField<DataMskClusterBrokerNodeGroupInfoElStorageInfoEl>>,
    ) -> Self {
        self.storage_info = Some(v.into());
        self
    }
}

impl ToListMappable for DataMskClusterBrokerNodeGroupInfoEl {
    type O = BlockAssignable<DataMskClusterBrokerNodeGroupInfoEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMskClusterBrokerNodeGroupInfoEl {}

impl BuildDataMskClusterBrokerNodeGroupInfoEl {
    pub fn build(self) -> DataMskClusterBrokerNodeGroupInfoEl {
        DataMskClusterBrokerNodeGroupInfoEl {
            az_distribution: core::default::Default::default(),
            client_subnets: core::default::Default::default(),
            connectivity_info: core::default::Default::default(),
            instance_type: core::default::Default::default(),
            security_groups: core::default::Default::default(),
            storage_info: core::default::Default::default(),
        }
    }
}

pub struct DataMskClusterBrokerNodeGroupInfoElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataMskClusterBrokerNodeGroupInfoElRef {
    fn new(shared: StackShared, base: String) -> DataMskClusterBrokerNodeGroupInfoElRef {
        DataMskClusterBrokerNodeGroupInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMskClusterBrokerNodeGroupInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `az_distribution` after provisioning.\n"]
    pub fn az_distribution(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.az_distribution", self.base))
    }

    #[doc = "Get a reference to the value of field `client_subnets` after provisioning.\n"]
    pub fn client_subnets(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.client_subnets", self.base))
    }

    #[doc = "Get a reference to the value of field `connectivity_info` after provisioning.\n"]
    pub fn connectivity_info(&self) -> ListRef<DataMskClusterBrokerNodeGroupInfoElConnectivityInfoElRef> {
        ListRef::new(self.shared().clone(), format!("{}.connectivity_info", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.base))
    }

    #[doc = "Get a reference to the value of field `security_groups` after provisioning.\n"]
    pub fn security_groups(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.security_groups", self.base))
    }

    #[doc = "Get a reference to the value of field `storage_info` after provisioning.\n"]
    pub fn storage_info(&self) -> ListRef<DataMskClusterBrokerNodeGroupInfoElStorageInfoElRef> {
        ListRef::new(self.shared().clone(), format!("{}.storage_info", self.base))
    }
}
