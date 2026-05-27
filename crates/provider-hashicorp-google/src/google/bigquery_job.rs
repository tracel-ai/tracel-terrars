use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct BigqueryJobData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    job_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    job_timeout_ms: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    copy: Option<Vec<BigqueryJobCopyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    extract: Option<Vec<BigqueryJobExtractEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    load: Option<Vec<BigqueryJobLoadEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    query: Option<Vec<BigqueryJobQueryEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<BigqueryJobTimeoutsEl>,
    dynamic: BigqueryJobDynamic,
}
struct BigqueryJob_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<BigqueryJobData>,
}
#[derive(Clone)]
pub struct BigqueryJob(Rc<BigqueryJob_>);
impl BigqueryJob {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(self, provider: &ProviderGoogle) -> Self {
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
    #[doc = "Set the field `job_timeout_ms`.\nJob timeout in milliseconds. If this time limit is exceeded, BigQuery may attempt to terminate the job."]
    pub fn set_job_timeout_ms(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().job_timeout_ms = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nThe labels associated with this job. You can use these to organize and group your jobs.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `location`.\nThe geographic location of the job. The default value is US."]
    pub fn set_location(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().location = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `copy`.\n"]
    pub fn set_copy(self, v: impl Into<BlockAssignable<BigqueryJobCopyEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().copy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.copy = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `extract`.\n"]
    pub fn set_extract(self, v: impl Into<BlockAssignable<BigqueryJobExtractEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().extract = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.extract = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `load`.\n"]
    pub fn set_load(self, v: impl Into<BlockAssignable<BigqueryJobLoadEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().load = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.load = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `query`.\n"]
    pub fn set_query(self, v: impl Into<BlockAssignable<BigqueryJobQueryEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().query = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.query = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<BigqueryJobTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `job_id` after provisioning.\nThe ID of the job. The ID must contain only letters (a-z, A-Z), numbers (0-9), underscores (_), or dashes (-). The maximum length is 1,024 characters."]
    pub fn job_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.job_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `job_timeout_ms` after provisioning.\nJob timeout in milliseconds. If this time limit is exceeded, BigQuery may attempt to terminate the job."]
    pub fn job_timeout_ms(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.job_timeout_ms", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `job_type` after provisioning.\nThe type of the job."]
    pub fn job_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.job_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nThe labels associated with this job. You can use these to organize and group your jobs.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe geographic location of the job. The default value is US."]
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
    #[doc = "Get a reference to the value of field `status` after provisioning.\nThe status of this job. Examine this value when polling an asynchronous job to see if the job is complete."]
    pub fn status(&self) -> ListRef<BigqueryJobStatusElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `user_email` after provisioning.\nEmail address of the user who ran the job."]
    pub fn user_email(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_email", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `copy` after provisioning.\n"]
    pub fn copy(&self) -> ListRef<BigqueryJobCopyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.copy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `extract` after provisioning.\n"]
    pub fn extract(&self) -> ListRef<BigqueryJobExtractElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.extract", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `load` after provisioning.\n"]
    pub fn load(&self) -> ListRef<BigqueryJobLoadElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.load", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `query` after provisioning.\n"]
    pub fn query(&self) -> ListRef<BigqueryJobQueryElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.query", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BigqueryJobTimeoutsElRef {
        BigqueryJobTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for BigqueryJob {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for BigqueryJob {}
impl ToListMappable for BigqueryJob {
    type O = ListRef<BigqueryJobRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for BigqueryJob_ {
    fn extract_resource_type(&self) -> String {
        "google_bigquery_job".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildBigqueryJob {
    pub tf_id: String,
    #[doc = "The ID of the job. The ID must contain only letters (a-z, A-Z), numbers (0-9), underscores (_), or dashes (-). The maximum length is 1,024 characters."]
    pub job_id: PrimField<String>,
}
impl BuildBigqueryJob {
    pub fn build(self, stack: &mut Stack) -> BigqueryJob {
        let out = BigqueryJob(Rc::new(BigqueryJob_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(BigqueryJobData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                job_id: self.job_id,
                job_timeout_ms: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: core::default::Default::default(),
                project: core::default::Default::default(),
                copy: core::default::Default::default(),
                extract: core::default::Default::default(),
                load: core::default::Default::default(),
                query: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct BigqueryJobRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryJobRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl BigqueryJobRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `job_id` after provisioning.\nThe ID of the job. The ID must contain only letters (a-z, A-Z), numbers (0-9), underscores (_), or dashes (-). The maximum length is 1,024 characters."]
    pub fn job_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.job_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `job_timeout_ms` after provisioning.\nJob timeout in milliseconds. If this time limit is exceeded, BigQuery may attempt to terminate the job."]
    pub fn job_timeout_ms(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.job_timeout_ms", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `job_type` after provisioning.\nThe type of the job."]
    pub fn job_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.job_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nThe labels associated with this job. You can use these to organize and group your jobs.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe geographic location of the job. The default value is US."]
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
    #[doc = "Get a reference to the value of field `status` after provisioning.\nThe status of this job. Examine this value when polling an asynchronous job to see if the job is complete."]
    pub fn status(&self) -> ListRef<BigqueryJobStatusElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `user_email` after provisioning.\nEmail address of the user who ran the job."]
    pub fn user_email(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_email", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `copy` after provisioning.\n"]
    pub fn copy(&self) -> ListRef<BigqueryJobCopyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.copy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `extract` after provisioning.\n"]
    pub fn extract(&self) -> ListRef<BigqueryJobExtractElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.extract", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `load` after provisioning.\n"]
    pub fn load(&self) -> ListRef<BigqueryJobLoadElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.load", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `query` after provisioning.\n"]
    pub fn query(&self) -> ListRef<BigqueryJobQueryElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.query", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BigqueryJobTimeoutsElRef {
        BigqueryJobTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct BigqueryJobStatusElErrorResultEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reason: Option<PrimField<String>>,
}
impl BigqueryJobStatusElErrorResultEl {
    #[doc = "Set the field `location`.\n"]
    pub fn set_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.location = Some(v.into());
        self
    }
    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message = Some(v.into());
        self
    }
    #[doc = "Set the field `reason`.\n"]
    pub fn set_reason(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.reason = Some(v.into());
        self
    }
}
impl ToListMappable for BigqueryJobStatusElErrorResultEl {
    type O = BlockAssignable<BigqueryJobStatusElErrorResultEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBigqueryJobStatusElErrorResultEl {}
impl BuildBigqueryJobStatusElErrorResultEl {
    pub fn build(self) -> BigqueryJobStatusElErrorResultEl {
        BigqueryJobStatusElErrorResultEl {
            location: core::default::Default::default(),
            message: core::default::Default::default(),
            reason: core::default::Default::default(),
        }
    }
}
pub struct BigqueryJobStatusElErrorResultElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryJobStatusElErrorResultElRef {
    fn new(shared: StackShared, base: String) -> BigqueryJobStatusElErrorResultElRef {
        BigqueryJobStatusElErrorResultElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BigqueryJobStatusElErrorResultElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }
    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
    #[doc = "Get a reference to the value of field `reason` after provisioning.\n"]
    pub fn reason(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.reason", self.base))
    }
}
#[derive(Serialize)]
pub struct BigqueryJobStatusElErrorsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reason: Option<PrimField<String>>,
}
impl BigqueryJobStatusElErrorsEl {
    #[doc = "Set the field `location`.\n"]
    pub fn set_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.location = Some(v.into());
        self
    }
    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message = Some(v.into());
        self
    }
    #[doc = "Set the field `reason`.\n"]
    pub fn set_reason(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.reason = Some(v.into());
        self
    }
}
impl ToListMappable for BigqueryJobStatusElErrorsEl {
    type O = BlockAssignable<BigqueryJobStatusElErrorsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBigqueryJobStatusElErrorsEl {}
impl BuildBigqueryJobStatusElErrorsEl {
    pub fn build(self) -> BigqueryJobStatusElErrorsEl {
        BigqueryJobStatusElErrorsEl {
            location: core::default::Default::default(),
            message: core::default::Default::default(),
            reason: core::default::Default::default(),
        }
    }
}
pub struct BigqueryJobStatusElErrorsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryJobStatusElErrorsElRef {
    fn new(shared: StackShared, base: String) -> BigqueryJobStatusElErrorsElRef {
        BigqueryJobStatusElErrorsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BigqueryJobStatusElErrorsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }
    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
    #[doc = "Get a reference to the value of field `reason` after provisioning.\n"]
    pub fn reason(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.reason", self.base))
    }
}
#[derive(Serialize)]
pub struct BigqueryJobStatusEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    error_result: Option<ListField<BigqueryJobStatusElErrorResultEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    errors: Option<ListField<BigqueryJobStatusElErrorsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
}
impl BigqueryJobStatusEl {
    #[doc = "Set the field `error_result`.\n"]
    pub fn set_error_result(
        mut self,
        v: impl Into<ListField<BigqueryJobStatusElErrorResultEl>>,
    ) -> Self {
        self.error_result = Some(v.into());
        self
    }
    #[doc = "Set the field `errors`.\n"]
    pub fn set_errors(mut self, v: impl Into<ListField<BigqueryJobStatusElErrorsEl>>) -> Self {
        self.errors = Some(v.into());
        self
    }
    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }
}
impl ToListMappable for BigqueryJobStatusEl {
    type O = BlockAssignable<BigqueryJobStatusEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBigqueryJobStatusEl {}
impl BuildBigqueryJobStatusEl {
    pub fn build(self) -> BigqueryJobStatusEl {
        BigqueryJobStatusEl {
            error_result: core::default::Default::default(),
            errors: core::default::Default::default(),
            state: core::default::Default::default(),
        }
    }
}
pub struct BigqueryJobStatusElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryJobStatusElRef {
    fn new(shared: StackShared, base: String) -> BigqueryJobStatusElRef {
        BigqueryJobStatusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BigqueryJobStatusElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `error_result` after provisioning.\n"]
    pub fn error_result(&self) -> ListRef<BigqueryJobStatusElErrorResultElRef> {
        ListRef::new(self.shared().clone(), format!("{}.error_result", self.base))
    }
    #[doc = "Get a reference to the value of field `errors` after provisioning.\n"]
    pub fn errors(&self) -> ListRef<BigqueryJobStatusElErrorsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.errors", self.base))
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}
#[derive(Serialize)]
pub struct BigqueryJobCopyElDestinationEncryptionConfigurationEl {
    kms_key_name: PrimField<String>,
}
impl BigqueryJobCopyElDestinationEncryptionConfigurationEl {}
impl ToListMappable for BigqueryJobCopyElDestinationEncryptionConfigurationEl {
    type O = BlockAssignable<BigqueryJobCopyElDestinationEncryptionConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBigqueryJobCopyElDestinationEncryptionConfigurationEl {
    #[doc = "Describes the Cloud KMS encryption key that will be used to protect destination BigQuery table.\nThe BigQuery Service Account associated with your project requires access to this encryption key."]
    pub kms_key_name: PrimField<String>,
}
impl BuildBigqueryJobCopyElDestinationEncryptionConfigurationEl {
    pub fn build(self) -> BigqueryJobCopyElDestinationEncryptionConfigurationEl {
        BigqueryJobCopyElDestinationEncryptionConfigurationEl {
            kms_key_name: self.kms_key_name,
        }
    }
}
pub struct BigqueryJobCopyElDestinationEncryptionConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryJobCopyElDestinationEncryptionConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BigqueryJobCopyElDestinationEncryptionConfigurationElRef {
        BigqueryJobCopyElDestinationEncryptionConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BigqueryJobCopyElDestinationEncryptionConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `kms_key_name` after provisioning.\nDescribes the Cloud KMS encryption key that will be used to protect destination BigQuery table.\nThe BigQuery Service Account associated with your project requires access to this encryption key."]
    pub fn kms_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.base))
    }
    #[doc = "Get a reference to the value of field `kms_key_version` after provisioning.\nDescribes the Cloud KMS encryption key version used to protect destination BigQuery table."]
    pub fn kms_key_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_version", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct BigqueryJobCopyElDestinationTableEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dataset_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project_id: Option<PrimField<String>>,
    table_id: PrimField<String>,
}
impl BigqueryJobCopyElDestinationTableEl {
    #[doc = "Set the field `dataset_id`.\nThe ID of the dataset containing this table."]
    pub fn set_dataset_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dataset_id = Some(v.into());
        self
    }
    #[doc = "Set the field `project_id`.\nThe ID of the project containing this table."]
    pub fn set_project_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.project_id = Some(v.into());
        self
    }
}
impl ToListMappable for BigqueryJobCopyElDestinationTableEl {
    type O = BlockAssignable<BigqueryJobCopyElDestinationTableEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBigqueryJobCopyElDestinationTableEl {
    #[doc = "The table. Can be specified '{{table_id}}' if 'project_id' and 'dataset_id' are also set,\nor of the form 'projects/{{project}}/datasets/{{dataset_id}}/tables/{{table_id}}' if not."]
    pub table_id: PrimField<String>,
}
impl BuildBigqueryJobCopyElDestinationTableEl {
    pub fn build(self) -> BigqueryJobCopyElDestinationTableEl {
        BigqueryJobCopyElDestinationTableEl {
            dataset_id: core::default::Default::default(),
            project_id: core::default::Default::default(),
            table_id: self.table_id,
        }
    }
}
pub struct BigqueryJobCopyElDestinationTableElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryJobCopyElDestinationTableElRef {
    fn new(shared: StackShared, base: String) -> BigqueryJobCopyElDestinationTableElRef {
        BigqueryJobCopyElDestinationTableElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BigqueryJobCopyElDestinationTableElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `dataset_id` after provisioning.\nThe ID of the dataset containing this table."]
    pub fn dataset_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dataset_id", self.base))
    }
    #[doc = "Get a reference to the value of field `project_id` after provisioning.\nThe ID of the project containing this table."]
    pub fn project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }
    #[doc = "Get a reference to the value of field `table_id` after provisioning.\nThe table. Can be specified '{{table_id}}' if 'project_id' and 'dataset_id' are also set,\nor of the form 'projects/{{project}}/datasets/{{dataset_id}}/tables/{{table_id}}' if not."]
    pub fn table_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.table_id", self.base))
    }
}
#[derive(Serialize)]
pub struct BigqueryJobCopyElSourceTablesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dataset_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project_id: Option<PrimField<String>>,
    table_id: PrimField<String>,
}
impl BigqueryJobCopyElSourceTablesEl {
    #[doc = "Set the field `dataset_id`.\nThe ID of the dataset containing this table."]
    pub fn set_dataset_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dataset_id = Some(v.into());
        self
    }
    #[doc = "Set the field `project_id`.\nThe ID of the project containing this table."]
    pub fn set_project_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.project_id = Some(v.into());
        self
    }
}
impl ToListMappable for BigqueryJobCopyElSourceTablesEl {
    type O = BlockAssignable<BigqueryJobCopyElSourceTablesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBigqueryJobCopyElSourceTablesEl {
    #[doc = "The table. Can be specified '{{table_id}}' if 'project_id' and 'dataset_id' are also set,\nor of the form 'projects/{{project}}/datasets/{{dataset_id}}/tables/{{table_id}}' if not."]
    pub table_id: PrimField<String>,
}
impl BuildBigqueryJobCopyElSourceTablesEl {
    pub fn build(self) -> BigqueryJobCopyElSourceTablesEl {
        BigqueryJobCopyElSourceTablesEl {
            dataset_id: core::default::Default::default(),
            project_id: core::default::Default::default(),
            table_id: self.table_id,
        }
    }
}
pub struct BigqueryJobCopyElSourceTablesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryJobCopyElSourceTablesElRef {
    fn new(shared: StackShared, base: String) -> BigqueryJobCopyElSourceTablesElRef {
        BigqueryJobCopyElSourceTablesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BigqueryJobCopyElSourceTablesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `dataset_id` after provisioning.\nThe ID of the dataset containing this table."]
    pub fn dataset_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dataset_id", self.base))
    }
    #[doc = "Get a reference to the value of field `project_id` after provisioning.\nThe ID of the project containing this table."]
    pub fn project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }
    #[doc = "Get a reference to the value of field `table_id` after provisioning.\nThe table. Can be specified '{{table_id}}' if 'project_id' and 'dataset_id' are also set,\nor of the form 'projects/{{project}}/datasets/{{dataset_id}}/tables/{{table_id}}' if not."]
    pub fn table_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.table_id", self.base))
    }
}
#[derive(Serialize, Default)]
struct BigqueryJobCopyElDynamic {
    destination_encryption_configuration:
        Option<DynamicBlock<BigqueryJobCopyElDestinationEncryptionConfigurationEl>>,
    destination_table: Option<DynamicBlock<BigqueryJobCopyElDestinationTableEl>>,
    source_tables: Option<DynamicBlock<BigqueryJobCopyElSourceTablesEl>>,
}
#[derive(Serialize)]
pub struct BigqueryJobCopyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create_disposition: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    write_disposition: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_encryption_configuration:
        Option<Vec<BigqueryJobCopyElDestinationEncryptionConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_table: Option<Vec<BigqueryJobCopyElDestinationTableEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_tables: Option<Vec<BigqueryJobCopyElSourceTablesEl>>,
    dynamic: BigqueryJobCopyElDynamic,
}
impl BigqueryJobCopyEl {
    #[doc = "Set the field `create_disposition`.\nSpecifies whether the job is allowed to create new tables. The following values are supported:\nCREATE_IF_NEEDED: If the table does not exist, BigQuery creates the table.\nCREATE_NEVER: The table must already exist. If it does not, a 'notFound' error is returned in the job result.\nCreation, truncation and append actions occur as one atomic update upon job completion Default value: \"CREATE_IF_NEEDED\" Possible values: [\"CREATE_IF_NEEDED\", \"CREATE_NEVER\"]"]
    pub fn set_create_disposition(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create_disposition = Some(v.into());
        self
    }
    #[doc = "Set the field `write_disposition`.\nSpecifies the action that occurs if the destination table already exists. The following values are supported:\nWRITE_TRUNCATE: If the table already exists, BigQuery overwrites the table data and uses the schema from the query result.\nWRITE_APPEND: If the table already exists, BigQuery appends the data to the table.\nWRITE_EMPTY: If the table already exists and contains data, a 'duplicate' error is returned in the job result.\nEach action is atomic and only occurs if BigQuery is able to complete the job successfully.\nCreation, truncation and append actions occur as one atomic update upon job completion. Default value: \"WRITE_EMPTY\" Possible values: [\"WRITE_TRUNCATE\", \"WRITE_APPEND\", \"WRITE_EMPTY\"]"]
    pub fn set_write_disposition(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.write_disposition = Some(v.into());
        self
    }
    #[doc = "Set the field `destination_encryption_configuration`.\n"]
    pub fn set_destination_encryption_configuration(
        mut self,
        v: impl Into<BlockAssignable<BigqueryJobCopyElDestinationEncryptionConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.destination_encryption_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.destination_encryption_configuration = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `destination_table`.\n"]
    pub fn set_destination_table(
        mut self,
        v: impl Into<BlockAssignable<BigqueryJobCopyElDestinationTableEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.destination_table = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.destination_table = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `source_tables`.\n"]
    pub fn set_source_tables(
        mut self,
        v: impl Into<BlockAssignable<BigqueryJobCopyElSourceTablesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.source_tables = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.source_tables = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for BigqueryJobCopyEl {
    type O = BlockAssignable<BigqueryJobCopyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBigqueryJobCopyEl {}
impl BuildBigqueryJobCopyEl {
    pub fn build(self) -> BigqueryJobCopyEl {
        BigqueryJobCopyEl {
            create_disposition: core::default::Default::default(),
            write_disposition: core::default::Default::default(),
            destination_encryption_configuration: core::default::Default::default(),
            destination_table: core::default::Default::default(),
            source_tables: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct BigqueryJobCopyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryJobCopyElRef {
    fn new(shared: StackShared, base: String) -> BigqueryJobCopyElRef {
        BigqueryJobCopyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BigqueryJobCopyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_disposition` after provisioning.\nSpecifies whether the job is allowed to create new tables. The following values are supported:\nCREATE_IF_NEEDED: If the table does not exist, BigQuery creates the table.\nCREATE_NEVER: The table must already exist. If it does not, a 'notFound' error is returned in the job result.\nCreation, truncation and append actions occur as one atomic update upon job completion Default value: \"CREATE_IF_NEEDED\" Possible values: [\"CREATE_IF_NEEDED\", \"CREATE_NEVER\"]"]
    pub fn create_disposition(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_disposition", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `write_disposition` after provisioning.\nSpecifies the action that occurs if the destination table already exists. The following values are supported:\nWRITE_TRUNCATE: If the table already exists, BigQuery overwrites the table data and uses the schema from the query result.\nWRITE_APPEND: If the table already exists, BigQuery appends the data to the table.\nWRITE_EMPTY: If the table already exists and contains data, a 'duplicate' error is returned in the job result.\nEach action is atomic and only occurs if BigQuery is able to complete the job successfully.\nCreation, truncation and append actions occur as one atomic update upon job completion. Default value: \"WRITE_EMPTY\" Possible values: [\"WRITE_TRUNCATE\", \"WRITE_APPEND\", \"WRITE_EMPTY\"]"]
    pub fn write_disposition(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.write_disposition", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `destination_encryption_configuration` after provisioning.\n"]
    pub fn destination_encryption_configuration(
        &self,
    ) -> ListRef<BigqueryJobCopyElDestinationEncryptionConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.destination_encryption_configuration", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `destination_table` after provisioning.\n"]
    pub fn destination_table(&self) -> ListRef<BigqueryJobCopyElDestinationTableElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.destination_table", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `source_tables` after provisioning.\n"]
    pub fn source_tables(&self) -> ListRef<BigqueryJobCopyElSourceTablesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source_tables", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct BigqueryJobExtractElSourceModelEl {
    dataset_id: PrimField<String>,
    model_id: PrimField<String>,
    project_id: PrimField<String>,
}
impl BigqueryJobExtractElSourceModelEl {}
impl ToListMappable for BigqueryJobExtractElSourceModelEl {
    type O = BlockAssignable<BigqueryJobExtractElSourceModelEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBigqueryJobExtractElSourceModelEl {
    #[doc = "The ID of the dataset containing this model."]
    pub dataset_id: PrimField<String>,
    #[doc = "The ID of the model."]
    pub model_id: PrimField<String>,
    #[doc = "The ID of the project containing this model."]
    pub project_id: PrimField<String>,
}
impl BuildBigqueryJobExtractElSourceModelEl {
    pub fn build(self) -> BigqueryJobExtractElSourceModelEl {
        BigqueryJobExtractElSourceModelEl {
            dataset_id: self.dataset_id,
            model_id: self.model_id,
            project_id: self.project_id,
        }
    }
}
pub struct BigqueryJobExtractElSourceModelElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryJobExtractElSourceModelElRef {
    fn new(shared: StackShared, base: String) -> BigqueryJobExtractElSourceModelElRef {
        BigqueryJobExtractElSourceModelElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BigqueryJobExtractElSourceModelElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `dataset_id` after provisioning.\nThe ID of the dataset containing this model."]
    pub fn dataset_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dataset_id", self.base))
    }
    #[doc = "Get a reference to the value of field `model_id` after provisioning.\nThe ID of the model."]
    pub fn model_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.model_id", self.base))
    }
    #[doc = "Get a reference to the value of field `project_id` after provisioning.\nThe ID of the project containing this model."]
    pub fn project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }
}
#[derive(Serialize)]
pub struct BigqueryJobExtractElSourceTableEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dataset_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project_id: Option<PrimField<String>>,
    table_id: PrimField<String>,
}
impl BigqueryJobExtractElSourceTableEl {
    #[doc = "Set the field `dataset_id`.\nThe ID of the dataset containing this table."]
    pub fn set_dataset_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dataset_id = Some(v.into());
        self
    }
    #[doc = "Set the field `project_id`.\nThe ID of the project containing this table."]
    pub fn set_project_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.project_id = Some(v.into());
        self
    }
}
impl ToListMappable for BigqueryJobExtractElSourceTableEl {
    type O = BlockAssignable<BigqueryJobExtractElSourceTableEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBigqueryJobExtractElSourceTableEl {
    #[doc = "The table. Can be specified '{{table_id}}' if 'project_id' and 'dataset_id' are also set,\nor of the form 'projects/{{project}}/datasets/{{dataset_id}}/tables/{{table_id}}' if not."]
    pub table_id: PrimField<String>,
}
impl BuildBigqueryJobExtractElSourceTableEl {
    pub fn build(self) -> BigqueryJobExtractElSourceTableEl {
        BigqueryJobExtractElSourceTableEl {
            dataset_id: core::default::Default::default(),
            project_id: core::default::Default::default(),
            table_id: self.table_id,
        }
    }
}
pub struct BigqueryJobExtractElSourceTableElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryJobExtractElSourceTableElRef {
    fn new(shared: StackShared, base: String) -> BigqueryJobExtractElSourceTableElRef {
        BigqueryJobExtractElSourceTableElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BigqueryJobExtractElSourceTableElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `dataset_id` after provisioning.\nThe ID of the dataset containing this table."]
    pub fn dataset_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dataset_id", self.base))
    }
    #[doc = "Get a reference to the value of field `project_id` after provisioning.\nThe ID of the project containing this table."]
    pub fn project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }
    #[doc = "Get a reference to the value of field `table_id` after provisioning.\nThe table. Can be specified '{{table_id}}' if 'project_id' and 'dataset_id' are also set,\nor of the form 'projects/{{project}}/datasets/{{dataset_id}}/tables/{{table_id}}' if not."]
    pub fn table_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.table_id", self.base))
    }
}
#[derive(Serialize, Default)]
struct BigqueryJobExtractElDynamic {
    source_model: Option<DynamicBlock<BigqueryJobExtractElSourceModelEl>>,
    source_table: Option<DynamicBlock<BigqueryJobExtractElSourceTableEl>>,
}
#[derive(Serialize)]
pub struct BigqueryJobExtractEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    compression: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_format: Option<PrimField<String>>,
    destination_uris: ListField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    field_delimiter: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    print_header: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_avro_logical_types: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_model: Option<Vec<BigqueryJobExtractElSourceModelEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_table: Option<Vec<BigqueryJobExtractElSourceTableEl>>,
    dynamic: BigqueryJobExtractElDynamic,
}
impl BigqueryJobExtractEl {
    #[doc = "Set the field `compression`.\nThe compression type to use for exported files. Possible values include GZIP, DEFLATE, SNAPPY, and NONE.\nThe default value is NONE. DEFLATE and SNAPPY are only supported for Avro."]
    pub fn set_compression(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.compression = Some(v.into());
        self
    }
    #[doc = "Set the field `destination_format`.\nThe exported file format. Possible values include CSV, NEWLINE_DELIMITED_JSON and AVRO for tables and SAVED_MODEL for models.\nThe default value for tables is CSV. Tables with nested or repeated fields cannot be exported as CSV.\nThe default value for models is SAVED_MODEL."]
    pub fn set_destination_format(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.destination_format = Some(v.into());
        self
    }
    #[doc = "Set the field `field_delimiter`.\nWhen extracting data in CSV format, this defines the delimiter to use between fields in the exported data.\nDefault is ','"]
    pub fn set_field_delimiter(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.field_delimiter = Some(v.into());
        self
    }
    #[doc = "Set the field `print_header`.\nWhether to print out a header row in the results. Default is true."]
    pub fn set_print_header(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.print_header = Some(v.into());
        self
    }
    #[doc = "Set the field `use_avro_logical_types`.\nWhether to use logical types when extracting to AVRO format."]
    pub fn set_use_avro_logical_types(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.use_avro_logical_types = Some(v.into());
        self
    }
    #[doc = "Set the field `source_model`.\n"]
    pub fn set_source_model(
        mut self,
        v: impl Into<BlockAssignable<BigqueryJobExtractElSourceModelEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.source_model = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.source_model = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `source_table`.\n"]
    pub fn set_source_table(
        mut self,
        v: impl Into<BlockAssignable<BigqueryJobExtractElSourceTableEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.source_table = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.source_table = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for BigqueryJobExtractEl {
    type O = BlockAssignable<BigqueryJobExtractEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBigqueryJobExtractEl {
    #[doc = "A list of fully-qualified Google Cloud Storage URIs where the extracted table should be written."]
    pub destination_uris: ListField<PrimField<String>>,
}
impl BuildBigqueryJobExtractEl {
    pub fn build(self) -> BigqueryJobExtractEl {
        BigqueryJobExtractEl {
            compression: core::default::Default::default(),
            destination_format: core::default::Default::default(),
            destination_uris: self.destination_uris,
            field_delimiter: core::default::Default::default(),
            print_header: core::default::Default::default(),
            use_avro_logical_types: core::default::Default::default(),
            source_model: core::default::Default::default(),
            source_table: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct BigqueryJobExtractElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryJobExtractElRef {
    fn new(shared: StackShared, base: String) -> BigqueryJobExtractElRef {
        BigqueryJobExtractElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BigqueryJobExtractElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `compression` after provisioning.\nThe compression type to use for exported files. Possible values include GZIP, DEFLATE, SNAPPY, and NONE.\nThe default value is NONE. DEFLATE and SNAPPY are only supported for Avro."]
    pub fn compression(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.compression", self.base))
    }
    #[doc = "Get a reference to the value of field `destination_format` after provisioning.\nThe exported file format. Possible values include CSV, NEWLINE_DELIMITED_JSON and AVRO for tables and SAVED_MODEL for models.\nThe default value for tables is CSV. Tables with nested or repeated fields cannot be exported as CSV.\nThe default value for models is SAVED_MODEL."]
    pub fn destination_format(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.destination_format", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `destination_uris` after provisioning.\nA list of fully-qualified Google Cloud Storage URIs where the extracted table should be written."]
    pub fn destination_uris(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.destination_uris", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `field_delimiter` after provisioning.\nWhen extracting data in CSV format, this defines the delimiter to use between fields in the exported data.\nDefault is ','"]
    pub fn field_delimiter(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.field_delimiter", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `print_header` after provisioning.\nWhether to print out a header row in the results. Default is true."]
    pub fn print_header(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.print_header", self.base))
    }
    #[doc = "Get a reference to the value of field `use_avro_logical_types` after provisioning.\nWhether to use logical types when extracting to AVRO format."]
    pub fn use_avro_logical_types(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.use_avro_logical_types", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `source_model` after provisioning.\n"]
    pub fn source_model(&self) -> ListRef<BigqueryJobExtractElSourceModelElRef> {
        ListRef::new(self.shared().clone(), format!("{}.source_model", self.base))
    }
    #[doc = "Get a reference to the value of field `source_table` after provisioning.\n"]
    pub fn source_table(&self) -> ListRef<BigqueryJobExtractElSourceTableElRef> {
        ListRef::new(self.shared().clone(), format!("{}.source_table", self.base))
    }
}
#[derive(Serialize)]
pub struct BigqueryJobLoadElDestinationEncryptionConfigurationEl {
    kms_key_name: PrimField<String>,
}
impl BigqueryJobLoadElDestinationEncryptionConfigurationEl {}
impl ToListMappable for BigqueryJobLoadElDestinationEncryptionConfigurationEl {
    type O = BlockAssignable<BigqueryJobLoadElDestinationEncryptionConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBigqueryJobLoadElDestinationEncryptionConfigurationEl {
    #[doc = "Describes the Cloud KMS encryption key that will be used to protect destination BigQuery table.\nThe BigQuery Service Account associated with your project requires access to this encryption key."]
    pub kms_key_name: PrimField<String>,
}
impl BuildBigqueryJobLoadElDestinationEncryptionConfigurationEl {
    pub fn build(self) -> BigqueryJobLoadElDestinationEncryptionConfigurationEl {
        BigqueryJobLoadElDestinationEncryptionConfigurationEl {
            kms_key_name: self.kms_key_name,
        }
    }
}
pub struct BigqueryJobLoadElDestinationEncryptionConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryJobLoadElDestinationEncryptionConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BigqueryJobLoadElDestinationEncryptionConfigurationElRef {
        BigqueryJobLoadElDestinationEncryptionConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BigqueryJobLoadElDestinationEncryptionConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `kms_key_name` after provisioning.\nDescribes the Cloud KMS encryption key that will be used to protect destination BigQuery table.\nThe BigQuery Service Account associated with your project requires access to this encryption key."]
    pub fn kms_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.base))
    }
    #[doc = "Get a reference to the value of field `kms_key_version` after provisioning.\nDescribes the Cloud KMS encryption key version used to protect destination BigQuery table."]
    pub fn kms_key_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_version", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct BigqueryJobLoadElDestinationTableEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dataset_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project_id: Option<PrimField<String>>,
    table_id: PrimField<String>,
}
impl BigqueryJobLoadElDestinationTableEl {
    #[doc = "Set the field `dataset_id`.\nThe ID of the dataset containing this table."]
    pub fn set_dataset_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dataset_id = Some(v.into());
        self
    }
    #[doc = "Set the field `project_id`.\nThe ID of the project containing this table."]
    pub fn set_project_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.project_id = Some(v.into());
        self
    }
}
impl ToListMappable for BigqueryJobLoadElDestinationTableEl {
    type O = BlockAssignable<BigqueryJobLoadElDestinationTableEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBigqueryJobLoadElDestinationTableEl {
    #[doc = "The table. Can be specified '{{table_id}}' if 'project_id' and 'dataset_id' are also set,\nor of the form 'projects/{{project}}/datasets/{{dataset_id}}/tables/{{table_id}}' if not."]
    pub table_id: PrimField<String>,
}
impl BuildBigqueryJobLoadElDestinationTableEl {
    pub fn build(self) -> BigqueryJobLoadElDestinationTableEl {
        BigqueryJobLoadElDestinationTableEl {
            dataset_id: core::default::Default::default(),
            project_id: core::default::Default::default(),
            table_id: self.table_id,
        }
    }
}
pub struct BigqueryJobLoadElDestinationTableElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryJobLoadElDestinationTableElRef {
    fn new(shared: StackShared, base: String) -> BigqueryJobLoadElDestinationTableElRef {
        BigqueryJobLoadElDestinationTableElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BigqueryJobLoadElDestinationTableElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `dataset_id` after provisioning.\nThe ID of the dataset containing this table."]
    pub fn dataset_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dataset_id", self.base))
    }
    #[doc = "Get a reference to the value of field `project_id` after provisioning.\nThe ID of the project containing this table."]
    pub fn project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }
    #[doc = "Get a reference to the value of field `table_id` after provisioning.\nThe table. Can be specified '{{table_id}}' if 'project_id' and 'dataset_id' are also set,\nor of the form 'projects/{{project}}/datasets/{{dataset_id}}/tables/{{table_id}}' if not."]
    pub fn table_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.table_id", self.base))
    }
}
#[derive(Serialize)]
pub struct BigqueryJobLoadElParquetOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_list_inference: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enum_as_string: Option<PrimField<bool>>,
}
impl BigqueryJobLoadElParquetOptionsEl {
    #[doc = "Set the field `enable_list_inference`.\nIf sourceFormat is set to PARQUET, indicates whether to use schema inference specifically for Parquet LIST logical type."]
    pub fn set_enable_list_inference(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_list_inference = Some(v.into());
        self
    }
    #[doc = "Set the field `enum_as_string`.\nIf sourceFormat is set to PARQUET, indicates whether to infer Parquet ENUM logical type as STRING instead of BYTES by default."]
    pub fn set_enum_as_string(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enum_as_string = Some(v.into());
        self
    }
}
impl ToListMappable for BigqueryJobLoadElParquetOptionsEl {
    type O = BlockAssignable<BigqueryJobLoadElParquetOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBigqueryJobLoadElParquetOptionsEl {}
impl BuildBigqueryJobLoadElParquetOptionsEl {
    pub fn build(self) -> BigqueryJobLoadElParquetOptionsEl {
        BigqueryJobLoadElParquetOptionsEl {
            enable_list_inference: core::default::Default::default(),
            enum_as_string: core::default::Default::default(),
        }
    }
}
pub struct BigqueryJobLoadElParquetOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryJobLoadElParquetOptionsElRef {
    fn new(shared: StackShared, base: String) -> BigqueryJobLoadElParquetOptionsElRef {
        BigqueryJobLoadElParquetOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BigqueryJobLoadElParquetOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enable_list_inference` after provisioning.\nIf sourceFormat is set to PARQUET, indicates whether to use schema inference specifically for Parquet LIST logical type."]
    pub fn enable_list_inference(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_list_inference", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enum_as_string` after provisioning.\nIf sourceFormat is set to PARQUET, indicates whether to infer Parquet ENUM logical type as STRING instead of BYTES by default."]
    pub fn enum_as_string(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enum_as_string", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct BigqueryJobLoadElTimePartitioningEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    expiration_ms: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    field: Option<PrimField<String>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}
impl BigqueryJobLoadElTimePartitioningEl {
    #[doc = "Set the field `expiration_ms`.\nNumber of milliseconds for which to keep the storage for a partition. A wrapper is used here because 0 is an invalid value."]
    pub fn set_expiration_ms(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.expiration_ms = Some(v.into());
        self
    }
    #[doc = "Set the field `field`.\nIf not set, the table is partitioned by pseudo column '_PARTITIONTIME'; if set, the table is partitioned by this field.\nThe field must be a top-level TIMESTAMP or DATE field. Its mode must be NULLABLE or REQUIRED.\nA wrapper is used here because an empty string is an invalid value."]
    pub fn set_field(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.field = Some(v.into());
        self
    }
}
impl ToListMappable for BigqueryJobLoadElTimePartitioningEl {
    type O = BlockAssignable<BigqueryJobLoadElTimePartitioningEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBigqueryJobLoadElTimePartitioningEl {
    #[doc = "The only type supported is DAY, which will generate one partition per day. Providing an empty string used to cause an error,\nbut in OnePlatform the field will be treated as unset."]
    pub type_: PrimField<String>,
}
impl BuildBigqueryJobLoadElTimePartitioningEl {
    pub fn build(self) -> BigqueryJobLoadElTimePartitioningEl {
        BigqueryJobLoadElTimePartitioningEl {
            expiration_ms: core::default::Default::default(),
            field: core::default::Default::default(),
            type_: self.type_,
        }
    }
}
pub struct BigqueryJobLoadElTimePartitioningElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryJobLoadElTimePartitioningElRef {
    fn new(shared: StackShared, base: String) -> BigqueryJobLoadElTimePartitioningElRef {
        BigqueryJobLoadElTimePartitioningElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BigqueryJobLoadElTimePartitioningElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `expiration_ms` after provisioning.\nNumber of milliseconds for which to keep the storage for a partition. A wrapper is used here because 0 is an invalid value."]
    pub fn expiration_ms(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.expiration_ms", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `field` after provisioning.\nIf not set, the table is partitioned by pseudo column '_PARTITIONTIME'; if set, the table is partitioned by this field.\nThe field must be a top-level TIMESTAMP or DATE field. Its mode must be NULLABLE or REQUIRED.\nA wrapper is used here because an empty string is an invalid value."]
    pub fn field(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.field", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe only type supported is DAY, which will generate one partition per day. Providing an empty string used to cause an error,\nbut in OnePlatform the field will be treated as unset."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize, Default)]
struct BigqueryJobLoadElDynamic {
    destination_encryption_configuration:
        Option<DynamicBlock<BigqueryJobLoadElDestinationEncryptionConfigurationEl>>,
    destination_table: Option<DynamicBlock<BigqueryJobLoadElDestinationTableEl>>,
    parquet_options: Option<DynamicBlock<BigqueryJobLoadElParquetOptionsEl>>,
    time_partitioning: Option<DynamicBlock<BigqueryJobLoadElTimePartitioningEl>>,
}
#[derive(Serialize)]
pub struct BigqueryJobLoadEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_jagged_rows: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_quoted_newlines: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    autodetect: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    create_disposition: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encoding: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    field_delimiter: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ignore_unknown_values: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    json_extension: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_bad_records: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    null_marker: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    projection_fields: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    quote: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schema_update_options: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    skip_leading_rows: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_format: Option<PrimField<String>>,
    source_uris: ListField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    write_disposition: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_encryption_configuration:
        Option<Vec<BigqueryJobLoadElDestinationEncryptionConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_table: Option<Vec<BigqueryJobLoadElDestinationTableEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parquet_options: Option<Vec<BigqueryJobLoadElParquetOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time_partitioning: Option<Vec<BigqueryJobLoadElTimePartitioningEl>>,
    dynamic: BigqueryJobLoadElDynamic,
}
impl BigqueryJobLoadEl {
    #[doc = "Set the field `allow_jagged_rows`.\nAccept rows that are missing trailing optional columns. The missing values are treated as nulls.\nIf false, records with missing trailing columns are treated as bad records, and if there are too many bad records,\nan invalid error is returned in the job result. The default value is false. Only applicable to CSV, ignored for other formats."]
    pub fn set_allow_jagged_rows(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.allow_jagged_rows = Some(v.into());
        self
    }
    #[doc = "Set the field `allow_quoted_newlines`.\nIndicates if BigQuery should allow quoted data sections that contain newline characters in a CSV file.\nThe default value is false."]
    pub fn set_allow_quoted_newlines(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.allow_quoted_newlines = Some(v.into());
        self
    }
    #[doc = "Set the field `autodetect`.\nIndicates if we should automatically infer the options and schema for CSV and JSON sources."]
    pub fn set_autodetect(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.autodetect = Some(v.into());
        self
    }
    #[doc = "Set the field `create_disposition`.\nSpecifies whether the job is allowed to create new tables. The following values are supported:\nCREATE_IF_NEEDED: If the table does not exist, BigQuery creates the table.\nCREATE_NEVER: The table must already exist. If it does not, a 'notFound' error is returned in the job result.\nCreation, truncation and append actions occur as one atomic update upon job completion Default value: \"CREATE_IF_NEEDED\" Possible values: [\"CREATE_IF_NEEDED\", \"CREATE_NEVER\"]"]
    pub fn set_create_disposition(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create_disposition = Some(v.into());
        self
    }
    #[doc = "Set the field `encoding`.\nThe character encoding of the data. The supported values are UTF-8 or ISO-8859-1.\nThe default value is UTF-8. BigQuery decodes the data after the raw, binary data\nhas been split using the values of the quote and fieldDelimiter properties."]
    pub fn set_encoding(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.encoding = Some(v.into());
        self
    }
    #[doc = "Set the field `field_delimiter`.\nThe separator for fields in a CSV file. The separator can be any ISO-8859-1 single-byte character.\nTo use a character in the range 128-255, you must encode the character as UTF8. BigQuery converts\nthe string to ISO-8859-1 encoding, and then uses the first byte of the encoded string to split the\ndata in its raw, binary state. BigQuery also supports the escape sequence \"\\t\" to specify a tab separator.\nThe default value is a comma (',')."]
    pub fn set_field_delimiter(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.field_delimiter = Some(v.into());
        self
    }
    #[doc = "Set the field `ignore_unknown_values`.\nIndicates if BigQuery should allow extra values that are not represented in the table schema.\nIf true, the extra values are ignored. If false, records with extra columns are treated as bad records,\nand if there are too many bad records, an invalid error is returned in the job result.\nThe default value is false. The sourceFormat property determines what BigQuery treats as an extra value:\nCSV: Trailing columns\nJSON: Named values that don't match any column names"]
    pub fn set_ignore_unknown_values(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.ignore_unknown_values = Some(v.into());
        self
    }
    #[doc = "Set the field `json_extension`.\nIf sourceFormat is set to newline-delimited JSON, indicates whether it should be processed as a JSON variant such as GeoJSON.\nFor a sourceFormat other than JSON, omit this field. If the sourceFormat is newline-delimited JSON: - for newline-delimited\nGeoJSON: set to GEOJSON."]
    pub fn set_json_extension(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.json_extension = Some(v.into());
        self
    }
    #[doc = "Set the field `max_bad_records`.\nThe maximum number of bad records that BigQuery can ignore when running the job. If the number of bad records exceeds this value,\nan invalid error is returned in the job result. The default value is 0, which requires that all records are valid."]
    pub fn set_max_bad_records(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_bad_records = Some(v.into());
        self
    }
    #[doc = "Set the field `null_marker`.\nSpecifies a string that represents a null value in a CSV file. For example, if you specify \"\\N\", BigQuery interprets \"\\N\" as a null value\nwhen loading a CSV file. The default value is the empty string. If you set this property to a custom value, BigQuery throws an error if an\nempty string is present for all data types except for STRING and BYTE. For STRING and BYTE columns, BigQuery interprets the empty string as\nan empty value."]
    pub fn set_null_marker(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.null_marker = Some(v.into());
        self
    }
    #[doc = "Set the field `projection_fields`.\nIf sourceFormat is set to \"DATASTORE_BACKUP\", indicates which entity properties to load into BigQuery from a Cloud Datastore backup.\nProperty names are case sensitive and must be top-level properties. If no properties are specified, BigQuery loads all properties.\nIf any named property isn't found in the Cloud Datastore backup, an invalid error is returned in the job result."]
    pub fn set_projection_fields(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.projection_fields = Some(v.into());
        self
    }
    #[doc = "Set the field `quote`.\nThe value that is used to quote data sections in a CSV file. BigQuery converts the string to ISO-8859-1 encoding,\nand then uses the first byte of the encoded string to split the data in its raw, binary state.\nThe default value is a double-quote ('\"'). If your data does not contain quoted sections, set the property value to an empty string.\nIf your data contains quoted newline characters, you must also set the allowQuotedNewlines property to true."]
    pub fn set_quote(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.quote = Some(v.into());
        self
    }
    #[doc = "Set the field `schema_update_options`.\nAllows the schema of the destination table to be updated as a side effect of the load job if a schema is autodetected or\nsupplied in the job configuration. Schema update options are supported in two cases: when writeDisposition is WRITE_APPEND;\nwhen writeDisposition is WRITE_TRUNCATE and the destination table is a partition of a table, specified by partition decorators.\nFor normal tables, WRITE_TRUNCATE will always overwrite the schema. One or more of the following values are specified:\nALLOW_FIELD_ADDITION: allow adding a nullable field to the schema.\nALLOW_FIELD_RELAXATION: allow relaxing a required field in the original schema to nullable."]
    pub fn set_schema_update_options(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.schema_update_options = Some(v.into());
        self
    }
    #[doc = "Set the field `skip_leading_rows`.\nThe number of rows at the top of a CSV file that BigQuery will skip when loading the data.\nThe default value is 0. This property is useful if you have header rows in the file that should be skipped.\nWhen autodetect is on, the behavior is the following:\nskipLeadingRows unspecified - Autodetect tries to detect headers in the first row. If they are not detected,\nthe row is read as data. Otherwise data is read starting from the second row.\nskipLeadingRows is 0 - Instructs autodetect that there are no headers and data should be read starting from the first row.\nskipLeadingRows = N > 0 - Autodetect skips N-1 rows and tries to detect headers in row N. If headers are not detected,\nrow N is just skipped. Otherwise row N is used to extract column names for the detected schema."]
    pub fn set_skip_leading_rows(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.skip_leading_rows = Some(v.into());
        self
    }
    #[doc = "Set the field `source_format`.\nThe format of the data files. For CSV files, specify \"CSV\". For datastore backups, specify \"DATASTORE_BACKUP\".\nFor newline-delimited JSON, specify \"NEWLINE_DELIMITED_JSON\". For Avro, specify \"AVRO\". For parquet, specify \"PARQUET\".\nFor orc, specify \"ORC\". [Beta] For Bigtable, specify \"BIGTABLE\".\nThe default value is CSV."]
    pub fn set_source_format(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_format = Some(v.into());
        self
    }
    #[doc = "Set the field `write_disposition`.\nSpecifies the action that occurs if the destination table already exists. The following values are supported:\nWRITE_TRUNCATE: If the table already exists, BigQuery overwrites the table data and uses the schema from the query result.\nWRITE_APPEND: If the table already exists, BigQuery appends the data to the table.\nWRITE_EMPTY: If the table already exists and contains data, a 'duplicate' error is returned in the job result.\nEach action is atomic and only occurs if BigQuery is able to complete the job successfully.\nCreation, truncation and append actions occur as one atomic update upon job completion. Default value: \"WRITE_EMPTY\" Possible values: [\"WRITE_TRUNCATE\", \"WRITE_APPEND\", \"WRITE_EMPTY\"]"]
    pub fn set_write_disposition(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.write_disposition = Some(v.into());
        self
    }
    #[doc = "Set the field `destination_encryption_configuration`.\n"]
    pub fn set_destination_encryption_configuration(
        mut self,
        v: impl Into<BlockAssignable<BigqueryJobLoadElDestinationEncryptionConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.destination_encryption_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.destination_encryption_configuration = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `destination_table`.\n"]
    pub fn set_destination_table(
        mut self,
        v: impl Into<BlockAssignable<BigqueryJobLoadElDestinationTableEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.destination_table = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.destination_table = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `parquet_options`.\n"]
    pub fn set_parquet_options(
        mut self,
        v: impl Into<BlockAssignable<BigqueryJobLoadElParquetOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.parquet_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.parquet_options = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `time_partitioning`.\n"]
    pub fn set_time_partitioning(
        mut self,
        v: impl Into<BlockAssignable<BigqueryJobLoadElTimePartitioningEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.time_partitioning = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.time_partitioning = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for BigqueryJobLoadEl {
    type O = BlockAssignable<BigqueryJobLoadEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBigqueryJobLoadEl {
    #[doc = "The fully-qualified URIs that point to your data in Google Cloud.\nFor Google Cloud Storage URIs: Each URI can contain one '\\*' wildcard character\nand it must come after the 'bucket' name. Size limits related to load jobs apply\nto external data sources. For Google Cloud Bigtable URIs: Exactly one URI can be\nspecified and it has be a fully specified and valid HTTPS URL for a Google Cloud Bigtable table.\nFor Google Cloud Datastore backups: Exactly one URI can be specified. Also, the '\\*' wildcard character is not allowed."]
    pub source_uris: ListField<PrimField<String>>,
}
impl BuildBigqueryJobLoadEl {
    pub fn build(self) -> BigqueryJobLoadEl {
        BigqueryJobLoadEl {
            allow_jagged_rows: core::default::Default::default(),
            allow_quoted_newlines: core::default::Default::default(),
            autodetect: core::default::Default::default(),
            create_disposition: core::default::Default::default(),
            encoding: core::default::Default::default(),
            field_delimiter: core::default::Default::default(),
            ignore_unknown_values: core::default::Default::default(),
            json_extension: core::default::Default::default(),
            max_bad_records: core::default::Default::default(),
            null_marker: core::default::Default::default(),
            projection_fields: core::default::Default::default(),
            quote: core::default::Default::default(),
            schema_update_options: core::default::Default::default(),
            skip_leading_rows: core::default::Default::default(),
            source_format: core::default::Default::default(),
            source_uris: self.source_uris,
            write_disposition: core::default::Default::default(),
            destination_encryption_configuration: core::default::Default::default(),
            destination_table: core::default::Default::default(),
            parquet_options: core::default::Default::default(),
            time_partitioning: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct BigqueryJobLoadElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryJobLoadElRef {
    fn new(shared: StackShared, base: String) -> BigqueryJobLoadElRef {
        BigqueryJobLoadElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BigqueryJobLoadElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allow_jagged_rows` after provisioning.\nAccept rows that are missing trailing optional columns. The missing values are treated as nulls.\nIf false, records with missing trailing columns are treated as bad records, and if there are too many bad records,\nan invalid error is returned in the job result. The default value is false. Only applicable to CSV, ignored for other formats."]
    pub fn allow_jagged_rows(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_jagged_rows", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `allow_quoted_newlines` after provisioning.\nIndicates if BigQuery should allow quoted data sections that contain newline characters in a CSV file.\nThe default value is false."]
    pub fn allow_quoted_newlines(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_quoted_newlines", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `autodetect` after provisioning.\nIndicates if we should automatically infer the options and schema for CSV and JSON sources."]
    pub fn autodetect(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.autodetect", self.base))
    }
    #[doc = "Get a reference to the value of field `create_disposition` after provisioning.\nSpecifies whether the job is allowed to create new tables. The following values are supported:\nCREATE_IF_NEEDED: If the table does not exist, BigQuery creates the table.\nCREATE_NEVER: The table must already exist. If it does not, a 'notFound' error is returned in the job result.\nCreation, truncation and append actions occur as one atomic update upon job completion Default value: \"CREATE_IF_NEEDED\" Possible values: [\"CREATE_IF_NEEDED\", \"CREATE_NEVER\"]"]
    pub fn create_disposition(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_disposition", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `encoding` after provisioning.\nThe character encoding of the data. The supported values are UTF-8 or ISO-8859-1.\nThe default value is UTF-8. BigQuery decodes the data after the raw, binary data\nhas been split using the values of the quote and fieldDelimiter properties."]
    pub fn encoding(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.encoding", self.base))
    }
    #[doc = "Get a reference to the value of field `field_delimiter` after provisioning.\nThe separator for fields in a CSV file. The separator can be any ISO-8859-1 single-byte character.\nTo use a character in the range 128-255, you must encode the character as UTF8. BigQuery converts\nthe string to ISO-8859-1 encoding, and then uses the first byte of the encoded string to split the\ndata in its raw, binary state. BigQuery also supports the escape sequence \"\\t\" to specify a tab separator.\nThe default value is a comma (',')."]
    pub fn field_delimiter(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.field_delimiter", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ignore_unknown_values` after provisioning.\nIndicates if BigQuery should allow extra values that are not represented in the table schema.\nIf true, the extra values are ignored. If false, records with extra columns are treated as bad records,\nand if there are too many bad records, an invalid error is returned in the job result.\nThe default value is false. The sourceFormat property determines what BigQuery treats as an extra value:\nCSV: Trailing columns\nJSON: Named values that don't match any column names"]
    pub fn ignore_unknown_values(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ignore_unknown_values", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `json_extension` after provisioning.\nIf sourceFormat is set to newline-delimited JSON, indicates whether it should be processed as a JSON variant such as GeoJSON.\nFor a sourceFormat other than JSON, omit this field. If the sourceFormat is newline-delimited JSON: - for newline-delimited\nGeoJSON: set to GEOJSON."]
    pub fn json_extension(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.json_extension", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_bad_records` after provisioning.\nThe maximum number of bad records that BigQuery can ignore when running the job. If the number of bad records exceeds this value,\nan invalid error is returned in the job result. The default value is 0, which requires that all records are valid."]
    pub fn max_bad_records(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_bad_records", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `null_marker` after provisioning.\nSpecifies a string that represents a null value in a CSV file. For example, if you specify \"\\N\", BigQuery interprets \"\\N\" as a null value\nwhen loading a CSV file. The default value is the empty string. If you set this property to a custom value, BigQuery throws an error if an\nempty string is present for all data types except for STRING and BYTE. For STRING and BYTE columns, BigQuery interprets the empty string as\nan empty value."]
    pub fn null_marker(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.null_marker", self.base))
    }
    #[doc = "Get a reference to the value of field `projection_fields` after provisioning.\nIf sourceFormat is set to \"DATASTORE_BACKUP\", indicates which entity properties to load into BigQuery from a Cloud Datastore backup.\nProperty names are case sensitive and must be top-level properties. If no properties are specified, BigQuery loads all properties.\nIf any named property isn't found in the Cloud Datastore backup, an invalid error is returned in the job result."]
    pub fn projection_fields(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.projection_fields", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `quote` after provisioning.\nThe value that is used to quote data sections in a CSV file. BigQuery converts the string to ISO-8859-1 encoding,\nand then uses the first byte of the encoded string to split the data in its raw, binary state.\nThe default value is a double-quote ('\"'). If your data does not contain quoted sections, set the property value to an empty string.\nIf your data contains quoted newline characters, you must also set the allowQuotedNewlines property to true."]
    pub fn quote(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.quote", self.base))
    }
    #[doc = "Get a reference to the value of field `schema_update_options` after provisioning.\nAllows the schema of the destination table to be updated as a side effect of the load job if a schema is autodetected or\nsupplied in the job configuration. Schema update options are supported in two cases: when writeDisposition is WRITE_APPEND;\nwhen writeDisposition is WRITE_TRUNCATE and the destination table is a partition of a table, specified by partition decorators.\nFor normal tables, WRITE_TRUNCATE will always overwrite the schema. One or more of the following values are specified:\nALLOW_FIELD_ADDITION: allow adding a nullable field to the schema.\nALLOW_FIELD_RELAXATION: allow relaxing a required field in the original schema to nullable."]
    pub fn schema_update_options(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.schema_update_options", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `skip_leading_rows` after provisioning.\nThe number of rows at the top of a CSV file that BigQuery will skip when loading the data.\nThe default value is 0. This property is useful if you have header rows in the file that should be skipped.\nWhen autodetect is on, the behavior is the following:\nskipLeadingRows unspecified - Autodetect tries to detect headers in the first row. If they are not detected,\nthe row is read as data. Otherwise data is read starting from the second row.\nskipLeadingRows is 0 - Instructs autodetect that there are no headers and data should be read starting from the first row.\nskipLeadingRows = N > 0 - Autodetect skips N-1 rows and tries to detect headers in row N. If headers are not detected,\nrow N is just skipped. Otherwise row N is used to extract column names for the detected schema."]
    pub fn skip_leading_rows(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.skip_leading_rows", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `source_format` after provisioning.\nThe format of the data files. For CSV files, specify \"CSV\". For datastore backups, specify \"DATASTORE_BACKUP\".\nFor newline-delimited JSON, specify \"NEWLINE_DELIMITED_JSON\". For Avro, specify \"AVRO\". For parquet, specify \"PARQUET\".\nFor orc, specify \"ORC\". [Beta] For Bigtable, specify \"BIGTABLE\".\nThe default value is CSV."]
    pub fn source_format(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_format", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `source_uris` after provisioning.\nThe fully-qualified URIs that point to your data in Google Cloud.\nFor Google Cloud Storage URIs: Each URI can contain one '\\*' wildcard character\nand it must come after the 'bucket' name. Size limits related to load jobs apply\nto external data sources. For Google Cloud Bigtable URIs: Exactly one URI can be\nspecified and it has be a fully specified and valid HTTPS URL for a Google Cloud Bigtable table.\nFor Google Cloud Datastore backups: Exactly one URI can be specified. Also, the '\\*' wildcard character is not allowed."]
    pub fn source_uris(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.source_uris", self.base))
    }
    #[doc = "Get a reference to the value of field `write_disposition` after provisioning.\nSpecifies the action that occurs if the destination table already exists. The following values are supported:\nWRITE_TRUNCATE: If the table already exists, BigQuery overwrites the table data and uses the schema from the query result.\nWRITE_APPEND: If the table already exists, BigQuery appends the data to the table.\nWRITE_EMPTY: If the table already exists and contains data, a 'duplicate' error is returned in the job result.\nEach action is atomic and only occurs if BigQuery is able to complete the job successfully.\nCreation, truncation and append actions occur as one atomic update upon job completion. Default value: \"WRITE_EMPTY\" Possible values: [\"WRITE_TRUNCATE\", \"WRITE_APPEND\", \"WRITE_EMPTY\"]"]
    pub fn write_disposition(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.write_disposition", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `destination_encryption_configuration` after provisioning.\n"]
    pub fn destination_encryption_configuration(
        &self,
    ) -> ListRef<BigqueryJobLoadElDestinationEncryptionConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.destination_encryption_configuration", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `destination_table` after provisioning.\n"]
    pub fn destination_table(&self) -> ListRef<BigqueryJobLoadElDestinationTableElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.destination_table", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `parquet_options` after provisioning.\n"]
    pub fn parquet_options(&self) -> ListRef<BigqueryJobLoadElParquetOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.parquet_options", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `time_partitioning` after provisioning.\n"]
    pub fn time_partitioning(&self) -> ListRef<BigqueryJobLoadElTimePartitioningElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.time_partitioning", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct BigqueryJobQueryElConnectionPropertiesEl {
    key: PrimField<String>,
    value: PrimField<String>,
}
impl BigqueryJobQueryElConnectionPropertiesEl {}
impl ToListMappable for BigqueryJobQueryElConnectionPropertiesEl {
    type O = BlockAssignable<BigqueryJobQueryElConnectionPropertiesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBigqueryJobQueryElConnectionPropertiesEl {
    #[doc = "The key of the property to set. Currently supported connection properties:\n* 'dataset_project_id': represents the default project for datasets that are used in the query\n* 'time_zone': represents the default timezone used to run the query\n* 'session_id': associates the query with a given session\n* 'query_label': associates the query with a given job label\n* 'service_account': indicates the service account to use to run a continuous query"]
    pub key: PrimField<String>,
    #[doc = "The value of the property to set."]
    pub value: PrimField<String>,
}
impl BuildBigqueryJobQueryElConnectionPropertiesEl {
    pub fn build(self) -> BigqueryJobQueryElConnectionPropertiesEl {
        BigqueryJobQueryElConnectionPropertiesEl {
            key: self.key,
            value: self.value,
        }
    }
}
pub struct BigqueryJobQueryElConnectionPropertiesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryJobQueryElConnectionPropertiesElRef {
    fn new(shared: StackShared, base: String) -> BigqueryJobQueryElConnectionPropertiesElRef {
        BigqueryJobQueryElConnectionPropertiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BigqueryJobQueryElConnectionPropertiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\nThe key of the property to set. Currently supported connection properties:\n* 'dataset_project_id': represents the default project for datasets that are used in the query\n* 'time_zone': represents the default timezone used to run the query\n* 'session_id': associates the query with a given session\n* 'query_label': associates the query with a given job label\n* 'service_account': indicates the service account to use to run a continuous query"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\nThe value of the property to set."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct BigqueryJobQueryElDefaultDatasetEl {
    dataset_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project_id: Option<PrimField<String>>,
}
impl BigqueryJobQueryElDefaultDatasetEl {
    #[doc = "Set the field `project_id`.\nThe ID of the project containing this table."]
    pub fn set_project_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.project_id = Some(v.into());
        self
    }
}
impl ToListMappable for BigqueryJobQueryElDefaultDatasetEl {
    type O = BlockAssignable<BigqueryJobQueryElDefaultDatasetEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBigqueryJobQueryElDefaultDatasetEl {
    #[doc = "The dataset. Can be specified '{{dataset_id}}' if 'project_id' is also set,\nor of the form 'projects/{{project}}/datasets/{{dataset_id}}' if not."]
    pub dataset_id: PrimField<String>,
}
impl BuildBigqueryJobQueryElDefaultDatasetEl {
    pub fn build(self) -> BigqueryJobQueryElDefaultDatasetEl {
        BigqueryJobQueryElDefaultDatasetEl {
            dataset_id: self.dataset_id,
            project_id: core::default::Default::default(),
        }
    }
}
pub struct BigqueryJobQueryElDefaultDatasetElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryJobQueryElDefaultDatasetElRef {
    fn new(shared: StackShared, base: String) -> BigqueryJobQueryElDefaultDatasetElRef {
        BigqueryJobQueryElDefaultDatasetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BigqueryJobQueryElDefaultDatasetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `dataset_id` after provisioning.\nThe dataset. Can be specified '{{dataset_id}}' if 'project_id' is also set,\nor of the form 'projects/{{project}}/datasets/{{dataset_id}}' if not."]
    pub fn dataset_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dataset_id", self.base))
    }
    #[doc = "Get a reference to the value of field `project_id` after provisioning.\nThe ID of the project containing this table."]
    pub fn project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }
}
#[derive(Serialize)]
pub struct BigqueryJobQueryElDestinationEncryptionConfigurationEl {
    kms_key_name: PrimField<String>,
}
impl BigqueryJobQueryElDestinationEncryptionConfigurationEl {}
impl ToListMappable for BigqueryJobQueryElDestinationEncryptionConfigurationEl {
    type O = BlockAssignable<BigqueryJobQueryElDestinationEncryptionConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBigqueryJobQueryElDestinationEncryptionConfigurationEl {
    #[doc = "Describes the Cloud KMS encryption key that will be used to protect destination BigQuery table.\nThe BigQuery Service Account associated with your project requires access to this encryption key."]
    pub kms_key_name: PrimField<String>,
}
impl BuildBigqueryJobQueryElDestinationEncryptionConfigurationEl {
    pub fn build(self) -> BigqueryJobQueryElDestinationEncryptionConfigurationEl {
        BigqueryJobQueryElDestinationEncryptionConfigurationEl {
            kms_key_name: self.kms_key_name,
        }
    }
}
pub struct BigqueryJobQueryElDestinationEncryptionConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryJobQueryElDestinationEncryptionConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BigqueryJobQueryElDestinationEncryptionConfigurationElRef {
        BigqueryJobQueryElDestinationEncryptionConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BigqueryJobQueryElDestinationEncryptionConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `kms_key_name` after provisioning.\nDescribes the Cloud KMS encryption key that will be used to protect destination BigQuery table.\nThe BigQuery Service Account associated with your project requires access to this encryption key."]
    pub fn kms_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.base))
    }
    #[doc = "Get a reference to the value of field `kms_key_version` after provisioning.\nDescribes the Cloud KMS encryption key version used to protect destination BigQuery table."]
    pub fn kms_key_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_version", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct BigqueryJobQueryElDestinationTableEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dataset_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project_id: Option<PrimField<String>>,
    table_id: PrimField<String>,
}
impl BigqueryJobQueryElDestinationTableEl {
    #[doc = "Set the field `dataset_id`.\nThe ID of the dataset containing this table."]
    pub fn set_dataset_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dataset_id = Some(v.into());
        self
    }
    #[doc = "Set the field `project_id`.\nThe ID of the project containing this table."]
    pub fn set_project_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.project_id = Some(v.into());
        self
    }
}
impl ToListMappable for BigqueryJobQueryElDestinationTableEl {
    type O = BlockAssignable<BigqueryJobQueryElDestinationTableEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBigqueryJobQueryElDestinationTableEl {
    #[doc = "The table. Can be specified '{{table_id}}' if 'project_id' and 'dataset_id' are also set,\nor of the form 'projects/{{project}}/datasets/{{dataset_id}}/tables/{{table_id}}' if not."]
    pub table_id: PrimField<String>,
}
impl BuildBigqueryJobQueryElDestinationTableEl {
    pub fn build(self) -> BigqueryJobQueryElDestinationTableEl {
        BigqueryJobQueryElDestinationTableEl {
            dataset_id: core::default::Default::default(),
            project_id: core::default::Default::default(),
            table_id: self.table_id,
        }
    }
}
pub struct BigqueryJobQueryElDestinationTableElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryJobQueryElDestinationTableElRef {
    fn new(shared: StackShared, base: String) -> BigqueryJobQueryElDestinationTableElRef {
        BigqueryJobQueryElDestinationTableElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BigqueryJobQueryElDestinationTableElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `dataset_id` after provisioning.\nThe ID of the dataset containing this table."]
    pub fn dataset_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dataset_id", self.base))
    }
    #[doc = "Get a reference to the value of field `project_id` after provisioning.\nThe ID of the project containing this table."]
    pub fn project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }
    #[doc = "Get a reference to the value of field `table_id` after provisioning.\nThe table. Can be specified '{{table_id}}' if 'project_id' and 'dataset_id' are also set,\nor of the form 'projects/{{project}}/datasets/{{dataset_id}}/tables/{{table_id}}' if not."]
    pub fn table_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.table_id", self.base))
    }
}
#[derive(Serialize)]
pub struct BigqueryJobQueryElScriptOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key_result_statement: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    statement_byte_budget: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    statement_timeout_ms: Option<PrimField<String>>,
}
impl BigqueryJobQueryElScriptOptionsEl {
    #[doc = "Set the field `key_result_statement`.\nDetermines which statement in the script represents the \"key result\",\nused to populate the schema and query results of the script job. Possible values: [\"LAST\", \"FIRST_SELECT\"]"]
    pub fn set_key_result_statement(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key_result_statement = Some(v.into());
        self
    }
    #[doc = "Set the field `statement_byte_budget`.\nLimit on the number of bytes billed per statement. Exceeding this budget results in an error."]
    pub fn set_statement_byte_budget(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.statement_byte_budget = Some(v.into());
        self
    }
    #[doc = "Set the field `statement_timeout_ms`.\nTimeout period for each statement in a script."]
    pub fn set_statement_timeout_ms(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.statement_timeout_ms = Some(v.into());
        self
    }
}
impl ToListMappable for BigqueryJobQueryElScriptOptionsEl {
    type O = BlockAssignable<BigqueryJobQueryElScriptOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBigqueryJobQueryElScriptOptionsEl {}
impl BuildBigqueryJobQueryElScriptOptionsEl {
    pub fn build(self) -> BigqueryJobQueryElScriptOptionsEl {
        BigqueryJobQueryElScriptOptionsEl {
            key_result_statement: core::default::Default::default(),
            statement_byte_budget: core::default::Default::default(),
            statement_timeout_ms: core::default::Default::default(),
        }
    }
}
pub struct BigqueryJobQueryElScriptOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryJobQueryElScriptOptionsElRef {
    fn new(shared: StackShared, base: String) -> BigqueryJobQueryElScriptOptionsElRef {
        BigqueryJobQueryElScriptOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BigqueryJobQueryElScriptOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `key_result_statement` after provisioning.\nDetermines which statement in the script represents the \"key result\",\nused to populate the schema and query results of the script job. Possible values: [\"LAST\", \"FIRST_SELECT\"]"]
    pub fn key_result_statement(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.key_result_statement", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `statement_byte_budget` after provisioning.\nLimit on the number of bytes billed per statement. Exceeding this budget results in an error."]
    pub fn statement_byte_budget(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.statement_byte_budget", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `statement_timeout_ms` after provisioning.\nTimeout period for each statement in a script."]
    pub fn statement_timeout_ms(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.statement_timeout_ms", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct BigqueryJobQueryElUserDefinedFunctionResourcesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    inline_code: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_uri: Option<PrimField<String>>,
}
impl BigqueryJobQueryElUserDefinedFunctionResourcesEl {
    #[doc = "Set the field `inline_code`.\nAn inline resource that contains code for a user-defined function (UDF).\nProviding a inline code resource is equivalent to providing a URI for a file containing the same code."]
    pub fn set_inline_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.inline_code = Some(v.into());
        self
    }
    #[doc = "Set the field `resource_uri`.\nA code resource to load from a Google Cloud Storage URI (gs://bucket/path)."]
    pub fn set_resource_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.resource_uri = Some(v.into());
        self
    }
}
impl ToListMappable for BigqueryJobQueryElUserDefinedFunctionResourcesEl {
    type O = BlockAssignable<BigqueryJobQueryElUserDefinedFunctionResourcesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBigqueryJobQueryElUserDefinedFunctionResourcesEl {}
impl BuildBigqueryJobQueryElUserDefinedFunctionResourcesEl {
    pub fn build(self) -> BigqueryJobQueryElUserDefinedFunctionResourcesEl {
        BigqueryJobQueryElUserDefinedFunctionResourcesEl {
            inline_code: core::default::Default::default(),
            resource_uri: core::default::Default::default(),
        }
    }
}
pub struct BigqueryJobQueryElUserDefinedFunctionResourcesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryJobQueryElUserDefinedFunctionResourcesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BigqueryJobQueryElUserDefinedFunctionResourcesElRef {
        BigqueryJobQueryElUserDefinedFunctionResourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BigqueryJobQueryElUserDefinedFunctionResourcesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `inline_code` after provisioning.\nAn inline resource that contains code for a user-defined function (UDF).\nProviding a inline code resource is equivalent to providing a URI for a file containing the same code."]
    pub fn inline_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.inline_code", self.base))
    }
    #[doc = "Get a reference to the value of field `resource_uri` after provisioning.\nA code resource to load from a Google Cloud Storage URI (gs://bucket/path)."]
    pub fn resource_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_uri", self.base))
    }
}
#[derive(Serialize, Default)]
struct BigqueryJobQueryElDynamic {
    connection_properties: Option<DynamicBlock<BigqueryJobQueryElConnectionPropertiesEl>>,
    default_dataset: Option<DynamicBlock<BigqueryJobQueryElDefaultDatasetEl>>,
    destination_encryption_configuration:
        Option<DynamicBlock<BigqueryJobQueryElDestinationEncryptionConfigurationEl>>,
    destination_table: Option<DynamicBlock<BigqueryJobQueryElDestinationTableEl>>,
    script_options: Option<DynamicBlock<BigqueryJobQueryElScriptOptionsEl>>,
    user_defined_function_resources:
        Option<DynamicBlock<BigqueryJobQueryElUserDefinedFunctionResourcesEl>>,
}
#[derive(Serialize)]
pub struct BigqueryJobQueryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_large_results: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    create_disposition: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    flatten_results: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maximum_billing_tier: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maximum_bytes_billed: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameter_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<PrimField<String>>,
    query: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schema_update_options: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_legacy_sql: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_query_cache: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    write_disposition: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    connection_properties: Option<Vec<BigqueryJobQueryElConnectionPropertiesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_dataset: Option<Vec<BigqueryJobQueryElDefaultDatasetEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_encryption_configuration:
        Option<Vec<BigqueryJobQueryElDestinationEncryptionConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_table: Option<Vec<BigqueryJobQueryElDestinationTableEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    script_options: Option<Vec<BigqueryJobQueryElScriptOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_defined_function_resources: Option<Vec<BigqueryJobQueryElUserDefinedFunctionResourcesEl>>,
    dynamic: BigqueryJobQueryElDynamic,
}
impl BigqueryJobQueryEl {
    #[doc = "Set the field `allow_large_results`.\nIf true and query uses legacy SQL dialect, allows the query to produce arbitrarily large result tables at a slight cost in performance.\nRequires destinationTable to be set. For standard SQL queries, this flag is ignored and large results are always allowed.\nHowever, you must still set destinationTable when result size exceeds the allowed maximum response size."]
    pub fn set_allow_large_results(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.allow_large_results = Some(v.into());
        self
    }
    #[doc = "Set the field `create_disposition`.\nSpecifies whether the job is allowed to create new tables. The following values are supported:\nCREATE_IF_NEEDED: If the table does not exist, BigQuery creates the table.\nCREATE_NEVER: The table must already exist. If it does not, a 'notFound' error is returned in the job result.\nCreation, truncation and append actions occur as one atomic update upon job completion Default value: \"CREATE_IF_NEEDED\" Possible values: [\"CREATE_IF_NEEDED\", \"CREATE_NEVER\"]"]
    pub fn set_create_disposition(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create_disposition = Some(v.into());
        self
    }
    #[doc = "Set the field `flatten_results`.\nIf true and query uses legacy SQL dialect, flattens all nested and repeated fields in the query results.\nallowLargeResults must be true if this is set to false. For standard SQL queries, this flag is ignored and results are never flattened."]
    pub fn set_flatten_results(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.flatten_results = Some(v.into());
        self
    }
    #[doc = "Set the field `maximum_billing_tier`.\nLimits the billing tier for this job. Queries that have resource usage beyond this tier will fail (without incurring a charge).\nIf unspecified, this will be set to your project default."]
    pub fn set_maximum_billing_tier(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.maximum_billing_tier = Some(v.into());
        self
    }
    #[doc = "Set the field `maximum_bytes_billed`.\nLimits the bytes billed for this job. Queries that will have bytes billed beyond this limit will fail (without incurring a charge).\nIf unspecified, this will be set to your project default."]
    pub fn set_maximum_bytes_billed(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.maximum_bytes_billed = Some(v.into());
        self
    }
    #[doc = "Set the field `parameter_mode`.\nStandard SQL only. Set to POSITIONAL to use positional (?) query parameters or to NAMED to use named (@myparam) query parameters in this query."]
    pub fn set_parameter_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.parameter_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `priority`.\nSpecifies a priority for the query. Default value: \"INTERACTIVE\" Possible values: [\"INTERACTIVE\", \"BATCH\"]"]
    pub fn set_priority(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.priority = Some(v.into());
        self
    }
    #[doc = "Set the field `schema_update_options`.\nAllows the schema of the destination table to be updated as a side effect of the query job.\nSchema update options are supported in two cases: when writeDisposition is WRITE_APPEND;\nwhen writeDisposition is WRITE_TRUNCATE and the destination table is a partition of a table,\nspecified by partition decorators. For normal tables, WRITE_TRUNCATE will always overwrite the schema.\nOne or more of the following values are specified:\nALLOW_FIELD_ADDITION: allow adding a nullable field to the schema.\nALLOW_FIELD_RELAXATION: allow relaxing a required field in the original schema to nullable."]
    pub fn set_schema_update_options(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.schema_update_options = Some(v.into());
        self
    }
    #[doc = "Set the field `use_legacy_sql`.\nSpecifies whether to use BigQuery's legacy SQL dialect for this query. The default value is true.\nIf set to false, the query will use BigQuery's standard SQL."]
    pub fn set_use_legacy_sql(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.use_legacy_sql = Some(v.into());
        self
    }
    #[doc = "Set the field `use_query_cache`.\nWhether to look for the result in the query cache. The query cache is a best-effort cache that will be flushed whenever\ntables in the query are modified. Moreover, the query cache is only available when a query does not have a destination table specified.\nThe default value is true."]
    pub fn set_use_query_cache(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.use_query_cache = Some(v.into());
        self
    }
    #[doc = "Set the field `write_disposition`.\nSpecifies the action that occurs if the destination table already exists. The following values are supported:\nWRITE_TRUNCATE: If the table already exists, BigQuery overwrites the table data and uses the schema from the query result.\nWRITE_APPEND: If the table already exists, BigQuery appends the data to the table.\nWRITE_EMPTY: If the table already exists and contains data, a 'duplicate' error is returned in the job result.\nEach action is atomic and only occurs if BigQuery is able to complete the job successfully.\nCreation, truncation and append actions occur as one atomic update upon job completion. Default value: \"WRITE_EMPTY\" Possible values: [\"WRITE_TRUNCATE\", \"WRITE_APPEND\", \"WRITE_EMPTY\"]"]
    pub fn set_write_disposition(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.write_disposition = Some(v.into());
        self
    }
    #[doc = "Set the field `connection_properties`.\n"]
    pub fn set_connection_properties(
        mut self,
        v: impl Into<BlockAssignable<BigqueryJobQueryElConnectionPropertiesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.connection_properties = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.connection_properties = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `default_dataset`.\n"]
    pub fn set_default_dataset(
        mut self,
        v: impl Into<BlockAssignable<BigqueryJobQueryElDefaultDatasetEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.default_dataset = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.default_dataset = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `destination_encryption_configuration`.\n"]
    pub fn set_destination_encryption_configuration(
        mut self,
        v: impl Into<BlockAssignable<BigqueryJobQueryElDestinationEncryptionConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.destination_encryption_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.destination_encryption_configuration = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `destination_table`.\n"]
    pub fn set_destination_table(
        mut self,
        v: impl Into<BlockAssignable<BigqueryJobQueryElDestinationTableEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.destination_table = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.destination_table = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `script_options`.\n"]
    pub fn set_script_options(
        mut self,
        v: impl Into<BlockAssignable<BigqueryJobQueryElScriptOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.script_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.script_options = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `user_defined_function_resources`.\n"]
    pub fn set_user_defined_function_resources(
        mut self,
        v: impl Into<BlockAssignable<BigqueryJobQueryElUserDefinedFunctionResourcesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.user_defined_function_resources = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.user_defined_function_resources = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for BigqueryJobQueryEl {
    type O = BlockAssignable<BigqueryJobQueryEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBigqueryJobQueryEl {
    #[doc = "SQL query text to execute. The useLegacySql field can be used to indicate whether the query uses legacy SQL or standard SQL.\n*NOTE*: queries containing [DML language](https://cloud.google.com/bigquery/docs/reference/standard-sql/data-manipulation-language)\n('DELETE', 'UPDATE', 'MERGE', 'INSERT') must specify 'create_disposition = \"\"' and 'write_disposition = \"\"'."]
    pub query: PrimField<String>,
}
impl BuildBigqueryJobQueryEl {
    pub fn build(self) -> BigqueryJobQueryEl {
        BigqueryJobQueryEl {
            allow_large_results: core::default::Default::default(),
            create_disposition: core::default::Default::default(),
            flatten_results: core::default::Default::default(),
            maximum_billing_tier: core::default::Default::default(),
            maximum_bytes_billed: core::default::Default::default(),
            parameter_mode: core::default::Default::default(),
            priority: core::default::Default::default(),
            query: self.query,
            schema_update_options: core::default::Default::default(),
            use_legacy_sql: core::default::Default::default(),
            use_query_cache: core::default::Default::default(),
            write_disposition: core::default::Default::default(),
            connection_properties: core::default::Default::default(),
            default_dataset: core::default::Default::default(),
            destination_encryption_configuration: core::default::Default::default(),
            destination_table: core::default::Default::default(),
            script_options: core::default::Default::default(),
            user_defined_function_resources: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct BigqueryJobQueryElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryJobQueryElRef {
    fn new(shared: StackShared, base: String) -> BigqueryJobQueryElRef {
        BigqueryJobQueryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BigqueryJobQueryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allow_large_results` after provisioning.\nIf true and query uses legacy SQL dialect, allows the query to produce arbitrarily large result tables at a slight cost in performance.\nRequires destinationTable to be set. For standard SQL queries, this flag is ignored and large results are always allowed.\nHowever, you must still set destinationTable when result size exceeds the allowed maximum response size."]
    pub fn allow_large_results(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_large_results", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `create_disposition` after provisioning.\nSpecifies whether the job is allowed to create new tables. The following values are supported:\nCREATE_IF_NEEDED: If the table does not exist, BigQuery creates the table.\nCREATE_NEVER: The table must already exist. If it does not, a 'notFound' error is returned in the job result.\nCreation, truncation and append actions occur as one atomic update upon job completion Default value: \"CREATE_IF_NEEDED\" Possible values: [\"CREATE_IF_NEEDED\", \"CREATE_NEVER\"]"]
    pub fn create_disposition(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_disposition", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `flatten_results` after provisioning.\nIf true and query uses legacy SQL dialect, flattens all nested and repeated fields in the query results.\nallowLargeResults must be true if this is set to false. For standard SQL queries, this flag is ignored and results are never flattened."]
    pub fn flatten_results(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.flatten_results", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `maximum_billing_tier` after provisioning.\nLimits the billing tier for this job. Queries that have resource usage beyond this tier will fail (without incurring a charge).\nIf unspecified, this will be set to your project default."]
    pub fn maximum_billing_tier(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.maximum_billing_tier", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `maximum_bytes_billed` after provisioning.\nLimits the bytes billed for this job. Queries that will have bytes billed beyond this limit will fail (without incurring a charge).\nIf unspecified, this will be set to your project default."]
    pub fn maximum_bytes_billed(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.maximum_bytes_billed", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `parameter_mode` after provisioning.\nStandard SQL only. Set to POSITIONAL to use positional (?) query parameters or to NAMED to use named (@myparam) query parameters in this query."]
    pub fn parameter_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parameter_mode", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `priority` after provisioning.\nSpecifies a priority for the query. Default value: \"INTERACTIVE\" Possible values: [\"INTERACTIVE\", \"BATCH\"]"]
    pub fn priority(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }
    #[doc = "Get a reference to the value of field `query` after provisioning.\nSQL query text to execute. The useLegacySql field can be used to indicate whether the query uses legacy SQL or standard SQL.\n*NOTE*: queries containing [DML language](https://cloud.google.com/bigquery/docs/reference/standard-sql/data-manipulation-language)\n('DELETE', 'UPDATE', 'MERGE', 'INSERT') must specify 'create_disposition = \"\"' and 'write_disposition = \"\"'."]
    pub fn query(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.query", self.base))
    }
    #[doc = "Get a reference to the value of field `schema_update_options` after provisioning.\nAllows the schema of the destination table to be updated as a side effect of the query job.\nSchema update options are supported in two cases: when writeDisposition is WRITE_APPEND;\nwhen writeDisposition is WRITE_TRUNCATE and the destination table is a partition of a table,\nspecified by partition decorators. For normal tables, WRITE_TRUNCATE will always overwrite the schema.\nOne or more of the following values are specified:\nALLOW_FIELD_ADDITION: allow adding a nullable field to the schema.\nALLOW_FIELD_RELAXATION: allow relaxing a required field in the original schema to nullable."]
    pub fn schema_update_options(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.schema_update_options", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `use_legacy_sql` after provisioning.\nSpecifies whether to use BigQuery's legacy SQL dialect for this query. The default value is true.\nIf set to false, the query will use BigQuery's standard SQL."]
    pub fn use_legacy_sql(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.use_legacy_sql", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `use_query_cache` after provisioning.\nWhether to look for the result in the query cache. The query cache is a best-effort cache that will be flushed whenever\ntables in the query are modified. Moreover, the query cache is only available when a query does not have a destination table specified.\nThe default value is true."]
    pub fn use_query_cache(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.use_query_cache", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `write_disposition` after provisioning.\nSpecifies the action that occurs if the destination table already exists. The following values are supported:\nWRITE_TRUNCATE: If the table already exists, BigQuery overwrites the table data and uses the schema from the query result.\nWRITE_APPEND: If the table already exists, BigQuery appends the data to the table.\nWRITE_EMPTY: If the table already exists and contains data, a 'duplicate' error is returned in the job result.\nEach action is atomic and only occurs if BigQuery is able to complete the job successfully.\nCreation, truncation and append actions occur as one atomic update upon job completion. Default value: \"WRITE_EMPTY\" Possible values: [\"WRITE_TRUNCATE\", \"WRITE_APPEND\", \"WRITE_EMPTY\"]"]
    pub fn write_disposition(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.write_disposition", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `connection_properties` after provisioning.\n"]
    pub fn connection_properties(&self) -> ListRef<BigqueryJobQueryElConnectionPropertiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.connection_properties", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `default_dataset` after provisioning.\n"]
    pub fn default_dataset(&self) -> ListRef<BigqueryJobQueryElDefaultDatasetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.default_dataset", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `destination_encryption_configuration` after provisioning.\n"]
    pub fn destination_encryption_configuration(
        &self,
    ) -> ListRef<BigqueryJobQueryElDestinationEncryptionConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.destination_encryption_configuration", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `destination_table` after provisioning.\n"]
    pub fn destination_table(&self) -> ListRef<BigqueryJobQueryElDestinationTableElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.destination_table", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `script_options` after provisioning.\n"]
    pub fn script_options(&self) -> ListRef<BigqueryJobQueryElScriptOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.script_options", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `user_defined_function_resources` after provisioning.\n"]
    pub fn user_defined_function_resources(
        &self,
    ) -> ListRef<BigqueryJobQueryElUserDefinedFunctionResourcesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.user_defined_function_resources", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct BigqueryJobTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl BigqueryJobTimeoutsEl {
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
impl ToListMappable for BigqueryJobTimeoutsEl {
    type O = BlockAssignable<BigqueryJobTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBigqueryJobTimeoutsEl {}
impl BuildBigqueryJobTimeoutsEl {
    pub fn build(self) -> BigqueryJobTimeoutsEl {
        BigqueryJobTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct BigqueryJobTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryJobTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> BigqueryJobTimeoutsElRef {
        BigqueryJobTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BigqueryJobTimeoutsElRef {
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
struct BigqueryJobDynamic {
    copy: Option<DynamicBlock<BigqueryJobCopyEl>>,
    extract: Option<DynamicBlock<BigqueryJobExtractEl>>,
    load: Option<DynamicBlock<BigqueryJobLoadEl>>,
    query: Option<DynamicBlock<BigqueryJobQueryEl>>,
}
