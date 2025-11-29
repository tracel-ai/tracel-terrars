use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataVpnConnectionData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpn_connection_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<Vec<DataVpnConnectionFilterEl>>,
    dynamic: DataVpnConnectionDynamic,
}

struct DataVpnConnection_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataVpnConnectionData>,
}

#[derive(Clone)]
pub struct DataVpnConnection(Rc<DataVpnConnection_>);

impl DataVpnConnection {
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

    #[doc = "Set the field `vpn_connection_id`.\n"]
    pub fn set_vpn_connection_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().vpn_connection_id = Some(v.into());
        self
    }

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(self, v: impl Into<BlockAssignable<DataVpnConnectionFilterEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().filter = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.filter = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `category` after provisioning.\n"]
    pub fn category(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.category", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `core_network_arn` after provisioning.\n"]
    pub fn core_network_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.core_network_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `core_network_attachment_arn` after provisioning.\n"]
    pub fn core_network_attachment_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.core_network_attachment_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `customer_gateway_configuration` after provisioning.\n"]
    pub fn customer_gateway_configuration(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.customer_gateway_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `customer_gateway_id` after provisioning.\n"]
    pub fn customer_gateway_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.customer_gateway_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `gateway_association_state` after provisioning.\n"]
    pub fn gateway_association_state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.gateway_association_state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pre_shared_key_arn` after provisioning.\n"]
    pub fn pre_shared_key_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.pre_shared_key_arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `routes` after provisioning.\n"]
    pub fn routes(&self) -> ListRef<DataVpnConnectionRoutesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.routes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `transit_gateway_id` after provisioning.\n"]
    pub fn transit_gateway_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.transit_gateway_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vgw_telemetries` after provisioning.\n"]
    pub fn vgw_telemetries(&self) -> ListRef<DataVpnConnectionVgwTelemetriesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.vgw_telemetries", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpn_connection_id` after provisioning.\n"]
    pub fn vpn_connection_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpn_connection_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpn_gateway_id` after provisioning.\n"]
    pub fn vpn_gateway_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpn_gateway_id", self.extract_ref()))
    }
}

impl Referable for DataVpnConnection {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataVpnConnection { }

impl ToListMappable for DataVpnConnection {
    type O = ListRef<DataVpnConnectionRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataVpnConnection_ {
    fn extract_datasource_type(&self) -> String {
        "aws_vpn_connection".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataVpnConnection {
    pub tf_id: String,
}

impl BuildDataVpnConnection {
    pub fn build(self, stack: &mut Stack) -> DataVpnConnection {
        let out = DataVpnConnection(Rc::new(DataVpnConnection_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataVpnConnectionData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                region: core::default::Default::default(),
                vpn_connection_id: core::default::Default::default(),
                filter: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataVpnConnectionRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataVpnConnectionRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataVpnConnectionRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `category` after provisioning.\n"]
    pub fn category(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.category", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `core_network_arn` after provisioning.\n"]
    pub fn core_network_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.core_network_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `core_network_attachment_arn` after provisioning.\n"]
    pub fn core_network_attachment_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.core_network_attachment_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `customer_gateway_configuration` after provisioning.\n"]
    pub fn customer_gateway_configuration(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.customer_gateway_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `customer_gateway_id` after provisioning.\n"]
    pub fn customer_gateway_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.customer_gateway_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `gateway_association_state` after provisioning.\n"]
    pub fn gateway_association_state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.gateway_association_state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pre_shared_key_arn` after provisioning.\n"]
    pub fn pre_shared_key_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.pre_shared_key_arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `routes` after provisioning.\n"]
    pub fn routes(&self) -> ListRef<DataVpnConnectionRoutesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.routes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `transit_gateway_id` after provisioning.\n"]
    pub fn transit_gateway_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.transit_gateway_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vgw_telemetries` after provisioning.\n"]
    pub fn vgw_telemetries(&self) -> ListRef<DataVpnConnectionVgwTelemetriesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.vgw_telemetries", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpn_connection_id` after provisioning.\n"]
    pub fn vpn_connection_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpn_connection_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpn_gateway_id` after provisioning.\n"]
    pub fn vpn_gateway_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpn_gateway_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataVpnConnectionRoutesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_cidr_block: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
}

impl DataVpnConnectionRoutesEl {
    #[doc = "Set the field `destination_cidr_block`.\n"]
    pub fn set_destination_cidr_block(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.destination_cidr_block = Some(v.into());
        self
    }

    #[doc = "Set the field `source`.\n"]
    pub fn set_source(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source = Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }
}

impl ToListMappable for DataVpnConnectionRoutesEl {
    type O = BlockAssignable<DataVpnConnectionRoutesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVpnConnectionRoutesEl {}

impl BuildDataVpnConnectionRoutesEl {
    pub fn build(self) -> DataVpnConnectionRoutesEl {
        DataVpnConnectionRoutesEl {
            destination_cidr_block: core::default::Default::default(),
            source: core::default::Default::default(),
            state: core::default::Default::default(),
        }
    }
}

pub struct DataVpnConnectionRoutesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataVpnConnectionRoutesElRef {
    fn new(shared: StackShared, base: String) -> DataVpnConnectionRoutesElRef {
        DataVpnConnectionRoutesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVpnConnectionRoutesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destination_cidr_block` after provisioning.\n"]
    pub fn destination_cidr_block(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.destination_cidr_block", self.base))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}

#[derive(Serialize)]
pub struct DataVpnConnectionVgwTelemetriesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    accepted_route_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_status_change: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    outside_ip_address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status_message: Option<PrimField<String>>,
}

impl DataVpnConnectionVgwTelemetriesEl {
    #[doc = "Set the field `accepted_route_count`.\n"]
    pub fn set_accepted_route_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.accepted_route_count = Some(v.into());
        self
    }

    #[doc = "Set the field `last_status_change`.\n"]
    pub fn set_last_status_change(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.last_status_change = Some(v.into());
        self
    }

    #[doc = "Set the field `outside_ip_address`.\n"]
    pub fn set_outside_ip_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.outside_ip_address = Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status = Some(v.into());
        self
    }

    #[doc = "Set the field `status_message`.\n"]
    pub fn set_status_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status_message = Some(v.into());
        self
    }
}

