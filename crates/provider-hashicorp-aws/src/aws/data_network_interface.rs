use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct DataNetworkInterfaceData {
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
    filter: Option<Vec<DataNetworkInterfaceFilterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DataNetworkInterfaceTimeoutsEl>,
    dynamic: DataNetworkInterfaceDynamic,
}

struct DataNetworkInterface_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataNetworkInterfaceData>,
}

#[derive(Clone)]
pub struct DataNetworkInterface(Rc<DataNetworkInterface_>);

impl DataNetworkInterface {
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

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(self, v: impl Into<BlockAssignable<DataNetworkInterfaceFilterEl>>) -> Self {
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
    pub fn set_timeouts(self, v: impl Into<DataNetworkInterfaceTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `association` after provisioning.\n"]
    pub fn association(&self) -> ListRef<DataNetworkInterfaceAssociationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.association", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `attachment` after provisioning.\n"]
    pub fn attachment(&self) -> ListRef<DataNetworkInterfaceAttachmentElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.attachment", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `availability_zone` after provisioning.\n"]
    pub fn availability_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.availability_zone", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `interface_type` after provisioning.\n"]
    pub fn interface_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.interface_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `ipv6_addresses` after provisioning.\n"]
    pub fn ipv6_addresses(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.ipv6_addresses", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `mac_address` after provisioning.\n"]
    pub fn mac_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.mac_address", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `outpost_arn` after provisioning.\n"]
    pub fn outpost_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.outpost_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.owner_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `private_dns_name` after provisioning.\n"]
    pub fn private_dns_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.private_dns_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `private_ip` after provisioning.\n"]
    pub fn private_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.private_ip", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `private_ips` after provisioning.\n"]
    pub fn private_ips(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.private_ips", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `requester_id` after provisioning.\n"]
    pub fn requester_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.requester_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `security_groups` after provisioning.\n"]
    pub fn security_groups(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.security_groups", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `subnet_id` after provisioning.\n"]
    pub fn subnet_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subnet_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vpc_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataNetworkInterfaceTimeoutsElRef {
        DataNetworkInterfaceTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for DataNetworkInterface {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Datasource for DataNetworkInterface {}

impl ToListMappable for DataNetworkInterface {
    type O = ListRef<DataNetworkInterfaceRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataNetworkInterface_ {
    fn extract_datasource_type(&self) -> String {
        "aws_network_interface".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataNetworkInterface {
    pub tf_id: String,
}

impl BuildDataNetworkInterface {
    pub fn build(self, stack: &mut Stack) -> DataNetworkInterface {
        let out = DataNetworkInterface(Rc::new(DataNetworkInterface_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataNetworkInterfaceData {
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

pub struct DataNetworkInterfaceRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataNetworkInterfaceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl DataNetworkInterfaceRef {
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

    #[doc = "Get a reference to the value of field `association` after provisioning.\n"]
    pub fn association(&self) -> ListRef<DataNetworkInterfaceAssociationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.association", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `attachment` after provisioning.\n"]
    pub fn attachment(&self) -> ListRef<DataNetworkInterfaceAttachmentElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.attachment", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `availability_zone` after provisioning.\n"]
    pub fn availability_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.availability_zone", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `interface_type` after provisioning.\n"]
    pub fn interface_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.interface_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `ipv6_addresses` after provisioning.\n"]
    pub fn ipv6_addresses(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.ipv6_addresses", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `mac_address` after provisioning.\n"]
    pub fn mac_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.mac_address", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `outpost_arn` after provisioning.\n"]
    pub fn outpost_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.outpost_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.owner_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `private_dns_name` after provisioning.\n"]
    pub fn private_dns_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.private_dns_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `private_ip` after provisioning.\n"]
    pub fn private_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.private_ip", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `private_ips` after provisioning.\n"]
    pub fn private_ips(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.private_ips", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `requester_id` after provisioning.\n"]
    pub fn requester_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.requester_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `security_groups` after provisioning.\n"]
    pub fn security_groups(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.security_groups", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `subnet_id` after provisioning.\n"]
    pub fn subnet_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subnet_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vpc_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataNetworkInterfaceTimeoutsElRef {
        DataNetworkInterfaceTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DataNetworkInterfaceAssociationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allocation_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    association_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    carrier_ip: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_owned_ip: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_owner_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    public_dns_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    public_ip: Option<PrimField<String>>,
}

impl DataNetworkInterfaceAssociationEl {
    #[doc = "Set the field `allocation_id`.\n"]
    pub fn set_allocation_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.allocation_id = Some(v.into());
        self
    }

    #[doc = "Set the field `association_id`.\n"]
    pub fn set_association_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.association_id = Some(v.into());
        self
    }

    #[doc = "Set the field `carrier_ip`.\n"]
    pub fn set_carrier_ip(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.carrier_ip = Some(v.into());
        self
    }

    #[doc = "Set the field `customer_owned_ip`.\n"]
    pub fn set_customer_owned_ip(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.customer_owned_ip = Some(v.into());
        self
    }

    #[doc = "Set the field `ip_owner_id`.\n"]
    pub fn set_ip_owner_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ip_owner_id = Some(v.into());
        self
    }

    #[doc = "Set the field `public_dns_name`.\n"]
    pub fn set_public_dns_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.public_dns_name = Some(v.into());
        self
    }

    #[doc = "Set the field `public_ip`.\n"]
    pub fn set_public_ip(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.public_ip = Some(v.into());
        self
    }
}

impl ToListMappable for DataNetworkInterfaceAssociationEl {
    type O = BlockAssignable<DataNetworkInterfaceAssociationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkInterfaceAssociationEl {}

impl BuildDataNetworkInterfaceAssociationEl {
    pub fn build(self) -> DataNetworkInterfaceAssociationEl {
        DataNetworkInterfaceAssociationEl {
            allocation_id: core::default::Default::default(),
            association_id: core::default::Default::default(),
            carrier_ip: core::default::Default::default(),
            customer_owned_ip: core::default::Default::default(),
            ip_owner_id: core::default::Default::default(),
            public_dns_name: core::default::Default::default(),
            public_ip: core::default::Default::default(),
        }
    }
}

pub struct DataNetworkInterfaceAssociationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataNetworkInterfaceAssociationElRef {
    fn new(shared: StackShared, base: String) -> DataNetworkInterfaceAssociationElRef {
        DataNetworkInterfaceAssociationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkInterfaceAssociationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allocation_id` after provisioning.\n"]
    pub fn allocation_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allocation_id", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `association_id` after provisioning.\n"]
    pub fn association_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.association_id", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `carrier_ip` after provisioning.\n"]
    pub fn carrier_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.carrier_ip", self.base))
    }

    #[doc = "Get a reference to the value of field `customer_owned_ip` after provisioning.\n"]
    pub fn customer_owned_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.customer_owned_ip", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `ip_owner_id` after provisioning.\n"]
    pub fn ip_owner_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip_owner_id", self.base))
    }

    #[doc = "Get a reference to the value of field `public_dns_name` after provisioning.\n"]
    pub fn public_dns_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.public_dns_name", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `public_ip` after provisioning.\n"]
    pub fn public_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.public_ip", self.base))
    }
}

#[derive(Serialize)]
pub struct DataNetworkInterfaceAttachmentEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    attachment_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    device_index: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_owner_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_card_index: Option<PrimField<f64>>,
}

impl DataNetworkInterfaceAttachmentEl {
    #[doc = "Set the field `attachment_id`.\n"]
    pub fn set_attachment_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.attachment_id = Some(v.into());
        self
    }

    #[doc = "Set the field `device_index`.\n"]
    pub fn set_device_index(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.device_index = Some(v.into());
        self
    }

    #[doc = "Set the field `instance_id`.\n"]
    pub fn set_instance_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance_id = Some(v.into());
        self
    }

    #[doc = "Set the field `instance_owner_id`.\n"]
    pub fn set_instance_owner_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance_owner_id = Some(v.into());
        self
    }

    #[doc = "Set the field `network_card_index`.\n"]
    pub fn set_network_card_index(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.network_card_index = Some(v.into());
        self
    }
}

impl ToListMappable for DataNetworkInterfaceAttachmentEl {
    type O = BlockAssignable<DataNetworkInterfaceAttachmentEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkInterfaceAttachmentEl {}

impl BuildDataNetworkInterfaceAttachmentEl {
    pub fn build(self) -> DataNetworkInterfaceAttachmentEl {
        DataNetworkInterfaceAttachmentEl {
            attachment_id: core::default::Default::default(),
            device_index: core::default::Default::default(),
            instance_id: core::default::Default::default(),
            instance_owner_id: core::default::Default::default(),
            network_card_index: core::default::Default::default(),
        }
    }
}

pub struct DataNetworkInterfaceAttachmentElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataNetworkInterfaceAttachmentElRef {
    fn new(shared: StackShared, base: String) -> DataNetworkInterfaceAttachmentElRef {
        DataNetworkInterfaceAttachmentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkInterfaceAttachmentElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `attachment_id` after provisioning.\n"]
    pub fn attachment_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.attachment_id", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `device_index` after provisioning.\n"]
    pub fn device_index(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.device_index", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_id` after provisioning.\n"]
    pub fn instance_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_id", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_owner_id` after provisioning.\n"]
    pub fn instance_owner_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_owner_id", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `network_card_index` after provisioning.\n"]
    pub fn network_card_index(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network_card_index", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataNetworkInterfaceFilterEl {
    name: PrimField<String>,
    values: SetField<PrimField<String>>,
}

impl DataNetworkInterfaceFilterEl {}

impl ToListMappable for DataNetworkInterfaceFilterEl {
    type O = BlockAssignable<DataNetworkInterfaceFilterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkInterfaceFilterEl {
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub values: SetField<PrimField<String>>,
}

impl BuildDataNetworkInterfaceFilterEl {
    pub fn build(self) -> DataNetworkInterfaceFilterEl {
        DataNetworkInterfaceFilterEl {
            name: self.name,
            values: self.values,
        }
    }
}

pub struct DataNetworkInterfaceFilterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataNetworkInterfaceFilterElRef {
    fn new(shared: StackShared, base: String) -> DataNetworkInterfaceFilterElRef {
        DataNetworkInterfaceFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkInterfaceFilterElRef {
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
pub struct DataNetworkInterfaceTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    read: Option<PrimField<String>>,
}

impl DataNetworkInterfaceTimeoutsEl {
    #[doc = "Set the field `read`.\n"]
    pub fn set_read(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.read = Some(v.into());
        self
    }
}

impl ToListMappable for DataNetworkInterfaceTimeoutsEl {
    type O = BlockAssignable<DataNetworkInterfaceTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkInterfaceTimeoutsEl {}

impl BuildDataNetworkInterfaceTimeoutsEl {
    pub fn build(self) -> DataNetworkInterfaceTimeoutsEl {
        DataNetworkInterfaceTimeoutsEl {
            read: core::default::Default::default(),
        }
    }
}

pub struct DataNetworkInterfaceTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataNetworkInterfaceTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DataNetworkInterfaceTimeoutsElRef {
        DataNetworkInterfaceTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkInterfaceTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataNetworkInterfaceDynamic {
    filter: Option<DynamicBlock<DataNetworkInterfaceFilterEl>>,
}
