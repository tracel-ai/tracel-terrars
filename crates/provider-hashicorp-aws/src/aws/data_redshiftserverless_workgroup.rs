use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataRedshiftserverlessWorkgroupData {
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
    workgroup_name: PrimField<String>,
}
struct DataRedshiftserverlessWorkgroup_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataRedshiftserverlessWorkgroupData>,
}
#[derive(Clone)]
pub struct DataRedshiftserverlessWorkgroup(Rc<DataRedshiftserverlessWorkgroup_>);
impl DataRedshiftserverlessWorkgroup {
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
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> ListRef<DataRedshiftserverlessWorkgroupEndpointElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.endpoint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enhanced_vpc_routing` after provisioning.\n"]
    pub fn enhanced_vpc_routing(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enhanced_vpc_routing", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `namespace_name` after provisioning.\n"]
    pub fn namespace_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.namespace_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `publicly_accessible` after provisioning.\n"]
    pub fn publicly_accessible(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.publicly_accessible", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.security_group_ids", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.subnet_ids", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `track_name` after provisioning.\n"]
    pub fn track_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.track_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `workgroup_id` after provisioning.\n"]
    pub fn workgroup_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.workgroup_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `workgroup_name` after provisioning.\n"]
    pub fn workgroup_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.workgroup_name", self.extract_ref()),
        )
    }
}
impl Referable for DataRedshiftserverlessWorkgroup {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataRedshiftserverlessWorkgroup {}
impl ToListMappable for DataRedshiftserverlessWorkgroup {
    type O = ListRef<DataRedshiftserverlessWorkgroupRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataRedshiftserverlessWorkgroup_ {
    fn extract_datasource_type(&self) -> String {
        "aws_redshiftserverless_workgroup".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataRedshiftserverlessWorkgroup {
    pub tf_id: String,
    #[doc = ""]
    pub workgroup_name: PrimField<String>,
}
impl BuildDataRedshiftserverlessWorkgroup {
    pub fn build(self, stack: &mut Stack) -> DataRedshiftserverlessWorkgroup {
        let out = DataRedshiftserverlessWorkgroup(Rc::new(DataRedshiftserverlessWorkgroup_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataRedshiftserverlessWorkgroupData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                workgroup_name: self.workgroup_name,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataRedshiftserverlessWorkgroupRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataRedshiftserverlessWorkgroupRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataRedshiftserverlessWorkgroupRef {
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
    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> ListRef<DataRedshiftserverlessWorkgroupEndpointElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.endpoint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enhanced_vpc_routing` after provisioning.\n"]
    pub fn enhanced_vpc_routing(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enhanced_vpc_routing", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `namespace_name` after provisioning.\n"]
    pub fn namespace_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.namespace_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `publicly_accessible` after provisioning.\n"]
    pub fn publicly_accessible(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.publicly_accessible", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.security_group_ids", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.subnet_ids", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `track_name` after provisioning.\n"]
    pub fn track_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.track_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `workgroup_id` after provisioning.\n"]
    pub fn workgroup_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.workgroup_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `workgroup_name` after provisioning.\n"]
    pub fn workgroup_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.workgroup_name", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataRedshiftserverlessWorkgroupEndpointElVpcEndpointElNetworkInterfaceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    availability_zone: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_interface_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    private_ip_address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnet_id: Option<PrimField<String>>,
}
impl DataRedshiftserverlessWorkgroupEndpointElVpcEndpointElNetworkInterfaceEl {
    #[doc = "Set the field `availability_zone`.\n"]
    pub fn set_availability_zone(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.availability_zone = Some(v.into());
        self
    }
    #[doc = "Set the field `network_interface_id`.\n"]
    pub fn set_network_interface_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network_interface_id = Some(v.into());
        self
    }
    #[doc = "Set the field `private_ip_address`.\n"]
    pub fn set_private_ip_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.private_ip_address = Some(v.into());
        self
    }
    #[doc = "Set the field `subnet_id`.\n"]
    pub fn set_subnet_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.subnet_id = Some(v.into());
        self
    }
}
impl ToListMappable for DataRedshiftserverlessWorkgroupEndpointElVpcEndpointElNetworkInterfaceEl {
    type O =
        BlockAssignable<DataRedshiftserverlessWorkgroupEndpointElVpcEndpointElNetworkInterfaceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataRedshiftserverlessWorkgroupEndpointElVpcEndpointElNetworkInterfaceEl {}
impl BuildDataRedshiftserverlessWorkgroupEndpointElVpcEndpointElNetworkInterfaceEl {
    pub fn build(self) -> DataRedshiftserverlessWorkgroupEndpointElVpcEndpointElNetworkInterfaceEl {
        DataRedshiftserverlessWorkgroupEndpointElVpcEndpointElNetworkInterfaceEl {
            availability_zone: core::default::Default::default(),
            network_interface_id: core::default::Default::default(),
            private_ip_address: core::default::Default::default(),
            subnet_id: core::default::Default::default(),
        }
    }
}
pub struct DataRedshiftserverlessWorkgroupEndpointElVpcEndpointElNetworkInterfaceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataRedshiftserverlessWorkgroupEndpointElVpcEndpointElNetworkInterfaceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataRedshiftserverlessWorkgroupEndpointElVpcEndpointElNetworkInterfaceElRef {
        DataRedshiftserverlessWorkgroupEndpointElVpcEndpointElNetworkInterfaceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataRedshiftserverlessWorkgroupEndpointElVpcEndpointElNetworkInterfaceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `availability_zone` after provisioning.\n"]
    pub fn availability_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.availability_zone", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `network_interface_id` after provisioning.\n"]
    pub fn network_interface_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network_interface_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `private_ip_address` after provisioning.\n"]
    pub fn private_ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.private_ip_address", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `subnet_id` after provisioning.\n"]
    pub fn subnet_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subnet_id", self.base))
    }
}
#[derive(Serialize)]
pub struct DataRedshiftserverlessWorkgroupEndpointElVpcEndpointEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    network_interface:
        Option<ListField<DataRedshiftserverlessWorkgroupEndpointElVpcEndpointElNetworkInterfaceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_endpoint_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_id: Option<PrimField<String>>,
}
impl DataRedshiftserverlessWorkgroupEndpointElVpcEndpointEl {
    #[doc = "Set the field `network_interface`.\n"]
    pub fn set_network_interface(
        mut self,
        v: impl Into<
            ListField<DataRedshiftserverlessWorkgroupEndpointElVpcEndpointElNetworkInterfaceEl>,
        >,
    ) -> Self {
        self.network_interface = Some(v.into());
        self
    }
    #[doc = "Set the field `vpc_endpoint_id`.\n"]
    pub fn set_vpc_endpoint_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.vpc_endpoint_id = Some(v.into());
        self
    }
    #[doc = "Set the field `vpc_id`.\n"]
    pub fn set_vpc_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.vpc_id = Some(v.into());
        self
    }
}
impl ToListMappable for DataRedshiftserverlessWorkgroupEndpointElVpcEndpointEl {
    type O = BlockAssignable<DataRedshiftserverlessWorkgroupEndpointElVpcEndpointEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataRedshiftserverlessWorkgroupEndpointElVpcEndpointEl {}
impl BuildDataRedshiftserverlessWorkgroupEndpointElVpcEndpointEl {
    pub fn build(self) -> DataRedshiftserverlessWorkgroupEndpointElVpcEndpointEl {
        DataRedshiftserverlessWorkgroupEndpointElVpcEndpointEl {
            network_interface: core::default::Default::default(),
            vpc_endpoint_id: core::default::Default::default(),
            vpc_id: core::default::Default::default(),
        }
    }
}
pub struct DataRedshiftserverlessWorkgroupEndpointElVpcEndpointElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataRedshiftserverlessWorkgroupEndpointElVpcEndpointElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataRedshiftserverlessWorkgroupEndpointElVpcEndpointElRef {
        DataRedshiftserverlessWorkgroupEndpointElVpcEndpointElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataRedshiftserverlessWorkgroupEndpointElVpcEndpointElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `network_interface` after provisioning.\n"]
    pub fn network_interface(
        &self,
    ) -> ListRef<DataRedshiftserverlessWorkgroupEndpointElVpcEndpointElNetworkInterfaceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_interface", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `vpc_endpoint_id` after provisioning.\n"]
    pub fn vpc_endpoint_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vpc_endpoint_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.base))
    }
}
#[derive(Serialize)]
pub struct DataRedshiftserverlessWorkgroupEndpointEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_endpoint: Option<ListField<DataRedshiftserverlessWorkgroupEndpointElVpcEndpointEl>>,
}
impl DataRedshiftserverlessWorkgroupEndpointEl {
    #[doc = "Set the field `address`.\n"]
    pub fn set_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.address = Some(v.into());
        self
    }
    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
    #[doc = "Set the field `vpc_endpoint`.\n"]
    pub fn set_vpc_endpoint(
        mut self,
        v: impl Into<ListField<DataRedshiftserverlessWorkgroupEndpointElVpcEndpointEl>>,
    ) -> Self {
        self.vpc_endpoint = Some(v.into());
        self
    }
}
impl ToListMappable for DataRedshiftserverlessWorkgroupEndpointEl {
    type O = BlockAssignable<DataRedshiftserverlessWorkgroupEndpointEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataRedshiftserverlessWorkgroupEndpointEl {}
impl BuildDataRedshiftserverlessWorkgroupEndpointEl {
    pub fn build(self) -> DataRedshiftserverlessWorkgroupEndpointEl {
        DataRedshiftserverlessWorkgroupEndpointEl {
            address: core::default::Default::default(),
            port: core::default::Default::default(),
            vpc_endpoint: core::default::Default::default(),
        }
    }
}
pub struct DataRedshiftserverlessWorkgroupEndpointElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataRedshiftserverlessWorkgroupEndpointElRef {
    fn new(shared: StackShared, base: String) -> DataRedshiftserverlessWorkgroupEndpointElRef {
        DataRedshiftserverlessWorkgroupEndpointElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataRedshiftserverlessWorkgroupEndpointElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `address` after provisioning.\n"]
    pub fn address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.address", self.base))
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
    #[doc = "Get a reference to the value of field `vpc_endpoint` after provisioning.\n"]
    pub fn vpc_endpoint(
        &self,
    ) -> ListRef<DataRedshiftserverlessWorkgroupEndpointElVpcEndpointElRef> {
        ListRef::new(self.shared().clone(), format!("{}.vpc_endpoint", self.base))
    }
}
