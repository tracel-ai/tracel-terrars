use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct DataEc2TransitGatewayMulticastDomainData {
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
    transit_gateway_multicast_domain_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<Vec<DataEc2TransitGatewayMulticastDomainFilterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DataEc2TransitGatewayMulticastDomainTimeoutsEl>,
    dynamic: DataEc2TransitGatewayMulticastDomainDynamic,
}

struct DataEc2TransitGatewayMulticastDomain_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataEc2TransitGatewayMulticastDomainData>,
}

#[derive(Clone)]
pub struct DataEc2TransitGatewayMulticastDomain(Rc<DataEc2TransitGatewayMulticastDomain_>);

impl DataEc2TransitGatewayMulticastDomain {
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

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc = "Set the field `transit_gateway_multicast_domain_id`.\n"]
    pub fn set_transit_gateway_multicast_domain_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().transit_gateway_multicast_domain_id = Some(v.into());
        self
    }

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(
        self,
        v: impl Into<BlockAssignable<DataEc2TransitGatewayMulticastDomainFilterEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().filter = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.filter = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(
        self,
        v: impl Into<DataEc2TransitGatewayMulticastDomainTimeoutsEl>,
    ) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `associations` after provisioning.\n"]
    pub fn associations(&self) -> ListRef<DataEc2TransitGatewayMulticastDomainAssociationsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.associations", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `auto_accept_shared_associations` after provisioning.\n"]
    pub fn auto_accept_shared_associations(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.auto_accept_shared_associations", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `igmpv2_support` after provisioning.\n"]
    pub fn igmpv2_support(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.igmpv2_support", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `members` after provisioning.\n"]
    pub fn members(&self) -> ListRef<DataEc2TransitGatewayMulticastDomainMembersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.members", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.owner_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `sources` after provisioning.\n"]
    pub fn sources(&self) -> ListRef<DataEc2TransitGatewayMulticastDomainSourcesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.sources", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `static_sources_support` after provisioning.\n"]
    pub fn static_sources_support(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.static_sources_support", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `transit_gateway_attachment_id` after provisioning.\n"]
    pub fn transit_gateway_attachment_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.transit_gateway_attachment_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `transit_gateway_id` after provisioning.\n"]
    pub fn transit_gateway_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.transit_gateway_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `transit_gateway_multicast_domain_id` after provisioning.\n"]
    pub fn transit_gateway_multicast_domain_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.transit_gateway_multicast_domain_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataEc2TransitGatewayMulticastDomainTimeoutsElRef {
        DataEc2TransitGatewayMulticastDomainTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for DataEc2TransitGatewayMulticastDomain {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Datasource for DataEc2TransitGatewayMulticastDomain {}

impl ToListMappable for DataEc2TransitGatewayMulticastDomain {
    type O = ListRef<DataEc2TransitGatewayMulticastDomainRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataEc2TransitGatewayMulticastDomain_ {
    fn extract_datasource_type(&self) -> String {
        "aws_ec2_transit_gateway_multicast_domain".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataEc2TransitGatewayMulticastDomain {
    pub tf_id: String,
}

impl BuildDataEc2TransitGatewayMulticastDomain {
    pub fn build(self, stack: &mut Stack) -> DataEc2TransitGatewayMulticastDomain {
        let out =
            DataEc2TransitGatewayMulticastDomain(Rc::new(DataEc2TransitGatewayMulticastDomain_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(DataEc2TransitGatewayMulticastDomainData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    for_each: None,
                    id: core::default::Default::default(),
                    region: core::default::Default::default(),
                    tags: core::default::Default::default(),
                    transit_gateway_multicast_domain_id: core::default::Default::default(),
                    filter: core::default::Default::default(),
                    timeouts: core::default::Default::default(),
                    dynamic: Default::default(),
                }),
            }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataEc2TransitGatewayMulticastDomainRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEc2TransitGatewayMulticastDomainRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl DataEc2TransitGatewayMulticastDomainRef {
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
    pub fn associations(&self) -> ListRef<DataEc2TransitGatewayMulticastDomainAssociationsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.associations", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `auto_accept_shared_associations` after provisioning.\n"]
    pub fn auto_accept_shared_associations(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.auto_accept_shared_associations", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `igmpv2_support` after provisioning.\n"]
    pub fn igmpv2_support(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.igmpv2_support", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `members` after provisioning.\n"]
    pub fn members(&self) -> ListRef<DataEc2TransitGatewayMulticastDomainMembersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.members", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.owner_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `sources` after provisioning.\n"]
    pub fn sources(&self) -> ListRef<DataEc2TransitGatewayMulticastDomainSourcesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.sources", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `static_sources_support` after provisioning.\n"]
    pub fn static_sources_support(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.static_sources_support", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `transit_gateway_attachment_id` after provisioning.\n"]
    pub fn transit_gateway_attachment_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.transit_gateway_attachment_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `transit_gateway_id` after provisioning.\n"]
    pub fn transit_gateway_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.transit_gateway_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `transit_gateway_multicast_domain_id` after provisioning.\n"]
    pub fn transit_gateway_multicast_domain_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.transit_gateway_multicast_domain_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataEc2TransitGatewayMulticastDomainTimeoutsElRef {
        DataEc2TransitGatewayMulticastDomainTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DataEc2TransitGatewayMulticastDomainAssociationsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    subnet_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transit_gateway_attachment_id: Option<PrimField<String>>,
}

impl DataEc2TransitGatewayMulticastDomainAssociationsEl {
    #[doc = "Set the field `subnet_id`.\n"]
    pub fn set_subnet_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.subnet_id = Some(v.into());
        self
    }

    #[doc = "Set the field `transit_gateway_attachment_id`.\n"]
    pub fn set_transit_gateway_attachment_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.transit_gateway_attachment_id = Some(v.into());
        self
    }
}

impl ToListMappable for DataEc2TransitGatewayMulticastDomainAssociationsEl {
    type O = BlockAssignable<DataEc2TransitGatewayMulticastDomainAssociationsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2TransitGatewayMulticastDomainAssociationsEl {}

impl BuildDataEc2TransitGatewayMulticastDomainAssociationsEl {
    pub fn build(self) -> DataEc2TransitGatewayMulticastDomainAssociationsEl {
        DataEc2TransitGatewayMulticastDomainAssociationsEl {
            subnet_id: core::default::Default::default(),
            transit_gateway_attachment_id: core::default::Default::default(),
        }
    }
}

pub struct DataEc2TransitGatewayMulticastDomainAssociationsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEc2TransitGatewayMulticastDomainAssociationsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataEc2TransitGatewayMulticastDomainAssociationsElRef {
        DataEc2TransitGatewayMulticastDomainAssociationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2TransitGatewayMulticastDomainAssociationsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `subnet_id` after provisioning.\n"]
    pub fn subnet_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subnet_id", self.base))
    }

    #[doc = "Get a reference to the value of field `transit_gateway_attachment_id` after provisioning.\n"]
    pub fn transit_gateway_attachment_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.transit_gateway_attachment_id", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataEc2TransitGatewayMulticastDomainMembersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    group_ip_address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_interface_id: Option<PrimField<String>>,
}

impl DataEc2TransitGatewayMulticastDomainMembersEl {
    #[doc = "Set the field `group_ip_address`.\n"]
    pub fn set_group_ip_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.group_ip_address = Some(v.into());
        self
    }

    #[doc = "Set the field `network_interface_id`.\n"]
    pub fn set_network_interface_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network_interface_id = Some(v.into());
        self
    }
}

impl ToListMappable for DataEc2TransitGatewayMulticastDomainMembersEl {
    type O = BlockAssignable<DataEc2TransitGatewayMulticastDomainMembersEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2TransitGatewayMulticastDomainMembersEl {}

impl BuildDataEc2TransitGatewayMulticastDomainMembersEl {
    pub fn build(self) -> DataEc2TransitGatewayMulticastDomainMembersEl {
        DataEc2TransitGatewayMulticastDomainMembersEl {
            group_ip_address: core::default::Default::default(),
            network_interface_id: core::default::Default::default(),
        }
    }
}

pub struct DataEc2TransitGatewayMulticastDomainMembersElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEc2TransitGatewayMulticastDomainMembersElRef {
    fn new(shared: StackShared, base: String) -> DataEc2TransitGatewayMulticastDomainMembersElRef {
        DataEc2TransitGatewayMulticastDomainMembersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2TransitGatewayMulticastDomainMembersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `group_ip_address` after provisioning.\n"]
    pub fn group_ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.group_ip_address", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `network_interface_id` after provisioning.\n"]
    pub fn network_interface_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network_interface_id", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataEc2TransitGatewayMulticastDomainSourcesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    group_ip_address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_interface_id: Option<PrimField<String>>,
}

impl DataEc2TransitGatewayMulticastDomainSourcesEl {
    #[doc = "Set the field `group_ip_address`.\n"]
    pub fn set_group_ip_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.group_ip_address = Some(v.into());
        self
    }

    #[doc = "Set the field `network_interface_id`.\n"]
    pub fn set_network_interface_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network_interface_id = Some(v.into());
        self
    }
}

