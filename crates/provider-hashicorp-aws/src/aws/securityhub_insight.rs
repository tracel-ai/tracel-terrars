use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct SecurityhubInsightData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    group_by_attribute: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filters: Option<Vec<SecurityhubInsightFiltersEl>>,
    dynamic: SecurityhubInsightDynamic,
}
struct SecurityhubInsight_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SecurityhubInsightData>,
}
#[derive(Clone)]
pub struct SecurityhubInsight(Rc<SecurityhubInsight_>);
impl SecurityhubInsight {
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
    #[doc = "Set the field `filters`.\n"]
    pub fn set_filters(self, v: impl Into<BlockAssignable<SecurityhubInsightFiltersEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().filters = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.filters = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `group_by_attribute` after provisioning.\n"]
    pub fn group_by_attribute(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.group_by_attribute", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `filters` after provisioning.\n"]
    pub fn filters(&self) -> ListRef<SecurityhubInsightFiltersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.filters", self.extract_ref()),
        )
    }
}
impl Referable for SecurityhubInsight {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for SecurityhubInsight {}
impl ToListMappable for SecurityhubInsight {
    type O = ListRef<SecurityhubInsightRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for SecurityhubInsight_ {
    fn extract_resource_type(&self) -> String {
        "aws_securityhub_insight".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildSecurityhubInsight {
    pub tf_id: String,
    #[doc = ""]
    pub group_by_attribute: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
}
impl BuildSecurityhubInsight {
    pub fn build(self, stack: &mut Stack) -> SecurityhubInsight {
        let out = SecurityhubInsight(Rc::new(SecurityhubInsight_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(SecurityhubInsightData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                group_by_attribute: self.group_by_attribute,
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                filters: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct SecurityhubInsightRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl SecurityhubInsightRef {
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
    #[doc = "Get a reference to the value of field `group_by_attribute` after provisioning.\n"]
    pub fn group_by_attribute(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.group_by_attribute", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `filters` after provisioning.\n"]
    pub fn filters(&self) -> ListRef<SecurityhubInsightFiltersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.filters", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElAwsAccountIdEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}
impl SecurityhubInsightFiltersElAwsAccountIdEl {}
impl ToListMappable for SecurityhubInsightFiltersElAwsAccountIdEl {
    type O = BlockAssignable<SecurityhubInsightFiltersElAwsAccountIdEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElAwsAccountIdEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildSecurityhubInsightFiltersElAwsAccountIdEl {
    pub fn build(self) -> SecurityhubInsightFiltersElAwsAccountIdEl {
        SecurityhubInsightFiltersElAwsAccountIdEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}
pub struct SecurityhubInsightFiltersElAwsAccountIdElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElAwsAccountIdElRef {
    fn new(shared: StackShared, base: String) -> SecurityhubInsightFiltersElAwsAccountIdElRef {
        SecurityhubInsightFiltersElAwsAccountIdElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElAwsAccountIdElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElCompanyNameEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}
impl SecurityhubInsightFiltersElCompanyNameEl {}
impl ToListMappable for SecurityhubInsightFiltersElCompanyNameEl {
    type O = BlockAssignable<SecurityhubInsightFiltersElCompanyNameEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElCompanyNameEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildSecurityhubInsightFiltersElCompanyNameEl {
    pub fn build(self) -> SecurityhubInsightFiltersElCompanyNameEl {
        SecurityhubInsightFiltersElCompanyNameEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}
pub struct SecurityhubInsightFiltersElCompanyNameElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElCompanyNameElRef {
    fn new(shared: StackShared, base: String) -> SecurityhubInsightFiltersElCompanyNameElRef {
        SecurityhubInsightFiltersElCompanyNameElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElCompanyNameElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElComplianceStatusEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}
impl SecurityhubInsightFiltersElComplianceStatusEl {}
impl ToListMappable for SecurityhubInsightFiltersElComplianceStatusEl {
    type O = BlockAssignable<SecurityhubInsightFiltersElComplianceStatusEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElComplianceStatusEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildSecurityhubInsightFiltersElComplianceStatusEl {
    pub fn build(self) -> SecurityhubInsightFiltersElComplianceStatusEl {
        SecurityhubInsightFiltersElComplianceStatusEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}
pub struct SecurityhubInsightFiltersElComplianceStatusElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElComplianceStatusElRef {
    fn new(shared: StackShared, base: String) -> SecurityhubInsightFiltersElComplianceStatusElRef {
        SecurityhubInsightFiltersElComplianceStatusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElComplianceStatusElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElConfidenceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    eq: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gte: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lte: Option<PrimField<String>>,
}
impl SecurityhubInsightFiltersElConfidenceEl {
    #[doc = "Set the field `eq`.\n"]
    pub fn set_eq(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.eq = Some(v.into());
        self
    }
    #[doc = "Set the field `gte`.\n"]
    pub fn set_gte(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gte = Some(v.into());
        self
    }
    #[doc = "Set the field `lte`.\n"]
    pub fn set_lte(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.lte = Some(v.into());
        self
    }
}
impl ToListMappable for SecurityhubInsightFiltersElConfidenceEl {
    type O = BlockAssignable<SecurityhubInsightFiltersElConfidenceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElConfidenceEl {}
impl BuildSecurityhubInsightFiltersElConfidenceEl {
    pub fn build(self) -> SecurityhubInsightFiltersElConfidenceEl {
        SecurityhubInsightFiltersElConfidenceEl {
            eq: core::default::Default::default(),
            gte: core::default::Default::default(),
            lte: core::default::Default::default(),
        }
    }
}
pub struct SecurityhubInsightFiltersElConfidenceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElConfidenceElRef {
    fn new(shared: StackShared, base: String) -> SecurityhubInsightFiltersElConfidenceElRef {
        SecurityhubInsightFiltersElConfidenceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElConfidenceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `eq` after provisioning.\n"]
    pub fn eq(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.eq", self.base))
    }
    #[doc = "Get a reference to the value of field `gte` after provisioning.\n"]
    pub fn gte(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.gte", self.base))
    }
    #[doc = "Get a reference to the value of field `lte` after provisioning.\n"]
    pub fn lte(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.lte", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElCreatedAtElDateRangeEl {
    unit: PrimField<String>,
    value: PrimField<f64>,
}
impl SecurityhubInsightFiltersElCreatedAtElDateRangeEl {}
impl ToListMappable for SecurityhubInsightFiltersElCreatedAtElDateRangeEl {
    type O = BlockAssignable<SecurityhubInsightFiltersElCreatedAtElDateRangeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElCreatedAtElDateRangeEl {
    #[doc = ""]
    pub unit: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<f64>,
}
impl BuildSecurityhubInsightFiltersElCreatedAtElDateRangeEl {
    pub fn build(self) -> SecurityhubInsightFiltersElCreatedAtElDateRangeEl {
        SecurityhubInsightFiltersElCreatedAtElDateRangeEl {
            unit: self.unit,
            value: self.value,
        }
    }
}
pub struct SecurityhubInsightFiltersElCreatedAtElDateRangeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElCreatedAtElDateRangeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubInsightFiltersElCreatedAtElDateRangeElRef {
        SecurityhubInsightFiltersElCreatedAtElDateRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElCreatedAtElDateRangeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize, Default)]
struct SecurityhubInsightFiltersElCreatedAtElDynamic {
    date_range: Option<DynamicBlock<SecurityhubInsightFiltersElCreatedAtElDateRangeEl>>,
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElCreatedAtEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    end: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    date_range: Option<Vec<SecurityhubInsightFiltersElCreatedAtElDateRangeEl>>,
    dynamic: SecurityhubInsightFiltersElCreatedAtElDynamic,
}
impl SecurityhubInsightFiltersElCreatedAtEl {
    #[doc = "Set the field `end`.\n"]
    pub fn set_end(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.end = Some(v.into());
        self
    }
    #[doc = "Set the field `start`.\n"]
    pub fn set_start(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.start = Some(v.into());
        self
    }
    #[doc = "Set the field `date_range`.\n"]
    pub fn set_date_range(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubInsightFiltersElCreatedAtElDateRangeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.date_range = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.date_range = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for SecurityhubInsightFiltersElCreatedAtEl {
    type O = BlockAssignable<SecurityhubInsightFiltersElCreatedAtEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElCreatedAtEl {}
impl BuildSecurityhubInsightFiltersElCreatedAtEl {
    pub fn build(self) -> SecurityhubInsightFiltersElCreatedAtEl {
        SecurityhubInsightFiltersElCreatedAtEl {
            end: core::default::Default::default(),
            start: core::default::Default::default(),
            date_range: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct SecurityhubInsightFiltersElCreatedAtElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElCreatedAtElRef {
    fn new(shared: StackShared, base: String) -> SecurityhubInsightFiltersElCreatedAtElRef {
        SecurityhubInsightFiltersElCreatedAtElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElCreatedAtElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `end` after provisioning.\n"]
    pub fn end(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.end", self.base))
    }
    #[doc = "Get a reference to the value of field `start` after provisioning.\n"]
    pub fn start(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start", self.base))
    }
    #[doc = "Get a reference to the value of field `date_range` after provisioning.\n"]
    pub fn date_range(&self) -> ListRef<SecurityhubInsightFiltersElCreatedAtElDateRangeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.date_range", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElCriticalityEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    eq: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gte: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lte: Option<PrimField<String>>,
}
impl SecurityhubInsightFiltersElCriticalityEl {
    #[doc = "Set the field `eq`.\n"]
    pub fn set_eq(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.eq = Some(v.into());
        self
    }
    #[doc = "Set the field `gte`.\n"]
    pub fn set_gte(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gte = Some(v.into());
        self
    }
    #[doc = "Set the field `lte`.\n"]
    pub fn set_lte(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.lte = Some(v.into());
        self
    }
}
impl ToListMappable for SecurityhubInsightFiltersElCriticalityEl {
    type O = BlockAssignable<SecurityhubInsightFiltersElCriticalityEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElCriticalityEl {}
impl BuildSecurityhubInsightFiltersElCriticalityEl {
    pub fn build(self) -> SecurityhubInsightFiltersElCriticalityEl {
        SecurityhubInsightFiltersElCriticalityEl {
            eq: core::default::Default::default(),
            gte: core::default::Default::default(),
            lte: core::default::Default::default(),
        }
    }
}
pub struct SecurityhubInsightFiltersElCriticalityElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElCriticalityElRef {
    fn new(shared: StackShared, base: String) -> SecurityhubInsightFiltersElCriticalityElRef {
        SecurityhubInsightFiltersElCriticalityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElCriticalityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `eq` after provisioning.\n"]
    pub fn eq(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.eq", self.base))
    }
    #[doc = "Get a reference to the value of field `gte` after provisioning.\n"]
    pub fn gte(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.gte", self.base))
    }
    #[doc = "Get a reference to the value of field `lte` after provisioning.\n"]
    pub fn lte(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.lte", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElDescriptionEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}
impl SecurityhubInsightFiltersElDescriptionEl {}
impl ToListMappable for SecurityhubInsightFiltersElDescriptionEl {
    type O = BlockAssignable<SecurityhubInsightFiltersElDescriptionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElDescriptionEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildSecurityhubInsightFiltersElDescriptionEl {
    pub fn build(self) -> SecurityhubInsightFiltersElDescriptionEl {
        SecurityhubInsightFiltersElDescriptionEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}
pub struct SecurityhubInsightFiltersElDescriptionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElDescriptionElRef {
    fn new(shared: StackShared, base: String) -> SecurityhubInsightFiltersElDescriptionElRef {
        SecurityhubInsightFiltersElDescriptionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElDescriptionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElFindingProviderFieldsConfidenceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    eq: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gte: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lte: Option<PrimField<String>>,
}
impl SecurityhubInsightFiltersElFindingProviderFieldsConfidenceEl {
    #[doc = "Set the field `eq`.\n"]
    pub fn set_eq(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.eq = Some(v.into());
        self
    }
    #[doc = "Set the field `gte`.\n"]
    pub fn set_gte(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gte = Some(v.into());
        self
    }
    #[doc = "Set the field `lte`.\n"]
    pub fn set_lte(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.lte = Some(v.into());
        self
    }
}
impl ToListMappable for SecurityhubInsightFiltersElFindingProviderFieldsConfidenceEl {
    type O = BlockAssignable<SecurityhubInsightFiltersElFindingProviderFieldsConfidenceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElFindingProviderFieldsConfidenceEl {}
impl BuildSecurityhubInsightFiltersElFindingProviderFieldsConfidenceEl {
    pub fn build(self) -> SecurityhubInsightFiltersElFindingProviderFieldsConfidenceEl {
        SecurityhubInsightFiltersElFindingProviderFieldsConfidenceEl {
            eq: core::default::Default::default(),
            gte: core::default::Default::default(),
            lte: core::default::Default::default(),
        }
    }
}
pub struct SecurityhubInsightFiltersElFindingProviderFieldsConfidenceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElFindingProviderFieldsConfidenceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubInsightFiltersElFindingProviderFieldsConfidenceElRef {
        SecurityhubInsightFiltersElFindingProviderFieldsConfidenceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElFindingProviderFieldsConfidenceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `eq` after provisioning.\n"]
    pub fn eq(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.eq", self.base))
    }
    #[doc = "Get a reference to the value of field `gte` after provisioning.\n"]
    pub fn gte(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.gte", self.base))
    }
    #[doc = "Get a reference to the value of field `lte` after provisioning.\n"]
    pub fn lte(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.lte", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElFindingProviderFieldsCriticalityEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    eq: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gte: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lte: Option<PrimField<String>>,
}
impl SecurityhubInsightFiltersElFindingProviderFieldsCriticalityEl {
    #[doc = "Set the field `eq`.\n"]
    pub fn set_eq(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.eq = Some(v.into());
        self
    }
    #[doc = "Set the field `gte`.\n"]
    pub fn set_gte(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gte = Some(v.into());
        self
    }
    #[doc = "Set the field `lte`.\n"]
    pub fn set_lte(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.lte = Some(v.into());
        self
    }
}
impl ToListMappable for SecurityhubInsightFiltersElFindingProviderFieldsCriticalityEl {
    type O = BlockAssignable<SecurityhubInsightFiltersElFindingProviderFieldsCriticalityEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElFindingProviderFieldsCriticalityEl {}
impl BuildSecurityhubInsightFiltersElFindingProviderFieldsCriticalityEl {
    pub fn build(self) -> SecurityhubInsightFiltersElFindingProviderFieldsCriticalityEl {
        SecurityhubInsightFiltersElFindingProviderFieldsCriticalityEl {
            eq: core::default::Default::default(),
            gte: core::default::Default::default(),
            lte: core::default::Default::default(),
        }
    }
}
pub struct SecurityhubInsightFiltersElFindingProviderFieldsCriticalityElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElFindingProviderFieldsCriticalityElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubInsightFiltersElFindingProviderFieldsCriticalityElRef {
        SecurityhubInsightFiltersElFindingProviderFieldsCriticalityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElFindingProviderFieldsCriticalityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `eq` after provisioning.\n"]
    pub fn eq(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.eq", self.base))
    }
    #[doc = "Get a reference to the value of field `gte` after provisioning.\n"]
    pub fn gte(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.gte", self.base))
    }
    #[doc = "Get a reference to the value of field `lte` after provisioning.\n"]
    pub fn lte(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.lte", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElFindingProviderFieldsRelatedFindingsIdEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}
impl SecurityhubInsightFiltersElFindingProviderFieldsRelatedFindingsIdEl {}
impl ToListMappable for SecurityhubInsightFiltersElFindingProviderFieldsRelatedFindingsIdEl {
    type O = BlockAssignable<SecurityhubInsightFiltersElFindingProviderFieldsRelatedFindingsIdEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElFindingProviderFieldsRelatedFindingsIdEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildSecurityhubInsightFiltersElFindingProviderFieldsRelatedFindingsIdEl {
    pub fn build(self) -> SecurityhubInsightFiltersElFindingProviderFieldsRelatedFindingsIdEl {
        SecurityhubInsightFiltersElFindingProviderFieldsRelatedFindingsIdEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}
pub struct SecurityhubInsightFiltersElFindingProviderFieldsRelatedFindingsIdElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElFindingProviderFieldsRelatedFindingsIdElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubInsightFiltersElFindingProviderFieldsRelatedFindingsIdElRef {
        SecurityhubInsightFiltersElFindingProviderFieldsRelatedFindingsIdElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElFindingProviderFieldsRelatedFindingsIdElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElFindingProviderFieldsRelatedFindingsProductArnEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}
impl SecurityhubInsightFiltersElFindingProviderFieldsRelatedFindingsProductArnEl {}
impl ToListMappable
    for SecurityhubInsightFiltersElFindingProviderFieldsRelatedFindingsProductArnEl
{
    type O = BlockAssignable<
        SecurityhubInsightFiltersElFindingProviderFieldsRelatedFindingsProductArnEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElFindingProviderFieldsRelatedFindingsProductArnEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildSecurityhubInsightFiltersElFindingProviderFieldsRelatedFindingsProductArnEl {
    pub fn build(
        self,
    ) -> SecurityhubInsightFiltersElFindingProviderFieldsRelatedFindingsProductArnEl {
        SecurityhubInsightFiltersElFindingProviderFieldsRelatedFindingsProductArnEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}
pub struct SecurityhubInsightFiltersElFindingProviderFieldsRelatedFindingsProductArnElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElFindingProviderFieldsRelatedFindingsProductArnElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubInsightFiltersElFindingProviderFieldsRelatedFindingsProductArnElRef {
        SecurityhubInsightFiltersElFindingProviderFieldsRelatedFindingsProductArnElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElFindingProviderFieldsRelatedFindingsProductArnElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElFindingProviderFieldsSeverityLabelEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}
impl SecurityhubInsightFiltersElFindingProviderFieldsSeverityLabelEl {}
impl ToListMappable for SecurityhubInsightFiltersElFindingProviderFieldsSeverityLabelEl {
    type O = BlockAssignable<SecurityhubInsightFiltersElFindingProviderFieldsSeverityLabelEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElFindingProviderFieldsSeverityLabelEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildSecurityhubInsightFiltersElFindingProviderFieldsSeverityLabelEl {
    pub fn build(self) -> SecurityhubInsightFiltersElFindingProviderFieldsSeverityLabelEl {
        SecurityhubInsightFiltersElFindingProviderFieldsSeverityLabelEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}
pub struct SecurityhubInsightFiltersElFindingProviderFieldsSeverityLabelElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElFindingProviderFieldsSeverityLabelElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubInsightFiltersElFindingProviderFieldsSeverityLabelElRef {
        SecurityhubInsightFiltersElFindingProviderFieldsSeverityLabelElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElFindingProviderFieldsSeverityLabelElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElFindingProviderFieldsSeverityOriginalEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}
impl SecurityhubInsightFiltersElFindingProviderFieldsSeverityOriginalEl {}
impl ToListMappable for SecurityhubInsightFiltersElFindingProviderFieldsSeverityOriginalEl {
    type O = BlockAssignable<SecurityhubInsightFiltersElFindingProviderFieldsSeverityOriginalEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElFindingProviderFieldsSeverityOriginalEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildSecurityhubInsightFiltersElFindingProviderFieldsSeverityOriginalEl {
    pub fn build(self) -> SecurityhubInsightFiltersElFindingProviderFieldsSeverityOriginalEl {
        SecurityhubInsightFiltersElFindingProviderFieldsSeverityOriginalEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}
pub struct SecurityhubInsightFiltersElFindingProviderFieldsSeverityOriginalElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElFindingProviderFieldsSeverityOriginalElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubInsightFiltersElFindingProviderFieldsSeverityOriginalElRef {
        SecurityhubInsightFiltersElFindingProviderFieldsSeverityOriginalElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElFindingProviderFieldsSeverityOriginalElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElFindingProviderFieldsTypesEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}
impl SecurityhubInsightFiltersElFindingProviderFieldsTypesEl {}
impl ToListMappable for SecurityhubInsightFiltersElFindingProviderFieldsTypesEl {
    type O = BlockAssignable<SecurityhubInsightFiltersElFindingProviderFieldsTypesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElFindingProviderFieldsTypesEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildSecurityhubInsightFiltersElFindingProviderFieldsTypesEl {
    pub fn build(self) -> SecurityhubInsightFiltersElFindingProviderFieldsTypesEl {
        SecurityhubInsightFiltersElFindingProviderFieldsTypesEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}
pub struct SecurityhubInsightFiltersElFindingProviderFieldsTypesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElFindingProviderFieldsTypesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubInsightFiltersElFindingProviderFieldsTypesElRef {
        SecurityhubInsightFiltersElFindingProviderFieldsTypesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElFindingProviderFieldsTypesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElFirstObservedAtElDateRangeEl {
    unit: PrimField<String>,
    value: PrimField<f64>,
}
impl SecurityhubInsightFiltersElFirstObservedAtElDateRangeEl {}
impl ToListMappable for SecurityhubInsightFiltersElFirstObservedAtElDateRangeEl {
    type O = BlockAssignable<SecurityhubInsightFiltersElFirstObservedAtElDateRangeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElFirstObservedAtElDateRangeEl {
    #[doc = ""]
    pub unit: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<f64>,
}
impl BuildSecurityhubInsightFiltersElFirstObservedAtElDateRangeEl {
    pub fn build(self) -> SecurityhubInsightFiltersElFirstObservedAtElDateRangeEl {
        SecurityhubInsightFiltersElFirstObservedAtElDateRangeEl {
            unit: self.unit,
            value: self.value,
        }
    }
}
pub struct SecurityhubInsightFiltersElFirstObservedAtElDateRangeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElFirstObservedAtElDateRangeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubInsightFiltersElFirstObservedAtElDateRangeElRef {
        SecurityhubInsightFiltersElFirstObservedAtElDateRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElFirstObservedAtElDateRangeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize, Default)]
struct SecurityhubInsightFiltersElFirstObservedAtElDynamic {
    date_range: Option<DynamicBlock<SecurityhubInsightFiltersElFirstObservedAtElDateRangeEl>>,
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElFirstObservedAtEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    end: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    date_range: Option<Vec<SecurityhubInsightFiltersElFirstObservedAtElDateRangeEl>>,
    dynamic: SecurityhubInsightFiltersElFirstObservedAtElDynamic,
}
impl SecurityhubInsightFiltersElFirstObservedAtEl {
    #[doc = "Set the field `end`.\n"]
    pub fn set_end(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.end = Some(v.into());
        self
    }
    #[doc = "Set the field `start`.\n"]
    pub fn set_start(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.start = Some(v.into());
        self
    }
    #[doc = "Set the field `date_range`.\n"]
    pub fn set_date_range(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubInsightFiltersElFirstObservedAtElDateRangeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.date_range = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.date_range = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for SecurityhubInsightFiltersElFirstObservedAtEl {
    type O = BlockAssignable<SecurityhubInsightFiltersElFirstObservedAtEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElFirstObservedAtEl {}
impl BuildSecurityhubInsightFiltersElFirstObservedAtEl {
    pub fn build(self) -> SecurityhubInsightFiltersElFirstObservedAtEl {
        SecurityhubInsightFiltersElFirstObservedAtEl {
            end: core::default::Default::default(),
            start: core::default::Default::default(),
            date_range: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct SecurityhubInsightFiltersElFirstObservedAtElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElFirstObservedAtElRef {
    fn new(shared: StackShared, base: String) -> SecurityhubInsightFiltersElFirstObservedAtElRef {
        SecurityhubInsightFiltersElFirstObservedAtElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElFirstObservedAtElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `end` after provisioning.\n"]
    pub fn end(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.end", self.base))
    }
    #[doc = "Get a reference to the value of field `start` after provisioning.\n"]
    pub fn start(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start", self.base))
    }
    #[doc = "Get a reference to the value of field `date_range` after provisioning.\n"]
    pub fn date_range(
        &self,
    ) -> ListRef<SecurityhubInsightFiltersElFirstObservedAtElDateRangeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.date_range", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElGeneratorIdEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}
impl SecurityhubInsightFiltersElGeneratorIdEl {}
impl ToListMappable for SecurityhubInsightFiltersElGeneratorIdEl {
    type O = BlockAssignable<SecurityhubInsightFiltersElGeneratorIdEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElGeneratorIdEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildSecurityhubInsightFiltersElGeneratorIdEl {
    pub fn build(self) -> SecurityhubInsightFiltersElGeneratorIdEl {
        SecurityhubInsightFiltersElGeneratorIdEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}
pub struct SecurityhubInsightFiltersElGeneratorIdElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElGeneratorIdElRef {
    fn new(shared: StackShared, base: String) -> SecurityhubInsightFiltersElGeneratorIdElRef {
        SecurityhubInsightFiltersElGeneratorIdElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElGeneratorIdElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElIdEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}
impl SecurityhubInsightFiltersElIdEl {}
impl ToListMappable for SecurityhubInsightFiltersElIdEl {
    type O = BlockAssignable<SecurityhubInsightFiltersElIdEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElIdEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildSecurityhubInsightFiltersElIdEl {
    pub fn build(self) -> SecurityhubInsightFiltersElIdEl {
        SecurityhubInsightFiltersElIdEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}
pub struct SecurityhubInsightFiltersElIdElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElIdElRef {
    fn new(shared: StackShared, base: String) -> SecurityhubInsightFiltersElIdElRef {
        SecurityhubInsightFiltersElIdElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElIdElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElKeywordEl {
    value: PrimField<String>,
}
impl SecurityhubInsightFiltersElKeywordEl {}
impl ToListMappable for SecurityhubInsightFiltersElKeywordEl {
    type O = BlockAssignable<SecurityhubInsightFiltersElKeywordEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElKeywordEl {
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildSecurityhubInsightFiltersElKeywordEl {
    pub fn build(self) -> SecurityhubInsightFiltersElKeywordEl {
        SecurityhubInsightFiltersElKeywordEl { value: self.value }
    }
}
pub struct SecurityhubInsightFiltersElKeywordElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElKeywordElRef {
    fn new(shared: StackShared, base: String) -> SecurityhubInsightFiltersElKeywordElRef {
        SecurityhubInsightFiltersElKeywordElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElKeywordElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElLastObservedAtElDateRangeEl {
    unit: PrimField<String>,
    value: PrimField<f64>,
}
impl SecurityhubInsightFiltersElLastObservedAtElDateRangeEl {}
impl ToListMappable for SecurityhubInsightFiltersElLastObservedAtElDateRangeEl {
    type O = BlockAssignable<SecurityhubInsightFiltersElLastObservedAtElDateRangeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElLastObservedAtElDateRangeEl {
    #[doc = ""]
    pub unit: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<f64>,
}
impl BuildSecurityhubInsightFiltersElLastObservedAtElDateRangeEl {
    pub fn build(self) -> SecurityhubInsightFiltersElLastObservedAtElDateRangeEl {
        SecurityhubInsightFiltersElLastObservedAtElDateRangeEl {
            unit: self.unit,
            value: self.value,
        }
    }
}
pub struct SecurityhubInsightFiltersElLastObservedAtElDateRangeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElLastObservedAtElDateRangeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubInsightFiltersElLastObservedAtElDateRangeElRef {
        SecurityhubInsightFiltersElLastObservedAtElDateRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElLastObservedAtElDateRangeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize, Default)]
struct SecurityhubInsightFiltersElLastObservedAtElDynamic {
    date_range: Option<DynamicBlock<SecurityhubInsightFiltersElLastObservedAtElDateRangeEl>>,
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElLastObservedAtEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    end: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    date_range: Option<Vec<SecurityhubInsightFiltersElLastObservedAtElDateRangeEl>>,
    dynamic: SecurityhubInsightFiltersElLastObservedAtElDynamic,
}
impl SecurityhubInsightFiltersElLastObservedAtEl {
    #[doc = "Set the field `end`.\n"]
    pub fn set_end(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.end = Some(v.into());
        self
    }
    #[doc = "Set the field `start`.\n"]
    pub fn set_start(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.start = Some(v.into());
        self
    }
    #[doc = "Set the field `date_range`.\n"]
    pub fn set_date_range(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubInsightFiltersElLastObservedAtElDateRangeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.date_range = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.date_range = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for SecurityhubInsightFiltersElLastObservedAtEl {
    type O = BlockAssignable<SecurityhubInsightFiltersElLastObservedAtEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElLastObservedAtEl {}
impl BuildSecurityhubInsightFiltersElLastObservedAtEl {
    pub fn build(self) -> SecurityhubInsightFiltersElLastObservedAtEl {
        SecurityhubInsightFiltersElLastObservedAtEl {
            end: core::default::Default::default(),
            start: core::default::Default::default(),
            date_range: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct SecurityhubInsightFiltersElLastObservedAtElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElLastObservedAtElRef {
    fn new(shared: StackShared, base: String) -> SecurityhubInsightFiltersElLastObservedAtElRef {
        SecurityhubInsightFiltersElLastObservedAtElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElLastObservedAtElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `end` after provisioning.\n"]
    pub fn end(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.end", self.base))
    }
    #[doc = "Get a reference to the value of field `start` after provisioning.\n"]
    pub fn start(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start", self.base))
    }
    #[doc = "Get a reference to the value of field `date_range` after provisioning.\n"]
    pub fn date_range(&self) -> ListRef<SecurityhubInsightFiltersElLastObservedAtElDateRangeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.date_range", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElMalwareNameEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}
impl SecurityhubInsightFiltersElMalwareNameEl {}
impl ToListMappable for SecurityhubInsightFiltersElMalwareNameEl {
    type O = BlockAssignable<SecurityhubInsightFiltersElMalwareNameEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElMalwareNameEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildSecurityhubInsightFiltersElMalwareNameEl {
    pub fn build(self) -> SecurityhubInsightFiltersElMalwareNameEl {
        SecurityhubInsightFiltersElMalwareNameEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}
pub struct SecurityhubInsightFiltersElMalwareNameElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElMalwareNameElRef {
    fn new(shared: StackShared, base: String) -> SecurityhubInsightFiltersElMalwareNameElRef {
        SecurityhubInsightFiltersElMalwareNameElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElMalwareNameElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElMalwarePathEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}
impl SecurityhubInsightFiltersElMalwarePathEl {}
impl ToListMappable for SecurityhubInsightFiltersElMalwarePathEl {
    type O = BlockAssignable<SecurityhubInsightFiltersElMalwarePathEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElMalwarePathEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildSecurityhubInsightFiltersElMalwarePathEl {
    pub fn build(self) -> SecurityhubInsightFiltersElMalwarePathEl {
        SecurityhubInsightFiltersElMalwarePathEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}
pub struct SecurityhubInsightFiltersElMalwarePathElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElMalwarePathElRef {
    fn new(shared: StackShared, base: String) -> SecurityhubInsightFiltersElMalwarePathElRef {
        SecurityhubInsightFiltersElMalwarePathElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElMalwarePathElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElMalwareStateEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}
impl SecurityhubInsightFiltersElMalwareStateEl {}
impl ToListMappable for SecurityhubInsightFiltersElMalwareStateEl {
    type O = BlockAssignable<SecurityhubInsightFiltersElMalwareStateEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElMalwareStateEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildSecurityhubInsightFiltersElMalwareStateEl {
    pub fn build(self) -> SecurityhubInsightFiltersElMalwareStateEl {
        SecurityhubInsightFiltersElMalwareStateEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}
pub struct SecurityhubInsightFiltersElMalwareStateElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElMalwareStateElRef {
    fn new(shared: StackShared, base: String) -> SecurityhubInsightFiltersElMalwareStateElRef {
        SecurityhubInsightFiltersElMalwareStateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElMalwareStateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElMalwareTypeEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}
impl SecurityhubInsightFiltersElMalwareTypeEl {}
impl ToListMappable for SecurityhubInsightFiltersElMalwareTypeEl {
    type O = BlockAssignable<SecurityhubInsightFiltersElMalwareTypeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElMalwareTypeEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildSecurityhubInsightFiltersElMalwareTypeEl {
    pub fn build(self) -> SecurityhubInsightFiltersElMalwareTypeEl {
        SecurityhubInsightFiltersElMalwareTypeEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}
pub struct SecurityhubInsightFiltersElMalwareTypeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElMalwareTypeElRef {
    fn new(shared: StackShared, base: String) -> SecurityhubInsightFiltersElMalwareTypeElRef {
        SecurityhubInsightFiltersElMalwareTypeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElMalwareTypeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElNetworkDestinationDomainEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}
impl SecurityhubInsightFiltersElNetworkDestinationDomainEl {}
impl ToListMappable for SecurityhubInsightFiltersElNetworkDestinationDomainEl {
    type O = BlockAssignable<SecurityhubInsightFiltersElNetworkDestinationDomainEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElNetworkDestinationDomainEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildSecurityhubInsightFiltersElNetworkDestinationDomainEl {
    pub fn build(self) -> SecurityhubInsightFiltersElNetworkDestinationDomainEl {
        SecurityhubInsightFiltersElNetworkDestinationDomainEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}
pub struct SecurityhubInsightFiltersElNetworkDestinationDomainElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElNetworkDestinationDomainElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubInsightFiltersElNetworkDestinationDomainElRef {
        SecurityhubInsightFiltersElNetworkDestinationDomainElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElNetworkDestinationDomainElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElNetworkDestinationIpv4El {
    cidr: PrimField<String>,
}
impl SecurityhubInsightFiltersElNetworkDestinationIpv4El {}
impl ToListMappable for SecurityhubInsightFiltersElNetworkDestinationIpv4El {
    type O = BlockAssignable<SecurityhubInsightFiltersElNetworkDestinationIpv4El>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElNetworkDestinationIpv4El {
    #[doc = ""]
    pub cidr: PrimField<String>,
}
impl BuildSecurityhubInsightFiltersElNetworkDestinationIpv4El {
    pub fn build(self) -> SecurityhubInsightFiltersElNetworkDestinationIpv4El {
        SecurityhubInsightFiltersElNetworkDestinationIpv4El { cidr: self.cidr }
    }
}
pub struct SecurityhubInsightFiltersElNetworkDestinationIpv4ElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElNetworkDestinationIpv4ElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubInsightFiltersElNetworkDestinationIpv4ElRef {
        SecurityhubInsightFiltersElNetworkDestinationIpv4ElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElNetworkDestinationIpv4ElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cidr` after provisioning.\n"]
    pub fn cidr(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cidr", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElNetworkDestinationIpv6El {
    cidr: PrimField<String>,
}
impl SecurityhubInsightFiltersElNetworkDestinationIpv6El {}
impl ToListMappable for SecurityhubInsightFiltersElNetworkDestinationIpv6El {
    type O = BlockAssignable<SecurityhubInsightFiltersElNetworkDestinationIpv6El>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElNetworkDestinationIpv6El {
    #[doc = ""]
    pub cidr: PrimField<String>,
}
impl BuildSecurityhubInsightFiltersElNetworkDestinationIpv6El {
    pub fn build(self) -> SecurityhubInsightFiltersElNetworkDestinationIpv6El {
        SecurityhubInsightFiltersElNetworkDestinationIpv6El { cidr: self.cidr }
    }
}
pub struct SecurityhubInsightFiltersElNetworkDestinationIpv6ElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElNetworkDestinationIpv6ElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubInsightFiltersElNetworkDestinationIpv6ElRef {
        SecurityhubInsightFiltersElNetworkDestinationIpv6ElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElNetworkDestinationIpv6ElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cidr` after provisioning.\n"]
    pub fn cidr(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cidr", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElNetworkDestinationPortEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    eq: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gte: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lte: Option<PrimField<String>>,
}
impl SecurityhubInsightFiltersElNetworkDestinationPortEl {
    #[doc = "Set the field `eq`.\n"]
    pub fn set_eq(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.eq = Some(v.into());
        self
    }
    #[doc = "Set the field `gte`.\n"]
    pub fn set_gte(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gte = Some(v.into());
        self
    }
    #[doc = "Set the field `lte`.\n"]
    pub fn set_lte(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.lte = Some(v.into());
        self
    }
}
impl ToListMappable for SecurityhubInsightFiltersElNetworkDestinationPortEl {
    type O = BlockAssignable<SecurityhubInsightFiltersElNetworkDestinationPortEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElNetworkDestinationPortEl {}
impl BuildSecurityhubInsightFiltersElNetworkDestinationPortEl {
    pub fn build(self) -> SecurityhubInsightFiltersElNetworkDestinationPortEl {
        SecurityhubInsightFiltersElNetworkDestinationPortEl {
            eq: core::default::Default::default(),
            gte: core::default::Default::default(),
            lte: core::default::Default::default(),
        }
    }
}
pub struct SecurityhubInsightFiltersElNetworkDestinationPortElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElNetworkDestinationPortElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubInsightFiltersElNetworkDestinationPortElRef {
        SecurityhubInsightFiltersElNetworkDestinationPortElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElNetworkDestinationPortElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `eq` after provisioning.\n"]
    pub fn eq(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.eq", self.base))
    }
    #[doc = "Get a reference to the value of field `gte` after provisioning.\n"]
    pub fn gte(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.gte", self.base))
    }
    #[doc = "Get a reference to the value of field `lte` after provisioning.\n"]
    pub fn lte(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.lte", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElNetworkDirectionEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}
impl SecurityhubInsightFiltersElNetworkDirectionEl {}
impl ToListMappable for SecurityhubInsightFiltersElNetworkDirectionEl {
    type O = BlockAssignable<SecurityhubInsightFiltersElNetworkDirectionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElNetworkDirectionEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildSecurityhubInsightFiltersElNetworkDirectionEl {
    pub fn build(self) -> SecurityhubInsightFiltersElNetworkDirectionEl {
        SecurityhubInsightFiltersElNetworkDirectionEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}
pub struct SecurityhubInsightFiltersElNetworkDirectionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElNetworkDirectionElRef {
    fn new(shared: StackShared, base: String) -> SecurityhubInsightFiltersElNetworkDirectionElRef {
        SecurityhubInsightFiltersElNetworkDirectionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElNetworkDirectionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElNetworkProtocolEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}
impl SecurityhubInsightFiltersElNetworkProtocolEl {}
impl ToListMappable for SecurityhubInsightFiltersElNetworkProtocolEl {
    type O = BlockAssignable<SecurityhubInsightFiltersElNetworkProtocolEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElNetworkProtocolEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildSecurityhubInsightFiltersElNetworkProtocolEl {
    pub fn build(self) -> SecurityhubInsightFiltersElNetworkProtocolEl {
        SecurityhubInsightFiltersElNetworkProtocolEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}
pub struct SecurityhubInsightFiltersElNetworkProtocolElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElNetworkProtocolElRef {
    fn new(shared: StackShared, base: String) -> SecurityhubInsightFiltersElNetworkProtocolElRef {
        SecurityhubInsightFiltersElNetworkProtocolElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElNetworkProtocolElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElNetworkSourceDomainEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}
impl SecurityhubInsightFiltersElNetworkSourceDomainEl {}
impl ToListMappable for SecurityhubInsightFiltersElNetworkSourceDomainEl {
    type O = BlockAssignable<SecurityhubInsightFiltersElNetworkSourceDomainEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElNetworkSourceDomainEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildSecurityhubInsightFiltersElNetworkSourceDomainEl {
    pub fn build(self) -> SecurityhubInsightFiltersElNetworkSourceDomainEl {
        SecurityhubInsightFiltersElNetworkSourceDomainEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}
pub struct SecurityhubInsightFiltersElNetworkSourceDomainElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElNetworkSourceDomainElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubInsightFiltersElNetworkSourceDomainElRef {
        SecurityhubInsightFiltersElNetworkSourceDomainElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElNetworkSourceDomainElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElNetworkSourceIpv4El {
    cidr: PrimField<String>,
}
impl SecurityhubInsightFiltersElNetworkSourceIpv4El {}
impl ToListMappable for SecurityhubInsightFiltersElNetworkSourceIpv4El {
    type O = BlockAssignable<SecurityhubInsightFiltersElNetworkSourceIpv4El>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElNetworkSourceIpv4El {
    #[doc = ""]
    pub cidr: PrimField<String>,
}
impl BuildSecurityhubInsightFiltersElNetworkSourceIpv4El {
    pub fn build(self) -> SecurityhubInsightFiltersElNetworkSourceIpv4El {
        SecurityhubInsightFiltersElNetworkSourceIpv4El { cidr: self.cidr }
    }
}
pub struct SecurityhubInsightFiltersElNetworkSourceIpv4ElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElNetworkSourceIpv4ElRef {
    fn new(shared: StackShared, base: String) -> SecurityhubInsightFiltersElNetworkSourceIpv4ElRef {
        SecurityhubInsightFiltersElNetworkSourceIpv4ElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElNetworkSourceIpv4ElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cidr` after provisioning.\n"]
    pub fn cidr(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cidr", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElNetworkSourceIpv6El {
    cidr: PrimField<String>,
}
impl SecurityhubInsightFiltersElNetworkSourceIpv6El {}
impl ToListMappable for SecurityhubInsightFiltersElNetworkSourceIpv6El {
    type O = BlockAssignable<SecurityhubInsightFiltersElNetworkSourceIpv6El>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElNetworkSourceIpv6El {
    #[doc = ""]
    pub cidr: PrimField<String>,
}
impl BuildSecurityhubInsightFiltersElNetworkSourceIpv6El {
    pub fn build(self) -> SecurityhubInsightFiltersElNetworkSourceIpv6El {
        SecurityhubInsightFiltersElNetworkSourceIpv6El { cidr: self.cidr }
    }
}
pub struct SecurityhubInsightFiltersElNetworkSourceIpv6ElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElNetworkSourceIpv6ElRef {
    fn new(shared: StackShared, base: String) -> SecurityhubInsightFiltersElNetworkSourceIpv6ElRef {
        SecurityhubInsightFiltersElNetworkSourceIpv6ElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElNetworkSourceIpv6ElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cidr` after provisioning.\n"]
    pub fn cidr(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cidr", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElNetworkSourceMacEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}
impl SecurityhubInsightFiltersElNetworkSourceMacEl {}
impl ToListMappable for SecurityhubInsightFiltersElNetworkSourceMacEl {
    type O = BlockAssignable<SecurityhubInsightFiltersElNetworkSourceMacEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElNetworkSourceMacEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildSecurityhubInsightFiltersElNetworkSourceMacEl {
    pub fn build(self) -> SecurityhubInsightFiltersElNetworkSourceMacEl {
        SecurityhubInsightFiltersElNetworkSourceMacEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}
pub struct SecurityhubInsightFiltersElNetworkSourceMacElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElNetworkSourceMacElRef {
    fn new(shared: StackShared, base: String) -> SecurityhubInsightFiltersElNetworkSourceMacElRef {
        SecurityhubInsightFiltersElNetworkSourceMacElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElNetworkSourceMacElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElNetworkSourcePortEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    eq: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gte: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lte: Option<PrimField<String>>,
}
impl SecurityhubInsightFiltersElNetworkSourcePortEl {
    #[doc = "Set the field `eq`.\n"]
    pub fn set_eq(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.eq = Some(v.into());
        self
    }
    #[doc = "Set the field `gte`.\n"]
    pub fn set_gte(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gte = Some(v.into());
        self
    }
    #[doc = "Set the field `lte`.\n"]
    pub fn set_lte(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.lte = Some(v.into());
        self
    }
}
impl ToListMappable for SecurityhubInsightFiltersElNetworkSourcePortEl {
    type O = BlockAssignable<SecurityhubInsightFiltersElNetworkSourcePortEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElNetworkSourcePortEl {}
impl BuildSecurityhubInsightFiltersElNetworkSourcePortEl {
    pub fn build(self) -> SecurityhubInsightFiltersElNetworkSourcePortEl {
        SecurityhubInsightFiltersElNetworkSourcePortEl {
            eq: core::default::Default::default(),
            gte: core::default::Default::default(),
            lte: core::default::Default::default(),
        }
    }
}
pub struct SecurityhubInsightFiltersElNetworkSourcePortElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElNetworkSourcePortElRef {
    fn new(shared: StackShared, base: String) -> SecurityhubInsightFiltersElNetworkSourcePortElRef {
        SecurityhubInsightFiltersElNetworkSourcePortElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElNetworkSourcePortElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `eq` after provisioning.\n"]
    pub fn eq(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.eq", self.base))
    }
    #[doc = "Get a reference to the value of field `gte` after provisioning.\n"]
    pub fn gte(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.gte", self.base))
    }
    #[doc = "Get a reference to the value of field `lte` after provisioning.\n"]
    pub fn lte(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.lte", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElNoteTextEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}
impl SecurityhubInsightFiltersElNoteTextEl {}
impl ToListMappable for SecurityhubInsightFiltersElNoteTextEl {
    type O = BlockAssignable<SecurityhubInsightFiltersElNoteTextEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElNoteTextEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildSecurityhubInsightFiltersElNoteTextEl {
    pub fn build(self) -> SecurityhubInsightFiltersElNoteTextEl {
        SecurityhubInsightFiltersElNoteTextEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}
pub struct SecurityhubInsightFiltersElNoteTextElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElNoteTextElRef {
    fn new(shared: StackShared, base: String) -> SecurityhubInsightFiltersElNoteTextElRef {
        SecurityhubInsightFiltersElNoteTextElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElNoteTextElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElNoteUpdatedAtElDateRangeEl {
    unit: PrimField<String>,
    value: PrimField<f64>,
}
impl SecurityhubInsightFiltersElNoteUpdatedAtElDateRangeEl {}
impl ToListMappable for SecurityhubInsightFiltersElNoteUpdatedAtElDateRangeEl {
    type O = BlockAssignable<SecurityhubInsightFiltersElNoteUpdatedAtElDateRangeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElNoteUpdatedAtElDateRangeEl {
    #[doc = ""]
    pub unit: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<f64>,
}
impl BuildSecurityhubInsightFiltersElNoteUpdatedAtElDateRangeEl {
    pub fn build(self) -> SecurityhubInsightFiltersElNoteUpdatedAtElDateRangeEl {
        SecurityhubInsightFiltersElNoteUpdatedAtElDateRangeEl {
            unit: self.unit,
            value: self.value,
        }
    }
}
pub struct SecurityhubInsightFiltersElNoteUpdatedAtElDateRangeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElNoteUpdatedAtElDateRangeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubInsightFiltersElNoteUpdatedAtElDateRangeElRef {
        SecurityhubInsightFiltersElNoteUpdatedAtElDateRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElNoteUpdatedAtElDateRangeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize, Default)]
struct SecurityhubInsightFiltersElNoteUpdatedAtElDynamic {
    date_range: Option<DynamicBlock<SecurityhubInsightFiltersElNoteUpdatedAtElDateRangeEl>>,
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElNoteUpdatedAtEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    end: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    date_range: Option<Vec<SecurityhubInsightFiltersElNoteUpdatedAtElDateRangeEl>>,
    dynamic: SecurityhubInsightFiltersElNoteUpdatedAtElDynamic,
}
impl SecurityhubInsightFiltersElNoteUpdatedAtEl {
    #[doc = "Set the field `end`.\n"]
    pub fn set_end(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.end = Some(v.into());
        self
    }
    #[doc = "Set the field `start`.\n"]
    pub fn set_start(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.start = Some(v.into());
        self
    }
    #[doc = "Set the field `date_range`.\n"]
    pub fn set_date_range(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubInsightFiltersElNoteUpdatedAtElDateRangeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.date_range = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.date_range = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for SecurityhubInsightFiltersElNoteUpdatedAtEl {
    type O = BlockAssignable<SecurityhubInsightFiltersElNoteUpdatedAtEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElNoteUpdatedAtEl {}
impl BuildSecurityhubInsightFiltersElNoteUpdatedAtEl {
    pub fn build(self) -> SecurityhubInsightFiltersElNoteUpdatedAtEl {
        SecurityhubInsightFiltersElNoteUpdatedAtEl {
            end: core::default::Default::default(),
            start: core::default::Default::default(),
            date_range: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct SecurityhubInsightFiltersElNoteUpdatedAtElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElNoteUpdatedAtElRef {
    fn new(shared: StackShared, base: String) -> SecurityhubInsightFiltersElNoteUpdatedAtElRef {
        SecurityhubInsightFiltersElNoteUpdatedAtElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElNoteUpdatedAtElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `end` after provisioning.\n"]
    pub fn end(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.end", self.base))
    }
    #[doc = "Get a reference to the value of field `start` after provisioning.\n"]
    pub fn start(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start", self.base))
    }
    #[doc = "Get a reference to the value of field `date_range` after provisioning.\n"]
    pub fn date_range(&self) -> ListRef<SecurityhubInsightFiltersElNoteUpdatedAtElDateRangeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.date_range", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElNoteUpdatedByEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}
impl SecurityhubInsightFiltersElNoteUpdatedByEl {}
impl ToListMappable for SecurityhubInsightFiltersElNoteUpdatedByEl {
    type O = BlockAssignable<SecurityhubInsightFiltersElNoteUpdatedByEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElNoteUpdatedByEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildSecurityhubInsightFiltersElNoteUpdatedByEl {
    pub fn build(self) -> SecurityhubInsightFiltersElNoteUpdatedByEl {
        SecurityhubInsightFiltersElNoteUpdatedByEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}
pub struct SecurityhubInsightFiltersElNoteUpdatedByElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElNoteUpdatedByElRef {
    fn new(shared: StackShared, base: String) -> SecurityhubInsightFiltersElNoteUpdatedByElRef {
        SecurityhubInsightFiltersElNoteUpdatedByElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElNoteUpdatedByElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElProcessLaunchedAtElDateRangeEl {
    unit: PrimField<String>,
    value: PrimField<f64>,
}
impl SecurityhubInsightFiltersElProcessLaunchedAtElDateRangeEl {}
impl ToListMappable for SecurityhubInsightFiltersElProcessLaunchedAtElDateRangeEl {
    type O = BlockAssignable<SecurityhubInsightFiltersElProcessLaunchedAtElDateRangeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElProcessLaunchedAtElDateRangeEl {
    #[doc = ""]
    pub unit: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<f64>,
}
impl BuildSecurityhubInsightFiltersElProcessLaunchedAtElDateRangeEl {
    pub fn build(self) -> SecurityhubInsightFiltersElProcessLaunchedAtElDateRangeEl {
        SecurityhubInsightFiltersElProcessLaunchedAtElDateRangeEl {
            unit: self.unit,
            value: self.value,
        }
    }
}
pub struct SecurityhubInsightFiltersElProcessLaunchedAtElDateRangeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElProcessLaunchedAtElDateRangeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubInsightFiltersElProcessLaunchedAtElDateRangeElRef {
        SecurityhubInsightFiltersElProcessLaunchedAtElDateRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElProcessLaunchedAtElDateRangeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize, Default)]
struct SecurityhubInsightFiltersElProcessLaunchedAtElDynamic {
    date_range: Option<DynamicBlock<SecurityhubInsightFiltersElProcessLaunchedAtElDateRangeEl>>,
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElProcessLaunchedAtEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    end: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    date_range: Option<Vec<SecurityhubInsightFiltersElProcessLaunchedAtElDateRangeEl>>,
    dynamic: SecurityhubInsightFiltersElProcessLaunchedAtElDynamic,
}
impl SecurityhubInsightFiltersElProcessLaunchedAtEl {
    #[doc = "Set the field `end`.\n"]
    pub fn set_end(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.end = Some(v.into());
        self
    }
    #[doc = "Set the field `start`.\n"]
    pub fn set_start(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.start = Some(v.into());
        self
    }
    #[doc = "Set the field `date_range`.\n"]
    pub fn set_date_range(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubInsightFiltersElProcessLaunchedAtElDateRangeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.date_range = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.date_range = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for SecurityhubInsightFiltersElProcessLaunchedAtEl {
    type O = BlockAssignable<SecurityhubInsightFiltersElProcessLaunchedAtEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElProcessLaunchedAtEl {}
impl BuildSecurityhubInsightFiltersElProcessLaunchedAtEl {
    pub fn build(self) -> SecurityhubInsightFiltersElProcessLaunchedAtEl {
        SecurityhubInsightFiltersElProcessLaunchedAtEl {
            end: core::default::Default::default(),
            start: core::default::Default::default(),
            date_range: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct SecurityhubInsightFiltersElProcessLaunchedAtElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElProcessLaunchedAtElRef {
    fn new(shared: StackShared, base: String) -> SecurityhubInsightFiltersElProcessLaunchedAtElRef {
        SecurityhubInsightFiltersElProcessLaunchedAtElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElProcessLaunchedAtElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `end` after provisioning.\n"]
    pub fn end(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.end", self.base))
    }
    #[doc = "Get a reference to the value of field `start` after provisioning.\n"]
    pub fn start(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start", self.base))
    }
    #[doc = "Get a reference to the value of field `date_range` after provisioning.\n"]
    pub fn date_range(
        &self,
    ) -> ListRef<SecurityhubInsightFiltersElProcessLaunchedAtElDateRangeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.date_range", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElProcessNameEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}
impl SecurityhubInsightFiltersElProcessNameEl {}
impl ToListMappable for SecurityhubInsightFiltersElProcessNameEl {
    type O = BlockAssignable<SecurityhubInsightFiltersElProcessNameEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElProcessNameEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildSecurityhubInsightFiltersElProcessNameEl {
    pub fn build(self) -> SecurityhubInsightFiltersElProcessNameEl {
        SecurityhubInsightFiltersElProcessNameEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}
pub struct SecurityhubInsightFiltersElProcessNameElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElProcessNameElRef {
    fn new(shared: StackShared, base: String) -> SecurityhubInsightFiltersElProcessNameElRef {
        SecurityhubInsightFiltersElProcessNameElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElProcessNameElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElProcessParentPidEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    eq: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gte: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lte: Option<PrimField<String>>,
}
impl SecurityhubInsightFiltersElProcessParentPidEl {
    #[doc = "Set the field `eq`.\n"]
    pub fn set_eq(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.eq = Some(v.into());
        self
    }
    #[doc = "Set the field `gte`.\n"]
    pub fn set_gte(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gte = Some(v.into());
        self
    }
    #[doc = "Set the field `lte`.\n"]
    pub fn set_lte(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.lte = Some(v.into());
        self
    }
}
impl ToListMappable for SecurityhubInsightFiltersElProcessParentPidEl {
    type O = BlockAssignable<SecurityhubInsightFiltersElProcessParentPidEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElProcessParentPidEl {}
impl BuildSecurityhubInsightFiltersElProcessParentPidEl {
    pub fn build(self) -> SecurityhubInsightFiltersElProcessParentPidEl {
        SecurityhubInsightFiltersElProcessParentPidEl {
            eq: core::default::Default::default(),
            gte: core::default::Default::default(),
            lte: core::default::Default::default(),
        }
    }
}
pub struct SecurityhubInsightFiltersElProcessParentPidElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElProcessParentPidElRef {
    fn new(shared: StackShared, base: String) -> SecurityhubInsightFiltersElProcessParentPidElRef {
        SecurityhubInsightFiltersElProcessParentPidElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElProcessParentPidElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `eq` after provisioning.\n"]
    pub fn eq(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.eq", self.base))
    }
    #[doc = "Get a reference to the value of field `gte` after provisioning.\n"]
    pub fn gte(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.gte", self.base))
    }
    #[doc = "Get a reference to the value of field `lte` after provisioning.\n"]
    pub fn lte(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.lte", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElProcessPathEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}
impl SecurityhubInsightFiltersElProcessPathEl {}
impl ToListMappable for SecurityhubInsightFiltersElProcessPathEl {
    type O = BlockAssignable<SecurityhubInsightFiltersElProcessPathEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElProcessPathEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildSecurityhubInsightFiltersElProcessPathEl {
    pub fn build(self) -> SecurityhubInsightFiltersElProcessPathEl {
        SecurityhubInsightFiltersElProcessPathEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}
pub struct SecurityhubInsightFiltersElProcessPathElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElProcessPathElRef {
    fn new(shared: StackShared, base: String) -> SecurityhubInsightFiltersElProcessPathElRef {
        SecurityhubInsightFiltersElProcessPathElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElProcessPathElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElProcessPidEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    eq: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gte: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lte: Option<PrimField<String>>,
}
impl SecurityhubInsightFiltersElProcessPidEl {
    #[doc = "Set the field `eq`.\n"]
    pub fn set_eq(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.eq = Some(v.into());
        self
    }
    #[doc = "Set the field `gte`.\n"]
    pub fn set_gte(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gte = Some(v.into());
        self
    }
    #[doc = "Set the field `lte`.\n"]
    pub fn set_lte(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.lte = Some(v.into());
        self
    }
}
impl ToListMappable for SecurityhubInsightFiltersElProcessPidEl {
    type O = BlockAssignable<SecurityhubInsightFiltersElProcessPidEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElProcessPidEl {}
impl BuildSecurityhubInsightFiltersElProcessPidEl {
    pub fn build(self) -> SecurityhubInsightFiltersElProcessPidEl {
        SecurityhubInsightFiltersElProcessPidEl {
            eq: core::default::Default::default(),
            gte: core::default::Default::default(),
            lte: core::default::Default::default(),
        }
    }
}
pub struct SecurityhubInsightFiltersElProcessPidElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElProcessPidElRef {
    fn new(shared: StackShared, base: String) -> SecurityhubInsightFiltersElProcessPidElRef {
        SecurityhubInsightFiltersElProcessPidElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElProcessPidElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `eq` after provisioning.\n"]
    pub fn eq(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.eq", self.base))
    }
    #[doc = "Get a reference to the value of field `gte` after provisioning.\n"]
    pub fn gte(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.gte", self.base))
    }
    #[doc = "Get a reference to the value of field `lte` after provisioning.\n"]
    pub fn lte(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.lte", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElProcessTerminatedAtElDateRangeEl {
    unit: PrimField<String>,
    value: PrimField<f64>,
}
impl SecurityhubInsightFiltersElProcessTerminatedAtElDateRangeEl {}
impl ToListMappable for SecurityhubInsightFiltersElProcessTerminatedAtElDateRangeEl {
    type O = BlockAssignable<SecurityhubInsightFiltersElProcessTerminatedAtElDateRangeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElProcessTerminatedAtElDateRangeEl {
    #[doc = ""]
    pub unit: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<f64>,
}
impl BuildSecurityhubInsightFiltersElProcessTerminatedAtElDateRangeEl {
    pub fn build(self) -> SecurityhubInsightFiltersElProcessTerminatedAtElDateRangeEl {
        SecurityhubInsightFiltersElProcessTerminatedAtElDateRangeEl {
            unit: self.unit,
            value: self.value,
        }
    }
}
pub struct SecurityhubInsightFiltersElProcessTerminatedAtElDateRangeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElProcessTerminatedAtElDateRangeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubInsightFiltersElProcessTerminatedAtElDateRangeElRef {
        SecurityhubInsightFiltersElProcessTerminatedAtElDateRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElProcessTerminatedAtElDateRangeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize, Default)]
struct SecurityhubInsightFiltersElProcessTerminatedAtElDynamic {
    date_range: Option<DynamicBlock<SecurityhubInsightFiltersElProcessTerminatedAtElDateRangeEl>>,
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElProcessTerminatedAtEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    end: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    date_range: Option<Vec<SecurityhubInsightFiltersElProcessTerminatedAtElDateRangeEl>>,
    dynamic: SecurityhubInsightFiltersElProcessTerminatedAtElDynamic,
}
impl SecurityhubInsightFiltersElProcessTerminatedAtEl {
    #[doc = "Set the field `end`.\n"]
    pub fn set_end(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.end = Some(v.into());
        self
    }
    #[doc = "Set the field `start`.\n"]
    pub fn set_start(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.start = Some(v.into());
        self
    }
    #[doc = "Set the field `date_range`.\n"]
    pub fn set_date_range(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubInsightFiltersElProcessTerminatedAtElDateRangeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.date_range = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.date_range = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for SecurityhubInsightFiltersElProcessTerminatedAtEl {
    type O = BlockAssignable<SecurityhubInsightFiltersElProcessTerminatedAtEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElProcessTerminatedAtEl {}
impl BuildSecurityhubInsightFiltersElProcessTerminatedAtEl {
    pub fn build(self) -> SecurityhubInsightFiltersElProcessTerminatedAtEl {
        SecurityhubInsightFiltersElProcessTerminatedAtEl {
            end: core::default::Default::default(),
            start: core::default::Default::default(),
            date_range: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct SecurityhubInsightFiltersElProcessTerminatedAtElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElProcessTerminatedAtElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubInsightFiltersElProcessTerminatedAtElRef {
        SecurityhubInsightFiltersElProcessTerminatedAtElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElProcessTerminatedAtElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `end` after provisioning.\n"]
    pub fn end(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.end", self.base))
    }
    #[doc = "Get a reference to the value of field `start` after provisioning.\n"]
    pub fn start(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start", self.base))
    }
    #[doc = "Get a reference to the value of field `date_range` after provisioning.\n"]
    pub fn date_range(
        &self,
    ) -> ListRef<SecurityhubInsightFiltersElProcessTerminatedAtElDateRangeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.date_range", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElProductArnEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}
impl SecurityhubInsightFiltersElProductArnEl {}
impl ToListMappable for SecurityhubInsightFiltersElProductArnEl {
    type O = BlockAssignable<SecurityhubInsightFiltersElProductArnEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElProductArnEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildSecurityhubInsightFiltersElProductArnEl {
    pub fn build(self) -> SecurityhubInsightFiltersElProductArnEl {
        SecurityhubInsightFiltersElProductArnEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}
pub struct SecurityhubInsightFiltersElProductArnElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElProductArnElRef {
    fn new(shared: StackShared, base: String) -> SecurityhubInsightFiltersElProductArnElRef {
        SecurityhubInsightFiltersElProductArnElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElProductArnElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElProductFieldsEl {
    comparison: PrimField<String>,
    key: PrimField<String>,
    value: PrimField<String>,
}
impl SecurityhubInsightFiltersElProductFieldsEl {}
impl ToListMappable for SecurityhubInsightFiltersElProductFieldsEl {
    type O = BlockAssignable<SecurityhubInsightFiltersElProductFieldsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElProductFieldsEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub key: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildSecurityhubInsightFiltersElProductFieldsEl {
    pub fn build(self) -> SecurityhubInsightFiltersElProductFieldsEl {
        SecurityhubInsightFiltersElProductFieldsEl {
            comparison: self.comparison,
            key: self.key,
            value: self.value,
        }
    }
}
pub struct SecurityhubInsightFiltersElProductFieldsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElProductFieldsElRef {
    fn new(shared: StackShared, base: String) -> SecurityhubInsightFiltersElProductFieldsElRef {
        SecurityhubInsightFiltersElProductFieldsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElProductFieldsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElProductNameEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}
impl SecurityhubInsightFiltersElProductNameEl {}
impl ToListMappable for SecurityhubInsightFiltersElProductNameEl {
    type O = BlockAssignable<SecurityhubInsightFiltersElProductNameEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElProductNameEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildSecurityhubInsightFiltersElProductNameEl {
    pub fn build(self) -> SecurityhubInsightFiltersElProductNameEl {
        SecurityhubInsightFiltersElProductNameEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}
pub struct SecurityhubInsightFiltersElProductNameElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElProductNameElRef {
    fn new(shared: StackShared, base: String) -> SecurityhubInsightFiltersElProductNameElRef {
        SecurityhubInsightFiltersElProductNameElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElProductNameElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElRecommendationTextEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}
impl SecurityhubInsightFiltersElRecommendationTextEl {}
impl ToListMappable for SecurityhubInsightFiltersElRecommendationTextEl {
    type O = BlockAssignable<SecurityhubInsightFiltersElRecommendationTextEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElRecommendationTextEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildSecurityhubInsightFiltersElRecommendationTextEl {
    pub fn build(self) -> SecurityhubInsightFiltersElRecommendationTextEl {
        SecurityhubInsightFiltersElRecommendationTextEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}
pub struct SecurityhubInsightFiltersElRecommendationTextElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElRecommendationTextElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubInsightFiltersElRecommendationTextElRef {
        SecurityhubInsightFiltersElRecommendationTextElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElRecommendationTextElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElRecordStateEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}
impl SecurityhubInsightFiltersElRecordStateEl {}
impl ToListMappable for SecurityhubInsightFiltersElRecordStateEl {
    type O = BlockAssignable<SecurityhubInsightFiltersElRecordStateEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElRecordStateEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildSecurityhubInsightFiltersElRecordStateEl {
    pub fn build(self) -> SecurityhubInsightFiltersElRecordStateEl {
        SecurityhubInsightFiltersElRecordStateEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}
pub struct SecurityhubInsightFiltersElRecordStateElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElRecordStateElRef {
    fn new(shared: StackShared, base: String) -> SecurityhubInsightFiltersElRecordStateElRef {
        SecurityhubInsightFiltersElRecordStateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElRecordStateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElRelatedFindingsIdEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}
impl SecurityhubInsightFiltersElRelatedFindingsIdEl {}
impl ToListMappable for SecurityhubInsightFiltersElRelatedFindingsIdEl {
    type O = BlockAssignable<SecurityhubInsightFiltersElRelatedFindingsIdEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElRelatedFindingsIdEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildSecurityhubInsightFiltersElRelatedFindingsIdEl {
    pub fn build(self) -> SecurityhubInsightFiltersElRelatedFindingsIdEl {
        SecurityhubInsightFiltersElRelatedFindingsIdEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}
pub struct SecurityhubInsightFiltersElRelatedFindingsIdElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElRelatedFindingsIdElRef {
    fn new(shared: StackShared, base: String) -> SecurityhubInsightFiltersElRelatedFindingsIdElRef {
        SecurityhubInsightFiltersElRelatedFindingsIdElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElRelatedFindingsIdElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElRelatedFindingsProductArnEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}
impl SecurityhubInsightFiltersElRelatedFindingsProductArnEl {}
impl ToListMappable for SecurityhubInsightFiltersElRelatedFindingsProductArnEl {
    type O = BlockAssignable<SecurityhubInsightFiltersElRelatedFindingsProductArnEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElRelatedFindingsProductArnEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildSecurityhubInsightFiltersElRelatedFindingsProductArnEl {
    pub fn build(self) -> SecurityhubInsightFiltersElRelatedFindingsProductArnEl {
        SecurityhubInsightFiltersElRelatedFindingsProductArnEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}
pub struct SecurityhubInsightFiltersElRelatedFindingsProductArnElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElRelatedFindingsProductArnElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubInsightFiltersElRelatedFindingsProductArnElRef {
        SecurityhubInsightFiltersElRelatedFindingsProductArnElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElRelatedFindingsProductArnElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElResourceAwsEc2InstanceIamInstanceProfileArnEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}
impl SecurityhubInsightFiltersElResourceAwsEc2InstanceIamInstanceProfileArnEl {}
impl ToListMappable for SecurityhubInsightFiltersElResourceAwsEc2InstanceIamInstanceProfileArnEl {
    type O =
        BlockAssignable<SecurityhubInsightFiltersElResourceAwsEc2InstanceIamInstanceProfileArnEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElResourceAwsEc2InstanceIamInstanceProfileArnEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildSecurityhubInsightFiltersElResourceAwsEc2InstanceIamInstanceProfileArnEl {
    pub fn build(self) -> SecurityhubInsightFiltersElResourceAwsEc2InstanceIamInstanceProfileArnEl {
        SecurityhubInsightFiltersElResourceAwsEc2InstanceIamInstanceProfileArnEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}
pub struct SecurityhubInsightFiltersElResourceAwsEc2InstanceIamInstanceProfileArnElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElResourceAwsEc2InstanceIamInstanceProfileArnElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubInsightFiltersElResourceAwsEc2InstanceIamInstanceProfileArnElRef {
        SecurityhubInsightFiltersElResourceAwsEc2InstanceIamInstanceProfileArnElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElResourceAwsEc2InstanceIamInstanceProfileArnElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElResourceAwsEc2InstanceImageIdEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}
impl SecurityhubInsightFiltersElResourceAwsEc2InstanceImageIdEl {}
impl ToListMappable for SecurityhubInsightFiltersElResourceAwsEc2InstanceImageIdEl {
    type O = BlockAssignable<SecurityhubInsightFiltersElResourceAwsEc2InstanceImageIdEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElResourceAwsEc2InstanceImageIdEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildSecurityhubInsightFiltersElResourceAwsEc2InstanceImageIdEl {
    pub fn build(self) -> SecurityhubInsightFiltersElResourceAwsEc2InstanceImageIdEl {
        SecurityhubInsightFiltersElResourceAwsEc2InstanceImageIdEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}
pub struct SecurityhubInsightFiltersElResourceAwsEc2InstanceImageIdElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElResourceAwsEc2InstanceImageIdElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubInsightFiltersElResourceAwsEc2InstanceImageIdElRef {
        SecurityhubInsightFiltersElResourceAwsEc2InstanceImageIdElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElResourceAwsEc2InstanceImageIdElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElResourceAwsEc2InstanceIpv4AddressesEl {
    cidr: PrimField<String>,
}
impl SecurityhubInsightFiltersElResourceAwsEc2InstanceIpv4AddressesEl {}
impl ToListMappable for SecurityhubInsightFiltersElResourceAwsEc2InstanceIpv4AddressesEl {
    type O = BlockAssignable<SecurityhubInsightFiltersElResourceAwsEc2InstanceIpv4AddressesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElResourceAwsEc2InstanceIpv4AddressesEl {
    #[doc = ""]
    pub cidr: PrimField<String>,
}
impl BuildSecurityhubInsightFiltersElResourceAwsEc2InstanceIpv4AddressesEl {
    pub fn build(self) -> SecurityhubInsightFiltersElResourceAwsEc2InstanceIpv4AddressesEl {
        SecurityhubInsightFiltersElResourceAwsEc2InstanceIpv4AddressesEl { cidr: self.cidr }
    }
}
pub struct SecurityhubInsightFiltersElResourceAwsEc2InstanceIpv4AddressesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElResourceAwsEc2InstanceIpv4AddressesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubInsightFiltersElResourceAwsEc2InstanceIpv4AddressesElRef {
        SecurityhubInsightFiltersElResourceAwsEc2InstanceIpv4AddressesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElResourceAwsEc2InstanceIpv4AddressesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cidr` after provisioning.\n"]
    pub fn cidr(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cidr", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElResourceAwsEc2InstanceIpv6AddressesEl {
    cidr: PrimField<String>,
}
impl SecurityhubInsightFiltersElResourceAwsEc2InstanceIpv6AddressesEl {}
impl ToListMappable for SecurityhubInsightFiltersElResourceAwsEc2InstanceIpv6AddressesEl {
    type O = BlockAssignable<SecurityhubInsightFiltersElResourceAwsEc2InstanceIpv6AddressesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElResourceAwsEc2InstanceIpv6AddressesEl {
    #[doc = ""]
    pub cidr: PrimField<String>,
}
impl BuildSecurityhubInsightFiltersElResourceAwsEc2InstanceIpv6AddressesEl {
    pub fn build(self) -> SecurityhubInsightFiltersElResourceAwsEc2InstanceIpv6AddressesEl {
        SecurityhubInsightFiltersElResourceAwsEc2InstanceIpv6AddressesEl { cidr: self.cidr }
    }
}
pub struct SecurityhubInsightFiltersElResourceAwsEc2InstanceIpv6AddressesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElResourceAwsEc2InstanceIpv6AddressesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubInsightFiltersElResourceAwsEc2InstanceIpv6AddressesElRef {
        SecurityhubInsightFiltersElResourceAwsEc2InstanceIpv6AddressesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElResourceAwsEc2InstanceIpv6AddressesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cidr` after provisioning.\n"]
    pub fn cidr(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cidr", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElResourceAwsEc2InstanceKeyNameEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}
impl SecurityhubInsightFiltersElResourceAwsEc2InstanceKeyNameEl {}
impl ToListMappable for SecurityhubInsightFiltersElResourceAwsEc2InstanceKeyNameEl {
    type O = BlockAssignable<SecurityhubInsightFiltersElResourceAwsEc2InstanceKeyNameEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElResourceAwsEc2InstanceKeyNameEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildSecurityhubInsightFiltersElResourceAwsEc2InstanceKeyNameEl {
    pub fn build(self) -> SecurityhubInsightFiltersElResourceAwsEc2InstanceKeyNameEl {
        SecurityhubInsightFiltersElResourceAwsEc2InstanceKeyNameEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}
pub struct SecurityhubInsightFiltersElResourceAwsEc2InstanceKeyNameElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElResourceAwsEc2InstanceKeyNameElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubInsightFiltersElResourceAwsEc2InstanceKeyNameElRef {
        SecurityhubInsightFiltersElResourceAwsEc2InstanceKeyNameElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElResourceAwsEc2InstanceKeyNameElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElResourceAwsEc2InstanceLaunchedAtElDateRangeEl {
    unit: PrimField<String>,
    value: PrimField<f64>,
}
impl SecurityhubInsightFiltersElResourceAwsEc2InstanceLaunchedAtElDateRangeEl {}
impl ToListMappable for SecurityhubInsightFiltersElResourceAwsEc2InstanceLaunchedAtElDateRangeEl {
    type O =
        BlockAssignable<SecurityhubInsightFiltersElResourceAwsEc2InstanceLaunchedAtElDateRangeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElResourceAwsEc2InstanceLaunchedAtElDateRangeEl {
    #[doc = ""]
    pub unit: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<f64>,
}
impl BuildSecurityhubInsightFiltersElResourceAwsEc2InstanceLaunchedAtElDateRangeEl {
    pub fn build(self) -> SecurityhubInsightFiltersElResourceAwsEc2InstanceLaunchedAtElDateRangeEl {
        SecurityhubInsightFiltersElResourceAwsEc2InstanceLaunchedAtElDateRangeEl {
            unit: self.unit,
            value: self.value,
        }
    }
}
pub struct SecurityhubInsightFiltersElResourceAwsEc2InstanceLaunchedAtElDateRangeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElResourceAwsEc2InstanceLaunchedAtElDateRangeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubInsightFiltersElResourceAwsEc2InstanceLaunchedAtElDateRangeElRef {
        SecurityhubInsightFiltersElResourceAwsEc2InstanceLaunchedAtElDateRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElResourceAwsEc2InstanceLaunchedAtElDateRangeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize, Default)]
struct SecurityhubInsightFiltersElResourceAwsEc2InstanceLaunchedAtElDynamic {
    date_range: Option<
        DynamicBlock<SecurityhubInsightFiltersElResourceAwsEc2InstanceLaunchedAtElDateRangeEl>,
    >,
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElResourceAwsEc2InstanceLaunchedAtEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    end: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    date_range:
        Option<Vec<SecurityhubInsightFiltersElResourceAwsEc2InstanceLaunchedAtElDateRangeEl>>,
    dynamic: SecurityhubInsightFiltersElResourceAwsEc2InstanceLaunchedAtElDynamic,
}
impl SecurityhubInsightFiltersElResourceAwsEc2InstanceLaunchedAtEl {
    #[doc = "Set the field `end`.\n"]
    pub fn set_end(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.end = Some(v.into());
        self
    }
    #[doc = "Set the field `start`.\n"]
    pub fn set_start(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.start = Some(v.into());
        self
    }
    #[doc = "Set the field `date_range`.\n"]
    pub fn set_date_range(
        mut self,
        v: impl Into<
            BlockAssignable<
                SecurityhubInsightFiltersElResourceAwsEc2InstanceLaunchedAtElDateRangeEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.date_range = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.date_range = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for SecurityhubInsightFiltersElResourceAwsEc2InstanceLaunchedAtEl {
    type O = BlockAssignable<SecurityhubInsightFiltersElResourceAwsEc2InstanceLaunchedAtEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElResourceAwsEc2InstanceLaunchedAtEl {}
impl BuildSecurityhubInsightFiltersElResourceAwsEc2InstanceLaunchedAtEl {
    pub fn build(self) -> SecurityhubInsightFiltersElResourceAwsEc2InstanceLaunchedAtEl {
        SecurityhubInsightFiltersElResourceAwsEc2InstanceLaunchedAtEl {
            end: core::default::Default::default(),
            start: core::default::Default::default(),
            date_range: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct SecurityhubInsightFiltersElResourceAwsEc2InstanceLaunchedAtElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElResourceAwsEc2InstanceLaunchedAtElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubInsightFiltersElResourceAwsEc2InstanceLaunchedAtElRef {
        SecurityhubInsightFiltersElResourceAwsEc2InstanceLaunchedAtElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElResourceAwsEc2InstanceLaunchedAtElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `end` after provisioning.\n"]
    pub fn end(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.end", self.base))
    }
    #[doc = "Get a reference to the value of field `start` after provisioning.\n"]
    pub fn start(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start", self.base))
    }
    #[doc = "Get a reference to the value of field `date_range` after provisioning.\n"]
    pub fn date_range(
        &self,
    ) -> ListRef<SecurityhubInsightFiltersElResourceAwsEc2InstanceLaunchedAtElDateRangeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.date_range", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElResourceAwsEc2InstanceSubnetIdEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}
impl SecurityhubInsightFiltersElResourceAwsEc2InstanceSubnetIdEl {}
impl ToListMappable for SecurityhubInsightFiltersElResourceAwsEc2InstanceSubnetIdEl {
    type O = BlockAssignable<SecurityhubInsightFiltersElResourceAwsEc2InstanceSubnetIdEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElResourceAwsEc2InstanceSubnetIdEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildSecurityhubInsightFiltersElResourceAwsEc2InstanceSubnetIdEl {
    pub fn build(self) -> SecurityhubInsightFiltersElResourceAwsEc2InstanceSubnetIdEl {
        SecurityhubInsightFiltersElResourceAwsEc2InstanceSubnetIdEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}
pub struct SecurityhubInsightFiltersElResourceAwsEc2InstanceSubnetIdElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElResourceAwsEc2InstanceSubnetIdElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubInsightFiltersElResourceAwsEc2InstanceSubnetIdElRef {
        SecurityhubInsightFiltersElResourceAwsEc2InstanceSubnetIdElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElResourceAwsEc2InstanceSubnetIdElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElResourceAwsEc2InstanceTypeEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}
impl SecurityhubInsightFiltersElResourceAwsEc2InstanceTypeEl {}
impl ToListMappable for SecurityhubInsightFiltersElResourceAwsEc2InstanceTypeEl {
    type O = BlockAssignable<SecurityhubInsightFiltersElResourceAwsEc2InstanceTypeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElResourceAwsEc2InstanceTypeEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildSecurityhubInsightFiltersElResourceAwsEc2InstanceTypeEl {
    pub fn build(self) -> SecurityhubInsightFiltersElResourceAwsEc2InstanceTypeEl {
        SecurityhubInsightFiltersElResourceAwsEc2InstanceTypeEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}
pub struct SecurityhubInsightFiltersElResourceAwsEc2InstanceTypeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElResourceAwsEc2InstanceTypeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubInsightFiltersElResourceAwsEc2InstanceTypeElRef {
        SecurityhubInsightFiltersElResourceAwsEc2InstanceTypeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElResourceAwsEc2InstanceTypeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElResourceAwsEc2InstanceVpcIdEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}
impl SecurityhubInsightFiltersElResourceAwsEc2InstanceVpcIdEl {}
impl ToListMappable for SecurityhubInsightFiltersElResourceAwsEc2InstanceVpcIdEl {
    type O = BlockAssignable<SecurityhubInsightFiltersElResourceAwsEc2InstanceVpcIdEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElResourceAwsEc2InstanceVpcIdEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildSecurityhubInsightFiltersElResourceAwsEc2InstanceVpcIdEl {
    pub fn build(self) -> SecurityhubInsightFiltersElResourceAwsEc2InstanceVpcIdEl {
        SecurityhubInsightFiltersElResourceAwsEc2InstanceVpcIdEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}
pub struct SecurityhubInsightFiltersElResourceAwsEc2InstanceVpcIdElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElResourceAwsEc2InstanceVpcIdElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubInsightFiltersElResourceAwsEc2InstanceVpcIdElRef {
        SecurityhubInsightFiltersElResourceAwsEc2InstanceVpcIdElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElResourceAwsEc2InstanceVpcIdElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElResourceAwsIamAccessKeyCreatedAtElDateRangeEl {
    unit: PrimField<String>,
    value: PrimField<f64>,
}
impl SecurityhubInsightFiltersElResourceAwsIamAccessKeyCreatedAtElDateRangeEl {}
impl ToListMappable for SecurityhubInsightFiltersElResourceAwsIamAccessKeyCreatedAtElDateRangeEl {
    type O =
        BlockAssignable<SecurityhubInsightFiltersElResourceAwsIamAccessKeyCreatedAtElDateRangeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElResourceAwsIamAccessKeyCreatedAtElDateRangeEl {
    #[doc = ""]
    pub unit: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<f64>,
}
impl BuildSecurityhubInsightFiltersElResourceAwsIamAccessKeyCreatedAtElDateRangeEl {
    pub fn build(self) -> SecurityhubInsightFiltersElResourceAwsIamAccessKeyCreatedAtElDateRangeEl {
        SecurityhubInsightFiltersElResourceAwsIamAccessKeyCreatedAtElDateRangeEl {
            unit: self.unit,
            value: self.value,
        }
    }
}
pub struct SecurityhubInsightFiltersElResourceAwsIamAccessKeyCreatedAtElDateRangeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElResourceAwsIamAccessKeyCreatedAtElDateRangeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubInsightFiltersElResourceAwsIamAccessKeyCreatedAtElDateRangeElRef {
        SecurityhubInsightFiltersElResourceAwsIamAccessKeyCreatedAtElDateRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElResourceAwsIamAccessKeyCreatedAtElDateRangeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize, Default)]
struct SecurityhubInsightFiltersElResourceAwsIamAccessKeyCreatedAtElDynamic {
    date_range: Option<
        DynamicBlock<SecurityhubInsightFiltersElResourceAwsIamAccessKeyCreatedAtElDateRangeEl>,
    >,
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElResourceAwsIamAccessKeyCreatedAtEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    end: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    date_range:
        Option<Vec<SecurityhubInsightFiltersElResourceAwsIamAccessKeyCreatedAtElDateRangeEl>>,
    dynamic: SecurityhubInsightFiltersElResourceAwsIamAccessKeyCreatedAtElDynamic,
}
impl SecurityhubInsightFiltersElResourceAwsIamAccessKeyCreatedAtEl {
    #[doc = "Set the field `end`.\n"]
    pub fn set_end(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.end = Some(v.into());
        self
    }
    #[doc = "Set the field `start`.\n"]
    pub fn set_start(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.start = Some(v.into());
        self
    }
    #[doc = "Set the field `date_range`.\n"]
    pub fn set_date_range(
        mut self,
        v: impl Into<
            BlockAssignable<
                SecurityhubInsightFiltersElResourceAwsIamAccessKeyCreatedAtElDateRangeEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.date_range = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.date_range = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for SecurityhubInsightFiltersElResourceAwsIamAccessKeyCreatedAtEl {
    type O = BlockAssignable<SecurityhubInsightFiltersElResourceAwsIamAccessKeyCreatedAtEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElResourceAwsIamAccessKeyCreatedAtEl {}
impl BuildSecurityhubInsightFiltersElResourceAwsIamAccessKeyCreatedAtEl {
    pub fn build(self) -> SecurityhubInsightFiltersElResourceAwsIamAccessKeyCreatedAtEl {
        SecurityhubInsightFiltersElResourceAwsIamAccessKeyCreatedAtEl {
            end: core::default::Default::default(),
            start: core::default::Default::default(),
            date_range: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct SecurityhubInsightFiltersElResourceAwsIamAccessKeyCreatedAtElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElResourceAwsIamAccessKeyCreatedAtElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubInsightFiltersElResourceAwsIamAccessKeyCreatedAtElRef {
        SecurityhubInsightFiltersElResourceAwsIamAccessKeyCreatedAtElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElResourceAwsIamAccessKeyCreatedAtElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `end` after provisioning.\n"]
    pub fn end(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.end", self.base))
    }
    #[doc = "Get a reference to the value of field `start` after provisioning.\n"]
    pub fn start(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start", self.base))
    }
    #[doc = "Get a reference to the value of field `date_range` after provisioning.\n"]
    pub fn date_range(
        &self,
    ) -> ListRef<SecurityhubInsightFiltersElResourceAwsIamAccessKeyCreatedAtElDateRangeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.date_range", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElResourceAwsIamAccessKeyStatusEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}
impl SecurityhubInsightFiltersElResourceAwsIamAccessKeyStatusEl {}
impl ToListMappable for SecurityhubInsightFiltersElResourceAwsIamAccessKeyStatusEl {
    type O = BlockAssignable<SecurityhubInsightFiltersElResourceAwsIamAccessKeyStatusEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElResourceAwsIamAccessKeyStatusEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildSecurityhubInsightFiltersElResourceAwsIamAccessKeyStatusEl {
    pub fn build(self) -> SecurityhubInsightFiltersElResourceAwsIamAccessKeyStatusEl {
        SecurityhubInsightFiltersElResourceAwsIamAccessKeyStatusEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}
pub struct SecurityhubInsightFiltersElResourceAwsIamAccessKeyStatusElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElResourceAwsIamAccessKeyStatusElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubInsightFiltersElResourceAwsIamAccessKeyStatusElRef {
        SecurityhubInsightFiltersElResourceAwsIamAccessKeyStatusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElResourceAwsIamAccessKeyStatusElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElResourceAwsIamAccessKeyUserNameEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}
impl SecurityhubInsightFiltersElResourceAwsIamAccessKeyUserNameEl {}
impl ToListMappable for SecurityhubInsightFiltersElResourceAwsIamAccessKeyUserNameEl {
    type O = BlockAssignable<SecurityhubInsightFiltersElResourceAwsIamAccessKeyUserNameEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElResourceAwsIamAccessKeyUserNameEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildSecurityhubInsightFiltersElResourceAwsIamAccessKeyUserNameEl {
    pub fn build(self) -> SecurityhubInsightFiltersElResourceAwsIamAccessKeyUserNameEl {
        SecurityhubInsightFiltersElResourceAwsIamAccessKeyUserNameEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}
pub struct SecurityhubInsightFiltersElResourceAwsIamAccessKeyUserNameElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElResourceAwsIamAccessKeyUserNameElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubInsightFiltersElResourceAwsIamAccessKeyUserNameElRef {
        SecurityhubInsightFiltersElResourceAwsIamAccessKeyUserNameElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElResourceAwsIamAccessKeyUserNameElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElResourceAwsS3BucketOwnerIdEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}
impl SecurityhubInsightFiltersElResourceAwsS3BucketOwnerIdEl {}
impl ToListMappable for SecurityhubInsightFiltersElResourceAwsS3BucketOwnerIdEl {
    type O = BlockAssignable<SecurityhubInsightFiltersElResourceAwsS3BucketOwnerIdEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElResourceAwsS3BucketOwnerIdEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildSecurityhubInsightFiltersElResourceAwsS3BucketOwnerIdEl {
    pub fn build(self) -> SecurityhubInsightFiltersElResourceAwsS3BucketOwnerIdEl {
        SecurityhubInsightFiltersElResourceAwsS3BucketOwnerIdEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}
pub struct SecurityhubInsightFiltersElResourceAwsS3BucketOwnerIdElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElResourceAwsS3BucketOwnerIdElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubInsightFiltersElResourceAwsS3BucketOwnerIdElRef {
        SecurityhubInsightFiltersElResourceAwsS3BucketOwnerIdElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElResourceAwsS3BucketOwnerIdElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElResourceAwsS3BucketOwnerNameEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}
impl SecurityhubInsightFiltersElResourceAwsS3BucketOwnerNameEl {}
impl ToListMappable for SecurityhubInsightFiltersElResourceAwsS3BucketOwnerNameEl {
    type O = BlockAssignable<SecurityhubInsightFiltersElResourceAwsS3BucketOwnerNameEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElResourceAwsS3BucketOwnerNameEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildSecurityhubInsightFiltersElResourceAwsS3BucketOwnerNameEl {
    pub fn build(self) -> SecurityhubInsightFiltersElResourceAwsS3BucketOwnerNameEl {
        SecurityhubInsightFiltersElResourceAwsS3BucketOwnerNameEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}
pub struct SecurityhubInsightFiltersElResourceAwsS3BucketOwnerNameElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElResourceAwsS3BucketOwnerNameElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubInsightFiltersElResourceAwsS3BucketOwnerNameElRef {
        SecurityhubInsightFiltersElResourceAwsS3BucketOwnerNameElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElResourceAwsS3BucketOwnerNameElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElResourceContainerImageIdEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}
impl SecurityhubInsightFiltersElResourceContainerImageIdEl {}
impl ToListMappable for SecurityhubInsightFiltersElResourceContainerImageIdEl {
    type O = BlockAssignable<SecurityhubInsightFiltersElResourceContainerImageIdEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElResourceContainerImageIdEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildSecurityhubInsightFiltersElResourceContainerImageIdEl {
    pub fn build(self) -> SecurityhubInsightFiltersElResourceContainerImageIdEl {
        SecurityhubInsightFiltersElResourceContainerImageIdEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}
pub struct SecurityhubInsightFiltersElResourceContainerImageIdElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElResourceContainerImageIdElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubInsightFiltersElResourceContainerImageIdElRef {
        SecurityhubInsightFiltersElResourceContainerImageIdElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElResourceContainerImageIdElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElResourceContainerImageNameEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}
impl SecurityhubInsightFiltersElResourceContainerImageNameEl {}
impl ToListMappable for SecurityhubInsightFiltersElResourceContainerImageNameEl {
    type O = BlockAssignable<SecurityhubInsightFiltersElResourceContainerImageNameEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElResourceContainerImageNameEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildSecurityhubInsightFiltersElResourceContainerImageNameEl {
    pub fn build(self) -> SecurityhubInsightFiltersElResourceContainerImageNameEl {
        SecurityhubInsightFiltersElResourceContainerImageNameEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}
pub struct SecurityhubInsightFiltersElResourceContainerImageNameElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElResourceContainerImageNameElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubInsightFiltersElResourceContainerImageNameElRef {
        SecurityhubInsightFiltersElResourceContainerImageNameElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElResourceContainerImageNameElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElResourceContainerLaunchedAtElDateRangeEl {
    unit: PrimField<String>,
    value: PrimField<f64>,
}
impl SecurityhubInsightFiltersElResourceContainerLaunchedAtElDateRangeEl {}
impl ToListMappable for SecurityhubInsightFiltersElResourceContainerLaunchedAtElDateRangeEl {
    type O = BlockAssignable<SecurityhubInsightFiltersElResourceContainerLaunchedAtElDateRangeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElResourceContainerLaunchedAtElDateRangeEl {
    #[doc = ""]
    pub unit: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<f64>,
}
impl BuildSecurityhubInsightFiltersElResourceContainerLaunchedAtElDateRangeEl {
    pub fn build(self) -> SecurityhubInsightFiltersElResourceContainerLaunchedAtElDateRangeEl {
        SecurityhubInsightFiltersElResourceContainerLaunchedAtElDateRangeEl {
            unit: self.unit,
            value: self.value,
        }
    }
}
pub struct SecurityhubInsightFiltersElResourceContainerLaunchedAtElDateRangeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElResourceContainerLaunchedAtElDateRangeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubInsightFiltersElResourceContainerLaunchedAtElDateRangeElRef {
        SecurityhubInsightFiltersElResourceContainerLaunchedAtElDateRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElResourceContainerLaunchedAtElDateRangeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize, Default)]
struct SecurityhubInsightFiltersElResourceContainerLaunchedAtElDynamic {
    date_range:
        Option<DynamicBlock<SecurityhubInsightFiltersElResourceContainerLaunchedAtElDateRangeEl>>,
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElResourceContainerLaunchedAtEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    end: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    date_range: Option<Vec<SecurityhubInsightFiltersElResourceContainerLaunchedAtElDateRangeEl>>,
    dynamic: SecurityhubInsightFiltersElResourceContainerLaunchedAtElDynamic,
}
impl SecurityhubInsightFiltersElResourceContainerLaunchedAtEl {
    #[doc = "Set the field `end`.\n"]
    pub fn set_end(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.end = Some(v.into());
        self
    }
    #[doc = "Set the field `start`.\n"]
    pub fn set_start(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.start = Some(v.into());
        self
    }
    #[doc = "Set the field `date_range`.\n"]
    pub fn set_date_range(
        mut self,
        v: impl Into<
            BlockAssignable<SecurityhubInsightFiltersElResourceContainerLaunchedAtElDateRangeEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.date_range = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.date_range = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for SecurityhubInsightFiltersElResourceContainerLaunchedAtEl {
    type O = BlockAssignable<SecurityhubInsightFiltersElResourceContainerLaunchedAtEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElResourceContainerLaunchedAtEl {}
impl BuildSecurityhubInsightFiltersElResourceContainerLaunchedAtEl {
    pub fn build(self) -> SecurityhubInsightFiltersElResourceContainerLaunchedAtEl {
        SecurityhubInsightFiltersElResourceContainerLaunchedAtEl {
            end: core::default::Default::default(),
            start: core::default::Default::default(),
            date_range: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct SecurityhubInsightFiltersElResourceContainerLaunchedAtElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElResourceContainerLaunchedAtElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubInsightFiltersElResourceContainerLaunchedAtElRef {
        SecurityhubInsightFiltersElResourceContainerLaunchedAtElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElResourceContainerLaunchedAtElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `end` after provisioning.\n"]
    pub fn end(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.end", self.base))
    }
    #[doc = "Get a reference to the value of field `start` after provisioning.\n"]
    pub fn start(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start", self.base))
    }
    #[doc = "Get a reference to the value of field `date_range` after provisioning.\n"]
    pub fn date_range(
        &self,
    ) -> ListRef<SecurityhubInsightFiltersElResourceContainerLaunchedAtElDateRangeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.date_range", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElResourceContainerNameEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}
impl SecurityhubInsightFiltersElResourceContainerNameEl {}
impl ToListMappable for SecurityhubInsightFiltersElResourceContainerNameEl {
    type O = BlockAssignable<SecurityhubInsightFiltersElResourceContainerNameEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElResourceContainerNameEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildSecurityhubInsightFiltersElResourceContainerNameEl {
    pub fn build(self) -> SecurityhubInsightFiltersElResourceContainerNameEl {
        SecurityhubInsightFiltersElResourceContainerNameEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}
pub struct SecurityhubInsightFiltersElResourceContainerNameElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElResourceContainerNameElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubInsightFiltersElResourceContainerNameElRef {
        SecurityhubInsightFiltersElResourceContainerNameElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElResourceContainerNameElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElResourceDetailsOtherEl {
    comparison: PrimField<String>,
    key: PrimField<String>,
    value: PrimField<String>,
}
impl SecurityhubInsightFiltersElResourceDetailsOtherEl {}
impl ToListMappable for SecurityhubInsightFiltersElResourceDetailsOtherEl {
    type O = BlockAssignable<SecurityhubInsightFiltersElResourceDetailsOtherEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElResourceDetailsOtherEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub key: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildSecurityhubInsightFiltersElResourceDetailsOtherEl {
    pub fn build(self) -> SecurityhubInsightFiltersElResourceDetailsOtherEl {
        SecurityhubInsightFiltersElResourceDetailsOtherEl {
            comparison: self.comparison,
            key: self.key,
            value: self.value,
        }
    }
}
pub struct SecurityhubInsightFiltersElResourceDetailsOtherElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElResourceDetailsOtherElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubInsightFiltersElResourceDetailsOtherElRef {
        SecurityhubInsightFiltersElResourceDetailsOtherElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElResourceDetailsOtherElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElResourceIdEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}
impl SecurityhubInsightFiltersElResourceIdEl {}
impl ToListMappable for SecurityhubInsightFiltersElResourceIdEl {
    type O = BlockAssignable<SecurityhubInsightFiltersElResourceIdEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElResourceIdEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildSecurityhubInsightFiltersElResourceIdEl {
    pub fn build(self) -> SecurityhubInsightFiltersElResourceIdEl {
        SecurityhubInsightFiltersElResourceIdEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}
pub struct SecurityhubInsightFiltersElResourceIdElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElResourceIdElRef {
    fn new(shared: StackShared, base: String) -> SecurityhubInsightFiltersElResourceIdElRef {
        SecurityhubInsightFiltersElResourceIdElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElResourceIdElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElResourcePartitionEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}
impl SecurityhubInsightFiltersElResourcePartitionEl {}
impl ToListMappable for SecurityhubInsightFiltersElResourcePartitionEl {
    type O = BlockAssignable<SecurityhubInsightFiltersElResourcePartitionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElResourcePartitionEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildSecurityhubInsightFiltersElResourcePartitionEl {
    pub fn build(self) -> SecurityhubInsightFiltersElResourcePartitionEl {
        SecurityhubInsightFiltersElResourcePartitionEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}
pub struct SecurityhubInsightFiltersElResourcePartitionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElResourcePartitionElRef {
    fn new(shared: StackShared, base: String) -> SecurityhubInsightFiltersElResourcePartitionElRef {
        SecurityhubInsightFiltersElResourcePartitionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElResourcePartitionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElResourceRegionEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}
impl SecurityhubInsightFiltersElResourceRegionEl {}
impl ToListMappable for SecurityhubInsightFiltersElResourceRegionEl {
    type O = BlockAssignable<SecurityhubInsightFiltersElResourceRegionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElResourceRegionEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildSecurityhubInsightFiltersElResourceRegionEl {
    pub fn build(self) -> SecurityhubInsightFiltersElResourceRegionEl {
        SecurityhubInsightFiltersElResourceRegionEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}
pub struct SecurityhubInsightFiltersElResourceRegionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElResourceRegionElRef {
    fn new(shared: StackShared, base: String) -> SecurityhubInsightFiltersElResourceRegionElRef {
        SecurityhubInsightFiltersElResourceRegionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElResourceRegionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElResourceTagsEl {
    comparison: PrimField<String>,
    key: PrimField<String>,
    value: PrimField<String>,
}
impl SecurityhubInsightFiltersElResourceTagsEl {}
impl ToListMappable for SecurityhubInsightFiltersElResourceTagsEl {
    type O = BlockAssignable<SecurityhubInsightFiltersElResourceTagsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElResourceTagsEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub key: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildSecurityhubInsightFiltersElResourceTagsEl {
    pub fn build(self) -> SecurityhubInsightFiltersElResourceTagsEl {
        SecurityhubInsightFiltersElResourceTagsEl {
            comparison: self.comparison,
            key: self.key,
            value: self.value,
        }
    }
}
pub struct SecurityhubInsightFiltersElResourceTagsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElResourceTagsElRef {
    fn new(shared: StackShared, base: String) -> SecurityhubInsightFiltersElResourceTagsElRef {
        SecurityhubInsightFiltersElResourceTagsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElResourceTagsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElResourceTypeEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}
impl SecurityhubInsightFiltersElResourceTypeEl {}
impl ToListMappable for SecurityhubInsightFiltersElResourceTypeEl {
    type O = BlockAssignable<SecurityhubInsightFiltersElResourceTypeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElResourceTypeEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildSecurityhubInsightFiltersElResourceTypeEl {
    pub fn build(self) -> SecurityhubInsightFiltersElResourceTypeEl {
        SecurityhubInsightFiltersElResourceTypeEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}
pub struct SecurityhubInsightFiltersElResourceTypeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElResourceTypeElRef {
    fn new(shared: StackShared, base: String) -> SecurityhubInsightFiltersElResourceTypeElRef {
        SecurityhubInsightFiltersElResourceTypeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElResourceTypeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElSeverityLabelEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}
impl SecurityhubInsightFiltersElSeverityLabelEl {}
impl ToListMappable for SecurityhubInsightFiltersElSeverityLabelEl {
    type O = BlockAssignable<SecurityhubInsightFiltersElSeverityLabelEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElSeverityLabelEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildSecurityhubInsightFiltersElSeverityLabelEl {
    pub fn build(self) -> SecurityhubInsightFiltersElSeverityLabelEl {
        SecurityhubInsightFiltersElSeverityLabelEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}
pub struct SecurityhubInsightFiltersElSeverityLabelElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElSeverityLabelElRef {
    fn new(shared: StackShared, base: String) -> SecurityhubInsightFiltersElSeverityLabelElRef {
        SecurityhubInsightFiltersElSeverityLabelElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElSeverityLabelElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElSourceUrlEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}
impl SecurityhubInsightFiltersElSourceUrlEl {}
impl ToListMappable for SecurityhubInsightFiltersElSourceUrlEl {
    type O = BlockAssignable<SecurityhubInsightFiltersElSourceUrlEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElSourceUrlEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildSecurityhubInsightFiltersElSourceUrlEl {
    pub fn build(self) -> SecurityhubInsightFiltersElSourceUrlEl {
        SecurityhubInsightFiltersElSourceUrlEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}
pub struct SecurityhubInsightFiltersElSourceUrlElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElSourceUrlElRef {
    fn new(shared: StackShared, base: String) -> SecurityhubInsightFiltersElSourceUrlElRef {
        SecurityhubInsightFiltersElSourceUrlElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElSourceUrlElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElThreatIntelIndicatorCategoryEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}
impl SecurityhubInsightFiltersElThreatIntelIndicatorCategoryEl {}
impl ToListMappable for SecurityhubInsightFiltersElThreatIntelIndicatorCategoryEl {
    type O = BlockAssignable<SecurityhubInsightFiltersElThreatIntelIndicatorCategoryEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElThreatIntelIndicatorCategoryEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildSecurityhubInsightFiltersElThreatIntelIndicatorCategoryEl {
    pub fn build(self) -> SecurityhubInsightFiltersElThreatIntelIndicatorCategoryEl {
        SecurityhubInsightFiltersElThreatIntelIndicatorCategoryEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}
pub struct SecurityhubInsightFiltersElThreatIntelIndicatorCategoryElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElThreatIntelIndicatorCategoryElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubInsightFiltersElThreatIntelIndicatorCategoryElRef {
        SecurityhubInsightFiltersElThreatIntelIndicatorCategoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElThreatIntelIndicatorCategoryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElThreatIntelIndicatorLastObservedAtElDateRangeEl {
    unit: PrimField<String>,
    value: PrimField<f64>,
}
impl SecurityhubInsightFiltersElThreatIntelIndicatorLastObservedAtElDateRangeEl {}
impl ToListMappable for SecurityhubInsightFiltersElThreatIntelIndicatorLastObservedAtElDateRangeEl {
    type O =
        BlockAssignable<SecurityhubInsightFiltersElThreatIntelIndicatorLastObservedAtElDateRangeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElThreatIntelIndicatorLastObservedAtElDateRangeEl {
    #[doc = ""]
    pub unit: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<f64>,
}
impl BuildSecurityhubInsightFiltersElThreatIntelIndicatorLastObservedAtElDateRangeEl {
    pub fn build(
        self,
    ) -> SecurityhubInsightFiltersElThreatIntelIndicatorLastObservedAtElDateRangeEl {
        SecurityhubInsightFiltersElThreatIntelIndicatorLastObservedAtElDateRangeEl {
            unit: self.unit,
            value: self.value,
        }
    }
}
pub struct SecurityhubInsightFiltersElThreatIntelIndicatorLastObservedAtElDateRangeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElThreatIntelIndicatorLastObservedAtElDateRangeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubInsightFiltersElThreatIntelIndicatorLastObservedAtElDateRangeElRef {
        SecurityhubInsightFiltersElThreatIntelIndicatorLastObservedAtElDateRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElThreatIntelIndicatorLastObservedAtElDateRangeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize, Default)]
struct SecurityhubInsightFiltersElThreatIntelIndicatorLastObservedAtElDynamic {
    date_range: Option<
        DynamicBlock<SecurityhubInsightFiltersElThreatIntelIndicatorLastObservedAtElDateRangeEl>,
    >,
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElThreatIntelIndicatorLastObservedAtEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    end: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    date_range:
        Option<Vec<SecurityhubInsightFiltersElThreatIntelIndicatorLastObservedAtElDateRangeEl>>,
    dynamic: SecurityhubInsightFiltersElThreatIntelIndicatorLastObservedAtElDynamic,
}
impl SecurityhubInsightFiltersElThreatIntelIndicatorLastObservedAtEl {
    #[doc = "Set the field `end`.\n"]
    pub fn set_end(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.end = Some(v.into());
        self
    }
    #[doc = "Set the field `start`.\n"]
    pub fn set_start(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.start = Some(v.into());
        self
    }
    #[doc = "Set the field `date_range`.\n"]
    pub fn set_date_range(
        mut self,
        v: impl Into<
            BlockAssignable<
                SecurityhubInsightFiltersElThreatIntelIndicatorLastObservedAtElDateRangeEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.date_range = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.date_range = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for SecurityhubInsightFiltersElThreatIntelIndicatorLastObservedAtEl {
    type O = BlockAssignable<SecurityhubInsightFiltersElThreatIntelIndicatorLastObservedAtEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElThreatIntelIndicatorLastObservedAtEl {}
impl BuildSecurityhubInsightFiltersElThreatIntelIndicatorLastObservedAtEl {
    pub fn build(self) -> SecurityhubInsightFiltersElThreatIntelIndicatorLastObservedAtEl {
        SecurityhubInsightFiltersElThreatIntelIndicatorLastObservedAtEl {
            end: core::default::Default::default(),
            start: core::default::Default::default(),
            date_range: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct SecurityhubInsightFiltersElThreatIntelIndicatorLastObservedAtElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElThreatIntelIndicatorLastObservedAtElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubInsightFiltersElThreatIntelIndicatorLastObservedAtElRef {
        SecurityhubInsightFiltersElThreatIntelIndicatorLastObservedAtElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElThreatIntelIndicatorLastObservedAtElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `end` after provisioning.\n"]
    pub fn end(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.end", self.base))
    }
    #[doc = "Get a reference to the value of field `start` after provisioning.\n"]
    pub fn start(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start", self.base))
    }
    #[doc = "Get a reference to the value of field `date_range` after provisioning.\n"]
    pub fn date_range(
        &self,
    ) -> ListRef<SecurityhubInsightFiltersElThreatIntelIndicatorLastObservedAtElDateRangeElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.date_range", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElThreatIntelIndicatorSourceEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}
impl SecurityhubInsightFiltersElThreatIntelIndicatorSourceEl {}
impl ToListMappable for SecurityhubInsightFiltersElThreatIntelIndicatorSourceEl {
    type O = BlockAssignable<SecurityhubInsightFiltersElThreatIntelIndicatorSourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElThreatIntelIndicatorSourceEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildSecurityhubInsightFiltersElThreatIntelIndicatorSourceEl {
    pub fn build(self) -> SecurityhubInsightFiltersElThreatIntelIndicatorSourceEl {
        SecurityhubInsightFiltersElThreatIntelIndicatorSourceEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}
pub struct SecurityhubInsightFiltersElThreatIntelIndicatorSourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElThreatIntelIndicatorSourceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubInsightFiltersElThreatIntelIndicatorSourceElRef {
        SecurityhubInsightFiltersElThreatIntelIndicatorSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElThreatIntelIndicatorSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElThreatIntelIndicatorSourceUrlEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}
impl SecurityhubInsightFiltersElThreatIntelIndicatorSourceUrlEl {}
impl ToListMappable for SecurityhubInsightFiltersElThreatIntelIndicatorSourceUrlEl {
    type O = BlockAssignable<SecurityhubInsightFiltersElThreatIntelIndicatorSourceUrlEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElThreatIntelIndicatorSourceUrlEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildSecurityhubInsightFiltersElThreatIntelIndicatorSourceUrlEl {
    pub fn build(self) -> SecurityhubInsightFiltersElThreatIntelIndicatorSourceUrlEl {
        SecurityhubInsightFiltersElThreatIntelIndicatorSourceUrlEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}
pub struct SecurityhubInsightFiltersElThreatIntelIndicatorSourceUrlElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElThreatIntelIndicatorSourceUrlElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubInsightFiltersElThreatIntelIndicatorSourceUrlElRef {
        SecurityhubInsightFiltersElThreatIntelIndicatorSourceUrlElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElThreatIntelIndicatorSourceUrlElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElThreatIntelIndicatorTypeEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}
impl SecurityhubInsightFiltersElThreatIntelIndicatorTypeEl {}
impl ToListMappable for SecurityhubInsightFiltersElThreatIntelIndicatorTypeEl {
    type O = BlockAssignable<SecurityhubInsightFiltersElThreatIntelIndicatorTypeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElThreatIntelIndicatorTypeEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildSecurityhubInsightFiltersElThreatIntelIndicatorTypeEl {
    pub fn build(self) -> SecurityhubInsightFiltersElThreatIntelIndicatorTypeEl {
        SecurityhubInsightFiltersElThreatIntelIndicatorTypeEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}
pub struct SecurityhubInsightFiltersElThreatIntelIndicatorTypeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElThreatIntelIndicatorTypeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubInsightFiltersElThreatIntelIndicatorTypeElRef {
        SecurityhubInsightFiltersElThreatIntelIndicatorTypeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElThreatIntelIndicatorTypeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElThreatIntelIndicatorValueEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}
impl SecurityhubInsightFiltersElThreatIntelIndicatorValueEl {}
impl ToListMappable for SecurityhubInsightFiltersElThreatIntelIndicatorValueEl {
    type O = BlockAssignable<SecurityhubInsightFiltersElThreatIntelIndicatorValueEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElThreatIntelIndicatorValueEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildSecurityhubInsightFiltersElThreatIntelIndicatorValueEl {
    pub fn build(self) -> SecurityhubInsightFiltersElThreatIntelIndicatorValueEl {
        SecurityhubInsightFiltersElThreatIntelIndicatorValueEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}
pub struct SecurityhubInsightFiltersElThreatIntelIndicatorValueElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElThreatIntelIndicatorValueElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubInsightFiltersElThreatIntelIndicatorValueElRef {
        SecurityhubInsightFiltersElThreatIntelIndicatorValueElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElThreatIntelIndicatorValueElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElTitleEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}
impl SecurityhubInsightFiltersElTitleEl {}
impl ToListMappable for SecurityhubInsightFiltersElTitleEl {
    type O = BlockAssignable<SecurityhubInsightFiltersElTitleEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElTitleEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildSecurityhubInsightFiltersElTitleEl {
    pub fn build(self) -> SecurityhubInsightFiltersElTitleEl {
        SecurityhubInsightFiltersElTitleEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}
pub struct SecurityhubInsightFiltersElTitleElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElTitleElRef {
    fn new(shared: StackShared, base: String) -> SecurityhubInsightFiltersElTitleElRef {
        SecurityhubInsightFiltersElTitleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElTitleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElTypeEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}
impl SecurityhubInsightFiltersElTypeEl {}
impl ToListMappable for SecurityhubInsightFiltersElTypeEl {
    type O = BlockAssignable<SecurityhubInsightFiltersElTypeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElTypeEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildSecurityhubInsightFiltersElTypeEl {
    pub fn build(self) -> SecurityhubInsightFiltersElTypeEl {
        SecurityhubInsightFiltersElTypeEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}
pub struct SecurityhubInsightFiltersElTypeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElTypeElRef {
    fn new(shared: StackShared, base: String) -> SecurityhubInsightFiltersElTypeElRef {
        SecurityhubInsightFiltersElTypeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElTypeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElUpdatedAtElDateRangeEl {
    unit: PrimField<String>,
    value: PrimField<f64>,
}
impl SecurityhubInsightFiltersElUpdatedAtElDateRangeEl {}
impl ToListMappable for SecurityhubInsightFiltersElUpdatedAtElDateRangeEl {
    type O = BlockAssignable<SecurityhubInsightFiltersElUpdatedAtElDateRangeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElUpdatedAtElDateRangeEl {
    #[doc = ""]
    pub unit: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<f64>,
}
impl BuildSecurityhubInsightFiltersElUpdatedAtElDateRangeEl {
    pub fn build(self) -> SecurityhubInsightFiltersElUpdatedAtElDateRangeEl {
        SecurityhubInsightFiltersElUpdatedAtElDateRangeEl {
            unit: self.unit,
            value: self.value,
        }
    }
}
pub struct SecurityhubInsightFiltersElUpdatedAtElDateRangeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElUpdatedAtElDateRangeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityhubInsightFiltersElUpdatedAtElDateRangeElRef {
        SecurityhubInsightFiltersElUpdatedAtElDateRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElUpdatedAtElDateRangeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize, Default)]
struct SecurityhubInsightFiltersElUpdatedAtElDynamic {
    date_range: Option<DynamicBlock<SecurityhubInsightFiltersElUpdatedAtElDateRangeEl>>,
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElUpdatedAtEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    end: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    date_range: Option<Vec<SecurityhubInsightFiltersElUpdatedAtElDateRangeEl>>,
    dynamic: SecurityhubInsightFiltersElUpdatedAtElDynamic,
}
impl SecurityhubInsightFiltersElUpdatedAtEl {
    #[doc = "Set the field `end`.\n"]
    pub fn set_end(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.end = Some(v.into());
        self
    }
    #[doc = "Set the field `start`.\n"]
    pub fn set_start(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.start = Some(v.into());
        self
    }
    #[doc = "Set the field `date_range`.\n"]
    pub fn set_date_range(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubInsightFiltersElUpdatedAtElDateRangeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.date_range = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.date_range = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for SecurityhubInsightFiltersElUpdatedAtEl {
    type O = BlockAssignable<SecurityhubInsightFiltersElUpdatedAtEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElUpdatedAtEl {}
impl BuildSecurityhubInsightFiltersElUpdatedAtEl {
    pub fn build(self) -> SecurityhubInsightFiltersElUpdatedAtEl {
        SecurityhubInsightFiltersElUpdatedAtEl {
            end: core::default::Default::default(),
            start: core::default::Default::default(),
            date_range: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct SecurityhubInsightFiltersElUpdatedAtElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElUpdatedAtElRef {
    fn new(shared: StackShared, base: String) -> SecurityhubInsightFiltersElUpdatedAtElRef {
        SecurityhubInsightFiltersElUpdatedAtElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElUpdatedAtElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `end` after provisioning.\n"]
    pub fn end(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.end", self.base))
    }
    #[doc = "Get a reference to the value of field `start` after provisioning.\n"]
    pub fn start(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start", self.base))
    }
    #[doc = "Get a reference to the value of field `date_range` after provisioning.\n"]
    pub fn date_range(&self) -> ListRef<SecurityhubInsightFiltersElUpdatedAtElDateRangeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.date_range", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElUserDefinedValuesEl {
    comparison: PrimField<String>,
    key: PrimField<String>,
    value: PrimField<String>,
}
impl SecurityhubInsightFiltersElUserDefinedValuesEl {}
impl ToListMappable for SecurityhubInsightFiltersElUserDefinedValuesEl {
    type O = BlockAssignable<SecurityhubInsightFiltersElUserDefinedValuesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElUserDefinedValuesEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub key: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildSecurityhubInsightFiltersElUserDefinedValuesEl {
    pub fn build(self) -> SecurityhubInsightFiltersElUserDefinedValuesEl {
        SecurityhubInsightFiltersElUserDefinedValuesEl {
            comparison: self.comparison,
            key: self.key,
            value: self.value,
        }
    }
}
pub struct SecurityhubInsightFiltersElUserDefinedValuesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElUserDefinedValuesElRef {
    fn new(shared: StackShared, base: String) -> SecurityhubInsightFiltersElUserDefinedValuesElRef {
        SecurityhubInsightFiltersElUserDefinedValuesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElUserDefinedValuesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElVerificationStateEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}
impl SecurityhubInsightFiltersElVerificationStateEl {}
impl ToListMappable for SecurityhubInsightFiltersElVerificationStateEl {
    type O = BlockAssignable<SecurityhubInsightFiltersElVerificationStateEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElVerificationStateEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildSecurityhubInsightFiltersElVerificationStateEl {
    pub fn build(self) -> SecurityhubInsightFiltersElVerificationStateEl {
        SecurityhubInsightFiltersElVerificationStateEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}
pub struct SecurityhubInsightFiltersElVerificationStateElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElVerificationStateElRef {
    fn new(shared: StackShared, base: String) -> SecurityhubInsightFiltersElVerificationStateElRef {
        SecurityhubInsightFiltersElVerificationStateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElVerificationStateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElWorkflowStatusEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}
impl SecurityhubInsightFiltersElWorkflowStatusEl {}
impl ToListMappable for SecurityhubInsightFiltersElWorkflowStatusEl {
    type O = BlockAssignable<SecurityhubInsightFiltersElWorkflowStatusEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersElWorkflowStatusEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildSecurityhubInsightFiltersElWorkflowStatusEl {
    pub fn build(self) -> SecurityhubInsightFiltersElWorkflowStatusEl {
        SecurityhubInsightFiltersElWorkflowStatusEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}
pub struct SecurityhubInsightFiltersElWorkflowStatusElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElWorkflowStatusElRef {
    fn new(shared: StackShared, base: String) -> SecurityhubInsightFiltersElWorkflowStatusElRef {
        SecurityhubInsightFiltersElWorkflowStatusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElWorkflowStatusElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize, Default)]
struct SecurityhubInsightFiltersElDynamic {
    aws_account_id: Option<DynamicBlock<SecurityhubInsightFiltersElAwsAccountIdEl>>,
    company_name: Option<DynamicBlock<SecurityhubInsightFiltersElCompanyNameEl>>,
    compliance_status: Option<DynamicBlock<SecurityhubInsightFiltersElComplianceStatusEl>>,
    confidence: Option<DynamicBlock<SecurityhubInsightFiltersElConfidenceEl>>,
    created_at: Option<DynamicBlock<SecurityhubInsightFiltersElCreatedAtEl>>,
    criticality: Option<DynamicBlock<SecurityhubInsightFiltersElCriticalityEl>>,
    description: Option<DynamicBlock<SecurityhubInsightFiltersElDescriptionEl>>,
    finding_provider_fields_confidence:
        Option<DynamicBlock<SecurityhubInsightFiltersElFindingProviderFieldsConfidenceEl>>,
    finding_provider_fields_criticality:
        Option<DynamicBlock<SecurityhubInsightFiltersElFindingProviderFieldsCriticalityEl>>,
    finding_provider_fields_related_findings_id:
        Option<DynamicBlock<SecurityhubInsightFiltersElFindingProviderFieldsRelatedFindingsIdEl>>,
    finding_provider_fields_related_findings_product_arn: Option<
        DynamicBlock<SecurityhubInsightFiltersElFindingProviderFieldsRelatedFindingsProductArnEl>,
    >,
    finding_provider_fields_severity_label:
        Option<DynamicBlock<SecurityhubInsightFiltersElFindingProviderFieldsSeverityLabelEl>>,
    finding_provider_fields_severity_original:
        Option<DynamicBlock<SecurityhubInsightFiltersElFindingProviderFieldsSeverityOriginalEl>>,
    finding_provider_fields_types:
        Option<DynamicBlock<SecurityhubInsightFiltersElFindingProviderFieldsTypesEl>>,
    first_observed_at: Option<DynamicBlock<SecurityhubInsightFiltersElFirstObservedAtEl>>,
    generator_id: Option<DynamicBlock<SecurityhubInsightFiltersElGeneratorIdEl>>,
    id: Option<DynamicBlock<SecurityhubInsightFiltersElIdEl>>,
    keyword: Option<DynamicBlock<SecurityhubInsightFiltersElKeywordEl>>,
    last_observed_at: Option<DynamicBlock<SecurityhubInsightFiltersElLastObservedAtEl>>,
    malware_name: Option<DynamicBlock<SecurityhubInsightFiltersElMalwareNameEl>>,
    malware_path: Option<DynamicBlock<SecurityhubInsightFiltersElMalwarePathEl>>,
    malware_state: Option<DynamicBlock<SecurityhubInsightFiltersElMalwareStateEl>>,
    malware_type: Option<DynamicBlock<SecurityhubInsightFiltersElMalwareTypeEl>>,
    network_destination_domain:
        Option<DynamicBlock<SecurityhubInsightFiltersElNetworkDestinationDomainEl>>,
    network_destination_ipv4:
        Option<DynamicBlock<SecurityhubInsightFiltersElNetworkDestinationIpv4El>>,
    network_destination_ipv6:
        Option<DynamicBlock<SecurityhubInsightFiltersElNetworkDestinationIpv6El>>,
    network_destination_port:
        Option<DynamicBlock<SecurityhubInsightFiltersElNetworkDestinationPortEl>>,
    network_direction: Option<DynamicBlock<SecurityhubInsightFiltersElNetworkDirectionEl>>,
    network_protocol: Option<DynamicBlock<SecurityhubInsightFiltersElNetworkProtocolEl>>,
    network_source_domain: Option<DynamicBlock<SecurityhubInsightFiltersElNetworkSourceDomainEl>>,
    network_source_ipv4: Option<DynamicBlock<SecurityhubInsightFiltersElNetworkSourceIpv4El>>,
    network_source_ipv6: Option<DynamicBlock<SecurityhubInsightFiltersElNetworkSourceIpv6El>>,
    network_source_mac: Option<DynamicBlock<SecurityhubInsightFiltersElNetworkSourceMacEl>>,
    network_source_port: Option<DynamicBlock<SecurityhubInsightFiltersElNetworkSourcePortEl>>,
    note_text: Option<DynamicBlock<SecurityhubInsightFiltersElNoteTextEl>>,
    note_updated_at: Option<DynamicBlock<SecurityhubInsightFiltersElNoteUpdatedAtEl>>,
    note_updated_by: Option<DynamicBlock<SecurityhubInsightFiltersElNoteUpdatedByEl>>,
    process_launched_at: Option<DynamicBlock<SecurityhubInsightFiltersElProcessLaunchedAtEl>>,
    process_name: Option<DynamicBlock<SecurityhubInsightFiltersElProcessNameEl>>,
    process_parent_pid: Option<DynamicBlock<SecurityhubInsightFiltersElProcessParentPidEl>>,
    process_path: Option<DynamicBlock<SecurityhubInsightFiltersElProcessPathEl>>,
    process_pid: Option<DynamicBlock<SecurityhubInsightFiltersElProcessPidEl>>,
    process_terminated_at: Option<DynamicBlock<SecurityhubInsightFiltersElProcessTerminatedAtEl>>,
    product_arn: Option<DynamicBlock<SecurityhubInsightFiltersElProductArnEl>>,
    product_fields: Option<DynamicBlock<SecurityhubInsightFiltersElProductFieldsEl>>,
    product_name: Option<DynamicBlock<SecurityhubInsightFiltersElProductNameEl>>,
    recommendation_text: Option<DynamicBlock<SecurityhubInsightFiltersElRecommendationTextEl>>,
    record_state: Option<DynamicBlock<SecurityhubInsightFiltersElRecordStateEl>>,
    related_findings_id: Option<DynamicBlock<SecurityhubInsightFiltersElRelatedFindingsIdEl>>,
    related_findings_product_arn:
        Option<DynamicBlock<SecurityhubInsightFiltersElRelatedFindingsProductArnEl>>,
    resource_aws_ec2_instance_iam_instance_profile_arn: Option<
        DynamicBlock<SecurityhubInsightFiltersElResourceAwsEc2InstanceIamInstanceProfileArnEl>,
    >,
    resource_aws_ec2_instance_image_id:
        Option<DynamicBlock<SecurityhubInsightFiltersElResourceAwsEc2InstanceImageIdEl>>,
    resource_aws_ec2_instance_ipv4_addresses:
        Option<DynamicBlock<SecurityhubInsightFiltersElResourceAwsEc2InstanceIpv4AddressesEl>>,
    resource_aws_ec2_instance_ipv6_addresses:
        Option<DynamicBlock<SecurityhubInsightFiltersElResourceAwsEc2InstanceIpv6AddressesEl>>,
    resource_aws_ec2_instance_key_name:
        Option<DynamicBlock<SecurityhubInsightFiltersElResourceAwsEc2InstanceKeyNameEl>>,
    resource_aws_ec2_instance_launched_at:
        Option<DynamicBlock<SecurityhubInsightFiltersElResourceAwsEc2InstanceLaunchedAtEl>>,
    resource_aws_ec2_instance_subnet_id:
        Option<DynamicBlock<SecurityhubInsightFiltersElResourceAwsEc2InstanceSubnetIdEl>>,
    resource_aws_ec2_instance_type:
        Option<DynamicBlock<SecurityhubInsightFiltersElResourceAwsEc2InstanceTypeEl>>,
    resource_aws_ec2_instance_vpc_id:
        Option<DynamicBlock<SecurityhubInsightFiltersElResourceAwsEc2InstanceVpcIdEl>>,
    resource_aws_iam_access_key_created_at:
        Option<DynamicBlock<SecurityhubInsightFiltersElResourceAwsIamAccessKeyCreatedAtEl>>,
    resource_aws_iam_access_key_status:
        Option<DynamicBlock<SecurityhubInsightFiltersElResourceAwsIamAccessKeyStatusEl>>,
    resource_aws_iam_access_key_user_name:
        Option<DynamicBlock<SecurityhubInsightFiltersElResourceAwsIamAccessKeyUserNameEl>>,
    resource_aws_s3_bucket_owner_id:
        Option<DynamicBlock<SecurityhubInsightFiltersElResourceAwsS3BucketOwnerIdEl>>,
    resource_aws_s3_bucket_owner_name:
        Option<DynamicBlock<SecurityhubInsightFiltersElResourceAwsS3BucketOwnerNameEl>>,
    resource_container_image_id:
        Option<DynamicBlock<SecurityhubInsightFiltersElResourceContainerImageIdEl>>,
    resource_container_image_name:
        Option<DynamicBlock<SecurityhubInsightFiltersElResourceContainerImageNameEl>>,
    resource_container_launched_at:
        Option<DynamicBlock<SecurityhubInsightFiltersElResourceContainerLaunchedAtEl>>,
    resource_container_name:
        Option<DynamicBlock<SecurityhubInsightFiltersElResourceContainerNameEl>>,
    resource_details_other: Option<DynamicBlock<SecurityhubInsightFiltersElResourceDetailsOtherEl>>,
    resource_id: Option<DynamicBlock<SecurityhubInsightFiltersElResourceIdEl>>,
    resource_partition: Option<DynamicBlock<SecurityhubInsightFiltersElResourcePartitionEl>>,
    resource_region: Option<DynamicBlock<SecurityhubInsightFiltersElResourceRegionEl>>,
    resource_tags: Option<DynamicBlock<SecurityhubInsightFiltersElResourceTagsEl>>,
    resource_type: Option<DynamicBlock<SecurityhubInsightFiltersElResourceTypeEl>>,
    severity_label: Option<DynamicBlock<SecurityhubInsightFiltersElSeverityLabelEl>>,
    source_url: Option<DynamicBlock<SecurityhubInsightFiltersElSourceUrlEl>>,
    threat_intel_indicator_category:
        Option<DynamicBlock<SecurityhubInsightFiltersElThreatIntelIndicatorCategoryEl>>,
    threat_intel_indicator_last_observed_at:
        Option<DynamicBlock<SecurityhubInsightFiltersElThreatIntelIndicatorLastObservedAtEl>>,
    threat_intel_indicator_source:
        Option<DynamicBlock<SecurityhubInsightFiltersElThreatIntelIndicatorSourceEl>>,
    threat_intel_indicator_source_url:
        Option<DynamicBlock<SecurityhubInsightFiltersElThreatIntelIndicatorSourceUrlEl>>,
    threat_intel_indicator_type:
        Option<DynamicBlock<SecurityhubInsightFiltersElThreatIntelIndicatorTypeEl>>,
    threat_intel_indicator_value:
        Option<DynamicBlock<SecurityhubInsightFiltersElThreatIntelIndicatorValueEl>>,
    title: Option<DynamicBlock<SecurityhubInsightFiltersElTitleEl>>,
    type_: Option<DynamicBlock<SecurityhubInsightFiltersElTypeEl>>,
    updated_at: Option<DynamicBlock<SecurityhubInsightFiltersElUpdatedAtEl>>,
    user_defined_values: Option<DynamicBlock<SecurityhubInsightFiltersElUserDefinedValuesEl>>,
    verification_state: Option<DynamicBlock<SecurityhubInsightFiltersElVerificationStateEl>>,
    workflow_status: Option<DynamicBlock<SecurityhubInsightFiltersElWorkflowStatusEl>>,
}
#[derive(Serialize)]
pub struct SecurityhubInsightFiltersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    aws_account_id: Option<Vec<SecurityhubInsightFiltersElAwsAccountIdEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    company_name: Option<Vec<SecurityhubInsightFiltersElCompanyNameEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    compliance_status: Option<Vec<SecurityhubInsightFiltersElComplianceStatusEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    confidence: Option<Vec<SecurityhubInsightFiltersElConfidenceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    created_at: Option<Vec<SecurityhubInsightFiltersElCreatedAtEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    criticality: Option<Vec<SecurityhubInsightFiltersElCriticalityEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<Vec<SecurityhubInsightFiltersElDescriptionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    finding_provider_fields_confidence:
        Option<Vec<SecurityhubInsightFiltersElFindingProviderFieldsConfidenceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    finding_provider_fields_criticality:
        Option<Vec<SecurityhubInsightFiltersElFindingProviderFieldsCriticalityEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    finding_provider_fields_related_findings_id:
        Option<Vec<SecurityhubInsightFiltersElFindingProviderFieldsRelatedFindingsIdEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    finding_provider_fields_related_findings_product_arn:
        Option<Vec<SecurityhubInsightFiltersElFindingProviderFieldsRelatedFindingsProductArnEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    finding_provider_fields_severity_label:
        Option<Vec<SecurityhubInsightFiltersElFindingProviderFieldsSeverityLabelEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    finding_provider_fields_severity_original:
        Option<Vec<SecurityhubInsightFiltersElFindingProviderFieldsSeverityOriginalEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    finding_provider_fields_types:
        Option<Vec<SecurityhubInsightFiltersElFindingProviderFieldsTypesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    first_observed_at: Option<Vec<SecurityhubInsightFiltersElFirstObservedAtEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    generator_id: Option<Vec<SecurityhubInsightFiltersElGeneratorIdEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<Vec<SecurityhubInsightFiltersElIdEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    keyword: Option<Vec<SecurityhubInsightFiltersElKeywordEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_observed_at: Option<Vec<SecurityhubInsightFiltersElLastObservedAtEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    malware_name: Option<Vec<SecurityhubInsightFiltersElMalwareNameEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    malware_path: Option<Vec<SecurityhubInsightFiltersElMalwarePathEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    malware_state: Option<Vec<SecurityhubInsightFiltersElMalwareStateEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    malware_type: Option<Vec<SecurityhubInsightFiltersElMalwareTypeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_destination_domain: Option<Vec<SecurityhubInsightFiltersElNetworkDestinationDomainEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_destination_ipv4: Option<Vec<SecurityhubInsightFiltersElNetworkDestinationIpv4El>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_destination_ipv6: Option<Vec<SecurityhubInsightFiltersElNetworkDestinationIpv6El>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_destination_port: Option<Vec<SecurityhubInsightFiltersElNetworkDestinationPortEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_direction: Option<Vec<SecurityhubInsightFiltersElNetworkDirectionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_protocol: Option<Vec<SecurityhubInsightFiltersElNetworkProtocolEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_source_domain: Option<Vec<SecurityhubInsightFiltersElNetworkSourceDomainEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_source_ipv4: Option<Vec<SecurityhubInsightFiltersElNetworkSourceIpv4El>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_source_ipv6: Option<Vec<SecurityhubInsightFiltersElNetworkSourceIpv6El>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_source_mac: Option<Vec<SecurityhubInsightFiltersElNetworkSourceMacEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_source_port: Option<Vec<SecurityhubInsightFiltersElNetworkSourcePortEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    note_text: Option<Vec<SecurityhubInsightFiltersElNoteTextEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    note_updated_at: Option<Vec<SecurityhubInsightFiltersElNoteUpdatedAtEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    note_updated_by: Option<Vec<SecurityhubInsightFiltersElNoteUpdatedByEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    process_launched_at: Option<Vec<SecurityhubInsightFiltersElProcessLaunchedAtEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    process_name: Option<Vec<SecurityhubInsightFiltersElProcessNameEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    process_parent_pid: Option<Vec<SecurityhubInsightFiltersElProcessParentPidEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    process_path: Option<Vec<SecurityhubInsightFiltersElProcessPathEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    process_pid: Option<Vec<SecurityhubInsightFiltersElProcessPidEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    process_terminated_at: Option<Vec<SecurityhubInsightFiltersElProcessTerminatedAtEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    product_arn: Option<Vec<SecurityhubInsightFiltersElProductArnEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    product_fields: Option<Vec<SecurityhubInsightFiltersElProductFieldsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    product_name: Option<Vec<SecurityhubInsightFiltersElProductNameEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recommendation_text: Option<Vec<SecurityhubInsightFiltersElRecommendationTextEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    record_state: Option<Vec<SecurityhubInsightFiltersElRecordStateEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    related_findings_id: Option<Vec<SecurityhubInsightFiltersElRelatedFindingsIdEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    related_findings_product_arn:
        Option<Vec<SecurityhubInsightFiltersElRelatedFindingsProductArnEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_aws_ec2_instance_iam_instance_profile_arn:
        Option<Vec<SecurityhubInsightFiltersElResourceAwsEc2InstanceIamInstanceProfileArnEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_aws_ec2_instance_image_id:
        Option<Vec<SecurityhubInsightFiltersElResourceAwsEc2InstanceImageIdEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_aws_ec2_instance_ipv4_addresses:
        Option<Vec<SecurityhubInsightFiltersElResourceAwsEc2InstanceIpv4AddressesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_aws_ec2_instance_ipv6_addresses:
        Option<Vec<SecurityhubInsightFiltersElResourceAwsEc2InstanceIpv6AddressesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_aws_ec2_instance_key_name:
        Option<Vec<SecurityhubInsightFiltersElResourceAwsEc2InstanceKeyNameEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_aws_ec2_instance_launched_at:
        Option<Vec<SecurityhubInsightFiltersElResourceAwsEc2InstanceLaunchedAtEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_aws_ec2_instance_subnet_id:
        Option<Vec<SecurityhubInsightFiltersElResourceAwsEc2InstanceSubnetIdEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_aws_ec2_instance_type:
        Option<Vec<SecurityhubInsightFiltersElResourceAwsEc2InstanceTypeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_aws_ec2_instance_vpc_id:
        Option<Vec<SecurityhubInsightFiltersElResourceAwsEc2InstanceVpcIdEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_aws_iam_access_key_created_at:
        Option<Vec<SecurityhubInsightFiltersElResourceAwsIamAccessKeyCreatedAtEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_aws_iam_access_key_status:
        Option<Vec<SecurityhubInsightFiltersElResourceAwsIamAccessKeyStatusEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_aws_iam_access_key_user_name:
        Option<Vec<SecurityhubInsightFiltersElResourceAwsIamAccessKeyUserNameEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_aws_s3_bucket_owner_id:
        Option<Vec<SecurityhubInsightFiltersElResourceAwsS3BucketOwnerIdEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_aws_s3_bucket_owner_name:
        Option<Vec<SecurityhubInsightFiltersElResourceAwsS3BucketOwnerNameEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_container_image_id: Option<Vec<SecurityhubInsightFiltersElResourceContainerImageIdEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_container_image_name:
        Option<Vec<SecurityhubInsightFiltersElResourceContainerImageNameEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_container_launched_at:
        Option<Vec<SecurityhubInsightFiltersElResourceContainerLaunchedAtEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_container_name: Option<Vec<SecurityhubInsightFiltersElResourceContainerNameEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_details_other: Option<Vec<SecurityhubInsightFiltersElResourceDetailsOtherEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_id: Option<Vec<SecurityhubInsightFiltersElResourceIdEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_partition: Option<Vec<SecurityhubInsightFiltersElResourcePartitionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_region: Option<Vec<SecurityhubInsightFiltersElResourceRegionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_tags: Option<Vec<SecurityhubInsightFiltersElResourceTagsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_type: Option<Vec<SecurityhubInsightFiltersElResourceTypeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    severity_label: Option<Vec<SecurityhubInsightFiltersElSeverityLabelEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_url: Option<Vec<SecurityhubInsightFiltersElSourceUrlEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    threat_intel_indicator_category:
        Option<Vec<SecurityhubInsightFiltersElThreatIntelIndicatorCategoryEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    threat_intel_indicator_last_observed_at:
        Option<Vec<SecurityhubInsightFiltersElThreatIntelIndicatorLastObservedAtEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    threat_intel_indicator_source:
        Option<Vec<SecurityhubInsightFiltersElThreatIntelIndicatorSourceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    threat_intel_indicator_source_url:
        Option<Vec<SecurityhubInsightFiltersElThreatIntelIndicatorSourceUrlEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    threat_intel_indicator_type: Option<Vec<SecurityhubInsightFiltersElThreatIntelIndicatorTypeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    threat_intel_indicator_value:
        Option<Vec<SecurityhubInsightFiltersElThreatIntelIndicatorValueEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<Vec<SecurityhubInsightFiltersElTitleEl>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<Vec<SecurityhubInsightFiltersElTypeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    updated_at: Option<Vec<SecurityhubInsightFiltersElUpdatedAtEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_defined_values: Option<Vec<SecurityhubInsightFiltersElUserDefinedValuesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    verification_state: Option<Vec<SecurityhubInsightFiltersElVerificationStateEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    workflow_status: Option<Vec<SecurityhubInsightFiltersElWorkflowStatusEl>>,
    dynamic: SecurityhubInsightFiltersElDynamic,
}
impl SecurityhubInsightFiltersEl {
    #[doc = "Set the field `aws_account_id`.\n"]
    pub fn set_aws_account_id(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubInsightFiltersElAwsAccountIdEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.aws_account_id = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.aws_account_id = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `company_name`.\n"]
    pub fn set_company_name(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubInsightFiltersElCompanyNameEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.company_name = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.company_name = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `compliance_status`.\n"]
    pub fn set_compliance_status(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubInsightFiltersElComplianceStatusEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.compliance_status = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.compliance_status = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `confidence`.\n"]
    pub fn set_confidence(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubInsightFiltersElConfidenceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.confidence = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.confidence = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `created_at`.\n"]
    pub fn set_created_at(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubInsightFiltersElCreatedAtEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.created_at = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.created_at = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `criticality`.\n"]
    pub fn set_criticality(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubInsightFiltersElCriticalityEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.criticality = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.criticality = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubInsightFiltersElDescriptionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.description = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.description = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `finding_provider_fields_confidence`.\n"]
    pub fn set_finding_provider_fields_confidence(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubInsightFiltersElFindingProviderFieldsConfidenceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.finding_provider_fields_confidence = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.finding_provider_fields_confidence = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `finding_provider_fields_criticality`.\n"]
    pub fn set_finding_provider_fields_criticality(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubInsightFiltersElFindingProviderFieldsCriticalityEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.finding_provider_fields_criticality = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.finding_provider_fields_criticality = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `finding_provider_fields_related_findings_id`.\n"]
    pub fn set_finding_provider_fields_related_findings_id(
        mut self,
        v: impl Into<
            BlockAssignable<SecurityhubInsightFiltersElFindingProviderFieldsRelatedFindingsIdEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.finding_provider_fields_related_findings_id = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.finding_provider_fields_related_findings_id = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `finding_provider_fields_related_findings_product_arn`.\n"]
    pub fn set_finding_provider_fields_related_findings_product_arn(
        mut self,
        v: impl Into<
            BlockAssignable<
                SecurityhubInsightFiltersElFindingProviderFieldsRelatedFindingsProductArnEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.finding_provider_fields_related_findings_product_arn = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic
                    .finding_provider_fields_related_findings_product_arn = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `finding_provider_fields_severity_label`.\n"]
    pub fn set_finding_provider_fields_severity_label(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubInsightFiltersElFindingProviderFieldsSeverityLabelEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.finding_provider_fields_severity_label = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.finding_provider_fields_severity_label = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `finding_provider_fields_severity_original`.\n"]
    pub fn set_finding_provider_fields_severity_original(
        mut self,
        v: impl Into<
            BlockAssignable<SecurityhubInsightFiltersElFindingProviderFieldsSeverityOriginalEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.finding_provider_fields_severity_original = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.finding_provider_fields_severity_original = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `finding_provider_fields_types`.\n"]
    pub fn set_finding_provider_fields_types(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubInsightFiltersElFindingProviderFieldsTypesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.finding_provider_fields_types = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.finding_provider_fields_types = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `first_observed_at`.\n"]
    pub fn set_first_observed_at(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubInsightFiltersElFirstObservedAtEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.first_observed_at = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.first_observed_at = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `generator_id`.\n"]
    pub fn set_generator_id(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubInsightFiltersElGeneratorIdEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.generator_id = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.generator_id = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubInsightFiltersElIdEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.id = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.id = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `keyword`.\n"]
    pub fn set_keyword(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubInsightFiltersElKeywordEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.keyword = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.keyword = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `last_observed_at`.\n"]
    pub fn set_last_observed_at(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubInsightFiltersElLastObservedAtEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.last_observed_at = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.last_observed_at = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `malware_name`.\n"]
    pub fn set_malware_name(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubInsightFiltersElMalwareNameEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.malware_name = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.malware_name = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `malware_path`.\n"]
    pub fn set_malware_path(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubInsightFiltersElMalwarePathEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.malware_path = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.malware_path = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `malware_state`.\n"]
    pub fn set_malware_state(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubInsightFiltersElMalwareStateEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.malware_state = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.malware_state = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `malware_type`.\n"]
    pub fn set_malware_type(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubInsightFiltersElMalwareTypeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.malware_type = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.malware_type = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `network_destination_domain`.\n"]
    pub fn set_network_destination_domain(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubInsightFiltersElNetworkDestinationDomainEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.network_destination_domain = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.network_destination_domain = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `network_destination_ipv4`.\n"]
    pub fn set_network_destination_ipv4(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubInsightFiltersElNetworkDestinationIpv4El>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.network_destination_ipv4 = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.network_destination_ipv4 = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `network_destination_ipv6`.\n"]
    pub fn set_network_destination_ipv6(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubInsightFiltersElNetworkDestinationIpv6El>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.network_destination_ipv6 = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.network_destination_ipv6 = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `network_destination_port`.\n"]
    pub fn set_network_destination_port(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubInsightFiltersElNetworkDestinationPortEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.network_destination_port = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.network_destination_port = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `network_direction`.\n"]
    pub fn set_network_direction(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubInsightFiltersElNetworkDirectionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.network_direction = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.network_direction = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `network_protocol`.\n"]
    pub fn set_network_protocol(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubInsightFiltersElNetworkProtocolEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.network_protocol = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.network_protocol = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `network_source_domain`.\n"]
    pub fn set_network_source_domain(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubInsightFiltersElNetworkSourceDomainEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.network_source_domain = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.network_source_domain = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `network_source_ipv4`.\n"]
    pub fn set_network_source_ipv4(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubInsightFiltersElNetworkSourceIpv4El>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.network_source_ipv4 = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.network_source_ipv4 = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `network_source_ipv6`.\n"]
    pub fn set_network_source_ipv6(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubInsightFiltersElNetworkSourceIpv6El>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.network_source_ipv6 = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.network_source_ipv6 = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `network_source_mac`.\n"]
    pub fn set_network_source_mac(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubInsightFiltersElNetworkSourceMacEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.network_source_mac = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.network_source_mac = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `network_source_port`.\n"]
    pub fn set_network_source_port(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubInsightFiltersElNetworkSourcePortEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.network_source_port = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.network_source_port = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `note_text`.\n"]
    pub fn set_note_text(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubInsightFiltersElNoteTextEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.note_text = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.note_text = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `note_updated_at`.\n"]
    pub fn set_note_updated_at(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubInsightFiltersElNoteUpdatedAtEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.note_updated_at = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.note_updated_at = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `note_updated_by`.\n"]
    pub fn set_note_updated_by(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubInsightFiltersElNoteUpdatedByEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.note_updated_by = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.note_updated_by = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `process_launched_at`.\n"]
    pub fn set_process_launched_at(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubInsightFiltersElProcessLaunchedAtEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.process_launched_at = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.process_launched_at = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `process_name`.\n"]
    pub fn set_process_name(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubInsightFiltersElProcessNameEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.process_name = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.process_name = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `process_parent_pid`.\n"]
    pub fn set_process_parent_pid(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubInsightFiltersElProcessParentPidEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.process_parent_pid = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.process_parent_pid = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `process_path`.\n"]
    pub fn set_process_path(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubInsightFiltersElProcessPathEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.process_path = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.process_path = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `process_pid`.\n"]
    pub fn set_process_pid(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubInsightFiltersElProcessPidEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.process_pid = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.process_pid = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `process_terminated_at`.\n"]
    pub fn set_process_terminated_at(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubInsightFiltersElProcessTerminatedAtEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.process_terminated_at = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.process_terminated_at = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `product_arn`.\n"]
    pub fn set_product_arn(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubInsightFiltersElProductArnEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.product_arn = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.product_arn = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `product_fields`.\n"]
    pub fn set_product_fields(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubInsightFiltersElProductFieldsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.product_fields = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.product_fields = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `product_name`.\n"]
    pub fn set_product_name(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubInsightFiltersElProductNameEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.product_name = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.product_name = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `recommendation_text`.\n"]
    pub fn set_recommendation_text(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubInsightFiltersElRecommendationTextEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.recommendation_text = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.recommendation_text = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `record_state`.\n"]
    pub fn set_record_state(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubInsightFiltersElRecordStateEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.record_state = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.record_state = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `related_findings_id`.\n"]
    pub fn set_related_findings_id(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubInsightFiltersElRelatedFindingsIdEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.related_findings_id = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.related_findings_id = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `related_findings_product_arn`.\n"]
    pub fn set_related_findings_product_arn(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubInsightFiltersElRelatedFindingsProductArnEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.related_findings_product_arn = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.related_findings_product_arn = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `resource_aws_ec2_instance_iam_instance_profile_arn`.\n"]
    pub fn set_resource_aws_ec2_instance_iam_instance_profile_arn(
        mut self,
        v: impl Into<
            BlockAssignable<
                SecurityhubInsightFiltersElResourceAwsEc2InstanceIamInstanceProfileArnEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.resource_aws_ec2_instance_iam_instance_profile_arn = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic
                    .resource_aws_ec2_instance_iam_instance_profile_arn = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `resource_aws_ec2_instance_image_id`.\n"]
    pub fn set_resource_aws_ec2_instance_image_id(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubInsightFiltersElResourceAwsEc2InstanceImageIdEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.resource_aws_ec2_instance_image_id = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_aws_ec2_instance_image_id = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `resource_aws_ec2_instance_ipv4_addresses`.\n"]
    pub fn set_resource_aws_ec2_instance_ipv4_addresses(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubInsightFiltersElResourceAwsEc2InstanceIpv4AddressesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.resource_aws_ec2_instance_ipv4_addresses = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_aws_ec2_instance_ipv4_addresses = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `resource_aws_ec2_instance_ipv6_addresses`.\n"]
    pub fn set_resource_aws_ec2_instance_ipv6_addresses(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubInsightFiltersElResourceAwsEc2InstanceIpv6AddressesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.resource_aws_ec2_instance_ipv6_addresses = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_aws_ec2_instance_ipv6_addresses = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `resource_aws_ec2_instance_key_name`.\n"]
    pub fn set_resource_aws_ec2_instance_key_name(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubInsightFiltersElResourceAwsEc2InstanceKeyNameEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.resource_aws_ec2_instance_key_name = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_aws_ec2_instance_key_name = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `resource_aws_ec2_instance_launched_at`.\n"]
    pub fn set_resource_aws_ec2_instance_launched_at(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubInsightFiltersElResourceAwsEc2InstanceLaunchedAtEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.resource_aws_ec2_instance_launched_at = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_aws_ec2_instance_launched_at = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `resource_aws_ec2_instance_subnet_id`.\n"]
    pub fn set_resource_aws_ec2_instance_subnet_id(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubInsightFiltersElResourceAwsEc2InstanceSubnetIdEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.resource_aws_ec2_instance_subnet_id = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_aws_ec2_instance_subnet_id = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `resource_aws_ec2_instance_type`.\n"]
    pub fn set_resource_aws_ec2_instance_type(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubInsightFiltersElResourceAwsEc2InstanceTypeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.resource_aws_ec2_instance_type = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_aws_ec2_instance_type = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `resource_aws_ec2_instance_vpc_id`.\n"]
    pub fn set_resource_aws_ec2_instance_vpc_id(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubInsightFiltersElResourceAwsEc2InstanceVpcIdEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.resource_aws_ec2_instance_vpc_id = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_aws_ec2_instance_vpc_id = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `resource_aws_iam_access_key_created_at`.\n"]
    pub fn set_resource_aws_iam_access_key_created_at(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubInsightFiltersElResourceAwsIamAccessKeyCreatedAtEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.resource_aws_iam_access_key_created_at = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_aws_iam_access_key_created_at = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `resource_aws_iam_access_key_status`.\n"]
    pub fn set_resource_aws_iam_access_key_status(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubInsightFiltersElResourceAwsIamAccessKeyStatusEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.resource_aws_iam_access_key_status = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_aws_iam_access_key_status = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `resource_aws_iam_access_key_user_name`.\n"]
    pub fn set_resource_aws_iam_access_key_user_name(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubInsightFiltersElResourceAwsIamAccessKeyUserNameEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.resource_aws_iam_access_key_user_name = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_aws_iam_access_key_user_name = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `resource_aws_s3_bucket_owner_id`.\n"]
    pub fn set_resource_aws_s3_bucket_owner_id(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubInsightFiltersElResourceAwsS3BucketOwnerIdEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.resource_aws_s3_bucket_owner_id = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_aws_s3_bucket_owner_id = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `resource_aws_s3_bucket_owner_name`.\n"]
    pub fn set_resource_aws_s3_bucket_owner_name(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubInsightFiltersElResourceAwsS3BucketOwnerNameEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.resource_aws_s3_bucket_owner_name = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_aws_s3_bucket_owner_name = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `resource_container_image_id`.\n"]
    pub fn set_resource_container_image_id(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubInsightFiltersElResourceContainerImageIdEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.resource_container_image_id = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_container_image_id = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `resource_container_image_name`.\n"]
    pub fn set_resource_container_image_name(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubInsightFiltersElResourceContainerImageNameEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.resource_container_image_name = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_container_image_name = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `resource_container_launched_at`.\n"]
    pub fn set_resource_container_launched_at(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubInsightFiltersElResourceContainerLaunchedAtEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.resource_container_launched_at = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_container_launched_at = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `resource_container_name`.\n"]
    pub fn set_resource_container_name(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubInsightFiltersElResourceContainerNameEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.resource_container_name = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_container_name = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `resource_details_other`.\n"]
    pub fn set_resource_details_other(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubInsightFiltersElResourceDetailsOtherEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.resource_details_other = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_details_other = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `resource_id`.\n"]
    pub fn set_resource_id(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubInsightFiltersElResourceIdEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.resource_id = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_id = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `resource_partition`.\n"]
    pub fn set_resource_partition(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubInsightFiltersElResourcePartitionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.resource_partition = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_partition = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `resource_region`.\n"]
    pub fn set_resource_region(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubInsightFiltersElResourceRegionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.resource_region = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_region = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `resource_tags`.\n"]
    pub fn set_resource_tags(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubInsightFiltersElResourceTagsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.resource_tags = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_tags = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `resource_type`.\n"]
    pub fn set_resource_type(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubInsightFiltersElResourceTypeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.resource_type = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_type = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `severity_label`.\n"]
    pub fn set_severity_label(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubInsightFiltersElSeverityLabelEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.severity_label = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.severity_label = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `source_url`.\n"]
    pub fn set_source_url(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubInsightFiltersElSourceUrlEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.source_url = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.source_url = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `threat_intel_indicator_category`.\n"]
    pub fn set_threat_intel_indicator_category(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubInsightFiltersElThreatIntelIndicatorCategoryEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.threat_intel_indicator_category = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.threat_intel_indicator_category = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `threat_intel_indicator_last_observed_at`.\n"]
    pub fn set_threat_intel_indicator_last_observed_at(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubInsightFiltersElThreatIntelIndicatorLastObservedAtEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.threat_intel_indicator_last_observed_at = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.threat_intel_indicator_last_observed_at = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `threat_intel_indicator_source`.\n"]
    pub fn set_threat_intel_indicator_source(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubInsightFiltersElThreatIntelIndicatorSourceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.threat_intel_indicator_source = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.threat_intel_indicator_source = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `threat_intel_indicator_source_url`.\n"]
    pub fn set_threat_intel_indicator_source_url(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubInsightFiltersElThreatIntelIndicatorSourceUrlEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.threat_intel_indicator_source_url = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.threat_intel_indicator_source_url = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `threat_intel_indicator_type`.\n"]
    pub fn set_threat_intel_indicator_type(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubInsightFiltersElThreatIntelIndicatorTypeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.threat_intel_indicator_type = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.threat_intel_indicator_type = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `threat_intel_indicator_value`.\n"]
    pub fn set_threat_intel_indicator_value(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubInsightFiltersElThreatIntelIndicatorValueEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.threat_intel_indicator_value = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.threat_intel_indicator_value = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `title`.\n"]
    pub fn set_title(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubInsightFiltersElTitleEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.title = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.title = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubInsightFiltersElTypeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.type_ = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.type_ = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `updated_at`.\n"]
    pub fn set_updated_at(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubInsightFiltersElUpdatedAtEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.updated_at = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.updated_at = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `user_defined_values`.\n"]
    pub fn set_user_defined_values(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubInsightFiltersElUserDefinedValuesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.user_defined_values = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.user_defined_values = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `verification_state`.\n"]
    pub fn set_verification_state(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubInsightFiltersElVerificationStateEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.verification_state = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.verification_state = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `workflow_status`.\n"]
    pub fn set_workflow_status(
        mut self,
        v: impl Into<BlockAssignable<SecurityhubInsightFiltersElWorkflowStatusEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.workflow_status = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.workflow_status = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for SecurityhubInsightFiltersEl {
    type O = BlockAssignable<SecurityhubInsightFiltersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityhubInsightFiltersEl {}
impl BuildSecurityhubInsightFiltersEl {
    pub fn build(self) -> SecurityhubInsightFiltersEl {
        SecurityhubInsightFiltersEl {
            aws_account_id: core::default::Default::default(),
            company_name: core::default::Default::default(),
            compliance_status: core::default::Default::default(),
            confidence: core::default::Default::default(),
            created_at: core::default::Default::default(),
            criticality: core::default::Default::default(),
            description: core::default::Default::default(),
            finding_provider_fields_confidence: core::default::Default::default(),
            finding_provider_fields_criticality: core::default::Default::default(),
            finding_provider_fields_related_findings_id: core::default::Default::default(),
            finding_provider_fields_related_findings_product_arn: core::default::Default::default(),
            finding_provider_fields_severity_label: core::default::Default::default(),
            finding_provider_fields_severity_original: core::default::Default::default(),
            finding_provider_fields_types: core::default::Default::default(),
            first_observed_at: core::default::Default::default(),
            generator_id: core::default::Default::default(),
            id: core::default::Default::default(),
            keyword: core::default::Default::default(),
            last_observed_at: core::default::Default::default(),
            malware_name: core::default::Default::default(),
            malware_path: core::default::Default::default(),
            malware_state: core::default::Default::default(),
            malware_type: core::default::Default::default(),
            network_destination_domain: core::default::Default::default(),
            network_destination_ipv4: core::default::Default::default(),
            network_destination_ipv6: core::default::Default::default(),
            network_destination_port: core::default::Default::default(),
            network_direction: core::default::Default::default(),
            network_protocol: core::default::Default::default(),
            network_source_domain: core::default::Default::default(),
            network_source_ipv4: core::default::Default::default(),
            network_source_ipv6: core::default::Default::default(),
            network_source_mac: core::default::Default::default(),
            network_source_port: core::default::Default::default(),
            note_text: core::default::Default::default(),
            note_updated_at: core::default::Default::default(),
            note_updated_by: core::default::Default::default(),
            process_launched_at: core::default::Default::default(),
            process_name: core::default::Default::default(),
            process_parent_pid: core::default::Default::default(),
            process_path: core::default::Default::default(),
            process_pid: core::default::Default::default(),
            process_terminated_at: core::default::Default::default(),
            product_arn: core::default::Default::default(),
            product_fields: core::default::Default::default(),
            product_name: core::default::Default::default(),
            recommendation_text: core::default::Default::default(),
            record_state: core::default::Default::default(),
            related_findings_id: core::default::Default::default(),
            related_findings_product_arn: core::default::Default::default(),
            resource_aws_ec2_instance_iam_instance_profile_arn: core::default::Default::default(),
            resource_aws_ec2_instance_image_id: core::default::Default::default(),
            resource_aws_ec2_instance_ipv4_addresses: core::default::Default::default(),
            resource_aws_ec2_instance_ipv6_addresses: core::default::Default::default(),
            resource_aws_ec2_instance_key_name: core::default::Default::default(),
            resource_aws_ec2_instance_launched_at: core::default::Default::default(),
            resource_aws_ec2_instance_subnet_id: core::default::Default::default(),
            resource_aws_ec2_instance_type: core::default::Default::default(),
            resource_aws_ec2_instance_vpc_id: core::default::Default::default(),
            resource_aws_iam_access_key_created_at: core::default::Default::default(),
            resource_aws_iam_access_key_status: core::default::Default::default(),
            resource_aws_iam_access_key_user_name: core::default::Default::default(),
            resource_aws_s3_bucket_owner_id: core::default::Default::default(),
            resource_aws_s3_bucket_owner_name: core::default::Default::default(),
            resource_container_image_id: core::default::Default::default(),
            resource_container_image_name: core::default::Default::default(),
            resource_container_launched_at: core::default::Default::default(),
            resource_container_name: core::default::Default::default(),
            resource_details_other: core::default::Default::default(),
            resource_id: core::default::Default::default(),
            resource_partition: core::default::Default::default(),
            resource_region: core::default::Default::default(),
            resource_tags: core::default::Default::default(),
            resource_type: core::default::Default::default(),
            severity_label: core::default::Default::default(),
            source_url: core::default::Default::default(),
            threat_intel_indicator_category: core::default::Default::default(),
            threat_intel_indicator_last_observed_at: core::default::Default::default(),
            threat_intel_indicator_source: core::default::Default::default(),
            threat_intel_indicator_source_url: core::default::Default::default(),
            threat_intel_indicator_type: core::default::Default::default(),
            threat_intel_indicator_value: core::default::Default::default(),
            title: core::default::Default::default(),
            type_: core::default::Default::default(),
            updated_at: core::default::Default::default(),
            user_defined_values: core::default::Default::default(),
            verification_state: core::default::Default::default(),
            workflow_status: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct SecurityhubInsightFiltersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityhubInsightFiltersElRef {
    fn new(shared: StackShared, base: String) -> SecurityhubInsightFiltersElRef {
        SecurityhubInsightFiltersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityhubInsightFiltersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize, Default)]
struct SecurityhubInsightDynamic {
    filters: Option<DynamicBlock<SecurityhubInsightFiltersEl>>,
}
