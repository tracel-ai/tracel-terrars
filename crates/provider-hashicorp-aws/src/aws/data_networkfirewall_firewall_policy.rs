use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataNetworkfirewallFirewallPolicyData {
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
struct DataNetworkfirewallFirewallPolicy_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataNetworkfirewallFirewallPolicyData>,
}
#[derive(Clone)]
pub struct DataNetworkfirewallFirewallPolicy(Rc<DataNetworkfirewallFirewallPolicy_>);
impl DataNetworkfirewallFirewallPolicy {
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `firewall_policy` after provisioning.\n"]
    pub fn firewall_policy(&self) -> ListRef<DataNetworkfirewallFirewallPolicyFirewallPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.firewall_policy", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_token` after provisioning.\n"]
    pub fn update_token(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_token", self.extract_ref()),
        )
    }
}
impl Referable for DataNetworkfirewallFirewallPolicy {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataNetworkfirewallFirewallPolicy {}
impl ToListMappable for DataNetworkfirewallFirewallPolicy {
    type O = ListRef<DataNetworkfirewallFirewallPolicyRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataNetworkfirewallFirewallPolicy_ {
    fn extract_datasource_type(&self) -> String {
        "aws_networkfirewall_firewall_policy".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataNetworkfirewallFirewallPolicy {
    pub tf_id: String,
}
impl BuildDataNetworkfirewallFirewallPolicy {
    pub fn build(self, stack: &mut Stack) -> DataNetworkfirewallFirewallPolicy {
        let out = DataNetworkfirewallFirewallPolicy(Rc::new(DataNetworkfirewallFirewallPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataNetworkfirewallFirewallPolicyData {
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
pub struct DataNetworkfirewallFirewallPolicyRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataNetworkfirewallFirewallPolicyRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataNetworkfirewallFirewallPolicyRef {
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `firewall_policy` after provisioning.\n"]
    pub fn firewall_policy(&self) -> ListRef<DataNetworkfirewallFirewallPolicyFirewallPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.firewall_policy", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_token` after provisioning.\n"]
    pub fn update_token(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_token", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataNetworkfirewallFirewallPolicyFirewallPolicyElPolicyVariablesElRuleVariablesElIpSetEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    definition: Option<SetField<PrimField<String>>>,
}
impl DataNetworkfirewallFirewallPolicyFirewallPolicyElPolicyVariablesElRuleVariablesElIpSetEl {
    #[doc = "Set the field `definition`.\n"]
    pub fn set_definition(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.definition = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataNetworkfirewallFirewallPolicyFirewallPolicyElPolicyVariablesElRuleVariablesElIpSetEl
{
    type O = BlockAssignable<
        DataNetworkfirewallFirewallPolicyFirewallPolicyElPolicyVariablesElRuleVariablesElIpSetEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataNetworkfirewallFirewallPolicyFirewallPolicyElPolicyVariablesElRuleVariablesElIpSetEl
{}
impl BuildDataNetworkfirewallFirewallPolicyFirewallPolicyElPolicyVariablesElRuleVariablesElIpSetEl {
    pub fn build(
        self,
    ) -> DataNetworkfirewallFirewallPolicyFirewallPolicyElPolicyVariablesElRuleVariablesElIpSetEl
    {
        DataNetworkfirewallFirewallPolicyFirewallPolicyElPolicyVariablesElRuleVariablesElIpSetEl {
            definition: core::default::Default::default(),
        }
    }
}
pub struct DataNetworkfirewallFirewallPolicyFirewallPolicyElPolicyVariablesElRuleVariablesElIpSetElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for DataNetworkfirewallFirewallPolicyFirewallPolicyElPolicyVariablesElRuleVariablesElIpSetElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataNetworkfirewallFirewallPolicyFirewallPolicyElPolicyVariablesElRuleVariablesElIpSetElRef
    {
        DataNetworkfirewallFirewallPolicyFirewallPolicyElPolicyVariablesElRuleVariablesElIpSetElRef { shared : shared , base : base . to_string () , }
    }
}
impl DataNetworkfirewallFirewallPolicyFirewallPolicyElPolicyVariablesElRuleVariablesElIpSetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `definition` after provisioning.\n"]
    pub fn definition(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.definition", self.base))
    }
}
#[derive(Serialize)]
pub struct DataNetworkfirewallFirewallPolicyFirewallPolicyElPolicyVariablesElRuleVariablesEl { # [serde (skip_serializing_if = "Option::is_none")] ip_set : Option < ListField < DataNetworkfirewallFirewallPolicyFirewallPolicyElPolicyVariablesElRuleVariablesElIpSetEl > > , # [serde (skip_serializing_if = "Option::is_none")] key : Option < PrimField < String > > , }
impl DataNetworkfirewallFirewallPolicyFirewallPolicyElPolicyVariablesElRuleVariablesEl {
    #[doc = "Set the field `ip_set`.\n"]
    pub fn set_ip_set(
        mut self,
        v : impl Into < ListField < DataNetworkfirewallFirewallPolicyFirewallPolicyElPolicyVariablesElRuleVariablesElIpSetEl > >,
    ) -> Self {
        self.ip_set = Some(v.into());
        self
    }
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataNetworkfirewallFirewallPolicyFirewallPolicyElPolicyVariablesElRuleVariablesEl
{
    type O = BlockAssignable<
        DataNetworkfirewallFirewallPolicyFirewallPolicyElPolicyVariablesElRuleVariablesEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataNetworkfirewallFirewallPolicyFirewallPolicyElPolicyVariablesElRuleVariablesEl {
}
impl BuildDataNetworkfirewallFirewallPolicyFirewallPolicyElPolicyVariablesElRuleVariablesEl {
    pub fn build(
        self,
    ) -> DataNetworkfirewallFirewallPolicyFirewallPolicyElPolicyVariablesElRuleVariablesEl {
        DataNetworkfirewallFirewallPolicyFirewallPolicyElPolicyVariablesElRuleVariablesEl {
            ip_set: core::default::Default::default(),
            key: core::default::Default::default(),
        }
    }
}
pub struct DataNetworkfirewallFirewallPolicyFirewallPolicyElPolicyVariablesElRuleVariablesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataNetworkfirewallFirewallPolicyFirewallPolicyElPolicyVariablesElRuleVariablesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataNetworkfirewallFirewallPolicyFirewallPolicyElPolicyVariablesElRuleVariablesElRef {
        DataNetworkfirewallFirewallPolicyFirewallPolicyElPolicyVariablesElRuleVariablesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataNetworkfirewallFirewallPolicyFirewallPolicyElPolicyVariablesElRuleVariablesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ip_set` after provisioning.\n"]
    pub fn ip_set(
        &self,
    ) -> ListRef<
        DataNetworkfirewallFirewallPolicyFirewallPolicyElPolicyVariablesElRuleVariablesElIpSetElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.ip_set", self.base))
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
}
#[derive(Serialize)]
pub struct DataNetworkfirewallFirewallPolicyFirewallPolicyElPolicyVariablesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    rule_variables: Option<
        SetField<DataNetworkfirewallFirewallPolicyFirewallPolicyElPolicyVariablesElRuleVariablesEl>,
    >,
}
impl DataNetworkfirewallFirewallPolicyFirewallPolicyElPolicyVariablesEl {
    #[doc = "Set the field `rule_variables`.\n"]
    pub fn set_rule_variables(
        mut self,
        v: impl Into<
            SetField<
                DataNetworkfirewallFirewallPolicyFirewallPolicyElPolicyVariablesElRuleVariablesEl,
            >,
        >,
    ) -> Self {
        self.rule_variables = Some(v.into());
        self
    }
}
impl ToListMappable for DataNetworkfirewallFirewallPolicyFirewallPolicyElPolicyVariablesEl {
    type O = BlockAssignable<DataNetworkfirewallFirewallPolicyFirewallPolicyElPolicyVariablesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataNetworkfirewallFirewallPolicyFirewallPolicyElPolicyVariablesEl {}
impl BuildDataNetworkfirewallFirewallPolicyFirewallPolicyElPolicyVariablesEl {
    pub fn build(self) -> DataNetworkfirewallFirewallPolicyFirewallPolicyElPolicyVariablesEl {
        DataNetworkfirewallFirewallPolicyFirewallPolicyElPolicyVariablesEl {
            rule_variables: core::default::Default::default(),
        }
    }
}
pub struct DataNetworkfirewallFirewallPolicyFirewallPolicyElPolicyVariablesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataNetworkfirewallFirewallPolicyFirewallPolicyElPolicyVariablesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataNetworkfirewallFirewallPolicyFirewallPolicyElPolicyVariablesElRef {
        DataNetworkfirewallFirewallPolicyFirewallPolicyElPolicyVariablesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataNetworkfirewallFirewallPolicyFirewallPolicyElPolicyVariablesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `rule_variables` after provisioning.\n"]
    pub fn rule_variables(
        &self,
    ) -> SetRef<DataNetworkfirewallFirewallPolicyFirewallPolicyElPolicyVariablesElRuleVariablesElRef>
    {
        SetRef::new(
            self.shared().clone(),
            format!("{}.rule_variables", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulEngineOptionsElFlowTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    tcp_idle_timeout_seconds: Option<PrimField<f64>>,
}
impl DataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulEngineOptionsElFlowTimeoutsEl {
    #[doc = "Set the field `tcp_idle_timeout_seconds`.\n"]
    pub fn set_tcp_idle_timeout_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.tcp_idle_timeout_seconds = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulEngineOptionsElFlowTimeoutsEl
{
    type O = BlockAssignable<
        DataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulEngineOptionsElFlowTimeoutsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulEngineOptionsElFlowTimeoutsEl
{}
impl BuildDataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulEngineOptionsElFlowTimeoutsEl {
    pub fn build(
        self,
    ) -> DataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulEngineOptionsElFlowTimeoutsEl
    {
        DataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulEngineOptionsElFlowTimeoutsEl {
            tcp_idle_timeout_seconds: core::default::Default::default(),
        }
    }
}
pub struct DataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulEngineOptionsElFlowTimeoutsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for DataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulEngineOptionsElFlowTimeoutsElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulEngineOptionsElFlowTimeoutsElRef
    {
        DataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulEngineOptionsElFlowTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulEngineOptionsElFlowTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `tcp_idle_timeout_seconds` after provisioning.\n"]
    pub fn tcp_idle_timeout_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.tcp_idle_timeout_seconds", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulEngineOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    flow_timeouts: Option<
        ListField<
            DataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulEngineOptionsElFlowTimeoutsEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    rule_order: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stream_exception_policy: Option<PrimField<String>>,
}
impl DataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulEngineOptionsEl {
    #[doc = "Set the field `flow_timeouts`.\n"]
    pub fn set_flow_timeouts(
        mut self,
        v : impl Into < ListField < DataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulEngineOptionsElFlowTimeoutsEl > >,
    ) -> Self {
        self.flow_timeouts = Some(v.into());
        self
    }
    #[doc = "Set the field `rule_order`.\n"]
    pub fn set_rule_order(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.rule_order = Some(v.into());
        self
    }
    #[doc = "Set the field `stream_exception_policy`.\n"]
    pub fn set_stream_exception_policy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.stream_exception_policy = Some(v.into());
        self
    }
}
impl ToListMappable for DataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulEngineOptionsEl {
    type O =
        BlockAssignable<DataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulEngineOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulEngineOptionsEl {}
impl BuildDataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulEngineOptionsEl {
    pub fn build(self) -> DataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulEngineOptionsEl {
        DataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulEngineOptionsEl {
            flow_timeouts: core::default::Default::default(),
            rule_order: core::default::Default::default(),
            stream_exception_policy: core::default::Default::default(),
        }
    }
}
pub struct DataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulEngineOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulEngineOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulEngineOptionsElRef {
        DataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulEngineOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulEngineOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `flow_timeouts` after provisioning.\n"]
    pub fn flow_timeouts(
        &self,
    ) -> ListRef<
        DataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulEngineOptionsElFlowTimeoutsElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.flow_timeouts", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `rule_order` after provisioning.\n"]
    pub fn rule_order(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.rule_order", self.base))
    }
    #[doc = "Get a reference to the value of field `stream_exception_policy` after provisioning.\n"]
    pub fn stream_exception_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.stream_exception_policy", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceElOverrideEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<PrimField<String>>,
}
impl DataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceElOverrideEl {
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.action = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceElOverrideEl
{
    type O = BlockAssignable<
        DataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceElOverrideEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceElOverrideEl
{}
impl BuildDataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceElOverrideEl {
    pub fn build(
        self,
    ) -> DataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceElOverrideEl
    {
        DataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceElOverrideEl {
            action: core::default::Default::default(),
        }
    }
}
pub struct DataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceElOverrideElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for DataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceElOverrideElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceElOverrideElRef
    {
        DataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceElOverrideElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceElOverrideElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }
}
#[derive(Serialize)]
pub struct DataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    deep_threat_inspection: Option<PrimField<String>>,
    #[serde(rename = "override", skip_serializing_if = "Option::is_none")]
    override_: Option<
        ListField<
            DataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceElOverrideEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_arn: Option<PrimField<String>>,
}
impl DataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceEl {
    #[doc = "Set the field `deep_threat_inspection`.\n"]
    pub fn set_deep_threat_inspection(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.deep_threat_inspection = Some(v.into());
        self
    }
    #[doc = "Set the field `override_`.\n"]
    pub fn set_override(
        mut self,
        v : impl Into < ListField < DataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceElOverrideEl > >,
    ) -> Self {
        self.override_ = Some(v.into());
        self
    }
    #[doc = "Set the field `priority`.\n"]
    pub fn set_priority(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.priority = Some(v.into());
        self
    }
    #[doc = "Set the field `resource_arn`.\n"]
    pub fn set_resource_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.resource_arn = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceEl
{
    type O = BlockAssignable<
        DataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceEl {}
impl BuildDataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceEl {
    pub fn build(
        self,
    ) -> DataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceEl {
        DataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceEl {
            deep_threat_inspection: core::default::Default::default(),
            override_: core::default::Default::default(),
            priority: core::default::Default::default(),
            resource_arn: core::default::Default::default(),
        }
    }
}
pub struct DataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceElRef {
        DataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `deep_threat_inspection` after provisioning.\n"]
    pub fn deep_threat_inspection(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deep_threat_inspection", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `override_` after provisioning.\n"]
    pub fn override_(
        &self,
    ) -> ListRef<
        DataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceElOverrideElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.override", self.base))
    }
    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }
    #[doc = "Get a reference to the value of field `resource_arn` after provisioning.\n"]
    pub fn resource_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_arn", self.base))
    }
}
#[derive(Serialize)]
pub struct DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionElDimensionEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionElDimensionEl { # [doc = "Set the field `value`.\n"] pub fn set_value (mut self , v : impl Into < PrimField < String > >) -> Self { self . value = Some (v . into ()) ; self } }
impl ToListMappable for DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionElDimensionEl { type O = BlockAssignable < DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionElDimensionEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionElDimensionEl
{}
impl BuildDataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionElDimensionEl { pub fn build (self) -> DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionElDimensionEl { DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionElDimensionEl { value : core :: default :: Default :: default () , } } }
pub struct DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionElDimensionElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionElDimensionElRef { fn new (shared : StackShared , base : String) -> DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionElDimensionElRef { DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionElDimensionElRef { shared : shared , base : base . to_string () , } } }
impl DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionElDimensionElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize)]
pub struct DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionEl { # [serde (skip_serializing_if = "Option::is_none")] dimension : Option < SetField < DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionElDimensionEl > > , }
impl DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionEl { # [doc = "Set the field `dimension`.\n"] pub fn set_dimension (mut self , v : impl Into < SetField < DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionElDimensionEl > >) -> Self { self . dimension = Some (v . into ()) ; self } }
impl ToListMappable for DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionEl { type O = BlockAssignable < DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionEl
{}
impl BuildDataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionEl { pub fn build (self) -> DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionEl { DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionEl { dimension : core :: default :: Default :: default () , } } }
pub struct DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionElRef { fn new (shared : StackShared , base : String) -> DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionElRef { DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionElRef { shared : shared , base : base . to_string () , } } }
impl DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `dimension` after provisioning.\n"] pub fn dimension (& self) -> SetRef < DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionElDimensionElRef > { SetRef :: new (self . shared () . clone () , format ! ("{}.dimension" , self . base)) } }
#[derive(Serialize)]
pub struct DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionEl { # [serde (skip_serializing_if = "Option::is_none")] publish_metric_action : Option < ListField < DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionEl > > , }
impl DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionEl {
    #[doc = "Set the field `publish_metric_action`.\n"]
    pub fn set_publish_metric_action(
        mut self,
        v : impl Into < ListField < DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionEl > >,
    ) -> Self {
        self.publish_metric_action = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionEl
{
    type O = BlockAssignable<
        DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionEl
{}
impl
    BuildDataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionEl
{
    pub fn build(
        self,
    ) -> DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionEl
    {
        DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionEl {
            publish_metric_action: core::default::Default::default(),
        }
    }
}
pub struct DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElRef { fn new (shared : StackShared , base : String) -> DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElRef { DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElRef { shared : shared , base : base . to_string () , } } }
impl DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `publish_metric_action` after provisioning.\n"]    pub fn publish_metric_action (& self) -> ListRef < DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.publish_metric_action", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionEl { # [serde (skip_serializing_if = "Option::is_none")] action_definition : Option < ListField < DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionEl > > , # [serde (skip_serializing_if = "Option::is_none")] action_name : Option < PrimField < String > > , }
impl DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionEl {
    #[doc = "Set the field `action_definition`.\n"]
    pub fn set_action_definition(
        mut self,
        v : impl Into < ListField < DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionEl > >,
    ) -> Self {
        self.action_definition = Some(v.into());
        self
    }
    #[doc = "Set the field `action_name`.\n"]
    pub fn set_action_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.action_name = Some(v.into());
        self
    }
}
impl ToListMappable for DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionEl {
    type O =
        BlockAssignable<DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionEl {}
impl BuildDataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionEl {
    pub fn build(self) -> DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionEl {
        DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionEl {
            action_definition: core::default::Default::default(),
            action_name: core::default::Default::default(),
        }
    }
}
pub struct DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElRef {
        DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `action_definition` after provisioning.\n"]    pub fn action_definition (& self) -> ListRef < DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.action_definition", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `action_name` after provisioning.\n"]
    pub fn action_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.action_name", self.base))
    }
}
#[derive(Serialize)]
pub struct DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessRuleGroupReferenceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_arn: Option<PrimField<String>>,
}
impl DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessRuleGroupReferenceEl {
    #[doc = "Set the field `priority`.\n"]
    pub fn set_priority(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.priority = Some(v.into());
        self
    }
    #[doc = "Set the field `resource_arn`.\n"]
    pub fn set_resource_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.resource_arn = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessRuleGroupReferenceEl
{
    type O = BlockAssignable<
        DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessRuleGroupReferenceEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessRuleGroupReferenceEl {}
impl BuildDataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessRuleGroupReferenceEl {
    pub fn build(
        self,
    ) -> DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessRuleGroupReferenceEl {
        DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessRuleGroupReferenceEl {
            priority: core::default::Default::default(),
            resource_arn: core::default::Default::default(),
        }
    }
}
pub struct DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessRuleGroupReferenceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessRuleGroupReferenceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessRuleGroupReferenceElRef {
        DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessRuleGroupReferenceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessRuleGroupReferenceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }
    #[doc = "Get a reference to the value of field `resource_arn` after provisioning.\n"]
    pub fn resource_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_arn", self.base))
    }
}
#[derive(Serialize)]
pub struct DataNetworkfirewallFirewallPolicyFirewallPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    policy_variables:
        Option<ListField<DataNetworkfirewallFirewallPolicyFirewallPolicyElPolicyVariablesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stateful_default_actions: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stateful_engine_options:
        Option<ListField<DataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulEngineOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stateful_rule_group_reference: Option<
        SetField<DataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    stateless_custom_action:
        Option<SetField<DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stateless_default_actions: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stateless_fragment_default_actions: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stateless_rule_group_reference: Option<
        SetField<DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessRuleGroupReferenceEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    tls_inspection_configuration_arn: Option<PrimField<String>>,
}
impl DataNetworkfirewallFirewallPolicyFirewallPolicyEl {
    #[doc = "Set the field `policy_variables`.\n"]
    pub fn set_policy_variables(
        mut self,
        v: impl Into<ListField<DataNetworkfirewallFirewallPolicyFirewallPolicyElPolicyVariablesEl>>,
    ) -> Self {
        self.policy_variables = Some(v.into());
        self
    }
    #[doc = "Set the field `stateful_default_actions`.\n"]
    pub fn set_stateful_default_actions(
        mut self,
        v: impl Into<SetField<PrimField<String>>>,
    ) -> Self {
        self.stateful_default_actions = Some(v.into());
        self
    }
    #[doc = "Set the field `stateful_engine_options`.\n"]
    pub fn set_stateful_engine_options(
        mut self,
        v: impl Into<
            ListField<DataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulEngineOptionsEl>,
        >,
    ) -> Self {
        self.stateful_engine_options = Some(v.into());
        self
    }
    #[doc = "Set the field `stateful_rule_group_reference`.\n"]
    pub fn set_stateful_rule_group_reference(
        mut self,
        v: impl Into<
            SetField<DataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceEl>,
        >,
    ) -> Self {
        self.stateful_rule_group_reference = Some(v.into());
        self
    }
    #[doc = "Set the field `stateless_custom_action`.\n"]
    pub fn set_stateless_custom_action(
        mut self,
        v: impl Into<SetField<DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionEl>>,
    ) -> Self {
        self.stateless_custom_action = Some(v.into());
        self
    }
    #[doc = "Set the field `stateless_default_actions`.\n"]
    pub fn set_stateless_default_actions(
        mut self,
        v: impl Into<SetField<PrimField<String>>>,
    ) -> Self {
        self.stateless_default_actions = Some(v.into());
        self
    }
    #[doc = "Set the field `stateless_fragment_default_actions`.\n"]
    pub fn set_stateless_fragment_default_actions(
        mut self,
        v: impl Into<SetField<PrimField<String>>>,
    ) -> Self {
        self.stateless_fragment_default_actions = Some(v.into());
        self
    }
    #[doc = "Set the field `stateless_rule_group_reference`.\n"]
    pub fn set_stateless_rule_group_reference(
        mut self,
        v: impl Into<
            SetField<
                DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessRuleGroupReferenceEl,
            >,
        >,
    ) -> Self {
        self.stateless_rule_group_reference = Some(v.into());
        self
    }
    #[doc = "Set the field `tls_inspection_configuration_arn`.\n"]
    pub fn set_tls_inspection_configuration_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tls_inspection_configuration_arn = Some(v.into());
        self
    }
}
impl ToListMappable for DataNetworkfirewallFirewallPolicyFirewallPolicyEl {
    type O = BlockAssignable<DataNetworkfirewallFirewallPolicyFirewallPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataNetworkfirewallFirewallPolicyFirewallPolicyEl {}
impl BuildDataNetworkfirewallFirewallPolicyFirewallPolicyEl {
    pub fn build(self) -> DataNetworkfirewallFirewallPolicyFirewallPolicyEl {
        DataNetworkfirewallFirewallPolicyFirewallPolicyEl {
            policy_variables: core::default::Default::default(),
            stateful_default_actions: core::default::Default::default(),
            stateful_engine_options: core::default::Default::default(),
            stateful_rule_group_reference: core::default::Default::default(),
            stateless_custom_action: core::default::Default::default(),
            stateless_default_actions: core::default::Default::default(),
            stateless_fragment_default_actions: core::default::Default::default(),
            stateless_rule_group_reference: core::default::Default::default(),
            tls_inspection_configuration_arn: core::default::Default::default(),
        }
    }
}
pub struct DataNetworkfirewallFirewallPolicyFirewallPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataNetworkfirewallFirewallPolicyFirewallPolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataNetworkfirewallFirewallPolicyFirewallPolicyElRef {
        DataNetworkfirewallFirewallPolicyFirewallPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataNetworkfirewallFirewallPolicyFirewallPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `policy_variables` after provisioning.\n"]
    pub fn policy_variables(
        &self,
    ) -> ListRef<DataNetworkfirewallFirewallPolicyFirewallPolicyElPolicyVariablesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.policy_variables", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `stateful_default_actions` after provisioning.\n"]
    pub fn stateful_default_actions(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.stateful_default_actions", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `stateful_engine_options` after provisioning.\n"]
    pub fn stateful_engine_options(
        &self,
    ) -> ListRef<DataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulEngineOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.stateful_engine_options", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `stateful_rule_group_reference` after provisioning.\n"]
    pub fn stateful_rule_group_reference(
        &self,
    ) -> SetRef<DataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceElRef>
    {
        SetRef::new(
            self.shared().clone(),
            format!("{}.stateful_rule_group_reference", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `stateless_custom_action` after provisioning.\n"]
    pub fn stateless_custom_action(
        &self,
    ) -> SetRef<DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.stateless_custom_action", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `stateless_default_actions` after provisioning.\n"]
    pub fn stateless_default_actions(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.stateless_default_actions", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `stateless_fragment_default_actions` after provisioning.\n"]
    pub fn stateless_fragment_default_actions(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.stateless_fragment_default_actions", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `stateless_rule_group_reference` after provisioning.\n"]
    pub fn stateless_rule_group_reference(
        &self,
    ) -> SetRef<DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessRuleGroupReferenceElRef>
    {
        SetRef::new(
            self.shared().clone(),
            format!("{}.stateless_rule_group_reference", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `tls_inspection_configuration_arn` after provisioning.\n"]
    pub fn tls_inspection_configuration_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.tls_inspection_configuration_arn", self.base),
        )
    }
}
