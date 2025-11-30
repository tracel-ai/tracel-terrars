use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct DataNetworkfirewallFirewallData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
}

struct DataNetworkfirewallFirewall_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataNetworkfirewallFirewallData>,
}

#[derive(Clone)]
pub struct DataNetworkfirewallFirewall(Rc<DataNetworkfirewallFirewall_>);

impl DataNetworkfirewallFirewall {
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

    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().arn = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name = Some(v.into());
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

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `availability_zone_change_protection` after provisioning.\n"]
    pub fn availability_zone_change_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.availability_zone_change_protection", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `availability_zone_mapping` after provisioning.\n"]
    pub fn availability_zone_mapping(
        &self,
    ) -> SetRef<DataNetworkfirewallFirewallAvailabilityZoneMappingElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.availability_zone_mapping", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `delete_protection` after provisioning.\n"]
    pub fn delete_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delete_protection", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `enabled_analysis_types` after provisioning.\n"]
    pub fn enabled_analysis_types(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.enabled_analysis_types", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `encryption_configuration` after provisioning.\n"]
    pub fn encryption_configuration(
        &self,
    ) -> SetRef<DataNetworkfirewallFirewallEncryptionConfigurationElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.encryption_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `firewall_policy_arn` after provisioning.\n"]
    pub fn firewall_policy_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.firewall_policy_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `firewall_policy_change_protection` after provisioning.\n"]
    pub fn firewall_policy_change_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.firewall_policy_change_protection", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `firewall_status` after provisioning.\n"]
    pub fn firewall_status(&self) -> ListRef<DataNetworkfirewallFirewallFirewallStatusElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.firewall_status", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `subnet_change_protection` after provisioning.\n"]
    pub fn subnet_change_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subnet_change_protection", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `subnet_mapping` after provisioning.\n"]
    pub fn subnet_mapping(&self) -> SetRef<DataNetworkfirewallFirewallSubnetMappingElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.subnet_mapping", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `transit_gateway_id` after provisioning.\n"]
    pub fn transit_gateway_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.transit_gateway_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `transit_gateway_owner_account_id` after provisioning.\n"]
    pub fn transit_gateway_owner_account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.transit_gateway_owner_account_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `update_token` after provisioning.\n"]
    pub fn update_token(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_token", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vpc_id", self.extract_ref()),
        )
    }
}

