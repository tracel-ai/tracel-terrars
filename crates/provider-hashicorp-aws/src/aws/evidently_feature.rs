use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct EvidentlyFeatureData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_variation: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    entity_overrides: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    evaluation_strategy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    project: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<EvidentlyFeatureTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    variations: Option<Vec<EvidentlyFeatureVariationsEl>>,
    dynamic: EvidentlyFeatureDynamic,
}

struct EvidentlyFeature_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<EvidentlyFeatureData>,
}

#[derive(Clone)]
pub struct EvidentlyFeature(Rc<EvidentlyFeature_>);

impl EvidentlyFeature {
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

    #[doc = "Set the field `default_variation`.\n"]
    pub fn set_default_variation(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().default_variation = Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc = "Set the field `entity_overrides`.\n"]
    pub fn set_entity_overrides(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().entity_overrides = Some(v.into());
        self
    }

    #[doc = "Set the field `evaluation_strategy`.\n"]
    pub fn set_evaluation_strategy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().evaluation_strategy = Some(v.into());
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

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<EvidentlyFeatureTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Set the field `variations`.\n"]
    pub fn set_variations(
        self,
        v: impl Into<BlockAssignable<EvidentlyFeatureVariationsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().variations = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.variations = Some(d);
            }
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_time` after provisioning.\n"]
    pub fn created_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created_time", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `default_variation` after provisioning.\n"]
    pub fn default_variation(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_variation", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `entity_overrides` after provisioning.\n"]
    pub fn entity_overrides(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.entity_overrides", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `evaluation_rules` after provisioning.\n"]
    pub fn evaluation_rules(&self) -> SetRef<EvidentlyFeatureEvaluationRulesElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.evaluation_rules", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `evaluation_strategy` after provisioning.\n"]
    pub fn evaluation_strategy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.evaluation_strategy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_updated_time` after provisioning.\n"]
    pub fn last_updated_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_updated_time", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `value_type` after provisioning.\n"]
    pub fn value_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.value_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> EvidentlyFeatureTimeoutsElRef {
        EvidentlyFeatureTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for EvidentlyFeature {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for EvidentlyFeature {}

impl ToListMappable for EvidentlyFeature {
    type O = ListRef<EvidentlyFeatureRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for EvidentlyFeature_ {
    fn extract_resource_type(&self) -> String {
        "aws_evidently_feature".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildEvidentlyFeature {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub project: PrimField<String>,
}

impl BuildEvidentlyFeature {
    pub fn build(self, stack: &mut Stack) -> EvidentlyFeature {
        let out = EvidentlyFeature(Rc::new(EvidentlyFeature_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(EvidentlyFeatureData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                default_variation: core::default::Default::default(),
                description: core::default::Default::default(),
                entity_overrides: core::default::Default::default(),
                evaluation_strategy: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                project: self.project,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                variations: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct EvidentlyFeatureRef {
    shared: StackShared,
    base: String,
}

impl Ref for EvidentlyFeatureRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl EvidentlyFeatureRef {
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

    #[doc = "Get a reference to the value of field `created_time` after provisioning.\n"]
    pub fn created_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created_time", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `default_variation` after provisioning.\n"]
    pub fn default_variation(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_variation", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `entity_overrides` after provisioning.\n"]
    pub fn entity_overrides(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.entity_overrides", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `evaluation_rules` after provisioning.\n"]
    pub fn evaluation_rules(&self) -> SetRef<EvidentlyFeatureEvaluationRulesElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.evaluation_rules", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `evaluation_strategy` after provisioning.\n"]
    pub fn evaluation_strategy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.evaluation_strategy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_updated_time` after provisioning.\n"]
    pub fn last_updated_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_updated_time", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `value_type` after provisioning.\n"]
    pub fn value_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.value_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> EvidentlyFeatureTimeoutsElRef {
        EvidentlyFeatureTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct EvidentlyFeatureEvaluationRulesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}

impl EvidentlyFeatureEvaluationRulesEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}

impl ToListMappable for EvidentlyFeatureEvaluationRulesEl {
    type O = BlockAssignable<EvidentlyFeatureEvaluationRulesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEvidentlyFeatureEvaluationRulesEl {}

impl BuildEvidentlyFeatureEvaluationRulesEl {
    pub fn build(self) -> EvidentlyFeatureEvaluationRulesEl {
        EvidentlyFeatureEvaluationRulesEl {
            name: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct EvidentlyFeatureEvaluationRulesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EvidentlyFeatureEvaluationRulesElRef {
    fn new(shared: StackShared, base: String) -> EvidentlyFeatureEvaluationRulesElRef {
        EvidentlyFeatureEvaluationRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EvidentlyFeatureEvaluationRulesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct EvidentlyFeatureTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl EvidentlyFeatureTimeoutsEl {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }

    #[doc = "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }

    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}

impl ToListMappable for EvidentlyFeatureTimeoutsEl {
    type O = BlockAssignable<EvidentlyFeatureTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEvidentlyFeatureTimeoutsEl {}

impl BuildEvidentlyFeatureTimeoutsEl {
    pub fn build(self) -> EvidentlyFeatureTimeoutsEl {
        EvidentlyFeatureTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct EvidentlyFeatureTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EvidentlyFeatureTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> EvidentlyFeatureTimeoutsElRef {
        EvidentlyFeatureTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EvidentlyFeatureTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc = "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize)]
pub struct EvidentlyFeatureVariationsElValueEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bool_value: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    double_value: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    long_value: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    string_value: Option<PrimField<String>>,
}

impl EvidentlyFeatureVariationsElValueEl {
    #[doc = "Set the field `bool_value`.\n"]
    pub fn set_bool_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bool_value = Some(v.into());
        self
    }

    #[doc = "Set the field `double_value`.\n"]
    pub fn set_double_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.double_value = Some(v.into());
        self
    }

    #[doc = "Set the field `long_value`.\n"]
    pub fn set_long_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.long_value = Some(v.into());
        self
    }

    #[doc = "Set the field `string_value`.\n"]
    pub fn set_string_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.string_value = Some(v.into());
        self
    }
}

impl ToListMappable for EvidentlyFeatureVariationsElValueEl {
    type O = BlockAssignable<EvidentlyFeatureVariationsElValueEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEvidentlyFeatureVariationsElValueEl {}

impl BuildEvidentlyFeatureVariationsElValueEl {
    pub fn build(self) -> EvidentlyFeatureVariationsElValueEl {
        EvidentlyFeatureVariationsElValueEl {
            bool_value: core::default::Default::default(),
            double_value: core::default::Default::default(),
            long_value: core::default::Default::default(),
            string_value: core::default::Default::default(),
        }
    }
}

pub struct EvidentlyFeatureVariationsElValueElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EvidentlyFeatureVariationsElValueElRef {
    fn new(shared: StackShared, base: String) -> EvidentlyFeatureVariationsElValueElRef {
        EvidentlyFeatureVariationsElValueElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EvidentlyFeatureVariationsElValueElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bool_value` after provisioning.\n"]
    pub fn bool_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bool_value", self.base))
    }

    #[doc = "Get a reference to the value of field `double_value` after provisioning.\n"]
    pub fn double_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.double_value", self.base))
    }

    #[doc = "Get a reference to the value of field `long_value` after provisioning.\n"]
    pub fn long_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.long_value", self.base))
    }

    #[doc = "Get a reference to the value of field `string_value` after provisioning.\n"]
    pub fn string_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.string_value", self.base))
    }
}

#[derive(Serialize, Default)]
struct EvidentlyFeatureVariationsElDynamic {
    value: Option<DynamicBlock<EvidentlyFeatureVariationsElValueEl>>,
}

#[derive(Serialize)]
pub struct EvidentlyFeatureVariationsEl {
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<Vec<EvidentlyFeatureVariationsElValueEl>>,
    dynamic: EvidentlyFeatureVariationsElDynamic,
}

impl EvidentlyFeatureVariationsEl {
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(
        mut self,
        v: impl Into<BlockAssignable<EvidentlyFeatureVariationsElValueEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.value = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.value = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for EvidentlyFeatureVariationsEl {
    type O = BlockAssignable<EvidentlyFeatureVariationsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEvidentlyFeatureVariationsEl {
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildEvidentlyFeatureVariationsEl {
    pub fn build(self) -> EvidentlyFeatureVariationsEl {
        EvidentlyFeatureVariationsEl {
            name: self.name,
            value: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct EvidentlyFeatureVariationsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EvidentlyFeatureVariationsElRef {
    fn new(shared: StackShared, base: String) -> EvidentlyFeatureVariationsElRef {
        EvidentlyFeatureVariationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EvidentlyFeatureVariationsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> ListRef<EvidentlyFeatureVariationsElValueElRef> {
        ListRef::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct EvidentlyFeatureDynamic {
    variations: Option<DynamicBlock<EvidentlyFeatureVariationsEl>>,
}
