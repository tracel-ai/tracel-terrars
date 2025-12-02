use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct WafregionalSizeConstraintSetData {
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
    size_constraints: Option<Vec<WafregionalSizeConstraintSetSizeConstraintsEl>>,
    dynamic: WafregionalSizeConstraintSetDynamic,
}
struct WafregionalSizeConstraintSet_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<WafregionalSizeConstraintSetData>,
}
#[derive(Clone)]
pub struct WafregionalSizeConstraintSet(Rc<WafregionalSizeConstraintSet_>);
impl WafregionalSizeConstraintSet {
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
    #[doc = "Set the field `size_constraints`.\n"]
    pub fn set_size_constraints(
        self,
        v: impl Into<BlockAssignable<WafregionalSizeConstraintSetSizeConstraintsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().size_constraints = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.size_constraints = Some(d);
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
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
}
impl Referable for WafregionalSizeConstraintSet {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for WafregionalSizeConstraintSet {}
impl ToListMappable for WafregionalSizeConstraintSet {
    type O = ListRef<WafregionalSizeConstraintSetRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for WafregionalSizeConstraintSet_ {
    fn extract_resource_type(&self) -> String {
        "aws_wafregional_size_constraint_set".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildWafregionalSizeConstraintSet {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
}
impl BuildWafregionalSizeConstraintSet {
    pub fn build(self, stack: &mut Stack) -> WafregionalSizeConstraintSet {
        let out = WafregionalSizeConstraintSet(Rc::new(WafregionalSizeConstraintSet_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(WafregionalSizeConstraintSetData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                size_constraints: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct WafregionalSizeConstraintSetRef {
    shared: StackShared,
    base: String,
}
impl Ref for WafregionalSizeConstraintSetRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl WafregionalSizeConstraintSetRef {
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
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct WafregionalSizeConstraintSetSizeConstraintsElFieldToMatchEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<PrimField<String>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}
impl WafregionalSizeConstraintSetSizeConstraintsElFieldToMatchEl {
    #[doc = "Set the field `data`.\n"]
    pub fn set_data(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.data = Some(v.into());
        self
    }
}
impl ToListMappable for WafregionalSizeConstraintSetSizeConstraintsElFieldToMatchEl {
    type O = BlockAssignable<WafregionalSizeConstraintSetSizeConstraintsElFieldToMatchEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildWafregionalSizeConstraintSetSizeConstraintsElFieldToMatchEl {
    #[doc = ""]
    pub type_: PrimField<String>,
}
impl BuildWafregionalSizeConstraintSetSizeConstraintsElFieldToMatchEl {
    pub fn build(self) -> WafregionalSizeConstraintSetSizeConstraintsElFieldToMatchEl {
        WafregionalSizeConstraintSetSizeConstraintsElFieldToMatchEl {
            data: core::default::Default::default(),
            type_: self.type_,
        }
    }
}
pub struct WafregionalSizeConstraintSetSizeConstraintsElFieldToMatchElRef {
    shared: StackShared,
    base: String,
}
impl Ref for WafregionalSizeConstraintSetSizeConstraintsElFieldToMatchElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> WafregionalSizeConstraintSetSizeConstraintsElFieldToMatchElRef {
        WafregionalSizeConstraintSetSizeConstraintsElFieldToMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl WafregionalSizeConstraintSetSizeConstraintsElFieldToMatchElRef {
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
struct WafregionalSizeConstraintSetSizeConstraintsElDynamic {
    field_to_match:
        Option<DynamicBlock<WafregionalSizeConstraintSetSizeConstraintsElFieldToMatchEl>>,
}
#[derive(Serialize)]
pub struct WafregionalSizeConstraintSetSizeConstraintsEl {
    comparison_operator: PrimField<String>,
    size: PrimField<f64>,
    text_transformation: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    field_to_match: Option<Vec<WafregionalSizeConstraintSetSizeConstraintsElFieldToMatchEl>>,
    dynamic: WafregionalSizeConstraintSetSizeConstraintsElDynamic,
}
impl WafregionalSizeConstraintSetSizeConstraintsEl {
    #[doc = "Set the field `field_to_match`.\n"]
    pub fn set_field_to_match(
        mut self,
        v: impl Into<BlockAssignable<WafregionalSizeConstraintSetSizeConstraintsElFieldToMatchEl>>,
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
impl ToListMappable for WafregionalSizeConstraintSetSizeConstraintsEl {
    type O = BlockAssignable<WafregionalSizeConstraintSetSizeConstraintsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildWafregionalSizeConstraintSetSizeConstraintsEl {
    #[doc = ""]
    pub comparison_operator: PrimField<String>,
    #[doc = ""]
    pub size: PrimField<f64>,
    #[doc = ""]
    pub text_transformation: PrimField<String>,
}
impl BuildWafregionalSizeConstraintSetSizeConstraintsEl {
    pub fn build(self) -> WafregionalSizeConstraintSetSizeConstraintsEl {
        WafregionalSizeConstraintSetSizeConstraintsEl {
            comparison_operator: self.comparison_operator,
            size: self.size,
            text_transformation: self.text_transformation,
            field_to_match: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct WafregionalSizeConstraintSetSizeConstraintsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for WafregionalSizeConstraintSetSizeConstraintsElRef {
    fn new(shared: StackShared, base: String) -> WafregionalSizeConstraintSetSizeConstraintsElRef {
        WafregionalSizeConstraintSetSizeConstraintsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl WafregionalSizeConstraintSetSizeConstraintsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `comparison_operator` after provisioning.\n"]
    pub fn comparison_operator(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.comparison_operator", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `size` after provisioning.\n"]
    pub fn size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.size", self.base))
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
    ) -> ListRef<WafregionalSizeConstraintSetSizeConstraintsElFieldToMatchElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.field_to_match", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct WafregionalSizeConstraintSetDynamic {
    size_constraints: Option<DynamicBlock<WafregionalSizeConstraintSetSizeConstraintsEl>>,
}