impl Referable for DataNetworkfirewallFirewall {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Datasource for DataNetworkfirewallFirewall {}

impl ToListMappable for DataNetworkfirewallFirewall {
    type O = ListRef<DataNetworkfirewallFirewallRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataNetworkfirewallFirewall_ {
    fn extract_datasource_type(&self) -> String {
        "aws_networkfirewall_firewall".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataNetworkfirewallFirewall {
    pub tf_id: String,
}

impl BuildDataNetworkfirewallFirewall {
    pub fn build(self, stack: &mut Stack) -> DataNetworkfirewallFirewall {
        let out = DataNetworkfirewallFirewall(Rc::new(DataNetworkfirewallFirewall_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataNetworkfirewallFirewallData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                arn: core::default::Default::default(),
                id: core::default::Default::default(),
                name: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataNetworkfirewallFirewallRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataNetworkfirewallFirewallRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl DataNetworkfirewallFirewallRef {
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

    #[doc = "Get a reference to the value of field `availability_zone_change_protection` after provisioning.\n"]
    pub fn availability_zone_change_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.availability_zone_change_protection", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `availability_zone_mapping` after provisioning.\n"]
    pub fn availability_zone_mapping(
        &self,
    ) -> SetRef<DataNetworkfirewallFirewallAvailabilityZoneMappingElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.availability_zone_mapping", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `delete_protection` after provisioning.\n"]
    pub fn delete_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delete_protection", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `enabled_analysis_types` after provisioning.\n"]
    pub fn enabled_analysis_types(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.enabled_analysis_types", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `encryption_configuration` after provisioning.\n"]
    pub fn encryption_configuration(
        &self,
    ) -> SetRef<DataNetworkfirewallFirewallEncryptionConfigurationElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.encryption_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `firewall_policy_arn` after provisioning.\n"]
    pub fn firewall_policy_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.firewall_policy_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `firewall_policy_change_protection` after provisioning.\n"]
    pub fn firewall_policy_change_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.firewall_policy_change_protection", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `firewall_status` after provisioning.\n"]
    pub fn firewall_status(&self) -> ListRef<DataNetworkfirewallFirewallFirewallStatusElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.firewall_status", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `subnet_change_protection` after provisioning.\n"]
    pub fn subnet_change_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subnet_change_protection", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `subnet_mapping` after provisioning.\n"]
    pub fn subnet_mapping(&self) -> SetRef<DataNetworkfirewallFirewallSubnetMappingElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.subnet_mapping", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `transit_gateway_id` after provisioning.\n"]
    pub fn transit_gateway_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.transit_gateway_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `transit_gateway_owner_account_id` after provisioning.\n"]
    pub fn transit_gateway_owner_account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.transit_gateway_owner_account_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `update_token` after provisioning.\n"]
    pub fn update_token(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_token", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vpc_id", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DataNetworkfirewallFirewallAvailabilityZoneMappingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    availability_zone_id: Option<PrimField<String>>,
}

impl DataNetworkfirewallFirewallAvailabilityZoneMappingEl {
    #[doc = "Set the field `availability_zone_id`.\n"]
    pub fn set_availability_zone_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.availability_zone_id = Some(v.into());
        self
    }
}

impl ToListMappable for DataNetworkfirewallFirewallAvailabilityZoneMappingEl {
    type O = BlockAssignable<DataNetworkfirewallFirewallAvailabilityZoneMappingEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkfirewallFirewallAvailabilityZoneMappingEl {}

impl BuildDataNetworkfirewallFirewallAvailabilityZoneMappingEl {
    pub fn build(self) -> DataNetworkfirewallFirewallAvailabilityZoneMappingEl {
        DataNetworkfirewallFirewallAvailabilityZoneMappingEl {
            availability_zone_id: core::default::Default::default(),
        }
    }
}

pub struct DataNetworkfirewallFirewallAvailabilityZoneMappingElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataNetworkfirewallFirewallAvailabilityZoneMappingElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataNetworkfirewallFirewallAvailabilityZoneMappingElRef {
        DataNetworkfirewallFirewallAvailabilityZoneMappingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkfirewallFirewallAvailabilityZoneMappingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `availability_zone_id` after provisioning.\n"]
    pub fn availability_zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.availability_zone_id", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataNetworkfirewallFirewallEncryptionConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key_id: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}

impl DataNetworkfirewallFirewallEncryptionConfigurationEl {
    #[doc = "Set the field `key_id`.\n"]
    pub fn set_key_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key_id = Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}

impl ToListMappable for DataNetworkfirewallFirewallEncryptionConfigurationEl {
    type O = BlockAssignable<DataNetworkfirewallFirewallEncryptionConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkfirewallFirewallEncryptionConfigurationEl {}

impl BuildDataNetworkfirewallFirewallEncryptionConfigurationEl {
    pub fn build(self) -> DataNetworkfirewallFirewallEncryptionConfigurationEl {
        DataNetworkfirewallFirewallEncryptionConfigurationEl {
            key_id: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct DataNetworkfirewallFirewallEncryptionConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataNetworkfirewallFirewallEncryptionConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataNetworkfirewallFirewallEncryptionConfigurationElRef {
        DataNetworkfirewallFirewallEncryptionConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkfirewallFirewallEncryptionConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key_id` after provisioning.\n"]
    pub fn key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_id", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryElCidrsElIpSetReferencesEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    resolved_cidr_count: Option<PrimField<f64>>,
}

impl DataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryElCidrsElIpSetReferencesEl {
    #[doc = "Set the field `resolved_cidr_count`.\n"]
    pub fn set_resolved_cidr_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.resolved_cidr_count = Some(v.into());
        self
    }
}

impl ToListMappable
    for DataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryElCidrsElIpSetReferencesEl
{
    type O = BlockAssignable<
        DataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryElCidrsElIpSetReferencesEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryElCidrsElIpSetReferencesEl
{}

impl
    BuildDataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryElCidrsElIpSetReferencesEl
{
    pub fn build(
        self,
    ) -> DataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryElCidrsElIpSetReferencesEl
    {
        DataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryElCidrsElIpSetReferencesEl {
            resolved_cidr_count: core::default::Default::default(),
        }
    }
}

pub struct DataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryElCidrsElIpSetReferencesElRef
{
    shared: StackShared,
    base: String,
}

impl Ref
    for DataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryElCidrsElIpSetReferencesElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryElCidrsElIpSetReferencesElRef
    {
        DataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryElCidrsElIpSetReferencesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryElCidrsElIpSetReferencesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `resolved_cidr_count` after provisioning.\n"]
    pub fn resolved_cidr_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resolved_cidr_count", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryElCidrsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    available_cidr_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_set_references: Option<
        SetField<DataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryElCidrsElIpSetReferencesEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    utilized_cidr_count: Option<PrimField<f64>>,
}

impl DataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryElCidrsEl {
    #[doc = "Set the field `available_cidr_count`.\n"]
    pub fn set_available_cidr_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.available_cidr_count = Some(v.into());
        self
    }

    #[doc = "Set the field `ip_set_references`.\n"]
    pub fn set_ip_set_references(
        mut self,
        v:
            impl

                    Into<
                        SetField<
                            DataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryElCidrsElIpSetReferencesEl,
                        >,
                    >,
    ) -> Self {
        self.ip_set_references = Some(v.into());
        self
    }

    #[doc = "Set the field `utilized_cidr_count`.\n"]
    pub fn set_utilized_cidr_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.utilized_cidr_count = Some(v.into());
        self
    }
}

impl ToListMappable for DataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryElCidrsEl {
    type O =
        BlockAssignable<DataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryElCidrsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryElCidrsEl {}

impl BuildDataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryElCidrsEl {
    pub fn build(self) -> DataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryElCidrsEl {
        DataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryElCidrsEl {
            available_cidr_count: core::default::Default::default(),
            ip_set_references: core::default::Default::default(),
            utilized_cidr_count: core::default::Default::default(),
        }
    }
}

pub struct DataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryElCidrsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryElCidrsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryElCidrsElRef {
        DataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryElCidrsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryElCidrsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `available_cidr_count` after provisioning.\n"]
    pub fn available_cidr_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.available_cidr_count", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `ip_set_references` after provisioning.\n"]
    pub fn ip_set_references(
        &self,
    ) -> SetRef<DataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryElCidrsElIpSetReferencesElRef>{
        SetRef::new(
            self.shared().clone(),
            format!("{}.ip_set_references", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `utilized_cidr_count` after provisioning.\n"]
    pub fn utilized_cidr_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.utilized_cidr_count", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cidrs:
        Option<SetField<DataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryElCidrsEl>>,
}

impl DataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryEl {
    #[doc = "Set the field `cidrs`.\n"]
    pub fn set_cidrs(
        mut self,
        v: impl Into<SetField<DataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryElCidrsEl>>,
    ) -> Self {
        self.cidrs = Some(v.into());
        self
    }
}

impl ToListMappable for DataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryEl {
    type O = BlockAssignable<DataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryEl {}

impl BuildDataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryEl {
    pub fn build(self) -> DataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryEl {
        DataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryEl {
            cidrs: core::default::Default::default(),
        }
    }
}

pub struct DataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryElRef {
        DataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cidrs` after provisioning.\n"]
    pub fn cidrs(
        &self,
    ) -> SetRef<DataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryElCidrsElRef> {
        SetRef::new(self.shared().clone(), format!("{}.cidrs", self.base))
    }
}

#[derive(Serialize)]
pub struct DataNetworkfirewallFirewallFirewallStatusElSyncStatesElAttachmentEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    endpoint_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status_message: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnet_id: Option<PrimField<String>>,
}

impl DataNetworkfirewallFirewallFirewallStatusElSyncStatesElAttachmentEl {
    #[doc = "Set the field `endpoint_id`.\n"]
    pub fn set_endpoint_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.endpoint_id = Some(v.into());
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

    #[doc = "Set the field `subnet_id`.\n"]
    pub fn set_subnet_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.subnet_id = Some(v.into());
        self
    }
}

impl ToListMappable for DataNetworkfirewallFirewallFirewallStatusElSyncStatesElAttachmentEl {
    type O = BlockAssignable<DataNetworkfirewallFirewallFirewallStatusElSyncStatesElAttachmentEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkfirewallFirewallFirewallStatusElSyncStatesElAttachmentEl {}

impl BuildDataNetworkfirewallFirewallFirewallStatusElSyncStatesElAttachmentEl {
    pub fn build(self) -> DataNetworkfirewallFirewallFirewallStatusElSyncStatesElAttachmentEl {
        DataNetworkfirewallFirewallFirewallStatusElSyncStatesElAttachmentEl {
            endpoint_id: core::default::Default::default(),
            status: core::default::Default::default(),
            status_message: core::default::Default::default(),
            subnet_id: core::default::Default::default(),
        }
    }
}

pub struct DataNetworkfirewallFirewallFirewallStatusElSyncStatesElAttachmentElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataNetworkfirewallFirewallFirewallStatusElSyncStatesElAttachmentElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataNetworkfirewallFirewallFirewallStatusElSyncStatesElAttachmentElRef {
        DataNetworkfirewallFirewallFirewallStatusElSyncStatesElAttachmentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkfirewallFirewallFirewallStatusElSyncStatesElAttachmentElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `endpoint_id` after provisioning.\n"]
    pub fn endpoint_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint_id", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc = "Get a reference to the value of field `status_message` after provisioning.\n"]
    pub fn status_message(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status_message", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `subnet_id` after provisioning.\n"]
    pub fn subnet_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subnet_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataNetworkfirewallFirewallFirewallStatusElSyncStatesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    attachment:
        Option<ListField<DataNetworkfirewallFirewallFirewallStatusElSyncStatesElAttachmentEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    availability_zone: Option<PrimField<String>>,
}

impl DataNetworkfirewallFirewallFirewallStatusElSyncStatesEl {
    #[doc = "Set the field `attachment`.\n"]
    pub fn set_attachment(
        mut self,
        v: impl Into<ListField<DataNetworkfirewallFirewallFirewallStatusElSyncStatesElAttachmentEl>>,
    ) -> Self {
        self.attachment = Some(v.into());
        self
    }

