use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct WafregionalSqlInjectionMatchSetData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sql_injection_match_tuple: Option<Vec<WafregionalSqlInjectionMatchSetSqlInjectionMatchTupleEl>>,
    dynamic: WafregionalSqlInjectionMatchSetDynamic,
}
struct WafregionalSqlInjectionMatchSet_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<WafregionalSqlInjectionMatchSetData>,
}
#[derive(Clone)]
pub struct WafregionalSqlInjectionMatchSet(Rc<WafregionalSqlInjectionMatchSet_>);
impl WafregionalSqlInjectionMatchSet {
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
    #[doc = "Set the field `sql_injection_match_tuple`.\n"]
    pub fn set_sql_injection_match_tuple(
        self,
        v: impl Into<BlockAssignable<WafregionalSqlInjectionMatchSetSqlInjectionMatchTupleEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().sql_injection_match_tuple = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.sql_injection_match_tuple = Some(d);
            }
        }
        self
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
}
impl Referable for WafregionalSqlInjectionMatchSet {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for WafregionalSqlInjectionMatchSet {}
impl ToListMappable for WafregionalSqlInjectionMatchSet {
    type O = ListRef<WafregionalSqlInjectionMatchSetRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for WafregionalSqlInjectionMatchSet_ {
    fn extract_resource_type(&self) -> String {
        "aws_wafregional_sql_injection_match_set".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildWafregionalSqlInjectionMatchSet {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
}
impl BuildWafregionalSqlInjectionMatchSet {
    pub fn build(self, stack: &mut Stack) -> WafregionalSqlInjectionMatchSet {
        let out = WafregionalSqlInjectionMatchSet(Rc::new(WafregionalSqlInjectionMatchSet_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(WafregionalSqlInjectionMatchSetData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                sql_injection_match_tuple: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct WafregionalSqlInjectionMatchSetRef {
    shared: StackShared,
    base: String,
}
impl Ref for WafregionalSqlInjectionMatchSetRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl WafregionalSqlInjectionMatchSetRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
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
}
#[derive(Serialize)]
pub struct WafregionalSqlInjectionMatchSetSqlInjectionMatchTupleElFieldToMatchEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<PrimField<String>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}
impl WafregionalSqlInjectionMatchSetSqlInjectionMatchTupleElFieldToMatchEl {
    #[doc = "Set the field `data`.\n"]
    pub fn set_data(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.data = Some(v.into());
        self
    }
}
impl ToListMappable for WafregionalSqlInjectionMatchSetSqlInjectionMatchTupleElFieldToMatchEl {
    type O = BlockAssignable<WafregionalSqlInjectionMatchSetSqlInjectionMatchTupleElFieldToMatchEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildWafregionalSqlInjectionMatchSetSqlInjectionMatchTupleElFieldToMatchEl {
    #[doc = ""]
    pub type_: PrimField<String>,
}
impl BuildWafregionalSqlInjectionMatchSetSqlInjectionMatchTupleElFieldToMatchEl {
    pub fn build(self) -> WafregionalSqlInjectionMatchSetSqlInjectionMatchTupleElFieldToMatchEl {
        WafregionalSqlInjectionMatchSetSqlInjectionMatchTupleElFieldToMatchEl {
            data: core::default::Default::default(),
            type_: self.type_,
        }
    }
}
pub struct WafregionalSqlInjectionMatchSetSqlInjectionMatchTupleElFieldToMatchElRef {
    shared: StackShared,
    base: String,
}
impl Ref for WafregionalSqlInjectionMatchSetSqlInjectionMatchTupleElFieldToMatchElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> WafregionalSqlInjectionMatchSetSqlInjectionMatchTupleElFieldToMatchElRef {
        WafregionalSqlInjectionMatchSetSqlInjectionMatchTupleElFieldToMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl WafregionalSqlInjectionMatchSetSqlInjectionMatchTupleElFieldToMatchElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `data` after provisioning.\n"]
    pub fn data(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.data", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize, Default)]
struct WafregionalSqlInjectionMatchSetSqlInjectionMatchTupleElDynamic {
    field_to_match:
        Option<DynamicBlock<WafregionalSqlInjectionMatchSetSqlInjectionMatchTupleElFieldToMatchEl>>,
}
#[derive(Serialize)]
pub struct WafregionalSqlInjectionMatchSetSqlInjectionMatchTupleEl {
    text_transformation: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    field_to_match:
        Option<Vec<WafregionalSqlInjectionMatchSetSqlInjectionMatchTupleElFieldToMatchEl>>,
    dynamic: WafregionalSqlInjectionMatchSetSqlInjectionMatchTupleElDynamic,
}
impl WafregionalSqlInjectionMatchSetSqlInjectionMatchTupleEl {
    #[doc = "Set the field `field_to_match`.\n"]
    pub fn set_field_to_match(
        mut self,
        v: impl Into<
            BlockAssignable<WafregionalSqlInjectionMatchSetSqlInjectionMatchTupleElFieldToMatchEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.field_to_match = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.field_to_match = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for WafregionalSqlInjectionMatchSetSqlInjectionMatchTupleEl {
    type O = BlockAssignable<WafregionalSqlInjectionMatchSetSqlInjectionMatchTupleEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildWafregionalSqlInjectionMatchSetSqlInjectionMatchTupleEl {
    #[doc = ""]
    pub text_transformation: PrimField<String>,
}
impl BuildWafregionalSqlInjectionMatchSetSqlInjectionMatchTupleEl {
    pub fn build(self) -> WafregionalSqlInjectionMatchSetSqlInjectionMatchTupleEl {
        WafregionalSqlInjectionMatchSetSqlInjectionMatchTupleEl {
            text_transformation: self.text_transformation,
            field_to_match: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct WafregionalSqlInjectionMatchSetSqlInjectionMatchTupleElRef {
    shared: StackShared,
    base: String,
}
impl Ref for WafregionalSqlInjectionMatchSetSqlInjectionMatchTupleElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> WafregionalSqlInjectionMatchSetSqlInjectionMatchTupleElRef {
        WafregionalSqlInjectionMatchSetSqlInjectionMatchTupleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl WafregionalSqlInjectionMatchSetSqlInjectionMatchTupleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `text_transformation` after provisioning.\n"]
    pub fn text_transformation(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.text_transformation", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `field_to_match` after provisioning.\n"]
    pub fn field_to_match(
        &self,
    ) -> ListRef<WafregionalSqlInjectionMatchSetSqlInjectionMatchTupleElFieldToMatchElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.field_to_match", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct WafregionalSqlInjectionMatchSetDynamic {
    sql_injection_match_tuple:
        Option<DynamicBlock<WafregionalSqlInjectionMatchSetSqlInjectionMatchTupleEl>>,
}
