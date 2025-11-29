use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct WafregionalXssMatchSetData {
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
    xss_match_tuple: Option<Vec<WafregionalXssMatchSetXssMatchTupleEl>>,
    dynamic: WafregionalXssMatchSetDynamic,
}

struct WafregionalXssMatchSet_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<WafregionalXssMatchSetData>,
}

#[derive(Clone)]
pub struct WafregionalXssMatchSet(Rc<WafregionalXssMatchSet_>);

impl WafregionalXssMatchSet {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `xss_match_tuple`.\n"]
    pub fn set_xss_match_tuple(self, v: impl Into<BlockAssignable<WafregionalXssMatchSetXssMatchTupleEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().xss_match_tuple = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.xss_match_tuple = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}

impl Referable for WafregionalXssMatchSet {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for WafregionalXssMatchSet { }

impl ToListMappable for WafregionalXssMatchSet {
    type O = ListRef<WafregionalXssMatchSetRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for WafregionalXssMatchSet_ {
    fn extract_resource_type(&self) -> String {
        "aws_wafregional_xss_match_set".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildWafregionalXssMatchSet {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildWafregionalXssMatchSet {
    pub fn build(self, stack: &mut Stack) -> WafregionalXssMatchSet {
        let out = WafregionalXssMatchSet(Rc::new(WafregionalXssMatchSet_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(WafregionalXssMatchSetData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                xss_match_tuple: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct WafregionalXssMatchSetRef {
    shared: StackShared,
    base: String,
}

impl Ref for WafregionalXssMatchSetRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl WafregionalXssMatchSetRef {
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
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct WafregionalXssMatchSetXssMatchTupleElFieldToMatchEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<PrimField<String>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}

impl WafregionalXssMatchSetXssMatchTupleElFieldToMatchEl {
    #[doc = "Set the field `data`.\n"]
    pub fn set_data(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.data = Some(v.into());
        self
    }
}

impl ToListMappable for WafregionalXssMatchSetXssMatchTupleElFieldToMatchEl {
    type O = BlockAssignable<WafregionalXssMatchSetXssMatchTupleElFieldToMatchEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafregionalXssMatchSetXssMatchTupleElFieldToMatchEl {
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl BuildWafregionalXssMatchSetXssMatchTupleElFieldToMatchEl {
    pub fn build(self) -> WafregionalXssMatchSetXssMatchTupleElFieldToMatchEl {
        WafregionalXssMatchSetXssMatchTupleElFieldToMatchEl {
            data: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

pub struct WafregionalXssMatchSetXssMatchTupleElFieldToMatchElRef {
    shared: StackShared,
    base: String,
}

impl Ref for WafregionalXssMatchSetXssMatchTupleElFieldToMatchElRef {
    fn new(shared: StackShared, base: String) -> WafregionalXssMatchSetXssMatchTupleElFieldToMatchElRef {
        WafregionalXssMatchSetXssMatchTupleElFieldToMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WafregionalXssMatchSetXssMatchTupleElFieldToMatchElRef {
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
struct WafregionalXssMatchSetXssMatchTupleElDynamic {
    field_to_match: Option<DynamicBlock<WafregionalXssMatchSetXssMatchTupleElFieldToMatchEl>>,
}

#[derive(Serialize)]
pub struct WafregionalXssMatchSetXssMatchTupleEl {
    text_transformation: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    field_to_match: Option<Vec<WafregionalXssMatchSetXssMatchTupleElFieldToMatchEl>>,
    dynamic: WafregionalXssMatchSetXssMatchTupleElDynamic,
}

impl WafregionalXssMatchSetXssMatchTupleEl {
    #[doc = "Set the field `field_to_match`.\n"]
    pub fn set_field_to_match(
        mut self,
        v: impl Into<BlockAssignable<WafregionalXssMatchSetXssMatchTupleElFieldToMatchEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.field_to_match = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.field_to_match = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for WafregionalXssMatchSetXssMatchTupleEl {
    type O = BlockAssignable<WafregionalXssMatchSetXssMatchTupleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafregionalXssMatchSetXssMatchTupleEl {
    #[doc = ""]
    pub text_transformation: PrimField<String>,
}

impl BuildWafregionalXssMatchSetXssMatchTupleEl {
    pub fn build(self) -> WafregionalXssMatchSetXssMatchTupleEl {
        WafregionalXssMatchSetXssMatchTupleEl {
            text_transformation: self.text_transformation,
            field_to_match: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct WafregionalXssMatchSetXssMatchTupleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for WafregionalXssMatchSetXssMatchTupleElRef {
    fn new(shared: StackShared, base: String) -> WafregionalXssMatchSetXssMatchTupleElRef {
        WafregionalXssMatchSetXssMatchTupleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WafregionalXssMatchSetXssMatchTupleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `text_transformation` after provisioning.\n"]
    pub fn text_transformation(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.text_transformation", self.base))
    }

    #[doc = "Get a reference to the value of field `field_to_match` after provisioning.\n"]
    pub fn field_to_match(&self) -> ListRef<WafregionalXssMatchSetXssMatchTupleElFieldToMatchElRef> {
        ListRef::new(self.shared().clone(), format!("{}.field_to_match", self.base))
    }
}

#[derive(Serialize, Default)]
struct WafregionalXssMatchSetDynamic {
    xss_match_tuple: Option<DynamicBlock<WafregionalXssMatchSetXssMatchTupleEl>>,
}
