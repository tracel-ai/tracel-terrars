use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataSqlTiersData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
}
struct DataSqlTiers_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataSqlTiersData>,
}
#[derive(Clone)]
pub struct DataSqlTiers(Rc<DataSqlTiers_>);
impl DataSqlTiers {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(&self, provider: &ProviderGoogle) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\nProject ID of the project for which to list tiers."]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nProject ID of the project for which to list tiers."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tiers` after provisioning.\n"]
    pub fn tiers(&self) -> ListRef<DataSqlTiersTiersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tiers", self.extract_ref()),
        )
    }
}
impl Referable for DataSqlTiers {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataSqlTiers {}
impl ToListMappable for DataSqlTiers {
    type O = ListRef<DataSqlTiersRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataSqlTiers_ {
    fn extract_datasource_type(&self) -> String {
        "google_sql_tiers".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataSqlTiers {
    pub tf_id: String,
}
impl BuildDataSqlTiers {
    pub fn build(self, stack: &mut Stack) -> DataSqlTiers {
        let out = DataSqlTiers(Rc::new(DataSqlTiers_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataSqlTiersData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                project: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataSqlTiersRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataSqlTiersRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataSqlTiersRef {
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
    #[doc = "Get a reference to the value of field `project` after provisioning.\nProject ID of the project for which to list tiers."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tiers` after provisioning.\n"]
    pub fn tiers(&self) -> ListRef<DataSqlTiersTiersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tiers", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataSqlTiersTiersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_quota: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ram: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tier: Option<PrimField<String>>,
}
impl DataSqlTiersTiersEl {
    #[doc = "Set the field `disk_quota`.\n"]
    pub fn set_disk_quota(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.disk_quota = Some(v.into());
        self
    }
    #[doc = "Set the field `ram`.\n"]
    pub fn set_ram(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.ram = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\n"]
    pub fn set_region(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.region = Some(v.into());
        self
    }
    #[doc = "Set the field `tier`.\n"]
    pub fn set_tier(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tier = Some(v.into());
        self
    }
}
impl ToListMappable for DataSqlTiersTiersEl {
    type O = BlockAssignable<DataSqlTiersTiersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataSqlTiersTiersEl {}
impl BuildDataSqlTiersTiersEl {
    pub fn build(self) -> DataSqlTiersTiersEl {
        DataSqlTiersTiersEl {
            disk_quota: core::default::Default::default(),
            ram: core::default::Default::default(),
            region: core::default::Default::default(),
            tier: core::default::Default::default(),
        }
    }
}
pub struct DataSqlTiersTiersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataSqlTiersTiersElRef {
    fn new(shared: StackShared, base: String) -> DataSqlTiersTiersElRef {
        DataSqlTiersTiersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataSqlTiersTiersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `disk_quota` after provisioning.\n"]
    pub fn disk_quota(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk_quota", self.base))
    }
    #[doc = "Get a reference to the value of field `ram` after provisioning.\n"]
    pub fn ram(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.ram", self.base))
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.region", self.base))
    }
    #[doc = "Get a reference to the value of field `tier` after provisioning.\n"]
    pub fn tier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tier", self.base))
    }
}
