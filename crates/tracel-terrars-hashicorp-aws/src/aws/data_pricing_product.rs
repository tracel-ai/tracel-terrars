use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataPricingProductData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    service_code: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filters: Option<Vec<DataPricingProductFiltersEl>>,
    dynamic: DataPricingProductDynamic,
}

struct DataPricingProduct_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataPricingProductData>,
}

#[derive(Clone)]
pub struct DataPricingProduct(Rc<DataPricingProduct_>);

impl DataPricingProduct {
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

    #[doc = "Set the field `filters`.\n"]
    pub fn set_filters(self, v: impl Into<BlockAssignable<DataPricingProductFiltersEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().filters = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.filters = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\n"]
    pub fn result(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_code` after provisioning.\n"]
    pub fn service_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_code", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filters` after provisioning.\n"]
    pub fn filters(&self) -> ListRef<DataPricingProductFiltersElRef> {
        ListRef::new(self.shared().clone(), format!("{}.filters", self.extract_ref()))
    }
}

impl Referable for DataPricingProduct {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataPricingProduct { }

impl ToListMappable for DataPricingProduct {
    type O = ListRef<DataPricingProductRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataPricingProduct_ {
    fn extract_datasource_type(&self) -> String {
        "aws_pricing_product".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataPricingProduct {
    pub tf_id: String,
    #[doc = ""]
    pub service_code: PrimField<String>,
}

impl BuildDataPricingProduct {
    pub fn build(self, stack: &mut Stack) -> DataPricingProduct {
        let out = DataPricingProduct(Rc::new(DataPricingProduct_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataPricingProductData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                service_code: self.service_code,
                filters: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataPricingProductRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataPricingProductRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataPricingProductRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\n"]
    pub fn result(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_code` after provisioning.\n"]
    pub fn service_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_code", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filters` after provisioning.\n"]
    pub fn filters(&self) -> ListRef<DataPricingProductFiltersElRef> {
        ListRef::new(self.shared().clone(), format!("{}.filters", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataPricingProductFiltersEl {
    field: PrimField<String>,
    value: PrimField<String>,
}

impl DataPricingProductFiltersEl { }

impl ToListMappable for DataPricingProductFiltersEl {
    type O = BlockAssignable<DataPricingProductFiltersEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPricingProductFiltersEl {
    #[doc = ""]
    pub field: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildDataPricingProductFiltersEl {
    pub fn build(self) -> DataPricingProductFiltersEl {
        DataPricingProductFiltersEl {
            field: self.field,
            value: self.value,
        }
    }
}

pub struct DataPricingProductFiltersElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataPricingProductFiltersElRef {
    fn new(shared: StackShared, base: String) -> DataPricingProductFiltersElRef {
        DataPricingProductFiltersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPricingProductFiltersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `field` after provisioning.\n"]
    pub fn field(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.field", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataPricingProductDynamic {
    filters: Option<DynamicBlock<DataPricingProductFiltersEl>>,
}