    #[doc = "Set the field `availability_zone`.\n"]
    pub fn set_availability_zone(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.availability_zone = Some(v.into());
        self
    }
}

impl ToListMappable for DataNetworkfirewallFirewallFirewallStatusElSyncStatesEl {
    type O = BlockAssignable<DataNetworkfirewallFirewallFirewallStatusElSyncStatesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkfirewallFirewallFirewallStatusElSyncStatesEl {}

impl BuildDataNetworkfirewallFirewallFirewallStatusElSyncStatesEl {
    pub fn build(self) -> DataNetworkfirewallFirewallFirewallStatusElSyncStatesEl {
        DataNetworkfirewallFirewallFirewallStatusElSyncStatesEl {
            attachment: core::default::Default::default(),
            availability_zone: core::default::Default::default(),
        }
    }
}

pub struct DataNetworkfirewallFirewallFirewallStatusElSyncStatesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataNetworkfirewallFirewallFirewallStatusElSyncStatesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataNetworkfirewallFirewallFirewallStatusElSyncStatesElRef {
        DataNetworkfirewallFirewallFirewallStatusElSyncStatesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkfirewallFirewallFirewallStatusElSyncStatesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `attachment` after provisioning.\n"]
    pub fn attachment(
        &self,
    ) -> ListRef<DataNetworkfirewallFirewallFirewallStatusElSyncStatesElAttachmentElRef> {
        ListRef::new(self.shared().clone(), format!("{}.attachment", self.base))
    }

