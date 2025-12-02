use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataServicecatalogPortfolioConstraintsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    accept_language: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    portfolio_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    product_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DataServicecatalogPortfolioConstraintsTimeoutsEl>,
}
struct DataServicecatalogPortfolioConstraints_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataServicecatalogPortfolioConstraintsData>,
}
#[derive(Clone)]
pub struct DataServicecatalogPortfolioConstraints(Rc<DataServicecatalogPortfolioConstraints_>);
impl DataServicecatalogPortfolioConstraints {
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
    #[doc = "Set the field `accept_language`.\n"]
    pub fn set_accept_language(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().accept_language = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `product_id`.\n"]
    pub fn set_product_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().product_id = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(
        self,
        v: impl Into<DataServicecatalogPortfolioConstraintsTimeoutsEl>,
    ) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `accept_language` after provisioning.\n"]
    pub fn accept_language(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.accept_language", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `details` after provisioning.\n"]
    pub fn details(&self) -> ListRef<DataServicecatalogPortfolioConstraintsDetailsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.details", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `portfolio_id` after provisioning.\n"]
    pub fn portfolio_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.portfolio_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `product_id` after provisioning.\n"]
    pub fn product_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.product_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataServicecatalogPortfolioConstraintsTimeoutsElRef {
        DataServicecatalogPortfolioConstraintsTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for DataServicecatalogPortfolioConstraints {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataServicecatalogPortfolioConstraints {}
impl ToListMappable for DataServicecatalogPortfolioConstraints {
    type O = ListRef<DataServicecatalogPortfolioConstraintsRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataServicecatalogPortfolioConstraints_ {
    fn extract_datasource_type(&self) -> String {
        "aws_servicecatalog_portfolio_constraints".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataServicecatalogPortfolioConstraints {
    pub tf_id: String,
    #[doc = ""]
    pub portfolio_id: PrimField<String>,
}
impl BuildDataServicecatalogPortfolioConstraints {
    pub fn build(self, stack: &mut Stack) -> DataServicecatalogPortfolioConstraints {
        let out = DataServicecatalogPortfolioConstraints(Rc::new(
            DataServicecatalogPortfolioConstraints_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(DataServicecatalogPortfolioConstraintsData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    for_each: None,
                    accept_language: core::default::Default::default(),
                    id: core::default::Default::default(),
                    portfolio_id: self.portfolio_id,
                    product_id: core::default::Default::default(),
                    region: core::default::Default::default(),
                    timeouts: core::default::Default::default(),
                }),
            },
        ));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataServicecatalogPortfolioConstraintsRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataServicecatalogPortfolioConstraintsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataServicecatalogPortfolioConstraintsRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `accept_language` after provisioning.\n"]
    pub fn accept_language(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.accept_language", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `details` after provisioning.\n"]
    pub fn details(&self) -> ListRef<DataServicecatalogPortfolioConstraintsDetailsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.details", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `portfolio_id` after provisioning.\n"]
    pub fn portfolio_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.portfolio_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `product_id` after provisioning.\n"]
    pub fn product_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.product_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataServicecatalogPortfolioConstraintsTimeoutsElRef {
        DataServicecatalogPortfolioConstraintsTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataServicecatalogPortfolioConstraintsDetailsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    constraint_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    owner: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    portfolio_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    product_id: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl DataServicecatalogPortfolioConstraintsDetailsEl {
    #[doc = "Set the field `constraint_id`.\n"]
    pub fn set_constraint_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.constraint_id = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `owner`.\n"]
    pub fn set_owner(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.owner = Some(v.into());
        self
    }
    #[doc = "Set the field `portfolio_id`.\n"]
    pub fn set_portfolio_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.portfolio_id = Some(v.into());
        self
    }
    #[doc = "Set the field `product_id`.\n"]
    pub fn set_product_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.product_id = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable for DataServicecatalogPortfolioConstraintsDetailsEl {
    type O = BlockAssignable<DataServicecatalogPortfolioConstraintsDetailsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataServicecatalogPortfolioConstraintsDetailsEl {}
impl BuildDataServicecatalogPortfolioConstraintsDetailsEl {
    pub fn build(self) -> DataServicecatalogPortfolioConstraintsDetailsEl {
        DataServicecatalogPortfolioConstraintsDetailsEl {
            constraint_id: core::default::Default::default(),
            description: core::default::Default::default(),
            owner: core::default::Default::default(),
            portfolio_id: core::default::Default::default(),
            product_id: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}
pub struct DataServicecatalogPortfolioConstraintsDetailsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataServicecatalogPortfolioConstraintsDetailsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataServicecatalogPortfolioConstraintsDetailsElRef {
        DataServicecatalogPortfolioConstraintsDetailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataServicecatalogPortfolioConstraintsDetailsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `constraint_id` after provisioning.\n"]
    pub fn constraint_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.constraint_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `owner` after provisioning.\n"]
    pub fn owner(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.owner", self.base))
    }
    #[doc = "Get a reference to the value of field `portfolio_id` after provisioning.\n"]
    pub fn portfolio_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.portfolio_id", self.base))
    }
    #[doc = "Get a reference to the value of field `product_id` after provisioning.\n"]
    pub fn product_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.product_id", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct DataServicecatalogPortfolioConstraintsTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    read: Option<PrimField<String>>,
}
impl DataServicecatalogPortfolioConstraintsTimeoutsEl {
    #[doc = "Set the field `read`.\n"]
    pub fn set_read(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.read = Some(v.into());
        self
    }
}
impl ToListMappable for DataServicecatalogPortfolioConstraintsTimeoutsEl {
    type O = BlockAssignable<DataServicecatalogPortfolioConstraintsTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataServicecatalogPortfolioConstraintsTimeoutsEl {}
impl BuildDataServicecatalogPortfolioConstraintsTimeoutsEl {
    pub fn build(self) -> DataServicecatalogPortfolioConstraintsTimeoutsEl {
        DataServicecatalogPortfolioConstraintsTimeoutsEl {
            read: core::default::Default::default(),
        }
    }
}
pub struct DataServicecatalogPortfolioConstraintsTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataServicecatalogPortfolioConstraintsTimeoutsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataServicecatalogPortfolioConstraintsTimeoutsElRef {
        DataServicecatalogPortfolioConstraintsTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataServicecatalogPortfolioConstraintsTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }
}