impl ToListMappable for DataEc2TransitGatewayMulticastDomainSourcesEl {
    type O = BlockAssignable<DataEc2TransitGatewayMulticastDomainSourcesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2TransitGatewayMulticastDomainSourcesEl {}

impl BuildDataEc2TransitGatewayMulticastDomainSourcesEl {
    pub fn build(self) -> DataEc2TransitGatewayMulticastDomainSourcesEl {
        DataEc2TransitGatewayMulticastDomainSourcesEl {
            group_ip_address: core::default::Default::default(),
            network_interface_id: core::default::Default::default(),
        }
    }
}

pub struct DataEc2TransitGatewayMulticastDomainSourcesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEc2TransitGatewayMulticastDomainSourcesElRef {
    fn new(shared: StackShared, base: String) -> DataEc2TransitGatewayMulticastDomainSourcesElRef {
        DataEc2TransitGatewayMulticastDomainSourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2TransitGatewayMulticastDomainSourcesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `group_ip_address` after provisioning.\n"]
    pub fn group_ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.group_ip_address", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `network_interface_id` after provisioning.\n"]
    pub fn network_interface_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network_interface_id", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataEc2TransitGatewayMulticastDomainFilterEl {
    name: PrimField<String>,
    values: SetField<PrimField<String>>,
}

impl DataEc2TransitGatewayMulticastDomainFilterEl {}

impl ToListMappable for DataEc2TransitGatewayMulticastDomainFilterEl {
    type O = BlockAssignable<DataEc2TransitGatewayMulticastDomainFilterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2TransitGatewayMulticastDomainFilterEl {
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub values: SetField<PrimField<String>>,
}

impl BuildDataEc2TransitGatewayMulticastDomainFilterEl {
    pub fn build(self) -> DataEc2TransitGatewayMulticastDomainFilterEl {
        DataEc2TransitGatewayMulticastDomainFilterEl {
            name: self.name,
            values: self.values,
        }
    }
}

pub struct DataEc2TransitGatewayMulticastDomainFilterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEc2TransitGatewayMulticastDomainFilterElRef {
    fn new(shared: StackShared, base: String) -> DataEc2TransitGatewayMulticastDomainFilterElRef {
        DataEc2TransitGatewayMulticastDomainFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2TransitGatewayMulticastDomainFilterElRef {
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
pub struct DataEc2TransitGatewayMulticastDomainTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    read: Option<PrimField<String>>,
}

impl DataEc2TransitGatewayMulticastDomainTimeoutsEl {
    #[doc = "Set the field `read`.\n"]
    pub fn set_read(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.read = Some(v.into());
        self
    }
}

impl ToListMappable for DataEc2TransitGatewayMulticastDomainTimeoutsEl {
    type O = BlockAssignable<DataEc2TransitGatewayMulticastDomainTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2TransitGatewayMulticastDomainTimeoutsEl {}

impl BuildDataEc2TransitGatewayMulticastDomainTimeoutsEl {
    pub fn build(self) -> DataEc2TransitGatewayMulticastDomainTimeoutsEl {
        DataEc2TransitGatewayMulticastDomainTimeoutsEl {
            read: core::default::Default::default(),
        }
    }
}

pub struct DataEc2TransitGatewayMulticastDomainTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEc2TransitGatewayMulticastDomainTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DataEc2TransitGatewayMulticastDomainTimeoutsElRef {
        DataEc2TransitGatewayMulticastDomainTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2TransitGatewayMulticastDomainTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataEc2TransitGatewayMulticastDomainDynamic {
    filter: Option<DynamicBlock<DataEc2TransitGatewayMulticastDomainFilterEl>>,
}