    #[doc = "Get a reference to the value of field `availability_zone` after provisioning.\n"]
    pub fn availability_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.availability_zone", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataNetworkfirewallFirewallFirewallStatusElTransitGatewayAttachmentSyncStatesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    attachment_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status_message: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transit_gateway_attachment_status: Option<PrimField<String>>,
}

impl DataNetworkfirewallFirewallFirewallStatusElTransitGatewayAttachmentSyncStatesEl {
    #[doc = "Set the field `attachment_id`.\n"]
    pub fn set_attachment_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.attachment_id = Some(v.into());
        self
    }

    #[doc = "Set the field `status_message`.\n"]
    pub fn set_status_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status_message = Some(v.into());
        self
    }

    #[doc = "Set the field `transit_gateway_attachment_status`.\n"]
    pub fn set_transit_gateway_attachment_status(
        mut self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.transit_gateway_attachment_status = Some(v.into());
        self
    }
}

impl ToListMappable
    for DataNetworkfirewallFirewallFirewallStatusElTransitGatewayAttachmentSyncStatesEl
{
    type O = BlockAssignable<
        DataNetworkfirewallFirewallFirewallStatusElTransitGatewayAttachmentSyncStatesEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkfirewallFirewallFirewallStatusElTransitGatewayAttachmentSyncStatesEl {}

impl BuildDataNetworkfirewallFirewallFirewallStatusElTransitGatewayAttachmentSyncStatesEl {
    pub fn build(
        self,
    ) -> DataNetworkfirewallFirewallFirewallStatusElTransitGatewayAttachmentSyncStatesEl {
        DataNetworkfirewallFirewallFirewallStatusElTransitGatewayAttachmentSyncStatesEl {
            attachment_id: core::default::Default::default(),
            status_message: core::default::Default::default(),
            transit_gateway_attachment_status: core::default::Default::default(),
        }
    }
}

pub struct DataNetworkfirewallFirewallFirewallStatusElTransitGatewayAttachmentSyncStatesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataNetworkfirewallFirewallFirewallStatusElTransitGatewayAttachmentSyncStatesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataNetworkfirewallFirewallFirewallStatusElTransitGatewayAttachmentSyncStatesElRef {
        DataNetworkfirewallFirewallFirewallStatusElTransitGatewayAttachmentSyncStatesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkfirewallFirewallFirewallStatusElTransitGatewayAttachmentSyncStatesElRef {
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

    #[doc = "Get a reference to the value of field `status_message` after provisioning.\n"]
    pub fn status_message(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status_message", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `transit_gateway_attachment_status` after provisioning.\n"]
    pub fn transit_gateway_attachment_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.transit_gateway_attachment_status", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataNetworkfirewallFirewallFirewallStatusEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    capacity_usage_summary:
        Option<SetField<DataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    configuration_sync_state_summary: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sync_states: Option<SetField<DataNetworkfirewallFirewallFirewallStatusElSyncStatesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transit_gateway_attachment_sync_states: Option<
        ListField<DataNetworkfirewallFirewallFirewallStatusElTransitGatewayAttachmentSyncStatesEl>,
    >,
}

impl DataNetworkfirewallFirewallFirewallStatusEl {
    #[doc = "Set the field `capacity_usage_summary`.\n"]
    pub fn set_capacity_usage_summary(
        mut self,
        v: impl Into<SetField<DataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryEl>>,
    ) -> Self {
        self.capacity_usage_summary = Some(v.into());
        self
    }

    #[doc = "Set the field `configuration_sync_state_summary`.\n"]
    pub fn set_configuration_sync_state_summary(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.configuration_sync_state_summary = Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status = Some(v.into());
        self
    }

    #[doc = "Set the field `sync_states`.\n"]
    pub fn set_sync_states(
        mut self,
        v: impl Into<SetField<DataNetworkfirewallFirewallFirewallStatusElSyncStatesEl>>,
    ) -> Self {
        self.sync_states = Some(v.into());
        self
    }

    #[doc = "Set the field `transit_gateway_attachment_sync_states`.\n"]
    pub fn set_transit_gateway_attachment_sync_states(
        mut self,
        v: impl Into<
            ListField<
                DataNetworkfirewallFirewallFirewallStatusElTransitGatewayAttachmentSyncStatesEl,
            >,
        >,
    ) -> Self {
        self.transit_gateway_attachment_sync_states = Some(v.into());
        self
    }
}

impl ToListMappable for DataNetworkfirewallFirewallFirewallStatusEl {
    type O = BlockAssignable<DataNetworkfirewallFirewallFirewallStatusEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkfirewallFirewallFirewallStatusEl {}

impl BuildDataNetworkfirewallFirewallFirewallStatusEl {
    pub fn build(self) -> DataNetworkfirewallFirewallFirewallStatusEl {
        DataNetworkfirewallFirewallFirewallStatusEl {
            capacity_usage_summary: core::default::Default::default(),
            configuration_sync_state_summary: core::default::Default::default(),
            status: core::default::Default::default(),
            sync_states: core::default::Default::default(),
            transit_gateway_attachment_sync_states: core::default::Default::default(),
        }
    }
}

pub struct DataNetworkfirewallFirewallFirewallStatusElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataNetworkfirewallFirewallFirewallStatusElRef {
    fn new(shared: StackShared, base: String) -> DataNetworkfirewallFirewallFirewallStatusElRef {
        DataNetworkfirewallFirewallFirewallStatusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkfirewallFirewallFirewallStatusElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `capacity_usage_summary` after provisioning.\n"]
    pub fn capacity_usage_summary(
        &self,
    ) -> SetRef<DataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.capacity_usage_summary", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `configuration_sync_state_summary` after provisioning.\n"]
    pub fn configuration_sync_state_summary(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.configuration_sync_state_summary", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc = "Get a reference to the value of field `sync_states` after provisioning.\n"]
    pub fn sync_states(
        &self,
    ) -> SetRef<DataNetworkfirewallFirewallFirewallStatusElSyncStatesElRef> {
        SetRef::new(self.shared().clone(), format!("{}.sync_states", self.base))
    }

    #[doc = "Get a reference to the value of field `transit_gateway_attachment_sync_states` after provisioning.\n"]
    pub fn transit_gateway_attachment_sync_states(
        &self,
    ) -> ListRef<DataNetworkfirewallFirewallFirewallStatusElTransitGatewayAttachmentSyncStatesElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.transit_gateway_attachment_sync_states", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataNetworkfirewallFirewallSubnetMappingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    subnet_id: Option<PrimField<String>>,
}

impl DataNetworkfirewallFirewallSubnetMappingEl {
    #[doc = "Set the field `subnet_id`.\n"]
    pub fn set_subnet_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.subnet_id = Some(v.into());
        self
    }
}

impl ToListMappable for DataNetworkfirewallFirewallSubnetMappingEl {
    type O = BlockAssignable<DataNetworkfirewallFirewallSubnetMappingEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkfirewallFirewallSubnetMappingEl {}

impl BuildDataNetworkfirewallFirewallSubnetMappingEl {
    pub fn build(self) -> DataNetworkfirewallFirewallSubnetMappingEl {
        DataNetworkfirewallFirewallSubnetMappingEl {
            subnet_id: core::default::Default::default(),
        }
    }
}

pub struct DataNetworkfirewallFirewallSubnetMappingElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataNetworkfirewallFirewallSubnetMappingElRef {
    fn new(shared: StackShared, base: String) -> DataNetworkfirewallFirewallSubnetMappingElRef {
        DataNetworkfirewallFirewallSubnetMappingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkfirewallFirewallSubnetMappingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `subnet_id` after provisioning.\n"]
    pub fn subnet_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subnet_id", self.base))
    }
}
