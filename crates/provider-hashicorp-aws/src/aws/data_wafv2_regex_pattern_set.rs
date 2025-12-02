use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataWafv2RegexPatternSetData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    scope: PrimField<String>,
}
struct DataWafv2RegexPatternSet_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataWafv2RegexPatternSetData>,
}
#[derive(Clone)]
pub struct DataWafv2RegexPatternSet(Rc<DataWafv2RegexPatternSet_>);
impl DataWafv2RegexPatternSet {
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
    #[doc = "Get a reference to the value of field `regular_expression` after provisioning.\n"]
    pub fn regular_expression(&self) -> SetRef<DataWafv2RegexPatternSetRegularExpressionElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.regular_expression", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `scope` after provisioning.\n"]
    pub fn scope(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.scope", self.extract_ref()),
        )
    }
}
impl Referable for DataWafv2RegexPatternSet {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataWafv2RegexPatternSet {}
impl ToListMappable for DataWafv2RegexPatternSet {
    type O = ListRef<DataWafv2RegexPatternSetRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataWafv2RegexPatternSet_ {
    fn extract_datasource_type(&self) -> String {
        "aws_wafv2_regex_pattern_set".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataWafv2RegexPatternSet {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub scope: PrimField<String>,
}
impl BuildDataWafv2RegexPatternSet {
    pub fn build(self, stack: &mut Stack) -> DataWafv2RegexPatternSet {
        let out = DataWafv2RegexPatternSet(Rc::new(DataWafv2RegexPatternSet_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataWafv2RegexPatternSetData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                scope: self.scope,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataWafv2RegexPatternSetRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataWafv2RegexPatternSetRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataWafv2RegexPatternSetRef {
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
    #[doc = "Get a reference to the value of field `regular_expression` after provisioning.\n"]
    pub fn regular_expression(&self) -> SetRef<DataWafv2RegexPatternSetRegularExpressionElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.regular_expression", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `scope` after provisioning.\n"]
    pub fn scope(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.scope", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataWafv2RegexPatternSetRegularExpressionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    regex_string: Option<PrimField<String>>,
}
impl DataWafv2RegexPatternSetRegularExpressionEl {
    #[doc = "Set the field `regex_string`.\n"]
    pub fn set_regex_string(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.regex_string = Some(v.into());
        self
    }
}
impl ToListMappable for DataWafv2RegexPatternSetRegularExpressionEl {
    type O = BlockAssignable<DataWafv2RegexPatternSetRegularExpressionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataWafv2RegexPatternSetRegularExpressionEl {}
impl BuildDataWafv2RegexPatternSetRegularExpressionEl {
    pub fn build(self) -> DataWafv2RegexPatternSetRegularExpressionEl {
        DataWafv2RegexPatternSetRegularExpressionEl {
            regex_string: core::default::Default::default(),
        }
    }
}
pub struct DataWafv2RegexPatternSetRegularExpressionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataWafv2RegexPatternSetRegularExpressionElRef {
    fn new(shared: StackShared, base: String) -> DataWafv2RegexPatternSetRegularExpressionElRef {
        DataWafv2RegexPatternSetRegularExpressionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataWafv2RegexPatternSetRegularExpressionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `regex_string` after provisioning.\n"]
    pub fn regex_string(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.regex_string", self.base))
    }
}
