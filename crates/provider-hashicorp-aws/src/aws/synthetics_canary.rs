use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct SyntheticsCanaryData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    artifact_s3_location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete_lambda: Option<PrimField<bool>>,
    execution_role_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    failure_retention_period: Option<PrimField<f64>>,
    handler: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    runtime_version: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_bucket: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_canary: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    success_retention_period: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zip_file: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    artifact_config: Option<Vec<SyntheticsCanaryArtifactConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    run_config: Option<Vec<SyntheticsCanaryRunConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schedule: Option<Vec<SyntheticsCanaryScheduleEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_config: Option<Vec<SyntheticsCanaryVpcConfigEl>>,
    dynamic: SyntheticsCanaryDynamic,
}

struct SyntheticsCanary_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SyntheticsCanaryData>,
}

#[derive(Clone)]
pub struct SyntheticsCanary(Rc<SyntheticsCanary_>);

impl SyntheticsCanary {
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

    #[doc = "Set the field `delete_lambda`.\n"]
    pub fn set_delete_lambda(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().delete_lambda = Some(v.into());
        self
    }

    #[doc = "Set the field `failure_retention_period`.\n"]
    pub fn set_failure_retention_period(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().failure_retention_period = Some(v.into());
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

    #[doc = "Set the field `s3_bucket`.\n"]
    pub fn set_s3_bucket(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().s3_bucket = Some(v.into());
        self
    }

    #[doc = "Set the field `s3_key`.\n"]
    pub fn set_s3_key(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().s3_key = Some(v.into());
        self
    }

    #[doc = "Set the field `s3_version`.\n"]
    pub fn set_s3_version(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().s3_version = Some(v.into());
        self
    }

    #[doc = "Set the field `start_canary`.\n"]
    pub fn set_start_canary(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().start_canary = Some(v.into());
        self
    }

    #[doc = "Set the field `success_retention_period`.\n"]
    pub fn set_success_retention_period(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().success_retention_period = Some(v.into());
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

    #[doc = "Set the field `zip_file`.\n"]
    pub fn set_zip_file(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().zip_file = Some(v.into());
        self
    }

    #[doc = "Set the field `artifact_config`.\n"]
    pub fn set_artifact_config(
        self,
        v: impl Into<BlockAssignable<SyntheticsCanaryArtifactConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().artifact_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.artifact_config = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `run_config`.\n"]
    pub fn set_run_config(
        self,
        v: impl Into<BlockAssignable<SyntheticsCanaryRunConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().run_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.run_config = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `schedule`.\n"]
    pub fn set_schedule(self, v: impl Into<BlockAssignable<SyntheticsCanaryScheduleEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().schedule = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.schedule = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `vpc_config`.\n"]
    pub fn set_vpc_config(
        self,
        v: impl Into<BlockAssignable<SyntheticsCanaryVpcConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().vpc_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.vpc_config = Some(d);
            }
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `artifact_s3_location` after provisioning.\n"]
    pub fn artifact_s3_location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.artifact_s3_location", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `delete_lambda` after provisioning.\n"]
    pub fn delete_lambda(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delete_lambda", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `engine_arn` after provisioning.\n"]
    pub fn engine_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.engine_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `execution_role_arn` after provisioning.\n"]
    pub fn execution_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.execution_role_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `failure_retention_period` after provisioning.\n"]
    pub fn failure_retention_period(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.failure_retention_period", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `handler` after provisioning.\n"]
    pub fn handler(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.handler", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `runtime_version` after provisioning.\n"]
    pub fn runtime_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.runtime_version", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `s3_bucket` after provisioning.\n"]
    pub fn s3_bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.s3_bucket", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `s3_key` after provisioning.\n"]
    pub fn s3_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.s3_key", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `s3_version` after provisioning.\n"]
    pub fn s3_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.s3_version", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `source_location_arn` after provisioning.\n"]
    pub fn source_location_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_location_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `start_canary` after provisioning.\n"]
    pub fn start_canary(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.start_canary", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `success_retention_period` after provisioning.\n"]
    pub fn success_retention_period(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.success_retention_period", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `timeline` after provisioning.\n"]
    pub fn timeline(&self) -> ListRef<SyntheticsCanaryTimelineElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.timeline", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `zip_file` after provisioning.\n"]
    pub fn zip_file(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.zip_file", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `artifact_config` after provisioning.\n"]
    pub fn artifact_config(&self) -> ListRef<SyntheticsCanaryArtifactConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.artifact_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `run_config` after provisioning.\n"]
    pub fn run_config(&self) -> ListRef<SyntheticsCanaryRunConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.run_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `schedule` after provisioning.\n"]
    pub fn schedule(&self) -> ListRef<SyntheticsCanaryScheduleElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.schedule", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `vpc_config` after provisioning.\n"]
    pub fn vpc_config(&self) -> ListRef<SyntheticsCanaryVpcConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vpc_config", self.extract_ref()),
        )
    }
}

