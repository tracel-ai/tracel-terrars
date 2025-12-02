use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataMskBrokerNodesData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    cluster_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}
struct DataMskBrokerNodes_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataMskBrokerNodesData>,
}
#[derive(Clone)]
pub struct DataMskBrokerNodes(Rc<DataMskBrokerNodes_>);
impl DataMskBrokerNodes {
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
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `cluster_arn` after provisioning.\n"]
    pub fn cluster_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `node_info_list` after provisioning.\n"]
    pub fn node_info_list(&self) -> ListRef<DataMskBrokerNodesNodeInfoListElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.node_info_list", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
}
impl Referable for DataMskBrokerNodes {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataMskBrokerNodes {}
impl ToListMappable for DataMskBrokerNodes {
    type O = ListRef<DataMskBrokerNodesRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataMskBrokerNodes_ {
    fn extract_datasource_type(&self) -> String {
        "aws_msk_broker_nodes".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataMskBrokerNodes {
    pub tf_id: String,
    #[doc = ""]
    pub cluster_arn: PrimField<String>,
}
impl BuildDataMskBrokerNodes {
    pub fn build(self, stack: &mut Stack) -> DataMskBrokerNodes {
        let out = DataMskBrokerNodes(Rc::new(DataMskBrokerNodes_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataMskBrokerNodesData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                cluster_arn: self.cluster_arn,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataMskBrokerNodesRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataMskBrokerNodesRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataMskBrokerNodesRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `cluster_arn` after provisioning.\n"]
    pub fn cluster_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `node_info_list` after provisioning.\n"]
    pub fn node_info_list(&self) -> ListRef<DataMskBrokerNodesNodeInfoListElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.node_info_list", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataMskBrokerNodesNodeInfoListEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    attached_eni_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    broker_id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_subnet: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_vpc_ip_address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    endpoints: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_arn: Option<PrimField<String>>,
}
impl DataMskBrokerNodesNodeInfoListEl {
    #[doc = "Set the field `attached_eni_id`.\n"]
    pub fn set_attached_eni_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.attached_eni_id = Some(v.into());
        self
    }
    #[doc = "Set the field `broker_id`.\n"]
    pub fn set_broker_id(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.broker_id = Some(v.into());
        self
    }
    #[doc = "Set the field `client_subnet`.\n"]
    pub fn set_client_subnet(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_subnet = Some(v.into());
        self
    }
    #[doc = "Set the field `client_vpc_ip_address`.\n"]
    pub fn set_client_vpc_ip_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_vpc_ip_address = Some(v.into());
        self
    }
    #[doc = "Set the field `endpoints`.\n"]
    pub fn set_endpoints(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.endpoints = Some(v.into());
        self
    }
    #[doc = "Set the field `node_arn`.\n"]
    pub fn set_node_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.node_arn = Some(v.into());
        self
    }
}
impl ToListMappable for DataMskBrokerNodesNodeInfoListEl {
    type O = BlockAssignable<DataMskBrokerNodesNodeInfoListEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataMskBrokerNodesNodeInfoListEl {}
impl BuildDataMskBrokerNodesNodeInfoListEl {
    pub fn build(self) -> DataMskBrokerNodesNodeInfoListEl {
        DataMskBrokerNodesNodeInfoListEl {
            attached_eni_id: core::default::Default::default(),
            broker_id: core::default::Default::default(),
            client_subnet: core::default::Default::default(),
            client_vpc_ip_address: core::default::Default::default(),
            endpoints: core::default::Default::default(),
            node_arn: core::default::Default::default(),
        }
    }
}
pub struct DataMskBrokerNodesNodeInfoListElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataMskBrokerNodesNodeInfoListElRef {
    fn new(shared: StackShared, base: String) -> DataMskBrokerNodesNodeInfoListElRef {
        DataMskBrokerNodesNodeInfoListElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataMskBrokerNodesNodeInfoListElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `attached_eni_id` after provisioning.\n"]
    pub fn attached_eni_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.attached_eni_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `broker_id` after provisioning.\n"]
    pub fn broker_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.broker_id", self.base))
    }
    #[doc = "Get a reference to the value of field `client_subnet` after provisioning.\n"]
    pub fn client_subnet(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_subnet", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `client_vpc_ip_address` after provisioning.\n"]
    pub fn client_vpc_ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_vpc_ip_address", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `endpoints` after provisioning.\n"]
    pub fn endpoints(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.endpoints", self.base))
    }
    #[doc = "Get a reference to the value of field `node_arn` after provisioning.\n"]
    pub fn node_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.node_arn", self.base))
    }
}
