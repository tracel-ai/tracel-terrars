use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct EvidentlyLaunchData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    project: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    randomization_salt: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    groups: Option<Vec<EvidentlyLaunchGroupsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metric_monitors: Option<Vec<EvidentlyLaunchMetricMonitorsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scheduled_splits_config: Option<Vec<EvidentlyLaunchScheduledSplitsConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<EvidentlyLaunchTimeoutsEl>,
    dynamic: EvidentlyLaunchDynamic,
}

struct EvidentlyLaunch_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<EvidentlyLaunchData>,
}

#[derive(Clone)]
pub struct EvidentlyLaunch(Rc<EvidentlyLaunch_>);

impl EvidentlyLaunch {
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

    #[doc = "Set the field `randomization_salt`.\n"]
    pub fn set_randomization_salt(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().randomization_salt = Some(v.into());
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

    #[doc = "Set the field `groups`.\n"]
    pub fn set_groups(self, v: impl Into<BlockAssignable<EvidentlyLaunchGroupsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().groups = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.groups = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `metric_monitors`.\n"]
    pub fn set_metric_monitors(
        self,
        v: impl Into<BlockAssignable<EvidentlyLaunchMetricMonitorsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().metric_monitors = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.metric_monitors = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `scheduled_splits_config`.\n"]
    pub fn set_scheduled_splits_config(
        self,
        v: impl Into<BlockAssignable<EvidentlyLaunchScheduledSplitsConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().scheduled_splits_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.scheduled_splits_config = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<EvidentlyLaunchTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
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

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `execution` after provisioning.\n"]
    pub fn execution(&self) -> ListRef<EvidentlyLaunchExecutionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.execution", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `randomization_salt` after provisioning.\n"]
    pub fn randomization_salt(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.randomization_salt", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `status_reason` after provisioning.\n"]
    pub fn status_reason(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status_reason", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `groups` after provisioning.\n"]
    pub fn groups(&self) -> ListRef<EvidentlyLaunchGroupsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.groups", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `metric_monitors` after provisioning.\n"]
    pub fn metric_monitors(&self) -> ListRef<EvidentlyLaunchMetricMonitorsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.metric_monitors", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `scheduled_splits_config` after provisioning.\n"]
    pub fn scheduled_splits_config(&self) -> ListRef<EvidentlyLaunchScheduledSplitsConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.scheduled_splits_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> EvidentlyLaunchTimeoutsElRef {
        EvidentlyLaunchTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for EvidentlyLaunch {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for EvidentlyLaunch {}

impl ToListMappable for EvidentlyLaunch {
    type O = ListRef<EvidentlyLaunchRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for EvidentlyLaunch_ {
    fn extract_resource_type(&self) -> String {
        "aws_evidently_launch".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildEvidentlyLaunch {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub project: PrimField<String>,
}

impl BuildEvidentlyLaunch {
    pub fn build(self, stack: &mut Stack) -> EvidentlyLaunch {
        let out = EvidentlyLaunch(Rc::new(EvidentlyLaunch_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(EvidentlyLaunchData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                project: self.project,
                randomization_salt: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                groups: core::default::Default::default(),
                metric_monitors: core::default::Default::default(),
                scheduled_splits_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct EvidentlyLaunchRef {
    shared: StackShared,
    base: String,
}

impl Ref for EvidentlyLaunchRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl EvidentlyLaunchRef {
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

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `execution` after provisioning.\n"]
    pub fn execution(&self) -> ListRef<EvidentlyLaunchExecutionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.execution", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `randomization_salt` after provisioning.\n"]
    pub fn randomization_salt(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.randomization_salt", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `status_reason` after provisioning.\n"]
    pub fn status_reason(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status_reason", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `groups` after provisioning.\n"]
    pub fn groups(&self) -> ListRef<EvidentlyLaunchGroupsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.groups", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `metric_monitors` after provisioning.\n"]
    pub fn metric_monitors(&self) -> ListRef<EvidentlyLaunchMetricMonitorsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.metric_monitors", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `scheduled_splits_config` after provisioning.\n"]
    pub fn scheduled_splits_config(&self) -> ListRef<EvidentlyLaunchScheduledSplitsConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.scheduled_splits_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> EvidentlyLaunchTimeoutsElRef {
        EvidentlyLaunchTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct EvidentlyLaunchExecutionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ended_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    started_time: Option<PrimField<String>>,
}

impl EvidentlyLaunchExecutionEl {
    #[doc = "Set the field `ended_time`.\n"]
    pub fn set_ended_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ended_time = Some(v.into());
        self
    }

    #[doc = "Set the field `started_time`.\n"]
    pub fn set_started_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.started_time = Some(v.into());
        self
    }
}

impl ToListMappable for EvidentlyLaunchExecutionEl {
    type O = BlockAssignable<EvidentlyLaunchExecutionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEvidentlyLaunchExecutionEl {}

impl BuildEvidentlyLaunchExecutionEl {
    pub fn build(self) -> EvidentlyLaunchExecutionEl {
        EvidentlyLaunchExecutionEl {
            ended_time: core::default::Default::default(),
            started_time: core::default::Default::default(),
        }
    }
}

pub struct EvidentlyLaunchExecutionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EvidentlyLaunchExecutionElRef {
    fn new(shared: StackShared, base: String) -> EvidentlyLaunchExecutionElRef {
        EvidentlyLaunchExecutionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EvidentlyLaunchExecutionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ended_time` after provisioning.\n"]
    pub fn ended_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ended_time", self.base))
    }

    #[doc = "Get a reference to the value of field `started_time` after provisioning.\n"]
    pub fn started_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.started_time", self.base))
    }
}

#[derive(Serialize)]
pub struct EvidentlyLaunchGroupsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    feature: PrimField<String>,
    name: PrimField<String>,
    variation: PrimField<String>,
}

impl EvidentlyLaunchGroupsEl {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
}

impl ToListMappable for EvidentlyLaunchGroupsEl {
    type O = BlockAssignable<EvidentlyLaunchGroupsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEvidentlyLaunchGroupsEl {
    #[doc = ""]
    pub feature: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub variation: PrimField<String>,
}

impl BuildEvidentlyLaunchGroupsEl {
    pub fn build(self) -> EvidentlyLaunchGroupsEl {
        EvidentlyLaunchGroupsEl {
            description: core::default::Default::default(),
            feature: self.feature,
            name: self.name,
            variation: self.variation,
        }
    }
}

pub struct EvidentlyLaunchGroupsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EvidentlyLaunchGroupsElRef {
    fn new(shared: StackShared, base: String) -> EvidentlyLaunchGroupsElRef {
        EvidentlyLaunchGroupsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EvidentlyLaunchGroupsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `feature` after provisioning.\n"]
    pub fn feature(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.feature", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `variation` after provisioning.\n"]
    pub fn variation(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.variation", self.base))
    }
}

#[derive(Serialize)]
pub struct EvidentlyLaunchMetricMonitorsElMetricDefinitionEl {
    entity_id_key: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    event_pattern: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unit_label: Option<PrimField<String>>,
    value_key: PrimField<String>,
}

impl EvidentlyLaunchMetricMonitorsElMetricDefinitionEl {
    #[doc = "Set the field `event_pattern`.\n"]
    pub fn set_event_pattern(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.event_pattern = Some(v.into());
        self
    }

    #[doc = "Set the field `unit_label`.\n"]
    pub fn set_unit_label(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.unit_label = Some(v.into());
        self
    }
}

impl ToListMappable for EvidentlyLaunchMetricMonitorsElMetricDefinitionEl {
    type O = BlockAssignable<EvidentlyLaunchMetricMonitorsElMetricDefinitionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEvidentlyLaunchMetricMonitorsElMetricDefinitionEl {
    #[doc = ""]
    pub entity_id_key: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub value_key: PrimField<String>,
}

impl BuildEvidentlyLaunchMetricMonitorsElMetricDefinitionEl {
    pub fn build(self) -> EvidentlyLaunchMetricMonitorsElMetricDefinitionEl {
        EvidentlyLaunchMetricMonitorsElMetricDefinitionEl {
            entity_id_key: self.entity_id_key,
            event_pattern: core::default::Default::default(),
            name: self.name,
            unit_label: core::default::Default::default(),
            value_key: self.value_key,
        }
    }
}

pub struct EvidentlyLaunchMetricMonitorsElMetricDefinitionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EvidentlyLaunchMetricMonitorsElMetricDefinitionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> EvidentlyLaunchMetricMonitorsElMetricDefinitionElRef {
        EvidentlyLaunchMetricMonitorsElMetricDefinitionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EvidentlyLaunchMetricMonitorsElMetricDefinitionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `entity_id_key` after provisioning.\n"]
    pub fn entity_id_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.entity_id_key", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `event_pattern` after provisioning.\n"]
    pub fn event_pattern(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.event_pattern", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `unit_label` after provisioning.\n"]
    pub fn unit_label(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.unit_label", self.base))
    }

    #[doc = "Get a reference to the value of field `value_key` after provisioning.\n"]
    pub fn value_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value_key", self.base))
    }
}

#[derive(Serialize, Default)]
struct EvidentlyLaunchMetricMonitorsElDynamic {
    metric_definition: Option<DynamicBlock<EvidentlyLaunchMetricMonitorsElMetricDefinitionEl>>,
}

#[derive(Serialize)]
pub struct EvidentlyLaunchMetricMonitorsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    metric_definition: Option<Vec<EvidentlyLaunchMetricMonitorsElMetricDefinitionEl>>,
    dynamic: EvidentlyLaunchMetricMonitorsElDynamic,
}

impl EvidentlyLaunchMetricMonitorsEl {
    #[doc = "Set the field `metric_definition`.\n"]
    pub fn set_metric_definition(
        mut self,
        v: impl Into<BlockAssignable<EvidentlyLaunchMetricMonitorsElMetricDefinitionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.metric_definition = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.metric_definition = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for EvidentlyLaunchMetricMonitorsEl {
    type O = BlockAssignable<EvidentlyLaunchMetricMonitorsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEvidentlyLaunchMetricMonitorsEl {}

impl BuildEvidentlyLaunchMetricMonitorsEl {
    pub fn build(self) -> EvidentlyLaunchMetricMonitorsEl {
        EvidentlyLaunchMetricMonitorsEl {
            metric_definition: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct EvidentlyLaunchMetricMonitorsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EvidentlyLaunchMetricMonitorsElRef {
    fn new(shared: StackShared, base: String) -> EvidentlyLaunchMetricMonitorsElRef {
        EvidentlyLaunchMetricMonitorsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EvidentlyLaunchMetricMonitorsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `metric_definition` after provisioning.\n"]
    pub fn metric_definition(
        &self,
    ) -> ListRef<EvidentlyLaunchMetricMonitorsElMetricDefinitionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.metric_definition", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct EvidentlyLaunchScheduledSplitsConfigElStepsElSegmentOverridesEl {
    evaluation_order: PrimField<f64>,
    segment: PrimField<String>,
    weights: RecField<PrimField<f64>>,
}

impl EvidentlyLaunchScheduledSplitsConfigElStepsElSegmentOverridesEl {}

impl ToListMappable for EvidentlyLaunchScheduledSplitsConfigElStepsElSegmentOverridesEl {
    type O = BlockAssignable<EvidentlyLaunchScheduledSplitsConfigElStepsElSegmentOverridesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEvidentlyLaunchScheduledSplitsConfigElStepsElSegmentOverridesEl {
    #[doc = ""]
    pub evaluation_order: PrimField<f64>,
    #[doc = ""]
    pub segment: PrimField<String>,
    #[doc = ""]
    pub weights: RecField<PrimField<f64>>,
}

impl BuildEvidentlyLaunchScheduledSplitsConfigElStepsElSegmentOverridesEl {
    pub fn build(self) -> EvidentlyLaunchScheduledSplitsConfigElStepsElSegmentOverridesEl {
        EvidentlyLaunchScheduledSplitsConfigElStepsElSegmentOverridesEl {
            evaluation_order: self.evaluation_order,
            segment: self.segment,
            weights: self.weights,
        }
    }
}

pub struct EvidentlyLaunchScheduledSplitsConfigElStepsElSegmentOverridesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EvidentlyLaunchScheduledSplitsConfigElStepsElSegmentOverridesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> EvidentlyLaunchScheduledSplitsConfigElStepsElSegmentOverridesElRef {
        EvidentlyLaunchScheduledSplitsConfigElStepsElSegmentOverridesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EvidentlyLaunchScheduledSplitsConfigElStepsElSegmentOverridesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `evaluation_order` after provisioning.\n"]
    pub fn evaluation_order(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.evaluation_order", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `segment` after provisioning.\n"]
    pub fn segment(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.segment", self.base))
    }

    #[doc = "Get a reference to the value of field `weights` after provisioning.\n"]
    pub fn weights(&self) -> RecRef<PrimExpr<f64>> {
        RecRef::new(self.shared().clone(), format!("{}.weights", self.base))
    }
}

#[derive(Serialize, Default)]
struct EvidentlyLaunchScheduledSplitsConfigElStepsElDynamic {
    segment_overrides:
        Option<DynamicBlock<EvidentlyLaunchScheduledSplitsConfigElStepsElSegmentOverridesEl>>,
}

#[derive(Serialize)]
pub struct EvidentlyLaunchScheduledSplitsConfigElStepsEl {
    group_weights: RecField<PrimField<f64>>,
    start_time: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    segment_overrides: Option<Vec<EvidentlyLaunchScheduledSplitsConfigElStepsElSegmentOverridesEl>>,
    dynamic: EvidentlyLaunchScheduledSplitsConfigElStepsElDynamic,
}

impl EvidentlyLaunchScheduledSplitsConfigElStepsEl {
    #[doc = "Set the field `segment_overrides`.\n"]
    pub fn set_segment_overrides(
        mut self,
        v: impl Into<BlockAssignable<EvidentlyLaunchScheduledSplitsConfigElStepsElSegmentOverridesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.segment_overrides = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.segment_overrides = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for EvidentlyLaunchScheduledSplitsConfigElStepsEl {
    type O = BlockAssignable<EvidentlyLaunchScheduledSplitsConfigElStepsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEvidentlyLaunchScheduledSplitsConfigElStepsEl {
    #[doc = ""]
    pub group_weights: RecField<PrimField<f64>>,
    #[doc = ""]
    pub start_time: PrimField<String>,
}

impl BuildEvidentlyLaunchScheduledSplitsConfigElStepsEl {
    pub fn build(self) -> EvidentlyLaunchScheduledSplitsConfigElStepsEl {
        EvidentlyLaunchScheduledSplitsConfigElStepsEl {
            group_weights: self.group_weights,
            start_time: self.start_time,
            segment_overrides: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct EvidentlyLaunchScheduledSplitsConfigElStepsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EvidentlyLaunchScheduledSplitsConfigElStepsElRef {
    fn new(shared: StackShared, base: String) -> EvidentlyLaunchScheduledSplitsConfigElStepsElRef {
        EvidentlyLaunchScheduledSplitsConfigElStepsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EvidentlyLaunchScheduledSplitsConfigElStepsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `group_weights` after provisioning.\n"]
    pub fn group_weights(&self) -> RecRef<PrimExpr<f64>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.group_weights", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }

    #[doc = "Get a reference to the value of field `segment_overrides` after provisioning.\n"]
    pub fn segment_overrides(
        &self,
    ) -> ListRef<EvidentlyLaunchScheduledSplitsConfigElStepsElSegmentOverridesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.segment_overrides", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct EvidentlyLaunchScheduledSplitsConfigElDynamic {
    steps: Option<DynamicBlock<EvidentlyLaunchScheduledSplitsConfigElStepsEl>>,
}

#[derive(Serialize)]
pub struct EvidentlyLaunchScheduledSplitsConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    steps: Option<Vec<EvidentlyLaunchScheduledSplitsConfigElStepsEl>>,
    dynamic: EvidentlyLaunchScheduledSplitsConfigElDynamic,
}

impl EvidentlyLaunchScheduledSplitsConfigEl {
    #[doc = "Set the field `steps`.\n"]
    pub fn set_steps(
        mut self,
        v: impl Into<BlockAssignable<EvidentlyLaunchScheduledSplitsConfigElStepsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.steps = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.steps = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for EvidentlyLaunchScheduledSplitsConfigEl {
    type O = BlockAssignable<EvidentlyLaunchScheduledSplitsConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEvidentlyLaunchScheduledSplitsConfigEl {}

impl BuildEvidentlyLaunchScheduledSplitsConfigEl {
    pub fn build(self) -> EvidentlyLaunchScheduledSplitsConfigEl {
        EvidentlyLaunchScheduledSplitsConfigEl {
            steps: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct EvidentlyLaunchScheduledSplitsConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EvidentlyLaunchScheduledSplitsConfigElRef {
    fn new(shared: StackShared, base: String) -> EvidentlyLaunchScheduledSplitsConfigElRef {
        EvidentlyLaunchScheduledSplitsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EvidentlyLaunchScheduledSplitsConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `steps` after provisioning.\n"]
    pub fn steps(&self) -> ListRef<EvidentlyLaunchScheduledSplitsConfigElStepsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.steps", self.base))
    }
}

#[derive(Serialize)]
pub struct EvidentlyLaunchTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl EvidentlyLaunchTimeoutsEl {
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

impl ToListMappable for EvidentlyLaunchTimeoutsEl {
    type O = BlockAssignable<EvidentlyLaunchTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEvidentlyLaunchTimeoutsEl {}

impl BuildEvidentlyLaunchTimeoutsEl {
    pub fn build(self) -> EvidentlyLaunchTimeoutsEl {
        EvidentlyLaunchTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct EvidentlyLaunchTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EvidentlyLaunchTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> EvidentlyLaunchTimeoutsElRef {
        EvidentlyLaunchTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EvidentlyLaunchTimeoutsElRef {
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

#[derive(Serialize, Default)]
struct EvidentlyLaunchDynamic {
    groups: Option<DynamicBlock<EvidentlyLaunchGroupsEl>>,
    metric_monitors: Option<DynamicBlock<EvidentlyLaunchMetricMonitorsEl>>,
    scheduled_splits_config: Option<DynamicBlock<EvidentlyLaunchScheduledSplitsConfigEl>>,
}