impl Referable for SyntheticsCanary {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for SyntheticsCanary {}

impl ToListMappable for SyntheticsCanary {
    type O = ListRef<SyntheticsCanaryRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for SyntheticsCanary_ {
    fn extract_resource_type(&self) -> String {
        "aws_synthetics_canary".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSyntheticsCanary {
    pub tf_id: String,
    #[doc = ""]
    pub artifact_s3_location: PrimField<String>,
    #[doc = ""]
    pub execution_role_arn: PrimField<String>,
    #[doc = ""]
    pub handler: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub runtime_version: PrimField<String>,
}

impl BuildSyntheticsCanary {
    pub fn build(self, stack: &mut Stack) -> SyntheticsCanary {
        let out = SyntheticsCanary(Rc::new(SyntheticsCanary_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(SyntheticsCanaryData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                artifact_s3_location: self.artifact_s3_location,
                delete_lambda: core::default::Default::default(),
                execution_role_arn: self.execution_role_arn,
                failure_retention_period: core::default::Default::default(),
                handler: self.handler,
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                runtime_version: self.runtime_version,
                s3_bucket: core::default::Default::default(),
                s3_key: core::default::Default::default(),
                s3_version: core::default::Default::default(),
                start_canary: core::default::Default::default(),
                success_retention_period: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                zip_file: core::default::Default::default(),
                artifact_config: core::default::Default::default(),
                run_config: core::default::Default::default(),
                schedule: core::default::Default::default(),
                vpc_config: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SyntheticsCanaryRef {
    shared: StackShared,
    base: String,
}

impl Ref for SyntheticsCanaryRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl SyntheticsCanaryRef {
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

    #[doc = "Get a reference to the value of field `artifact_s3_location` after provisioning.\n"]
    pub fn artifact_s3_location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.artifact_s3_location", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `delete_lambda` after provisioning.\n"]
    pub fn delete_lambda(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delete_lambda", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `engine_arn` after provisioning.\n"]
    pub fn engine_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.engine_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `execution_role_arn` after provisioning.\n"]
    pub fn execution_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.execution_role_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `failure_retention_period` after provisioning.\n"]
    pub fn failure_retention_period(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.failure_retention_period", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `handler` after provisioning.\n"]
    pub fn handler(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.handler", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `runtime_version` after provisioning.\n"]
    pub fn runtime_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.runtime_version", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `s3_bucket` after provisioning.\n"]
    pub fn s3_bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.s3_bucket", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `s3_key` after provisioning.\n"]
    pub fn s3_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.s3_key", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `s3_version` after provisioning.\n"]
    pub fn s3_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.s3_version", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `source_location_arn` after provisioning.\n"]
    pub fn source_location_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_location_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `start_canary` after provisioning.\n"]
    pub fn start_canary(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.start_canary", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `success_retention_period` after provisioning.\n"]
    pub fn success_retention_period(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.success_retention_period", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `timeline` after provisioning.\n"]
    pub fn timeline(&self) -> ListRef<SyntheticsCanaryTimelineElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.timeline", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `zip_file` after provisioning.\n"]
    pub fn zip_file(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.zip_file", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `artifact_config` after provisioning.\n"]
    pub fn artifact_config(&self) -> ListRef<SyntheticsCanaryArtifactConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.artifact_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `run_config` after provisioning.\n"]
    pub fn run_config(&self) -> ListRef<SyntheticsCanaryRunConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.run_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `schedule` after provisioning.\n"]
    pub fn schedule(&self) -> ListRef<SyntheticsCanaryScheduleElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.schedule", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `vpc_config` after provisioning.\n"]
    pub fn vpc_config(&self) -> ListRef<SyntheticsCanaryVpcConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vpc_config", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct SyntheticsCanaryTimelineEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    created: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_modified: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_started: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_stopped: Option<PrimField<String>>,
}

impl SyntheticsCanaryTimelineEl {
    #[doc = "Set the field `created`.\n"]
    pub fn set_created(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.created = Some(v.into());
        self
    }

