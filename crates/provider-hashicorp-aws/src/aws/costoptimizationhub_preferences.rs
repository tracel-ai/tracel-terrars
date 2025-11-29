use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct CostoptimizationhubPreferencesData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    member_account_discount_visibility: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    savings_estimation_mode: Option<PrimField<String>>,
}

struct CostoptimizationhubPreferences_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CostoptimizationhubPreferencesData>,
}

#[derive(Clone)]
pub struct CostoptimizationhubPreferences(Rc<CostoptimizationhubPreferences_>);

impl CostoptimizationhubPreferences {
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

    #[doc = "Set the field `member_account_discount_visibility`.\n"]
    pub fn set_member_account_discount_visibility(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().member_account_discount_visibility = Some(v.into());
        self
    }

    #[doc = "Set the field `savings_estimation_mode`.\n"]
    pub fn set_savings_estimation_mode(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().savings_estimation_mode = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `member_account_discount_visibility` after provisioning.\n"]
    pub fn member_account_discount_visibility(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.member_account_discount_visibility", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `savings_estimation_mode` after provisioning.\n"]
    pub fn savings_estimation_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.savings_estimation_mode", self.extract_ref()))
    }
}

impl Referable for CostoptimizationhubPreferences {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for CostoptimizationhubPreferences { }

impl ToListMappable for CostoptimizationhubPreferences {
    type O = ListRef<CostoptimizationhubPreferencesRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for CostoptimizationhubPreferences_ {
    fn extract_resource_type(&self) -> String {
        "aws_costoptimizationhub_preferences".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCostoptimizationhubPreferences {
    pub tf_id: String,
}

impl BuildCostoptimizationhubPreferences {
    pub fn build(self, stack: &mut Stack) -> CostoptimizationhubPreferences {
        let out = CostoptimizationhubPreferences(Rc::new(CostoptimizationhubPreferences_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(CostoptimizationhubPreferencesData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                member_account_discount_visibility: core::default::Default::default(),
                savings_estimation_mode: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CostoptimizationhubPreferencesRef {
    shared: StackShared,
    base: String,
}

impl Ref for CostoptimizationhubPreferencesRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl CostoptimizationhubPreferencesRef {
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

    #[doc = "Get a reference to the value of field `member_account_discount_visibility` after provisioning.\n"]
    pub fn member_account_discount_visibility(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.member_account_discount_visibility", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `savings_estimation_mode` after provisioning.\n"]
    pub fn savings_estimation_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.savings_estimation_mode", self.extract_ref()))
    }
}
