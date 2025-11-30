use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct TransferWorkflowData {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    on_exception_steps: Option<Vec<TransferWorkflowOnExceptionStepsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    steps: Option<Vec<TransferWorkflowStepsEl>>,
    dynamic: TransferWorkflowDynamic,
}

struct TransferWorkflow_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<TransferWorkflowData>,
}

#[derive(Clone)]
pub struct TransferWorkflow(Rc<TransferWorkflow_>);

impl TransferWorkflow {
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

    #[doc = "Set the field `on_exception_steps`.\n"]
    pub fn set_on_exception_steps(
        self,
        v: impl Into<BlockAssignable<TransferWorkflowOnExceptionStepsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().on_exception_steps = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.on_exception_steps = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `steps`.\n"]
    pub fn set_steps(self, v: impl Into<BlockAssignable<TransferWorkflowStepsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().steps = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.steps = Some(d);
            }
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `on_exception_steps` after provisioning.\n"]
    pub fn on_exception_steps(&self) -> ListRef<TransferWorkflowOnExceptionStepsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.on_exception_steps", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `steps` after provisioning.\n"]
    pub fn steps(&self) -> ListRef<TransferWorkflowStepsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.steps", self.extract_ref()),
        )
    }
}

impl Referable for TransferWorkflow {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for TransferWorkflow {}

impl ToListMappable for TransferWorkflow {
    type O = ListRef<TransferWorkflowRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for TransferWorkflow_ {
    fn extract_resource_type(&self) -> String {
        "aws_transfer_workflow".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildTransferWorkflow {
    pub tf_id: String,
}

impl BuildTransferWorkflow {
    pub fn build(self, stack: &mut Stack) -> TransferWorkflow {
        let out = TransferWorkflow(Rc::new(TransferWorkflow_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(TransferWorkflowData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                on_exception_steps: core::default::Default::default(),
                steps: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct TransferWorkflowRef {
    shared: StackShared,
    base: String,
}

impl Ref for TransferWorkflowRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl TransferWorkflowRef {
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

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `on_exception_steps` after provisioning.\n"]
    pub fn on_exception_steps(&self) -> ListRef<TransferWorkflowOnExceptionStepsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.on_exception_steps", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `steps` after provisioning.\n"]
    pub fn steps(&self) -> ListRef<TransferWorkflowStepsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.steps", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationElEfsFileLocationEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    file_system_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
}

impl TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationElEfsFileLocationEl {
    #[doc = "Set the field `file_system_id`.\n"]
    pub fn set_file_system_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.file_system_id = Some(v.into());
        self
    }

    #[doc = "Set the field `path`.\n"]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }
}

impl ToListMappable for TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationElEfsFileLocationEl {
    type O =
        BlockAssignable<
            TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationElEfsFileLocationEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationElEfsFileLocationEl
{}

impl BuildTransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationElEfsFileLocationEl {
    pub fn build(
        self,
    ) -> TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationElEfsFileLocationEl {
        TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationElEfsFileLocationEl {
            file_system_id: core::default::Default::default(),
            path: core::default::Default::default(),
        }
    }
}

pub struct TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationElEfsFileLocationElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationElEfsFileLocationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationElEfsFileLocationElRef {
        TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationElEfsFileLocationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl
    TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationElEfsFileLocationElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `file_system_id` after provisioning.\n"]
    pub fn file_system_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.file_system_id", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
}

#[derive(Serialize)]
pub struct TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationElS3FileLocationEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
}

impl TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationElS3FileLocationEl {
    #[doc = "Set the field `bucket`.\n"]
    pub fn set_bucket(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket = Some(v.into());
        self
    }

    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }
}

impl ToListMappable
    for TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationElS3FileLocationEl
{
    type O =
        BlockAssignable<
            TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationElS3FileLocationEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationElS3FileLocationEl
{}

impl BuildTransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationElS3FileLocationEl {
    pub fn build(
        self,
    ) -> TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationElS3FileLocationEl {
        TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationElS3FileLocationEl {
            bucket: core::default::Default::default(),
            key: core::default::Default::default(),
        }
    }
}

pub struct TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationElS3FileLocationElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationElS3FileLocationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationElS3FileLocationElRef {
        TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationElS3FileLocationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl
    TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationElS3FileLocationElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
}

#[derive(Serialize, Default)]
struct TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationElDynamic {
    efs_file_location: Option<
        DynamicBlock<TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationElEfsFileLocationEl>,
    >,
    s3_file_location: Option<
        DynamicBlock<TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationElS3FileLocationEl>,
    >,
}

#[derive(Serialize)]
pub struct TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    efs_file_location: Option<
        Vec<TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationElEfsFileLocationEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_file_location: Option<
        Vec<TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationElS3FileLocationEl>,
    >,
    dynamic: TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationElDynamic,
}

impl TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationEl {
    #[doc = "Set the field `efs_file_location`.\n"]
    pub fn set_efs_file_location(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationElEfsFileLocationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.efs_file_location = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.efs_file_location = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `s3_file_location`.\n"]
    pub fn set_s3_file_location(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationElS3FileLocationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.s3_file_location = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_file_location = Some(d);
            }
        }
        self
    }
}

impl ToListMappable
    for TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationEl
{
    type O = BlockAssignable<
        TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationEl {}

impl BuildTransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationEl {
    pub fn build(
        self,
    ) -> TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationEl {
        TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationEl {
            efs_file_location: core::default::Default::default(),
            s3_file_location: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationElRef {
        TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `efs_file_location` after provisioning.\n"]
    pub fn efs_file_location(
        &self,
    ) -> ListRef<TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationElEfsFileLocationElRef>{
        ListRef::new(
            self.shared().clone(),
            format!("{}.efs_file_location", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `s3_file_location` after provisioning.\n"]
    pub fn s3_file_location(
        &self,
    ) -> ListRef<TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationElS3FileLocationElRef>{
        ListRef::new(
            self.shared().clone(),
            format!("{}.s3_file_location", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct TransferWorkflowOnExceptionStepsElCopyStepDetailsElDynamic {
    destination_file_location: Option<
        DynamicBlock<TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationEl>,
    >,
}

#[derive(Serialize)]
pub struct TransferWorkflowOnExceptionStepsElCopyStepDetailsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    overwrite_existing: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_file_location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_file_location:
        Option<Vec<TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationEl>>,
    dynamic: TransferWorkflowOnExceptionStepsElCopyStepDetailsElDynamic,
}

impl TransferWorkflowOnExceptionStepsElCopyStepDetailsEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc = "Set the field `overwrite_existing`.\n"]
    pub fn set_overwrite_existing(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.overwrite_existing = Some(v.into());
        self
    }

    #[doc = "Set the field `source_file_location`.\n"]
    pub fn set_source_file_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_file_location = Some(v.into());
        self
    }

    #[doc = "Set the field `destination_file_location`.\n"]
    pub fn set_destination_file_location(
        mut self,
        v: impl Into<
            BlockAssignable<
                TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.destination_file_location = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.destination_file_location = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for TransferWorkflowOnExceptionStepsElCopyStepDetailsEl {
    type O = BlockAssignable<TransferWorkflowOnExceptionStepsElCopyStepDetailsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTransferWorkflowOnExceptionStepsElCopyStepDetailsEl {}

impl BuildTransferWorkflowOnExceptionStepsElCopyStepDetailsEl {
    pub fn build(self) -> TransferWorkflowOnExceptionStepsElCopyStepDetailsEl {
        TransferWorkflowOnExceptionStepsElCopyStepDetailsEl {
            name: core::default::Default::default(),
            overwrite_existing: core::default::Default::default(),
            source_file_location: core::default::Default::default(),
            destination_file_location: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct TransferWorkflowOnExceptionStepsElCopyStepDetailsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for TransferWorkflowOnExceptionStepsElCopyStepDetailsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> TransferWorkflowOnExceptionStepsElCopyStepDetailsElRef {
        TransferWorkflowOnExceptionStepsElCopyStepDetailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TransferWorkflowOnExceptionStepsElCopyStepDetailsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `overwrite_existing` after provisioning.\n"]
    pub fn overwrite_existing(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.overwrite_existing", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `source_file_location` after provisioning.\n"]
    pub fn source_file_location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_file_location", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `destination_file_location` after provisioning.\n"]
    pub fn destination_file_location(
        &self,
    ) -> ListRef<TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.destination_file_location", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct TransferWorkflowOnExceptionStepsElCustomStepDetailsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_file_location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout_seconds: Option<PrimField<f64>>,
}

impl TransferWorkflowOnExceptionStepsElCustomStepDetailsEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc = "Set the field `source_file_location`.\n"]
    pub fn set_source_file_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_file_location = Some(v.into());
        self
    }

    #[doc = "Set the field `target`.\n"]
    pub fn set_target(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.target = Some(v.into());
        self
    }

    #[doc = "Set the field `timeout_seconds`.\n"]
    pub fn set_timeout_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.timeout_seconds = Some(v.into());
        self
    }
}

impl ToListMappable for TransferWorkflowOnExceptionStepsElCustomStepDetailsEl {
    type O = BlockAssignable<TransferWorkflowOnExceptionStepsElCustomStepDetailsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTransferWorkflowOnExceptionStepsElCustomStepDetailsEl {}

impl BuildTransferWorkflowOnExceptionStepsElCustomStepDetailsEl {
    pub fn build(self) -> TransferWorkflowOnExceptionStepsElCustomStepDetailsEl {
        TransferWorkflowOnExceptionStepsElCustomStepDetailsEl {
            name: core::default::Default::default(),
            source_file_location: core::default::Default::default(),
            target: core::default::Default::default(),
            timeout_seconds: core::default::Default::default(),
        }
    }
}

pub struct TransferWorkflowOnExceptionStepsElCustomStepDetailsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for TransferWorkflowOnExceptionStepsElCustomStepDetailsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> TransferWorkflowOnExceptionStepsElCustomStepDetailsElRef {
        TransferWorkflowOnExceptionStepsElCustomStepDetailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TransferWorkflowOnExceptionStepsElCustomStepDetailsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `source_file_location` after provisioning.\n"]
    pub fn source_file_location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_file_location", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout_seconds` after provisioning.\n"]
    pub fn timeout_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.timeout_seconds", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct TransferWorkflowOnExceptionStepsElDecryptStepDetailsElDestinationFileLocationElEfsFileLocationEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    file_system_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
}

impl
    TransferWorkflowOnExceptionStepsElDecryptStepDetailsElDestinationFileLocationElEfsFileLocationEl
{
    #[doc = "Set the field `file_system_id`.\n"]
    pub fn set_file_system_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.file_system_id = Some(v.into());
        self
    }

    #[doc = "Set the field `path`.\n"]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }
}

impl ToListMappable for TransferWorkflowOnExceptionStepsElDecryptStepDetailsElDestinationFileLocationElEfsFileLocationEl {
    type O =
        BlockAssignable<
            TransferWorkflowOnExceptionStepsElDecryptStepDetailsElDestinationFileLocationElEfsFileLocationEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTransferWorkflowOnExceptionStepsElDecryptStepDetailsElDestinationFileLocationElEfsFileLocationEl
{}

impl BuildTransferWorkflowOnExceptionStepsElDecryptStepDetailsElDestinationFileLocationElEfsFileLocationEl {
    pub fn build(
        self,
    ) -> TransferWorkflowOnExceptionStepsElDecryptStepDetailsElDestinationFileLocationElEfsFileLocationEl {
        TransferWorkflowOnExceptionStepsElDecryptStepDetailsElDestinationFileLocationElEfsFileLocationEl {
            file_system_id: core::default::Default::default(),
            path: core::default::Default::default(),
        }
    }
}

pub struct TransferWorkflowOnExceptionStepsElDecryptStepDetailsElDestinationFileLocationElEfsFileLocationElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for TransferWorkflowOnExceptionStepsElDecryptStepDetailsElDestinationFileLocationElEfsFileLocationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> TransferWorkflowOnExceptionStepsElDecryptStepDetailsElDestinationFileLocationElEfsFileLocationElRef {
        TransferWorkflowOnExceptionStepsElDecryptStepDetailsElDestinationFileLocationElEfsFileLocationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TransferWorkflowOnExceptionStepsElDecryptStepDetailsElDestinationFileLocationElEfsFileLocationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `file_system_id` after provisioning.\n"]
    pub fn file_system_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.file_system_id", self.base))
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
}

#[derive(Serialize)]
pub struct TransferWorkflowOnExceptionStepsElDecryptStepDetailsElDestinationFileLocationElS3FileLocationEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
}

impl
    TransferWorkflowOnExceptionStepsElDecryptStepDetailsElDestinationFileLocationElS3FileLocationEl
{
    #[doc = "Set the field `bucket`.\n"]
    pub fn set_bucket(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket = Some(v.into());
        self
    }

    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }
}

impl ToListMappable for TransferWorkflowOnExceptionStepsElDecryptStepDetailsElDestinationFileLocationElS3FileLocationEl {
    type O =
        BlockAssignable<
            TransferWorkflowOnExceptionStepsElDecryptStepDetailsElDestinationFileLocationElS3FileLocationEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTransferWorkflowOnExceptionStepsElDecryptStepDetailsElDestinationFileLocationElS3FileLocationEl
{}

impl BuildTransferWorkflowOnExceptionStepsElDecryptStepDetailsElDestinationFileLocationElS3FileLocationEl {
    pub fn build(
        self,
    ) -> TransferWorkflowOnExceptionStepsElDecryptStepDetailsElDestinationFileLocationElS3FileLocationEl {
        TransferWorkflowOnExceptionStepsElDecryptStepDetailsElDestinationFileLocationElS3FileLocationEl {
            bucket: core::default::Default::default(),
            key: core::default::Default::default(),
        }
    }
}

pub struct TransferWorkflowOnExceptionStepsElDecryptStepDetailsElDestinationFileLocationElS3FileLocationElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for TransferWorkflowOnExceptionStepsElDecryptStepDetailsElDestinationFileLocationElS3FileLocationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> TransferWorkflowOnExceptionStepsElDecryptStepDetailsElDestinationFileLocationElS3FileLocationElRef {
        TransferWorkflowOnExceptionStepsElDecryptStepDetailsElDestinationFileLocationElS3FileLocationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TransferWorkflowOnExceptionStepsElDecryptStepDetailsElDestinationFileLocationElS3FileLocationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
}

#[derive(Serialize, Default)]
struct TransferWorkflowOnExceptionStepsElDecryptStepDetailsElDestinationFileLocationElDynamic {
    efs_file_location: Option<
        DynamicBlock<TransferWorkflowOnExceptionStepsElDecryptStepDetailsElDestinationFileLocationElEfsFileLocationEl>,
    >,
    s3_file_location: Option<
        DynamicBlock<TransferWorkflowOnExceptionStepsElDecryptStepDetailsElDestinationFileLocationElS3FileLocationEl>,
    >,
}

#[derive(Serialize)]
pub struct TransferWorkflowOnExceptionStepsElDecryptStepDetailsElDestinationFileLocationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    efs_file_location: Option<
        Vec<TransferWorkflowOnExceptionStepsElDecryptStepDetailsElDestinationFileLocationElEfsFileLocationEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_file_location: Option<
        Vec<TransferWorkflowOnExceptionStepsElDecryptStepDetailsElDestinationFileLocationElS3FileLocationEl>,
    >,
    dynamic: TransferWorkflowOnExceptionStepsElDecryptStepDetailsElDestinationFileLocationElDynamic,
}

impl TransferWorkflowOnExceptionStepsElDecryptStepDetailsElDestinationFileLocationEl {
    #[doc = "Set the field `efs_file_location`.\n"]
    pub fn set_efs_file_location(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            TransferWorkflowOnExceptionStepsElDecryptStepDetailsElDestinationFileLocationElEfsFileLocationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.efs_file_location = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.efs_file_location = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `s3_file_location`.\n"]
    pub fn set_s3_file_location(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            TransferWorkflowOnExceptionStepsElDecryptStepDetailsElDestinationFileLocationElS3FileLocationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.s3_file_location = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_file_location = Some(d);
            }
        }
        self
    }
}

impl ToListMappable
    for TransferWorkflowOnExceptionStepsElDecryptStepDetailsElDestinationFileLocationEl
{
    type O = BlockAssignable<
        TransferWorkflowOnExceptionStepsElDecryptStepDetailsElDestinationFileLocationEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTransferWorkflowOnExceptionStepsElDecryptStepDetailsElDestinationFileLocationEl {}

impl BuildTransferWorkflowOnExceptionStepsElDecryptStepDetailsElDestinationFileLocationEl {
    pub fn build(
        self,
    ) -> TransferWorkflowOnExceptionStepsElDecryptStepDetailsElDestinationFileLocationEl {
        TransferWorkflowOnExceptionStepsElDecryptStepDetailsElDestinationFileLocationEl {
            efs_file_location: core::default::Default::default(),
            s3_file_location: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct TransferWorkflowOnExceptionStepsElDecryptStepDetailsElDestinationFileLocationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for TransferWorkflowOnExceptionStepsElDecryptStepDetailsElDestinationFileLocationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> TransferWorkflowOnExceptionStepsElDecryptStepDetailsElDestinationFileLocationElRef {
        TransferWorkflowOnExceptionStepsElDecryptStepDetailsElDestinationFileLocationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TransferWorkflowOnExceptionStepsElDecryptStepDetailsElDestinationFileLocationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `efs_file_location` after provisioning.\n"]
    pub fn efs_file_location(
        &self,
    ) -> ListRef<TransferWorkflowOnExceptionStepsElDecryptStepDetailsElDestinationFileLocationElEfsFileLocationElRef>{
        ListRef::new(
            self.shared().clone(),
            format!("{}.efs_file_location", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `s3_file_location` after provisioning.\n"]
    pub fn s3_file_location(
        &self,
    ) -> ListRef<TransferWorkflowOnExceptionStepsElDecryptStepDetailsElDestinationFileLocationElS3FileLocationElRef>{
        ListRef::new(
            self.shared().clone(),
            format!("{}.s3_file_location", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct TransferWorkflowOnExceptionStepsElDecryptStepDetailsElDynamic {
    destination_file_location: Option<
        DynamicBlock<
            TransferWorkflowOnExceptionStepsElDecryptStepDetailsElDestinationFileLocationEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct TransferWorkflowOnExceptionStepsElDecryptStepDetailsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    overwrite_existing: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_file_location: Option<PrimField<String>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_file_location: Option<
        Vec<TransferWorkflowOnExceptionStepsElDecryptStepDetailsElDestinationFileLocationEl>,
    >,
    dynamic: TransferWorkflowOnExceptionStepsElDecryptStepDetailsElDynamic,
}

impl TransferWorkflowOnExceptionStepsElDecryptStepDetailsEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc = "Set the field `overwrite_existing`.\n"]
    pub fn set_overwrite_existing(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.overwrite_existing = Some(v.into());
        self
    }

    #[doc = "Set the field `source_file_location`.\n"]
    pub fn set_source_file_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_file_location = Some(v.into());
        self
    }

    #[doc = "Set the field `destination_file_location`.\n"]
    pub fn set_destination_file_location(
        mut self,
        v: impl Into<
            BlockAssignable<
                TransferWorkflowOnExceptionStepsElDecryptStepDetailsElDestinationFileLocationEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.destination_file_location = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.destination_file_location = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for TransferWorkflowOnExceptionStepsElDecryptStepDetailsEl {
    type O = BlockAssignable<TransferWorkflowOnExceptionStepsElDecryptStepDetailsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTransferWorkflowOnExceptionStepsElDecryptStepDetailsEl {
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl BuildTransferWorkflowOnExceptionStepsElDecryptStepDetailsEl {
    pub fn build(self) -> TransferWorkflowOnExceptionStepsElDecryptStepDetailsEl {
        TransferWorkflowOnExceptionStepsElDecryptStepDetailsEl {
            name: core::default::Default::default(),
            overwrite_existing: core::default::Default::default(),
            source_file_location: core::default::Default::default(),
            type_: self.type_,
            destination_file_location: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct TransferWorkflowOnExceptionStepsElDecryptStepDetailsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for TransferWorkflowOnExceptionStepsElDecryptStepDetailsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> TransferWorkflowOnExceptionStepsElDecryptStepDetailsElRef {
        TransferWorkflowOnExceptionStepsElDecryptStepDetailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TransferWorkflowOnExceptionStepsElDecryptStepDetailsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `overwrite_existing` after provisioning.\n"]
    pub fn overwrite_existing(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.overwrite_existing", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `source_file_location` after provisioning.\n"]
    pub fn source_file_location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_file_location", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `destination_file_location` after provisioning.\n"]
    pub fn destination_file_location(
        &self,
    ) -> ListRef<TransferWorkflowOnExceptionStepsElDecryptStepDetailsElDestinationFileLocationElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.destination_file_location", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct TransferWorkflowOnExceptionStepsElDeleteStepDetailsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_file_location: Option<PrimField<String>>,
}

impl TransferWorkflowOnExceptionStepsElDeleteStepDetailsEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc = "Set the field `source_file_location`.\n"]
    pub fn set_source_file_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_file_location = Some(v.into());
        self
    }
}

impl ToListMappable for TransferWorkflowOnExceptionStepsElDeleteStepDetailsEl {
    type O = BlockAssignable<TransferWorkflowOnExceptionStepsElDeleteStepDetailsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTransferWorkflowOnExceptionStepsElDeleteStepDetailsEl {}

impl BuildTransferWorkflowOnExceptionStepsElDeleteStepDetailsEl {
    pub fn build(self) -> TransferWorkflowOnExceptionStepsElDeleteStepDetailsEl {
        TransferWorkflowOnExceptionStepsElDeleteStepDetailsEl {
            name: core::default::Default::default(),
            source_file_location: core::default::Default::default(),
        }
    }
}

pub struct TransferWorkflowOnExceptionStepsElDeleteStepDetailsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for TransferWorkflowOnExceptionStepsElDeleteStepDetailsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> TransferWorkflowOnExceptionStepsElDeleteStepDetailsElRef {
        TransferWorkflowOnExceptionStepsElDeleteStepDetailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TransferWorkflowOnExceptionStepsElDeleteStepDetailsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `source_file_location` after provisioning.\n"]
    pub fn source_file_location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_file_location", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct TransferWorkflowOnExceptionStepsElTagStepDetailsElTagsEl {
    key: PrimField<String>,
    value: PrimField<String>,
}

impl TransferWorkflowOnExceptionStepsElTagStepDetailsElTagsEl {}

impl ToListMappable for TransferWorkflowOnExceptionStepsElTagStepDetailsElTagsEl {
    type O = BlockAssignable<TransferWorkflowOnExceptionStepsElTagStepDetailsElTagsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTransferWorkflowOnExceptionStepsElTagStepDetailsElTagsEl {
    #[doc = ""]
    pub key: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildTransferWorkflowOnExceptionStepsElTagStepDetailsElTagsEl {
    pub fn build(self) -> TransferWorkflowOnExceptionStepsElTagStepDetailsElTagsEl {
        TransferWorkflowOnExceptionStepsElTagStepDetailsElTagsEl {
            key: self.key,
            value: self.value,
        }
    }
}

pub struct TransferWorkflowOnExceptionStepsElTagStepDetailsElTagsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for TransferWorkflowOnExceptionStepsElTagStepDetailsElTagsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> TransferWorkflowOnExceptionStepsElTagStepDetailsElTagsElRef {
        TransferWorkflowOnExceptionStepsElTagStepDetailsElTagsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TransferWorkflowOnExceptionStepsElTagStepDetailsElTagsElRef {
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
struct TransferWorkflowOnExceptionStepsElTagStepDetailsElDynamic {
    tags: Option<DynamicBlock<TransferWorkflowOnExceptionStepsElTagStepDetailsElTagsEl>>,
}

#[derive(Serialize)]
pub struct TransferWorkflowOnExceptionStepsElTagStepDetailsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_file_location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<TransferWorkflowOnExceptionStepsElTagStepDetailsElTagsEl>>,
    dynamic: TransferWorkflowOnExceptionStepsElTagStepDetailsElDynamic,
}

impl TransferWorkflowOnExceptionStepsElTagStepDetailsEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc = "Set the field `source_file_location`.\n"]
    pub fn set_source_file_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_file_location = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        mut self,
        v: impl Into<BlockAssignable<TransferWorkflowOnExceptionStepsElTagStepDetailsElTagsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.tags = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.tags = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for TransferWorkflowOnExceptionStepsElTagStepDetailsEl {
    type O = BlockAssignable<TransferWorkflowOnExceptionStepsElTagStepDetailsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTransferWorkflowOnExceptionStepsElTagStepDetailsEl {}

impl BuildTransferWorkflowOnExceptionStepsElTagStepDetailsEl {
    pub fn build(self) -> TransferWorkflowOnExceptionStepsElTagStepDetailsEl {
        TransferWorkflowOnExceptionStepsElTagStepDetailsEl {
            name: core::default::Default::default(),
            source_file_location: core::default::Default::default(),
            tags: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct TransferWorkflowOnExceptionStepsElTagStepDetailsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for TransferWorkflowOnExceptionStepsElTagStepDetailsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> TransferWorkflowOnExceptionStepsElTagStepDetailsElRef {
        TransferWorkflowOnExceptionStepsElTagStepDetailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TransferWorkflowOnExceptionStepsElTagStepDetailsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `source_file_location` after provisioning.\n"]
    pub fn source_file_location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_file_location", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> ListRef<TransferWorkflowOnExceptionStepsElTagStepDetailsElTagsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize, Default)]
struct TransferWorkflowOnExceptionStepsElDynamic {
    copy_step_details: Option<DynamicBlock<TransferWorkflowOnExceptionStepsElCopyStepDetailsEl>>,
    custom_step_details:
        Option<DynamicBlock<TransferWorkflowOnExceptionStepsElCustomStepDetailsEl>>,
    decrypt_step_details:
        Option<DynamicBlock<TransferWorkflowOnExceptionStepsElDecryptStepDetailsEl>>,
    delete_step_details:
        Option<DynamicBlock<TransferWorkflowOnExceptionStepsElDeleteStepDetailsEl>>,
    tag_step_details: Option<DynamicBlock<TransferWorkflowOnExceptionStepsElTagStepDetailsEl>>,
}

#[derive(Serialize)]
pub struct TransferWorkflowOnExceptionStepsEl {
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    copy_step_details: Option<Vec<TransferWorkflowOnExceptionStepsElCopyStepDetailsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_step_details: Option<Vec<TransferWorkflowOnExceptionStepsElCustomStepDetailsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    decrypt_step_details: Option<Vec<TransferWorkflowOnExceptionStepsElDecryptStepDetailsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete_step_details: Option<Vec<TransferWorkflowOnExceptionStepsElDeleteStepDetailsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag_step_details: Option<Vec<TransferWorkflowOnExceptionStepsElTagStepDetailsEl>>,
    dynamic: TransferWorkflowOnExceptionStepsElDynamic,
}

impl TransferWorkflowOnExceptionStepsEl {
    #[doc = "Set the field `copy_step_details`.\n"]
    pub fn set_copy_step_details(
        mut self,
        v: impl Into<BlockAssignable<TransferWorkflowOnExceptionStepsElCopyStepDetailsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.copy_step_details = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.copy_step_details = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `custom_step_details`.\n"]
    pub fn set_custom_step_details(
        mut self,
        v: impl Into<BlockAssignable<TransferWorkflowOnExceptionStepsElCustomStepDetailsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.custom_step_details = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_step_details = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `decrypt_step_details`.\n"]
    pub fn set_decrypt_step_details(
        mut self,
        v: impl Into<BlockAssignable<TransferWorkflowOnExceptionStepsElDecryptStepDetailsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.decrypt_step_details = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.decrypt_step_details = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `delete_step_details`.\n"]
    pub fn set_delete_step_details(
        mut self,
        v: impl Into<BlockAssignable<TransferWorkflowOnExceptionStepsElDeleteStepDetailsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.delete_step_details = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.delete_step_details = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `tag_step_details`.\n"]
    pub fn set_tag_step_details(
        mut self,
        v: impl Into<BlockAssignable<TransferWorkflowOnExceptionStepsElTagStepDetailsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.tag_step_details = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.tag_step_details = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for TransferWorkflowOnExceptionStepsEl {
    type O = BlockAssignable<TransferWorkflowOnExceptionStepsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTransferWorkflowOnExceptionStepsEl {
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl BuildTransferWorkflowOnExceptionStepsEl {
    pub fn build(self) -> TransferWorkflowOnExceptionStepsEl {
        TransferWorkflowOnExceptionStepsEl {
            type_: self.type_,
            copy_step_details: core::default::Default::default(),
            custom_step_details: core::default::Default::default(),
            decrypt_step_details: core::default::Default::default(),
            delete_step_details: core::default::Default::default(),
            tag_step_details: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct TransferWorkflowOnExceptionStepsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for TransferWorkflowOnExceptionStepsElRef {
    fn new(shared: StackShared, base: String) -> TransferWorkflowOnExceptionStepsElRef {
        TransferWorkflowOnExceptionStepsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TransferWorkflowOnExceptionStepsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `copy_step_details` after provisioning.\n"]
    pub fn copy_step_details(
        &self,
    ) -> ListRef<TransferWorkflowOnExceptionStepsElCopyStepDetailsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.copy_step_details", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `custom_step_details` after provisioning.\n"]
    pub fn custom_step_details(
        &self,
    ) -> ListRef<TransferWorkflowOnExceptionStepsElCustomStepDetailsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_step_details", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `decrypt_step_details` after provisioning.\n"]
    pub fn decrypt_step_details(
        &self,
    ) -> ListRef<TransferWorkflowOnExceptionStepsElDecryptStepDetailsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.decrypt_step_details", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `delete_step_details` after provisioning.\n"]
    pub fn delete_step_details(
        &self,
    ) -> ListRef<TransferWorkflowOnExceptionStepsElDeleteStepDetailsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.delete_step_details", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `tag_step_details` after provisioning.\n"]
    pub fn tag_step_details(
        &self,
    ) -> ListRef<TransferWorkflowOnExceptionStepsElTagStepDetailsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tag_step_details", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationElEfsFileLocationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    file_system_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
}

impl TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationElEfsFileLocationEl {
    #[doc = "Set the field `file_system_id`.\n"]
    pub fn set_file_system_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.file_system_id = Some(v.into());
        self
    }

    #[doc = "Set the field `path`.\n"]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }
}

impl ToListMappable
    for TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationElEfsFileLocationEl
{
    type O = BlockAssignable<
        TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationElEfsFileLocationEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationElEfsFileLocationEl
{}

impl BuildTransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationElEfsFileLocationEl {
    pub fn build(
        self,
    ) -> TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationElEfsFileLocationEl {
        TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationElEfsFileLocationEl {
            file_system_id: core::default::Default::default(),
            path: core::default::Default::default(),
        }
    }
}

pub struct TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationElEfsFileLocationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationElEfsFileLocationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationElEfsFileLocationElRef {
        TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationElEfsFileLocationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationElEfsFileLocationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `file_system_id` after provisioning.\n"]
    pub fn file_system_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.file_system_id", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
}

#[derive(Serialize)]
pub struct TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationElS3FileLocationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
}

impl TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationElS3FileLocationEl {
    #[doc = "Set the field `bucket`.\n"]
    pub fn set_bucket(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket = Some(v.into());
        self
    }

    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }
}

impl ToListMappable
    for TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationElS3FileLocationEl
{
    type O = BlockAssignable<
        TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationElS3FileLocationEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationElS3FileLocationEl {
}

impl BuildTransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationElS3FileLocationEl {
    pub fn build(
        self,
    ) -> TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationElS3FileLocationEl {
        TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationElS3FileLocationEl {
            bucket: core::default::Default::default(),
            key: core::default::Default::default(),
        }
    }
}

pub struct TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationElS3FileLocationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationElS3FileLocationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationElS3FileLocationElRef {
        TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationElS3FileLocationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationElS3FileLocationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
}

#[derive(Serialize, Default)]
struct TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationElDynamic {
    efs_file_location: Option<
        DynamicBlock<
            TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationElEfsFileLocationEl,
        >,
    >,
    s3_file_location: Option<
        DynamicBlock<
            TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationElS3FileLocationEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    efs_file_location: Option<
        Vec<TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationElEfsFileLocationEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_file_location: Option<
        Vec<TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationElS3FileLocationEl>,
    >,
    dynamic: TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationElDynamic,
}

impl TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationEl {
    #[doc = "Set the field `efs_file_location`.\n"]
    pub fn set_efs_file_location(
        mut self,
        v: impl Into<
            BlockAssignable<
                TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationElEfsFileLocationEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.efs_file_location = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.efs_file_location = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `s3_file_location`.\n"]
    pub fn set_s3_file_location(
        mut self,
        v: impl Into<
            BlockAssignable<
                TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationElS3FileLocationEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.s3_file_location = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_file_location = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationEl {
    type O = BlockAssignable<TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationEl {}

impl BuildTransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationEl {
    pub fn build(self) -> TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationEl {
        TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationEl {
            efs_file_location: core::default::Default::default(),
            s3_file_location: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationElRef {
        TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `efs_file_location` after provisioning.\n"]
    pub fn efs_file_location(
        &self,
    ) -> ListRef<
        TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationElEfsFileLocationElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.efs_file_location", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `s3_file_location` after provisioning.\n"]
    pub fn s3_file_location(
        &self,
    ) -> ListRef<TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationElS3FileLocationElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.s3_file_location", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct TransferWorkflowStepsElCopyStepDetailsElDynamic {
    destination_file_location:
        Option<DynamicBlock<TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationEl>>,
}

#[derive(Serialize)]
pub struct TransferWorkflowStepsElCopyStepDetailsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    overwrite_existing: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_file_location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_file_location:
        Option<Vec<TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationEl>>,
    dynamic: TransferWorkflowStepsElCopyStepDetailsElDynamic,
}

impl TransferWorkflowStepsElCopyStepDetailsEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc = "Set the field `overwrite_existing`.\n"]
    pub fn set_overwrite_existing(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.overwrite_existing = Some(v.into());
        self
    }

    #[doc = "Set the field `source_file_location`.\n"]
    pub fn set_source_file_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_file_location = Some(v.into());
        self
    }

    #[doc = "Set the field `destination_file_location`.\n"]
    pub fn set_destination_file_location(
        mut self,
        v: impl Into<BlockAssignable<TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.destination_file_location = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.destination_file_location = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for TransferWorkflowStepsElCopyStepDetailsEl {
    type O = BlockAssignable<TransferWorkflowStepsElCopyStepDetailsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTransferWorkflowStepsElCopyStepDetailsEl {}

impl BuildTransferWorkflowStepsElCopyStepDetailsEl {
    pub fn build(self) -> TransferWorkflowStepsElCopyStepDetailsEl {
        TransferWorkflowStepsElCopyStepDetailsEl {
            name: core::default::Default::default(),
            overwrite_existing: core::default::Default::default(),
            source_file_location: core::default::Default::default(),
            destination_file_location: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct TransferWorkflowStepsElCopyStepDetailsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for TransferWorkflowStepsElCopyStepDetailsElRef {
    fn new(shared: StackShared, base: String) -> TransferWorkflowStepsElCopyStepDetailsElRef {
        TransferWorkflowStepsElCopyStepDetailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TransferWorkflowStepsElCopyStepDetailsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `overwrite_existing` after provisioning.\n"]
    pub fn overwrite_existing(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.overwrite_existing", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `source_file_location` after provisioning.\n"]
    pub fn source_file_location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_file_location", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `destination_file_location` after provisioning.\n"]
    pub fn destination_file_location(
        &self,
    ) -> ListRef<TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.destination_file_location", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct TransferWorkflowStepsElCustomStepDetailsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_file_location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout_seconds: Option<PrimField<f64>>,
}

impl TransferWorkflowStepsElCustomStepDetailsEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc = "Set the field `source_file_location`.\n"]
    pub fn set_source_file_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_file_location = Some(v.into());
        self
    }

    #[doc = "Set the field `target`.\n"]
    pub fn set_target(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.target = Some(v.into());
        self
    }

    #[doc = "Set the field `timeout_seconds`.\n"]
    pub fn set_timeout_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.timeout_seconds = Some(v.into());
        self
    }
}

impl ToListMappable for TransferWorkflowStepsElCustomStepDetailsEl {
    type O = BlockAssignable<TransferWorkflowStepsElCustomStepDetailsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTransferWorkflowStepsElCustomStepDetailsEl {}

impl BuildTransferWorkflowStepsElCustomStepDetailsEl {
    pub fn build(self) -> TransferWorkflowStepsElCustomStepDetailsEl {
        TransferWorkflowStepsElCustomStepDetailsEl {
            name: core::default::Default::default(),
            source_file_location: core::default::Default::default(),
            target: core::default::Default::default(),
            timeout_seconds: core::default::Default::default(),
        }
    }
}

pub struct TransferWorkflowStepsElCustomStepDetailsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for TransferWorkflowStepsElCustomStepDetailsElRef {
    fn new(shared: StackShared, base: String) -> TransferWorkflowStepsElCustomStepDetailsElRef {
        TransferWorkflowStepsElCustomStepDetailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TransferWorkflowStepsElCustomStepDetailsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `source_file_location` after provisioning.\n"]
    pub fn source_file_location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_file_location", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout_seconds` after provisioning.\n"]
    pub fn timeout_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.timeout_seconds", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct TransferWorkflowStepsElDecryptStepDetailsElDestinationFileLocationElEfsFileLocationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    file_system_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
}

impl TransferWorkflowStepsElDecryptStepDetailsElDestinationFileLocationElEfsFileLocationEl {
    #[doc = "Set the field `file_system_id`.\n"]
    pub fn set_file_system_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.file_system_id = Some(v.into());
        self
    }

    #[doc = "Set the field `path`.\n"]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }
}

impl ToListMappable
    for TransferWorkflowStepsElDecryptStepDetailsElDestinationFileLocationElEfsFileLocationEl
{
    type O = BlockAssignable<
        TransferWorkflowStepsElDecryptStepDetailsElDestinationFileLocationElEfsFileLocationEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTransferWorkflowStepsElDecryptStepDetailsElDestinationFileLocationElEfsFileLocationEl
{}

impl BuildTransferWorkflowStepsElDecryptStepDetailsElDestinationFileLocationElEfsFileLocationEl {
    pub fn build(
        self,
    ) -> TransferWorkflowStepsElDecryptStepDetailsElDestinationFileLocationElEfsFileLocationEl {
        TransferWorkflowStepsElDecryptStepDetailsElDestinationFileLocationElEfsFileLocationEl {
            file_system_id: core::default::Default::default(),
            path: core::default::Default::default(),
        }
    }
}

pub struct TransferWorkflowStepsElDecryptStepDetailsElDestinationFileLocationElEfsFileLocationElRef
{
    shared: StackShared,
    base: String,
}

impl Ref
    for TransferWorkflowStepsElDecryptStepDetailsElDestinationFileLocationElEfsFileLocationElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> TransferWorkflowStepsElDecryptStepDetailsElDestinationFileLocationElEfsFileLocationElRef
    {
        TransferWorkflowStepsElDecryptStepDetailsElDestinationFileLocationElEfsFileLocationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TransferWorkflowStepsElDecryptStepDetailsElDestinationFileLocationElEfsFileLocationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `file_system_id` after provisioning.\n"]
    pub fn file_system_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.file_system_id", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
}

#[derive(Serialize)]
pub struct TransferWorkflowStepsElDecryptStepDetailsElDestinationFileLocationElS3FileLocationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
}

impl TransferWorkflowStepsElDecryptStepDetailsElDestinationFileLocationElS3FileLocationEl {
    #[doc = "Set the field `bucket`.\n"]
    pub fn set_bucket(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket = Some(v.into());
        self
    }

    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }
}

impl ToListMappable
    for TransferWorkflowStepsElDecryptStepDetailsElDestinationFileLocationElS3FileLocationEl
{
    type O = BlockAssignable<
        TransferWorkflowStepsElDecryptStepDetailsElDestinationFileLocationElS3FileLocationEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTransferWorkflowStepsElDecryptStepDetailsElDestinationFileLocationElS3FileLocationEl
{}

impl BuildTransferWorkflowStepsElDecryptStepDetailsElDestinationFileLocationElS3FileLocationEl {
    pub fn build(
        self,
    ) -> TransferWorkflowStepsElDecryptStepDetailsElDestinationFileLocationElS3FileLocationEl {
        TransferWorkflowStepsElDecryptStepDetailsElDestinationFileLocationElS3FileLocationEl {
            bucket: core::default::Default::default(),
            key: core::default::Default::default(),
        }
    }
}

pub struct TransferWorkflowStepsElDecryptStepDetailsElDestinationFileLocationElS3FileLocationElRef {
    shared: StackShared,
    base: String,
}

impl Ref
    for TransferWorkflowStepsElDecryptStepDetailsElDestinationFileLocationElS3FileLocationElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> TransferWorkflowStepsElDecryptStepDetailsElDestinationFileLocationElS3FileLocationElRef
    {
        TransferWorkflowStepsElDecryptStepDetailsElDestinationFileLocationElS3FileLocationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TransferWorkflowStepsElDecryptStepDetailsElDestinationFileLocationElS3FileLocationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
}

#[derive(Serialize, Default)]
struct TransferWorkflowStepsElDecryptStepDetailsElDestinationFileLocationElDynamic {
    efs_file_location: Option<
        DynamicBlock<
            TransferWorkflowStepsElDecryptStepDetailsElDestinationFileLocationElEfsFileLocationEl,
        >,
    >,
    s3_file_location: Option<
        DynamicBlock<
            TransferWorkflowStepsElDecryptStepDetailsElDestinationFileLocationElS3FileLocationEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct TransferWorkflowStepsElDecryptStepDetailsElDestinationFileLocationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    efs_file_location: Option<
        Vec<TransferWorkflowStepsElDecryptStepDetailsElDestinationFileLocationElEfsFileLocationEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_file_location: Option<
        Vec<TransferWorkflowStepsElDecryptStepDetailsElDestinationFileLocationElS3FileLocationEl>,
    >,
    dynamic: TransferWorkflowStepsElDecryptStepDetailsElDestinationFileLocationElDynamic,
}

impl TransferWorkflowStepsElDecryptStepDetailsElDestinationFileLocationEl {
    #[doc = "Set the field `efs_file_location`.\n"]
    pub fn set_efs_file_location(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            TransferWorkflowStepsElDecryptStepDetailsElDestinationFileLocationElEfsFileLocationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.efs_file_location = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.efs_file_location = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `s3_file_location`.\n"]
    pub fn set_s3_file_location(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            TransferWorkflowStepsElDecryptStepDetailsElDestinationFileLocationElS3FileLocationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.s3_file_location = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_file_location = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for TransferWorkflowStepsElDecryptStepDetailsElDestinationFileLocationEl {
    type O = BlockAssignable<TransferWorkflowStepsElDecryptStepDetailsElDestinationFileLocationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTransferWorkflowStepsElDecryptStepDetailsElDestinationFileLocationEl {}

impl BuildTransferWorkflowStepsElDecryptStepDetailsElDestinationFileLocationEl {
    pub fn build(self) -> TransferWorkflowStepsElDecryptStepDetailsElDestinationFileLocationEl {
        TransferWorkflowStepsElDecryptStepDetailsElDestinationFileLocationEl {
            efs_file_location: core::default::Default::default(),
            s3_file_location: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct TransferWorkflowStepsElDecryptStepDetailsElDestinationFileLocationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for TransferWorkflowStepsElDecryptStepDetailsElDestinationFileLocationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> TransferWorkflowStepsElDecryptStepDetailsElDestinationFileLocationElRef {
        TransferWorkflowStepsElDecryptStepDetailsElDestinationFileLocationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TransferWorkflowStepsElDecryptStepDetailsElDestinationFileLocationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `efs_file_location` after provisioning.\n"]
    pub fn efs_file_location(
        &self,
    ) -> ListRef<
        TransferWorkflowStepsElDecryptStepDetailsElDestinationFileLocationElEfsFileLocationElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.efs_file_location", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `s3_file_location` after provisioning.\n"]
    pub fn s3_file_location(
        &self,
    ) -> ListRef<
        TransferWorkflowStepsElDecryptStepDetailsElDestinationFileLocationElS3FileLocationElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.s3_file_location", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct TransferWorkflowStepsElDecryptStepDetailsElDynamic {
    destination_file_location:
        Option<DynamicBlock<TransferWorkflowStepsElDecryptStepDetailsElDestinationFileLocationEl>>,
}

#[derive(Serialize)]
pub struct TransferWorkflowStepsElDecryptStepDetailsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    overwrite_existing: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_file_location: Option<PrimField<String>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_file_location:
        Option<Vec<TransferWorkflowStepsElDecryptStepDetailsElDestinationFileLocationEl>>,
    dynamic: TransferWorkflowStepsElDecryptStepDetailsElDynamic,
}

impl TransferWorkflowStepsElDecryptStepDetailsEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc = "Set the field `overwrite_existing`.\n"]
    pub fn set_overwrite_existing(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.overwrite_existing = Some(v.into());
        self
    }

    #[doc = "Set the field `source_file_location`.\n"]
    pub fn set_source_file_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_file_location = Some(v.into());
        self
    }

    #[doc = "Set the field `destination_file_location`.\n"]
    pub fn set_destination_file_location(
        mut self,
        v: impl Into<
            BlockAssignable<TransferWorkflowStepsElDecryptStepDetailsElDestinationFileLocationEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.destination_file_location = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.destination_file_location = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for TransferWorkflowStepsElDecryptStepDetailsEl {
    type O = BlockAssignable<TransferWorkflowStepsElDecryptStepDetailsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTransferWorkflowStepsElDecryptStepDetailsEl {
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl BuildTransferWorkflowStepsElDecryptStepDetailsEl {
    pub fn build(self) -> TransferWorkflowStepsElDecryptStepDetailsEl {
        TransferWorkflowStepsElDecryptStepDetailsEl {
            name: core::default::Default::default(),
            overwrite_existing: core::default::Default::default(),
            source_file_location: core::default::Default::default(),
            type_: self.type_,
            destination_file_location: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct TransferWorkflowStepsElDecryptStepDetailsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for TransferWorkflowStepsElDecryptStepDetailsElRef {
    fn new(shared: StackShared, base: String) -> TransferWorkflowStepsElDecryptStepDetailsElRef {
        TransferWorkflowStepsElDecryptStepDetailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TransferWorkflowStepsElDecryptStepDetailsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `overwrite_existing` after provisioning.\n"]
    pub fn overwrite_existing(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.overwrite_existing", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `source_file_location` after provisioning.\n"]
    pub fn source_file_location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_file_location", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `destination_file_location` after provisioning.\n"]
    pub fn destination_file_location(
        &self,
    ) -> ListRef<TransferWorkflowStepsElDecryptStepDetailsElDestinationFileLocationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.destination_file_location", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct TransferWorkflowStepsElDeleteStepDetailsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_file_location: Option<PrimField<String>>,
}

impl TransferWorkflowStepsElDeleteStepDetailsEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc = "Set the field `source_file_location`.\n"]
    pub fn set_source_file_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_file_location = Some(v.into());
        self
    }
}

impl ToListMappable for TransferWorkflowStepsElDeleteStepDetailsEl {
    type O = BlockAssignable<TransferWorkflowStepsElDeleteStepDetailsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTransferWorkflowStepsElDeleteStepDetailsEl {}

impl BuildTransferWorkflowStepsElDeleteStepDetailsEl {
    pub fn build(self) -> TransferWorkflowStepsElDeleteStepDetailsEl {
        TransferWorkflowStepsElDeleteStepDetailsEl {
            name: core::default::Default::default(),
            source_file_location: core::default::Default::default(),
        }
    }
}

pub struct TransferWorkflowStepsElDeleteStepDetailsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for TransferWorkflowStepsElDeleteStepDetailsElRef {
    fn new(shared: StackShared, base: String) -> TransferWorkflowStepsElDeleteStepDetailsElRef {
        TransferWorkflowStepsElDeleteStepDetailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TransferWorkflowStepsElDeleteStepDetailsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `source_file_location` after provisioning.\n"]
    pub fn source_file_location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_file_location", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct TransferWorkflowStepsElTagStepDetailsElTagsEl {
    key: PrimField<String>,
    value: PrimField<String>,
}

impl TransferWorkflowStepsElTagStepDetailsElTagsEl {}

impl ToListMappable for TransferWorkflowStepsElTagStepDetailsElTagsEl {
    type O = BlockAssignable<TransferWorkflowStepsElTagStepDetailsElTagsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTransferWorkflowStepsElTagStepDetailsElTagsEl {
    #[doc = ""]
    pub key: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildTransferWorkflowStepsElTagStepDetailsElTagsEl {
    pub fn build(self) -> TransferWorkflowStepsElTagStepDetailsElTagsEl {
        TransferWorkflowStepsElTagStepDetailsElTagsEl {
            key: self.key,
            value: self.value,
        }
    }
}

pub struct TransferWorkflowStepsElTagStepDetailsElTagsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for TransferWorkflowStepsElTagStepDetailsElTagsElRef {
    fn new(shared: StackShared, base: String) -> TransferWorkflowStepsElTagStepDetailsElTagsElRef {
        TransferWorkflowStepsElTagStepDetailsElTagsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TransferWorkflowStepsElTagStepDetailsElTagsElRef {
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
struct TransferWorkflowStepsElTagStepDetailsElDynamic {
    tags: Option<DynamicBlock<TransferWorkflowStepsElTagStepDetailsElTagsEl>>,
}

#[derive(Serialize)]
pub struct TransferWorkflowStepsElTagStepDetailsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_file_location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<TransferWorkflowStepsElTagStepDetailsElTagsEl>>,
    dynamic: TransferWorkflowStepsElTagStepDetailsElDynamic,
}

impl TransferWorkflowStepsElTagStepDetailsEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc = "Set the field `source_file_location`.\n"]
    pub fn set_source_file_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_file_location = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        mut self,
        v: impl Into<BlockAssignable<TransferWorkflowStepsElTagStepDetailsElTagsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.tags = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.tags = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for TransferWorkflowStepsElTagStepDetailsEl {
    type O = BlockAssignable<TransferWorkflowStepsElTagStepDetailsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTransferWorkflowStepsElTagStepDetailsEl {}

impl BuildTransferWorkflowStepsElTagStepDetailsEl {
    pub fn build(self) -> TransferWorkflowStepsElTagStepDetailsEl {
        TransferWorkflowStepsElTagStepDetailsEl {
            name: core::default::Default::default(),
            source_file_location: core::default::Default::default(),
            tags: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct TransferWorkflowStepsElTagStepDetailsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for TransferWorkflowStepsElTagStepDetailsElRef {
    fn new(shared: StackShared, base: String) -> TransferWorkflowStepsElTagStepDetailsElRef {
        TransferWorkflowStepsElTagStepDetailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TransferWorkflowStepsElTagStepDetailsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `source_file_location` after provisioning.\n"]
    pub fn source_file_location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_file_location", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> ListRef<TransferWorkflowStepsElTagStepDetailsElTagsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize, Default)]
struct TransferWorkflowStepsElDynamic {
    copy_step_details: Option<DynamicBlock<TransferWorkflowStepsElCopyStepDetailsEl>>,
    custom_step_details: Option<DynamicBlock<TransferWorkflowStepsElCustomStepDetailsEl>>,
    decrypt_step_details: Option<DynamicBlock<TransferWorkflowStepsElDecryptStepDetailsEl>>,
    delete_step_details: Option<DynamicBlock<TransferWorkflowStepsElDeleteStepDetailsEl>>,
    tag_step_details: Option<DynamicBlock<TransferWorkflowStepsElTagStepDetailsEl>>,
}

#[derive(Serialize)]
pub struct TransferWorkflowStepsEl {
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    copy_step_details: Option<Vec<TransferWorkflowStepsElCopyStepDetailsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_step_details: Option<Vec<TransferWorkflowStepsElCustomStepDetailsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    decrypt_step_details: Option<Vec<TransferWorkflowStepsElDecryptStepDetailsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete_step_details: Option<Vec<TransferWorkflowStepsElDeleteStepDetailsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag_step_details: Option<Vec<TransferWorkflowStepsElTagStepDetailsEl>>,
    dynamic: TransferWorkflowStepsElDynamic,
}

impl TransferWorkflowStepsEl {
    #[doc = "Set the field `copy_step_details`.\n"]
    pub fn set_copy_step_details(
        mut self,
        v: impl Into<BlockAssignable<TransferWorkflowStepsElCopyStepDetailsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.copy_step_details = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.copy_step_details = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `custom_step_details`.\n"]
    pub fn set_custom_step_details(
        mut self,
        v: impl Into<BlockAssignable<TransferWorkflowStepsElCustomStepDetailsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.custom_step_details = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_step_details = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `decrypt_step_details`.\n"]
    pub fn set_decrypt_step_details(
        mut self,
        v: impl Into<BlockAssignable<TransferWorkflowStepsElDecryptStepDetailsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.decrypt_step_details = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.decrypt_step_details = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `delete_step_details`.\n"]
    pub fn set_delete_step_details(
        mut self,
        v: impl Into<BlockAssignable<TransferWorkflowStepsElDeleteStepDetailsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.delete_step_details = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.delete_step_details = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `tag_step_details`.\n"]
    pub fn set_tag_step_details(
        mut self,
        v: impl Into<BlockAssignable<TransferWorkflowStepsElTagStepDetailsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.tag_step_details = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.tag_step_details = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for TransferWorkflowStepsEl {
    type O = BlockAssignable<TransferWorkflowStepsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTransferWorkflowStepsEl {
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl BuildTransferWorkflowStepsEl {
    pub fn build(self) -> TransferWorkflowStepsEl {
        TransferWorkflowStepsEl {
            type_: self.type_,
            copy_step_details: core::default::Default::default(),
            custom_step_details: core::default::Default::default(),
            decrypt_step_details: core::default::Default::default(),
            delete_step_details: core::default::Default::default(),
            tag_step_details: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct TransferWorkflowStepsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for TransferWorkflowStepsElRef {
    fn new(shared: StackShared, base: String) -> TransferWorkflowStepsElRef {
        TransferWorkflowStepsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TransferWorkflowStepsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `copy_step_details` after provisioning.\n"]
    pub fn copy_step_details(&self) -> ListRef<TransferWorkflowStepsElCopyStepDetailsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.copy_step_details", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `custom_step_details` after provisioning.\n"]
    pub fn custom_step_details(&self) -> ListRef<TransferWorkflowStepsElCustomStepDetailsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_step_details", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `decrypt_step_details` after provisioning.\n"]
    pub fn decrypt_step_details(&self) -> ListRef<TransferWorkflowStepsElDecryptStepDetailsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.decrypt_step_details", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `delete_step_details` after provisioning.\n"]
    pub fn delete_step_details(&self) -> ListRef<TransferWorkflowStepsElDeleteStepDetailsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.delete_step_details", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `tag_step_details` after provisioning.\n"]
    pub fn tag_step_details(&self) -> ListRef<TransferWorkflowStepsElTagStepDetailsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tag_step_details", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct TransferWorkflowDynamic {
    on_exception_steps: Option<DynamicBlock<TransferWorkflowOnExceptionStepsEl>>,
    steps: Option<DynamicBlock<TransferWorkflowStepsEl>>,
}