    #[doc = "Set the field `last_modified`.\n"]
    pub fn set_last_modified(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.last_modified = Some(v.into());
        self
    }

    #[doc = "Set the field `last_started`.\n"]
    pub fn set_last_started(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.last_started = Some(v.into());
        self
    }

    #[doc = "Set the field `last_stopped`.\n"]
    pub fn set_last_stopped(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.last_stopped = Some(v.into());
        self
    }
}

impl ToListMappable for SyntheticsCanaryTimelineEl {
    type O = BlockAssignable<SyntheticsCanaryTimelineEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSyntheticsCanaryTimelineEl {}

impl BuildSyntheticsCanaryTimelineEl {
    pub fn build(self) -> SyntheticsCanaryTimelineEl {
        SyntheticsCanaryTimelineEl {
            created: core::default::Default::default(),
            last_modified: core::default::Default::default(),
            last_started: core::default::Default::default(),
            last_stopped: core::default::Default::default(),
        }
    }
}

pub struct SyntheticsCanaryTimelineElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SyntheticsCanaryTimelineElRef {
    fn new(shared: StackShared, base: String) -> SyntheticsCanaryTimelineElRef {
        SyntheticsCanaryTimelineElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SyntheticsCanaryTimelineElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `created` after provisioning.\n"]
    pub fn created(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created", self.base))
    }

    #[doc = "Get a reference to the value of field `last_modified` after provisioning.\n"]
    pub fn last_modified(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_modified", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `last_started` after provisioning.\n"]
    pub fn last_started(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.last_started", self.base))
    }

    #[doc = "Get a reference to the value of field `last_stopped` after provisioning.\n"]
    pub fn last_stopped(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.last_stopped", self.base))
    }
}

#[derive(Serialize)]
pub struct SyntheticsCanaryArtifactConfigElS3EncryptionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_arn: Option<PrimField<String>>,
}

impl SyntheticsCanaryArtifactConfigElS3EncryptionEl {
    #[doc = "Set the field `encryption_mode`.\n"]
    pub fn set_encryption_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.encryption_mode = Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_arn`.\n"]
    pub fn set_kms_key_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_arn = Some(v.into());
        self
    }
}

impl ToListMappable for SyntheticsCanaryArtifactConfigElS3EncryptionEl {
    type O = BlockAssignable<SyntheticsCanaryArtifactConfigElS3EncryptionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSyntheticsCanaryArtifactConfigElS3EncryptionEl {}

impl BuildSyntheticsCanaryArtifactConfigElS3EncryptionEl {
    pub fn build(self) -> SyntheticsCanaryArtifactConfigElS3EncryptionEl {
        SyntheticsCanaryArtifactConfigElS3EncryptionEl {
            encryption_mode: core::default::Default::default(),
            kms_key_arn: core::default::Default::default(),
        }
    }
}

pub struct SyntheticsCanaryArtifactConfigElS3EncryptionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SyntheticsCanaryArtifactConfigElS3EncryptionElRef {
    fn new(shared: StackShared, base: String) -> SyntheticsCanaryArtifactConfigElS3EncryptionElRef {
        SyntheticsCanaryArtifactConfigElS3EncryptionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SyntheticsCanaryArtifactConfigElS3EncryptionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `encryption_mode` after provisioning.\n"]
    pub fn encryption_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.encryption_mode", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct SyntheticsCanaryArtifactConfigElDynamic {
    s3_encryption: Option<DynamicBlock<SyntheticsCanaryArtifactConfigElS3EncryptionEl>>,
}

#[derive(Serialize)]
pub struct SyntheticsCanaryArtifactConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_encryption: Option<Vec<SyntheticsCanaryArtifactConfigElS3EncryptionEl>>,
    dynamic: SyntheticsCanaryArtifactConfigElDynamic,
}

impl SyntheticsCanaryArtifactConfigEl {
    #[doc = "Set the field `s3_encryption`.\n"]
    pub fn set_s3_encryption(
        mut self,
        v: impl Into<BlockAssignable<SyntheticsCanaryArtifactConfigElS3EncryptionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.s3_encryption = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_encryption = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for SyntheticsCanaryArtifactConfigEl {
    type O = BlockAssignable<SyntheticsCanaryArtifactConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSyntheticsCanaryArtifactConfigEl {}

impl BuildSyntheticsCanaryArtifactConfigEl {
    pub fn build(self) -> SyntheticsCanaryArtifactConfigEl {
        SyntheticsCanaryArtifactConfigEl {
            s3_encryption: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SyntheticsCanaryArtifactConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SyntheticsCanaryArtifactConfigElRef {
    fn new(shared: StackShared, base: String) -> SyntheticsCanaryArtifactConfigElRef {
        SyntheticsCanaryArtifactConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SyntheticsCanaryArtifactConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3_encryption` after provisioning.\n"]
    pub fn s3_encryption(&self) -> ListRef<SyntheticsCanaryArtifactConfigElS3EncryptionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.s3_encryption", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct SyntheticsCanaryRunConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    active_tracing: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    environment_variables: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ephemeral_storage: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memory_in_mb: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout_in_seconds: Option<PrimField<f64>>,
}

impl SyntheticsCanaryRunConfigEl {
    #[doc = "Set the field `active_tracing`.\n"]
    pub fn set_active_tracing(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.active_tracing = Some(v.into());
        self
    }

