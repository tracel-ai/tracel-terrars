use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct Macie2ClassificationJobData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_data_identifier_ids: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    initial_run: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    job_status: Option<PrimField<String>>,
    job_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name_prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sampling_percentage: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_job_definition: Option<Vec<Macie2ClassificationJobS3JobDefinitionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schedule_frequency: Option<Vec<Macie2ClassificationJobScheduleFrequencyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<Macie2ClassificationJobTimeoutsEl>,
    dynamic: Macie2ClassificationJobDynamic,
}
struct Macie2ClassificationJob_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<Macie2ClassificationJobData>,
}
#[derive(Clone)]
pub struct Macie2ClassificationJob(Rc<Macie2ClassificationJob_>);
impl Macie2ClassificationJob {
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
    #[doc = "Set the field `custom_data_identifier_ids`.\n"]
    pub fn set_custom_data_identifier_ids(
        self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.0.data.borrow_mut().custom_data_identifier_ids = Some(v.into());
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
    #[doc = "Set the field `initial_run`.\n"]
    pub fn set_initial_run(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().initial_run = Some(v.into());
        self
    }
    #[doc = "Set the field `job_status`.\n"]
    pub fn set_job_status(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().job_status = Some(v.into());
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
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `sampling_percentage`.\n"]
    pub fn set_sampling_percentage(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().sampling_percentage = Some(v.into());
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
    #[doc = "Set the field `s3_job_definition`.\n"]
    pub fn set_s3_job_definition(
        self,
        v: impl Into<BlockAssignable<Macie2ClassificationJobS3JobDefinitionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().s3_job_definition = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.s3_job_definition = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `schedule_frequency`.\n"]
    pub fn set_schedule_frequency(
        self,
        v: impl Into<BlockAssignable<Macie2ClassificationJobScheduleFrequencyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().schedule_frequency = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.schedule_frequency = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<Macie2ClassificationJobTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created_at", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `custom_data_identifier_ids` after provisioning.\n"]
    pub fn custom_data_identifier_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_data_identifier_ids", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `initial_run` after provisioning.\n"]
    pub fn initial_run(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.initial_run", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `job_arn` after provisioning.\n"]
    pub fn job_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.job_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `job_id` after provisioning.\n"]
    pub fn job_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.job_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `job_status` after provisioning.\n"]
    pub fn job_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.job_status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `job_type` after provisioning.\n"]
    pub fn job_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.job_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name_prefix` after provisioning.\n"]
    pub fn name_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name_prefix", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `sampling_percentage` after provisioning.\n"]
    pub fn sampling_percentage(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sampling_percentage", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `user_paused_details` after provisioning.\n"]
    pub fn user_paused_details(&self) -> ListRef<Macie2ClassificationJobUserPausedDetailsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.user_paused_details", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `s3_job_definition` after provisioning.\n"]
    pub fn s3_job_definition(&self) -> ListRef<Macie2ClassificationJobS3JobDefinitionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.s3_job_definition", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `schedule_frequency` after provisioning.\n"]
    pub fn schedule_frequency(&self) -> ListRef<Macie2ClassificationJobScheduleFrequencyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.schedule_frequency", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Macie2ClassificationJobTimeoutsElRef {
        Macie2ClassificationJobTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for Macie2ClassificationJob {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for Macie2ClassificationJob {}
impl ToListMappable for Macie2ClassificationJob {
    type O = ListRef<Macie2ClassificationJobRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for Macie2ClassificationJob_ {
    fn extract_resource_type(&self) -> String {
        "aws_macie2_classification_job".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildMacie2ClassificationJob {
    pub tf_id: String,
    #[doc = ""]
    pub job_type: PrimField<String>,
}
impl BuildMacie2ClassificationJob {
    pub fn build(self, stack: &mut Stack) -> Macie2ClassificationJob {
        let out = Macie2ClassificationJob(Rc::new(Macie2ClassificationJob_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(Macie2ClassificationJobData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                custom_data_identifier_ids: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                initial_run: core::default::Default::default(),
                job_status: core::default::Default::default(),
                job_type: self.job_type,
                name: core::default::Default::default(),
                name_prefix: core::default::Default::default(),
                region: core::default::Default::default(),
                sampling_percentage: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                s3_job_definition: core::default::Default::default(),
                schedule_frequency: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct Macie2ClassificationJobRef {
    shared: StackShared,
    base: String,
}
impl Ref for Macie2ClassificationJobRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl Macie2ClassificationJobRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created_at", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `custom_data_identifier_ids` after provisioning.\n"]
    pub fn custom_data_identifier_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_data_identifier_ids", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `initial_run` after provisioning.\n"]
    pub fn initial_run(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.initial_run", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `job_arn` after provisioning.\n"]
    pub fn job_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.job_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `job_id` after provisioning.\n"]
    pub fn job_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.job_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `job_status` after provisioning.\n"]
    pub fn job_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.job_status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `job_type` after provisioning.\n"]
    pub fn job_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.job_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name_prefix` after provisioning.\n"]
    pub fn name_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name_prefix", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `sampling_percentage` after provisioning.\n"]
    pub fn sampling_percentage(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sampling_percentage", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `user_paused_details` after provisioning.\n"]
    pub fn user_paused_details(&self) -> ListRef<Macie2ClassificationJobUserPausedDetailsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.user_paused_details", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `s3_job_definition` after provisioning.\n"]
    pub fn s3_job_definition(&self) -> ListRef<Macie2ClassificationJobS3JobDefinitionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.s3_job_definition", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `schedule_frequency` after provisioning.\n"]
    pub fn schedule_frequency(&self) -> ListRef<Macie2ClassificationJobScheduleFrequencyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.schedule_frequency", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Macie2ClassificationJobTimeoutsElRef {
        Macie2ClassificationJobTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct Macie2ClassificationJobUserPausedDetailsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    job_expires_at: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    job_imminent_expiration_health_event_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    job_paused_at: Option<PrimField<String>>,
}
impl Macie2ClassificationJobUserPausedDetailsEl {
    #[doc = "Set the field `job_expires_at`.\n"]
    pub fn set_job_expires_at(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.job_expires_at = Some(v.into());
        self
    }
    #[doc = "Set the field `job_imminent_expiration_health_event_arn`.\n"]
    pub fn set_job_imminent_expiration_health_event_arn(
        mut self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.job_imminent_expiration_health_event_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `job_paused_at`.\n"]
    pub fn set_job_paused_at(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.job_paused_at = Some(v.into());
        self
    }
}
impl ToListMappable for Macie2ClassificationJobUserPausedDetailsEl {
    type O = BlockAssignable<Macie2ClassificationJobUserPausedDetailsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMacie2ClassificationJobUserPausedDetailsEl {}
impl BuildMacie2ClassificationJobUserPausedDetailsEl {
    pub fn build(self) -> Macie2ClassificationJobUserPausedDetailsEl {
        Macie2ClassificationJobUserPausedDetailsEl {
            job_expires_at: core::default::Default::default(),
            job_imminent_expiration_health_event_arn: core::default::Default::default(),
            job_paused_at: core::default::Default::default(),
        }
    }
}
pub struct Macie2ClassificationJobUserPausedDetailsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Macie2ClassificationJobUserPausedDetailsElRef {
    fn new(shared: StackShared, base: String) -> Macie2ClassificationJobUserPausedDetailsElRef {
        Macie2ClassificationJobUserPausedDetailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Macie2ClassificationJobUserPausedDetailsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `job_expires_at` after provisioning.\n"]
    pub fn job_expires_at(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.job_expires_at", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `job_imminent_expiration_health_event_arn` after provisioning.\n"]
    pub fn job_imminent_expiration_health_event_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.job_imminent_expiration_health_event_arn", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `job_paused_at` after provisioning.\n"]
    pub fn job_paused_at(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.job_paused_at", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElSimpleCriterionEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    comparator: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<ListField<PrimField<String>>>,
}
impl Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElSimpleCriterionEl {
    #[doc = "Set the field `comparator`.\n"]
    pub fn set_comparator(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.comparator = Some(v.into());
        self
    }
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }
    #[doc = "Set the field `values`.\n"]
    pub fn set_values(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}
impl ToListMappable
    for Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElSimpleCriterionEl
{
    type O = BlockAssignable<
        Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElSimpleCriterionEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMacie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElSimpleCriterionEl
{}
impl BuildMacie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElSimpleCriterionEl {
    pub fn build(
        self,
    ) -> Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElSimpleCriterionEl
    {
        Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElSimpleCriterionEl {
            comparator: core::default::Default::default(),
            key: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}
pub struct Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElSimpleCriterionElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElSimpleCriterionElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElSimpleCriterionElRef
    {
        Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElSimpleCriterionElRef { shared : shared , base : base . to_string () , }
    }
}
impl Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElSimpleCriterionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `comparator` after provisioning.\n"]
    pub fn comparator(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparator", self.base))
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}
#[derive(Serialize)]
pub struct Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElTagCriterionElTagValuesEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl
    Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElTagCriterionElTagValuesEl
{
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable for Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElTagCriterionElTagValuesEl { type O = BlockAssignable < Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElTagCriterionElTagValuesEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMacie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElTagCriterionElTagValuesEl
{}
impl BuildMacie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElTagCriterionElTagValuesEl { pub fn build (self) -> Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElTagCriterionElTagValuesEl { Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElTagCriterionElTagValuesEl { key : core :: default :: Default :: default () , value : core :: default :: Default :: default () , } } }
pub struct Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElTagCriterionElTagValuesElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElTagCriterionElTagValuesElRef { fn new (shared : StackShared , base : String) -> Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElTagCriterionElTagValuesElRef { Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElTagCriterionElTagValuesElRef { shared : shared , base : base . to_string () , } } }
impl Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElTagCriterionElTagValuesElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `key` after provisioning.\n"] pub fn key (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.key" , self . base)) } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize, Default)]
struct Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElTagCriterionElDynamic { tag_values : Option < DynamicBlock < Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElTagCriterionElTagValuesEl >> , }
#[derive(Serialize)]
pub struct Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElTagCriterionEl { # [serde (skip_serializing_if = "Option::is_none")] comparator : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] tag_values : Option < Vec < Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElTagCriterionElTagValuesEl > > , dynamic : Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElTagCriterionElDynamic , }
impl Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElTagCriterionEl {
    #[doc = "Set the field `comparator`.\n"]
    pub fn set_comparator(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.comparator = Some(v.into());
        self
    }
    #[doc = "Set the field `tag_values`.\n"]
    pub fn set_tag_values(
        mut self,
        v : impl Into < BlockAssignable < Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElTagCriterionElTagValuesEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.tag_values = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.tag_values = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElTagCriterionEl
{
    type O = BlockAssignable<
        Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElTagCriterionEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMacie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElTagCriterionEl
{}
impl BuildMacie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElTagCriterionEl {
    pub fn build(
        self,
    ) -> Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElTagCriterionEl {
        Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElTagCriterionEl {
            comparator: core::default::Default::default(),
            tag_values: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElTagCriterionElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElTagCriterionElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElTagCriterionElRef
    {
        Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElTagCriterionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElTagCriterionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `comparator` after provisioning.\n"]
    pub fn comparator(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparator", self.base))
    }
    #[doc = "Get a reference to the value of field `tag_values` after provisioning.\n"]    pub fn tag_values (& self) -> ListRef < Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElTagCriterionElTagValuesElRef >{
        ListRef::new(self.shared().clone(), format!("{}.tag_values", self.base))
    }
}
#[derive(Serialize, Default)]
struct Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElDynamic { simple_criterion : Option < DynamicBlock < Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElSimpleCriterionEl >> , tag_criterion : Option < DynamicBlock < Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElTagCriterionEl >> , }
#[derive(Serialize)]
pub struct Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndEl { # [serde (skip_serializing_if = "Option::is_none")] simple_criterion : Option < Vec < Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElSimpleCriterionEl > > , # [serde (skip_serializing_if = "Option::is_none")] tag_criterion : Option < Vec < Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElTagCriterionEl > > , dynamic : Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElDynamic , }
impl Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndEl {
    #[doc = "Set the field `simple_criterion`.\n"]
    pub fn set_simple_criterion(
        mut self,
        v : impl Into < BlockAssignable < Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElSimpleCriterionEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.simple_criterion = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.simple_criterion = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `tag_criterion`.\n"]
    pub fn set_tag_criterion(
        mut self,
        v : impl Into < BlockAssignable < Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElTagCriterionEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.tag_criterion = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.tag_criterion = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndEl {
    type O =
        BlockAssignable<Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMacie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndEl {}
impl BuildMacie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndEl {
    pub fn build(self) -> Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndEl {
        Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndEl {
            simple_criterion: core::default::Default::default(),
            tag_criterion: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElRef {
        Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `simple_criterion` after provisioning.\n"]
    pub fn simple_criterion(
        &self,
    ) -> ListRef<
        Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElSimpleCriterionElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.simple_criterion", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `tag_criterion` after provisioning.\n"]
    pub fn tag_criterion(
        &self,
    ) -> ListRef<
        Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElTagCriterionElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tag_criterion", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElDynamic {
    and: Option<
        DynamicBlock<Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndEl>,
    >,
}
#[derive(Serialize)]
pub struct Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    and: Option<Vec<Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndEl>>,
    dynamic: Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElDynamic,
}
impl Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesEl {
    #[doc = "Set the field `and`.\n"]
    pub fn set_and(
        mut self,
        v: impl Into<
            BlockAssignable<
                Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.and = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.and = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesEl {
    type O = BlockAssignable<Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMacie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesEl {}
impl BuildMacie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesEl {
    pub fn build(self) -> Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesEl {
        Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesEl {
            and: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElRef {
        Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `and` after provisioning.\n"]
    pub fn and(
        &self,
    ) -> ListRef<Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElRef> {
        ListRef::new(self.shared().clone(), format!("{}.and", self.base))
    }
}
#[derive(Serialize)]
pub struct Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElSimpleCriterionEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    comparator: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<ListField<PrimField<String>>>,
}
impl Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElSimpleCriterionEl {
    #[doc = "Set the field `comparator`.\n"]
    pub fn set_comparator(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.comparator = Some(v.into());
        self
    }
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }
    #[doc = "Set the field `values`.\n"]
    pub fn set_values(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}
impl ToListMappable
    for Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElSimpleCriterionEl
{
    type O = BlockAssignable<
        Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElSimpleCriterionEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMacie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElSimpleCriterionEl
{}
impl BuildMacie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElSimpleCriterionEl {
    pub fn build(
        self,
    ) -> Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElSimpleCriterionEl
    {
        Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElSimpleCriterionEl {
            comparator: core::default::Default::default(),
            key: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}
pub struct Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElSimpleCriterionElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElSimpleCriterionElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElSimpleCriterionElRef
    {
        Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElSimpleCriterionElRef { shared : shared , base : base . to_string () , }
    }
}
impl Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElSimpleCriterionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `comparator` after provisioning.\n"]
    pub fn comparator(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparator", self.base))
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}
#[derive(Serialize)]
pub struct Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElTagCriterionElTagValuesEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl
    Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElTagCriterionElTagValuesEl
{
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable for Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElTagCriterionElTagValuesEl { type O = BlockAssignable < Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElTagCriterionElTagValuesEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMacie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElTagCriterionElTagValuesEl
{}
impl BuildMacie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElTagCriterionElTagValuesEl { pub fn build (self) -> Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElTagCriterionElTagValuesEl { Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElTagCriterionElTagValuesEl { key : core :: default :: Default :: default () , value : core :: default :: Default :: default () , } } }
pub struct Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElTagCriterionElTagValuesElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElTagCriterionElTagValuesElRef { fn new (shared : StackShared , base : String) -> Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElTagCriterionElTagValuesElRef { Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElTagCriterionElTagValuesElRef { shared : shared , base : base . to_string () , } } }
impl Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElTagCriterionElTagValuesElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `key` after provisioning.\n"] pub fn key (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.key" , self . base)) } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize, Default)]
struct Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElTagCriterionElDynamic { tag_values : Option < DynamicBlock < Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElTagCriterionElTagValuesEl >> , }
#[derive(Serialize)]
pub struct Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElTagCriterionEl { # [serde (skip_serializing_if = "Option::is_none")] comparator : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] tag_values : Option < Vec < Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElTagCriterionElTagValuesEl > > , dynamic : Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElTagCriterionElDynamic , }
impl Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElTagCriterionEl {
    #[doc = "Set the field `comparator`.\n"]
    pub fn set_comparator(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.comparator = Some(v.into());
        self
    }
    #[doc = "Set the field `tag_values`.\n"]
    pub fn set_tag_values(
        mut self,
        v : impl Into < BlockAssignable < Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElTagCriterionElTagValuesEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.tag_values = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.tag_values = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElTagCriterionEl
{
    type O = BlockAssignable<
        Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElTagCriterionEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMacie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElTagCriterionEl
{}
impl BuildMacie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElTagCriterionEl {
    pub fn build(
        self,
    ) -> Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElTagCriterionEl {
        Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElTagCriterionEl {
            comparator: core::default::Default::default(),
            tag_values: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElTagCriterionElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElTagCriterionElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElTagCriterionElRef
    {
        Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElTagCriterionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElTagCriterionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `comparator` after provisioning.\n"]
    pub fn comparator(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparator", self.base))
    }
    #[doc = "Get a reference to the value of field `tag_values` after provisioning.\n"]    pub fn tag_values (& self) -> ListRef < Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElTagCriterionElTagValuesElRef >{
        ListRef::new(self.shared().clone(), format!("{}.tag_values", self.base))
    }
}
#[derive(Serialize, Default)]
struct Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElDynamic { simple_criterion : Option < DynamicBlock < Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElSimpleCriterionEl >> , tag_criterion : Option < DynamicBlock < Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElTagCriterionEl >> , }
#[derive(Serialize)]
pub struct Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndEl { # [serde (skip_serializing_if = "Option::is_none")] simple_criterion : Option < Vec < Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElSimpleCriterionEl > > , # [serde (skip_serializing_if = "Option::is_none")] tag_criterion : Option < Vec < Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElTagCriterionEl > > , dynamic : Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElDynamic , }
impl Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndEl {
    #[doc = "Set the field `simple_criterion`.\n"]
    pub fn set_simple_criterion(
        mut self,
        v : impl Into < BlockAssignable < Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElSimpleCriterionEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.simple_criterion = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.simple_criterion = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `tag_criterion`.\n"]
    pub fn set_tag_criterion(
        mut self,
        v : impl Into < BlockAssignable < Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElTagCriterionEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.tag_criterion = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.tag_criterion = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndEl {
    type O =
        BlockAssignable<Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMacie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndEl {}
impl BuildMacie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndEl {
    pub fn build(self) -> Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndEl {
        Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndEl {
            simple_criterion: core::default::Default::default(),
            tag_criterion: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElRef {
        Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `simple_criterion` after provisioning.\n"]
    pub fn simple_criterion(
        &self,
    ) -> ListRef<
        Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElSimpleCriterionElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.simple_criterion", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `tag_criterion` after provisioning.\n"]
    pub fn tag_criterion(
        &self,
    ) -> ListRef<
        Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElTagCriterionElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tag_criterion", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElDynamic {
    and: Option<
        DynamicBlock<Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndEl>,
    >,
}
#[derive(Serialize)]
pub struct Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    and: Option<Vec<Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndEl>>,
    dynamic: Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElDynamic,
}
impl Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesEl {
    #[doc = "Set the field `and`.\n"]
    pub fn set_and(
        mut self,
        v: impl Into<
            BlockAssignable<
                Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.and = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.and = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesEl {
    type O = BlockAssignable<Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMacie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesEl {}
impl BuildMacie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesEl {
    pub fn build(self) -> Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesEl {
        Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesEl {
            and: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElRef {
        Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `and` after provisioning.\n"]
    pub fn and(
        &self,
    ) -> ListRef<Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElRef> {
        ListRef::new(self.shared().clone(), format!("{}.and", self.base))
    }
}
#[derive(Serialize, Default)]
struct Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElDynamic {
    excludes:
        Option<DynamicBlock<Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesEl>>,
    includes:
        Option<DynamicBlock<Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesEl>>,
}
#[derive(Serialize)]
pub struct Macie2ClassificationJobS3JobDefinitionElBucketCriteriaEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    excludes: Option<Vec<Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    includes: Option<Vec<Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesEl>>,
    dynamic: Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElDynamic,
}
impl Macie2ClassificationJobS3JobDefinitionElBucketCriteriaEl {
    #[doc = "Set the field `excludes`.\n"]
    pub fn set_excludes(
        mut self,
        v: impl Into<
            BlockAssignable<Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.excludes = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.excludes = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `includes`.\n"]
    pub fn set_includes(
        mut self,
        v: impl Into<
            BlockAssignable<Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.includes = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.includes = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for Macie2ClassificationJobS3JobDefinitionElBucketCriteriaEl {
    type O = BlockAssignable<Macie2ClassificationJobS3JobDefinitionElBucketCriteriaEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMacie2ClassificationJobS3JobDefinitionElBucketCriteriaEl {}
impl BuildMacie2ClassificationJobS3JobDefinitionElBucketCriteriaEl {
    pub fn build(self) -> Macie2ClassificationJobS3JobDefinitionElBucketCriteriaEl {
        Macie2ClassificationJobS3JobDefinitionElBucketCriteriaEl {
            excludes: core::default::Default::default(),
            includes: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElRef {
        Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `excludes` after provisioning.\n"]
    pub fn excludes(
        &self,
    ) -> ListRef<Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.excludes", self.base))
    }
    #[doc = "Get a reference to the value of field `includes` after provisioning.\n"]
    pub fn includes(
        &self,
    ) -> ListRef<Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.includes", self.base))
    }
}
#[derive(Serialize)]
pub struct Macie2ClassificationJobS3JobDefinitionElBucketDefinitionsEl {
    account_id: PrimField<String>,
    buckets: ListField<PrimField<String>>,
}
impl Macie2ClassificationJobS3JobDefinitionElBucketDefinitionsEl {}
impl ToListMappable for Macie2ClassificationJobS3JobDefinitionElBucketDefinitionsEl {
    type O = BlockAssignable<Macie2ClassificationJobS3JobDefinitionElBucketDefinitionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMacie2ClassificationJobS3JobDefinitionElBucketDefinitionsEl {
    #[doc = ""]
    pub account_id: PrimField<String>,
    #[doc = ""]
    pub buckets: ListField<PrimField<String>>,
}
impl BuildMacie2ClassificationJobS3JobDefinitionElBucketDefinitionsEl {
    pub fn build(self) -> Macie2ClassificationJobS3JobDefinitionElBucketDefinitionsEl {
        Macie2ClassificationJobS3JobDefinitionElBucketDefinitionsEl {
            account_id: self.account_id,
            buckets: self.buckets,
        }
    }
}
pub struct Macie2ClassificationJobS3JobDefinitionElBucketDefinitionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Macie2ClassificationJobS3JobDefinitionElBucketDefinitionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Macie2ClassificationJobS3JobDefinitionElBucketDefinitionsElRef {
        Macie2ClassificationJobS3JobDefinitionElBucketDefinitionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Macie2ClassificationJobS3JobDefinitionElBucketDefinitionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.base))
    }
    #[doc = "Get a reference to the value of field `buckets` after provisioning.\n"]
    pub fn buckets(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.buckets", self.base))
    }
}
#[derive(Serialize)]
pub struct Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElSimpleScopeTermEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    comparator: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<ListField<PrimField<String>>>,
}
impl Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElSimpleScopeTermEl {
    #[doc = "Set the field `comparator`.\n"]
    pub fn set_comparator(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.comparator = Some(v.into());
        self
    }
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }
    #[doc = "Set the field `values`.\n"]
    pub fn set_values(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}
impl ToListMappable
    for Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElSimpleScopeTermEl
{
    type O = BlockAssignable<
        Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElSimpleScopeTermEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMacie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElSimpleScopeTermEl {
}
impl BuildMacie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElSimpleScopeTermEl {
    pub fn build(
        self,
    ) -> Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElSimpleScopeTermEl {
        Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElSimpleScopeTermEl {
            comparator: core::default::Default::default(),
            key: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}
pub struct Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElSimpleScopeTermElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElSimpleScopeTermElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElSimpleScopeTermElRef {
        Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElSimpleScopeTermElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElSimpleScopeTermElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `comparator` after provisioning.\n"]
    pub fn comparator(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparator", self.base))
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}
#[derive(Serialize)]
pub struct Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElTagScopeTermElTagValuesEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElTagScopeTermElTagValuesEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable
    for Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElTagScopeTermElTagValuesEl
{
    type O = BlockAssignable<
        Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElTagScopeTermElTagValuesEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMacie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElTagScopeTermElTagValuesEl
{}
impl
    BuildMacie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElTagScopeTermElTagValuesEl
{
    pub fn build(
        self,
    ) -> Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElTagScopeTermElTagValuesEl
    {
        Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElTagScopeTermElTagValuesEl {
            key: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}
pub struct Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElTagScopeTermElTagValuesElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElTagScopeTermElTagValuesElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElTagScopeTermElTagValuesElRef
    {
        Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElTagScopeTermElTagValuesElRef { shared : shared , base : base . to_string () , }
    }
}
impl Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElTagScopeTermElTagValuesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
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
#[derive(Serialize, Default)]
struct Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElTagScopeTermElDynamic { tag_values : Option < DynamicBlock < Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElTagScopeTermElTagValuesEl >> , }
#[derive(Serialize)]
pub struct Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElTagScopeTermEl { # [serde (skip_serializing_if = "Option::is_none")] comparator : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] key : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] target : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] tag_values : Option < Vec < Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElTagScopeTermElTagValuesEl > > , dynamic : Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElTagScopeTermElDynamic , }
impl Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElTagScopeTermEl {
    #[doc = "Set the field `comparator`.\n"]
    pub fn set_comparator(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.comparator = Some(v.into());
        self
    }
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }
    #[doc = "Set the field `target`.\n"]
    pub fn set_target(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.target = Some(v.into());
        self
    }
    #[doc = "Set the field `tag_values`.\n"]
    pub fn set_tag_values(
        mut self,
        v : impl Into < BlockAssignable < Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElTagScopeTermElTagValuesEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.tag_values = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.tag_values = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElTagScopeTermEl
{
    type O = BlockAssignable<
        Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElTagScopeTermEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMacie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElTagScopeTermEl {}
impl BuildMacie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElTagScopeTermEl {
    pub fn build(
        self,
    ) -> Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElTagScopeTermEl {
        Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElTagScopeTermEl {
            comparator: core::default::Default::default(),
            key: core::default::Default::default(),
            target: core::default::Default::default(),
            tag_values: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElTagScopeTermElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElTagScopeTermElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElTagScopeTermElRef {
        Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElTagScopeTermElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElTagScopeTermElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `comparator` after provisioning.\n"]
    pub fn comparator(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparator", self.base))
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target", self.base))
    }
    #[doc = "Get a reference to the value of field `tag_values` after provisioning.\n"]    pub fn tag_values (& self) -> ListRef < Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElTagScopeTermElTagValuesElRef >{
        ListRef::new(self.shared().clone(), format!("{}.tag_values", self.base))
    }
}
#[derive(Serialize, Default)]
struct Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElDynamic {
    simple_scope_term: Option<
        DynamicBlock<
            Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElSimpleScopeTermEl,
        >,
    >,
    tag_scope_term: Option<
        DynamicBlock<
            Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElTagScopeTermEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    simple_scope_term: Option<
        Vec<Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElSimpleScopeTermEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag_scope_term:
        Option<Vec<Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElTagScopeTermEl>>,
    dynamic: Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElDynamic,
}
impl Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndEl {
    #[doc = "Set the field `simple_scope_term`.\n"]
    pub fn set_simple_scope_term(
        mut self,
        v: impl Into<
            BlockAssignable<
                Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElSimpleScopeTermEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.simple_scope_term = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.simple_scope_term = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `tag_scope_term`.\n"]
    pub fn set_tag_scope_term(
        mut self,
        v: impl Into<
            BlockAssignable<
                Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElTagScopeTermEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.tag_scope_term = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.tag_scope_term = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndEl {
    type O = BlockAssignable<Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMacie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndEl {}
impl BuildMacie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndEl {
    pub fn build(self) -> Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndEl {
        Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndEl {
            simple_scope_term: core::default::Default::default(),
            tag_scope_term: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElRef {
        Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `simple_scope_term` after provisioning.\n"]
    pub fn simple_scope_term(
        &self,
    ) -> ListRef<Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElSimpleScopeTermElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.simple_scope_term", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `tag_scope_term` after provisioning.\n"]
    pub fn tag_scope_term(
        &self,
    ) -> ListRef<Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElTagScopeTermElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tag_scope_term", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElDynamic {
    and: Option<DynamicBlock<Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndEl>>,
}
#[derive(Serialize)]
pub struct Macie2ClassificationJobS3JobDefinitionElScopingElExcludesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    and: Option<Vec<Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndEl>>,
    dynamic: Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElDynamic,
}
impl Macie2ClassificationJobS3JobDefinitionElScopingElExcludesEl {
    #[doc = "Set the field `and`.\n"]
    pub fn set_and(
        mut self,
        v: impl Into<BlockAssignable<Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.and = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.and = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for Macie2ClassificationJobS3JobDefinitionElScopingElExcludesEl {
    type O = BlockAssignable<Macie2ClassificationJobS3JobDefinitionElScopingElExcludesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMacie2ClassificationJobS3JobDefinitionElScopingElExcludesEl {}
impl BuildMacie2ClassificationJobS3JobDefinitionElScopingElExcludesEl {
    pub fn build(self) -> Macie2ClassificationJobS3JobDefinitionElScopingElExcludesEl {
        Macie2ClassificationJobS3JobDefinitionElScopingElExcludesEl {
            and: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElRef {
        Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `and` after provisioning.\n"]
    pub fn and(
        &self,
    ) -> ListRef<Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElRef> {
        ListRef::new(self.shared().clone(), format!("{}.and", self.base))
    }
}
#[derive(Serialize)]
pub struct Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElSimpleScopeTermEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    comparator: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<ListField<PrimField<String>>>,
}
impl Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElSimpleScopeTermEl {
    #[doc = "Set the field `comparator`.\n"]
    pub fn set_comparator(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.comparator = Some(v.into());
        self
    }
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }
    #[doc = "Set the field `values`.\n"]
    pub fn set_values(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}
impl ToListMappable
    for Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElSimpleScopeTermEl
{
    type O = BlockAssignable<
        Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElSimpleScopeTermEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMacie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElSimpleScopeTermEl {
}
impl BuildMacie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElSimpleScopeTermEl {
    pub fn build(
        self,
    ) -> Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElSimpleScopeTermEl {
        Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElSimpleScopeTermEl {
            comparator: core::default::Default::default(),
            key: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}
pub struct Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElSimpleScopeTermElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElSimpleScopeTermElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElSimpleScopeTermElRef {
        Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElSimpleScopeTermElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElSimpleScopeTermElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `comparator` after provisioning.\n"]
    pub fn comparator(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparator", self.base))
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}
#[derive(Serialize)]
pub struct Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElTagScopeTermElTagValuesEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElTagScopeTermElTagValuesEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable
    for Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElTagScopeTermElTagValuesEl
{
    type O = BlockAssignable<
        Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElTagScopeTermElTagValuesEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMacie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElTagScopeTermElTagValuesEl
{}
impl
    BuildMacie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElTagScopeTermElTagValuesEl
{
    pub fn build(
        self,
    ) -> Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElTagScopeTermElTagValuesEl
    {
        Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElTagScopeTermElTagValuesEl {
            key: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}
pub struct Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElTagScopeTermElTagValuesElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElTagScopeTermElTagValuesElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElTagScopeTermElTagValuesElRef
    {
        Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElTagScopeTermElTagValuesElRef { shared : shared , base : base . to_string () , }
    }
}
impl Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElTagScopeTermElTagValuesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
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
#[derive(Serialize, Default)]
struct Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElTagScopeTermElDynamic { tag_values : Option < DynamicBlock < Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElTagScopeTermElTagValuesEl >> , }
#[derive(Serialize)]
pub struct Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElTagScopeTermEl { # [serde (skip_serializing_if = "Option::is_none")] comparator : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] key : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] target : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] tag_values : Option < Vec < Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElTagScopeTermElTagValuesEl > > , dynamic : Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElTagScopeTermElDynamic , }
impl Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElTagScopeTermEl {
    #[doc = "Set the field `comparator`.\n"]
    pub fn set_comparator(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.comparator = Some(v.into());
        self
    }
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }
    #[doc = "Set the field `target`.\n"]
    pub fn set_target(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.target = Some(v.into());
        self
    }
    #[doc = "Set the field `tag_values`.\n"]
    pub fn set_tag_values(
        mut self,
        v : impl Into < BlockAssignable < Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElTagScopeTermElTagValuesEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.tag_values = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.tag_values = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElTagScopeTermEl
{
    type O = BlockAssignable<
        Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElTagScopeTermEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMacie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElTagScopeTermEl {}
impl BuildMacie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElTagScopeTermEl {
    pub fn build(
        self,
    ) -> Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElTagScopeTermEl {
        Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElTagScopeTermEl {
            comparator: core::default::Default::default(),
            key: core::default::Default::default(),
            target: core::default::Default::default(),
            tag_values: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElTagScopeTermElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElTagScopeTermElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElTagScopeTermElRef {
        Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElTagScopeTermElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElTagScopeTermElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `comparator` after provisioning.\n"]
    pub fn comparator(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparator", self.base))
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target", self.base))
    }
    #[doc = "Get a reference to the value of field `tag_values` after provisioning.\n"]    pub fn tag_values (& self) -> ListRef < Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElTagScopeTermElTagValuesElRef >{
        ListRef::new(self.shared().clone(), format!("{}.tag_values", self.base))
    }
}
#[derive(Serialize, Default)]
struct Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElDynamic {
    simple_scope_term: Option<
        DynamicBlock<
            Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElSimpleScopeTermEl,
        >,
    >,
    tag_scope_term: Option<
        DynamicBlock<
            Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElTagScopeTermEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    simple_scope_term: Option<
        Vec<Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElSimpleScopeTermEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag_scope_term:
        Option<Vec<Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElTagScopeTermEl>>,
    dynamic: Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElDynamic,
}
impl Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndEl {
    #[doc = "Set the field `simple_scope_term`.\n"]
    pub fn set_simple_scope_term(
        mut self,
        v: impl Into<
            BlockAssignable<
                Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElSimpleScopeTermEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.simple_scope_term = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.simple_scope_term = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `tag_scope_term`.\n"]
    pub fn set_tag_scope_term(
        mut self,
        v: impl Into<
            BlockAssignable<
                Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElTagScopeTermEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.tag_scope_term = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.tag_scope_term = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndEl {
    type O = BlockAssignable<Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMacie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndEl {}
impl BuildMacie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndEl {
    pub fn build(self) -> Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndEl {
        Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndEl {
            simple_scope_term: core::default::Default::default(),
            tag_scope_term: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElRef {
        Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `simple_scope_term` after provisioning.\n"]
    pub fn simple_scope_term(
        &self,
    ) -> ListRef<Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElSimpleScopeTermElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.simple_scope_term", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `tag_scope_term` after provisioning.\n"]
    pub fn tag_scope_term(
        &self,
    ) -> ListRef<Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElTagScopeTermElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tag_scope_term", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElDynamic {
    and: Option<DynamicBlock<Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndEl>>,
}
#[derive(Serialize)]
pub struct Macie2ClassificationJobS3JobDefinitionElScopingElIncludesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    and: Option<Vec<Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndEl>>,
    dynamic: Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElDynamic,
}
impl Macie2ClassificationJobS3JobDefinitionElScopingElIncludesEl {
    #[doc = "Set the field `and`.\n"]
    pub fn set_and(
        mut self,
        v: impl Into<BlockAssignable<Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.and = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.and = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for Macie2ClassificationJobS3JobDefinitionElScopingElIncludesEl {
    type O = BlockAssignable<Macie2ClassificationJobS3JobDefinitionElScopingElIncludesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMacie2ClassificationJobS3JobDefinitionElScopingElIncludesEl {}
impl BuildMacie2ClassificationJobS3JobDefinitionElScopingElIncludesEl {
    pub fn build(self) -> Macie2ClassificationJobS3JobDefinitionElScopingElIncludesEl {
        Macie2ClassificationJobS3JobDefinitionElScopingElIncludesEl {
            and: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElRef {
        Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `and` after provisioning.\n"]
    pub fn and(
        &self,
    ) -> ListRef<Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElRef> {
        ListRef::new(self.shared().clone(), format!("{}.and", self.base))
    }
}
#[derive(Serialize, Default)]
struct Macie2ClassificationJobS3JobDefinitionElScopingElDynamic {
    excludes: Option<DynamicBlock<Macie2ClassificationJobS3JobDefinitionElScopingElExcludesEl>>,
    includes: Option<DynamicBlock<Macie2ClassificationJobS3JobDefinitionElScopingElIncludesEl>>,
}
#[derive(Serialize)]
pub struct Macie2ClassificationJobS3JobDefinitionElScopingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    excludes: Option<Vec<Macie2ClassificationJobS3JobDefinitionElScopingElExcludesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    includes: Option<Vec<Macie2ClassificationJobS3JobDefinitionElScopingElIncludesEl>>,
    dynamic: Macie2ClassificationJobS3JobDefinitionElScopingElDynamic,
}
impl Macie2ClassificationJobS3JobDefinitionElScopingEl {
    #[doc = "Set the field `excludes`.\n"]
    pub fn set_excludes(
        mut self,
        v: impl Into<BlockAssignable<Macie2ClassificationJobS3JobDefinitionElScopingElExcludesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.excludes = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.excludes = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `includes`.\n"]
    pub fn set_includes(
        mut self,
        v: impl Into<BlockAssignable<Macie2ClassificationJobS3JobDefinitionElScopingElIncludesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.includes = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.includes = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for Macie2ClassificationJobS3JobDefinitionElScopingEl {
    type O = BlockAssignable<Macie2ClassificationJobS3JobDefinitionElScopingEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMacie2ClassificationJobS3JobDefinitionElScopingEl {}
impl BuildMacie2ClassificationJobS3JobDefinitionElScopingEl {
    pub fn build(self) -> Macie2ClassificationJobS3JobDefinitionElScopingEl {
        Macie2ClassificationJobS3JobDefinitionElScopingEl {
            excludes: core::default::Default::default(),
            includes: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct Macie2ClassificationJobS3JobDefinitionElScopingElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Macie2ClassificationJobS3JobDefinitionElScopingElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Macie2ClassificationJobS3JobDefinitionElScopingElRef {
        Macie2ClassificationJobS3JobDefinitionElScopingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Macie2ClassificationJobS3JobDefinitionElScopingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `excludes` after provisioning.\n"]
    pub fn excludes(
        &self,
    ) -> ListRef<Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.excludes", self.base))
    }
    #[doc = "Get a reference to the value of field `includes` after provisioning.\n"]
    pub fn includes(
        &self,
    ) -> ListRef<Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.includes", self.base))
    }
}
#[derive(Serialize, Default)]
struct Macie2ClassificationJobS3JobDefinitionElDynamic {
    bucket_criteria: Option<DynamicBlock<Macie2ClassificationJobS3JobDefinitionElBucketCriteriaEl>>,
    bucket_definitions:
        Option<DynamicBlock<Macie2ClassificationJobS3JobDefinitionElBucketDefinitionsEl>>,
    scoping: Option<DynamicBlock<Macie2ClassificationJobS3JobDefinitionElScopingEl>>,
}
#[derive(Serialize)]
pub struct Macie2ClassificationJobS3JobDefinitionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_criteria: Option<Vec<Macie2ClassificationJobS3JobDefinitionElBucketCriteriaEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_definitions: Option<Vec<Macie2ClassificationJobS3JobDefinitionElBucketDefinitionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scoping: Option<Vec<Macie2ClassificationJobS3JobDefinitionElScopingEl>>,
    dynamic: Macie2ClassificationJobS3JobDefinitionElDynamic,
}
impl Macie2ClassificationJobS3JobDefinitionEl {
    #[doc = "Set the field `bucket_criteria`.\n"]
    pub fn set_bucket_criteria(
        mut self,
        v: impl Into<BlockAssignable<Macie2ClassificationJobS3JobDefinitionElBucketCriteriaEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.bucket_criteria = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.bucket_criteria = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `bucket_definitions`.\n"]
    pub fn set_bucket_definitions(
        mut self,
        v: impl Into<BlockAssignable<Macie2ClassificationJobS3JobDefinitionElBucketDefinitionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.bucket_definitions = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.bucket_definitions = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `scoping`.\n"]
    pub fn set_scoping(
        mut self,
        v: impl Into<BlockAssignable<Macie2ClassificationJobS3JobDefinitionElScopingEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.scoping = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.scoping = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for Macie2ClassificationJobS3JobDefinitionEl {
    type O = BlockAssignable<Macie2ClassificationJobS3JobDefinitionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMacie2ClassificationJobS3JobDefinitionEl {}
impl BuildMacie2ClassificationJobS3JobDefinitionEl {
    pub fn build(self) -> Macie2ClassificationJobS3JobDefinitionEl {
        Macie2ClassificationJobS3JobDefinitionEl {
            bucket_criteria: core::default::Default::default(),
            bucket_definitions: core::default::Default::default(),
            scoping: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct Macie2ClassificationJobS3JobDefinitionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Macie2ClassificationJobS3JobDefinitionElRef {
    fn new(shared: StackShared, base: String) -> Macie2ClassificationJobS3JobDefinitionElRef {
        Macie2ClassificationJobS3JobDefinitionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Macie2ClassificationJobS3JobDefinitionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bucket_criteria` after provisioning.\n"]
    pub fn bucket_criteria(
        &self,
    ) -> ListRef<Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.bucket_criteria", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `bucket_definitions` after provisioning.\n"]
    pub fn bucket_definitions(
        &self,
    ) -> ListRef<Macie2ClassificationJobS3JobDefinitionElBucketDefinitionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.bucket_definitions", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `scoping` after provisioning.\n"]
    pub fn scoping(&self) -> ListRef<Macie2ClassificationJobS3JobDefinitionElScopingElRef> {
        ListRef::new(self.shared().clone(), format!("{}.scoping", self.base))
    }
}
#[derive(Serialize)]
pub struct Macie2ClassificationJobScheduleFrequencyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    daily_schedule: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    monthly_schedule: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weekly_schedule: Option<PrimField<String>>,
}
impl Macie2ClassificationJobScheduleFrequencyEl {
    #[doc = "Set the field `daily_schedule`.\n"]
    pub fn set_daily_schedule(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.daily_schedule = Some(v.into());
        self
    }
    #[doc = "Set the field `monthly_schedule`.\n"]
    pub fn set_monthly_schedule(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.monthly_schedule = Some(v.into());
        self
    }
    #[doc = "Set the field `weekly_schedule`.\n"]
    pub fn set_weekly_schedule(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.weekly_schedule = Some(v.into());
        self
    }
}
impl ToListMappable for Macie2ClassificationJobScheduleFrequencyEl {
    type O = BlockAssignable<Macie2ClassificationJobScheduleFrequencyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMacie2ClassificationJobScheduleFrequencyEl {}
impl BuildMacie2ClassificationJobScheduleFrequencyEl {
    pub fn build(self) -> Macie2ClassificationJobScheduleFrequencyEl {
        Macie2ClassificationJobScheduleFrequencyEl {
            daily_schedule: core::default::Default::default(),
            monthly_schedule: core::default::Default::default(),
            weekly_schedule: core::default::Default::default(),
        }
    }
}
pub struct Macie2ClassificationJobScheduleFrequencyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Macie2ClassificationJobScheduleFrequencyElRef {
    fn new(shared: StackShared, base: String) -> Macie2ClassificationJobScheduleFrequencyElRef {
        Macie2ClassificationJobScheduleFrequencyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Macie2ClassificationJobScheduleFrequencyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `daily_schedule` after provisioning.\n"]
    pub fn daily_schedule(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.daily_schedule", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `monthly_schedule` after provisioning.\n"]
    pub fn monthly_schedule(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.monthly_schedule", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `weekly_schedule` after provisioning.\n"]
    pub fn weekly_schedule(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.weekly_schedule", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct Macie2ClassificationJobTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl Macie2ClassificationJobTimeoutsEl {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }
    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}
impl ToListMappable for Macie2ClassificationJobTimeoutsEl {
    type O = BlockAssignable<Macie2ClassificationJobTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMacie2ClassificationJobTimeoutsEl {}
impl BuildMacie2ClassificationJobTimeoutsEl {
    pub fn build(self) -> Macie2ClassificationJobTimeoutsEl {
        Macie2ClassificationJobTimeoutsEl {
            create: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct Macie2ClassificationJobTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Macie2ClassificationJobTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> Macie2ClassificationJobTimeoutsElRef {
        Macie2ClassificationJobTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Macie2ClassificationJobTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }
    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
#[derive(Serialize, Default)]
struct Macie2ClassificationJobDynamic {
    s3_job_definition: Option<DynamicBlock<Macie2ClassificationJobS3JobDefinitionEl>>,
    schedule_frequency: Option<DynamicBlock<Macie2ClassificationJobScheduleFrequencyEl>>,
}
