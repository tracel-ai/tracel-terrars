use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct GuarddutyFilterData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    action: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    detector_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    rank: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    finding_criteria: Option<Vec<GuarddutyFilterFindingCriteriaEl>>,
    dynamic: GuarddutyFilterDynamic,
}

struct GuarddutyFilter_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<GuarddutyFilterData>,
}

#[derive(Clone)]
pub struct GuarddutyFilter(Rc<GuarddutyFilter_>);

impl GuarddutyFilter {
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

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags_all = Some(v.into());
        self
    }

    #[doc = "Set the field `finding_criteria`.\n"]
    pub fn set_finding_criteria(self, v: impl Into<BlockAssignable<GuarddutyFilterFindingCriteriaEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().finding_criteria = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.finding_criteria = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `detector_id` after provisioning.\n"]
    pub fn detector_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.detector_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rank` after provisioning.\n"]
    pub fn rank(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.rank", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `finding_criteria` after provisioning.\n"]
    pub fn finding_criteria(&self) -> ListRef<GuarddutyFilterFindingCriteriaElRef> {
        ListRef::new(self.shared().clone(), format!("{}.finding_criteria", self.extract_ref()))
    }
}

impl Referable for GuarddutyFilter {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for GuarddutyFilter { }

impl ToListMappable for GuarddutyFilter {
    type O = ListRef<GuarddutyFilterRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for GuarddutyFilter_ {
    fn extract_resource_type(&self) -> String {
        "aws_guardduty_filter".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildGuarddutyFilter {
    pub tf_id: String,
    #[doc = ""]
    pub action: PrimField<String>,
    #[doc = ""]
    pub detector_id: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub rank: PrimField<f64>,
}

impl BuildGuarddutyFilter {
    pub fn build(self, stack: &mut Stack) -> GuarddutyFilter {
        let out = GuarddutyFilter(Rc::new(GuarddutyFilter_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(GuarddutyFilterData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                action: self.action,
                description: core::default::Default::default(),
                detector_id: self.detector_id,
                id: core::default::Default::default(),
                name: self.name,
                rank: self.rank,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                finding_criteria: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct GuarddutyFilterRef {
    shared: StackShared,
    base: String,
}

impl Ref for GuarddutyFilterRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl GuarddutyFilterRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `detector_id` after provisioning.\n"]
    pub fn detector_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.detector_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rank` after provisioning.\n"]
    pub fn rank(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.rank", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `finding_criteria` after provisioning.\n"]
    pub fn finding_criteria(&self) -> ListRef<GuarddutyFilterFindingCriteriaElRef> {
        ListRef::new(self.shared().clone(), format!("{}.finding_criteria", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct GuarddutyFilterFindingCriteriaElCriterionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    equals: Option<ListField<PrimField<String>>>,
    field: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    greater_than: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    greater_than_or_equal: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    less_than: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    less_than_or_equal: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    not_equals: Option<ListField<PrimField<String>>>,
}

impl GuarddutyFilterFindingCriteriaElCriterionEl {
    #[doc = "Set the field `equals`.\n"]
    pub fn set_equals(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.equals = Some(v.into());
        self
    }

    #[doc = "Set the field `greater_than`.\n"]
    pub fn set_greater_than(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.greater_than = Some(v.into());
        self
    }

    #[doc = "Set the field `greater_than_or_equal`.\n"]
    pub fn set_greater_than_or_equal(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.greater_than_or_equal = Some(v.into());
        self
    }

    #[doc = "Set the field `less_than`.\n"]
    pub fn set_less_than(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.less_than = Some(v.into());
        self
    }

    #[doc = "Set the field `less_than_or_equal`.\n"]
    pub fn set_less_than_or_equal(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.less_than_or_equal = Some(v.into());
        self
    }

    #[doc = "Set the field `not_equals`.\n"]
    pub fn set_not_equals(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.not_equals = Some(v.into());
        self
    }
}

impl ToListMappable for GuarddutyFilterFindingCriteriaElCriterionEl {
    type O = BlockAssignable<GuarddutyFilterFindingCriteriaElCriterionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGuarddutyFilterFindingCriteriaElCriterionEl {
    #[doc = ""]
    pub field: PrimField<String>,
}

impl BuildGuarddutyFilterFindingCriteriaElCriterionEl {
    pub fn build(self) -> GuarddutyFilterFindingCriteriaElCriterionEl {
        GuarddutyFilterFindingCriteriaElCriterionEl {
            equals: core::default::Default::default(),
            field: self.field,
            greater_than: core::default::Default::default(),
            greater_than_or_equal: core::default::Default::default(),
            less_than: core::default::Default::default(),
            less_than_or_equal: core::default::Default::default(),
            not_equals: core::default::Default::default(),
        }
    }
}

pub struct GuarddutyFilterFindingCriteriaElCriterionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for GuarddutyFilterFindingCriteriaElCriterionElRef {
    fn new(shared: StackShared, base: String) -> GuarddutyFilterFindingCriteriaElCriterionElRef {
        GuarddutyFilterFindingCriteriaElCriterionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GuarddutyFilterFindingCriteriaElCriterionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `equals` after provisioning.\n"]
    pub fn equals(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.equals", self.base))
    }

    #[doc = "Get a reference to the value of field `field` after provisioning.\n"]
    pub fn field(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.field", self.base))
    }

    #[doc = "Get a reference to the value of field `greater_than` after provisioning.\n"]
    pub fn greater_than(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.greater_than", self.base))
    }

    #[doc = "Get a reference to the value of field `greater_than_or_equal` after provisioning.\n"]
    pub fn greater_than_or_equal(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.greater_than_or_equal", self.base))
    }

    #[doc = "Get a reference to the value of field `less_than` after provisioning.\n"]
    pub fn less_than(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.less_than", self.base))
    }

    #[doc = "Get a reference to the value of field `less_than_or_equal` after provisioning.\n"]
    pub fn less_than_or_equal(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.less_than_or_equal", self.base))
    }

    #[doc = "Get a reference to the value of field `not_equals` after provisioning.\n"]
    pub fn not_equals(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.not_equals", self.base))
    }
}

#[derive(Serialize, Default)]
struct GuarddutyFilterFindingCriteriaElDynamic {
    criterion: Option<DynamicBlock<GuarddutyFilterFindingCriteriaElCriterionEl>>,
}

#[derive(Serialize)]
pub struct GuarddutyFilterFindingCriteriaEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    criterion: Option<Vec<GuarddutyFilterFindingCriteriaElCriterionEl>>,
    dynamic: GuarddutyFilterFindingCriteriaElDynamic,
}

impl GuarddutyFilterFindingCriteriaEl {
    #[doc = "Set the field `criterion`.\n"]
    pub fn set_criterion(mut self, v: impl Into<BlockAssignable<GuarddutyFilterFindingCriteriaElCriterionEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.criterion = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.criterion = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for GuarddutyFilterFindingCriteriaEl {
    type O = BlockAssignable<GuarddutyFilterFindingCriteriaEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGuarddutyFilterFindingCriteriaEl {}

impl BuildGuarddutyFilterFindingCriteriaEl {
    pub fn build(self) -> GuarddutyFilterFindingCriteriaEl {
        GuarddutyFilterFindingCriteriaEl {
            criterion: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct GuarddutyFilterFindingCriteriaElRef {
    shared: StackShared,
    base: String,
}

impl Ref for GuarddutyFilterFindingCriteriaElRef {
    fn new(shared: StackShared, base: String) -> GuarddutyFilterFindingCriteriaElRef {
        GuarddutyFilterFindingCriteriaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GuarddutyFilterFindingCriteriaElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct GuarddutyFilterDynamic {
    finding_criteria: Option<DynamicBlock<GuarddutyFilterFindingCriteriaEl>>,
}
