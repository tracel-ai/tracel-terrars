use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct SecurityhubConfigurationPolicyData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    configuration_policy: Option<Vec<SecurityhubConfigurationPolicyConfigurationPolicyEl>>,
    dynamic: SecurityhubConfigurationPolicyDynamic,
}
struct SecurityhubConfigurationPolicy_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SecurityhubConfigurationPolicyData>,
}
#[derive(Clone)]
pub struct SecurityhubConfigurationPolicy(Rc<SecurityhubConfigurationPolicy_>);
impl SecurityhubConfigurationPolicy {
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
    #[doc = "Set the field `configuration_policy`.\n"]
    pub fn set_configuration_policy(
        self,
        v: impl Into<BlockAssignable<SecurityhubConfigurationPolicyConfigurationPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().configuration_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.configuration_policy = Some(d);
            }
        }
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
    #[doc = "Get a reference to the value of field `configuration_policy` after provisioning.\n"]
    pub fn configuration_policy(
        &self,
    ) -> ListRef<SecurityhubConfigurationPolicyConfigurationPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.configuration_policy", self.extract_ref()),
        )
    }
}
impl Referable for SecurityhubConfigurationPolicy {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for SecurityhubConfigurationPolicy {}
impl ToListMappable for SecurityhubConfigurationPolicy {
    type O = ListRef<SecurityhubConfigurationPolicyRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for SecurityhubConfigurationPolicy_ {
    fn extract_resource_type(&self) -> String {
        "aws_securityhub_configuration_policy".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildSecurityhubConfigurationPolicy {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
}
impl BuildSecurityhubConfigurationPolicy {
    pub fn build(self, stack: &mut Stack) -> SecurityhubConfigurationPolicy {
        let out = SecurityhubConfigurationPolicy(Rc::new(SecurityhubConfigurationPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(SecurityhubConfigurationPolicyData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                configuration_policy: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct SecurityhubConfigurationPolicyRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubConfigurationPolicyRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl SecurityhubConfigurationPolicyRef {
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
    #[doc = "Get a reference to the value of field `configuration_policy` after provisioning.\n"]
    pub fn configuration_policy(
        &self,
    ) -> ListRef<SecurityhubConfigurationPolicyConfigurationPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.configuration_policy", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElBoolEl
{
    value: PrimField<bool>,
}
impl SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElBoolEl { }
impl ToListMappable for SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElBoolEl { type O = BlockAssignable < SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElBoolEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildSecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElBoolEl
{
    #[doc = ""]
    pub value: PrimField<bool>,
}
impl BuildSecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElBoolEl { pub fn build (self) -> SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElBoolEl { SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElBoolEl { value : self . value , } } }
pub struct SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElBoolElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElBoolElRef { fn new (shared : StackShared , base : String) -> SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElBoolElRef { SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElBoolElRef { shared : shared , base : base . to_string () , } } }
impl SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElBoolElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize)]
pub struct SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElDoubleEl
{
    value: PrimField<f64>,
}
impl SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElDoubleEl { }
impl ToListMappable for SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElDoubleEl { type O = BlockAssignable < SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElDoubleEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildSecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElDoubleEl
{
    #[doc = ""]
    pub value: PrimField<f64>,
}
impl BuildSecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElDoubleEl { pub fn build (self) -> SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElDoubleEl { SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElDoubleEl { value : self . value , } } }
pub struct SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElDoubleElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElDoubleElRef { fn new (shared : StackShared , base : String) -> SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElDoubleElRef { SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElDoubleElRef { shared : shared , base : base . to_string () , } } }
impl SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElDoubleElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize)]
pub struct SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElEnumEl
{
    value: PrimField<String>,
}
impl SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElEnumEl { }
impl ToListMappable for SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElEnumEl { type O = BlockAssignable < SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElEnumEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildSecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElEnumEl
{
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildSecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElEnumEl { pub fn build (self) -> SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElEnumEl { SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElEnumEl { value : self . value , } } }
pub struct SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElEnumElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElEnumElRef { fn new (shared : StackShared , base : String) -> SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElEnumElRef { SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElEnumElRef { shared : shared , base : base . to_string () , } } }
impl SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElEnumElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize)]
pub struct SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElEnumListEl
{
    value: ListField<PrimField<String>>,
}
impl SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElEnumListEl { }
impl ToListMappable for SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElEnumListEl { type O = BlockAssignable < SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElEnumListEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildSecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElEnumListEl
{
    #[doc = ""]
    pub value: ListField<PrimField<String>>,
}
impl BuildSecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElEnumListEl { pub fn build (self) -> SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElEnumListEl { SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElEnumListEl { value : self . value , } } }
pub struct SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElEnumListElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElEnumListElRef { fn new (shared : StackShared , base : String) -> SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElEnumListElRef { SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElEnumListElRef { shared : shared , base : base . to_string () , } } }
impl SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElEnumListElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize)]
pub struct SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElIntEl
{
    value: PrimField<f64>,
}
impl SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElIntEl { }
impl ToListMappable for SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElIntEl { type O = BlockAssignable < SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElIntEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildSecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElIntEl
{
    #[doc = ""]
    pub value: PrimField<f64>,
}
impl BuildSecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElIntEl { pub fn build (self) -> SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElIntEl { SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElIntEl { value : self . value , } } }
pub struct SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElIntElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElIntElRef { fn new (shared : StackShared , base : String) -> SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElIntElRef { SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElIntElRef { shared : shared , base : base . to_string () , } } }
impl SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElIntElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize)]
pub struct SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElIntListEl
{
    value: ListField<PrimField<f64>>,
}
impl SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElIntListEl { }
impl ToListMappable for SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElIntListEl { type O = BlockAssignable < SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElIntListEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildSecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElIntListEl
{
    #[doc = ""]
    pub value: ListField<PrimField<f64>>,
}
impl BuildSecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElIntListEl { pub fn build (self) -> SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElIntListEl { SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElIntListEl { value : self . value , } } }
pub struct SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElIntListElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElIntListElRef { fn new (shared : StackShared , base : String) -> SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElIntListElRef { SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElIntListElRef { shared : shared , base : base . to_string () , } } }
impl SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElIntListElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> ListRef < PrimExpr < f64 > > { ListRef :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize)]
pub struct SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElStringEl
{
    value: PrimField<String>,
}
impl SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElStringEl { }
impl ToListMappable for SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElStringEl { type O = BlockAssignable < SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElStringEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildSecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElStringEl
{
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildSecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElStringEl { pub fn build (self) -> SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElStringEl { SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElStringEl { value : self . value , } } }
pub struct SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElStringElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElStringElRef { fn new (shared : StackShared , base : String) -> SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElStringElRef { SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElStringElRef { shared : shared , base : base . to_string () , } } }
impl SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElStringElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize)]
pub struct SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElStringListEl
{
    value: ListField<PrimField<String>>,
}
impl SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElStringListEl { }
impl ToListMappable for SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElStringListEl { type O = BlockAssignable < SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElStringListEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildSecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElStringListEl
{
    #[doc = ""]
    pub value: ListField<PrimField<String>>,
}
impl BuildSecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElStringListEl { pub fn build (self) -> SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElStringListEl { SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElStringListEl { value : self . value , } } }
pub struct SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElStringListElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElStringListElRef { fn new (shared : StackShared , base : String) -> SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElStringListElRef { SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElStringListElRef { shared : shared , base : base . to_string () , } } }
impl SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElStringListElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize, Default)]
struct SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElDynamic { bool : Option < DynamicBlock < SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElBoolEl >> , double : Option < DynamicBlock < SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElDoubleEl >> , enum_ : Option < DynamicBlock < SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElEnumEl >> , enum_list : Option < DynamicBlock < SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElEnumListEl >> , int : Option < DynamicBlock < SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElIntEl >> , int_list : Option < DynamicBlock < SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElIntListEl >> , string : Option < DynamicBlock < SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElStringEl >> , string_list : Option < DynamicBlock < SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElStringListEl >> , }
#[derive(Serialize)]
pub struct SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterEl { name : PrimField < String > , value_type : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] bool : Option < Vec < SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElBoolEl > > , # [serde (skip_serializing_if = "Option::is_none")] double : Option < Vec < SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElDoubleEl > > , # [serde (rename = "enum" , skip_serializing_if = "Option::is_none")] enum_ : Option < Vec < SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElEnumEl > > , # [serde (skip_serializing_if = "Option::is_none")] enum_list : Option < Vec < SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElEnumListEl > > , # [serde (skip_serializing_if = "Option::is_none")] int : Option < Vec < SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElIntEl > > , # [serde (skip_serializing_if = "Option::is_none")] int_list : Option < Vec < SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElIntListEl > > , # [serde (skip_serializing_if = "Option::is_none")] string : Option < Vec < SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElStringEl > > , # [serde (skip_serializing_if = "Option::is_none")] string_list : Option < Vec < SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElStringListEl > > , dynamic : SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElDynamic , }
impl SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterEl { # [doc = "Set the field `bool`.\n"] pub fn set_bool (mut self , v : impl Into < BlockAssignable < SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElBoolEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . bool = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . bool = Some (d) ; } } self } # [doc = "Set the field `double`.\n"] pub fn set_double (mut self , v : impl Into < BlockAssignable < SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElDoubleEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . double = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . double = Some (d) ; } } self } # [doc = "Set the field `enum_`.\n"] pub fn set_enum (mut self , v : impl Into < BlockAssignable < SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElEnumEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . enum_ = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . enum_ = Some (d) ; } } self } # [doc = "Set the field `enum_list`.\n"] pub fn set_enum_list (mut self , v : impl Into < BlockAssignable < SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElEnumListEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . enum_list = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . enum_list = Some (d) ; } } self } # [doc = "Set the field `int`.\n"] pub fn set_int (mut self , v : impl Into < BlockAssignable < SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElIntEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . int = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . int = Some (d) ; } } self } # [doc = "Set the field `int_list`.\n"] pub fn set_int_list (mut self , v : impl Into < BlockAssignable < SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElIntListEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . int_list = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . int_list = Some (d) ; } } self } # [doc = "Set the field `string`.\n"] pub fn set_string (mut self , v : impl Into < BlockAssignable < SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElStringEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . string = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . string = Some (d) ; } } self } # [doc = "Set the field `string_list`.\n"] pub fn set_string_list (mut self , v : impl Into < BlockAssignable < SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElStringListEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . string_list = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . string_list = Some (d) ; } } self } }
impl ToListMappable for SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterEl { type O = BlockAssignable < SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildSecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterEl
{
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub value_type: PrimField<String>,
}
impl BuildSecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterEl { pub fn build (self) -> SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterEl { SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterEl { name : self . name , value_type : self . value_type , bool : core :: default :: Default :: default () , double : core :: default :: Default :: default () , enum_ : core :: default :: Default :: default () , enum_list : core :: default :: Default :: default () , int : core :: default :: Default :: default () , int_list : core :: default :: Default :: default () , string : core :: default :: Default :: default () , string_list : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElRef { fn new (shared : StackShared , base : String) -> SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElRef { SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElRef { shared : shared , base : base . to_string () , } } }
impl SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `name` after provisioning.\n"] pub fn name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.name" , self . base)) } # [doc = "Get a reference to the value of field `value_type` after provisioning.\n"] pub fn value_type (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value_type" , self . base)) } # [doc = "Get a reference to the value of field `bool` after provisioning.\n"] pub fn bool (& self) -> ListRef < SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElBoolElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.bool" , self . base)) } # [doc = "Get a reference to the value of field `double` after provisioning.\n"] pub fn double (& self) -> ListRef < SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElDoubleElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.double" , self . base)) } # [doc = "Get a reference to the value of field `enum_` after provisioning.\n"] pub fn enum_ (& self) -> ListRef < SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElEnumElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.enum" , self . base)) } # [doc = "Get a reference to the value of field `enum_list` after provisioning.\n"] pub fn enum_list (& self) -> ListRef < SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElEnumListElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.enum_list" , self . base)) } # [doc = "Get a reference to the value of field `int` after provisioning.\n"] pub fn int (& self) -> ListRef < SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElIntElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.int" , self . base)) } # [doc = "Get a reference to the value of field `int_list` after provisioning.\n"] pub fn int_list (& self) -> ListRef < SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElIntListElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.int_list" , self . base)) } # [doc = "Get a reference to the value of field `string` after provisioning.\n"] pub fn string (& self) -> ListRef < SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElStringElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.string" , self . base)) } # [doc = "Get a reference to the value of field `string_list` after provisioning.\n"] pub fn string_list (& self) -> ListRef < SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterElStringListElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.string_list" , self . base)) } }
#[derive(Serialize, Default)]
struct SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElDynamic { parameter : Option < DynamicBlock < SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterEl >> , }
#[derive(Serialize)]
pub struct SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterEl { security_control_id : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] parameter : Option < Vec < SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterEl > > , dynamic : SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElDynamic , }
impl SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterEl { # [doc = "Set the field `parameter`.\n"] pub fn set_parameter (mut self , v : impl Into < BlockAssignable < SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElParameterEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . parameter = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . parameter = Some (d) ; } } self } }
impl ToListMappable for SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterEl { type O = BlockAssignable < SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildSecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterEl
{
    #[doc = ""]
    pub security_control_id: PrimField<String>,
}
impl BuildSecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterEl { pub fn build (self) -> SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterEl { SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterEl { security_control_id : self . security_control_id , parameter : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElRef { fn new (shared : StackShared , base : String) -> SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElRef { SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElRef { shared : shared , base : base . to_string () , } } }
impl SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `security_control_id` after provisioning.\n"] pub fn security_control_id (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.security_control_id" , self . base)) } }
#[derive(Serialize, Default)]
struct SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElDynamic { security_control_custom_parameter : Option < DynamicBlock < SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterEl >> , }
#[derive(Serialize)]
pub struct SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationEl { # [serde (skip_serializing_if = "Option::is_none")] disabled_control_identifiers : Option < SetField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] enabled_control_identifiers : Option < SetField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] security_control_custom_parameter : Option < Vec < SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterEl > > , dynamic : SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElDynamic , }
impl SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationEl {
    #[doc = "Set the field `disabled_control_identifiers`.\n"]
    pub fn set_disabled_control_identifiers(
        mut self,
        v: impl Into<SetField<PrimField<String>>>,
    ) -> Self {
        self.disabled_control_identifiers = Some(v.into());
        self
    }
    #[doc = "Set the field `enabled_control_identifiers`.\n"]
    pub fn set_enabled_control_identifiers(
        mut self,
        v: impl Into<SetField<PrimField<String>>>,
    ) -> Self {
        self.enabled_control_identifiers = Some(v.into());
        self
    }
    #[doc = "Set the field `security_control_custom_parameter`.\n"]
    pub fn set_security_control_custom_parameter(
        mut self,
        v : impl Into < BlockAssignable < SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.security_control_custom_parameter = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.security_control_custom_parameter = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationEl
{
    type O = BlockAssignable<
        SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationEl
{}
impl BuildSecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationEl {
    pub fn build(
        self,
    ) -> SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationEl {
        SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationEl {
            disabled_control_identifiers: core::default::Default::default(),
            enabled_control_identifiers: core::default::Default::default(),
            security_control_custom_parameter: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElRef {
        SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `disabled_control_identifiers` after provisioning.\n"]
    pub fn disabled_control_identifiers(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.disabled_control_identifiers", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enabled_control_identifiers` after provisioning.\n"]
    pub fn enabled_control_identifiers(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.enabled_control_identifiers", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `security_control_custom_parameter` after provisioning.\n"]    pub fn security_control_custom_parameter (& self) -> ListRef < SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElSecurityControlCustomParameterElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.security_control_custom_parameter", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct SecurityhubConfigurationPolicyConfigurationPolicyElDynamic {
    security_controls_configuration: Option<
        DynamicBlock<
            SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct SecurityhubConfigurationPolicyConfigurationPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled_standard_arns: Option<SetField<PrimField<String>>>,
    service_enabled: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_controls_configuration: Option<
        Vec<SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationEl>,
    >,
    dynamic: SecurityhubConfigurationPolicyConfigurationPolicyElDynamic,
}
impl SecurityhubConfigurationPolicyConfigurationPolicyEl {
    #[doc = "Set the field `enabled_standard_arns`.\n"]
    pub fn set_enabled_standard_arns(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.enabled_standard_arns = Some(v.into());
        self
    }
    #[doc = "Set the field `security_controls_configuration`.\n"]
    pub fn set_security_controls_configuration(
        mut self,
        v: impl Into<
            BlockAssignable<
                SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.security_controls_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.security_controls_configuration = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for SecurityhubConfigurationPolicyConfigurationPolicyEl {
    type O = BlockAssignable<SecurityhubConfigurationPolicyConfigurationPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubConfigurationPolicyConfigurationPolicyEl {
    #[doc = ""]
    pub service_enabled: PrimField<bool>,
}
impl BuildSecurityhubConfigurationPolicyConfigurationPolicyEl {
    pub fn build(self) -> SecurityhubConfigurationPolicyConfigurationPolicyEl {
        SecurityhubConfigurationPolicyConfigurationPolicyEl {
            enabled_standard_arns: core::default::Default::default(),
            service_enabled: self.service_enabled,
            security_controls_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct SecurityhubConfigurationPolicyConfigurationPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubConfigurationPolicyConfigurationPolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubConfigurationPolicyConfigurationPolicyElRef {
        SecurityhubConfigurationPolicyConfigurationPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubConfigurationPolicyConfigurationPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled_standard_arns` after provisioning.\n"]
    pub fn enabled_standard_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.enabled_standard_arns", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `service_enabled` after provisioning.\n"]
    pub fn service_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_enabled", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `security_controls_configuration` after provisioning.\n"]
    pub fn security_controls_configuration(
        &self,
    ) -> ListRef<
        SecurityhubConfigurationPolicyConfigurationPolicyElSecurityControlsConfigurationElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.security_controls_configuration", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct SecurityhubConfigurationPolicyDynamic {
    configuration_policy: Option<DynamicBlock<SecurityhubConfigurationPolicyConfigurationPolicyEl>>,
}
