use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct Macie2FindingsFilterData {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name_prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    position: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    finding_criteria: Option<Vec<Macie2FindingsFilterFindingCriteriaEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<Macie2FindingsFilterTimeoutsEl>,
    dynamic: Macie2FindingsFilterDynamic,
}

struct Macie2FindingsFilter_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<Macie2FindingsFilterData>,
}

#[derive(Clone)]
pub struct Macie2FindingsFilter(Rc<Macie2FindingsFilter_>);

impl Macie2FindingsFilter {
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

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name = Some(v.into());
        self
    }

    #[doc = "Set the field `name_prefix`.\n"]
    pub fn set_name_prefix(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name_prefix = Some(v.into());
        self
    }

    #[doc = "Set the field `position`.\n"]
    pub fn set_position(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().position = Some(v.into());
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
    pub fn set_finding_criteria(self, v: impl Into<BlockAssignable<Macie2FindingsFilterFindingCriteriaEl>>) -> Self {
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

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<Macie2FindingsFilterTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name_prefix` after provisioning.\n"]
    pub fn name_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name_prefix", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `position` after provisioning.\n"]
    pub fn position(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.position", self.extract_ref()))
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
    pub fn finding_criteria(&self) -> ListRef<Macie2FindingsFilterFindingCriteriaElRef> {
        ListRef::new(self.shared().clone(), format!("{}.finding_criteria", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Macie2FindingsFilterTimeoutsElRef {
        Macie2FindingsFilterTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for Macie2FindingsFilter {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for Macie2FindingsFilter { }

impl ToListMappable for Macie2FindingsFilter {
    type O = ListRef<Macie2FindingsFilterRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for Macie2FindingsFilter_ {
    fn extract_resource_type(&self) -> String {
        "aws_macie2_findings_filter".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildMacie2FindingsFilter {
    pub tf_id: String,
    #[doc = ""]
    pub action: PrimField<String>,
}

impl BuildMacie2FindingsFilter {
    pub fn build(self, stack: &mut Stack) -> Macie2FindingsFilter {
        let out = Macie2FindingsFilter(Rc::new(Macie2FindingsFilter_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(Macie2FindingsFilterData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                action: self.action,
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                name: core::default::Default::default(),
                name_prefix: core::default::Default::default(),
                position: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                finding_criteria: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct Macie2FindingsFilterRef {
    shared: StackShared,
    base: String,
}

impl Ref for Macie2FindingsFilterRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl Macie2FindingsFilterRef {
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name_prefix` after provisioning.\n"]
    pub fn name_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name_prefix", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `position` after provisioning.\n"]
    pub fn position(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.position", self.extract_ref()))
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
    pub fn finding_criteria(&self) -> ListRef<Macie2FindingsFilterFindingCriteriaElRef> {
        ListRef::new(self.shared().clone(), format!("{}.finding_criteria", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Macie2FindingsFilterTimeoutsElRef {
        Macie2FindingsFilterTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct Macie2FindingsFilterFindingCriteriaElCriterionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    eq: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    eq_exact_match: Option<SetField<PrimField<String>>>,
    field: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gt: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gte: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lt: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lte: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    neq: Option<SetField<PrimField<String>>>,
}

impl Macie2FindingsFilterFindingCriteriaElCriterionEl {
    #[doc = "Set the field `eq`.\n"]
    pub fn set_eq(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.eq = Some(v.into());
        self
    }

    #[doc = "Set the field `eq_exact_match`.\n"]
    pub fn set_eq_exact_match(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.eq_exact_match = Some(v.into());
        self
    }

    #[doc = "Set the field `gt`.\n"]
    pub fn set_gt(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gt = Some(v.into());
        self
    }

    #[doc = "Set the field `gte`.\n"]
    pub fn set_gte(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gte = Some(v.into());
        self
    }

    #[doc = "Set the field `lt`.\n"]
    pub fn set_lt(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.lt = Some(v.into());
        self
    }

    #[doc = "Set the field `lte`.\n"]
    pub fn set_lte(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.lte = Some(v.into());
        self
    }

    #[doc = "Set the field `neq`.\n"]
    pub fn set_neq(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.neq = Some(v.into());
        self
    }
}

impl ToListMappable for Macie2FindingsFilterFindingCriteriaElCriterionEl {
    type O = BlockAssignable<Macie2FindingsFilterFindingCriteriaElCriterionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMacie2FindingsFilterFindingCriteriaElCriterionEl {
    #[doc = ""]
    pub field: PrimField<String>,
}

impl BuildMacie2FindingsFilterFindingCriteriaElCriterionEl {
    pub fn build(self) -> Macie2FindingsFilterFindingCriteriaElCriterionEl {
        Macie2FindingsFilterFindingCriteriaElCriterionEl {
            eq: core::default::Default::default(),
            eq_exact_match: core::default::Default::default(),
            field: self.field,
            gt: core::default::Default::default(),
            gte: core::default::Default::default(),
            lt: core::default::Default::default(),
            lte: core::default::Default::default(),
            neq: core::default::Default::default(),
        }
    }
}

pub struct Macie2FindingsFilterFindingCriteriaElCriterionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Macie2FindingsFilterFindingCriteriaElCriterionElRef {
    fn new(shared: StackShared, base: String) -> Macie2FindingsFilterFindingCriteriaElCriterionElRef {
        Macie2FindingsFilterFindingCriteriaElCriterionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Macie2FindingsFilterFindingCriteriaElCriterionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `eq` after provisioning.\n"]
    pub fn eq(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.eq", self.base))
    }

    #[doc = "Get a reference to the value of field `eq_exact_match` after provisioning.\n"]
    pub fn eq_exact_match(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.eq_exact_match", self.base))
    }

    #[doc = "Get a reference to the value of field `field` after provisioning.\n"]
    pub fn field(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.field", self.base))
    }

    #[doc = "Get a reference to the value of field `gt` after provisioning.\n"]
    pub fn gt(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.gt", self.base))
    }

    #[doc = "Get a reference to the value of field `gte` after provisioning.\n"]
    pub fn gte(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.gte", self.base))
    }

    #[doc = "Get a reference to the value of field `lt` after provisioning.\n"]
    pub fn lt(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.lt", self.base))
    }

    #[doc = "Get a reference to the value of field `lte` after provisioning.\n"]
    pub fn lte(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.lte", self.base))
    }

    #[doc = "Get a reference to the value of field `neq` after provisioning.\n"]
    pub fn neq(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.neq", self.base))
    }
}

#[derive(Serialize, Default)]
struct Macie2FindingsFilterFindingCriteriaElDynamic {
    criterion: Option<DynamicBlock<Macie2FindingsFilterFindingCriteriaElCriterionEl>>,
}

#[derive(Serialize)]
pub struct Macie2FindingsFilterFindingCriteriaEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    criterion: Option<Vec<Macie2FindingsFilterFindingCriteriaElCriterionEl>>,
    dynamic: Macie2FindingsFilterFindingCriteriaElDynamic,
}

impl Macie2FindingsFilterFindingCriteriaEl {
    #[doc = "Set the field `criterion`.\n"]
    pub fn set_criterion(
        mut self,
        v: impl Into<BlockAssignable<Macie2FindingsFilterFindingCriteriaElCriterionEl>>,
    ) -> Self {
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

impl ToListMappable for Macie2FindingsFilterFindingCriteriaEl {
    type O = BlockAssignable<Macie2FindingsFilterFindingCriteriaEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMacie2FindingsFilterFindingCriteriaEl {}

impl BuildMacie2FindingsFilterFindingCriteriaEl {
    pub fn build(self) -> Macie2FindingsFilterFindingCriteriaEl {
        Macie2FindingsFilterFindingCriteriaEl {
            criterion: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct Macie2FindingsFilterFindingCriteriaElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Macie2FindingsFilterFindingCriteriaElRef {
    fn new(shared: StackShared, base: String) -> Macie2FindingsFilterFindingCriteriaElRef {
        Macie2FindingsFilterFindingCriteriaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Macie2FindingsFilterFindingCriteriaElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct Macie2FindingsFilterTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
}

impl Macie2FindingsFilterTimeoutsEl {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }
}

impl ToListMappable for Macie2FindingsFilterTimeoutsEl {
    type O = BlockAssignable<Macie2FindingsFilterTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMacie2FindingsFilterTimeoutsEl {}

impl BuildMacie2FindingsFilterTimeoutsEl {
    pub fn build(self) -> Macie2FindingsFilterTimeoutsEl {
        Macie2FindingsFilterTimeoutsEl { create: core::default::Default::default() }
    }
}

pub struct Macie2FindingsFilterTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Macie2FindingsFilterTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> Macie2FindingsFilterTimeoutsElRef {
        Macie2FindingsFilterTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Macie2FindingsFilterTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }
}

#[derive(Serialize, Default)]
struct Macie2FindingsFilterDynamic {
    finding_criteria: Option<DynamicBlock<Macie2FindingsFilterFindingCriteriaEl>>,
}
