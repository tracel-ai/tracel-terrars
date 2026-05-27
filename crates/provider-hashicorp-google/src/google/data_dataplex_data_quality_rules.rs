use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataDataplexDataQualityRulesData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    data_scan_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
}
struct DataDataplexDataQualityRules_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataDataplexDataQualityRulesData>,
}
#[derive(Clone)]
pub struct DataDataplexDataQualityRules(Rc<DataDataplexDataQualityRules_>);
impl DataDataplexDataQualityRules {
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
    #[doc = "Set the field `location`.\n"]
    pub fn set_location(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().location = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `data_scan_id` after provisioning.\n"]
    pub fn data_scan_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_scan_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> ListRef<DataDataplexDataQualityRulesRulesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rules", self.extract_ref()),
        )
    }
}
impl Referable for DataDataplexDataQualityRules {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataDataplexDataQualityRules {}
impl ToListMappable for DataDataplexDataQualityRules {
    type O = ListRef<DataDataplexDataQualityRulesRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataDataplexDataQualityRules_ {
    fn extract_datasource_type(&self) -> String {
        "google_dataplex_data_quality_rules".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataDataplexDataQualityRules {
    pub tf_id: String,
    #[doc = ""]
    pub data_scan_id: PrimField<String>,
}
impl BuildDataDataplexDataQualityRules {
    pub fn build(self, stack: &mut Stack) -> DataDataplexDataQualityRules {
        let out = DataDataplexDataQualityRules(Rc::new(DataDataplexDataQualityRules_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataDataplexDataQualityRulesData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                data_scan_id: self.data_scan_id,
                id: core::default::Default::default(),
                location: core::default::Default::default(),
                project: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataDataplexDataQualityRulesRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataDataplexDataQualityRulesRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataDataplexDataQualityRulesRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `data_scan_id` after provisioning.\n"]
    pub fn data_scan_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_scan_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> ListRef<DataDataplexDataQualityRulesRulesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rules", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataDataplexDataQualityRulesRulesElNonNullExpectationEl {}
impl DataDataplexDataQualityRulesRulesElNonNullExpectationEl {}
impl ToListMappable for DataDataplexDataQualityRulesRulesElNonNullExpectationEl {
    type O = BlockAssignable<DataDataplexDataQualityRulesRulesElNonNullExpectationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataDataplexDataQualityRulesRulesElNonNullExpectationEl {}
impl BuildDataDataplexDataQualityRulesRulesElNonNullExpectationEl {
    pub fn build(self) -> DataDataplexDataQualityRulesRulesElNonNullExpectationEl {
        DataDataplexDataQualityRulesRulesElNonNullExpectationEl {}
    }
}
pub struct DataDataplexDataQualityRulesRulesElNonNullExpectationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataDataplexDataQualityRulesRulesElNonNullExpectationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataDataplexDataQualityRulesRulesElNonNullExpectationElRef {
        DataDataplexDataQualityRulesRulesElNonNullExpectationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataDataplexDataQualityRulesRulesElNonNullExpectationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct DataDataplexDataQualityRulesRulesElRangeExpectationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_value: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_value: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    strict_max_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    strict_min_enabled: Option<PrimField<bool>>,
}
impl DataDataplexDataQualityRulesRulesElRangeExpectationEl {
    #[doc = "Set the field `max_value`.\n"]
    pub fn set_max_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.max_value = Some(v.into());
        self
    }
    #[doc = "Set the field `min_value`.\n"]
    pub fn set_min_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.min_value = Some(v.into());
        self
    }
    #[doc = "Set the field `strict_max_enabled`.\n"]
    pub fn set_strict_max_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.strict_max_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `strict_min_enabled`.\n"]
    pub fn set_strict_min_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.strict_min_enabled = Some(v.into());
        self
    }
}
impl ToListMappable for DataDataplexDataQualityRulesRulesElRangeExpectationEl {
    type O = BlockAssignable<DataDataplexDataQualityRulesRulesElRangeExpectationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataDataplexDataQualityRulesRulesElRangeExpectationEl {}
impl BuildDataDataplexDataQualityRulesRulesElRangeExpectationEl {
    pub fn build(self) -> DataDataplexDataQualityRulesRulesElRangeExpectationEl {
        DataDataplexDataQualityRulesRulesElRangeExpectationEl {
            max_value: core::default::Default::default(),
            min_value: core::default::Default::default(),
            strict_max_enabled: core::default::Default::default(),
            strict_min_enabled: core::default::Default::default(),
        }
    }
}
pub struct DataDataplexDataQualityRulesRulesElRangeExpectationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataDataplexDataQualityRulesRulesElRangeExpectationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataDataplexDataQualityRulesRulesElRangeExpectationElRef {
        DataDataplexDataQualityRulesRulesElRangeExpectationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataDataplexDataQualityRulesRulesElRangeExpectationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `max_value` after provisioning.\n"]
    pub fn max_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_value", self.base))
    }
    #[doc = "Get a reference to the value of field `min_value` after provisioning.\n"]
    pub fn min_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_value", self.base))
    }
    #[doc = "Get a reference to the value of field `strict_max_enabled` after provisioning.\n"]
    pub fn strict_max_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.strict_max_enabled", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `strict_min_enabled` after provisioning.\n"]
    pub fn strict_min_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.strict_min_enabled", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataDataplexDataQualityRulesRulesElRegexExpectationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    regex: Option<PrimField<String>>,
}
impl DataDataplexDataQualityRulesRulesElRegexExpectationEl {
    #[doc = "Set the field `regex`.\n"]
    pub fn set_regex(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.regex = Some(v.into());
        self
    }
}
impl ToListMappable for DataDataplexDataQualityRulesRulesElRegexExpectationEl {
    type O = BlockAssignable<DataDataplexDataQualityRulesRulesElRegexExpectationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataDataplexDataQualityRulesRulesElRegexExpectationEl {}
impl BuildDataDataplexDataQualityRulesRulesElRegexExpectationEl {
    pub fn build(self) -> DataDataplexDataQualityRulesRulesElRegexExpectationEl {
        DataDataplexDataQualityRulesRulesElRegexExpectationEl {
            regex: core::default::Default::default(),
        }
    }
}
pub struct DataDataplexDataQualityRulesRulesElRegexExpectationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataDataplexDataQualityRulesRulesElRegexExpectationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataDataplexDataQualityRulesRulesElRegexExpectationElRef {
        DataDataplexDataQualityRulesRulesElRegexExpectationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataDataplexDataQualityRulesRulesElRegexExpectationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `regex` after provisioning.\n"]
    pub fn regex(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.regex", self.base))
    }
}
#[derive(Serialize)]
pub struct DataDataplexDataQualityRulesRulesElRowConditionExpectationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    sql_expression: Option<PrimField<String>>,
}
impl DataDataplexDataQualityRulesRulesElRowConditionExpectationEl {
    #[doc = "Set the field `sql_expression`.\n"]
    pub fn set_sql_expression(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sql_expression = Some(v.into());
        self
    }
}
impl ToListMappable for DataDataplexDataQualityRulesRulesElRowConditionExpectationEl {
    type O = BlockAssignable<DataDataplexDataQualityRulesRulesElRowConditionExpectationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataDataplexDataQualityRulesRulesElRowConditionExpectationEl {}
impl BuildDataDataplexDataQualityRulesRulesElRowConditionExpectationEl {
    pub fn build(self) -> DataDataplexDataQualityRulesRulesElRowConditionExpectationEl {
        DataDataplexDataQualityRulesRulesElRowConditionExpectationEl {
            sql_expression: core::default::Default::default(),
        }
    }
}
pub struct DataDataplexDataQualityRulesRulesElRowConditionExpectationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataDataplexDataQualityRulesRulesElRowConditionExpectationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataDataplexDataQualityRulesRulesElRowConditionExpectationElRef {
        DataDataplexDataQualityRulesRulesElRowConditionExpectationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataDataplexDataQualityRulesRulesElRowConditionExpectationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `sql_expression` after provisioning.\n"]
    pub fn sql_expression(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sql_expression", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataDataplexDataQualityRulesRulesElSetExpectationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<ListField<PrimField<String>>>,
}
impl DataDataplexDataQualityRulesRulesElSetExpectationEl {
    #[doc = "Set the field `values`.\n"]
    pub fn set_values(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}
impl ToListMappable for DataDataplexDataQualityRulesRulesElSetExpectationEl {
    type O = BlockAssignable<DataDataplexDataQualityRulesRulesElSetExpectationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataDataplexDataQualityRulesRulesElSetExpectationEl {}
impl BuildDataDataplexDataQualityRulesRulesElSetExpectationEl {
    pub fn build(self) -> DataDataplexDataQualityRulesRulesElSetExpectationEl {
        DataDataplexDataQualityRulesRulesElSetExpectationEl {
            values: core::default::Default::default(),
        }
    }
}
pub struct DataDataplexDataQualityRulesRulesElSetExpectationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataDataplexDataQualityRulesRulesElSetExpectationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataDataplexDataQualityRulesRulesElSetExpectationElRef {
        DataDataplexDataQualityRulesRulesElSetExpectationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataDataplexDataQualityRulesRulesElSetExpectationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}
#[derive(Serialize)]
pub struct DataDataplexDataQualityRulesRulesElSqlAssertionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    sql_statement: Option<PrimField<String>>,
}
impl DataDataplexDataQualityRulesRulesElSqlAssertionEl {
    #[doc = "Set the field `sql_statement`.\n"]
    pub fn set_sql_statement(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sql_statement = Some(v.into());
        self
    }
}
impl ToListMappable for DataDataplexDataQualityRulesRulesElSqlAssertionEl {
    type O = BlockAssignable<DataDataplexDataQualityRulesRulesElSqlAssertionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataDataplexDataQualityRulesRulesElSqlAssertionEl {}
impl BuildDataDataplexDataQualityRulesRulesElSqlAssertionEl {
    pub fn build(self) -> DataDataplexDataQualityRulesRulesElSqlAssertionEl {
        DataDataplexDataQualityRulesRulesElSqlAssertionEl {
            sql_statement: core::default::Default::default(),
        }
    }
}
pub struct DataDataplexDataQualityRulesRulesElSqlAssertionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataDataplexDataQualityRulesRulesElSqlAssertionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataDataplexDataQualityRulesRulesElSqlAssertionElRef {
        DataDataplexDataQualityRulesRulesElSqlAssertionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataDataplexDataQualityRulesRulesElSqlAssertionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `sql_statement` after provisioning.\n"]
    pub fn sql_statement(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sql_statement", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataDataplexDataQualityRulesRulesElStatisticRangeExpectationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_value: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_value: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    statistic: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    strict_max_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    strict_min_enabled: Option<PrimField<bool>>,
}
impl DataDataplexDataQualityRulesRulesElStatisticRangeExpectationEl {
    #[doc = "Set the field `max_value`.\n"]
    pub fn set_max_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.max_value = Some(v.into());
        self
    }
    #[doc = "Set the field `min_value`.\n"]
    pub fn set_min_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.min_value = Some(v.into());
        self
    }
    #[doc = "Set the field `statistic`.\n"]
    pub fn set_statistic(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.statistic = Some(v.into());
        self
    }
    #[doc = "Set the field `strict_max_enabled`.\n"]
    pub fn set_strict_max_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.strict_max_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `strict_min_enabled`.\n"]
    pub fn set_strict_min_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.strict_min_enabled = Some(v.into());
        self
    }
}
impl ToListMappable for DataDataplexDataQualityRulesRulesElStatisticRangeExpectationEl {
    type O = BlockAssignable<DataDataplexDataQualityRulesRulesElStatisticRangeExpectationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataDataplexDataQualityRulesRulesElStatisticRangeExpectationEl {}
impl BuildDataDataplexDataQualityRulesRulesElStatisticRangeExpectationEl {
    pub fn build(self) -> DataDataplexDataQualityRulesRulesElStatisticRangeExpectationEl {
        DataDataplexDataQualityRulesRulesElStatisticRangeExpectationEl {
            max_value: core::default::Default::default(),
            min_value: core::default::Default::default(),
            statistic: core::default::Default::default(),
            strict_max_enabled: core::default::Default::default(),
            strict_min_enabled: core::default::Default::default(),
        }
    }
}
pub struct DataDataplexDataQualityRulesRulesElStatisticRangeExpectationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataDataplexDataQualityRulesRulesElStatisticRangeExpectationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataDataplexDataQualityRulesRulesElStatisticRangeExpectationElRef {
        DataDataplexDataQualityRulesRulesElStatisticRangeExpectationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataDataplexDataQualityRulesRulesElStatisticRangeExpectationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `max_value` after provisioning.\n"]
    pub fn max_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_value", self.base))
    }
    #[doc = "Get a reference to the value of field `min_value` after provisioning.\n"]
    pub fn min_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_value", self.base))
    }
    #[doc = "Get a reference to the value of field `statistic` after provisioning.\n"]
    pub fn statistic(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.statistic", self.base))
    }
    #[doc = "Get a reference to the value of field `strict_max_enabled` after provisioning.\n"]
    pub fn strict_max_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.strict_max_enabled", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `strict_min_enabled` after provisioning.\n"]
    pub fn strict_min_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.strict_min_enabled", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataDataplexDataQualityRulesRulesElTableConditionExpectationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    sql_expression: Option<PrimField<String>>,
}
impl DataDataplexDataQualityRulesRulesElTableConditionExpectationEl {
    #[doc = "Set the field `sql_expression`.\n"]
    pub fn set_sql_expression(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sql_expression = Some(v.into());
        self
    }
}
impl ToListMappable for DataDataplexDataQualityRulesRulesElTableConditionExpectationEl {
    type O = BlockAssignable<DataDataplexDataQualityRulesRulesElTableConditionExpectationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataDataplexDataQualityRulesRulesElTableConditionExpectationEl {}
impl BuildDataDataplexDataQualityRulesRulesElTableConditionExpectationEl {
    pub fn build(self) -> DataDataplexDataQualityRulesRulesElTableConditionExpectationEl {
        DataDataplexDataQualityRulesRulesElTableConditionExpectationEl {
            sql_expression: core::default::Default::default(),
        }
    }
}
pub struct DataDataplexDataQualityRulesRulesElTableConditionExpectationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataDataplexDataQualityRulesRulesElTableConditionExpectationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataDataplexDataQualityRulesRulesElTableConditionExpectationElRef {
        DataDataplexDataQualityRulesRulesElTableConditionExpectationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataDataplexDataQualityRulesRulesElTableConditionExpectationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `sql_expression` after provisioning.\n"]
    pub fn sql_expression(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sql_expression", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataDataplexDataQualityRulesRulesElUniquenessExpectationEl {}
impl DataDataplexDataQualityRulesRulesElUniquenessExpectationEl {}
impl ToListMappable for DataDataplexDataQualityRulesRulesElUniquenessExpectationEl {
    type O = BlockAssignable<DataDataplexDataQualityRulesRulesElUniquenessExpectationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataDataplexDataQualityRulesRulesElUniquenessExpectationEl {}
impl BuildDataDataplexDataQualityRulesRulesElUniquenessExpectationEl {
    pub fn build(self) -> DataDataplexDataQualityRulesRulesElUniquenessExpectationEl {
        DataDataplexDataQualityRulesRulesElUniquenessExpectationEl {}
    }
}
pub struct DataDataplexDataQualityRulesRulesElUniquenessExpectationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataDataplexDataQualityRulesRulesElUniquenessExpectationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataDataplexDataQualityRulesRulesElUniquenessExpectationElRef {
        DataDataplexDataQualityRulesRulesElUniquenessExpectationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataDataplexDataQualityRulesRulesElUniquenessExpectationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct DataDataplexDataQualityRulesRulesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    column: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dimension: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ignore_null: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    non_null_expectation:
        Option<ListField<DataDataplexDataQualityRulesRulesElNonNullExpectationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    range_expectation: Option<ListField<DataDataplexDataQualityRulesRulesElRangeExpectationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    regex_expectation: Option<ListField<DataDataplexDataQualityRulesRulesElRegexExpectationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    row_condition_expectation:
        Option<ListField<DataDataplexDataQualityRulesRulesElRowConditionExpectationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    set_expectation: Option<ListField<DataDataplexDataQualityRulesRulesElSetExpectationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sql_assertion: Option<ListField<DataDataplexDataQualityRulesRulesElSqlAssertionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    statistic_range_expectation:
        Option<ListField<DataDataplexDataQualityRulesRulesElStatisticRangeExpectationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    suspended: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    table_condition_expectation:
        Option<ListField<DataDataplexDataQualityRulesRulesElTableConditionExpectationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    threshold: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    uniqueness_expectation:
        Option<ListField<DataDataplexDataQualityRulesRulesElUniquenessExpectationEl>>,
}
impl DataDataplexDataQualityRulesRulesEl {
    #[doc = "Set the field `column`.\n"]
    pub fn set_column(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.column = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `dimension`.\n"]
    pub fn set_dimension(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dimension = Some(v.into());
        self
    }
    #[doc = "Set the field `ignore_null`.\n"]
    pub fn set_ignore_null(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.ignore_null = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `non_null_expectation`.\n"]
    pub fn set_non_null_expectation(
        mut self,
        v: impl Into<ListField<DataDataplexDataQualityRulesRulesElNonNullExpectationEl>>,
    ) -> Self {
        self.non_null_expectation = Some(v.into());
        self
    }
    #[doc = "Set the field `range_expectation`.\n"]
    pub fn set_range_expectation(
        mut self,
        v: impl Into<ListField<DataDataplexDataQualityRulesRulesElRangeExpectationEl>>,
    ) -> Self {
        self.range_expectation = Some(v.into());
        self
    }
    #[doc = "Set the field `regex_expectation`.\n"]
    pub fn set_regex_expectation(
        mut self,
        v: impl Into<ListField<DataDataplexDataQualityRulesRulesElRegexExpectationEl>>,
    ) -> Self {
        self.regex_expectation = Some(v.into());
        self
    }
    #[doc = "Set the field `row_condition_expectation`.\n"]
    pub fn set_row_condition_expectation(
        mut self,
        v: impl Into<ListField<DataDataplexDataQualityRulesRulesElRowConditionExpectationEl>>,
    ) -> Self {
        self.row_condition_expectation = Some(v.into());
        self
    }
    #[doc = "Set the field `set_expectation`.\n"]
    pub fn set_set_expectation(
        mut self,
        v: impl Into<ListField<DataDataplexDataQualityRulesRulesElSetExpectationEl>>,
    ) -> Self {
        self.set_expectation = Some(v.into());
        self
    }
    #[doc = "Set the field `sql_assertion`.\n"]
    pub fn set_sql_assertion(
        mut self,
        v: impl Into<ListField<DataDataplexDataQualityRulesRulesElSqlAssertionEl>>,
    ) -> Self {
        self.sql_assertion = Some(v.into());
        self
    }
    #[doc = "Set the field `statistic_range_expectation`.\n"]
    pub fn set_statistic_range_expectation(
        mut self,
        v: impl Into<ListField<DataDataplexDataQualityRulesRulesElStatisticRangeExpectationEl>>,
    ) -> Self {
        self.statistic_range_expectation = Some(v.into());
        self
    }
    #[doc = "Set the field `suspended`.\n"]
    pub fn set_suspended(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.suspended = Some(v.into());
        self
    }
    #[doc = "Set the field `table_condition_expectation`.\n"]
    pub fn set_table_condition_expectation(
        mut self,
        v: impl Into<ListField<DataDataplexDataQualityRulesRulesElTableConditionExpectationEl>>,
    ) -> Self {
        self.table_condition_expectation = Some(v.into());
        self
    }
    #[doc = "Set the field `threshold`.\n"]
    pub fn set_threshold(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.threshold = Some(v.into());
        self
    }
    #[doc = "Set the field `uniqueness_expectation`.\n"]
    pub fn set_uniqueness_expectation(
        mut self,
        v: impl Into<ListField<DataDataplexDataQualityRulesRulesElUniquenessExpectationEl>>,
    ) -> Self {
        self.uniqueness_expectation = Some(v.into());
        self
    }
}
impl ToListMappable for DataDataplexDataQualityRulesRulesEl {
    type O = BlockAssignable<DataDataplexDataQualityRulesRulesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataDataplexDataQualityRulesRulesEl {}
impl BuildDataDataplexDataQualityRulesRulesEl {
    pub fn build(self) -> DataDataplexDataQualityRulesRulesEl {
        DataDataplexDataQualityRulesRulesEl {
            column: core::default::Default::default(),
            description: core::default::Default::default(),
            dimension: core::default::Default::default(),
            ignore_null: core::default::Default::default(),
            name: core::default::Default::default(),
            non_null_expectation: core::default::Default::default(),
            range_expectation: core::default::Default::default(),
            regex_expectation: core::default::Default::default(),
            row_condition_expectation: core::default::Default::default(),
            set_expectation: core::default::Default::default(),
            sql_assertion: core::default::Default::default(),
            statistic_range_expectation: core::default::Default::default(),
            suspended: core::default::Default::default(),
            table_condition_expectation: core::default::Default::default(),
            threshold: core::default::Default::default(),
            uniqueness_expectation: core::default::Default::default(),
        }
    }
}
pub struct DataDataplexDataQualityRulesRulesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataDataplexDataQualityRulesRulesElRef {
    fn new(shared: StackShared, base: String) -> DataDataplexDataQualityRulesRulesElRef {
        DataDataplexDataQualityRulesRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataDataplexDataQualityRulesRulesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `column` after provisioning.\n"]
    pub fn column(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.column", self.base))
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `dimension` after provisioning.\n"]
    pub fn dimension(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dimension", self.base))
    }
    #[doc = "Get a reference to the value of field `ignore_null` after provisioning.\n"]
    pub fn ignore_null(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.ignore_null", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `non_null_expectation` after provisioning.\n"]
    pub fn non_null_expectation(
        &self,
    ) -> ListRef<DataDataplexDataQualityRulesRulesElNonNullExpectationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.non_null_expectation", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `range_expectation` after provisioning.\n"]
    pub fn range_expectation(
        &self,
    ) -> ListRef<DataDataplexDataQualityRulesRulesElRangeExpectationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.range_expectation", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `regex_expectation` after provisioning.\n"]
    pub fn regex_expectation(
        &self,
    ) -> ListRef<DataDataplexDataQualityRulesRulesElRegexExpectationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.regex_expectation", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `row_condition_expectation` after provisioning.\n"]
    pub fn row_condition_expectation(
        &self,
    ) -> ListRef<DataDataplexDataQualityRulesRulesElRowConditionExpectationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.row_condition_expectation", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `set_expectation` after provisioning.\n"]
    pub fn set_expectation(
        &self,
    ) -> ListRef<DataDataplexDataQualityRulesRulesElSetExpectationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.set_expectation", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `sql_assertion` after provisioning.\n"]
    pub fn sql_assertion(&self) -> ListRef<DataDataplexDataQualityRulesRulesElSqlAssertionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.sql_assertion", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `statistic_range_expectation` after provisioning.\n"]
    pub fn statistic_range_expectation(
        &self,
    ) -> ListRef<DataDataplexDataQualityRulesRulesElStatisticRangeExpectationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.statistic_range_expectation", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `suspended` after provisioning.\n"]
    pub fn suspended(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.suspended", self.base))
    }
    #[doc = "Get a reference to the value of field `table_condition_expectation` after provisioning.\n"]
    pub fn table_condition_expectation(
        &self,
    ) -> ListRef<DataDataplexDataQualityRulesRulesElTableConditionExpectationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.table_condition_expectation", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `threshold` after provisioning.\n"]
    pub fn threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.threshold", self.base))
    }
    #[doc = "Get a reference to the value of field `uniqueness_expectation` after provisioning.\n"]
    pub fn uniqueness_expectation(
        &self,
    ) -> ListRef<DataDataplexDataQualityRulesRulesElUniquenessExpectationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.uniqueness_expectation", self.base),
        )
    }
}