    #[doc = "Set the field `environment_variables`.\n"]
    pub fn set_environment_variables(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.environment_variables = Some(v.into());
        self
    }

    #[doc = "Set the field `ephemeral_storage`.\n"]
    pub fn set_ephemeral_storage(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.ephemeral_storage = Some(v.into());
        self
    }

    #[doc = "Set the field `memory_in_mb`.\n"]
    pub fn set_memory_in_mb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.memory_in_mb = Some(v.into());
        self
    }

    #[doc = "Set the field `timeout_in_seconds`.\n"]
    pub fn set_timeout_in_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.timeout_in_seconds = Some(v.into());
        self
    }
}

impl ToListMappable for SyntheticsCanaryRunConfigEl {
    type O = BlockAssignable<SyntheticsCanaryRunConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSyntheticsCanaryRunConfigEl {}

impl BuildSyntheticsCanaryRunConfigEl {
    pub fn build(self) -> SyntheticsCanaryRunConfigEl {
        SyntheticsCanaryRunConfigEl {
            active_tracing: core::default::Default::default(),
            environment_variables: core::default::Default::default(),
            ephemeral_storage: core::default::Default::default(),
            memory_in_mb: core::default::Default::default(),
            timeout_in_seconds: core::default::Default::default(),
        }
    }
}

pub struct SyntheticsCanaryRunConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SyntheticsCanaryRunConfigElRef {
    fn new(shared: StackShared, base: String) -> SyntheticsCanaryRunConfigElRef {
        SyntheticsCanaryRunConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SyntheticsCanaryRunConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `active_tracing` after provisioning.\n"]
    pub fn active_tracing(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.active_tracing", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `environment_variables` after provisioning.\n"]
    pub fn environment_variables(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.environment_variables", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `ephemeral_storage` after provisioning.\n"]
    pub fn ephemeral_storage(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ephemeral_storage", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `memory_in_mb` after provisioning.\n"]
    pub fn memory_in_mb(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.memory_in_mb", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout_in_seconds` after provisioning.\n"]
    pub fn timeout_in_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.timeout_in_seconds", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct SyntheticsCanaryScheduleElRetryConfigEl {
    max_retries: PrimField<f64>,
}

impl SyntheticsCanaryScheduleElRetryConfigEl {}

impl ToListMappable for SyntheticsCanaryScheduleElRetryConfigEl {
    type O = BlockAssignable<SyntheticsCanaryScheduleElRetryConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSyntheticsCanaryScheduleElRetryConfigEl {
    #[doc = ""]
    pub max_retries: PrimField<f64>,
}

impl BuildSyntheticsCanaryScheduleElRetryConfigEl {
    pub fn build(self) -> SyntheticsCanaryScheduleElRetryConfigEl {
        SyntheticsCanaryScheduleElRetryConfigEl {
            max_retries: self.max_retries,
        }
    }
}

pub struct SyntheticsCanaryScheduleElRetryConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SyntheticsCanaryScheduleElRetryConfigElRef {
    fn new(shared: StackShared, base: String) -> SyntheticsCanaryScheduleElRetryConfigElRef {
        SyntheticsCanaryScheduleElRetryConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SyntheticsCanaryScheduleElRetryConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_retries` after provisioning.\n"]
    pub fn max_retries(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_retries", self.base))
    }
}

#[derive(Serialize, Default)]
struct SyntheticsCanaryScheduleElDynamic {
    retry_config: Option<DynamicBlock<SyntheticsCanaryScheduleElRetryConfigEl>>,
}

#[derive(Serialize)]
pub struct SyntheticsCanaryScheduleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    duration_in_seconds: Option<PrimField<f64>>,
    expression: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retry_config: Option<Vec<SyntheticsCanaryScheduleElRetryConfigEl>>,
    dynamic: SyntheticsCanaryScheduleElDynamic,
}

impl SyntheticsCanaryScheduleEl {
    #[doc = "Set the field `duration_in_seconds`.\n"]
    pub fn set_duration_in_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.duration_in_seconds = Some(v.into());
        self
    }

    #[doc = "Set the field `retry_config`.\n"]
    pub fn set_retry_config(
        mut self,
        v: impl Into<BlockAssignable<SyntheticsCanaryScheduleElRetryConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.retry_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.retry_config = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for SyntheticsCanaryScheduleEl {
    type O = BlockAssignable<SyntheticsCanaryScheduleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSyntheticsCanaryScheduleEl {
    #[doc = ""]
    pub expression: PrimField<String>,
}

impl BuildSyntheticsCanaryScheduleEl {
    pub fn build(self) -> SyntheticsCanaryScheduleEl {
        SyntheticsCanaryScheduleEl {
            duration_in_seconds: core::default::Default::default(),
            expression: self.expression,
            retry_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SyntheticsCanaryScheduleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SyntheticsCanaryScheduleElRef {
    fn new(shared: StackShared, base: String) -> SyntheticsCanaryScheduleElRef {
        SyntheticsCanaryScheduleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SyntheticsCanaryScheduleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `duration_in_seconds` after provisioning.\n"]
    pub fn duration_in_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.duration_in_seconds", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `expression` after provisioning.\n"]
    pub fn expression(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }

    #[doc = "Get a reference to the value of field `retry_config` after provisioning.\n"]
    pub fn retry_config(&self) -> ListRef<SyntheticsCanaryScheduleElRetryConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.retry_config", self.base))
    }
}

#[derive(Serialize)]
pub struct SyntheticsCanaryVpcConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ipv6_allowed_for_dual_stack: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_group_ids: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnet_ids: Option<SetField<PrimField<String>>>,
}

impl SyntheticsCanaryVpcConfigEl {
    #[doc = "Set the field `ipv6_allowed_for_dual_stack`.\n"]
    pub fn set_ipv6_allowed_for_dual_stack(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.ipv6_allowed_for_dual_stack = Some(v.into());
        self
    }

    #[doc = "Set the field `security_group_ids`.\n"]
    pub fn set_security_group_ids(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.security_group_ids = Some(v.into());
        self
    }

    #[doc = "Set the field `subnet_ids`.\n"]
    pub fn set_subnet_ids(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.subnet_ids = Some(v.into());
        self
    }
}

impl ToListMappable for SyntheticsCanaryVpcConfigEl {
    type O = BlockAssignable<SyntheticsCanaryVpcConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSyntheticsCanaryVpcConfigEl {}

impl BuildSyntheticsCanaryVpcConfigEl {
    pub fn build(self) -> SyntheticsCanaryVpcConfigEl {
        SyntheticsCanaryVpcConfigEl {
            ipv6_allowed_for_dual_stack: core::default::Default::default(),
            security_group_ids: core::default::Default::default(),
            subnet_ids: core::default::Default::default(),
        }
    }
}

pub struct SyntheticsCanaryVpcConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SyntheticsCanaryVpcConfigElRef {
    fn new(shared: StackShared, base: String) -> SyntheticsCanaryVpcConfigElRef {
        SyntheticsCanaryVpcConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SyntheticsCanaryVpcConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ipv6_allowed_for_dual_stack` after provisioning.\n"]
    pub fn ipv6_allowed_for_dual_stack(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ipv6_allowed_for_dual_stack", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.security_group_ids", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct SyntheticsCanaryDynamic {
    artifact_config: Option<DynamicBlock<SyntheticsCanaryArtifactConfigEl>>,
    run_config: Option<DynamicBlock<SyntheticsCanaryRunConfigEl>>,
    schedule: Option<DynamicBlock<SyntheticsCanaryScheduleEl>>,
    vpc_config: Option<DynamicBlock<SyntheticsCanaryVpcConfigEl>>,
}
