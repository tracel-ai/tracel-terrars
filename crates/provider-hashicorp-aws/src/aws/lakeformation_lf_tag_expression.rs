use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct LakeformationLfTagExpressionData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    catalog_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expression: Option<Vec<LakeformationLfTagExpressionExpressionEl>>,
    dynamic: LakeformationLfTagExpressionDynamic,
}

struct LakeformationLfTagExpression_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<LakeformationLfTagExpressionData>,
}

#[derive(Clone)]
pub struct LakeformationLfTagExpression(Rc<LakeformationLfTagExpression_>);

impl LakeformationLfTagExpression {
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

    #[doc = "Set the field `catalog_id`.\nThe ID of the Data Catalog."]
    pub fn set_catalog_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().catalog_id = Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nA description of the LF-Tag Expression."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `expression`.\n"]
    pub fn set_expression(self, v: impl Into<BlockAssignable<LakeformationLfTagExpressionExpressionEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().expression = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.expression = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `catalog_id` after provisioning.\nThe ID of the Data Catalog."]
    pub fn catalog_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.catalog_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA description of the LF-Tag Expression."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the LF-Tag Expression."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}

impl Referable for LakeformationLfTagExpression {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for LakeformationLfTagExpression { }

impl ToListMappable for LakeformationLfTagExpression {
    type O = ListRef<LakeformationLfTagExpressionRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for LakeformationLfTagExpression_ {
    fn extract_resource_type(&self) -> String {
        "aws_lakeformation_lf_tag_expression".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildLakeformationLfTagExpression {
    pub tf_id: String,
    #[doc = "The name of the LF-Tag Expression."]
    pub name: PrimField<String>,
}

impl BuildLakeformationLfTagExpression {
    pub fn build(self, stack: &mut Stack) -> LakeformationLfTagExpression {
        let out = LakeformationLfTagExpression(Rc::new(LakeformationLfTagExpression_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(LakeformationLfTagExpressionData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                catalog_id: core::default::Default::default(),
                description: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                expression: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct LakeformationLfTagExpressionRef {
    shared: StackShared,
    base: String,
}

impl Ref for LakeformationLfTagExpressionRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl LakeformationLfTagExpressionRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `catalog_id` after provisioning.\nThe ID of the Data Catalog."]
    pub fn catalog_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.catalog_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA description of the LF-Tag Expression."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the LF-Tag Expression."]
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
pub struct LakeformationLfTagExpressionExpressionEl {
    tag_key: PrimField<String>,
    tag_values: SetField<PrimField<String>>,
}

impl LakeformationLfTagExpressionExpressionEl { }

impl ToListMappable for LakeformationLfTagExpressionExpressionEl {
    type O = BlockAssignable<LakeformationLfTagExpressionExpressionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLakeformationLfTagExpressionExpressionEl {
    #[doc = ""]
    pub tag_key: PrimField<String>,
    #[doc = ""]
    pub tag_values: SetField<PrimField<String>>,
}

impl BuildLakeformationLfTagExpressionExpressionEl {
    pub fn build(self) -> LakeformationLfTagExpressionExpressionEl {
        LakeformationLfTagExpressionExpressionEl {
            tag_key: self.tag_key,
            tag_values: self.tag_values,
        }
    }
}

pub struct LakeformationLfTagExpressionExpressionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LakeformationLfTagExpressionExpressionElRef {
    fn new(shared: StackShared, base: String) -> LakeformationLfTagExpressionExpressionElRef {
        LakeformationLfTagExpressionExpressionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LakeformationLfTagExpressionExpressionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `tag_key` after provisioning.\n"]
    pub fn tag_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag_key", self.base))
    }

    #[doc = "Get a reference to the value of field `tag_values` after provisioning.\n"]
    pub fn tag_values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.tag_values", self.base))
    }
}

#[derive(Serialize, Default)]
struct LakeformationLfTagExpressionDynamic {
    expression: Option<DynamicBlock<LakeformationLfTagExpressionExpressionEl>>,
}
