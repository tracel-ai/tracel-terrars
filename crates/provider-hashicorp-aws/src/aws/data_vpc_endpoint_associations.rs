use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataVpcEndpointAssociationsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    vpc_endpoint_id: PrimField<String>,
}
struct DataVpcEndpointAssociations_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataVpcEndpointAssociationsData>,
}
#[derive(Clone)]
pub struct DataVpcEndpointAssociations(Rc<DataVpcEndpointAssociations_>);
impl DataVpcEndpointAssociations {
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
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `associations` after provisioning.\n"]
    pub fn associations(&self) -> ListRef<DataVpcEndpointAssociationsAssociationsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.associations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vpc_endpoint_id` after provisioning.\n"]
    pub fn vpc_endpoint_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vpc_endpoint_id", self.extract_ref()),
        )
    }
}
impl Referable for DataVpcEndpointAssociations {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataVpcEndpointAssociations {}
impl ToListMappable for DataVpcEndpointAssociations {
    type O = ListRef<DataVpcEndpointAssociationsRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataVpcEndpointAssociations_ {
    fn extract_datasource_type(&self) -> String {
        "aws_vpc_endpoint_associations".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataVpcEndpointAssociations {
    pub tf_id: String,
    #[doc = ""]
    pub vpc_endpoint_id: PrimField<String>,
}
impl BuildDataVpcEndpointAssociations {
    pub fn build(self, stack: &mut Stack) -> DataVpcEndpointAssociations {
        let out = DataVpcEndpointAssociations(Rc::new(DataVpcEndpointAssociations_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataVpcEndpointAssociationsData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                region: core::default::Default::default(),
                vpc_endpoint_id: self.vpc_endpoint_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataVpcEndpointAssociationsRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataVpcEndpointAssociationsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataVpcEndpointAssociationsRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `associations` after provisioning.\n"]
    pub fn associations(&self) -> ListRef<DataVpcEndpointAssociationsAssociationsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.associations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vpc_endpoint_id` after provisioning.\n"]
    pub fn vpc_endpoint_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vpc_endpoint_id", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataVpcEndpointAssociationsAssociationsElDnsEntryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dns_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hosted_zone_id: Option<PrimField<String>>,
}
impl DataVpcEndpointAssociationsAssociationsElDnsEntryEl {
    #[doc = "Set the field `dns_name`.\n"]
    pub fn set_dns_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dns_name = Some(v.into());
        self
    }
    #[doc = "Set the field `hosted_zone_id`.\n"]
    pub fn set_hosted_zone_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.hosted_zone_id = Some(v.into());
        self
    }
}
impl ToListMappable for DataVpcEndpointAssociationsAssociationsElDnsEntryEl {
    type O = BlockAssignable<DataVpcEndpointAssociationsAssociationsElDnsEntryEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataVpcEndpointAssociationsAssociationsElDnsEntryEl {}
impl BuildDataVpcEndpointAssociationsAssociationsElDnsEntryEl {
    pub fn build(self) -> DataVpcEndpointAssociationsAssociationsElDnsEntryEl {
        DataVpcEndpointAssociationsAssociationsElDnsEntryEl {
            dns_name: core::default::Default::default(),
            hosted_zone_id: core::default::Default::default(),
        }
    }
}
pub struct DataVpcEndpointAssociationsAssociationsElDnsEntryElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataVpcEndpointAssociationsAssociationsElDnsEntryElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataVpcEndpointAssociationsAssociationsElDnsEntryElRef {
        DataVpcEndpointAssociationsAssociationsElDnsEntryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataVpcEndpointAssociationsAssociationsElDnsEntryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `dns_name` after provisioning.\n"]
    pub fn dns_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dns_name", self.base))
    }
    #[doc = "Get a reference to the value of field `hosted_zone_id` after provisioning.\n"]
    pub fn hosted_zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.hosted_zone_id", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataVpcEndpointAssociationsAssociationsElPrivateDnsEntryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dns_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hosted_zone_id: Option<PrimField<String>>,
}
impl DataVpcEndpointAssociationsAssociationsElPrivateDnsEntryEl {
    #[doc = "Set the field `dns_name`.\n"]
    pub fn set_dns_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dns_name = Some(v.into());
        self
    }
    #[doc = "Set the field `hosted_zone_id`.\n"]
    pub fn set_hosted_zone_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.hosted_zone_id = Some(v.into());
        self
    }
}
impl ToListMappable for DataVpcEndpointAssociationsAssociationsElPrivateDnsEntryEl {
    type O = BlockAssignable<DataVpcEndpointAssociationsAssociationsElPrivateDnsEntryEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataVpcEndpointAssociationsAssociationsElPrivateDnsEntryEl {}
impl BuildDataVpcEndpointAssociationsAssociationsElPrivateDnsEntryEl {
    pub fn build(self) -> DataVpcEndpointAssociationsAssociationsElPrivateDnsEntryEl {
        DataVpcEndpointAssociationsAssociationsElPrivateDnsEntryEl {
            dns_name: core::default::Default::default(),
            hosted_zone_id: core::default::Default::default(),
        }
    }
}
pub struct DataVpcEndpointAssociationsAssociationsElPrivateDnsEntryElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataVpcEndpointAssociationsAssociationsElPrivateDnsEntryElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataVpcEndpointAssociationsAssociationsElPrivateDnsEntryElRef {
        DataVpcEndpointAssociationsAssociationsElPrivateDnsEntryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataVpcEndpointAssociationsAssociationsElPrivateDnsEntryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `dns_name` after provisioning.\n"]
    pub fn dns_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dns_name", self.base))
    }
    #[doc = "Get a reference to the value of field `hosted_zone_id` after provisioning.\n"]
    pub fn hosted_zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.hosted_zone_id", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataVpcEndpointAssociationsAssociationsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    associated_resource_accessibility: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    associated_resource_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dns_entry: Option<ListField<DataVpcEndpointAssociationsAssociationsElDnsEntryEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    private_dns_entry:
        Option<ListField<DataVpcEndpointAssociationsAssociationsElPrivateDnsEntryEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_configuration_group_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_network_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_network_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
}
impl DataVpcEndpointAssociationsAssociationsEl {
    #[doc = "Set the field `associated_resource_accessibility`.\n"]
    pub fn set_associated_resource_accessibility(
        mut self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.associated_resource_accessibility = Some(v.into());
        self
    }
    #[doc = "Set the field `associated_resource_arn`.\n"]
    pub fn set_associated_resource_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.associated_resource_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `dns_entry`.\n"]
    pub fn set_dns_entry(
        mut self,
        v: impl Into<ListField<DataVpcEndpointAssociationsAssociationsElDnsEntryEl>>,
    ) -> Self {
        self.dns_entry = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }
    #[doc = "Set the field `private_dns_entry`.\n"]
    pub fn set_private_dns_entry(
        mut self,
        v: impl Into<ListField<DataVpcEndpointAssociationsAssociationsElPrivateDnsEntryEl>>,
    ) -> Self {
        self.private_dns_entry = Some(v.into());
        self
    }
    #[doc = "Set the field `resource_configuration_group_arn`.\n"]
    pub fn set_resource_configuration_group_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.resource_configuration_group_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `service_network_arn`.\n"]
    pub fn set_service_network_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_network_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `service_network_name`.\n"]
    pub fn set_service_network_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_network_name = Some(v.into());
        self
    }
    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.tags = Some(v.into());
        self
    }
}
impl ToListMappable for DataVpcEndpointAssociationsAssociationsEl {
    type O = BlockAssignable<DataVpcEndpointAssociationsAssociationsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataVpcEndpointAssociationsAssociationsEl {}
impl BuildDataVpcEndpointAssociationsAssociationsEl {
    pub fn build(self) -> DataVpcEndpointAssociationsAssociationsEl {
        DataVpcEndpointAssociationsAssociationsEl {
            associated_resource_accessibility: core::default::Default::default(),
            associated_resource_arn: core::default::Default::default(),
            dns_entry: core::default::Default::default(),
            id: core::default::Default::default(),
            private_dns_entry: core::default::Default::default(),
            resource_configuration_group_arn: core::default::Default::default(),
            service_network_arn: core::default::Default::default(),
            service_network_name: core::default::Default::default(),
            tags: core::default::Default::default(),
        }
    }
}
pub struct DataVpcEndpointAssociationsAssociationsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataVpcEndpointAssociationsAssociationsElRef {
    fn new(shared: StackShared, base: String) -> DataVpcEndpointAssociationsAssociationsElRef {
        DataVpcEndpointAssociationsAssociationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataVpcEndpointAssociationsAssociationsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `associated_resource_accessibility` after provisioning.\n"]
    pub fn associated_resource_accessibility(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.associated_resource_accessibility", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `associated_resource_arn` after provisioning.\n"]
    pub fn associated_resource_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.associated_resource_arn", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `dns_entry` after provisioning.\n"]
    pub fn dns_entry(&self) -> ListRef<DataVpcEndpointAssociationsAssociationsElDnsEntryElRef> {
        ListRef::new(self.shared().clone(), format!("{}.dns_entry", self.base))
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
    #[doc = "Get a reference to the value of field `private_dns_entry` after provisioning.\n"]
    pub fn private_dns_entry(
        &self,
    ) -> ListRef<DataVpcEndpointAssociationsAssociationsElPrivateDnsEntryElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.private_dns_entry", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `resource_configuration_group_arn` after provisioning.\n"]
    pub fn resource_configuration_group_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_configuration_group_arn", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `service_network_arn` after provisioning.\n"]
    pub fn service_network_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_network_arn", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `service_network_name` after provisioning.\n"]
    pub fn service_network_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_network_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}
