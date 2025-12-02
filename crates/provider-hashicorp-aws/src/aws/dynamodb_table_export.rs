use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DynamodbTableExportData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    export_format: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    export_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    export_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    s3_bucket: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_bucket_owner: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_sse_algorithm: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_sse_kms_key_id: Option<PrimField<String>>,
    table_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    incremental_export_specification:
        Option<Vec<DynamodbTableExportIncrementalExportSpecificationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DynamodbTableExportTimeoutsEl>,
    dynamic: DynamodbTableExportDynamic,
}
struct DynamodbTableExport_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DynamodbTableExportData>,
}
#[derive(Clone)]
pub struct DynamodbTableExport(Rc<DynamodbTableExport_>);
impl DynamodbTableExport {
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
    #[doc = "Set the field `export_format`.\n"]
    pub fn set_export_format(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().export_format = Some(v.into());
        self
    }
    #[doc = "Set the field `export_time`.\n"]
    pub fn set_export_time(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().export_time = Some(v.into());
        self
    }
    #[doc = "Set the field `export_type`.\n"]
    pub fn set_export_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().export_type = Some(v.into());
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
    #[doc = "Set the field `s3_bucket_owner`.\n"]
    pub fn set_s3_bucket_owner(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().s3_bucket_owner = Some(v.into());
        self
    }
    #[doc = "Set the field `s3_prefix`.\n"]
    pub fn set_s3_prefix(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().s3_prefix = Some(v.into());
        self
    }
    #[doc = "Set the field `s3_sse_algorithm`.\n"]
    pub fn set_s3_sse_algorithm(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().s3_sse_algorithm = Some(v.into());
        self
    }
    #[doc = "Set the field `s3_sse_kms_key_id`.\n"]
    pub fn set_s3_sse_kms_key_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().s3_sse_kms_key_id = Some(v.into());
        self
    }
    #[doc = "Set the field `incremental_export_specification`.\n"]
    pub fn set_incremental_export_specification(
        self,
        v: impl Into<BlockAssignable<DynamodbTableExportIncrementalExportSpecificationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().incremental_export_specification = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .incremental_export_specification = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DynamodbTableExportTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `billed_size_in_bytes` after provisioning.\n"]
    pub fn billed_size_in_bytes(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.billed_size_in_bytes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `end_time` after provisioning.\n"]
    pub fn end_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.end_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `export_format` after provisioning.\n"]
    pub fn export_format(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.export_format", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `export_status` after provisioning.\n"]
    pub fn export_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.export_status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `export_time` after provisioning.\n"]
    pub fn export_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.export_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `export_type` after provisioning.\n"]
    pub fn export_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.export_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `item_count` after provisioning.\n"]
    pub fn item_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.item_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `manifest_files_s3_key` after provisioning.\n"]
    pub fn manifest_files_s3_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.manifest_files_s3_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `s3_bucket` after provisioning.\n"]
    pub fn s3_bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.s3_bucket", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `s3_bucket_owner` after provisioning.\n"]
    pub fn s3_bucket_owner(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.s3_bucket_owner", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `s3_prefix` after provisioning.\n"]
    pub fn s3_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.s3_prefix", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `s3_sse_algorithm` after provisioning.\n"]
    pub fn s3_sse_algorithm(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.s3_sse_algorithm", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `s3_sse_kms_key_id` after provisioning.\n"]
    pub fn s3_sse_kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.s3_sse_kms_key_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.start_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `table_arn` after provisioning.\n"]
    pub fn table_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.table_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `incremental_export_specification` after provisioning.\n"]
    pub fn incremental_export_specification(
        &self,
    ) -> ListRef<DynamodbTableExportIncrementalExportSpecificationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.incremental_export_specification", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DynamodbTableExportTimeoutsElRef {
        DynamodbTableExportTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for DynamodbTableExport {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for DynamodbTableExport {}
impl ToListMappable for DynamodbTableExport {
    type O = ListRef<DynamodbTableExportRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for DynamodbTableExport_ {
    fn extract_resource_type(&self) -> String {
        "aws_dynamodb_table_export".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDynamodbTableExport {
    pub tf_id: String,
    #[doc = ""]
    pub s3_bucket: PrimField<String>,
    #[doc = ""]
    pub table_arn: PrimField<String>,
}
impl BuildDynamodbTableExport {
    pub fn build(self, stack: &mut Stack) -> DynamodbTableExport {
        let out = DynamodbTableExport(Rc::new(DynamodbTableExport_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DynamodbTableExportData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                export_format: core::default::Default::default(),
                export_time: core::default::Default::default(),
                export_type: core::default::Default::default(),
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                s3_bucket: self.s3_bucket,
                s3_bucket_owner: core::default::Default::default(),
                s3_prefix: core::default::Default::default(),
                s3_sse_algorithm: core::default::Default::default(),
                s3_sse_kms_key_id: core::default::Default::default(),
                table_arn: self.table_arn,
                incremental_export_specification: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct DynamodbTableExportRef {
    shared: StackShared,
    base: String,
}
impl Ref for DynamodbTableExportRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DynamodbTableExportRef {
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
    #[doc = "Get a reference to the value of field `billed_size_in_bytes` after provisioning.\n"]
    pub fn billed_size_in_bytes(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.billed_size_in_bytes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `end_time` after provisioning.\n"]
    pub fn end_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.end_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `export_format` after provisioning.\n"]
    pub fn export_format(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.export_format", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `export_status` after provisioning.\n"]
    pub fn export_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.export_status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `export_time` after provisioning.\n"]
    pub fn export_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.export_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `export_type` after provisioning.\n"]
    pub fn export_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.export_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `item_count` after provisioning.\n"]
    pub fn item_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.item_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `manifest_files_s3_key` after provisioning.\n"]
    pub fn manifest_files_s3_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.manifest_files_s3_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `s3_bucket` after provisioning.\n"]
    pub fn s3_bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.s3_bucket", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `s3_bucket_owner` after provisioning.\n"]
    pub fn s3_bucket_owner(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.s3_bucket_owner", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `s3_prefix` after provisioning.\n"]
    pub fn s3_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.s3_prefix", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `s3_sse_algorithm` after provisioning.\n"]
    pub fn s3_sse_algorithm(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.s3_sse_algorithm", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `s3_sse_kms_key_id` after provisioning.\n"]
    pub fn s3_sse_kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.s3_sse_kms_key_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.start_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `table_arn` after provisioning.\n"]
    pub fn table_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.table_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `incremental_export_specification` after provisioning.\n"]
    pub fn incremental_export_specification(
        &self,
    ) -> ListRef<DynamodbTableExportIncrementalExportSpecificationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.incremental_export_specification", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DynamodbTableExportTimeoutsElRef {
        DynamodbTableExportTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DynamodbTableExportIncrementalExportSpecificationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    export_from_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    export_to_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    export_view_type: Option<PrimField<String>>,
}
impl DynamodbTableExportIncrementalExportSpecificationEl {
    #[doc = "Set the field `export_from_time`.\n"]
    pub fn set_export_from_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.export_from_time = Some(v.into());
        self
    }
    #[doc = "Set the field `export_to_time`.\n"]
    pub fn set_export_to_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.export_to_time = Some(v.into());
        self
    }
    #[doc = "Set the field `export_view_type`.\n"]
    pub fn set_export_view_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.export_view_type = Some(v.into());
        self
    }
}
impl ToListMappable for DynamodbTableExportIncrementalExportSpecificationEl {
    type O = BlockAssignable<DynamodbTableExportIncrementalExportSpecificationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDynamodbTableExportIncrementalExportSpecificationEl {}
impl BuildDynamodbTableExportIncrementalExportSpecificationEl {
    pub fn build(self) -> DynamodbTableExportIncrementalExportSpecificationEl {
        DynamodbTableExportIncrementalExportSpecificationEl {
            export_from_time: core::default::Default::default(),
            export_to_time: core::default::Default::default(),
            export_view_type: core::default::Default::default(),
        }
    }
}
pub struct DynamodbTableExportIncrementalExportSpecificationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DynamodbTableExportIncrementalExportSpecificationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DynamodbTableExportIncrementalExportSpecificationElRef {
        DynamodbTableExportIncrementalExportSpecificationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DynamodbTableExportIncrementalExportSpecificationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `export_from_time` after provisioning.\n"]
    pub fn export_from_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.export_from_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `export_to_time` after provisioning.\n"]
    pub fn export_to_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.export_to_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `export_view_type` after provisioning.\n"]
    pub fn export_view_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.export_view_type", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DynamodbTableExportTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}
impl DynamodbTableExportTimeoutsEl {
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
}
impl ToListMappable for DynamodbTableExportTimeoutsEl {
    type O = BlockAssignable<DynamodbTableExportTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDynamodbTableExportTimeoutsEl {}
impl BuildDynamodbTableExportTimeoutsEl {
    pub fn build(self) -> DynamodbTableExportTimeoutsEl {
        DynamodbTableExportTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}
pub struct DynamodbTableExportTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DynamodbTableExportTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DynamodbTableExportTimeoutsElRef {
        DynamodbTableExportTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DynamodbTableExportTimeoutsElRef {
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
}
#[derive(Serialize, Default)]
struct DynamodbTableExportDynamic {
    incremental_export_specification:
        Option<DynamicBlock<DynamodbTableExportIncrementalExportSpecificationEl>>,
}
