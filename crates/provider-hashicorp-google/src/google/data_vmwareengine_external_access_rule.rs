use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataVmwareengineExternalAccessRuleData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    parent: PrimField<String>,
}
struct DataVmwareengineExternalAccessRule_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataVmwareengineExternalAccessRuleData>,
}
#[derive(Clone)]
pub struct DataVmwareengineExternalAccessRule(Rc<DataVmwareengineExternalAccessRule_>);
impl DataVmwareengineExternalAccessRule {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(&self, provider: &ProviderGoogle) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `action` after provisioning.\nThe action that the external access rule performs. Possible values: [\"ALLOW\", \"DENY\"]"]
    pub fn action(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.action", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nCreation time of this resource.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and\nup to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nUser-provided description for the external access rule."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `destination_ip_ranges` after provisioning.\nIf destination ranges are specified, the external access rule applies only to\ntraffic that has a destination IP address in these ranges."]
    pub fn destination_ip_ranges(
        &self,
    ) -> ListRef<DataVmwareengineExternalAccessRuleDestinationIpRangesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.destination_ip_ranges", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `destination_ports` after provisioning.\nA list of destination ports to which the external access rule applies."]
    pub fn destination_ports(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.destination_ports", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `ip_protocol` after provisioning.\nThe IP protocol to which the external access rule applies."]
    pub fn ip_protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ip_protocol", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe ID of the external access rule."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\nThe resource name of the network policy.\nResource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names.\nFor example: projects/my-project/locations/us-west1-a/networkPolicies/my-policy"]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `priority` after provisioning.\nExternal access rule priority, which determines the external access rule to use when multiple rules apply."]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.priority", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_ip_ranges` after provisioning.\nIf source ranges are specified, the external access rule applies only to\ntraffic that has a source IP address in these ranges."]
    pub fn source_ip_ranges(
        &self,
    ) -> ListRef<DataVmwareengineExternalAccessRuleSourceIpRangesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source_ip_ranges", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_ports` after provisioning.\nA list of source ports to which the external access rule applies."]
    pub fn source_ports(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source_ports", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nState of the Cluster."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nSystem-generated unique identifier for the resource."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nLast updated time of this resource.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine\nfractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
}
impl Referable for DataVmwareengineExternalAccessRule {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataVmwareengineExternalAccessRule {}
impl ToListMappable for DataVmwareengineExternalAccessRule {
    type O = ListRef<DataVmwareengineExternalAccessRuleRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataVmwareengineExternalAccessRule_ {
    fn extract_datasource_type(&self) -> String {
        "google_vmwareengine_external_access_rule".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataVmwareengineExternalAccessRule {
    pub tf_id: String,
    #[doc = "The ID of the external access rule."]
    pub name: PrimField<String>,
    #[doc = "The resource name of the network policy.\nResource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names.\nFor example: projects/my-project/locations/us-west1-a/networkPolicies/my-policy"]
    pub parent: PrimField<String>,
}
impl BuildDataVmwareengineExternalAccessRule {
    pub fn build(self, stack: &mut Stack) -> DataVmwareengineExternalAccessRule {
        let out =
            DataVmwareengineExternalAccessRule(Rc::new(DataVmwareengineExternalAccessRule_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(DataVmwareengineExternalAccessRuleData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    for_each: None,
                    id: core::default::Default::default(),
                    name: self.name,
                    parent: self.parent,
                }),
            }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataVmwareengineExternalAccessRuleRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataVmwareengineExternalAccessRuleRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataVmwareengineExternalAccessRuleRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `action` after provisioning.\nThe action that the external access rule performs. Possible values: [\"ALLOW\", \"DENY\"]"]
    pub fn action(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.action", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nCreation time of this resource.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and\nup to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nUser-provided description for the external access rule."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `destination_ip_ranges` after provisioning.\nIf destination ranges are specified, the external access rule applies only to\ntraffic that has a destination IP address in these ranges."]
    pub fn destination_ip_ranges(
        &self,
    ) -> ListRef<DataVmwareengineExternalAccessRuleDestinationIpRangesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.destination_ip_ranges", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `destination_ports` after provisioning.\nA list of destination ports to which the external access rule applies."]
    pub fn destination_ports(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.destination_ports", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `ip_protocol` after provisioning.\nThe IP protocol to which the external access rule applies."]
    pub fn ip_protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ip_protocol", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe ID of the external access rule."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\nThe resource name of the network policy.\nResource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names.\nFor example: projects/my-project/locations/us-west1-a/networkPolicies/my-policy"]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `priority` after provisioning.\nExternal access rule priority, which determines the external access rule to use when multiple rules apply."]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.priority", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_ip_ranges` after provisioning.\nIf source ranges are specified, the external access rule applies only to\ntraffic that has a source IP address in these ranges."]
    pub fn source_ip_ranges(
        &self,
    ) -> ListRef<DataVmwareengineExternalAccessRuleSourceIpRangesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source_ip_ranges", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_ports` after provisioning.\nA list of source ports to which the external access rule applies."]
    pub fn source_ports(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source_ports", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nState of the Cluster."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nSystem-generated unique identifier for the resource."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nLast updated time of this resource.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine\nfractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataVmwareengineExternalAccessRuleDestinationIpRangesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    external_address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_address_range: Option<PrimField<String>>,
}
impl DataVmwareengineExternalAccessRuleDestinationIpRangesEl {
    #[doc = "Set the field `external_address`.\n"]
    pub fn set_external_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.external_address = Some(v.into());
        self
    }
    #[doc = "Set the field `ip_address_range`.\n"]
    pub fn set_ip_address_range(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ip_address_range = Some(v.into());
        self
    }
}
impl ToListMappable for DataVmwareengineExternalAccessRuleDestinationIpRangesEl {
    type O = BlockAssignable<DataVmwareengineExternalAccessRuleDestinationIpRangesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataVmwareengineExternalAccessRuleDestinationIpRangesEl {}
impl BuildDataVmwareengineExternalAccessRuleDestinationIpRangesEl {
    pub fn build(self) -> DataVmwareengineExternalAccessRuleDestinationIpRangesEl {
        DataVmwareengineExternalAccessRuleDestinationIpRangesEl {
            external_address: core::default::Default::default(),
            ip_address_range: core::default::Default::default(),
        }
    }
}
pub struct DataVmwareengineExternalAccessRuleDestinationIpRangesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataVmwareengineExternalAccessRuleDestinationIpRangesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataVmwareengineExternalAccessRuleDestinationIpRangesElRef {
        DataVmwareengineExternalAccessRuleDestinationIpRangesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataVmwareengineExternalAccessRuleDestinationIpRangesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `external_address` after provisioning.\n"]
    pub fn external_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.external_address", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ip_address_range` after provisioning.\n"]
    pub fn ip_address_range(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ip_address_range", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataVmwareengineExternalAccessRuleSourceIpRangesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_address_range: Option<PrimField<String>>,
}
impl DataVmwareengineExternalAccessRuleSourceIpRangesEl {
    #[doc = "Set the field `ip_address`.\n"]
    pub fn set_ip_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ip_address = Some(v.into());
        self
    }
    #[doc = "Set the field `ip_address_range`.\n"]
    pub fn set_ip_address_range(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ip_address_range = Some(v.into());
        self
    }
}
impl ToListMappable for DataVmwareengineExternalAccessRuleSourceIpRangesEl {
    type O = BlockAssignable<DataVmwareengineExternalAccessRuleSourceIpRangesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataVmwareengineExternalAccessRuleSourceIpRangesEl {}
impl BuildDataVmwareengineExternalAccessRuleSourceIpRangesEl {
    pub fn build(self) -> DataVmwareengineExternalAccessRuleSourceIpRangesEl {
        DataVmwareengineExternalAccessRuleSourceIpRangesEl {
            ip_address: core::default::Default::default(),
            ip_address_range: core::default::Default::default(),
        }
    }
}
pub struct DataVmwareengineExternalAccessRuleSourceIpRangesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataVmwareengineExternalAccessRuleSourceIpRangesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataVmwareengineExternalAccessRuleSourceIpRangesElRef {
        DataVmwareengineExternalAccessRuleSourceIpRangesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataVmwareengineExternalAccessRuleSourceIpRangesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ip_address` after provisioning.\n"]
    pub fn ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip_address", self.base))
    }
    #[doc = "Get a reference to the value of field `ip_address_range` after provisioning.\n"]
    pub fn ip_address_range(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ip_address_range", self.base),
        )
    }
}
