use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataEc2TransitGatewayRouteTableRoutesData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    transit_gateway_route_table_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<Vec<DataEc2TransitGatewayRouteTableRoutesFilterEl>>,
    dynamic: DataEc2TransitGatewayRouteTableRoutesDynamic,
}

struct DataEc2TransitGatewayRouteTableRoutes_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataEc2TransitGatewayRouteTableRoutesData>,
}

#[derive(Clone)]
pub struct DataEc2TransitGatewayRouteTableRoutes(Rc<DataEc2TransitGatewayRouteTableRoutes_>);

impl DataEc2TransitGatewayRouteTableRoutes {
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

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(self, v: impl Into<BlockAssignable<DataEc2TransitGatewayRouteTableRoutesFilterEl>>) -> Self {
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `routes` after provisioning.\n"]
    pub fn routes(&self) -> ListRef<DataEc2TransitGatewayRouteTableRoutesRoutesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.routes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `transit_gateway_route_table_id` after provisioning.\n"]
    pub fn transit_gateway_route_table_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.transit_gateway_route_table_id", self.extract_ref()))
    }
}

impl Referable for DataEc2TransitGatewayRouteTableRoutes {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataEc2TransitGatewayRouteTableRoutes { }

impl ToListMappable for DataEc2TransitGatewayRouteTableRoutes {
    type O = ListRef<DataEc2TransitGatewayRouteTableRoutesRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataEc2TransitGatewayRouteTableRoutes_ {
    fn extract_datasource_type(&self) -> String {
        "aws_ec2_transit_gateway_route_table_routes".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataEc2TransitGatewayRouteTableRoutes {
    pub tf_id: String,
    #[doc = ""]
    pub transit_gateway_route_table_id: PrimField<String>,
}

impl BuildDataEc2TransitGatewayRouteTableRoutes {
    pub fn build(self, stack: &mut Stack) -> DataEc2TransitGatewayRouteTableRoutes {
        let out = DataEc2TransitGatewayRouteTableRoutes(Rc::new(DataEc2TransitGatewayRouteTableRoutes_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataEc2TransitGatewayRouteTableRoutesData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                transit_gateway_route_table_id: self.transit_gateway_route_table_id,
                filter: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataEc2TransitGatewayRouteTableRoutesRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEc2TransitGatewayRouteTableRoutesRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataEc2TransitGatewayRouteTableRoutesRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `routes` after provisioning.\n"]
    pub fn routes(&self) -> ListRef<DataEc2TransitGatewayRouteTableRoutesRoutesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.routes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `transit_gateway_route_table_id` after provisioning.\n"]
    pub fn transit_gateway_route_table_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.transit_gateway_route_table_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataEc2TransitGatewayRouteTableRoutesRoutesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_cidr_block: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix_list_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transit_gateway_route_table_announcement_id: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}

impl DataEc2TransitGatewayRouteTableRoutesRoutesEl {
    #[doc = "Set the field `destination_cidr_block`.\n"]
    pub fn set_destination_cidr_block(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.destination_cidr_block = Some(v.into());
        self
    }

    #[doc = "Set the field `prefix_list_id`.\n"]
    pub fn set_prefix_list_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix_list_id = Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }

    #[doc = "Set the field `transit_gateway_route_table_announcement_id`.\n"]
    pub fn set_transit_gateway_route_table_announcement_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.transit_gateway_route_table_announcement_id = Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}

impl ToListMappable for DataEc2TransitGatewayRouteTableRoutesRoutesEl {
    type O = BlockAssignable<DataEc2TransitGatewayRouteTableRoutesRoutesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2TransitGatewayRouteTableRoutesRoutesEl {}

impl BuildDataEc2TransitGatewayRouteTableRoutesRoutesEl {
    pub fn build(self) -> DataEc2TransitGatewayRouteTableRoutesRoutesEl {
        DataEc2TransitGatewayRouteTableRoutesRoutesEl {
            destination_cidr_block: core::default::Default::default(),
            prefix_list_id: core::default::Default::default(),
            state: core::default::Default::default(),
            transit_gateway_route_table_announcement_id: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct DataEc2TransitGatewayRouteTableRoutesRoutesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEc2TransitGatewayRouteTableRoutesRoutesElRef {
    fn new(shared: StackShared, base: String) -> DataEc2TransitGatewayRouteTableRoutesRoutesElRef {
        DataEc2TransitGatewayRouteTableRoutesRoutesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2TransitGatewayRouteTableRoutesRoutesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destination_cidr_block` after provisioning.\n"]
    pub fn destination_cidr_block(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.destination_cidr_block", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix_list_id` after provisioning.\n"]
    pub fn prefix_list_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix_list_id", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc = "Get a reference to the value of field `transit_gateway_route_table_announcement_id` after provisioning.\n"]
    pub fn transit_gateway_route_table_announcement_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.transit_gateway_route_table_announcement_id", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEc2TransitGatewayRouteTableRoutesFilterEl {
    name: PrimField<String>,
    values: SetField<PrimField<String>>,
}

impl DataEc2TransitGatewayRouteTableRoutesFilterEl { }

impl ToListMappable for DataEc2TransitGatewayRouteTableRoutesFilterEl {
    type O = BlockAssignable<DataEc2TransitGatewayRouteTableRoutesFilterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2TransitGatewayRouteTableRoutesFilterEl {
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub values: SetField<PrimField<String>>,
}

impl BuildDataEc2TransitGatewayRouteTableRoutesFilterEl {
    pub fn build(self) -> DataEc2TransitGatewayRouteTableRoutesFilterEl {
        DataEc2TransitGatewayRouteTableRoutesFilterEl {
            name: self.name,
            values: self.values,
        }
    }
}

pub struct DataEc2TransitGatewayRouteTableRoutesFilterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEc2TransitGatewayRouteTableRoutesFilterElRef {
    fn new(shared: StackShared, base: String) -> DataEc2TransitGatewayRouteTableRoutesFilterElRef {
        DataEc2TransitGatewayRouteTableRoutesFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2TransitGatewayRouteTableRoutesFilterElRef {
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
struct DataEc2TransitGatewayRouteTableRoutesDynamic {
    filter: Option<DynamicBlock<DataEc2TransitGatewayRouteTableRoutesFilterEl>>,
}
