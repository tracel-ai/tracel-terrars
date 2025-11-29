use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataOdbNetworkPeeringConnectionsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    odb_peering_connections: Option<Vec<DataOdbNetworkPeeringConnectionsOdbPeeringConnectionsEl>>,
    dynamic: DataOdbNetworkPeeringConnectionsDynamic,
}

struct DataOdbNetworkPeeringConnections_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataOdbNetworkPeeringConnectionsData>,
}

#[derive(Clone)]
pub struct DataOdbNetworkPeeringConnections(Rc<DataOdbNetworkPeeringConnections_>);

impl DataOdbNetworkPeeringConnections {
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `odb_peering_connections`.\n"]
    pub fn set_odb_peering_connections(
        self,
        v: impl Into<BlockAssignable<DataOdbNetworkPeeringConnectionsOdbPeeringConnectionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().odb_peering_connections = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.odb_peering_connections = Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `odb_peering_connections` after provisioning.\n"]
    pub fn odb_peering_connections(&self) -> ListRef<DataOdbNetworkPeeringConnectionsOdbPeeringConnectionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.odb_peering_connections", self.extract_ref()))
    }
}

impl Referable for DataOdbNetworkPeeringConnections {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataOdbNetworkPeeringConnections { }

impl ToListMappable for DataOdbNetworkPeeringConnections {
    type O = ListRef<DataOdbNetworkPeeringConnectionsRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataOdbNetworkPeeringConnections_ {
    fn extract_datasource_type(&self) -> String {
        "aws_odb_network_peering_connections".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataOdbNetworkPeeringConnections {
    pub tf_id: String,
}

impl BuildDataOdbNetworkPeeringConnections {
    pub fn build(self, stack: &mut Stack) -> DataOdbNetworkPeeringConnections {
        let out = DataOdbNetworkPeeringConnections(Rc::new(DataOdbNetworkPeeringConnections_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataOdbNetworkPeeringConnectionsData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                region: core::default::Default::default(),
                odb_peering_connections: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataOdbNetworkPeeringConnectionsRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataOdbNetworkPeeringConnectionsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataOdbNetworkPeeringConnectionsRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `odb_peering_connections` after provisioning.\n"]
    pub fn odb_peering_connections(&self) -> ListRef<DataOdbNetworkPeeringConnectionsOdbPeeringConnectionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.odb_peering_connections", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataOdbNetworkPeeringConnectionsOdbPeeringConnectionsEl {}

impl DataOdbNetworkPeeringConnectionsOdbPeeringConnectionsEl { }

impl ToListMappable for DataOdbNetworkPeeringConnectionsOdbPeeringConnectionsEl {
    type O = BlockAssignable<DataOdbNetworkPeeringConnectionsOdbPeeringConnectionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOdbNetworkPeeringConnectionsOdbPeeringConnectionsEl {}

impl BuildDataOdbNetworkPeeringConnectionsOdbPeeringConnectionsEl {
    pub fn build(self) -> DataOdbNetworkPeeringConnectionsOdbPeeringConnectionsEl {
        DataOdbNetworkPeeringConnectionsOdbPeeringConnectionsEl {}
    }
}

pub struct DataOdbNetworkPeeringConnectionsOdbPeeringConnectionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataOdbNetworkPeeringConnectionsOdbPeeringConnectionsElRef {
    fn new(shared: StackShared, base: String) -> DataOdbNetworkPeeringConnectionsOdbPeeringConnectionsElRef {
        DataOdbNetworkPeeringConnectionsOdbPeeringConnectionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOdbNetworkPeeringConnectionsOdbPeeringConnectionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct DataOdbNetworkPeeringConnectionsDynamic {
    odb_peering_connections: Option<DynamicBlock<DataOdbNetworkPeeringConnectionsOdbPeeringConnectionsEl>>,
}
