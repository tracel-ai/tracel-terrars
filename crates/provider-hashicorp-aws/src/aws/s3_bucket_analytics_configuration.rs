use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct S3BucketAnalyticsConfigurationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    bucket: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<Vec<S3BucketAnalyticsConfigurationFilterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_class_analysis: Option<Vec<S3BucketAnalyticsConfigurationStorageClassAnalysisEl>>,
    dynamic: S3BucketAnalyticsConfigurationDynamic,
}

struct S3BucketAnalyticsConfiguration_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<S3BucketAnalyticsConfigurationData>,
}

#[derive(Clone)]
pub struct S3BucketAnalyticsConfiguration(Rc<S3BucketAnalyticsConfiguration_>);

impl S3BucketAnalyticsConfiguration {
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

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(
        self,
        v: impl Into<BlockAssignable<S3BucketAnalyticsConfigurationFilterEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().filter = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.filter = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `storage_class_analysis`.\n"]
    pub fn set_storage_class_analysis(
        self,
        v: impl Into<BlockAssignable<S3BucketAnalyticsConfigurationStorageClassAnalysisEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().storage_class_analysis = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.storage_class_analysis = Some(d);
            }
        }
        self
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bucket", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> ListRef<S3BucketAnalyticsConfigurationFilterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.filter", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `storage_class_analysis` after provisioning.\n"]
    pub fn storage_class_analysis(
        &self,
    ) -> ListRef<S3BucketAnalyticsConfigurationStorageClassAnalysisElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.storage_class_analysis", self.extract_ref()),
        )
    }
}

