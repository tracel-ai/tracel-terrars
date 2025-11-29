use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct RdsExportTaskData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    export_only: Option<ListField<PrimField<String>>>,
    export_task_identifier: PrimField<String>,
    iam_role_arn: PrimField<String>,
    kms_key_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    s3_bucket_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_prefix: Option<PrimField<String>>,
    source_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<RdsExportTaskTimeoutsEl>,
}

struct RdsExportTask_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<RdsExportTaskData>,
}

#[derive(Clone)]
pub struct RdsExportTask(Rc<RdsExportTask_>);

impl RdsExportTask {
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

    #[doc = "Set the field `export_only`.\n"]
    pub fn set_export_only(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().export_only = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `s3_prefix`.\n"]
    pub fn set_s3_prefix(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().s3_prefix = Some(v.into());
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<RdsExportTaskTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `export_only` after provisioning.\n"]
    pub fn export_only(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.export_only", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `export_task_identifier` after provisioning.\n"]
    pub fn export_task_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.export_task_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `failure_cause` after provisioning.\n"]
    pub fn failure_cause(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.failure_cause", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `iam_role_arn` after provisioning.\n"]
    pub fn iam_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.iam_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `percent_progress` after provisioning.\n"]
    pub fn percent_progress(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.percent_progress", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `s3_bucket_name` after provisioning.\n"]
    pub fn s3_bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_bucket_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `s3_prefix` after provisioning.\n"]
    pub fn s3_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_prefix", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `snapshot_time` after provisioning.\n"]
    pub fn snapshot_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.snapshot_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_arn` after provisioning.\n"]
    pub fn source_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_type` after provisioning.\n"]
    pub fn source_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `task_end_time` after provisioning.\n"]
    pub fn task_end_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.task_end_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `task_start_time` after provisioning.\n"]
    pub fn task_start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.task_start_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `warning_message` after provisioning.\n"]
    pub fn warning_message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.warning_message", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> RdsExportTaskTimeoutsElRef {
        RdsExportTaskTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for RdsExportTask {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for RdsExportTask { }

impl ToListMappable for RdsExportTask {
    type O = ListRef<RdsExportTaskRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for RdsExportTask_ {
    fn extract_resource_type(&self) -> String {
        "aws_rds_export_task".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildRdsExportTask {
    pub tf_id: String,
    #[doc = ""]
    pub export_task_identifier: PrimField<String>,
    #[doc = ""]
    pub iam_role_arn: PrimField<String>,
    #[doc = ""]
    pub kms_key_id: PrimField<String>,
    #[doc = ""]
    pub s3_bucket_name: PrimField<String>,
    #[doc = ""]
    pub source_arn: PrimField<String>,
}

impl BuildRdsExportTask {
    pub fn build(self, stack: &mut Stack) -> RdsExportTask {
        let out = RdsExportTask(Rc::new(RdsExportTask_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(RdsExportTaskData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                export_only: core::default::Default::default(),
                export_task_identifier: self.export_task_identifier,
                iam_role_arn: self.iam_role_arn,
                kms_key_id: self.kms_key_id,
                region: core::default::Default::default(),
                s3_bucket_name: self.s3_bucket_name,
                s3_prefix: core::default::Default::default(),
                source_arn: self.source_arn,
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct RdsExportTaskRef {
    shared: StackShared,
    base: String,
}

impl Ref for RdsExportTaskRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl RdsExportTaskRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `export_only` after provisioning.\n"]
    pub fn export_only(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.export_only", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `export_task_identifier` after provisioning.\n"]
    pub fn export_task_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.export_task_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `failure_cause` after provisioning.\n"]
    pub fn failure_cause(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.failure_cause", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `iam_role_arn` after provisioning.\n"]
    pub fn iam_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.iam_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `percent_progress` after provisioning.\n"]
    pub fn percent_progress(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.percent_progress", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `s3_bucket_name` after provisioning.\n"]
    pub fn s3_bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_bucket_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `s3_prefix` after provisioning.\n"]
    pub fn s3_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_prefix", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `snapshot_time` after provisioning.\n"]
    pub fn snapshot_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.snapshot_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_arn` after provisioning.\n"]
    pub fn source_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_type` after provisioning.\n"]
    pub fn source_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `task_end_time` after provisioning.\n"]
    pub fn task_end_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.task_end_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `task_start_time` after provisioning.\n"]
    pub fn task_start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.task_start_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `warning_message` after provisioning.\n"]
    pub fn warning_message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.warning_message", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> RdsExportTaskTimeoutsElRef {
        RdsExportTaskTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct RdsExportTaskTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}

impl RdsExportTaskTimeoutsEl {
    #[doc =
        "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }

    #[doc =
        "Set the field `delete`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }
}

impl ToListMappable for RdsExportTaskTimeoutsEl {
    type O = BlockAssignable<RdsExportTaskTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRdsExportTaskTimeoutsEl {}

impl BuildRdsExportTaskTimeoutsEl {
    pub fn build(self) -> RdsExportTaskTimeoutsEl {
        RdsExportTaskTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

pub struct RdsExportTaskTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RdsExportTaskTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> RdsExportTaskTimeoutsElRef {
        RdsExportTaskTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RdsExportTaskTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc =
        "Get a reference to the value of field `delete` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }
}
