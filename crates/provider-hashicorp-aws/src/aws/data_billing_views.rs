use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataBillingViewsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    billing_view_types: Option<ListField<PrimField<String>>>,
}
struct DataBillingViews_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataBillingViewsData>,
}
#[derive(Clone)]
pub struct DataBillingViews(Rc<DataBillingViews_>);
impl DataBillingViews {
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
    #[doc = "Set the field `billing_view_types`.\n"]
    pub fn set_billing_view_types(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().billing_view_types = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `billing_view` after provisioning.\n"]
    pub fn billing_view(&self) -> ListRef<DataBillingViewsBillingViewElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.billing_view", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `billing_view_types` after provisioning.\n"]
    pub fn billing_view_types(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.billing_view_types", self.extract_ref()),
        )
    }
}
impl Referable for DataBillingViews {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataBillingViews {}
impl ToListMappable for DataBillingViews {
    type O = ListRef<DataBillingViewsRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataBillingViews_ {
    fn extract_datasource_type(&self) -> String {
        "aws_billing_views".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataBillingViews {
    pub tf_id: String,
}
impl BuildDataBillingViews {
    pub fn build(self, stack: &mut Stack) -> DataBillingViews {
        let out = DataBillingViews(Rc::new(DataBillingViews_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataBillingViewsData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                billing_view_types: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataBillingViewsRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBillingViewsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataBillingViewsRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `billing_view` after provisioning.\n"]
    pub fn billing_view(&self) -> ListRef<DataBillingViewsBillingViewElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.billing_view", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `billing_view_types` after provisioning.\n"]
    pub fn billing_view_types(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.billing_view_types", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataBillingViewsBillingViewEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    billing_view_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    owner_account_id: Option<PrimField<String>>,
}
impl DataBillingViewsBillingViewEl {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.arn = Some(v.into());
        self
    }
    #[doc = "Set the field `billing_view_type`.\n"]
    pub fn set_billing_view_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.billing_view_type = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `owner_account_id`.\n"]
    pub fn set_owner_account_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.owner_account_id = Some(v.into());
        self
    }
}
impl ToListMappable for DataBillingViewsBillingViewEl {
    type O = BlockAssignable<DataBillingViewsBillingViewEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBillingViewsBillingViewEl {}
impl BuildDataBillingViewsBillingViewEl {
    pub fn build(self) -> DataBillingViewsBillingViewEl {
        DataBillingViewsBillingViewEl {
            arn: core::default::Default::default(),
            billing_view_type: core::default::Default::default(),
            description: core::default::Default::default(),
            name: core::default::Default::default(),
            owner_account_id: core::default::Default::default(),
        }
    }
}
pub struct DataBillingViewsBillingViewElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBillingViewsBillingViewElRef {
    fn new(shared: StackShared, base: String) -> DataBillingViewsBillingViewElRef {
        DataBillingViewsBillingViewElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBillingViewsBillingViewElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }
    #[doc = "Get a reference to the value of field `billing_view_type` after provisioning.\n"]
    pub fn billing_view_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.billing_view_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `owner_account_id` after provisioning.\n"]
    pub fn owner_account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.owner_account_id", self.base),
        )
    }
}
