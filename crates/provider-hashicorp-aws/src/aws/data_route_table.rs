use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataRouteTableData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gateway_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    route_table_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnet_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<Vec<DataRouteTableFilterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DataRouteTableTimeoutsEl>,
    dynamic: DataRouteTableDynamic,
}

struct DataRouteTable_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataRouteTableData>,
}

#[derive(Clone)]
pub struct DataRouteTable(Rc<DataRouteTable_>);

impl DataRouteTable {
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

    #[doc = "Set the field `gateway_id`.\n"]
    pub fn set_gateway_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().gateway_id = Some(v.into());
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

    #[doc = "Set the field `route_table_id`.\n"]
    pub fn set_route_table_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().route_table_id = Some(v.into());
        self
    }

    #[doc = "Set the field `subnet_id`.\n"]
    pub fn set_subnet_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().subnet_id = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_id`.\n"]
    pub fn set_vpc_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().vpc_id = Some(v.into());
        self
    }

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(self, v: impl Into<BlockAssignable<DataRouteTableFilterEl>>) -> Self {
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

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DataRouteTableTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `associations` after provisioning.\n"]
    pub fn associations(&self) -> ListRef<DataRouteTableAssociationsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.associations", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `gateway_id` after provisioning.\n"]
    pub fn gateway_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.gateway_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.owner_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `route_table_id` after provisioning.\n"]
    pub fn route_table_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.route_table_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `routes` after provisioning.\n"]
    pub fn routes(&self) -> ListRef<DataRouteTableRoutesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.routes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subnet_id` after provisioning.\n"]
    pub fn subnet_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subnet_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataRouteTableTimeoutsElRef {
        DataRouteTableTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for DataRouteTable {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataRouteTable { }

impl ToListMappable for DataRouteTable {
    type O = ListRef<DataRouteTableRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataRouteTable_ {
    fn extract_datasource_type(&self) -> String {
        "aws_route_table".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataRouteTable {
    pub tf_id: String,
}

impl BuildDataRouteTable {
    pub fn build(self, stack: &mut Stack) -> DataRouteTable {
        let out = DataRouteTable(Rc::new(DataRouteTable_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataRouteTableData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                gateway_id: core::default::Default::default(),
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                route_table_id: core::default::Default::default(),
                subnet_id: core::default::Default::default(),
                tags: core::default::Default::default(),
                vpc_id: core::default::Default::default(),
                filter: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataRouteTableRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataRouteTableRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataRouteTableRef {
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

    #[doc = "Get a reference to the value of field `associations` after provisioning.\n"]
    pub fn associations(&self) -> ListRef<DataRouteTableAssociationsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.associations", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `gateway_id` after provisioning.\n"]
    pub fn gateway_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.gateway_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.owner_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `route_table_id` after provisioning.\n"]
    pub fn route_table_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.route_table_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `routes` after provisioning.\n"]
    pub fn routes(&self) -> ListRef<DataRouteTableRoutesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.routes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subnet_id` after provisioning.\n"]
    pub fn subnet_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subnet_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataRouteTableTimeoutsElRef {
        DataRouteTableTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataRouteTableAssociationsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    gateway_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    main: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    route_table_association_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    route_table_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnet_id: Option<PrimField<String>>,
}

impl DataRouteTableAssociationsEl {
    #[doc = "Set the field `gateway_id`.\n"]
    pub fn set_gateway_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gateway_id = Some(v.into());
        self
    }

    #[doc = "Set the field `main`.\n"]
    pub fn set_main(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.main = Some(v.into());
        self
    }

    #[doc = "Set the field `route_table_association_id`.\n"]
    pub fn set_route_table_association_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.route_table_association_id = Some(v.into());
        self
    }

    #[doc = "Set the field `route_table_id`.\n"]
    pub fn set_route_table_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.route_table_id = Some(v.into());
        self
    }

    #[doc = "Set the field `subnet_id`.\n"]
    pub fn set_subnet_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.subnet_id = Some(v.into());
        self
    }
}

impl ToListMappable for DataRouteTableAssociationsEl {
    type O = BlockAssignable<DataRouteTableAssociationsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRouteTableAssociationsEl {}

impl BuildDataRouteTableAssociationsEl {
    pub fn build(self) -> DataRouteTableAssociationsEl {
        DataRouteTableAssociationsEl {
            gateway_id: core::default::Default::default(),
            main: core::default::Default::default(),
            route_table_association_id: core::default::Default::default(),
            route_table_id: core::default::Default::default(),
            subnet_id: core::default::Default::default(),
        }
    }
}

pub struct DataRouteTableAssociationsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataRouteTableAssociationsElRef {
    fn new(shared: StackShared, base: String) -> DataRouteTableAssociationsElRef {
        DataRouteTableAssociationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRouteTableAssociationsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `gateway_id` after provisioning.\n"]
    pub fn gateway_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.gateway_id", self.base))
    }

    #[doc = "Get a reference to the value of field `main` after provisioning.\n"]
    pub fn main(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.main", self.base))
    }

    #[doc = "Get a reference to the value of field `route_table_association_id` after provisioning.\n"]
    pub fn route_table_association_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.route_table_association_id", self.base))
    }

    #[doc = "Get a reference to the value of field `route_table_id` after provisioning.\n"]
    pub fn route_table_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.route_table_id", self.base))
    }

    #[doc = "Get a reference to the value of field `subnet_id` after provisioning.\n"]
    pub fn subnet_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subnet_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRouteTableRoutesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    carrier_gateway_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cidr_block: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    core_network_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_prefix_list_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    egress_only_gateway_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gateway_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ipv6_cidr_block: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    local_gateway_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nat_gateway_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_interface_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transit_gateway_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_endpoint_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_peering_connection_id: Option<PrimField<String>>,
}

impl DataRouteTableRoutesEl {
    #[doc = "Set the field `carrier_gateway_id`.\n"]
    pub fn set_carrier_gateway_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.carrier_gateway_id = Some(v.into());
        self
    }

    #[doc = "Set the field `cidr_block`.\n"]
    pub fn set_cidr_block(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cidr_block = Some(v.into());
        self
    }

    #[doc = "Set the field `core_network_arn`.\n"]
    pub fn set_core_network_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.core_network_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `destination_prefix_list_id`.\n"]
    pub fn set_destination_prefix_list_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.destination_prefix_list_id = Some(v.into());
        self
    }

    #[doc = "Set the field `egress_only_gateway_id`.\n"]
    pub fn set_egress_only_gateway_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.egress_only_gateway_id = Some(v.into());
        self
    }

    #[doc = "Set the field `gateway_id`.\n"]
    pub fn set_gateway_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gateway_id = Some(v.into());
        self
    }

    #[doc = "Set the field `instance_id`.\n"]
    pub fn set_instance_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance_id = Some(v.into());
        self
    }

    #[doc = "Set the field `ipv6_cidr_block`.\n"]
    pub fn set_ipv6_cidr_block(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ipv6_cidr_block = Some(v.into());
        self
    }

    #[doc = "Set the field `local_gateway_id`.\n"]
    pub fn set_local_gateway_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.local_gateway_id = Some(v.into());
        self
    }

    #[doc = "Set the field `nat_gateway_id`.\n"]
    pub fn set_nat_gateway_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.nat_gateway_id = Some(v.into());
        self
    }

    #[doc = "Set the field `network_interface_id`.\n"]
    pub fn set_network_interface_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network_interface_id = Some(v.into());
        self
    }

    #[doc = "Set the field `transit_gateway_id`.\n"]
    pub fn set_transit_gateway_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.transit_gateway_id = Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_endpoint_id`.\n"]
    pub fn set_vpc_endpoint_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.vpc_endpoint_id = Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_peering_connection_id`.\n"]
    pub fn set_vpc_peering_connection_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.vpc_peering_connection_id = Some(v.into());
        self
    }
}

impl ToListMappable for DataRouteTableRoutesEl {
    type O = BlockAssignable<DataRouteTableRoutesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRouteTableRoutesEl {}

impl BuildDataRouteTableRoutesEl {
    pub fn build(self) -> DataRouteTableRoutesEl {
        DataRouteTableRoutesEl {
            carrier_gateway_id: core::default::Default::default(),
            cidr_block: core::default::Default::default(),
            core_network_arn: core::default::Default::default(),
            destination_prefix_list_id: core::default::Default::default(),
            egress_only_gateway_id: core::default::Default::default(),
            gateway_id: core::default::Default::default(),
            instance_id: core::default::Default::default(),
            ipv6_cidr_block: core::default::Default::default(),
            local_gateway_id: core::default::Default::default(),
            nat_gateway_id: core::default::Default::default(),
            network_interface_id: core::default::Default::default(),
            transit_gateway_id: core::default::Default::default(),
            vpc_endpoint_id: core::default::Default::default(),
            vpc_peering_connection_id: core::default::Default::default(),
        }
    }
}

pub struct DataRouteTableRoutesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataRouteTableRoutesElRef {
    fn new(shared: StackShared, base: String) -> DataRouteTableRoutesElRef {
        DataRouteTableRoutesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRouteTableRoutesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `carrier_gateway_id` after provisioning.\n"]
    pub fn carrier_gateway_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.carrier_gateway_id", self.base))
    }

    #[doc = "Get a reference to the value of field `cidr_block` after provisioning.\n"]
    pub fn cidr_block(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cidr_block", self.base))
    }

    #[doc = "Get a reference to the value of field `core_network_arn` after provisioning.\n"]
    pub fn core_network_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.core_network_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `destination_prefix_list_id` after provisioning.\n"]
    pub fn destination_prefix_list_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.destination_prefix_list_id", self.base))
    }

    #[doc = "Get a reference to the value of field `egress_only_gateway_id` after provisioning.\n"]
    pub fn egress_only_gateway_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.egress_only_gateway_id", self.base))
    }

    #[doc = "Get a reference to the value of field `gateway_id` after provisioning.\n"]
    pub fn gateway_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.gateway_id", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_id` after provisioning.\n"]
    pub fn instance_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_id", self.base))
    }

    #[doc = "Get a reference to the value of field `ipv6_cidr_block` after provisioning.\n"]
    pub fn ipv6_cidr_block(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipv6_cidr_block", self.base))
    }

    #[doc = "Get a reference to the value of field `local_gateway_id` after provisioning.\n"]
    pub fn local_gateway_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.local_gateway_id", self.base))
    }

    #[doc = "Get a reference to the value of field `nat_gateway_id` after provisioning.\n"]
    pub fn nat_gateway_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.nat_gateway_id", self.base))
    }

    #[doc = "Get a reference to the value of field `network_interface_id` after provisioning.\n"]
    pub fn network_interface_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network_interface_id", self.base))
    }

    #[doc = "Get a reference to the value of field `transit_gateway_id` after provisioning.\n"]
    pub fn transit_gateway_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.transit_gateway_id", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_endpoint_id` after provisioning.\n"]
    pub fn vpc_endpoint_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_endpoint_id", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_peering_connection_id` after provisioning.\n"]
    pub fn vpc_peering_connection_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_peering_connection_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRouteTableFilterEl {
    name: PrimField<String>,
    values: SetField<PrimField<String>>,
}

impl DataRouteTableFilterEl { }

impl ToListMappable for DataRouteTableFilterEl {
    type O = BlockAssignable<DataRouteTableFilterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRouteTableFilterEl {
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub values: SetField<PrimField<String>>,
}

impl BuildDataRouteTableFilterEl {
    pub fn build(self) -> DataRouteTableFilterEl {
        DataRouteTableFilterEl {
            name: self.name,
            values: self.values,
        }
    }
}

pub struct DataRouteTableFilterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataRouteTableFilterElRef {
    fn new(shared: StackShared, base: String) -> DataRouteTableFilterElRef {
        DataRouteTableFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRouteTableFilterElRef {
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

#[derive(Serialize)]
pub struct DataRouteTableTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    read: Option<PrimField<String>>,
}

impl DataRouteTableTimeoutsEl {
    #[doc = "Set the field `read`.\n"]
    pub fn set_read(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.read = Some(v.into());
        self
    }
}

impl ToListMappable for DataRouteTableTimeoutsEl {
    type O = BlockAssignable<DataRouteTableTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRouteTableTimeoutsEl {}

impl BuildDataRouteTableTimeoutsEl {
    pub fn build(self) -> DataRouteTableTimeoutsEl {
        DataRouteTableTimeoutsEl { read: core::default::Default::default() }
    }
}

pub struct DataRouteTableTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataRouteTableTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DataRouteTableTimeoutsElRef {
        DataRouteTableTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRouteTableTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataRouteTableDynamic {
    filter: Option<DynamicBlock<DataRouteTableFilterEl>>,
}
