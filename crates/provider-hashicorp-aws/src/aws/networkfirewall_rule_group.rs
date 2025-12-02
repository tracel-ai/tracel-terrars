use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct NetworkfirewallRuleGroupData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    capacity: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rules: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_configuration: Option<Vec<NetworkfirewallRuleGroupEncryptionConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rule_group: Option<Vec<NetworkfirewallRuleGroupRuleGroupEl>>,
    dynamic: NetworkfirewallRuleGroupDynamic,
}
struct NetworkfirewallRuleGroup_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<NetworkfirewallRuleGroupData>,
}
#[derive(Clone)]
pub struct NetworkfirewallRuleGroup(Rc<NetworkfirewallRuleGroup_>);
impl NetworkfirewallRuleGroup {
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
        self.0.data.borrow_mut().lifecycle.ignore_changes =
            Some(IgnoreChanges::All(IgnoreChangesAll::All));
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
                    }
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }
    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0
            .data
            .borrow_mut()
            .lifecycle
            .replace_triggered_by
            .push(r.extract_ref());
        self
    }
    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0
            .data
            .borrow_mut()
            .lifecycle
            .replace_triggered_by
            .push(attr.to_string());
        self
    }
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
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
    #[doc = "Set the field `rules`.\n"]
    pub fn set_rules(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().rules = Some(v.into());
        self
    }
    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }
    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags_all = Some(v.into());
        self
    }
    #[doc = "Set the field `encryption_configuration`.\n"]
    pub fn set_encryption_configuration(
        self,
        v: impl Into<BlockAssignable<NetworkfirewallRuleGroupEncryptionConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().encryption_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.encryption_configuration = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `rule_group`.\n"]
    pub fn set_rule_group(
        self,
        v: impl Into<BlockAssignable<NetworkfirewallRuleGroupRuleGroupEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rule_group = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rule_group = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `capacity` after provisioning.\n"]
    pub fn capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.capacity", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rules", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_token` after provisioning.\n"]
    pub fn update_token(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_token", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `encryption_configuration` after provisioning.\n"]
    pub fn encryption_configuration(
        &self,
    ) -> ListRef<NetworkfirewallRuleGroupEncryptionConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.encryption_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rule_group` after provisioning.\n"]
    pub fn rule_group(&self) -> ListRef<NetworkfirewallRuleGroupRuleGroupElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rule_group", self.extract_ref()),
        )
    }
}
impl Referable for NetworkfirewallRuleGroup {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for NetworkfirewallRuleGroup {}
impl ToListMappable for NetworkfirewallRuleGroup {
    type O = ListRef<NetworkfirewallRuleGroupRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for NetworkfirewallRuleGroup_ {
    fn extract_resource_type(&self) -> String {
        "aws_networkfirewall_rule_group".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildNetworkfirewallRuleGroup {
    pub tf_id: String,
    #[doc = ""]
    pub capacity: PrimField<f64>,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub type_: PrimField<String>,
}
impl BuildNetworkfirewallRuleGroup {
    pub fn build(self, stack: &mut Stack) -> NetworkfirewallRuleGroup {
        let out = NetworkfirewallRuleGroup(Rc::new(NetworkfirewallRuleGroup_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(NetworkfirewallRuleGroupData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                capacity: self.capacity,
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                rules: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                type_: self.type_,
                encryption_configuration: core::default::Default::default(),
                rule_group: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct NetworkfirewallRuleGroupRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkfirewallRuleGroupRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl NetworkfirewallRuleGroupRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `capacity` after provisioning.\n"]
    pub fn capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.capacity", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rules", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_token` after provisioning.\n"]
    pub fn update_token(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_token", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `encryption_configuration` after provisioning.\n"]
    pub fn encryption_configuration(
        &self,
    ) -> ListRef<NetworkfirewallRuleGroupEncryptionConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.encryption_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rule_group` after provisioning.\n"]
    pub fn rule_group(&self) -> ListRef<NetworkfirewallRuleGroupRuleGroupElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rule_group", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct NetworkfirewallRuleGroupEncryptionConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key_id: Option<PrimField<String>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}
impl NetworkfirewallRuleGroupEncryptionConfigurationEl {
    #[doc = "Set the field `key_id`.\n"]
    pub fn set_key_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key_id = Some(v.into());
        self
    }
}
impl ToListMappable for NetworkfirewallRuleGroupEncryptionConfigurationEl {
    type O = BlockAssignable<NetworkfirewallRuleGroupEncryptionConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkfirewallRuleGroupEncryptionConfigurationEl {
    #[doc = ""]
    pub type_: PrimField<String>,
}
impl BuildNetworkfirewallRuleGroupEncryptionConfigurationEl {
    pub fn build(self) -> NetworkfirewallRuleGroupEncryptionConfigurationEl {
        NetworkfirewallRuleGroupEncryptionConfigurationEl {
            key_id: core::default::Default::default(),
            type_: self.type_,
        }
    }
}
pub struct NetworkfirewallRuleGroupEncryptionConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkfirewallRuleGroupEncryptionConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkfirewallRuleGroupEncryptionConfigurationElRef {
        NetworkfirewallRuleGroupEncryptionConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkfirewallRuleGroupEncryptionConfigurationElRef {
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
pub struct NetworkfirewallRuleGroupRuleGroupElReferenceSetsElIpSetReferencesElIpSetReferenceEl {
    reference_arn: PrimField<String>,
}
impl NetworkfirewallRuleGroupRuleGroupElReferenceSetsElIpSetReferencesElIpSetReferenceEl {}
impl ToListMappable
    for NetworkfirewallRuleGroupRuleGroupElReferenceSetsElIpSetReferencesElIpSetReferenceEl
{
    type O = BlockAssignable<
        NetworkfirewallRuleGroupRuleGroupElReferenceSetsElIpSetReferencesElIpSetReferenceEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkfirewallRuleGroupRuleGroupElReferenceSetsElIpSetReferencesElIpSetReferenceEl
{
    #[doc = ""]
    pub reference_arn: PrimField<String>,
}
impl BuildNetworkfirewallRuleGroupRuleGroupElReferenceSetsElIpSetReferencesElIpSetReferenceEl {
    pub fn build(
        self,
    ) -> NetworkfirewallRuleGroupRuleGroupElReferenceSetsElIpSetReferencesElIpSetReferenceEl {
        NetworkfirewallRuleGroupRuleGroupElReferenceSetsElIpSetReferencesElIpSetReferenceEl {
            reference_arn: self.reference_arn,
        }
    }
}
pub struct NetworkfirewallRuleGroupRuleGroupElReferenceSetsElIpSetReferencesElIpSetReferenceElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for NetworkfirewallRuleGroupRuleGroupElReferenceSetsElIpSetReferencesElIpSetReferenceElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkfirewallRuleGroupRuleGroupElReferenceSetsElIpSetReferencesElIpSetReferenceElRef
    {
        NetworkfirewallRuleGroupRuleGroupElReferenceSetsElIpSetReferencesElIpSetReferenceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkfirewallRuleGroupRuleGroupElReferenceSetsElIpSetReferencesElIpSetReferenceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `reference_arn` after provisioning.\n"]
    pub fn reference_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reference_arn", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct NetworkfirewallRuleGroupRuleGroupElReferenceSetsElIpSetReferencesElDynamic {
    ip_set_reference: Option<
        DynamicBlock<
            NetworkfirewallRuleGroupRuleGroupElReferenceSetsElIpSetReferencesElIpSetReferenceEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct NetworkfirewallRuleGroupRuleGroupElReferenceSetsElIpSetReferencesEl {
    key: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_set_reference: Option<
        Vec<NetworkfirewallRuleGroupRuleGroupElReferenceSetsElIpSetReferencesElIpSetReferenceEl>,
    >,
    dynamic: NetworkfirewallRuleGroupRuleGroupElReferenceSetsElIpSetReferencesElDynamic,
}
impl NetworkfirewallRuleGroupRuleGroupElReferenceSetsElIpSetReferencesEl {
    #[doc = "Set the field `ip_set_reference`.\n"]
    pub fn set_ip_set_reference(
        mut self,
        v: impl Into<
            BlockAssignable<
                NetworkfirewallRuleGroupRuleGroupElReferenceSetsElIpSetReferencesElIpSetReferenceEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ip_set_reference = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ip_set_reference = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for NetworkfirewallRuleGroupRuleGroupElReferenceSetsElIpSetReferencesEl {
    type O = BlockAssignable<NetworkfirewallRuleGroupRuleGroupElReferenceSetsElIpSetReferencesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkfirewallRuleGroupRuleGroupElReferenceSetsElIpSetReferencesEl {
    #[doc = ""]
    pub key: PrimField<String>,
}
impl BuildNetworkfirewallRuleGroupRuleGroupElReferenceSetsElIpSetReferencesEl {
    pub fn build(self) -> NetworkfirewallRuleGroupRuleGroupElReferenceSetsElIpSetReferencesEl {
        NetworkfirewallRuleGroupRuleGroupElReferenceSetsElIpSetReferencesEl {
            key: self.key,
            ip_set_reference: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct NetworkfirewallRuleGroupRuleGroupElReferenceSetsElIpSetReferencesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkfirewallRuleGroupRuleGroupElReferenceSetsElIpSetReferencesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkfirewallRuleGroupRuleGroupElReferenceSetsElIpSetReferencesElRef {
        NetworkfirewallRuleGroupRuleGroupElReferenceSetsElIpSetReferencesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkfirewallRuleGroupRuleGroupElReferenceSetsElIpSetReferencesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `ip_set_reference` after provisioning.\n"]
    pub fn ip_set_reference(
        &self,
    ) -> ListRef<
        NetworkfirewallRuleGroupRuleGroupElReferenceSetsElIpSetReferencesElIpSetReferenceElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ip_set_reference", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct NetworkfirewallRuleGroupRuleGroupElReferenceSetsElDynamic {
    ip_set_references:
        Option<DynamicBlock<NetworkfirewallRuleGroupRuleGroupElReferenceSetsElIpSetReferencesEl>>,
}
#[derive(Serialize)]
pub struct NetworkfirewallRuleGroupRuleGroupElReferenceSetsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_set_references:
        Option<Vec<NetworkfirewallRuleGroupRuleGroupElReferenceSetsElIpSetReferencesEl>>,
    dynamic: NetworkfirewallRuleGroupRuleGroupElReferenceSetsElDynamic,
}
impl NetworkfirewallRuleGroupRuleGroupElReferenceSetsEl {
    #[doc = "Set the field `ip_set_references`.\n"]
    pub fn set_ip_set_references(
        mut self,
        v: impl Into<
            BlockAssignable<NetworkfirewallRuleGroupRuleGroupElReferenceSetsElIpSetReferencesEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ip_set_references = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ip_set_references = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for NetworkfirewallRuleGroupRuleGroupElReferenceSetsEl {
    type O = BlockAssignable<NetworkfirewallRuleGroupRuleGroupElReferenceSetsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkfirewallRuleGroupRuleGroupElReferenceSetsEl {}
impl BuildNetworkfirewallRuleGroupRuleGroupElReferenceSetsEl {
    pub fn build(self) -> NetworkfirewallRuleGroupRuleGroupElReferenceSetsEl {
        NetworkfirewallRuleGroupRuleGroupElReferenceSetsEl {
            ip_set_references: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct NetworkfirewallRuleGroupRuleGroupElReferenceSetsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkfirewallRuleGroupRuleGroupElReferenceSetsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkfirewallRuleGroupRuleGroupElReferenceSetsElRef {
        NetworkfirewallRuleGroupRuleGroupElReferenceSetsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkfirewallRuleGroupRuleGroupElReferenceSetsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct NetworkfirewallRuleGroupRuleGroupElRuleVariablesElIpSetsElIpSetEl {
    definition: SetField<PrimField<String>>,
}
impl NetworkfirewallRuleGroupRuleGroupElRuleVariablesElIpSetsElIpSetEl {}
impl ToListMappable for NetworkfirewallRuleGroupRuleGroupElRuleVariablesElIpSetsElIpSetEl {
    type O = BlockAssignable<NetworkfirewallRuleGroupRuleGroupElRuleVariablesElIpSetsElIpSetEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkfirewallRuleGroupRuleGroupElRuleVariablesElIpSetsElIpSetEl {
    #[doc = ""]
    pub definition: SetField<PrimField<String>>,
}
impl BuildNetworkfirewallRuleGroupRuleGroupElRuleVariablesElIpSetsElIpSetEl {
    pub fn build(self) -> NetworkfirewallRuleGroupRuleGroupElRuleVariablesElIpSetsElIpSetEl {
        NetworkfirewallRuleGroupRuleGroupElRuleVariablesElIpSetsElIpSetEl {
            definition: self.definition,
        }
    }
}
pub struct NetworkfirewallRuleGroupRuleGroupElRuleVariablesElIpSetsElIpSetElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkfirewallRuleGroupRuleGroupElRuleVariablesElIpSetsElIpSetElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkfirewallRuleGroupRuleGroupElRuleVariablesElIpSetsElIpSetElRef {
        NetworkfirewallRuleGroupRuleGroupElRuleVariablesElIpSetsElIpSetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkfirewallRuleGroupRuleGroupElRuleVariablesElIpSetsElIpSetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `definition` after provisioning.\n"]
    pub fn definition(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.definition", self.base))
    }
}
#[derive(Serialize, Default)]
struct NetworkfirewallRuleGroupRuleGroupElRuleVariablesElIpSetsElDynamic {
    ip_set: Option<DynamicBlock<NetworkfirewallRuleGroupRuleGroupElRuleVariablesElIpSetsElIpSetEl>>,
}
#[derive(Serialize)]
pub struct NetworkfirewallRuleGroupRuleGroupElRuleVariablesElIpSetsEl {
    key: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_set: Option<Vec<NetworkfirewallRuleGroupRuleGroupElRuleVariablesElIpSetsElIpSetEl>>,
    dynamic: NetworkfirewallRuleGroupRuleGroupElRuleVariablesElIpSetsElDynamic,
}
impl NetworkfirewallRuleGroupRuleGroupElRuleVariablesElIpSetsEl {
    #[doc = "Set the field `ip_set`.\n"]
    pub fn set_ip_set(
        mut self,
        v: impl Into<BlockAssignable<NetworkfirewallRuleGroupRuleGroupElRuleVariablesElIpSetsElIpSetEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ip_set = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ip_set = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for NetworkfirewallRuleGroupRuleGroupElRuleVariablesElIpSetsEl {
    type O = BlockAssignable<NetworkfirewallRuleGroupRuleGroupElRuleVariablesElIpSetsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkfirewallRuleGroupRuleGroupElRuleVariablesElIpSetsEl {
    #[doc = ""]
    pub key: PrimField<String>,
}
impl BuildNetworkfirewallRuleGroupRuleGroupElRuleVariablesElIpSetsEl {
    pub fn build(self) -> NetworkfirewallRuleGroupRuleGroupElRuleVariablesElIpSetsEl {
        NetworkfirewallRuleGroupRuleGroupElRuleVariablesElIpSetsEl {
            key: self.key,
            ip_set: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct NetworkfirewallRuleGroupRuleGroupElRuleVariablesElIpSetsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkfirewallRuleGroupRuleGroupElRuleVariablesElIpSetsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkfirewallRuleGroupRuleGroupElRuleVariablesElIpSetsElRef {
        NetworkfirewallRuleGroupRuleGroupElRuleVariablesElIpSetsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkfirewallRuleGroupRuleGroupElRuleVariablesElIpSetsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `ip_set` after provisioning.\n"]
    pub fn ip_set(
        &self,
    ) -> ListRef<NetworkfirewallRuleGroupRuleGroupElRuleVariablesElIpSetsElIpSetElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ip_set", self.base))
    }
}
#[derive(Serialize)]
pub struct NetworkfirewallRuleGroupRuleGroupElRuleVariablesElPortSetsElPortSetEl {
    definition: SetField<PrimField<String>>,
}
impl NetworkfirewallRuleGroupRuleGroupElRuleVariablesElPortSetsElPortSetEl {}
impl ToListMappable for NetworkfirewallRuleGroupRuleGroupElRuleVariablesElPortSetsElPortSetEl {
    type O = BlockAssignable<NetworkfirewallRuleGroupRuleGroupElRuleVariablesElPortSetsElPortSetEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkfirewallRuleGroupRuleGroupElRuleVariablesElPortSetsElPortSetEl {
    #[doc = ""]
    pub definition: SetField<PrimField<String>>,
}
impl BuildNetworkfirewallRuleGroupRuleGroupElRuleVariablesElPortSetsElPortSetEl {
    pub fn build(self) -> NetworkfirewallRuleGroupRuleGroupElRuleVariablesElPortSetsElPortSetEl {
        NetworkfirewallRuleGroupRuleGroupElRuleVariablesElPortSetsElPortSetEl {
            definition: self.definition,
        }
    }
}
pub struct NetworkfirewallRuleGroupRuleGroupElRuleVariablesElPortSetsElPortSetElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkfirewallRuleGroupRuleGroupElRuleVariablesElPortSetsElPortSetElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkfirewallRuleGroupRuleGroupElRuleVariablesElPortSetsElPortSetElRef {
        NetworkfirewallRuleGroupRuleGroupElRuleVariablesElPortSetsElPortSetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkfirewallRuleGroupRuleGroupElRuleVariablesElPortSetsElPortSetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `definition` after provisioning.\n"]
    pub fn definition(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.definition", self.base))
    }
}
#[derive(Serialize, Default)]
struct NetworkfirewallRuleGroupRuleGroupElRuleVariablesElPortSetsElDynamic {
    port_set:
        Option<DynamicBlock<NetworkfirewallRuleGroupRuleGroupElRuleVariablesElPortSetsElPortSetEl>>,
}
#[derive(Serialize)]
pub struct NetworkfirewallRuleGroupRuleGroupElRuleVariablesElPortSetsEl {
    key: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port_set: Option<Vec<NetworkfirewallRuleGroupRuleGroupElRuleVariablesElPortSetsElPortSetEl>>,
    dynamic: NetworkfirewallRuleGroupRuleGroupElRuleVariablesElPortSetsElDynamic,
}
impl NetworkfirewallRuleGroupRuleGroupElRuleVariablesElPortSetsEl {
    #[doc = "Set the field `port_set`.\n"]
    pub fn set_port_set(
        mut self,
        v: impl Into<
            BlockAssignable<NetworkfirewallRuleGroupRuleGroupElRuleVariablesElPortSetsElPortSetEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.port_set = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.port_set = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for NetworkfirewallRuleGroupRuleGroupElRuleVariablesElPortSetsEl {
    type O = BlockAssignable<NetworkfirewallRuleGroupRuleGroupElRuleVariablesElPortSetsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkfirewallRuleGroupRuleGroupElRuleVariablesElPortSetsEl {
    #[doc = ""]
    pub key: PrimField<String>,
}
impl BuildNetworkfirewallRuleGroupRuleGroupElRuleVariablesElPortSetsEl {
    pub fn build(self) -> NetworkfirewallRuleGroupRuleGroupElRuleVariablesElPortSetsEl {
        NetworkfirewallRuleGroupRuleGroupElRuleVariablesElPortSetsEl {
            key: self.key,
            port_set: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct NetworkfirewallRuleGroupRuleGroupElRuleVariablesElPortSetsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkfirewallRuleGroupRuleGroupElRuleVariablesElPortSetsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkfirewallRuleGroupRuleGroupElRuleVariablesElPortSetsElRef {
        NetworkfirewallRuleGroupRuleGroupElRuleVariablesElPortSetsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkfirewallRuleGroupRuleGroupElRuleVariablesElPortSetsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `port_set` after provisioning.\n"]
    pub fn port_set(
        &self,
    ) -> ListRef<NetworkfirewallRuleGroupRuleGroupElRuleVariablesElPortSetsElPortSetElRef> {
        ListRef::new(self.shared().clone(), format!("{}.port_set", self.base))
    }
}
#[derive(Serialize, Default)]
struct NetworkfirewallRuleGroupRuleGroupElRuleVariablesElDynamic {
    ip_sets: Option<DynamicBlock<NetworkfirewallRuleGroupRuleGroupElRuleVariablesElIpSetsEl>>,
    port_sets: Option<DynamicBlock<NetworkfirewallRuleGroupRuleGroupElRuleVariablesElPortSetsEl>>,
}
#[derive(Serialize)]
pub struct NetworkfirewallRuleGroupRuleGroupElRuleVariablesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_sets: Option<Vec<NetworkfirewallRuleGroupRuleGroupElRuleVariablesElIpSetsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port_sets: Option<Vec<NetworkfirewallRuleGroupRuleGroupElRuleVariablesElPortSetsEl>>,
    dynamic: NetworkfirewallRuleGroupRuleGroupElRuleVariablesElDynamic,
}
impl NetworkfirewallRuleGroupRuleGroupElRuleVariablesEl {
    #[doc = "Set the field `ip_sets`.\n"]
    pub fn set_ip_sets(
        mut self,
        v: impl Into<BlockAssignable<NetworkfirewallRuleGroupRuleGroupElRuleVariablesElIpSetsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ip_sets = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ip_sets = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `port_sets`.\n"]
    pub fn set_port_sets(
        mut self,
        v: impl Into<BlockAssignable<NetworkfirewallRuleGroupRuleGroupElRuleVariablesElPortSetsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.port_sets = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.port_sets = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for NetworkfirewallRuleGroupRuleGroupElRuleVariablesEl {
    type O = BlockAssignable<NetworkfirewallRuleGroupRuleGroupElRuleVariablesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkfirewallRuleGroupRuleGroupElRuleVariablesEl {}
impl BuildNetworkfirewallRuleGroupRuleGroupElRuleVariablesEl {
    pub fn build(self) -> NetworkfirewallRuleGroupRuleGroupElRuleVariablesEl {
        NetworkfirewallRuleGroupRuleGroupElRuleVariablesEl {
            ip_sets: core::default::Default::default(),
            port_sets: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct NetworkfirewallRuleGroupRuleGroupElRuleVariablesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkfirewallRuleGroupRuleGroupElRuleVariablesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkfirewallRuleGroupRuleGroupElRuleVariablesElRef {
        NetworkfirewallRuleGroupRuleGroupElRuleVariablesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkfirewallRuleGroupRuleGroupElRuleVariablesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct NetworkfirewallRuleGroupRuleGroupElRulesSourceElRulesSourceListEl {
    generated_rules_type: PrimField<String>,
    target_types: SetField<PrimField<String>>,
    targets: SetField<PrimField<String>>,
}
impl NetworkfirewallRuleGroupRuleGroupElRulesSourceElRulesSourceListEl {}
impl ToListMappable for NetworkfirewallRuleGroupRuleGroupElRulesSourceElRulesSourceListEl {
    type O = BlockAssignable<NetworkfirewallRuleGroupRuleGroupElRulesSourceElRulesSourceListEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkfirewallRuleGroupRuleGroupElRulesSourceElRulesSourceListEl {
    #[doc = ""]
    pub generated_rules_type: PrimField<String>,
    #[doc = ""]
    pub target_types: SetField<PrimField<String>>,
    #[doc = ""]
    pub targets: SetField<PrimField<String>>,
}
impl BuildNetworkfirewallRuleGroupRuleGroupElRulesSourceElRulesSourceListEl {
    pub fn build(self) -> NetworkfirewallRuleGroupRuleGroupElRulesSourceElRulesSourceListEl {
        NetworkfirewallRuleGroupRuleGroupElRulesSourceElRulesSourceListEl {
            generated_rules_type: self.generated_rules_type,
            target_types: self.target_types,
            targets: self.targets,
        }
    }
}
pub struct NetworkfirewallRuleGroupRuleGroupElRulesSourceElRulesSourceListElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkfirewallRuleGroupRuleGroupElRulesSourceElRulesSourceListElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkfirewallRuleGroupRuleGroupElRulesSourceElRulesSourceListElRef {
        NetworkfirewallRuleGroupRuleGroupElRulesSourceElRulesSourceListElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkfirewallRuleGroupRuleGroupElRulesSourceElRulesSourceListElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `generated_rules_type` after provisioning.\n"]
    pub fn generated_rules_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.generated_rules_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `target_types` after provisioning.\n"]
    pub fn target_types(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.target_types", self.base))
    }
    #[doc = "Get a reference to the value of field `targets` after provisioning.\n"]
    pub fn targets(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.targets", self.base))
    }
}
#[derive(Serialize)]
pub struct NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleElHeaderEl {
    destination: PrimField<String>,
    destination_port: PrimField<String>,
    direction: PrimField<String>,
    protocol: PrimField<String>,
    source: PrimField<String>,
    source_port: PrimField<String>,
}
impl NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleElHeaderEl {}
impl ToListMappable for NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleElHeaderEl {
    type O =
        BlockAssignable<NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleElHeaderEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleElHeaderEl {
    #[doc = ""]
    pub destination: PrimField<String>,
    #[doc = ""]
    pub destination_port: PrimField<String>,
    #[doc = ""]
    pub direction: PrimField<String>,
    #[doc = ""]
    pub protocol: PrimField<String>,
    #[doc = ""]
    pub source: PrimField<String>,
    #[doc = ""]
    pub source_port: PrimField<String>,
}
impl BuildNetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleElHeaderEl {
    pub fn build(self) -> NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleElHeaderEl {
        NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleElHeaderEl {
            destination: self.destination,
            destination_port: self.destination_port,
            direction: self.direction,
            protocol: self.protocol,
            source: self.source,
            source_port: self.source_port,
        }
    }
}
pub struct NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleElHeaderElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleElHeaderElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleElHeaderElRef {
        NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleElHeaderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleElHeaderElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.destination", self.base))
    }
    #[doc = "Get a reference to the value of field `destination_port` after provisioning.\n"]
    pub fn destination_port(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.destination_port", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `direction` after provisioning.\n"]
    pub fn direction(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.direction", self.base))
    }
    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }
    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source", self.base))
    }
    #[doc = "Get a reference to the value of field `source_port` after provisioning.\n"]
    pub fn source_port(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_port", self.base))
    }
}
#[derive(Serialize)]
pub struct NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleElRuleOptionEl {
    keyword: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    settings: Option<SetField<PrimField<String>>>,
}
impl NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleElRuleOptionEl {
    #[doc = "Set the field `settings`.\n"]
    pub fn set_settings(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.settings = Some(v.into());
        self
    }
}
impl ToListMappable for NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleElRuleOptionEl {
    type O =
        BlockAssignable<NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleElRuleOptionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleElRuleOptionEl {
    #[doc = ""]
    pub keyword: PrimField<String>,
}
impl BuildNetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleElRuleOptionEl {
    pub fn build(
        self,
    ) -> NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleElRuleOptionEl {
        NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleElRuleOptionEl {
            keyword: self.keyword,
            settings: core::default::Default::default(),
        }
    }
}
pub struct NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleElRuleOptionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleElRuleOptionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleElRuleOptionElRef {
        NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleElRuleOptionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleElRuleOptionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `keyword` after provisioning.\n"]
    pub fn keyword(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.keyword", self.base))
    }
    #[doc = "Get a reference to the value of field `settings` after provisioning.\n"]
    pub fn settings(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.settings", self.base))
    }
}
#[derive(Serialize, Default)]
struct NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleElDynamic {
    header: Option<
        DynamicBlock<NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleElHeaderEl>,
    >,
    rule_option: Option<
        DynamicBlock<NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleElRuleOptionEl>,
    >,
}
#[derive(Serialize)]
pub struct NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleEl {
    action: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    header: Option<Vec<NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleElHeaderEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rule_option:
        Option<Vec<NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleElRuleOptionEl>>,
    dynamic: NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleElDynamic,
}
impl NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleEl {
    #[doc = "Set the field `header`.\n"]
    pub fn set_header(
        mut self,
        v: impl Into<
            BlockAssignable<NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleElHeaderEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.header = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.header = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `rule_option`.\n"]
    pub fn set_rule_option(
        mut self,
        v: impl Into<
            BlockAssignable<
                NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleElRuleOptionEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.rule_option = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.rule_option = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleEl {
    type O = BlockAssignable<NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleEl {
    #[doc = ""]
    pub action: PrimField<String>,
}
impl BuildNetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleEl {
    pub fn build(self) -> NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleEl {
        NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleEl {
            action: self.action,
            header: core::default::Default::default(),
            rule_option: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleElRef {
        NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }
    #[doc = "Get a reference to the value of field `header` after provisioning.\n"]
    pub fn header(
        &self,
    ) -> ListRef<NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleElHeaderElRef> {
        ListRef::new(self.shared().clone(), format!("{}.header", self.base))
    }
}
#[derive(Serialize)]
pub struct NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionElPublishMetricActionElDimensionEl
{
    value: PrimField<String>,
}
impl NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionElPublishMetricActionElDimensionEl { }
impl ToListMappable for NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionElPublishMetricActionElDimensionEl { type O = BlockAssignable < NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionElPublishMetricActionElDimensionEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildNetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionElPublishMetricActionElDimensionEl
{
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildNetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionElPublishMetricActionElDimensionEl { pub fn build (self) -> NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionElPublishMetricActionElDimensionEl { NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionElPublishMetricActionElDimensionEl { value : self . value , } } }
pub struct NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionElPublishMetricActionElDimensionElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionElPublishMetricActionElDimensionElRef { fn new (shared : StackShared , base : String) -> NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionElPublishMetricActionElDimensionElRef { NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionElPublishMetricActionElDimensionElRef { shared : shared , base : base . to_string () , } } }
impl NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionElPublishMetricActionElDimensionElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize, Default)]
struct NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionElPublishMetricActionElDynamic { dimension : Option < DynamicBlock < NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionElPublishMetricActionElDimensionEl >> , }
#[derive(Serialize)]
pub struct NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionElPublishMetricActionEl { # [serde (skip_serializing_if = "Option::is_none")] dimension : Option < Vec < NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionElPublishMetricActionElDimensionEl > > , dynamic : NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionElPublishMetricActionElDynamic , }
impl NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionElPublishMetricActionEl { # [doc = "Set the field `dimension`.\n"] pub fn set_dimension (mut self , v : impl Into < BlockAssignable < NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionElPublishMetricActionElDimensionEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . dimension = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . dimension = Some (d) ; } } self } }
impl ToListMappable for NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionElPublishMetricActionEl { type O = BlockAssignable < NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionElPublishMetricActionEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildNetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionElPublishMetricActionEl
{}
impl BuildNetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionElPublishMetricActionEl { pub fn build (self) -> NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionElPublishMetricActionEl { NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionElPublishMetricActionEl { dimension : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionElPublishMetricActionElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionElPublishMetricActionElRef { fn new (shared : StackShared , base : String) -> NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionElPublishMetricActionElRef { NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionElPublishMetricActionElRef { shared : shared , base : base . to_string () , } } }
impl NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionElPublishMetricActionElRef { fn shared (& self) -> & StackShared { & self . shared } }
#[derive(Serialize, Default)]
struct NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionElDynamic { publish_metric_action : Option < DynamicBlock < NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionElPublishMetricActionEl >> , }
#[derive(Serialize)]
pub struct NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionEl { # [serde (skip_serializing_if = "Option::is_none")] publish_metric_action : Option < Vec < NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionElPublishMetricActionEl > > , dynamic : NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionElDynamic , }
impl NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionEl { # [doc = "Set the field `publish_metric_action`.\n"] pub fn set_publish_metric_action (mut self , v : impl Into < BlockAssignable < NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionElPublishMetricActionEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . publish_metric_action = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . publish_metric_action = Some (d) ; } } self } }
impl ToListMappable for NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionEl { type O = BlockAssignable < NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildNetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionEl
{}
impl BuildNetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionEl { pub fn build (self) -> NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionEl { NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionEl { publish_metric_action : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionElRef { fn new (shared : StackShared , base : String) -> NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionElRef { NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionElRef { shared : shared , base : base . to_string () , } } }
impl NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `publish_metric_action` after provisioning.\n"] pub fn publish_metric_action (& self) -> ListRef < NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionElPublishMetricActionElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.publish_metric_action" , self . base)) } }
#[derive(Serialize, Default)]
struct NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElDynamic { action_definition : Option < DynamicBlock < NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionEl >> , }
#[derive(Serialize)]
pub struct NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionEl { action_name : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] action_definition : Option < Vec < NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionEl > > , dynamic : NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElDynamic , }
impl
    NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionEl
{
    #[doc = "Set the field `action_definition`.\n"]
    pub fn set_action_definition(
        mut self,
        v : impl Into < BlockAssignable < NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.action_definition = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.action_definition = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionEl { type O = BlockAssignable < NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildNetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionEl
{
    #[doc = ""]
    pub action_name: PrimField<String>,
}
impl BuildNetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionEl { pub fn build (self) -> NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionEl { NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionEl { action_name : self . action_name , action_definition : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElRef { fn new (shared : StackShared , base : String) -> NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElRef { NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElRef { shared : shared , base : base . to_string () , } } }
impl NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `action_name` after provisioning.\n"] pub fn action_name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.action_name" , self . base)) } # [doc = "Get a reference to the value of field `action_definition` after provisioning.\n"] pub fn action_definition (& self) -> ListRef < NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.action_definition" , self . base)) } }
#[derive(Serialize)]
pub struct NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElDestinationEl
{
    address_definition: PrimField<String>,
}
impl NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElDestinationEl { }
impl ToListMappable for NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElDestinationEl { type O = BlockAssignable < NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElDestinationEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildNetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElDestinationEl
{
    #[doc = ""]
    pub address_definition: PrimField<String>,
}
impl BuildNetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElDestinationEl { pub fn build (self) -> NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElDestinationEl { NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElDestinationEl { address_definition : self . address_definition , } } }
pub struct NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElDestinationElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElDestinationElRef { fn new (shared : StackShared , base : String) -> NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElDestinationElRef { NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElDestinationElRef { shared : shared , base : base . to_string () , } } }
impl NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElDestinationElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `address_definition` after provisioning.\n"] pub fn address_definition (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.address_definition" , self . base)) } }
#[derive(Serialize)]
pub struct NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElDestinationPortEl
{
    from_port: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    to_port: Option<PrimField<f64>>,
}
impl NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElDestinationPortEl { # [doc = "Set the field `to_port`.\n"] pub fn set_to_port (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . to_port = Some (v . into ()) ; self } }
impl ToListMappable for NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElDestinationPortEl { type O = BlockAssignable < NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElDestinationPortEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildNetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElDestinationPortEl
{
    #[doc = ""]
    pub from_port: PrimField<f64>,
}
impl BuildNetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElDestinationPortEl { pub fn build (self) -> NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElDestinationPortEl { NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElDestinationPortEl { from_port : self . from_port , to_port : core :: default :: Default :: default () , } } }
pub struct NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElDestinationPortElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElDestinationPortElRef { fn new (shared : StackShared , base : String) -> NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElDestinationPortElRef { NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElDestinationPortElRef { shared : shared , base : base . to_string () , } } }
impl NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElDestinationPortElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `from_port` after provisioning.\n"] pub fn from_port (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.from_port" , self . base)) } # [doc = "Get a reference to the value of field `to_port` after provisioning.\n"] pub fn to_port (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.to_port" , self . base)) } }
#[derive(Serialize)]
pub struct NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElSourceEl
{
    address_definition: PrimField<String>,
}
impl NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElSourceEl { }
impl ToListMappable for NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElSourceEl { type O = BlockAssignable < NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElSourceEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildNetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElSourceEl
{
    #[doc = ""]
    pub address_definition: PrimField<String>,
}
impl BuildNetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElSourceEl { pub fn build (self) -> NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElSourceEl { NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElSourceEl { address_definition : self . address_definition , } } }
pub struct NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElSourceElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElSourceElRef { fn new (shared : StackShared , base : String) -> NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElSourceElRef { NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElSourceElRef { shared : shared , base : base . to_string () , } } }
impl NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElSourceElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `address_definition` after provisioning.\n"] pub fn address_definition (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.address_definition" , self . base)) } }
#[derive(Serialize)]
pub struct NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElSourcePortEl
{
    from_port: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    to_port: Option<PrimField<f64>>,
}
impl NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElSourcePortEl { # [doc = "Set the field `to_port`.\n"] pub fn set_to_port (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . to_port = Some (v . into ()) ; self } }
impl ToListMappable for NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElSourcePortEl { type O = BlockAssignable < NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElSourcePortEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildNetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElSourcePortEl
{
    #[doc = ""]
    pub from_port: PrimField<f64>,
}
impl BuildNetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElSourcePortEl { pub fn build (self) -> NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElSourcePortEl { NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElSourcePortEl { from_port : self . from_port , to_port : core :: default :: Default :: default () , } } }
pub struct NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElSourcePortElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElSourcePortElRef { fn new (shared : StackShared , base : String) -> NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElSourcePortElRef { NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElSourcePortElRef { shared : shared , base : base . to_string () , } } }
impl NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElSourcePortElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `from_port` after provisioning.\n"] pub fn from_port (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.from_port" , self . base)) } # [doc = "Get a reference to the value of field `to_port` after provisioning.\n"] pub fn to_port (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.to_port" , self . base)) } }
#[derive(Serialize)]
pub struct NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElTcpFlagEl
{
    flags: SetField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    masks: Option<SetField<PrimField<String>>>,
}
impl NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElTcpFlagEl { # [doc = "Set the field `masks`.\n"] pub fn set_masks (mut self , v : impl Into < SetField < PrimField < String > > >) -> Self { self . masks = Some (v . into ()) ; self } }
impl ToListMappable for NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElTcpFlagEl { type O = BlockAssignable < NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElTcpFlagEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildNetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElTcpFlagEl
{
    #[doc = ""]
    pub flags: SetField<PrimField<String>>,
}
impl BuildNetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElTcpFlagEl { pub fn build (self) -> NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElTcpFlagEl { NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElTcpFlagEl { flags : self . flags , masks : core :: default :: Default :: default () , } } }
pub struct NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElTcpFlagElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElTcpFlagElRef { fn new (shared : StackShared , base : String) -> NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElTcpFlagElRef { NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElTcpFlagElRef { shared : shared , base : base . to_string () , } } }
impl NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElTcpFlagElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `flags` after provisioning.\n"] pub fn flags (& self) -> SetRef < PrimExpr < String > > { SetRef :: new (self . shared () . clone () , format ! ("{}.flags" , self . base)) } # [doc = "Get a reference to the value of field `masks` after provisioning.\n"] pub fn masks (& self) -> SetRef < PrimExpr < String > > { SetRef :: new (self . shared () . clone () , format ! ("{}.masks" , self . base)) } }
#[derive(Serialize, Default)]
struct NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElDynamic { destination : Option < DynamicBlock < NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElDestinationEl >> , destination_port : Option < DynamicBlock < NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElDestinationPortEl >> , source : Option < DynamicBlock < NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElSourceEl >> , source_port : Option < DynamicBlock < NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElSourcePortEl >> , tcp_flag : Option < DynamicBlock < NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElTcpFlagEl >> , }
#[derive(Serialize)]
pub struct NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesEl { # [serde (skip_serializing_if = "Option::is_none")] protocols : Option < SetField < PrimField < f64 > > > , # [serde (skip_serializing_if = "Option::is_none")] destination : Option < Vec < NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElDestinationEl > > , # [serde (skip_serializing_if = "Option::is_none")] destination_port : Option < Vec < NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElDestinationPortEl > > , # [serde (skip_serializing_if = "Option::is_none")] source : Option < Vec < NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElSourceEl > > , # [serde (skip_serializing_if = "Option::is_none")] source_port : Option < Vec < NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElSourcePortEl > > , # [serde (skip_serializing_if = "Option::is_none")] tcp_flag : Option < Vec < NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElTcpFlagEl > > , dynamic : NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElDynamic , }
impl NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesEl { # [doc = "Set the field `protocols`.\n"] pub fn set_protocols (mut self , v : impl Into < SetField < PrimField < f64 > > >) -> Self { self . protocols = Some (v . into ()) ; self } # [doc = "Set the field `destination`.\n"] pub fn set_destination (mut self , v : impl Into < BlockAssignable < NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElDestinationEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . destination = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . destination = Some (d) ; } } self } # [doc = "Set the field `destination_port`.\n"] pub fn set_destination_port (mut self , v : impl Into < BlockAssignable < NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElDestinationPortEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . destination_port = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . destination_port = Some (d) ; } } self } # [doc = "Set the field `source`.\n"] pub fn set_source (mut self , v : impl Into < BlockAssignable < NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElSourceEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . source = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . source = Some (d) ; } } self } # [doc = "Set the field `source_port`.\n"] pub fn set_source_port (mut self , v : impl Into < BlockAssignable < NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElSourcePortEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . source_port = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . source_port = Some (d) ; } } self } # [doc = "Set the field `tcp_flag`.\n"] pub fn set_tcp_flag (mut self , v : impl Into < BlockAssignable < NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElTcpFlagEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . tcp_flag = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . tcp_flag = Some (d) ; } } self } }
impl ToListMappable for NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesEl { type O = BlockAssignable < NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildNetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesEl
{}
impl BuildNetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesEl { pub fn build (self) -> NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesEl { NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesEl { protocols : core :: default :: Default :: default () , destination : core :: default :: Default :: default () , destination_port : core :: default :: Default :: default () , source : core :: default :: Default :: default () , source_port : core :: default :: Default :: default () , tcp_flag : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElRef { fn new (shared : StackShared , base : String) -> NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElRef { NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElRef { shared : shared , base : base . to_string () , } } }
impl NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `protocols` after provisioning.\n"] pub fn protocols (& self) -> SetRef < PrimExpr < f64 > > { SetRef :: new (self . shared () . clone () , format ! ("{}.protocols" , self . base)) } }
#[derive(Serialize, Default)]
struct NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElDynamic { match_attributes : Option < DynamicBlock < NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesEl >> , }
#[derive(Serialize)]
pub struct NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionEl { actions : SetField < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] match_attributes : Option < Vec < NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesEl > > , dynamic : NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElDynamic , }
impl NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionEl { # [doc = "Set the field `match_attributes`.\n"] pub fn set_match_attributes (mut self , v : impl Into < BlockAssignable < NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . match_attributes = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . match_attributes = Some (d) ; } } self } }
impl ToListMappable for NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionEl { type O = BlockAssignable < NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildNetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionEl
{
    #[doc = ""]
    pub actions: SetField<PrimField<String>>,
}
impl BuildNetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionEl { pub fn build (self) -> NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionEl { NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionEl { actions : self . actions , match_attributes : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElRef { fn new (shared : StackShared , base : String) -> NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElRef { NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElRef { shared : shared , base : base . to_string () , } } }
impl NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `actions` after provisioning.\n"] pub fn actions (& self) -> SetRef < PrimExpr < String > > { SetRef :: new (self . shared () . clone () , format ! ("{}.actions" , self . base)) } # [doc = "Get a reference to the value of field `match_attributes` after provisioning.\n"] pub fn match_attributes (& self) -> ListRef < NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.match_attributes" , self . base)) } }
#[derive(Serialize, Default)]
struct NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElDynamic { rule_definition : Option < DynamicBlock < NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionEl >> , }
#[derive(Serialize)]
pub struct NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleEl { priority : PrimField < f64 > , # [serde (skip_serializing_if = "Option::is_none")] rule_definition : Option < Vec < NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionEl > > , dynamic : NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElDynamic , }
impl
    NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleEl
{
    #[doc = "Set the field `rule_definition`.\n"]
    pub fn set_rule_definition(
        mut self,
        v : impl Into < BlockAssignable < NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.rule_definition = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.rule_definition = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleEl { type O = BlockAssignable < NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildNetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleEl
{
    #[doc = ""]
    pub priority: PrimField<f64>,
}
impl BuildNetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleEl { pub fn build (self) -> NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleEl { NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleEl { priority : self . priority , rule_definition : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRef { fn new (shared : StackShared , base : String) -> NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRef { NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRef { shared : shared , base : base . to_string () , } } }
impl NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `priority` after provisioning.\n"] pub fn priority (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.priority" , self . base)) } # [doc = "Get a reference to the value of field `rule_definition` after provisioning.\n"] pub fn rule_definition (& self) -> ListRef < NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.rule_definition" , self . base)) } }
#[derive(Serialize, Default)]
struct NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElDynamic { custom_action : Option < DynamicBlock < NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionEl >> , stateless_rule : Option < DynamicBlock < NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleEl >> , }
#[derive(Serialize)]
pub struct NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsEl { # [serde (skip_serializing_if = "Option::is_none")] custom_action : Option < Vec < NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionEl > > , # [serde (skip_serializing_if = "Option::is_none")] stateless_rule : Option < Vec < NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleEl > > , dynamic : NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElDynamic , }
impl NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsEl {
    #[doc = "Set the field `custom_action`.\n"]
    pub fn set_custom_action(
        mut self,
        v : impl Into < BlockAssignable < NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.custom_action = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_action = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `stateless_rule`.\n"]
    pub fn set_stateless_rule(
        mut self,
        v : impl Into < BlockAssignable < NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.stateless_rule = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.stateless_rule = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsEl
{
    type O = BlockAssignable<
        NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsEl {
}
impl BuildNetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsEl {
    pub fn build(
        self,
    ) -> NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsEl {
        NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsEl {
            custom_action: core::default::Default::default(),
            stateless_rule: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElRef {
        NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize, Default)]
struct NetworkfirewallRuleGroupRuleGroupElRulesSourceElDynamic {
    rules_source_list:
        Option<DynamicBlock<NetworkfirewallRuleGroupRuleGroupElRulesSourceElRulesSourceListEl>>,
    stateful_rule:
        Option<DynamicBlock<NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleEl>>,
    stateless_rules_and_custom_actions: Option<
        DynamicBlock<
            NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct NetworkfirewallRuleGroupRuleGroupElRulesSourceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    rules_string: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rules_source_list:
        Option<Vec<NetworkfirewallRuleGroupRuleGroupElRulesSourceElRulesSourceListEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stateful_rule: Option<Vec<NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stateless_rules_and_custom_actions: Option<
        Vec<NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsEl>,
    >,
    dynamic: NetworkfirewallRuleGroupRuleGroupElRulesSourceElDynamic,
}
impl NetworkfirewallRuleGroupRuleGroupElRulesSourceEl {
    #[doc = "Set the field `rules_string`.\n"]
    pub fn set_rules_string(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.rules_string = Some(v.into());
        self
    }
    #[doc = "Set the field `rules_source_list`.\n"]
    pub fn set_rules_source_list(
        mut self,
        v: impl Into<BlockAssignable<NetworkfirewallRuleGroupRuleGroupElRulesSourceElRulesSourceListEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.rules_source_list = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.rules_source_list = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `stateful_rule`.\n"]
    pub fn set_stateful_rule(
        mut self,
        v: impl Into<BlockAssignable<NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.stateful_rule = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.stateful_rule = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `stateless_rules_and_custom_actions`.\n"]
    pub fn set_stateless_rules_and_custom_actions(
        mut self,
        v: impl Into<
            BlockAssignable<
                NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.stateless_rules_and_custom_actions = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.stateless_rules_and_custom_actions = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for NetworkfirewallRuleGroupRuleGroupElRulesSourceEl {
    type O = BlockAssignable<NetworkfirewallRuleGroupRuleGroupElRulesSourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkfirewallRuleGroupRuleGroupElRulesSourceEl {}
impl BuildNetworkfirewallRuleGroupRuleGroupElRulesSourceEl {
    pub fn build(self) -> NetworkfirewallRuleGroupRuleGroupElRulesSourceEl {
        NetworkfirewallRuleGroupRuleGroupElRulesSourceEl {
            rules_string: core::default::Default::default(),
            rules_source_list: core::default::Default::default(),
            stateful_rule: core::default::Default::default(),
            stateless_rules_and_custom_actions: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct NetworkfirewallRuleGroupRuleGroupElRulesSourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkfirewallRuleGroupRuleGroupElRulesSourceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkfirewallRuleGroupRuleGroupElRulesSourceElRef {
        NetworkfirewallRuleGroupRuleGroupElRulesSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkfirewallRuleGroupRuleGroupElRulesSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `rules_string` after provisioning.\n"]
    pub fn rules_string(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.rules_string", self.base))
    }
    #[doc = "Get a reference to the value of field `rules_source_list` after provisioning.\n"]
    pub fn rules_source_list(
        &self,
    ) -> ListRef<NetworkfirewallRuleGroupRuleGroupElRulesSourceElRulesSourceListElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rules_source_list", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `stateful_rule` after provisioning.\n"]
    pub fn stateful_rule(
        &self,
    ) -> ListRef<NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.stateful_rule", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `stateless_rules_and_custom_actions` after provisioning.\n"]
    pub fn stateless_rules_and_custom_actions(
        &self,
    ) -> ListRef<NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.stateless_rules_and_custom_actions", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct NetworkfirewallRuleGroupRuleGroupElStatefulRuleOptionsEl {
    rule_order: PrimField<String>,
}
impl NetworkfirewallRuleGroupRuleGroupElStatefulRuleOptionsEl {}
impl ToListMappable for NetworkfirewallRuleGroupRuleGroupElStatefulRuleOptionsEl {
    type O = BlockAssignable<NetworkfirewallRuleGroupRuleGroupElStatefulRuleOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkfirewallRuleGroupRuleGroupElStatefulRuleOptionsEl {
    #[doc = ""]
    pub rule_order: PrimField<String>,
}
impl BuildNetworkfirewallRuleGroupRuleGroupElStatefulRuleOptionsEl {
    pub fn build(self) -> NetworkfirewallRuleGroupRuleGroupElStatefulRuleOptionsEl {
        NetworkfirewallRuleGroupRuleGroupElStatefulRuleOptionsEl {
            rule_order: self.rule_order,
        }
    }
}
pub struct NetworkfirewallRuleGroupRuleGroupElStatefulRuleOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkfirewallRuleGroupRuleGroupElStatefulRuleOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkfirewallRuleGroupRuleGroupElStatefulRuleOptionsElRef {
        NetworkfirewallRuleGroupRuleGroupElStatefulRuleOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkfirewallRuleGroupRuleGroupElStatefulRuleOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `rule_order` after provisioning.\n"]
    pub fn rule_order(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.rule_order", self.base))
    }
}
#[derive(Serialize, Default)]
struct NetworkfirewallRuleGroupRuleGroupElDynamic {
    reference_sets: Option<DynamicBlock<NetworkfirewallRuleGroupRuleGroupElReferenceSetsEl>>,
    rule_variables: Option<DynamicBlock<NetworkfirewallRuleGroupRuleGroupElRuleVariablesEl>>,
    rules_source: Option<DynamicBlock<NetworkfirewallRuleGroupRuleGroupElRulesSourceEl>>,
    stateful_rule_options:
        Option<DynamicBlock<NetworkfirewallRuleGroupRuleGroupElStatefulRuleOptionsEl>>,
}
#[derive(Serialize)]
pub struct NetworkfirewallRuleGroupRuleGroupEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    reference_sets: Option<Vec<NetworkfirewallRuleGroupRuleGroupElReferenceSetsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rule_variables: Option<Vec<NetworkfirewallRuleGroupRuleGroupElRuleVariablesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rules_source: Option<Vec<NetworkfirewallRuleGroupRuleGroupElRulesSourceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stateful_rule_options: Option<Vec<NetworkfirewallRuleGroupRuleGroupElStatefulRuleOptionsEl>>,
    dynamic: NetworkfirewallRuleGroupRuleGroupElDynamic,
}
impl NetworkfirewallRuleGroupRuleGroupEl {
    #[doc = "Set the field `reference_sets`.\n"]
    pub fn set_reference_sets(
        mut self,
        v: impl Into<BlockAssignable<NetworkfirewallRuleGroupRuleGroupElReferenceSetsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.reference_sets = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.reference_sets = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `rule_variables`.\n"]
    pub fn set_rule_variables(
        mut self,
        v: impl Into<BlockAssignable<NetworkfirewallRuleGroupRuleGroupElRuleVariablesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.rule_variables = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.rule_variables = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `rules_source`.\n"]
    pub fn set_rules_source(
        mut self,
        v: impl Into<BlockAssignable<NetworkfirewallRuleGroupRuleGroupElRulesSourceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.rules_source = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.rules_source = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `stateful_rule_options`.\n"]
    pub fn set_stateful_rule_options(
        mut self,
        v: impl Into<BlockAssignable<NetworkfirewallRuleGroupRuleGroupElStatefulRuleOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.stateful_rule_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.stateful_rule_options = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for NetworkfirewallRuleGroupRuleGroupEl {
    type O = BlockAssignable<NetworkfirewallRuleGroupRuleGroupEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkfirewallRuleGroupRuleGroupEl {}
impl BuildNetworkfirewallRuleGroupRuleGroupEl {
    pub fn build(self) -> NetworkfirewallRuleGroupRuleGroupEl {
        NetworkfirewallRuleGroupRuleGroupEl {
            reference_sets: core::default::Default::default(),
            rule_variables: core::default::Default::default(),
            rules_source: core::default::Default::default(),
            stateful_rule_options: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct NetworkfirewallRuleGroupRuleGroupElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkfirewallRuleGroupRuleGroupElRef {
    fn new(shared: StackShared, base: String) -> NetworkfirewallRuleGroupRuleGroupElRef {
        NetworkfirewallRuleGroupRuleGroupElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkfirewallRuleGroupRuleGroupElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `reference_sets` after provisioning.\n"]
    pub fn reference_sets(&self) -> ListRef<NetworkfirewallRuleGroupRuleGroupElReferenceSetsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.reference_sets", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `rule_variables` after provisioning.\n"]
    pub fn rule_variables(&self) -> ListRef<NetworkfirewallRuleGroupRuleGroupElRuleVariablesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rule_variables", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `rules_source` after provisioning.\n"]
    pub fn rules_source(&self) -> ListRef<NetworkfirewallRuleGroupRuleGroupElRulesSourceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.rules_source", self.base))
    }
    #[doc = "Get a reference to the value of field `stateful_rule_options` after provisioning.\n"]
    pub fn stateful_rule_options(
        &self,
    ) -> ListRef<NetworkfirewallRuleGroupRuleGroupElStatefulRuleOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.stateful_rule_options", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct NetworkfirewallRuleGroupDynamic {
    encryption_configuration:
        Option<DynamicBlock<NetworkfirewallRuleGroupEncryptionConfigurationEl>>,
    rule_group: Option<DynamicBlock<NetworkfirewallRuleGroupRuleGroupEl>>,
}