impl ToListMappable for DataVpnConnectionVgwTelemetriesEl {
    type O = BlockAssignable<DataVpnConnectionVgwTelemetriesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVpnConnectionVgwTelemetriesEl {}

impl BuildDataVpnConnectionVgwTelemetriesEl {
    pub fn build(self) -> DataVpnConnectionVgwTelemetriesEl {
        DataVpnConnectionVgwTelemetriesEl {
            accepted_route_count: core::default::Default::default(),
            last_status_change: core::default::Default::default(),
            outside_ip_address: core::default::Default::default(),
            status: core::default::Default::default(),
            status_message: core::default::Default::default(),
        }
    }
}

pub struct DataVpnConnectionVgwTelemetriesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataVpnConnectionVgwTelemetriesElRef {
    fn new(shared: StackShared, base: String) -> DataVpnConnectionVgwTelemetriesElRef {
        DataVpnConnectionVgwTelemetriesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVpnConnectionVgwTelemetriesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `accepted_route_count` after provisioning.\n"]
    pub fn accepted_route_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.accepted_route_count", self.base))
    }

    #[doc = "Get a reference to the value of field `last_status_change` after provisioning.\n"]
    pub fn last_status_change(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.last_status_change", self.base))
    }

    #[doc = "Get a reference to the value of field `outside_ip_address` after provisioning.\n"]
    pub fn outside_ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.outside_ip_address", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc = "Get a reference to the value of field `status_message` after provisioning.\n"]
    pub fn status_message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status_message", self.base))
    }
}

#[derive(Serialize)]
pub struct DataVpnConnectionFilterEl {
    name: PrimField<String>,
    values: SetField<PrimField<String>>,
}

impl DataVpnConnectionFilterEl { }

impl ToListMappable for DataVpnConnectionFilterEl {
    type O = BlockAssignable<DataVpnConnectionFilterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVpnConnectionFilterEl {
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub values: SetField<PrimField<String>>,
}

impl BuildDataVpnConnectionFilterEl {
    pub fn build(self) -> DataVpnConnectionFilterEl {
        DataVpnConnectionFilterEl {
            name: self.name,
            values: self.values,
        }
    }
}

pub struct DataVpnConnectionFilterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataVpnConnectionFilterElRef {
    fn new(shared: StackShared, base: String) -> DataVpnConnectionFilterElRef {
        DataVpnConnectionFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVpnConnectionFilterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataVpnConnectionDynamic {
    filter: Option<DynamicBlock<DataVpnConnectionFilterEl>>,
}
