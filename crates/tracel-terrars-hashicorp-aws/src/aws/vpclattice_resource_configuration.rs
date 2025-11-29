use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct VpclatticeResourceConfigurationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_association_to_shareable_service_network: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_domain_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    domain_verification_id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port_ranges: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protocol: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_configuration_group_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_gateway_identifier: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_configuration_definition: Option<Vec<VpclatticeResourceConfigurationResourceConfigurationDefinitionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<VpclatticeResourceConfigurationTimeoutsEl>,
    dynamic: VpclatticeResourceConfigurationDynamic,
}

struct VpclatticeResourceConfiguration_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<VpclatticeResourceConfigurationData>,
}

#[derive(Clone)]
pub struct VpclatticeResourceConfiguration(Rc<VpclatticeResourceConfiguration_>);

impl VpclatticeResourceConfiguration {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderAws) -> Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }

    pub fn set_create_before_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.create_before_destroy = v;
        self
    }

    pub fn set_prevent_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.prevent_destroy = v;
        self
    }

    pub fn ignore_changes_to_all(self) -> Self {
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                Some(i) => match i {
                    IgnoreChanges::All(_) => true,
                    IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc = "Set the field `allow_association_to_shareable_service_network`.\n"]
    pub fn set_allow_association_to_shareable_service_network(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().allow_association_to_shareable_service_network = Some(v.into());
        self
    }

    #[doc = "Set the field `custom_domain_name`.\n"]
    pub fn set_custom_domain_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().custom_domain_name = Some(v.into());
        self
    }

    #[doc = "Set the field `domain_verification_id`.\n"]
    pub fn set_domain_verification_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().domain_verification_id = Some(v.into());
        self
    }

    #[doc = "Set the field `port_ranges`.\n"]
    pub fn set_port_ranges(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().port_ranges = Some(v.into());
        self
    }

    #[doc = "Set the field `protocol`.\n"]
    pub fn set_protocol(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().protocol = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `resource_configuration_group_id`.\n"]
    pub fn set_resource_configuration_group_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().resource_configuration_group_id = Some(v.into());
        self
    }

    #[doc = "Set the field `resource_gateway_identifier`.\n"]
    pub fn set_resource_gateway_identifier(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().resource_gateway_identifier = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().type_ = Some(v.into());
        self
    }

    #[doc = "Set the field `resource_configuration_definition`.\n"]
    pub fn set_resource_configuration_definition(
        self,
        v: impl Into<BlockAssignable<VpclatticeResourceConfigurationResourceConfigurationDefinitionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().resource_configuration_definition = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.resource_configuration_definition = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<VpclatticeResourceConfigurationTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `allow_association_to_shareable_service_network` after provisioning.\n"]
    pub fn allow_association_to_shareable_service_network(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_association_to_shareable_service_network", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_domain_name` after provisioning.\n"]
    pub fn custom_domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.custom_domain_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_verification_arn` after provisioning.\n"]
    pub fn domain_verification_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain_verification_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_verification_id` after provisioning.\n"]
    pub fn domain_verification_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain_verification_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_verification_status` after provisioning.\n"]
    pub fn domain_verification_status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain_verification_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `port_ranges` after provisioning.\n"]
    pub fn port_ranges(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.port_ranges", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_configuration_group_id` after provisioning.\n"]
    pub fn resource_configuration_group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_configuration_group_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_gateway_identifier` after provisioning.\n"]
    pub fn resource_gateway_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_gateway_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_configuration_definition` after provisioning.\n"]
    pub fn resource_configuration_definition(
        &self,
    ) -> ListRef<VpclatticeResourceConfigurationResourceConfigurationDefinitionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.resource_configuration_definition", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VpclatticeResourceConfigurationTimeoutsElRef {
        VpclatticeResourceConfigurationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for VpclatticeResourceConfiguration {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for VpclatticeResourceConfiguration { }

impl ToListMappable for VpclatticeResourceConfiguration {
    type O = ListRef<VpclatticeResourceConfigurationRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for VpclatticeResourceConfiguration_ {
    fn extract_resource_type(&self) -> String {
        "aws_vpclattice_resource_configuration".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildVpclatticeResourceConfiguration {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildVpclatticeResourceConfiguration {
    pub fn build(self, stack: &mut Stack) -> VpclatticeResourceConfiguration {
        let out = VpclatticeResourceConfiguration(Rc::new(VpclatticeResourceConfiguration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(VpclatticeResourceConfigurationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                allow_association_to_shareable_service_network: core::default::Default::default(),
                custom_domain_name: core::default::Default::default(),
                domain_verification_id: core::default::Default::default(),
                name: self.name,
                port_ranges: core::default::Default::default(),
                protocol: core::default::Default::default(),
                region: core::default::Default::default(),
                resource_configuration_group_id: core::default::Default::default(),
                resource_gateway_identifier: core::default::Default::default(),
                tags: core::default::Default::default(),
                type_: core::default::Default::default(),
                resource_configuration_definition: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct VpclatticeResourceConfigurationRef {
    shared: StackShared,
    base: String,
}

impl Ref for VpclatticeResourceConfigurationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl VpclatticeResourceConfigurationRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allow_association_to_shareable_service_network` after provisioning.\n"]
    pub fn allow_association_to_shareable_service_network(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_association_to_shareable_service_network", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_domain_name` after provisioning.\n"]
    pub fn custom_domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.custom_domain_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_verification_arn` after provisioning.\n"]
    pub fn domain_verification_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain_verification_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_verification_id` after provisioning.\n"]
    pub fn domain_verification_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain_verification_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_verification_status` after provisioning.\n"]
    pub fn domain_verification_status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain_verification_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `port_ranges` after provisioning.\n"]
    pub fn port_ranges(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.port_ranges", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_configuration_group_id` after provisioning.\n"]
    pub fn resource_configuration_group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_configuration_group_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_gateway_identifier` after provisioning.\n"]
    pub fn resource_gateway_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_gateway_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_configuration_definition` after provisioning.\n"]
    pub fn resource_configuration_definition(
        &self,
    ) -> ListRef<VpclatticeResourceConfigurationResourceConfigurationDefinitionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.resource_configuration_definition", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VpclatticeResourceConfigurationTimeoutsElRef {
        VpclatticeResourceConfigurationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct VpclatticeResourceConfigurationResourceConfigurationDefinitionElArnResourceEl {
    arn: PrimField<String>,
}

impl VpclatticeResourceConfigurationResourceConfigurationDefinitionElArnResourceEl { }

impl ToListMappable for VpclatticeResourceConfigurationResourceConfigurationDefinitionElArnResourceEl {
    type O = BlockAssignable<VpclatticeResourceConfigurationResourceConfigurationDefinitionElArnResourceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVpclatticeResourceConfigurationResourceConfigurationDefinitionElArnResourceEl {
    #[doc = ""]
    pub arn: PrimField<String>,
}

impl BuildVpclatticeResourceConfigurationResourceConfigurationDefinitionElArnResourceEl {
    pub fn build(self) -> VpclatticeResourceConfigurationResourceConfigurationDefinitionElArnResourceEl {
        VpclatticeResourceConfigurationResourceConfigurationDefinitionElArnResourceEl { arn: self.arn }
    }
}

pub struct VpclatticeResourceConfigurationResourceConfigurationDefinitionElArnResourceElRef {
    shared: StackShared,
    base: String,
}

impl Ref for VpclatticeResourceConfigurationResourceConfigurationDefinitionElArnResourceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> VpclatticeResourceConfigurationResourceConfigurationDefinitionElArnResourceElRef {
        VpclatticeResourceConfigurationResourceConfigurationDefinitionElArnResourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VpclatticeResourceConfigurationResourceConfigurationDefinitionElArnResourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }
}

#[derive(Serialize)]
pub struct VpclatticeResourceConfigurationResourceConfigurationDefinitionElDnsResourceEl {
    domain_name: PrimField<String>,
    ip_address_type: PrimField<String>,
}

impl VpclatticeResourceConfigurationResourceConfigurationDefinitionElDnsResourceEl { }

impl ToListMappable for VpclatticeResourceConfigurationResourceConfigurationDefinitionElDnsResourceEl {
    type O = BlockAssignable<VpclatticeResourceConfigurationResourceConfigurationDefinitionElDnsResourceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVpclatticeResourceConfigurationResourceConfigurationDefinitionElDnsResourceEl {
    #[doc = ""]
    pub domain_name: PrimField<String>,
    #[doc = ""]
    pub ip_address_type: PrimField<String>,
}

impl BuildVpclatticeResourceConfigurationResourceConfigurationDefinitionElDnsResourceEl {
    pub fn build(self) -> VpclatticeResourceConfigurationResourceConfigurationDefinitionElDnsResourceEl {
        VpclatticeResourceConfigurationResourceConfigurationDefinitionElDnsResourceEl {
            domain_name: self.domain_name,
            ip_address_type: self.ip_address_type,
        }
    }
}

pub struct VpclatticeResourceConfigurationResourceConfigurationDefinitionElDnsResourceElRef {
    shared: StackShared,
    base: String,
}

impl Ref for VpclatticeResourceConfigurationResourceConfigurationDefinitionElDnsResourceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> VpclatticeResourceConfigurationResourceConfigurationDefinitionElDnsResourceElRef {
        VpclatticeResourceConfigurationResourceConfigurationDefinitionElDnsResourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VpclatticeResourceConfigurationResourceConfigurationDefinitionElDnsResourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_address_type` after provisioning.\n"]
    pub fn ip_address_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip_address_type", self.base))
    }
}

#[derive(Serialize)]
pub struct VpclatticeResourceConfigurationResourceConfigurationDefinitionElIpResourceEl {
    ip_address: PrimField<String>,
}

impl VpclatticeResourceConfigurationResourceConfigurationDefinitionElIpResourceEl { }

impl ToListMappable for VpclatticeResourceConfigurationResourceConfigurationDefinitionElIpResourceEl {
    type O = BlockAssignable<VpclatticeResourceConfigurationResourceConfigurationDefinitionElIpResourceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVpclatticeResourceConfigurationResourceConfigurationDefinitionElIpResourceEl {
    #[doc = ""]
    pub ip_address: PrimField<String>,
}

impl BuildVpclatticeResourceConfigurationResourceConfigurationDefinitionElIpResourceEl {
    pub fn build(self) -> VpclatticeResourceConfigurationResourceConfigurationDefinitionElIpResourceEl {
        VpclatticeResourceConfigurationResourceConfigurationDefinitionElIpResourceEl { ip_address: self.ip_address }
    }
}

pub struct VpclatticeResourceConfigurationResourceConfigurationDefinitionElIpResourceElRef {
    shared: StackShared,
    base: String,
}

impl Ref for VpclatticeResourceConfigurationResourceConfigurationDefinitionElIpResourceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> VpclatticeResourceConfigurationResourceConfigurationDefinitionElIpResourceElRef {
        VpclatticeResourceConfigurationResourceConfigurationDefinitionElIpResourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VpclatticeResourceConfigurationResourceConfigurationDefinitionElIpResourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ip_address` after provisioning.\n"]
    pub fn ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip_address", self.base))
    }
}

#[derive(Serialize, Default)]
struct VpclatticeResourceConfigurationResourceConfigurationDefinitionElDynamic {
    arn_resource: Option<DynamicBlock<VpclatticeResourceConfigurationResourceConfigurationDefinitionElArnResourceEl>>,
    dns_resource: Option<
        DynamicBlock<VpclatticeResourceConfigurationResourceConfigurationDefinitionElDnsResourceEl>,
    >,
    ip_resource: Option<DynamicBlock<VpclatticeResourceConfigurationResourceConfigurationDefinitionElIpResourceEl>>,
}

#[derive(Serialize)]
pub struct VpclatticeResourceConfigurationResourceConfigurationDefinitionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    arn_resource: Option<Vec<VpclatticeResourceConfigurationResourceConfigurationDefinitionElArnResourceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dns_resource: Option<Vec<VpclatticeResourceConfigurationResourceConfigurationDefinitionElDnsResourceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_resource: Option<Vec<VpclatticeResourceConfigurationResourceConfigurationDefinitionElIpResourceEl>>,
    dynamic: VpclatticeResourceConfigurationResourceConfigurationDefinitionElDynamic,
}

impl VpclatticeResourceConfigurationResourceConfigurationDefinitionEl {
    #[doc = "Set the field `arn_resource`.\n"]
    pub fn set_arn_resource(
        mut self,
        v: impl Into<BlockAssignable<VpclatticeResourceConfigurationResourceConfigurationDefinitionElArnResourceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.arn_resource = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.arn_resource = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `dns_resource`.\n"]
    pub fn set_dns_resource(
        mut self,
        v: impl Into<BlockAssignable<VpclatticeResourceConfigurationResourceConfigurationDefinitionElDnsResourceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.dns_resource = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.dns_resource = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ip_resource`.\n"]
    pub fn set_ip_resource(
        mut self,
        v: impl Into<BlockAssignable<VpclatticeResourceConfigurationResourceConfigurationDefinitionElIpResourceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ip_resource = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ip_resource = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for VpclatticeResourceConfigurationResourceConfigurationDefinitionEl {
    type O = BlockAssignable<VpclatticeResourceConfigurationResourceConfigurationDefinitionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVpclatticeResourceConfigurationResourceConfigurationDefinitionEl {}

impl BuildVpclatticeResourceConfigurationResourceConfigurationDefinitionEl {
    pub fn build(self) -> VpclatticeResourceConfigurationResourceConfigurationDefinitionEl {
        VpclatticeResourceConfigurationResourceConfigurationDefinitionEl {
            arn_resource: core::default::Default::default(),
            dns_resource: core::default::Default::default(),
            ip_resource: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct VpclatticeResourceConfigurationResourceConfigurationDefinitionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for VpclatticeResourceConfigurationResourceConfigurationDefinitionElRef {
    fn new(shared: StackShared, base: String) -> VpclatticeResourceConfigurationResourceConfigurationDefinitionElRef {
        VpclatticeResourceConfigurationResourceConfigurationDefinitionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VpclatticeResourceConfigurationResourceConfigurationDefinitionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn_resource` after provisioning.\n"]
    pub fn arn_resource(
        &self,
    ) -> ListRef<VpclatticeResourceConfigurationResourceConfigurationDefinitionElArnResourceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.arn_resource", self.base))
    }

    #[doc = "Get a reference to the value of field `dns_resource` after provisioning.\n"]
    pub fn dns_resource(
        &self,
    ) -> ListRef<VpclatticeResourceConfigurationResourceConfigurationDefinitionElDnsResourceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.dns_resource", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_resource` after provisioning.\n"]
    pub fn ip_resource(
        &self,
    ) -> ListRef<VpclatticeResourceConfigurationResourceConfigurationDefinitionElIpResourceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ip_resource", self.base))
    }
}

#[derive(Serialize)]
pub struct VpclatticeResourceConfigurationTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl VpclatticeResourceConfigurationTimeoutsEl {
    #[doc =
        "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }

    #[doc =
        "Set the field `delete`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }

    #[doc =
        "Set the field `update`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}

impl ToListMappable for VpclatticeResourceConfigurationTimeoutsEl {
    type O = BlockAssignable<VpclatticeResourceConfigurationTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVpclatticeResourceConfigurationTimeoutsEl {}

impl BuildVpclatticeResourceConfigurationTimeoutsEl {
    pub fn build(self) -> VpclatticeResourceConfigurationTimeoutsEl {
        VpclatticeResourceConfigurationTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct VpclatticeResourceConfigurationTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for VpclatticeResourceConfigurationTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> VpclatticeResourceConfigurationTimeoutsElRef {
        VpclatticeResourceConfigurationTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VpclatticeResourceConfigurationTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc =
        "Get a reference to the value of field `delete` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc =
        "Get a reference to the value of field `update` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize, Default)]
struct VpclatticeResourceConfigurationDynamic {
    resource_configuration_definition: Option<
        DynamicBlock<VpclatticeResourceConfigurationResourceConfigurationDefinitionEl>,
    >,
}
