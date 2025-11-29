use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct CodebuildReportGroupData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete_reports: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    export_config: Option<Vec<CodebuildReportGroupExportConfigEl>>,
    dynamic: CodebuildReportGroupDynamic,
}

struct CodebuildReportGroup_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CodebuildReportGroupData>,
}

#[derive(Clone)]
pub struct CodebuildReportGroup(Rc<CodebuildReportGroup_>);

impl CodebuildReportGroup {
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

    #[doc = "Set the field `delete_reports`.\n"]
    pub fn set_delete_reports(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().delete_reports = Some(v.into());
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

    #[doc = "Set the field `export_config`.\n"]
    pub fn set_export_config(self, v: impl Into<BlockAssignable<CodebuildReportGroupExportConfigEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().export_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.export_config = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created` after provisioning.\n"]
    pub fn created(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `delete_reports` after provisioning.\n"]
    pub fn delete_reports(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete_reports", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `export_config` after provisioning.\n"]
    pub fn export_config(&self) -> ListRef<CodebuildReportGroupExportConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.export_config", self.extract_ref()))
    }
}

impl Referable for CodebuildReportGroup {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for CodebuildReportGroup { }

impl ToListMappable for CodebuildReportGroup {
    type O = ListRef<CodebuildReportGroupRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for CodebuildReportGroup_ {
    fn extract_resource_type(&self) -> String {
        "aws_codebuild_report_group".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCodebuildReportGroup {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl BuildCodebuildReportGroup {
    pub fn build(self, stack: &mut Stack) -> CodebuildReportGroup {
        let out = CodebuildReportGroup(Rc::new(CodebuildReportGroup_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(CodebuildReportGroupData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                delete_reports: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                type_: self.type_,
                export_config: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CodebuildReportGroupRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodebuildReportGroupRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl CodebuildReportGroupRef {
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

    #[doc = "Get a reference to the value of field `created` after provisioning.\n"]
    pub fn created(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `delete_reports` after provisioning.\n"]
    pub fn delete_reports(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete_reports", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `export_config` after provisioning.\n"]
    pub fn export_config(&self) -> ListRef<CodebuildReportGroupExportConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.export_config", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct CodebuildReportGroupExportConfigElS3DestinationEl {
    bucket: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_disabled: Option<PrimField<bool>>,
    encryption_key: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    packaging: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
}

impl CodebuildReportGroupExportConfigElS3DestinationEl {
    #[doc = "Set the field `encryption_disabled`.\n"]
    pub fn set_encryption_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.encryption_disabled = Some(v.into());
        self
    }

    #[doc = "Set the field `packaging`.\n"]
    pub fn set_packaging(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.packaging = Some(v.into());
        self
    }

    #[doc = "Set the field `path`.\n"]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }
}

impl ToListMappable for CodebuildReportGroupExportConfigElS3DestinationEl {
    type O = BlockAssignable<CodebuildReportGroupExportConfigElS3DestinationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodebuildReportGroupExportConfigElS3DestinationEl {
    #[doc = ""]
    pub bucket: PrimField<String>,
    #[doc = ""]
    pub encryption_key: PrimField<String>,
}

impl BuildCodebuildReportGroupExportConfigElS3DestinationEl {
    pub fn build(self) -> CodebuildReportGroupExportConfigElS3DestinationEl {
        CodebuildReportGroupExportConfigElS3DestinationEl {
            bucket: self.bucket,
            encryption_disabled: core::default::Default::default(),
            encryption_key: self.encryption_key,
            packaging: core::default::Default::default(),
            path: core::default::Default::default(),
        }
    }
}

pub struct CodebuildReportGroupExportConfigElS3DestinationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodebuildReportGroupExportConfigElS3DestinationElRef {
    fn new(shared: StackShared, base: String) -> CodebuildReportGroupExportConfigElS3DestinationElRef {
        CodebuildReportGroupExportConfigElS3DestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodebuildReportGroupExportConfigElS3DestinationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc = "Get a reference to the value of field `encryption_disabled` after provisioning.\n"]
    pub fn encryption_disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.encryption_disabled", self.base))
    }

    #[doc = "Get a reference to the value of field `encryption_key` after provisioning.\n"]
    pub fn encryption_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.encryption_key", self.base))
    }

    #[doc = "Get a reference to the value of field `packaging` after provisioning.\n"]
    pub fn packaging(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.packaging", self.base))
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
}

#[derive(Serialize, Default)]
struct CodebuildReportGroupExportConfigElDynamic {
    s3_destination: Option<DynamicBlock<CodebuildReportGroupExportConfigElS3DestinationEl>>,
}

#[derive(Serialize)]
pub struct CodebuildReportGroupExportConfigEl {
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_destination: Option<Vec<CodebuildReportGroupExportConfigElS3DestinationEl>>,
    dynamic: CodebuildReportGroupExportConfigElDynamic,
}

impl CodebuildReportGroupExportConfigEl {
    #[doc = "Set the field `s3_destination`.\n"]
    pub fn set_s3_destination(
        mut self,
        v: impl Into<BlockAssignable<CodebuildReportGroupExportConfigElS3DestinationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.s3_destination = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_destination = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for CodebuildReportGroupExportConfigEl {
    type O = BlockAssignable<CodebuildReportGroupExportConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodebuildReportGroupExportConfigEl {
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl BuildCodebuildReportGroupExportConfigEl {
    pub fn build(self) -> CodebuildReportGroupExportConfigEl {
        CodebuildReportGroupExportConfigEl {
            type_: self.type_,
            s3_destination: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct CodebuildReportGroupExportConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodebuildReportGroupExportConfigElRef {
    fn new(shared: StackShared, base: String) -> CodebuildReportGroupExportConfigElRef {
        CodebuildReportGroupExportConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodebuildReportGroupExportConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_destination` after provisioning.\n"]
    pub fn s3_destination(&self) -> ListRef<CodebuildReportGroupExportConfigElS3DestinationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.s3_destination", self.base))
    }
}

#[derive(Serialize, Default)]
struct CodebuildReportGroupDynamic {
    export_config: Option<DynamicBlock<CodebuildReportGroupExportConfigEl>>,
}
