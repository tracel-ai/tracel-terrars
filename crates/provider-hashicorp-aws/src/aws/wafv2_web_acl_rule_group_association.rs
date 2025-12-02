use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct Wafv2WebAclRuleGroupAssociationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    override_action: Option<PrimField<String>>,
    priority: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    rule_name: PrimField<String>,
    web_acl_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    managed_rule_group: Option<Vec<Wafv2WebAclRuleGroupAssociationManagedRuleGroupEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rule_group_reference: Option<Vec<Wafv2WebAclRuleGroupAssociationRuleGroupReferenceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<Wafv2WebAclRuleGroupAssociationTimeoutsEl>,
    dynamic: Wafv2WebAclRuleGroupAssociationDynamic,
}
struct Wafv2WebAclRuleGroupAssociation_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<Wafv2WebAclRuleGroupAssociationData>,
}
#[derive(Clone)]
pub struct Wafv2WebAclRuleGroupAssociation(Rc<Wafv2WebAclRuleGroupAssociation_>);
impl Wafv2WebAclRuleGroupAssociation {
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
    #[doc = "Set the field `override_action`.\nOverride action for the rule group. Valid values are 'none' and 'count'. Defaults to 'none'."]
    pub fn set_override_action(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().override_action = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `managed_rule_group`.\n"]
    pub fn set_managed_rule_group(
        self,
        v: impl Into<BlockAssignable<Wafv2WebAclRuleGroupAssociationManagedRuleGroupEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().managed_rule_group = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.managed_rule_group = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `rule_group_reference`.\n"]
    pub fn set_rule_group_reference(
        self,
        v: impl Into<BlockAssignable<Wafv2WebAclRuleGroupAssociationRuleGroupReferenceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rule_group_reference = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rule_group_reference = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<Wafv2WebAclRuleGroupAssociationTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `override_action` after provisioning.\nOverride action for the rule group. Valid values are 'none' and 'count'. Defaults to 'none'."]
    pub fn override_action(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.override_action", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `priority` after provisioning.\nPriority of the rule within the Web ACL."]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.priority", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rule_name` after provisioning.\nName of the rule to create in the Web ACL that references the rule group."]
    pub fn rule_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rule_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `web_acl_arn` after provisioning.\nARN of the Web ACL to associate the Rule Group with."]
    pub fn web_acl_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.web_acl_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `managed_rule_group` after provisioning.\n"]
    pub fn managed_rule_group(
        &self,
    ) -> ListRef<Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.managed_rule_group", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rule_group_reference` after provisioning.\n"]
    pub fn rule_group_reference(
        &self,
    ) -> ListRef<Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rule_group_reference", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Wafv2WebAclRuleGroupAssociationTimeoutsElRef {
        Wafv2WebAclRuleGroupAssociationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for Wafv2WebAclRuleGroupAssociation {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for Wafv2WebAclRuleGroupAssociation {}
impl ToListMappable for Wafv2WebAclRuleGroupAssociation {
    type O = ListRef<Wafv2WebAclRuleGroupAssociationRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for Wafv2WebAclRuleGroupAssociation_ {
    fn extract_resource_type(&self) -> String {
        "aws_wafv2_web_acl_rule_group_association".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildWafv2WebAclRuleGroupAssociation {
    pub tf_id: String,
    #[doc = "Priority of the rule within the Web ACL."]
    pub priority: PrimField<f64>,
    #[doc = "Name of the rule to create in the Web ACL that references the rule group."]
    pub rule_name: PrimField<String>,
    #[doc = "ARN of the Web ACL to associate the Rule Group with."]
    pub web_acl_arn: PrimField<String>,
}
impl BuildWafv2WebAclRuleGroupAssociation {
    pub fn build(self, stack: &mut Stack) -> Wafv2WebAclRuleGroupAssociation {
        let out = Wafv2WebAclRuleGroupAssociation(Rc::new(Wafv2WebAclRuleGroupAssociation_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(Wafv2WebAclRuleGroupAssociationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                override_action: core::default::Default::default(),
                priority: self.priority,
                region: core::default::Default::default(),
                rule_name: self.rule_name,
                web_acl_arn: self.web_acl_arn,
                managed_rule_group: core::default::Default::default(),
                rule_group_reference: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct Wafv2WebAclRuleGroupAssociationRef {
    shared: StackShared,
    base: String,
}
impl Ref for Wafv2WebAclRuleGroupAssociationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl Wafv2WebAclRuleGroupAssociationRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `override_action` after provisioning.\nOverride action for the rule group. Valid values are 'none' and 'count'. Defaults to 'none'."]
    pub fn override_action(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.override_action", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `priority` after provisioning.\nPriority of the rule within the Web ACL."]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.priority", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rule_name` after provisioning.\nName of the rule to create in the Web ACL that references the rule group."]
    pub fn rule_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rule_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `web_acl_arn` after provisioning.\nARN of the Web ACL to associate the Rule Group with."]
    pub fn web_acl_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.web_acl_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `managed_rule_group` after provisioning.\n"]
    pub fn managed_rule_group(
        &self,
    ) -> ListRef<Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.managed_rule_group", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rule_group_reference` after provisioning.\n"]
    pub fn rule_group_reference(
        &self,
    ) -> ListRef<Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rule_group_reference", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Wafv2WebAclRuleGroupAssociationTimeoutsElRef {
        Wafv2WebAclRuleGroupAssociationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElAllowElCustomRequestHandlingElInsertHeaderEl
{
    name: PrimField<String>,
    value: PrimField<String>,
}
impl Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElAllowElCustomRequestHandlingElInsertHeaderEl { }
impl ToListMappable for Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElAllowElCustomRequestHandlingElInsertHeaderEl { type O = BlockAssignable < Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElAllowElCustomRequestHandlingElInsertHeaderEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildWafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElAllowElCustomRequestHandlingElInsertHeaderEl
{
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildWafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElAllowElCustomRequestHandlingElInsertHeaderEl { pub fn build (self) -> Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElAllowElCustomRequestHandlingElInsertHeaderEl { Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElAllowElCustomRequestHandlingElInsertHeaderEl { name : self . name , value : self . value , } } }
pub struct Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElAllowElCustomRequestHandlingElInsertHeaderElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElAllowElCustomRequestHandlingElInsertHeaderElRef { fn new (shared : StackShared , base : String) -> Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElAllowElCustomRequestHandlingElInsertHeaderElRef { Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElAllowElCustomRequestHandlingElInsertHeaderElRef { shared : shared , base : base . to_string () , } } }
impl Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElAllowElCustomRequestHandlingElInsertHeaderElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `name` after provisioning.\n"] pub fn name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.name" , self . base)) } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize, Default)]
struct Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElAllowElCustomRequestHandlingElDynamic { insert_header : Option < DynamicBlock < Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElAllowElCustomRequestHandlingElInsertHeaderEl >> , }
#[derive(Serialize)]
pub struct Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElAllowElCustomRequestHandlingEl { # [serde (skip_serializing_if = "Option::is_none")] insert_header : Option < Vec < Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElAllowElCustomRequestHandlingElInsertHeaderEl > > , dynamic : Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElAllowElCustomRequestHandlingElDynamic , }
impl Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElAllowElCustomRequestHandlingEl { # [doc = "Set the field `insert_header`.\n"] pub fn set_insert_header (mut self , v : impl Into < BlockAssignable < Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElAllowElCustomRequestHandlingElInsertHeaderEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . insert_header = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . insert_header = Some (d) ; } } self } }
impl ToListMappable for Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElAllowElCustomRequestHandlingEl { type O = BlockAssignable < Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElAllowElCustomRequestHandlingEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildWafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElAllowElCustomRequestHandlingEl
{}
impl BuildWafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElAllowElCustomRequestHandlingEl { pub fn build (self) -> Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElAllowElCustomRequestHandlingEl { Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElAllowElCustomRequestHandlingEl { insert_header : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElAllowElCustomRequestHandlingElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElAllowElCustomRequestHandlingElRef { fn new (shared : StackShared , base : String) -> Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElAllowElCustomRequestHandlingElRef { Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElAllowElCustomRequestHandlingElRef { shared : shared , base : base . to_string () , } } }
impl Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElAllowElCustomRequestHandlingElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `insert_header` after provisioning.\n"] pub fn insert_header (& self) -> ListRef < Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElAllowElCustomRequestHandlingElInsertHeaderElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.insert_header" , self . base)) } }
#[derive(Serialize, Default)]
struct Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElAllowElDynamic { custom_request_handling : Option < DynamicBlock < Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElAllowElCustomRequestHandlingEl >> , }
#[derive(Serialize)]
pub struct Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElAllowEl { # [serde (skip_serializing_if = "Option::is_none")] custom_request_handling : Option < Vec < Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElAllowElCustomRequestHandlingEl > > , dynamic : Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElAllowElDynamic , }
impl Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElAllowEl {
    #[doc = "Set the field `custom_request_handling`.\n"]
    pub fn set_custom_request_handling(
        mut self,
        v : impl Into < BlockAssignable < Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElAllowElCustomRequestHandlingEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.custom_request_handling = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_request_handling = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElAllowEl
{
    type O = BlockAssignable<
        Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElAllowEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildWafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElAllowEl
{}
impl
    BuildWafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElAllowEl
{
    pub fn build(
        self,
    ) -> Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElAllowEl
    {
        Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElAllowEl {
            custom_request_handling: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElAllowElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElAllowElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElAllowElRef
    {
        Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElAllowElRef { shared : shared , base : base . to_string () , }
    }
}
impl Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElAllowElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `custom_request_handling` after provisioning.\n"]    pub fn custom_request_handling (& self) -> ListRef < Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElAllowElCustomRequestHandlingElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_request_handling", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElBlockElCustomResponseElResponseHeaderEl
{
    name: PrimField<String>,
    value: PrimField<String>,
}
impl Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElBlockElCustomResponseElResponseHeaderEl { }
impl ToListMappable for Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElBlockElCustomResponseElResponseHeaderEl { type O = BlockAssignable < Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElBlockElCustomResponseElResponseHeaderEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildWafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElBlockElCustomResponseElResponseHeaderEl
{
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildWafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElBlockElCustomResponseElResponseHeaderEl { pub fn build (self) -> Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElBlockElCustomResponseElResponseHeaderEl { Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElBlockElCustomResponseElResponseHeaderEl { name : self . name , value : self . value , } } }
pub struct Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElBlockElCustomResponseElResponseHeaderElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElBlockElCustomResponseElResponseHeaderElRef { fn new (shared : StackShared , base : String) -> Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElBlockElCustomResponseElResponseHeaderElRef { Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElBlockElCustomResponseElResponseHeaderElRef { shared : shared , base : base . to_string () , } } }
impl Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElBlockElCustomResponseElResponseHeaderElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `name` after provisioning.\n"] pub fn name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.name" , self . base)) } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize, Default)]
struct Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElBlockElCustomResponseElDynamic { response_header : Option < DynamicBlock < Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElBlockElCustomResponseElResponseHeaderEl >> , }
#[derive(Serialize)]
pub struct Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElBlockElCustomResponseEl { # [serde (skip_serializing_if = "Option::is_none")] custom_response_body_key : Option < PrimField < String > > , response_code : PrimField < f64 > , # [serde (skip_serializing_if = "Option::is_none")] response_header : Option < Vec < Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElBlockElCustomResponseElResponseHeaderEl > > , dynamic : Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElBlockElCustomResponseElDynamic , }
impl Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElBlockElCustomResponseEl { # [doc = "Set the field `custom_response_body_key`.\n"] pub fn set_custom_response_body_key (mut self , v : impl Into < PrimField < String > >) -> Self { self . custom_response_body_key = Some (v . into ()) ; self } # [doc = "Set the field `response_header`.\n"] pub fn set_response_header (mut self , v : impl Into < BlockAssignable < Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElBlockElCustomResponseElResponseHeaderEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . response_header = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . response_header = Some (d) ; } } self } }
impl ToListMappable for Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElBlockElCustomResponseEl { type O = BlockAssignable < Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElBlockElCustomResponseEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildWafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElBlockElCustomResponseEl
{
    #[doc = ""]
    pub response_code: PrimField<f64>,
}
impl BuildWafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElBlockElCustomResponseEl { pub fn build (self) -> Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElBlockElCustomResponseEl { Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElBlockElCustomResponseEl { custom_response_body_key : core :: default :: Default :: default () , response_code : self . response_code , response_header : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElBlockElCustomResponseElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElBlockElCustomResponseElRef { fn new (shared : StackShared , base : String) -> Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElBlockElCustomResponseElRef { Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElBlockElCustomResponseElRef { shared : shared , base : base . to_string () , } } }
impl Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElBlockElCustomResponseElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `custom_response_body_key` after provisioning.\n"] pub fn custom_response_body_key (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.custom_response_body_key" , self . base)) } # [doc = "Get a reference to the value of field `response_code` after provisioning.\n"] pub fn response_code (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.response_code" , self . base)) } # [doc = "Get a reference to the value of field `response_header` after provisioning.\n"] pub fn response_header (& self) -> ListRef < Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElBlockElCustomResponseElResponseHeaderElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.response_header" , self . base)) } }
#[derive(Serialize, Default)]
struct Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElBlockElDynamic { custom_response : Option < DynamicBlock < Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElBlockElCustomResponseEl >> , }
#[derive(Serialize)]
pub struct Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElBlockEl { # [serde (skip_serializing_if = "Option::is_none")] custom_response : Option < Vec < Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElBlockElCustomResponseEl > > , dynamic : Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElBlockElDynamic , }
impl Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElBlockEl {
    #[doc = "Set the field `custom_response`.\n"]
    pub fn set_custom_response(
        mut self,
        v : impl Into < BlockAssignable < Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElBlockElCustomResponseEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.custom_response = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_response = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElBlockEl
{
    type O = BlockAssignable<
        Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElBlockEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildWafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElBlockEl
{}
impl
    BuildWafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElBlockEl
{
    pub fn build(
        self,
    ) -> Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElBlockEl
    {
        Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElBlockEl {
            custom_response: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElBlockElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElBlockElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElBlockElRef
    {
        Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElBlockElRef { shared : shared , base : base . to_string () , }
    }
}
impl Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElBlockElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `custom_response` after provisioning.\n"]    pub fn custom_response (& self) -> ListRef < Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElBlockElCustomResponseElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_response", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCaptchaElCustomRequestHandlingElInsertHeaderEl
{
    name: PrimField<String>,
    value: PrimField<String>,
}
impl Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCaptchaElCustomRequestHandlingElInsertHeaderEl { }
impl ToListMappable for Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCaptchaElCustomRequestHandlingElInsertHeaderEl { type O = BlockAssignable < Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCaptchaElCustomRequestHandlingElInsertHeaderEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildWafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCaptchaElCustomRequestHandlingElInsertHeaderEl
{
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildWafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCaptchaElCustomRequestHandlingElInsertHeaderEl { pub fn build (self) -> Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCaptchaElCustomRequestHandlingElInsertHeaderEl { Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCaptchaElCustomRequestHandlingElInsertHeaderEl { name : self . name , value : self . value , } } }
pub struct Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCaptchaElCustomRequestHandlingElInsertHeaderElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCaptchaElCustomRequestHandlingElInsertHeaderElRef { fn new (shared : StackShared , base : String) -> Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCaptchaElCustomRequestHandlingElInsertHeaderElRef { Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCaptchaElCustomRequestHandlingElInsertHeaderElRef { shared : shared , base : base . to_string () , } } }
impl Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCaptchaElCustomRequestHandlingElInsertHeaderElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `name` after provisioning.\n"] pub fn name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.name" , self . base)) } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize, Default)]
struct Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCaptchaElCustomRequestHandlingElDynamic { insert_header : Option < DynamicBlock < Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCaptchaElCustomRequestHandlingElInsertHeaderEl >> , }
#[derive(Serialize)]
pub struct Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCaptchaElCustomRequestHandlingEl { # [serde (skip_serializing_if = "Option::is_none")] insert_header : Option < Vec < Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCaptchaElCustomRequestHandlingElInsertHeaderEl > > , dynamic : Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCaptchaElCustomRequestHandlingElDynamic , }
impl Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCaptchaElCustomRequestHandlingEl { # [doc = "Set the field `insert_header`.\n"] pub fn set_insert_header (mut self , v : impl Into < BlockAssignable < Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCaptchaElCustomRequestHandlingElInsertHeaderEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . insert_header = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . insert_header = Some (d) ; } } self } }
impl ToListMappable for Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCaptchaElCustomRequestHandlingEl { type O = BlockAssignable < Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCaptchaElCustomRequestHandlingEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildWafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCaptchaElCustomRequestHandlingEl
{}
impl BuildWafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCaptchaElCustomRequestHandlingEl { pub fn build (self) -> Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCaptchaElCustomRequestHandlingEl { Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCaptchaElCustomRequestHandlingEl { insert_header : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCaptchaElCustomRequestHandlingElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCaptchaElCustomRequestHandlingElRef { fn new (shared : StackShared , base : String) -> Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCaptchaElCustomRequestHandlingElRef { Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCaptchaElCustomRequestHandlingElRef { shared : shared , base : base . to_string () , } } }
impl Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCaptchaElCustomRequestHandlingElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `insert_header` after provisioning.\n"] pub fn insert_header (& self) -> ListRef < Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCaptchaElCustomRequestHandlingElInsertHeaderElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.insert_header" , self . base)) } }
#[derive(Serialize, Default)]
struct Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCaptchaElDynamic { custom_request_handling : Option < DynamicBlock < Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCaptchaElCustomRequestHandlingEl >> , }
#[derive(Serialize)]
pub struct Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCaptchaEl { # [serde (skip_serializing_if = "Option::is_none")] custom_request_handling : Option < Vec < Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCaptchaElCustomRequestHandlingEl > > , dynamic : Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCaptchaElDynamic , }
impl Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCaptchaEl {
    #[doc = "Set the field `custom_request_handling`.\n"]
    pub fn set_custom_request_handling(
        mut self,
        v : impl Into < BlockAssignable < Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCaptchaElCustomRequestHandlingEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.custom_request_handling = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_request_handling = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCaptchaEl
{
    type O = BlockAssignable<
        Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCaptchaEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildWafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCaptchaEl
{}
impl
    BuildWafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCaptchaEl
{
    pub fn build(
        self,
    ) -> Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCaptchaEl
    {
        Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCaptchaEl { custom_request_handling : core :: default :: Default :: default () , dynamic : Default :: default () , }
    }
}
pub struct Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCaptchaElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCaptchaElRef { fn new (shared : StackShared , base : String) -> Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCaptchaElRef { Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCaptchaElRef { shared : shared , base : base . to_string () , } } }
impl
    Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCaptchaElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `custom_request_handling` after provisioning.\n"]    pub fn custom_request_handling (& self) -> ListRef < Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCaptchaElCustomRequestHandlingElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_request_handling", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElChallengeElCustomRequestHandlingElInsertHeaderEl
{
    name: PrimField<String>,
    value: PrimField<String>,
}
impl Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElChallengeElCustomRequestHandlingElInsertHeaderEl { }
impl ToListMappable for Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElChallengeElCustomRequestHandlingElInsertHeaderEl { type O = BlockAssignable < Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElChallengeElCustomRequestHandlingElInsertHeaderEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildWafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElChallengeElCustomRequestHandlingElInsertHeaderEl
{
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildWafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElChallengeElCustomRequestHandlingElInsertHeaderEl { pub fn build (self) -> Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElChallengeElCustomRequestHandlingElInsertHeaderEl { Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElChallengeElCustomRequestHandlingElInsertHeaderEl { name : self . name , value : self . value , } } }
pub struct Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElChallengeElCustomRequestHandlingElInsertHeaderElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElChallengeElCustomRequestHandlingElInsertHeaderElRef { fn new (shared : StackShared , base : String) -> Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElChallengeElCustomRequestHandlingElInsertHeaderElRef { Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElChallengeElCustomRequestHandlingElInsertHeaderElRef { shared : shared , base : base . to_string () , } } }
impl Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElChallengeElCustomRequestHandlingElInsertHeaderElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `name` after provisioning.\n"] pub fn name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.name" , self . base)) } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize, Default)]
struct Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElChallengeElCustomRequestHandlingElDynamic { insert_header : Option < DynamicBlock < Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElChallengeElCustomRequestHandlingElInsertHeaderEl >> , }
#[derive(Serialize)]
pub struct Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElChallengeElCustomRequestHandlingEl { # [serde (skip_serializing_if = "Option::is_none")] insert_header : Option < Vec < Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElChallengeElCustomRequestHandlingElInsertHeaderEl > > , dynamic : Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElChallengeElCustomRequestHandlingElDynamic , }
impl Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElChallengeElCustomRequestHandlingEl { # [doc = "Set the field `insert_header`.\n"] pub fn set_insert_header (mut self , v : impl Into < BlockAssignable < Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElChallengeElCustomRequestHandlingElInsertHeaderEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . insert_header = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . insert_header = Some (d) ; } } self } }
impl ToListMappable for Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElChallengeElCustomRequestHandlingEl { type O = BlockAssignable < Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElChallengeElCustomRequestHandlingEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildWafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElChallengeElCustomRequestHandlingEl
{}
impl BuildWafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElChallengeElCustomRequestHandlingEl { pub fn build (self) -> Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElChallengeElCustomRequestHandlingEl { Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElChallengeElCustomRequestHandlingEl { insert_header : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElChallengeElCustomRequestHandlingElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElChallengeElCustomRequestHandlingElRef { fn new (shared : StackShared , base : String) -> Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElChallengeElCustomRequestHandlingElRef { Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElChallengeElCustomRequestHandlingElRef { shared : shared , base : base . to_string () , } } }
impl Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElChallengeElCustomRequestHandlingElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `insert_header` after provisioning.\n"] pub fn insert_header (& self) -> ListRef < Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElChallengeElCustomRequestHandlingElInsertHeaderElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.insert_header" , self . base)) } }
#[derive(Serialize, Default)]
struct Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElChallengeElDynamic { custom_request_handling : Option < DynamicBlock < Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElChallengeElCustomRequestHandlingEl >> , }
#[derive(Serialize)]
pub struct Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElChallengeEl { # [serde (skip_serializing_if = "Option::is_none")] custom_request_handling : Option < Vec < Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElChallengeElCustomRequestHandlingEl > > , dynamic : Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElChallengeElDynamic , }
impl Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElChallengeEl {
    #[doc = "Set the field `custom_request_handling`.\n"]
    pub fn set_custom_request_handling(
        mut self,
        v : impl Into < BlockAssignable < Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElChallengeElCustomRequestHandlingEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.custom_request_handling = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_request_handling = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElChallengeEl { type O = BlockAssignable < Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElChallengeEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildWafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElChallengeEl
{}
impl BuildWafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElChallengeEl { pub fn build (self) -> Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElChallengeEl { Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElChallengeEl { custom_request_handling : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElChallengeElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElChallengeElRef { fn new (shared : StackShared , base : String) -> Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElChallengeElRef { Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElChallengeElRef { shared : shared , base : base . to_string () , } } }
impl
    Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElChallengeElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `custom_request_handling` after provisioning.\n"]    pub fn custom_request_handling (& self) -> ListRef < Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElChallengeElCustomRequestHandlingElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_request_handling", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCountElCustomRequestHandlingElInsertHeaderEl
{
    name: PrimField<String>,
    value: PrimField<String>,
}
impl Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCountElCustomRequestHandlingElInsertHeaderEl { }
impl ToListMappable for Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCountElCustomRequestHandlingElInsertHeaderEl { type O = BlockAssignable < Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCountElCustomRequestHandlingElInsertHeaderEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildWafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCountElCustomRequestHandlingElInsertHeaderEl
{
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildWafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCountElCustomRequestHandlingElInsertHeaderEl { pub fn build (self) -> Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCountElCustomRequestHandlingElInsertHeaderEl { Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCountElCustomRequestHandlingElInsertHeaderEl { name : self . name , value : self . value , } } }
pub struct Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCountElCustomRequestHandlingElInsertHeaderElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCountElCustomRequestHandlingElInsertHeaderElRef { fn new (shared : StackShared , base : String) -> Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCountElCustomRequestHandlingElInsertHeaderElRef { Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCountElCustomRequestHandlingElInsertHeaderElRef { shared : shared , base : base . to_string () , } } }
impl Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCountElCustomRequestHandlingElInsertHeaderElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `name` after provisioning.\n"] pub fn name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.name" , self . base)) } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize, Default)]
struct Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCountElCustomRequestHandlingElDynamic { insert_header : Option < DynamicBlock < Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCountElCustomRequestHandlingElInsertHeaderEl >> , }
#[derive(Serialize)]
pub struct Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCountElCustomRequestHandlingEl { # [serde (skip_serializing_if = "Option::is_none")] insert_header : Option < Vec < Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCountElCustomRequestHandlingElInsertHeaderEl > > , dynamic : Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCountElCustomRequestHandlingElDynamic , }
impl Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCountElCustomRequestHandlingEl { # [doc = "Set the field `insert_header`.\n"] pub fn set_insert_header (mut self , v : impl Into < BlockAssignable < Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCountElCustomRequestHandlingElInsertHeaderEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . insert_header = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . insert_header = Some (d) ; } } self } }
impl ToListMappable for Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCountElCustomRequestHandlingEl { type O = BlockAssignable < Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCountElCustomRequestHandlingEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildWafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCountElCustomRequestHandlingEl
{}
impl BuildWafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCountElCustomRequestHandlingEl { pub fn build (self) -> Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCountElCustomRequestHandlingEl { Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCountElCustomRequestHandlingEl { insert_header : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCountElCustomRequestHandlingElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCountElCustomRequestHandlingElRef { fn new (shared : StackShared , base : String) -> Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCountElCustomRequestHandlingElRef { Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCountElCustomRequestHandlingElRef { shared : shared , base : base . to_string () , } } }
impl Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCountElCustomRequestHandlingElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `insert_header` after provisioning.\n"] pub fn insert_header (& self) -> ListRef < Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCountElCustomRequestHandlingElInsertHeaderElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.insert_header" , self . base)) } }
#[derive(Serialize, Default)]
struct Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCountElDynamic { custom_request_handling : Option < DynamicBlock < Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCountElCustomRequestHandlingEl >> , }
#[derive(Serialize)]
pub struct Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCountEl { # [serde (skip_serializing_if = "Option::is_none")] custom_request_handling : Option < Vec < Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCountElCustomRequestHandlingEl > > , dynamic : Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCountElDynamic , }
impl Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCountEl {
    #[doc = "Set the field `custom_request_handling`.\n"]
    pub fn set_custom_request_handling(
        mut self,
        v : impl Into < BlockAssignable < Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCountElCustomRequestHandlingEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.custom_request_handling = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_request_handling = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCountEl
{
    type O = BlockAssignable<
        Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCountEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildWafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCountEl
{}
impl
    BuildWafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCountEl
{
    pub fn build(
        self,
    ) -> Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCountEl
    {
        Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCountEl {
            custom_request_handling: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCountElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCountElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCountElRef
    {
        Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCountElRef { shared : shared , base : base . to_string () , }
    }
}
impl Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCountElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `custom_request_handling` after provisioning.\n"]    pub fn custom_request_handling (& self) -> ListRef < Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCountElCustomRequestHandlingElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_request_handling", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElDynamic { allow : Option < DynamicBlock < Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElAllowEl >> , block : Option < DynamicBlock < Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElBlockEl >> , captcha : Option < DynamicBlock < Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCaptchaEl >> , challenge : Option < DynamicBlock < Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElChallengeEl >> , count : Option < DynamicBlock < Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCountEl >> , }
#[derive(Serialize)]
pub struct Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseEl { # [serde (skip_serializing_if = "Option::is_none")] allow : Option < Vec < Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElAllowEl > > , # [serde (skip_serializing_if = "Option::is_none")] block : Option < Vec < Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElBlockEl > > , # [serde (skip_serializing_if = "Option::is_none")] captcha : Option < Vec < Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCaptchaEl > > , # [serde (skip_serializing_if = "Option::is_none")] challenge : Option < Vec < Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElChallengeEl > > , # [serde (skip_serializing_if = "Option::is_none")] count : Option < Vec < Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCountEl > > , dynamic : Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElDynamic , }
impl Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseEl {
    #[doc = "Set the field `allow`.\n"]
    pub fn set_allow(
        mut self,
        v : impl Into < BlockAssignable < Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElAllowEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.allow = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.allow = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `block`.\n"]
    pub fn set_block(
        mut self,
        v : impl Into < BlockAssignable < Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElBlockEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.block = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.block = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `captcha`.\n"]
    pub fn set_captcha(
        mut self,
        v : impl Into < BlockAssignable < Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCaptchaEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.captcha = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.captcha = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `challenge`.\n"]
    pub fn set_challenge(
        mut self,
        v : impl Into < BlockAssignable < Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElChallengeEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.challenge = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.challenge = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `count`.\n"]
    pub fn set_count(
        mut self,
        v : impl Into < BlockAssignable < Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCountEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.count = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.count = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseEl
{
    type O = BlockAssignable<
        Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildWafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseEl
{}
impl BuildWafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseEl {
    pub fn build(
        self,
    ) -> Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseEl {
        Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseEl {
            allow: core::default::Default::default(),
            block: core::default::Default::default(),
            captcha: core::default::Default::default(),
            challenge: core::default::Default::default(),
            count: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElRef {
        Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allow` after provisioning.\n"]    pub fn allow (& self) -> ListRef < Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElAllowElRef >{
        ListRef::new(self.shared().clone(), format!("{}.allow", self.base))
    }
    #[doc = "Get a reference to the value of field `block` after provisioning.\n"]    pub fn block (& self) -> ListRef < Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElBlockElRef >{
        ListRef::new(self.shared().clone(), format!("{}.block", self.base))
    }
    #[doc = "Get a reference to the value of field `captcha` after provisioning.\n"]    pub fn captcha (& self) -> ListRef < Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCaptchaElRef >{
        ListRef::new(self.shared().clone(), format!("{}.captcha", self.base))
    }
    #[doc = "Get a reference to the value of field `challenge` after provisioning.\n"]    pub fn challenge (& self) -> ListRef < Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElChallengeElRef >{
        ListRef::new(self.shared().clone(), format!("{}.challenge", self.base))
    }
    #[doc = "Get a reference to the value of field `count` after provisioning.\n"]    pub fn count (& self) -> ListRef < Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElCountElRef >{
        ListRef::new(self.shared().clone(), format!("{}.count", self.base))
    }
}
#[derive(Serialize, Default)]
struct Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElDynamic {
    action_to_use: Option<
        DynamicBlock<
            Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideEl {
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    action_to_use: Option<
        Vec<Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseEl>,
    >,
    dynamic: Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElDynamic,
}
impl Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideEl {
    #[doc = "Set the field `action_to_use`.\n"]
    pub fn set_action_to_use(
        mut self,
        v: impl Into<
            BlockAssignable<
                Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.action_to_use = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.action_to_use = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideEl {
    type O = BlockAssignable<Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildWafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideEl {
    #[doc = "Name of the rule to override."]
    pub name: PrimField<String>,
}
impl BuildWafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideEl {
    pub fn build(self) -> Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideEl {
        Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideEl {
            name: self.name,
            action_to_use: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElRef {
        Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the rule to override."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `action_to_use` after provisioning.\n"]
    pub fn action_to_use(
        &self,
    ) -> ListRef<
        Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElActionToUseElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.action_to_use", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct Wafv2WebAclRuleGroupAssociationManagedRuleGroupElDynamic {
    rule_action_override:
        Option<DynamicBlock<Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideEl>>,
}
#[derive(Serialize)]
pub struct Wafv2WebAclRuleGroupAssociationManagedRuleGroupEl {
    name: PrimField<String>,
    vendor_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rule_action_override:
        Option<Vec<Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideEl>>,
    dynamic: Wafv2WebAclRuleGroupAssociationManagedRuleGroupElDynamic,
}
impl Wafv2WebAclRuleGroupAssociationManagedRuleGroupEl {
    #[doc = "Set the field `version`.\nVersion of the managed rule group. Omit this to use the default version."]
    pub fn set_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.version = Some(v.into());
        self
    }
    #[doc = "Set the field `rule_action_override`.\n"]
    pub fn set_rule_action_override(
        mut self,
        v: impl Into<
            BlockAssignable<Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.rule_action_override = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.rule_action_override = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for Wafv2WebAclRuleGroupAssociationManagedRuleGroupEl {
    type O = BlockAssignable<Wafv2WebAclRuleGroupAssociationManagedRuleGroupEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildWafv2WebAclRuleGroupAssociationManagedRuleGroupEl {
    #[doc = "Name of the managed rule group."]
    pub name: PrimField<String>,
    #[doc = "Name of the managed rule group vendor."]
    pub vendor_name: PrimField<String>,
}
impl BuildWafv2WebAclRuleGroupAssociationManagedRuleGroupEl {
    pub fn build(self) -> Wafv2WebAclRuleGroupAssociationManagedRuleGroupEl {
        Wafv2WebAclRuleGroupAssociationManagedRuleGroupEl {
            name: self.name,
            vendor_name: self.vendor_name,
            version: core::default::Default::default(),
            rule_action_override: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRef {
        Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the managed rule group."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `vendor_name` after provisioning.\nName of the managed rule group vendor."]
    pub fn vendor_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vendor_name", self.base))
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\nVersion of the managed rule group. Omit this to use the default version."]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
    #[doc = "Get a reference to the value of field `rule_action_override` after provisioning.\n"]
    pub fn rule_action_override(
        &self,
    ) -> ListRef<Wafv2WebAclRuleGroupAssociationManagedRuleGroupElRuleActionOverrideElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rule_action_override", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElAllowElCustomRequestHandlingElInsertHeaderEl
{
    name: PrimField<String>,
    value: PrimField<String>,
}
impl Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElAllowElCustomRequestHandlingElInsertHeaderEl { }
impl ToListMappable for Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElAllowElCustomRequestHandlingElInsertHeaderEl { type O = BlockAssignable < Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElAllowElCustomRequestHandlingElInsertHeaderEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildWafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElAllowElCustomRequestHandlingElInsertHeaderEl
{
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildWafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElAllowElCustomRequestHandlingElInsertHeaderEl { pub fn build (self) -> Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElAllowElCustomRequestHandlingElInsertHeaderEl { Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElAllowElCustomRequestHandlingElInsertHeaderEl { name : self . name , value : self . value , } } }
pub struct Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElAllowElCustomRequestHandlingElInsertHeaderElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElAllowElCustomRequestHandlingElInsertHeaderElRef { fn new (shared : StackShared , base : String) -> Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElAllowElCustomRequestHandlingElInsertHeaderElRef { Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElAllowElCustomRequestHandlingElInsertHeaderElRef { shared : shared , base : base . to_string () , } } }
impl Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElAllowElCustomRequestHandlingElInsertHeaderElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `name` after provisioning.\n"] pub fn name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.name" , self . base)) } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize, Default)]
struct Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElAllowElCustomRequestHandlingElDynamic { insert_header : Option < DynamicBlock < Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElAllowElCustomRequestHandlingElInsertHeaderEl >> , }
#[derive(Serialize)]
pub struct Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElAllowElCustomRequestHandlingEl { # [serde (skip_serializing_if = "Option::is_none")] insert_header : Option < Vec < Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElAllowElCustomRequestHandlingElInsertHeaderEl > > , dynamic : Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElAllowElCustomRequestHandlingElDynamic , }
impl Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElAllowElCustomRequestHandlingEl { # [doc = "Set the field `insert_header`.\n"] pub fn set_insert_header (mut self , v : impl Into < BlockAssignable < Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElAllowElCustomRequestHandlingElInsertHeaderEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . insert_header = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . insert_header = Some (d) ; } } self } }
impl ToListMappable for Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElAllowElCustomRequestHandlingEl { type O = BlockAssignable < Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElAllowElCustomRequestHandlingEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildWafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElAllowElCustomRequestHandlingEl
{}
impl BuildWafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElAllowElCustomRequestHandlingEl { pub fn build (self) -> Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElAllowElCustomRequestHandlingEl { Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElAllowElCustomRequestHandlingEl { insert_header : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElAllowElCustomRequestHandlingElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElAllowElCustomRequestHandlingElRef { fn new (shared : StackShared , base : String) -> Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElAllowElCustomRequestHandlingElRef { Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElAllowElCustomRequestHandlingElRef { shared : shared , base : base . to_string () , } } }
impl Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElAllowElCustomRequestHandlingElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `insert_header` after provisioning.\n"] pub fn insert_header (& self) -> ListRef < Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElAllowElCustomRequestHandlingElInsertHeaderElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.insert_header" , self . base)) } }
#[derive(Serialize, Default)]
struct Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElAllowElDynamic { custom_request_handling : Option < DynamicBlock < Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElAllowElCustomRequestHandlingEl >> , }
#[derive(Serialize)]
pub struct Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElAllowEl { # [serde (skip_serializing_if = "Option::is_none")] custom_request_handling : Option < Vec < Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElAllowElCustomRequestHandlingEl > > , dynamic : Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElAllowElDynamic , }
impl Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElAllowEl {
    #[doc = "Set the field `custom_request_handling`.\n"]
    pub fn set_custom_request_handling(
        mut self,
        v : impl Into < BlockAssignable < Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElAllowElCustomRequestHandlingEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.custom_request_handling = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_request_handling = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElAllowEl
{
    type O = BlockAssignable<
        Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElAllowEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildWafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElAllowEl
{}
impl
    BuildWafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElAllowEl
{
    pub fn build(
        self,
    ) -> Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElAllowEl
    {
        Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElAllowEl { custom_request_handling : core :: default :: Default :: default () , dynamic : Default :: default () , }
    }
}
pub struct Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElAllowElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElAllowElRef { fn new (shared : StackShared , base : String) -> Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElAllowElRef { Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElAllowElRef { shared : shared , base : base . to_string () , } } }
impl
    Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElAllowElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `custom_request_handling` after provisioning.\n"]    pub fn custom_request_handling (& self) -> ListRef < Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElAllowElCustomRequestHandlingElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_request_handling", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElBlockElCustomResponseElResponseHeaderEl
{
    name: PrimField<String>,
    value: PrimField<String>,
}
impl Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElBlockElCustomResponseElResponseHeaderEl { }
impl ToListMappable for Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElBlockElCustomResponseElResponseHeaderEl { type O = BlockAssignable < Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElBlockElCustomResponseElResponseHeaderEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildWafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElBlockElCustomResponseElResponseHeaderEl
{
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildWafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElBlockElCustomResponseElResponseHeaderEl { pub fn build (self) -> Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElBlockElCustomResponseElResponseHeaderEl { Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElBlockElCustomResponseElResponseHeaderEl { name : self . name , value : self . value , } } }
pub struct Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElBlockElCustomResponseElResponseHeaderElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElBlockElCustomResponseElResponseHeaderElRef { fn new (shared : StackShared , base : String) -> Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElBlockElCustomResponseElResponseHeaderElRef { Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElBlockElCustomResponseElResponseHeaderElRef { shared : shared , base : base . to_string () , } } }
impl Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElBlockElCustomResponseElResponseHeaderElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `name` after provisioning.\n"] pub fn name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.name" , self . base)) } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize, Default)]
struct Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElBlockElCustomResponseElDynamic { response_header : Option < DynamicBlock < Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElBlockElCustomResponseElResponseHeaderEl >> , }
#[derive(Serialize)]
pub struct Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElBlockElCustomResponseEl { # [serde (skip_serializing_if = "Option::is_none")] custom_response_body_key : Option < PrimField < String > > , response_code : PrimField < f64 > , # [serde (skip_serializing_if = "Option::is_none")] response_header : Option < Vec < Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElBlockElCustomResponseElResponseHeaderEl > > , dynamic : Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElBlockElCustomResponseElDynamic , }
impl Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElBlockElCustomResponseEl { # [doc = "Set the field `custom_response_body_key`.\n"] pub fn set_custom_response_body_key (mut self , v : impl Into < PrimField < String > >) -> Self { self . custom_response_body_key = Some (v . into ()) ; self } # [doc = "Set the field `response_header`.\n"] pub fn set_response_header (mut self , v : impl Into < BlockAssignable < Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElBlockElCustomResponseElResponseHeaderEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . response_header = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . response_header = Some (d) ; } } self } }
impl ToListMappable for Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElBlockElCustomResponseEl { type O = BlockAssignable < Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElBlockElCustomResponseEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildWafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElBlockElCustomResponseEl
{
    #[doc = ""]
    pub response_code: PrimField<f64>,
}
impl BuildWafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElBlockElCustomResponseEl { pub fn build (self) -> Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElBlockElCustomResponseEl { Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElBlockElCustomResponseEl { custom_response_body_key : core :: default :: Default :: default () , response_code : self . response_code , response_header : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElBlockElCustomResponseElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElBlockElCustomResponseElRef { fn new (shared : StackShared , base : String) -> Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElBlockElCustomResponseElRef { Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElBlockElCustomResponseElRef { shared : shared , base : base . to_string () , } } }
impl Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElBlockElCustomResponseElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `custom_response_body_key` after provisioning.\n"] pub fn custom_response_body_key (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.custom_response_body_key" , self . base)) } # [doc = "Get a reference to the value of field `response_code` after provisioning.\n"] pub fn response_code (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.response_code" , self . base)) } # [doc = "Get a reference to the value of field `response_header` after provisioning.\n"] pub fn response_header (& self) -> ListRef < Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElBlockElCustomResponseElResponseHeaderElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.response_header" , self . base)) } }
#[derive(Serialize, Default)]
struct Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElBlockElDynamic { custom_response : Option < DynamicBlock < Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElBlockElCustomResponseEl >> , }
#[derive(Serialize)]
pub struct Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElBlockEl { # [serde (skip_serializing_if = "Option::is_none")] custom_response : Option < Vec < Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElBlockElCustomResponseEl > > , dynamic : Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElBlockElDynamic , }
impl Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElBlockEl {
    #[doc = "Set the field `custom_response`.\n"]
    pub fn set_custom_response(
        mut self,
        v : impl Into < BlockAssignable < Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElBlockElCustomResponseEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.custom_response = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_response = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElBlockEl
{
    type O = BlockAssignable<
        Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElBlockEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildWafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElBlockEl
{}
impl
    BuildWafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElBlockEl
{
    pub fn build(
        self,
    ) -> Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElBlockEl
    {
        Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElBlockEl { custom_response : core :: default :: Default :: default () , dynamic : Default :: default () , }
    }
}
pub struct Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElBlockElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElBlockElRef { fn new (shared : StackShared , base : String) -> Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElBlockElRef { Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElBlockElRef { shared : shared , base : base . to_string () , } } }
impl
    Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElBlockElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `custom_response` after provisioning.\n"]    pub fn custom_response (& self) -> ListRef < Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElBlockElCustomResponseElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_response", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCaptchaElCustomRequestHandlingElInsertHeaderEl
{
    name: PrimField<String>,
    value: PrimField<String>,
}
impl Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCaptchaElCustomRequestHandlingElInsertHeaderEl { }
impl ToListMappable for Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCaptchaElCustomRequestHandlingElInsertHeaderEl { type O = BlockAssignable < Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCaptchaElCustomRequestHandlingElInsertHeaderEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildWafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCaptchaElCustomRequestHandlingElInsertHeaderEl
{
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildWafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCaptchaElCustomRequestHandlingElInsertHeaderEl { pub fn build (self) -> Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCaptchaElCustomRequestHandlingElInsertHeaderEl { Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCaptchaElCustomRequestHandlingElInsertHeaderEl { name : self . name , value : self . value , } } }
pub struct Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCaptchaElCustomRequestHandlingElInsertHeaderElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCaptchaElCustomRequestHandlingElInsertHeaderElRef { fn new (shared : StackShared , base : String) -> Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCaptchaElCustomRequestHandlingElInsertHeaderElRef { Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCaptchaElCustomRequestHandlingElInsertHeaderElRef { shared : shared , base : base . to_string () , } } }
impl Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCaptchaElCustomRequestHandlingElInsertHeaderElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `name` after provisioning.\n"] pub fn name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.name" , self . base)) } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize, Default)]
struct Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCaptchaElCustomRequestHandlingElDynamic { insert_header : Option < DynamicBlock < Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCaptchaElCustomRequestHandlingElInsertHeaderEl >> , }
#[derive(Serialize)]
pub struct Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCaptchaElCustomRequestHandlingEl { # [serde (skip_serializing_if = "Option::is_none")] insert_header : Option < Vec < Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCaptchaElCustomRequestHandlingElInsertHeaderEl > > , dynamic : Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCaptchaElCustomRequestHandlingElDynamic , }
impl Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCaptchaElCustomRequestHandlingEl { # [doc = "Set the field `insert_header`.\n"] pub fn set_insert_header (mut self , v : impl Into < BlockAssignable < Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCaptchaElCustomRequestHandlingElInsertHeaderEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . insert_header = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . insert_header = Some (d) ; } } self } }
impl ToListMappable for Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCaptchaElCustomRequestHandlingEl { type O = BlockAssignable < Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCaptchaElCustomRequestHandlingEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildWafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCaptchaElCustomRequestHandlingEl
{}
impl BuildWafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCaptchaElCustomRequestHandlingEl { pub fn build (self) -> Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCaptchaElCustomRequestHandlingEl { Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCaptchaElCustomRequestHandlingEl { insert_header : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCaptchaElCustomRequestHandlingElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCaptchaElCustomRequestHandlingElRef { fn new (shared : StackShared , base : String) -> Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCaptchaElCustomRequestHandlingElRef { Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCaptchaElCustomRequestHandlingElRef { shared : shared , base : base . to_string () , } } }
impl Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCaptchaElCustomRequestHandlingElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `insert_header` after provisioning.\n"] pub fn insert_header (& self) -> ListRef < Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCaptchaElCustomRequestHandlingElInsertHeaderElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.insert_header" , self . base)) } }
#[derive(Serialize, Default)]
struct Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCaptchaElDynamic { custom_request_handling : Option < DynamicBlock < Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCaptchaElCustomRequestHandlingEl >> , }
#[derive(Serialize)]
pub struct Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCaptchaEl { # [serde (skip_serializing_if = "Option::is_none")] custom_request_handling : Option < Vec < Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCaptchaElCustomRequestHandlingEl > > , dynamic : Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCaptchaElDynamic , }
impl Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCaptchaEl {
    #[doc = "Set the field `custom_request_handling`.\n"]
    pub fn set_custom_request_handling(
        mut self,
        v : impl Into < BlockAssignable < Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCaptchaElCustomRequestHandlingEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.custom_request_handling = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_request_handling = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCaptchaEl { type O = BlockAssignable < Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCaptchaEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildWafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCaptchaEl
{}
impl BuildWafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCaptchaEl { pub fn build (self) -> Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCaptchaEl { Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCaptchaEl { custom_request_handling : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCaptchaElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCaptchaElRef { fn new (shared : StackShared , base : String) -> Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCaptchaElRef { Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCaptchaElRef { shared : shared , base : base . to_string () , } } }
impl
    Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCaptchaElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `custom_request_handling` after provisioning.\n"]    pub fn custom_request_handling (& self) -> ListRef < Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCaptchaElCustomRequestHandlingElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_request_handling", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElChallengeElCustomRequestHandlingElInsertHeaderEl
{
    name: PrimField<String>,
    value: PrimField<String>,
}
impl Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElChallengeElCustomRequestHandlingElInsertHeaderEl { }
impl ToListMappable for Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElChallengeElCustomRequestHandlingElInsertHeaderEl { type O = BlockAssignable < Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElChallengeElCustomRequestHandlingElInsertHeaderEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildWafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElChallengeElCustomRequestHandlingElInsertHeaderEl
{
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildWafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElChallengeElCustomRequestHandlingElInsertHeaderEl { pub fn build (self) -> Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElChallengeElCustomRequestHandlingElInsertHeaderEl { Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElChallengeElCustomRequestHandlingElInsertHeaderEl { name : self . name , value : self . value , } } }
pub struct Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElChallengeElCustomRequestHandlingElInsertHeaderElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElChallengeElCustomRequestHandlingElInsertHeaderElRef { fn new (shared : StackShared , base : String) -> Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElChallengeElCustomRequestHandlingElInsertHeaderElRef { Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElChallengeElCustomRequestHandlingElInsertHeaderElRef { shared : shared , base : base . to_string () , } } }
impl Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElChallengeElCustomRequestHandlingElInsertHeaderElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `name` after provisioning.\n"] pub fn name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.name" , self . base)) } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize, Default)]
struct Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElChallengeElCustomRequestHandlingElDynamic { insert_header : Option < DynamicBlock < Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElChallengeElCustomRequestHandlingElInsertHeaderEl >> , }
#[derive(Serialize)]
pub struct Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElChallengeElCustomRequestHandlingEl { # [serde (skip_serializing_if = "Option::is_none")] insert_header : Option < Vec < Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElChallengeElCustomRequestHandlingElInsertHeaderEl > > , dynamic : Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElChallengeElCustomRequestHandlingElDynamic , }
impl Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElChallengeElCustomRequestHandlingEl { # [doc = "Set the field `insert_header`.\n"] pub fn set_insert_header (mut self , v : impl Into < BlockAssignable < Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElChallengeElCustomRequestHandlingElInsertHeaderEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . insert_header = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . insert_header = Some (d) ; } } self } }
impl ToListMappable for Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElChallengeElCustomRequestHandlingEl { type O = BlockAssignable < Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElChallengeElCustomRequestHandlingEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildWafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElChallengeElCustomRequestHandlingEl
{}
impl BuildWafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElChallengeElCustomRequestHandlingEl { pub fn build (self) -> Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElChallengeElCustomRequestHandlingEl { Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElChallengeElCustomRequestHandlingEl { insert_header : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElChallengeElCustomRequestHandlingElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElChallengeElCustomRequestHandlingElRef { fn new (shared : StackShared , base : String) -> Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElChallengeElCustomRequestHandlingElRef { Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElChallengeElCustomRequestHandlingElRef { shared : shared , base : base . to_string () , } } }
impl Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElChallengeElCustomRequestHandlingElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `insert_header` after provisioning.\n"] pub fn insert_header (& self) -> ListRef < Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElChallengeElCustomRequestHandlingElInsertHeaderElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.insert_header" , self . base)) } }
#[derive(Serialize, Default)]
struct Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElChallengeElDynamic { custom_request_handling : Option < DynamicBlock < Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElChallengeElCustomRequestHandlingEl >> , }
#[derive(Serialize)]
pub struct Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElChallengeEl { # [serde (skip_serializing_if = "Option::is_none")] custom_request_handling : Option < Vec < Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElChallengeElCustomRequestHandlingEl > > , dynamic : Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElChallengeElDynamic , }
impl
    Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElChallengeEl
{
    #[doc = "Set the field `custom_request_handling`.\n"]
    pub fn set_custom_request_handling(
        mut self,
        v : impl Into < BlockAssignable < Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElChallengeElCustomRequestHandlingEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.custom_request_handling = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_request_handling = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElChallengeEl { type O = BlockAssignable < Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElChallengeEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildWafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElChallengeEl
{}
impl BuildWafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElChallengeEl { pub fn build (self) -> Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElChallengeEl { Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElChallengeEl { custom_request_handling : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElChallengeElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElChallengeElRef { fn new (shared : StackShared , base : String) -> Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElChallengeElRef { Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElChallengeElRef { shared : shared , base : base . to_string () , } } }
impl Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElChallengeElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `custom_request_handling` after provisioning.\n"] pub fn custom_request_handling (& self) -> ListRef < Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElChallengeElCustomRequestHandlingElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.custom_request_handling" , self . base)) } }
#[derive(Serialize)]
pub struct Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCountElCustomRequestHandlingElInsertHeaderEl
{
    name: PrimField<String>,
    value: PrimField<String>,
}
impl Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCountElCustomRequestHandlingElInsertHeaderEl { }
impl ToListMappable for Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCountElCustomRequestHandlingElInsertHeaderEl { type O = BlockAssignable < Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCountElCustomRequestHandlingElInsertHeaderEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildWafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCountElCustomRequestHandlingElInsertHeaderEl
{
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildWafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCountElCustomRequestHandlingElInsertHeaderEl { pub fn build (self) -> Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCountElCustomRequestHandlingElInsertHeaderEl { Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCountElCustomRequestHandlingElInsertHeaderEl { name : self . name , value : self . value , } } }
pub struct Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCountElCustomRequestHandlingElInsertHeaderElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCountElCustomRequestHandlingElInsertHeaderElRef { fn new (shared : StackShared , base : String) -> Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCountElCustomRequestHandlingElInsertHeaderElRef { Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCountElCustomRequestHandlingElInsertHeaderElRef { shared : shared , base : base . to_string () , } } }
impl Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCountElCustomRequestHandlingElInsertHeaderElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `name` after provisioning.\n"] pub fn name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.name" , self . base)) } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize, Default)]
struct Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCountElCustomRequestHandlingElDynamic { insert_header : Option < DynamicBlock < Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCountElCustomRequestHandlingElInsertHeaderEl >> , }
#[derive(Serialize)]
pub struct Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCountElCustomRequestHandlingEl { # [serde (skip_serializing_if = "Option::is_none")] insert_header : Option < Vec < Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCountElCustomRequestHandlingElInsertHeaderEl > > , dynamic : Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCountElCustomRequestHandlingElDynamic , }
impl Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCountElCustomRequestHandlingEl { # [doc = "Set the field `insert_header`.\n"] pub fn set_insert_header (mut self , v : impl Into < BlockAssignable < Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCountElCustomRequestHandlingElInsertHeaderEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . insert_header = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . insert_header = Some (d) ; } } self } }
impl ToListMappable for Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCountElCustomRequestHandlingEl { type O = BlockAssignable < Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCountElCustomRequestHandlingEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildWafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCountElCustomRequestHandlingEl
{}
impl BuildWafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCountElCustomRequestHandlingEl { pub fn build (self) -> Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCountElCustomRequestHandlingEl { Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCountElCustomRequestHandlingEl { insert_header : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCountElCustomRequestHandlingElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCountElCustomRequestHandlingElRef { fn new (shared : StackShared , base : String) -> Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCountElCustomRequestHandlingElRef { Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCountElCustomRequestHandlingElRef { shared : shared , base : base . to_string () , } } }
impl Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCountElCustomRequestHandlingElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `insert_header` after provisioning.\n"] pub fn insert_header (& self) -> ListRef < Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCountElCustomRequestHandlingElInsertHeaderElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.insert_header" , self . base)) } }
#[derive(Serialize, Default)]
struct Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCountElDynamic { custom_request_handling : Option < DynamicBlock < Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCountElCustomRequestHandlingEl >> , }
#[derive(Serialize)]
pub struct Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCountEl { # [serde (skip_serializing_if = "Option::is_none")] custom_request_handling : Option < Vec < Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCountElCustomRequestHandlingEl > > , dynamic : Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCountElDynamic , }
impl Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCountEl {
    #[doc = "Set the field `custom_request_handling`.\n"]
    pub fn set_custom_request_handling(
        mut self,
        v : impl Into < BlockAssignable < Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCountElCustomRequestHandlingEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.custom_request_handling = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_request_handling = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCountEl
{
    type O = BlockAssignable<
        Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCountEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildWafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCountEl
{}
impl
    BuildWafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCountEl
{
    pub fn build(
        self,
    ) -> Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCountEl
    {
        Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCountEl { custom_request_handling : core :: default :: Default :: default () , dynamic : Default :: default () , }
    }
}
pub struct Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCountElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCountElRef { fn new (shared : StackShared , base : String) -> Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCountElRef { Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCountElRef { shared : shared , base : base . to_string () , } } }
impl
    Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCountElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `custom_request_handling` after provisioning.\n"]    pub fn custom_request_handling (& self) -> ListRef < Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCountElCustomRequestHandlingElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_request_handling", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElDynamic { allow : Option < DynamicBlock < Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElAllowEl >> , block : Option < DynamicBlock < Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElBlockEl >> , captcha : Option < DynamicBlock < Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCaptchaEl >> , challenge : Option < DynamicBlock < Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElChallengeEl >> , count : Option < DynamicBlock < Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCountEl >> , }
#[derive(Serialize)]
pub struct Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseEl { # [serde (skip_serializing_if = "Option::is_none")] allow : Option < Vec < Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElAllowEl > > , # [serde (skip_serializing_if = "Option::is_none")] block : Option < Vec < Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElBlockEl > > , # [serde (skip_serializing_if = "Option::is_none")] captcha : Option < Vec < Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCaptchaEl > > , # [serde (skip_serializing_if = "Option::is_none")] challenge : Option < Vec < Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElChallengeEl > > , # [serde (skip_serializing_if = "Option::is_none")] count : Option < Vec < Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCountEl > > , dynamic : Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElDynamic , }
impl Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseEl {
    #[doc = "Set the field `allow`.\n"]
    pub fn set_allow(
        mut self,
        v : impl Into < BlockAssignable < Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElAllowEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.allow = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.allow = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `block`.\n"]
    pub fn set_block(
        mut self,
        v : impl Into < BlockAssignable < Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElBlockEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.block = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.block = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `captcha`.\n"]
    pub fn set_captcha(
        mut self,
        v : impl Into < BlockAssignable < Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCaptchaEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.captcha = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.captcha = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `challenge`.\n"]
    pub fn set_challenge(
        mut self,
        v : impl Into < BlockAssignable < Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElChallengeEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.challenge = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.challenge = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `count`.\n"]
    pub fn set_count(
        mut self,
        v : impl Into < BlockAssignable < Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCountEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.count = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.count = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseEl
{
    type O = BlockAssignable<
        Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildWafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseEl
{}
impl BuildWafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseEl {
    pub fn build(
        self,
    ) -> Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseEl {
        Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseEl {
            allow: core::default::Default::default(),
            block: core::default::Default::default(),
            captcha: core::default::Default::default(),
            challenge: core::default::Default::default(),
            count: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElRef
    {
        Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allow` after provisioning.\n"]    pub fn allow (& self) -> ListRef < Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElAllowElRef >{
        ListRef::new(self.shared().clone(), format!("{}.allow", self.base))
    }
    #[doc = "Get a reference to the value of field `block` after provisioning.\n"]    pub fn block (& self) -> ListRef < Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElBlockElRef >{
        ListRef::new(self.shared().clone(), format!("{}.block", self.base))
    }
    #[doc = "Get a reference to the value of field `captcha` after provisioning.\n"]    pub fn captcha (& self) -> ListRef < Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCaptchaElRef >{
        ListRef::new(self.shared().clone(), format!("{}.captcha", self.base))
    }
    #[doc = "Get a reference to the value of field `challenge` after provisioning.\n"]    pub fn challenge (& self) -> ListRef < Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElChallengeElRef >{
        ListRef::new(self.shared().clone(), format!("{}.challenge", self.base))
    }
    #[doc = "Get a reference to the value of field `count` after provisioning.\n"]    pub fn count (& self) -> ListRef < Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElCountElRef >{
        ListRef::new(self.shared().clone(), format!("{}.count", self.base))
    }
}
#[derive(Serialize, Default)]
struct Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElDynamic {
    action_to_use: Option<
        DynamicBlock<
            Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideEl {
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    action_to_use: Option<
        Vec<Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseEl>,
    >,
    dynamic: Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElDynamic,
}
impl Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideEl {
    #[doc = "Set the field `action_to_use`.\n"]
    pub fn set_action_to_use(
        mut self,
        v : impl Into < BlockAssignable < Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.action_to_use = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.action_to_use = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideEl {
    type O =
        BlockAssignable<Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildWafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideEl {
    #[doc = "Name of the rule to override."]
    pub name: PrimField<String>,
}
impl BuildWafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideEl {
    pub fn build(self) -> Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideEl {
        Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideEl {
            name: self.name,
            action_to_use: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElRef {
        Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the rule to override."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `action_to_use` after provisioning.\n"]
    pub fn action_to_use(
        &self,
    ) -> ListRef<
        Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElActionToUseElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.action_to_use", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElDynamic {
    rule_action_override: Option<
        DynamicBlock<Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideEl>,
    >,
}
#[derive(Serialize)]
pub struct Wafv2WebAclRuleGroupAssociationRuleGroupReferenceEl {
    arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rule_action_override:
        Option<Vec<Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideEl>>,
    dynamic: Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElDynamic,
}
impl Wafv2WebAclRuleGroupAssociationRuleGroupReferenceEl {
    #[doc = "Set the field `rule_action_override`.\n"]
    pub fn set_rule_action_override(
        mut self,
        v: impl Into<
            BlockAssignable<
                Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.rule_action_override = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.rule_action_override = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for Wafv2WebAclRuleGroupAssociationRuleGroupReferenceEl {
    type O = BlockAssignable<Wafv2WebAclRuleGroupAssociationRuleGroupReferenceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildWafv2WebAclRuleGroupAssociationRuleGroupReferenceEl {
    #[doc = "ARN of the Rule Group to associate with the Web ACL."]
    pub arn: PrimField<String>,
}
impl BuildWafv2WebAclRuleGroupAssociationRuleGroupReferenceEl {
    pub fn build(self) -> Wafv2WebAclRuleGroupAssociationRuleGroupReferenceEl {
        Wafv2WebAclRuleGroupAssociationRuleGroupReferenceEl {
            arn: self.arn,
            rule_action_override: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRef {
        Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\nARN of the Rule Group to associate with the Web ACL."]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }
    #[doc = "Get a reference to the value of field `rule_action_override` after provisioning.\n"]
    pub fn rule_action_override(
        &self,
    ) -> ListRef<Wafv2WebAclRuleGroupAssociationRuleGroupReferenceElRuleActionOverrideElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rule_action_override", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct Wafv2WebAclRuleGroupAssociationTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl Wafv2WebAclRuleGroupAssociationTimeoutsEl {
    #[doc = "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }
    #[doc = "Set the field `delete`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }
    #[doc = "Set the field `update`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}
impl ToListMappable for Wafv2WebAclRuleGroupAssociationTimeoutsEl {
    type O = BlockAssignable<Wafv2WebAclRuleGroupAssociationTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildWafv2WebAclRuleGroupAssociationTimeoutsEl {}
impl BuildWafv2WebAclRuleGroupAssociationTimeoutsEl {
    pub fn build(self) -> Wafv2WebAclRuleGroupAssociationTimeoutsEl {
        Wafv2WebAclRuleGroupAssociationTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct Wafv2WebAclRuleGroupAssociationTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Wafv2WebAclRuleGroupAssociationTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> Wafv2WebAclRuleGroupAssociationTimeoutsElRef {
        Wafv2WebAclRuleGroupAssociationTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Wafv2WebAclRuleGroupAssociationTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }
    #[doc = "Get a reference to the value of field `delete` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }
    #[doc = "Get a reference to the value of field `update` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
#[derive(Serialize, Default)]
struct Wafv2WebAclRuleGroupAssociationDynamic {
    managed_rule_group: Option<DynamicBlock<Wafv2WebAclRuleGroupAssociationManagedRuleGroupEl>>,
    rule_group_reference: Option<DynamicBlock<Wafv2WebAclRuleGroupAssociationRuleGroupReferenceEl>>,
}