impl Referable for S3BucketAnalyticsConfiguration {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for S3BucketAnalyticsConfiguration {}

impl ToListMappable for S3BucketAnalyticsConfiguration {
    type O = ListRef<S3BucketAnalyticsConfigurationRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for S3BucketAnalyticsConfiguration_ {
    fn extract_resource_type(&self) -> String {
        "aws_s3_bucket_analytics_configuration".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildS3BucketAnalyticsConfiguration {
    pub tf_id: String,
    #[doc = ""]
    pub bucket: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildS3BucketAnalyticsConfiguration {
    pub fn build(self, stack: &mut Stack) -> S3BucketAnalyticsConfiguration {
        let out = S3BucketAnalyticsConfiguration(Rc::new(S3BucketAnalyticsConfiguration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(S3BucketAnalyticsConfigurationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                bucket: self.bucket,
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                filter: core::default::Default::default(),
                storage_class_analysis: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct S3BucketAnalyticsConfigurationRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3BucketAnalyticsConfigurationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl S3BucketAnalyticsConfigurationRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bucket", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> ListRef<S3BucketAnalyticsConfigurationFilterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.filter", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `storage_class_analysis` after provisioning.\n"]
    pub fn storage_class_analysis(
        &self,
    ) -> ListRef<S3BucketAnalyticsConfigurationStorageClassAnalysisElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.storage_class_analysis", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct S3BucketAnalyticsConfigurationFilterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
}

impl S3BucketAnalyticsConfigurationFilterEl {
    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.tags = Some(v.into());
        self
    }
}

impl ToListMappable for S3BucketAnalyticsConfigurationFilterEl {
    type O = BlockAssignable<S3BucketAnalyticsConfigurationFilterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketAnalyticsConfigurationFilterEl {}

impl BuildS3BucketAnalyticsConfigurationFilterEl {
    pub fn build(self) -> S3BucketAnalyticsConfigurationFilterEl {
        S3BucketAnalyticsConfigurationFilterEl {
            prefix: core::default::Default::default(),
            tags: core::default::Default::default(),
        }
    }
}

pub struct S3BucketAnalyticsConfigurationFilterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3BucketAnalyticsConfigurationFilterElRef {
    fn new(shared: StackShared, base: String) -> S3BucketAnalyticsConfigurationFilterElRef {
        S3BucketAnalyticsConfigurationFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketAnalyticsConfigurationFilterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize)]
pub struct S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportElDestinationElS3BucketDestinationEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_account_id: Option<PrimField<String>>,
    bucket_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    format: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<PrimField<String>>,
}

impl S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportElDestinationElS3BucketDestinationEl {
    #[doc = "Set the field `bucket_account_id`.\n"]
    pub fn set_bucket_account_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket_account_id = Some(v.into());
        self
    }

    #[doc = "Set the field `format`.\n"]
    pub fn set_format(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.format = Some(v.into());
        self
    }

    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix = Some(v.into());
        self
    }
}

impl ToListMappable for S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportElDestinationElS3BucketDestinationEl {
    type O =
        BlockAssignable<
            S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportElDestinationElS3BucketDestinationEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportElDestinationElS3BucketDestinationEl
{
    #[doc = ""]
    pub bucket_arn: PrimField<String>,
}

impl BuildS3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportElDestinationElS3BucketDestinationEl {
    pub fn build(
        self,
    ) -> S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportElDestinationElS3BucketDestinationEl {
        S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportElDestinationElS3BucketDestinationEl {
            bucket_account_id: core::default::Default::default(),
            bucket_arn: self.bucket_arn,
            format: core::default::Default::default(),
            prefix: core::default::Default::default(),
        }
    }
}

pub struct S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportElDestinationElS3BucketDestinationElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportElDestinationElS3BucketDestinationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportElDestinationElS3BucketDestinationElRef {
        S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportElDestinationElS3BucketDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportElDestinationElS3BucketDestinationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_account_id` after provisioning.\n"]
    pub fn bucket_account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_account_id", self.base))
    }

    #[doc = "Get a reference to the value of field `bucket_arn` after provisioning.\n"]
    pub fn bucket_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `format` after provisioning.\n"]
    pub fn format(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.format", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportElDestinationElDynamic {
    s3_bucket_destination: Option<
        DynamicBlock<
            S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportElDestinationElS3BucketDestinationEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportElDestinationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_bucket_destination: Option<
        Vec<S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportElDestinationElS3BucketDestinationEl>,
    >,
    dynamic: S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportElDestinationElDynamic,
}

impl S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportElDestinationEl {
    #[doc = "Set the field `s3_bucket_destination`.\n"]
    pub fn set_s3_bucket_destination(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportElDestinationElS3BucketDestinationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.s3_bucket_destination = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_bucket_destination = Some(d);
            }
        }
        self
    }
}

impl ToListMappable
    for S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportElDestinationEl
{
    type O = BlockAssignable<
        S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportElDestinationEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportElDestinationEl {}

impl BuildS3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportElDestinationEl {
    pub fn build(
        self,
    ) -> S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportElDestinationEl {
        S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportElDestinationEl {
            s3_bucket_destination: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportElDestinationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportElDestinationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportElDestinationElRef {
        S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportElDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportElDestinationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3_bucket_destination` after provisioning.\n"]
    pub fn s3_bucket_destination(
        &self,
    ) -> ListRef<
        S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportElDestinationElS3BucketDestinationElRef,
    >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.s3_bucket_destination", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportElDynamic {
    destination: Option<
        DynamicBlock<S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportElDestinationEl>,
    >,
}

#[derive(Serialize)]
pub struct S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    output_schema_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination:
        Option<Vec<S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportElDestinationEl>>,
    dynamic: S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportElDynamic,
}

impl S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportEl {
    #[doc = "Set the field `output_schema_version`.\n"]
    pub fn set_output_schema_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.output_schema_version = Some(v.into());
        self
    }

    #[doc = "Set the field `destination`.\n"]
    pub fn set_destination(
        mut self,
        v: impl Into<
            BlockAssignable<
                S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportElDestinationEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.destination = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.destination = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportEl {
    type O = BlockAssignable<S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportEl {}

impl BuildS3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportEl {
    pub fn build(self) -> S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportEl {
        S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportEl {
            output_schema_version: core::default::Default::default(),
            destination: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportElRef {
        S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `output_schema_version` after provisioning.\n"]
    pub fn output_schema_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.output_schema_version", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(
        &self,
    ) -> ListRef<S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportElDestinationElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.destination", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3BucketAnalyticsConfigurationStorageClassAnalysisElDynamic {
    data_export:
        Option<DynamicBlock<S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportEl>>,
}

#[derive(Serialize)]
pub struct S3BucketAnalyticsConfigurationStorageClassAnalysisEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    data_export: Option<Vec<S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportEl>>,
    dynamic: S3BucketAnalyticsConfigurationStorageClassAnalysisElDynamic,
}

impl S3BucketAnalyticsConfigurationStorageClassAnalysisEl {
    #[doc = "Set the field `data_export`.\n"]
    pub fn set_data_export(
        mut self,
        v: impl Into<BlockAssignable<S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.data_export = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.data_export = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for S3BucketAnalyticsConfigurationStorageClassAnalysisEl {
    type O = BlockAssignable<S3BucketAnalyticsConfigurationStorageClassAnalysisEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketAnalyticsConfigurationStorageClassAnalysisEl {}

impl BuildS3BucketAnalyticsConfigurationStorageClassAnalysisEl {
    pub fn build(self) -> S3BucketAnalyticsConfigurationStorageClassAnalysisEl {
        S3BucketAnalyticsConfigurationStorageClassAnalysisEl {
            data_export: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct S3BucketAnalyticsConfigurationStorageClassAnalysisElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3BucketAnalyticsConfigurationStorageClassAnalysisElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> S3BucketAnalyticsConfigurationStorageClassAnalysisElRef {
        S3BucketAnalyticsConfigurationStorageClassAnalysisElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketAnalyticsConfigurationStorageClassAnalysisElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data_export` after provisioning.\n"]
    pub fn data_export(
        &self,
    ) -> ListRef<S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportElRef> {
        ListRef::new(self.shared().clone(), format!("{}.data_export", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3BucketAnalyticsConfigurationDynamic {
    filter: Option<DynamicBlock<S3BucketAnalyticsConfigurationFilterEl>>,
    storage_class_analysis:
        Option<DynamicBlock<S3BucketAnalyticsConfigurationStorageClassAnalysisEl>>,
}
