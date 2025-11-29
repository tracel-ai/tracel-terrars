use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataEc2LocalGatewayVirtualInterfaceData {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<Vec<DataEc2LocalGatewayVirtualInterfaceFilterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DataEc2LocalGatewayVirtualInterfaceTimeoutsEl>,
    dynamic: DataEc2LocalGatewayVirtualInterfaceDynamic,
}

struct DataEc2LocalGatewayVirtualInterface_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataEc2LocalGatewayVirtualInterfaceData>,
}

#[derive(Clone)]
pub struct DataEc2LocalGatewayVirtualInterface(Rc<DataEc2LocalGatewayVirtualInterface_>);

impl DataEc2LocalGatewayVirtualInterface {
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

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(self, v: impl Into<BlockAssignable<DataEc2LocalGatewayVirtualInterfaceFilterEl>>) -> Self {
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
    pub fn set_timeouts(self, v: impl Into<DataEc2LocalGatewayVirtualInterfaceTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `local_address` after provisioning.\n"]
    pub fn local_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.local_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `local_bgp_asn` after provisioning.\n"]
    pub fn local_bgp_asn(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.local_bgp_asn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `local_gateway_id` after provisioning.\n"]
    pub fn local_gateway_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.local_gateway_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `local_gateway_virtual_interface_ids` after provisioning.\n"]
    pub fn local_gateway_virtual_interface_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.local_gateway_virtual_interface_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `peer_address` after provisioning.\n"]
    pub fn peer_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.peer_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `peer_bgp_asn` after provisioning.\n"]
    pub fn peer_bgp_asn(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.peer_bgp_asn", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `vlan` after provisioning.\n"]
    pub fn vlan(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.vlan", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataEc2LocalGatewayVirtualInterfaceTimeoutsElRef {
        DataEc2LocalGatewayVirtualInterfaceTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for DataEc2LocalGatewayVirtualInterface {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataEc2LocalGatewayVirtualInterface { }

impl ToListMappable for DataEc2LocalGatewayVirtualInterface {
    type O = ListRef<DataEc2LocalGatewayVirtualInterfaceRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataEc2LocalGatewayVirtualInterface_ {
    fn extract_datasource_type(&self) -> String {
        "aws_ec2_local_gateway_virtual_interface".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataEc2LocalGatewayVirtualInterface {
    pub tf_id: String,
}

impl BuildDataEc2LocalGatewayVirtualInterface {
    pub fn build(self, stack: &mut Stack) -> DataEc2LocalGatewayVirtualInterface {
        let out = DataEc2LocalGatewayVirtualInterface(Rc::new(DataEc2LocalGatewayVirtualInterface_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataEc2LocalGatewayVirtualInterfaceData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                filter: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataEc2LocalGatewayVirtualInterfaceRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEc2LocalGatewayVirtualInterfaceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataEc2LocalGatewayVirtualInterfaceRef {
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

    #[doc = "Get a reference to the value of field `local_address` after provisioning.\n"]
    pub fn local_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.local_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `local_bgp_asn` after provisioning.\n"]
    pub fn local_bgp_asn(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.local_bgp_asn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `local_gateway_id` after provisioning.\n"]
    pub fn local_gateway_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.local_gateway_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `local_gateway_virtual_interface_ids` after provisioning.\n"]
    pub fn local_gateway_virtual_interface_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.local_gateway_virtual_interface_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `peer_address` after provisioning.\n"]
    pub fn peer_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.peer_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `peer_bgp_asn` after provisioning.\n"]
    pub fn peer_bgp_asn(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.peer_bgp_asn", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `vlan` after provisioning.\n"]
    pub fn vlan(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.vlan", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataEc2LocalGatewayVirtualInterfaceTimeoutsElRef {
        DataEc2LocalGatewayVirtualInterfaceTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DataEc2LocalGatewayVirtualInterfaceFilterEl {
    name: PrimField<String>,
    values: SetField<PrimField<String>>,
}

impl DataEc2LocalGatewayVirtualInterfaceFilterEl { }

impl ToListMappable for DataEc2LocalGatewayVirtualInterfaceFilterEl {
    type O = BlockAssignable<DataEc2LocalGatewayVirtualInterfaceFilterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2LocalGatewayVirtualInterfaceFilterEl {
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub values: SetField<PrimField<String>>,
}

impl BuildDataEc2LocalGatewayVirtualInterfaceFilterEl {
    pub fn build(self) -> DataEc2LocalGatewayVirtualInterfaceFilterEl {
        DataEc2LocalGatewayVirtualInterfaceFilterEl {
            name: self.name,
            values: self.values,
        }
    }
}

pub struct DataEc2LocalGatewayVirtualInterfaceFilterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEc2LocalGatewayVirtualInterfaceFilterElRef {
    fn new(shared: StackShared, base: String) -> DataEc2LocalGatewayVirtualInterfaceFilterElRef {
        DataEc2LocalGatewayVirtualInterfaceFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2LocalGatewayVirtualInterfaceFilterElRef {
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
pub struct DataEc2LocalGatewayVirtualInterfaceTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    read: Option<PrimField<String>>,
}

impl DataEc2LocalGatewayVirtualInterfaceTimeoutsEl {
    #[doc = "Set the field `read`.\n"]
    pub fn set_read(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.read = Some(v.into());
        self
    }
}

impl ToListMappable for DataEc2LocalGatewayVirtualInterfaceTimeoutsEl {
    type O = BlockAssignable<DataEc2LocalGatewayVirtualInterfaceTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2LocalGatewayVirtualInterfaceTimeoutsEl {}

impl BuildDataEc2LocalGatewayVirtualInterfaceTimeoutsEl {
    pub fn build(self) -> DataEc2LocalGatewayVirtualInterfaceTimeoutsEl {
        DataEc2LocalGatewayVirtualInterfaceTimeoutsEl { read: core::default::Default::default() }
    }
}

pub struct DataEc2LocalGatewayVirtualInterfaceTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEc2LocalGatewayVirtualInterfaceTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DataEc2LocalGatewayVirtualInterfaceTimeoutsElRef {
        DataEc2LocalGatewayVirtualInterfaceTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2LocalGatewayVirtualInterfaceTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataEc2LocalGatewayVirtualInterfaceDynamic {
    filter: Option<DynamicBlock<DataEc2LocalGatewayVirtualInterfaceFilterEl>>,
}
