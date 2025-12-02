use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct AppfabricIngestionDestinationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    app_bundle_arn: PrimField<String>,
    ingestion_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_configuration: Option<Vec<AppfabricIngestionDestinationDestinationConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    processing_configuration: Option<Vec<AppfabricIngestionDestinationProcessingConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<AppfabricIngestionDestinationTimeoutsEl>,
    dynamic: AppfabricIngestionDestinationDynamic,
}
struct AppfabricIngestionDestination_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<AppfabricIngestionDestinationData>,
}
#[derive(Clone)]
pub struct AppfabricIngestionDestination(Rc<AppfabricIngestionDestination_>);
impl AppfabricIngestionDestination {
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
    #[doc = "Set the field `destination_configuration`.\n"]
    pub fn set_destination_configuration(
        self,
        v: impl Into<BlockAssignable<AppfabricIngestionDestinationDestinationConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().destination_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.destination_configuration = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `processing_configuration`.\n"]
    pub fn set_processing_configuration(
        self,
        v: impl Into<BlockAssignable<AppfabricIngestionDestinationProcessingConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().processing_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.processing_configuration = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<AppfabricIngestionDestinationTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `app_bundle_arn` after provisioning.\n"]
    pub fn app_bundle_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.app_bundle_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `ingestion_arn` after provisioning.\n"]
    pub fn ingestion_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ingestion_arn", self.extract_ref()),
        )
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
    #[doc = "Get a reference to the value of field `destination_configuration` after provisioning.\n"]
    pub fn destination_configuration(
        &self,
    ) -> ListRef<AppfabricIngestionDestinationDestinationConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.destination_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `processing_configuration` after provisioning.\n"]
    pub fn processing_configuration(
        &self,
    ) -> ListRef<AppfabricIngestionDestinationProcessingConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.processing_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AppfabricIngestionDestinationTimeoutsElRef {
        AppfabricIngestionDestinationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for AppfabricIngestionDestination {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for AppfabricIngestionDestination {}
impl ToListMappable for AppfabricIngestionDestination {
    type O = ListRef<AppfabricIngestionDestinationRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for AppfabricIngestionDestination_ {
    fn extract_resource_type(&self) -> String {
        "aws_appfabric_ingestion_destination".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildAppfabricIngestionDestination {
    pub tf_id: String,
    #[doc = ""]
    pub app_bundle_arn: PrimField<String>,
    #[doc = ""]
    pub ingestion_arn: PrimField<String>,
}
impl BuildAppfabricIngestionDestination {
    pub fn build(self, stack: &mut Stack) -> AppfabricIngestionDestination {
        let out = AppfabricIngestionDestination(Rc::new(AppfabricIngestionDestination_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(AppfabricIngestionDestinationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                app_bundle_arn: self.app_bundle_arn,
                ingestion_arn: self.ingestion_arn,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                destination_configuration: core::default::Default::default(),
                processing_configuration: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct AppfabricIngestionDestinationRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppfabricIngestionDestinationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl AppfabricIngestionDestinationRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `app_bundle_arn` after provisioning.\n"]
    pub fn app_bundle_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.app_bundle_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `ingestion_arn` after provisioning.\n"]
    pub fn ingestion_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ingestion_arn", self.extract_ref()),
        )
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
    #[doc = "Get a reference to the value of field `destination_configuration` after provisioning.\n"]
    pub fn destination_configuration(
        &self,
    ) -> ListRef<AppfabricIngestionDestinationDestinationConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.destination_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `processing_configuration` after provisioning.\n"]
    pub fn processing_configuration(
        &self,
    ) -> ListRef<AppfabricIngestionDestinationProcessingConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.processing_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AppfabricIngestionDestinationTimeoutsElRef {
        AppfabricIngestionDestinationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct AppfabricIngestionDestinationDestinationConfigurationElAuditLogElDestinationElFirehoseStreamEl
{
    stream_name: PrimField<String>,
}
impl
    AppfabricIngestionDestinationDestinationConfigurationElAuditLogElDestinationElFirehoseStreamEl
{
}
impl ToListMappable for AppfabricIngestionDestinationDestinationConfigurationElAuditLogElDestinationElFirehoseStreamEl { type O = BlockAssignable < AppfabricIngestionDestinationDestinationConfigurationElAuditLogElDestinationElFirehoseStreamEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildAppfabricIngestionDestinationDestinationConfigurationElAuditLogElDestinationElFirehoseStreamEl
{
    #[doc = ""]
    pub stream_name: PrimField<String>,
}
impl BuildAppfabricIngestionDestinationDestinationConfigurationElAuditLogElDestinationElFirehoseStreamEl { pub fn build (self) -> AppfabricIngestionDestinationDestinationConfigurationElAuditLogElDestinationElFirehoseStreamEl { AppfabricIngestionDestinationDestinationConfigurationElAuditLogElDestinationElFirehoseStreamEl { stream_name : self . stream_name , } } }
pub struct AppfabricIngestionDestinationDestinationConfigurationElAuditLogElDestinationElFirehoseStreamElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for AppfabricIngestionDestinationDestinationConfigurationElAuditLogElDestinationElFirehoseStreamElRef { fn new (shared : StackShared , base : String) -> AppfabricIngestionDestinationDestinationConfigurationElAuditLogElDestinationElFirehoseStreamElRef { AppfabricIngestionDestinationDestinationConfigurationElAuditLogElDestinationElFirehoseStreamElRef { shared : shared , base : base . to_string () , } } }
impl AppfabricIngestionDestinationDestinationConfigurationElAuditLogElDestinationElFirehoseStreamElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `stream_name` after provisioning.\n"] pub fn stream_name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.stream_name" , self . base)) } }
#[derive(Serialize)]
pub struct AppfabricIngestionDestinationDestinationConfigurationElAuditLogElDestinationElS3BucketEl
{
    bucket_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<PrimField<String>>,
}
impl AppfabricIngestionDestinationDestinationConfigurationElAuditLogElDestinationElS3BucketEl {
    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix = Some(v.into());
        self
    }
}
impl ToListMappable
    for AppfabricIngestionDestinationDestinationConfigurationElAuditLogElDestinationElS3BucketEl
{
    type O = BlockAssignable<
        AppfabricIngestionDestinationDestinationConfigurationElAuditLogElDestinationElS3BucketEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppfabricIngestionDestinationDestinationConfigurationElAuditLogElDestinationElS3BucketEl
{
    #[doc = ""]
    pub bucket_name: PrimField<String>,
}
impl BuildAppfabricIngestionDestinationDestinationConfigurationElAuditLogElDestinationElS3BucketEl {
    pub fn build(
        self,
    ) -> AppfabricIngestionDestinationDestinationConfigurationElAuditLogElDestinationElS3BucketEl
    {
        AppfabricIngestionDestinationDestinationConfigurationElAuditLogElDestinationElS3BucketEl {
            bucket_name: self.bucket_name,
            prefix: core::default::Default::default(),
        }
    }
}
pub struct AppfabricIngestionDestinationDestinationConfigurationElAuditLogElDestinationElS3BucketElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for AppfabricIngestionDestinationDestinationConfigurationElAuditLogElDestinationElS3BucketElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppfabricIngestionDestinationDestinationConfigurationElAuditLogElDestinationElS3BucketElRef
    {
        AppfabricIngestionDestinationDestinationConfigurationElAuditLogElDestinationElS3BucketElRef { shared : shared , base : base . to_string () , }
    }
}
impl AppfabricIngestionDestinationDestinationConfigurationElAuditLogElDestinationElS3BucketElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }
    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }
}
#[derive(Serialize, Default)]
struct AppfabricIngestionDestinationDestinationConfigurationElAuditLogElDestinationElDynamic { firehose_stream : Option < DynamicBlock < AppfabricIngestionDestinationDestinationConfigurationElAuditLogElDestinationElFirehoseStreamEl >> , s3_bucket : Option < DynamicBlock < AppfabricIngestionDestinationDestinationConfigurationElAuditLogElDestinationElS3BucketEl >> , }
#[derive(Serialize)]
pub struct AppfabricIngestionDestinationDestinationConfigurationElAuditLogElDestinationEl { # [serde (skip_serializing_if = "Option::is_none")] firehose_stream : Option < Vec < AppfabricIngestionDestinationDestinationConfigurationElAuditLogElDestinationElFirehoseStreamEl > > , # [serde (skip_serializing_if = "Option::is_none")] s3_bucket : Option < Vec < AppfabricIngestionDestinationDestinationConfigurationElAuditLogElDestinationElS3BucketEl > > , dynamic : AppfabricIngestionDestinationDestinationConfigurationElAuditLogElDestinationElDynamic , }
impl AppfabricIngestionDestinationDestinationConfigurationElAuditLogElDestinationEl {
    #[doc = "Set the field `firehose_stream`.\n"]
    pub fn set_firehose_stream(
        mut self,
        v : impl Into < BlockAssignable < AppfabricIngestionDestinationDestinationConfigurationElAuditLogElDestinationElFirehoseStreamEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.firehose_stream = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.firehose_stream = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `s3_bucket`.\n"]
    pub fn set_s3_bucket(
        mut self,
        v : impl Into < BlockAssignable < AppfabricIngestionDestinationDestinationConfigurationElAuditLogElDestinationElS3BucketEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.s3_bucket = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_bucket = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for AppfabricIngestionDestinationDestinationConfigurationElAuditLogElDestinationEl
{
    type O = BlockAssignable<
        AppfabricIngestionDestinationDestinationConfigurationElAuditLogElDestinationEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppfabricIngestionDestinationDestinationConfigurationElAuditLogElDestinationEl {}
impl BuildAppfabricIngestionDestinationDestinationConfigurationElAuditLogElDestinationEl {
    pub fn build(
        self,
    ) -> AppfabricIngestionDestinationDestinationConfigurationElAuditLogElDestinationEl {
        AppfabricIngestionDestinationDestinationConfigurationElAuditLogElDestinationEl {
            firehose_stream: core::default::Default::default(),
            s3_bucket: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AppfabricIngestionDestinationDestinationConfigurationElAuditLogElDestinationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppfabricIngestionDestinationDestinationConfigurationElAuditLogElDestinationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppfabricIngestionDestinationDestinationConfigurationElAuditLogElDestinationElRef {
        AppfabricIngestionDestinationDestinationConfigurationElAuditLogElDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppfabricIngestionDestinationDestinationConfigurationElAuditLogElDestinationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `firehose_stream` after provisioning.\n"]    pub fn firehose_stream (& self) -> ListRef < AppfabricIngestionDestinationDestinationConfigurationElAuditLogElDestinationElFirehoseStreamElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.firehose_stream", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `s3_bucket` after provisioning.\n"]
    pub fn s3_bucket(
        &self,
    ) -> ListRef<
        AppfabricIngestionDestinationDestinationConfigurationElAuditLogElDestinationElS3BucketElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.s3_bucket", self.base))
    }
}
#[derive(Serialize, Default)]
struct AppfabricIngestionDestinationDestinationConfigurationElAuditLogElDynamic {
    destination: Option<
        DynamicBlock<
            AppfabricIngestionDestinationDestinationConfigurationElAuditLogElDestinationEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct AppfabricIngestionDestinationDestinationConfigurationElAuditLogEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    destination:
        Option<Vec<AppfabricIngestionDestinationDestinationConfigurationElAuditLogElDestinationEl>>,
    dynamic: AppfabricIngestionDestinationDestinationConfigurationElAuditLogElDynamic,
}
impl AppfabricIngestionDestinationDestinationConfigurationElAuditLogEl {
    #[doc = "Set the field `destination`.\n"]
    pub fn set_destination(
        mut self,
        v: impl Into<
            BlockAssignable<
                AppfabricIngestionDestinationDestinationConfigurationElAuditLogElDestinationEl,
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
impl ToListMappable for AppfabricIngestionDestinationDestinationConfigurationElAuditLogEl {
    type O = BlockAssignable<AppfabricIngestionDestinationDestinationConfigurationElAuditLogEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppfabricIngestionDestinationDestinationConfigurationElAuditLogEl {}
impl BuildAppfabricIngestionDestinationDestinationConfigurationElAuditLogEl {
    pub fn build(self) -> AppfabricIngestionDestinationDestinationConfigurationElAuditLogEl {
        AppfabricIngestionDestinationDestinationConfigurationElAuditLogEl {
            destination: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AppfabricIngestionDestinationDestinationConfigurationElAuditLogElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppfabricIngestionDestinationDestinationConfigurationElAuditLogElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppfabricIngestionDestinationDestinationConfigurationElAuditLogElRef {
        AppfabricIngestionDestinationDestinationConfigurationElAuditLogElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppfabricIngestionDestinationDestinationConfigurationElAuditLogElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(
        &self,
    ) -> ListRef<AppfabricIngestionDestinationDestinationConfigurationElAuditLogElDestinationElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.destination", self.base))
    }
}
#[derive(Serialize, Default)]
struct AppfabricIngestionDestinationDestinationConfigurationElDynamic {
    audit_log:
        Option<DynamicBlock<AppfabricIngestionDestinationDestinationConfigurationElAuditLogEl>>,
}
#[derive(Serialize)]
pub struct AppfabricIngestionDestinationDestinationConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    audit_log: Option<Vec<AppfabricIngestionDestinationDestinationConfigurationElAuditLogEl>>,
    dynamic: AppfabricIngestionDestinationDestinationConfigurationElDynamic,
}
impl AppfabricIngestionDestinationDestinationConfigurationEl {
    #[doc = "Set the field `audit_log`.\n"]
    pub fn set_audit_log(
        mut self,
        v: impl Into<BlockAssignable<AppfabricIngestionDestinationDestinationConfigurationElAuditLogEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.audit_log = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.audit_log = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for AppfabricIngestionDestinationDestinationConfigurationEl {
    type O = BlockAssignable<AppfabricIngestionDestinationDestinationConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppfabricIngestionDestinationDestinationConfigurationEl {}
impl BuildAppfabricIngestionDestinationDestinationConfigurationEl {
    pub fn build(self) -> AppfabricIngestionDestinationDestinationConfigurationEl {
        AppfabricIngestionDestinationDestinationConfigurationEl {
            audit_log: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AppfabricIngestionDestinationDestinationConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppfabricIngestionDestinationDestinationConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppfabricIngestionDestinationDestinationConfigurationElRef {
        AppfabricIngestionDestinationDestinationConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppfabricIngestionDestinationDestinationConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `audit_log` after provisioning.\n"]
    pub fn audit_log(
        &self,
    ) -> ListRef<AppfabricIngestionDestinationDestinationConfigurationElAuditLogElRef> {
        ListRef::new(self.shared().clone(), format!("{}.audit_log", self.base))
    }
}
#[derive(Serialize)]
pub struct AppfabricIngestionDestinationProcessingConfigurationElAuditLogEl {
    format: PrimField<String>,
    schema: PrimField<String>,
}
impl AppfabricIngestionDestinationProcessingConfigurationElAuditLogEl {}
impl ToListMappable for AppfabricIngestionDestinationProcessingConfigurationElAuditLogEl {
    type O = BlockAssignable<AppfabricIngestionDestinationProcessingConfigurationElAuditLogEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppfabricIngestionDestinationProcessingConfigurationElAuditLogEl {
    #[doc = ""]
    pub format: PrimField<String>,
    #[doc = ""]
    pub schema: PrimField<String>,
}
impl BuildAppfabricIngestionDestinationProcessingConfigurationElAuditLogEl {
    pub fn build(self) -> AppfabricIngestionDestinationProcessingConfigurationElAuditLogEl {
        AppfabricIngestionDestinationProcessingConfigurationElAuditLogEl {
            format: self.format,
            schema: self.schema,
        }
    }
}
pub struct AppfabricIngestionDestinationProcessingConfigurationElAuditLogElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppfabricIngestionDestinationProcessingConfigurationElAuditLogElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppfabricIngestionDestinationProcessingConfigurationElAuditLogElRef {
        AppfabricIngestionDestinationProcessingConfigurationElAuditLogElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppfabricIngestionDestinationProcessingConfigurationElAuditLogElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `format` after provisioning.\n"]
    pub fn format(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.format", self.base))
    }
    #[doc = "Get a reference to the value of field `schema` after provisioning.\n"]
    pub fn schema(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.schema", self.base))
    }
}
#[derive(Serialize, Default)]
struct AppfabricIngestionDestinationProcessingConfigurationElDynamic {
    audit_log:
        Option<DynamicBlock<AppfabricIngestionDestinationProcessingConfigurationElAuditLogEl>>,
}
#[derive(Serialize)]
pub struct AppfabricIngestionDestinationProcessingConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    audit_log: Option<Vec<AppfabricIngestionDestinationProcessingConfigurationElAuditLogEl>>,
    dynamic: AppfabricIngestionDestinationProcessingConfigurationElDynamic,
}
impl AppfabricIngestionDestinationProcessingConfigurationEl {
    #[doc = "Set the field `audit_log`.\n"]
    pub fn set_audit_log(
        mut self,
        v: impl Into<BlockAssignable<AppfabricIngestionDestinationProcessingConfigurationElAuditLogEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.audit_log = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.audit_log = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for AppfabricIngestionDestinationProcessingConfigurationEl {
    type O = BlockAssignable<AppfabricIngestionDestinationProcessingConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppfabricIngestionDestinationProcessingConfigurationEl {}
impl BuildAppfabricIngestionDestinationProcessingConfigurationEl {
    pub fn build(self) -> AppfabricIngestionDestinationProcessingConfigurationEl {
        AppfabricIngestionDestinationProcessingConfigurationEl {
            audit_log: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AppfabricIngestionDestinationProcessingConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppfabricIngestionDestinationProcessingConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppfabricIngestionDestinationProcessingConfigurationElRef {
        AppfabricIngestionDestinationProcessingConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppfabricIngestionDestinationProcessingConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `audit_log` after provisioning.\n"]
    pub fn audit_log(
        &self,
    ) -> ListRef<AppfabricIngestionDestinationProcessingConfigurationElAuditLogElRef> {
        ListRef::new(self.shared().clone(), format!("{}.audit_log", self.base))
    }
}
#[derive(Serialize)]
pub struct AppfabricIngestionDestinationTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl AppfabricIngestionDestinationTimeoutsEl {
    #[doc = "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }
    #[doc = "Set the field `delete`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }
    #[doc = "Set the field `update`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}
impl ToListMappable for AppfabricIngestionDestinationTimeoutsEl {
    type O = BlockAssignable<AppfabricIngestionDestinationTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppfabricIngestionDestinationTimeoutsEl {}
impl BuildAppfabricIngestionDestinationTimeoutsEl {
    pub fn build(self) -> AppfabricIngestionDestinationTimeoutsEl {
        AppfabricIngestionDestinationTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct AppfabricIngestionDestinationTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppfabricIngestionDestinationTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> AppfabricIngestionDestinationTimeoutsElRef {
        AppfabricIngestionDestinationTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppfabricIngestionDestinationTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }
    #[doc = "Get a reference to the value of field `delete` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }
    #[doc = "Get a reference to the value of field `update` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
#[derive(Serialize, Default)]
struct AppfabricIngestionDestinationDynamic {
    destination_configuration:
        Option<DynamicBlock<AppfabricIngestionDestinationDestinationConfigurationEl>>,
    processing_configuration:
        Option<DynamicBlock<AppfabricIngestionDestinationProcessingConfigurationEl>>,
}
