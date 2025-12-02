use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct WafRegexMatchSetData {
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
    regex_match_tuple: Option<Vec<WafRegexMatchSetRegexMatchTupleEl>>,
    dynamic: WafRegexMatchSetDynamic,
}
struct WafRegexMatchSet_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<WafRegexMatchSetData>,
}
#[derive(Clone)]
pub struct WafRegexMatchSet(Rc<WafRegexMatchSet_>);
impl WafRegexMatchSet {
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
    #[doc = "Set the field `regex_match_tuple`.\n"]
    pub fn set_regex_match_tuple(
        self,
        v: impl Into<BlockAssignable<WafRegexMatchSetRegexMatchTupleEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().regex_match_tuple = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.regex_match_tuple = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
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
}
impl Referable for WafRegexMatchSet {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for WafRegexMatchSet {}
impl ToListMappable for WafRegexMatchSet {
    type O = ListRef<WafRegexMatchSetRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for WafRegexMatchSet_ {
    fn extract_resource_type(&self) -> String {
        "aws_waf_regex_match_set".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildWafRegexMatchSet {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
}
impl BuildWafRegexMatchSet {
    pub fn build(self, stack: &mut Stack) -> WafRegexMatchSet {
        let out = WafRegexMatchSet(Rc::new(WafRegexMatchSet_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(WafRegexMatchSetData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                name: self.name,
                regex_match_tuple: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct WafRegexMatchSetRef {
    shared: StackShared,
    base: String,
}
impl Ref for WafRegexMatchSetRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl WafRegexMatchSetRef {
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
}
#[derive(Serialize)]
pub struct WafRegexMatchSetRegexMatchTupleElFieldToMatchEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<PrimField<String>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}
impl WafRegexMatchSetRegexMatchTupleElFieldToMatchEl {
    #[doc = "Set the field `data`.\n"]
    pub fn set_data(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.data = Some(v.into());
        self
    }
}
impl ToListMappable for WafRegexMatchSetRegexMatchTupleElFieldToMatchEl {
    type O = BlockAssignable<WafRegexMatchSetRegexMatchTupleElFieldToMatchEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildWafRegexMatchSetRegexMatchTupleElFieldToMatchEl {
    #[doc = ""]
    pub type_: PrimField<String>,
}
impl BuildWafRegexMatchSetRegexMatchTupleElFieldToMatchEl {
    pub fn build(self) -> WafRegexMatchSetRegexMatchTupleElFieldToMatchEl {
        WafRegexMatchSetRegexMatchTupleElFieldToMatchEl {
            data: core::default::Default::default(),
            type_: self.type_,
        }
    }
}
pub struct WafRegexMatchSetRegexMatchTupleElFieldToMatchElRef {
    shared: StackShared,
    base: String,
}
impl Ref for WafRegexMatchSetRegexMatchTupleElFieldToMatchElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> WafRegexMatchSetRegexMatchTupleElFieldToMatchElRef {
        WafRegexMatchSetRegexMatchTupleElFieldToMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl WafRegexMatchSetRegexMatchTupleElFieldToMatchElRef {
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
struct WafRegexMatchSetRegexMatchTupleElDynamic {
    field_to_match: Option<DynamicBlock<WafRegexMatchSetRegexMatchTupleElFieldToMatchEl>>,
}
#[derive(Serialize)]
pub struct WafRegexMatchSetRegexMatchTupleEl {
    regex_pattern_set_id: PrimField<String>,
    text_transformation: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    field_to_match: Option<Vec<WafRegexMatchSetRegexMatchTupleElFieldToMatchEl>>,
    dynamic: WafRegexMatchSetRegexMatchTupleElDynamic,
}
impl WafRegexMatchSetRegexMatchTupleEl {
    #[doc = "Set the field `field_to_match`.\n"]
    pub fn set_field_to_match(
        mut self,
        v: impl Into<BlockAssignable<WafRegexMatchSetRegexMatchTupleElFieldToMatchEl>>,
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
impl ToListMappable for WafRegexMatchSetRegexMatchTupleEl {
    type O = BlockAssignable<WafRegexMatchSetRegexMatchTupleEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildWafRegexMatchSetRegexMatchTupleEl {
    #[doc = ""]
    pub regex_pattern_set_id: PrimField<String>,
    #[doc = ""]
    pub text_transformation: PrimField<String>,
}
impl BuildWafRegexMatchSetRegexMatchTupleEl {
    pub fn build(self) -> WafRegexMatchSetRegexMatchTupleEl {
        WafRegexMatchSetRegexMatchTupleEl {
            regex_pattern_set_id: self.regex_pattern_set_id,
            text_transformation: self.text_transformation,
            field_to_match: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct WafRegexMatchSetRegexMatchTupleElRef {
    shared: StackShared,
    base: String,
}
impl Ref for WafRegexMatchSetRegexMatchTupleElRef {
    fn new(shared: StackShared, base: String) -> WafRegexMatchSetRegexMatchTupleElRef {
        WafRegexMatchSetRegexMatchTupleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl WafRegexMatchSetRegexMatchTupleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `regex_pattern_set_id` after provisioning.\n"]
    pub fn regex_pattern_set_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.regex_pattern_set_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `text_transformation` after provisioning.\n"]
    pub fn text_transformation(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.text_transformation", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `field_to_match` after provisioning.\n"]
    pub fn field_to_match(&self) -> ListRef<WafRegexMatchSetRegexMatchTupleElFieldToMatchElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.field_to_match", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct WafRegexMatchSetDynamic {
    regex_match_tuple: Option<DynamicBlock<WafRegexMatchSetRegexMatchTupleEl>>,
}
