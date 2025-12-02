use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct BcmdataexportsExportData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    export: Option<Vec<BcmdataexportsExportExportEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<BcmdataexportsExportTimeoutsEl>,
    dynamic: BcmdataexportsExportDynamic,
}
struct BcmdataexportsExport_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<BcmdataexportsExportData>,
}
#[derive(Clone)]
pub struct BcmdataexportsExport(Rc<BcmdataexportsExport_>);
impl BcmdataexportsExport {
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
    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }
    #[doc = "Set the field `export`.\n"]
    pub fn set_export(self, v: impl Into<BlockAssignable<BcmdataexportsExportExportEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().export = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.export = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<BcmdataexportsExportTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
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
    #[doc = "Get a reference to the value of field `export` after provisioning.\n"]
    pub fn export(&self) -> ListRef<BcmdataexportsExportExportElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.export", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BcmdataexportsExportTimeoutsElRef {
        BcmdataexportsExportTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for BcmdataexportsExport {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for BcmdataexportsExport {}
impl ToListMappable for BcmdataexportsExport {
    type O = ListRef<BcmdataexportsExportRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for BcmdataexportsExport_ {
    fn extract_resource_type(&self) -> String {
        "aws_bcmdataexports_export".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildBcmdataexportsExport {
    pub tf_id: String,
}
impl BuildBcmdataexportsExport {
    pub fn build(self, stack: &mut Stack) -> BcmdataexportsExport {
        let out = BcmdataexportsExport(Rc::new(BcmdataexportsExport_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(BcmdataexportsExportData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                tags: core::default::Default::default(),
                export: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct BcmdataexportsExportRef {
    shared: StackShared,
    base: String,
}
impl Ref for BcmdataexportsExportRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl BcmdataexportsExportRef {
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
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
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
    #[doc = "Get a reference to the value of field `export` after provisioning.\n"]
    pub fn export(&self) -> ListRef<BcmdataexportsExportExportElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.export", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BcmdataexportsExportTimeoutsElRef {
        BcmdataexportsExportTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct BcmdataexportsExportExportElDataQueryEl {
    query_statement: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    table_configurations: Option<RecField<RecField<PrimField<String>>>>,
}
impl BcmdataexportsExportExportElDataQueryEl {
    #[doc = "Set the field `table_configurations`.\n"]
    pub fn set_table_configurations(
        mut self,
        v: impl Into<RecField<RecField<PrimField<String>>>>,
    ) -> Self {
        self.table_configurations = Some(v.into());
        self
    }
}
impl ToListMappable for BcmdataexportsExportExportElDataQueryEl {
    type O = BlockAssignable<BcmdataexportsExportExportElDataQueryEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBcmdataexportsExportExportElDataQueryEl {
    #[doc = ""]
    pub query_statement: PrimField<String>,
}
impl BuildBcmdataexportsExportExportElDataQueryEl {
    pub fn build(self) -> BcmdataexportsExportExportElDataQueryEl {
        BcmdataexportsExportExportElDataQueryEl {
            query_statement: self.query_statement,
            table_configurations: core::default::Default::default(),
        }
    }
}
pub struct BcmdataexportsExportExportElDataQueryElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BcmdataexportsExportExportElDataQueryElRef {
    fn new(shared: StackShared, base: String) -> BcmdataexportsExportExportElDataQueryElRef {
        BcmdataexportsExportExportElDataQueryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BcmdataexportsExportExportElDataQueryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `query_statement` after provisioning.\n"]
    pub fn query_statement(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.query_statement", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `table_configurations` after provisioning.\n"]
    pub fn table_configurations(&self) -> RecRef<RecRef<PrimExpr<String>>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.table_configurations", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct BcmdataexportsExportExportElDestinationConfigurationsElS3DestinationElS3OutputConfigurationsEl
{
    compression: PrimField<String>,
    format: PrimField<String>,
    output_type: PrimField<String>,
    overwrite: PrimField<String>,
}
impl
    BcmdataexportsExportExportElDestinationConfigurationsElS3DestinationElS3OutputConfigurationsEl
{
}
impl ToListMappable for BcmdataexportsExportExportElDestinationConfigurationsElS3DestinationElS3OutputConfigurationsEl { type O = BlockAssignable < BcmdataexportsExportExportElDestinationConfigurationsElS3DestinationElS3OutputConfigurationsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildBcmdataexportsExportExportElDestinationConfigurationsElS3DestinationElS3OutputConfigurationsEl
{
    #[doc = ""]
    pub compression: PrimField<String>,
    #[doc = ""]
    pub format: PrimField<String>,
    #[doc = ""]
    pub output_type: PrimField<String>,
    #[doc = ""]
    pub overwrite: PrimField<String>,
}
impl BuildBcmdataexportsExportExportElDestinationConfigurationsElS3DestinationElS3OutputConfigurationsEl { pub fn build (self) -> BcmdataexportsExportExportElDestinationConfigurationsElS3DestinationElS3OutputConfigurationsEl { BcmdataexportsExportExportElDestinationConfigurationsElS3DestinationElS3OutputConfigurationsEl { compression : self . compression , format : self . format , output_type : self . output_type , overwrite : self . overwrite , } } }
pub struct BcmdataexportsExportExportElDestinationConfigurationsElS3DestinationElS3OutputConfigurationsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for BcmdataexportsExportExportElDestinationConfigurationsElS3DestinationElS3OutputConfigurationsElRef { fn new (shared : StackShared , base : String) -> BcmdataexportsExportExportElDestinationConfigurationsElS3DestinationElS3OutputConfigurationsElRef { BcmdataexportsExportExportElDestinationConfigurationsElS3DestinationElS3OutputConfigurationsElRef { shared : shared , base : base . to_string () , } } }
impl BcmdataexportsExportExportElDestinationConfigurationsElS3DestinationElS3OutputConfigurationsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `compression` after provisioning.\n"] pub fn compression (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.compression" , self . base)) } # [doc = "Get a reference to the value of field `format` after provisioning.\n"] pub fn format (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.format" , self . base)) } # [doc = "Get a reference to the value of field `output_type` after provisioning.\n"] pub fn output_type (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.output_type" , self . base)) } # [doc = "Get a reference to the value of field `overwrite` after provisioning.\n"] pub fn overwrite (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.overwrite" , self . base)) } }
#[derive(Serialize, Default)]
struct BcmdataexportsExportExportElDestinationConfigurationsElS3DestinationElDynamic { s3_output_configurations : Option < DynamicBlock < BcmdataexportsExportExportElDestinationConfigurationsElS3DestinationElS3OutputConfigurationsEl >> , }
#[derive(Serialize)]
pub struct BcmdataexportsExportExportElDestinationConfigurationsElS3DestinationEl { s3_bucket : PrimField < String > , s3_prefix : PrimField < String > , s3_region : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] s3_output_configurations : Option < Vec < BcmdataexportsExportExportElDestinationConfigurationsElS3DestinationElS3OutputConfigurationsEl > > , dynamic : BcmdataexportsExportExportElDestinationConfigurationsElS3DestinationElDynamic , }
impl BcmdataexportsExportExportElDestinationConfigurationsElS3DestinationEl {
    #[doc = "Set the field `s3_output_configurations`.\n"]
    pub fn set_s3_output_configurations(
        mut self,
        v : impl Into < BlockAssignable < BcmdataexportsExportExportElDestinationConfigurationsElS3DestinationElS3OutputConfigurationsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.s3_output_configurations = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_output_configurations = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for BcmdataexportsExportExportElDestinationConfigurationsElS3DestinationEl {
    type O =
        BlockAssignable<BcmdataexportsExportExportElDestinationConfigurationsElS3DestinationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBcmdataexportsExportExportElDestinationConfigurationsElS3DestinationEl {
    #[doc = ""]
    pub s3_bucket: PrimField<String>,
    #[doc = ""]
    pub s3_prefix: PrimField<String>,
    #[doc = ""]
    pub s3_region: PrimField<String>,
}
impl BuildBcmdataexportsExportExportElDestinationConfigurationsElS3DestinationEl {
    pub fn build(self) -> BcmdataexportsExportExportElDestinationConfigurationsElS3DestinationEl {
        BcmdataexportsExportExportElDestinationConfigurationsElS3DestinationEl {
            s3_bucket: self.s3_bucket,
            s3_prefix: self.s3_prefix,
            s3_region: self.s3_region,
            s3_output_configurations: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct BcmdataexportsExportExportElDestinationConfigurationsElS3DestinationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BcmdataexportsExportExportElDestinationConfigurationsElS3DestinationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BcmdataexportsExportExportElDestinationConfigurationsElS3DestinationElRef {
        BcmdataexportsExportExportElDestinationConfigurationsElS3DestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BcmdataexportsExportExportElDestinationConfigurationsElS3DestinationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `s3_bucket` after provisioning.\n"]
    pub fn s3_bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_bucket", self.base))
    }
    #[doc = "Get a reference to the value of field `s3_prefix` after provisioning.\n"]
    pub fn s3_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_prefix", self.base))
    }
    #[doc = "Get a reference to the value of field `s3_region` after provisioning.\n"]
    pub fn s3_region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_region", self.base))
    }
    #[doc = "Get a reference to the value of field `s3_output_configurations` after provisioning.\n"]    pub fn s3_output_configurations (& self) -> ListRef < BcmdataexportsExportExportElDestinationConfigurationsElS3DestinationElS3OutputConfigurationsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.s3_output_configurations", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct BcmdataexportsExportExportElDestinationConfigurationsElDynamic {
    s3_destination: Option<
        DynamicBlock<BcmdataexportsExportExportElDestinationConfigurationsElS3DestinationEl>,
    >,
}
#[derive(Serialize)]
pub struct BcmdataexportsExportExportElDestinationConfigurationsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_destination:
        Option<Vec<BcmdataexportsExportExportElDestinationConfigurationsElS3DestinationEl>>,
    dynamic: BcmdataexportsExportExportElDestinationConfigurationsElDynamic,
}
impl BcmdataexportsExportExportElDestinationConfigurationsEl {
    #[doc = "Set the field `s3_destination`.\n"]
    pub fn set_s3_destination(
        mut self,
        v: impl Into<
            BlockAssignable<BcmdataexportsExportExportElDestinationConfigurationsElS3DestinationEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.s3_destination = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_destination = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for BcmdataexportsExportExportElDestinationConfigurationsEl {
    type O = BlockAssignable<BcmdataexportsExportExportElDestinationConfigurationsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBcmdataexportsExportExportElDestinationConfigurationsEl {}
impl BuildBcmdataexportsExportExportElDestinationConfigurationsEl {
    pub fn build(self) -> BcmdataexportsExportExportElDestinationConfigurationsEl {
        BcmdataexportsExportExportElDestinationConfigurationsEl {
            s3_destination: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct BcmdataexportsExportExportElDestinationConfigurationsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BcmdataexportsExportExportElDestinationConfigurationsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BcmdataexportsExportExportElDestinationConfigurationsElRef {
        BcmdataexportsExportExportElDestinationConfigurationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BcmdataexportsExportExportElDestinationConfigurationsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `s3_destination` after provisioning.\n"]
    pub fn s3_destination(
        &self,
    ) -> ListRef<BcmdataexportsExportExportElDestinationConfigurationsElS3DestinationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.s3_destination", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct BcmdataexportsExportExportElRefreshCadenceEl {
    frequency: PrimField<String>,
}
impl BcmdataexportsExportExportElRefreshCadenceEl {}
impl ToListMappable for BcmdataexportsExportExportElRefreshCadenceEl {
    type O = BlockAssignable<BcmdataexportsExportExportElRefreshCadenceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBcmdataexportsExportExportElRefreshCadenceEl {
    #[doc = ""]
    pub frequency: PrimField<String>,
}
impl BuildBcmdataexportsExportExportElRefreshCadenceEl {
    pub fn build(self) -> BcmdataexportsExportExportElRefreshCadenceEl {
        BcmdataexportsExportExportElRefreshCadenceEl {
            frequency: self.frequency,
        }
    }
}
pub struct BcmdataexportsExportExportElRefreshCadenceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BcmdataexportsExportExportElRefreshCadenceElRef {
    fn new(shared: StackShared, base: String) -> BcmdataexportsExportExportElRefreshCadenceElRef {
        BcmdataexportsExportExportElRefreshCadenceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BcmdataexportsExportExportElRefreshCadenceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `frequency` after provisioning.\n"]
    pub fn frequency(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.frequency", self.base))
    }
}
#[derive(Serialize, Default)]
struct BcmdataexportsExportExportElDynamic {
    data_query: Option<DynamicBlock<BcmdataexportsExportExportElDataQueryEl>>,
    destination_configurations:
        Option<DynamicBlock<BcmdataexportsExportExportElDestinationConfigurationsEl>>,
    refresh_cadence: Option<DynamicBlock<BcmdataexportsExportExportElRefreshCadenceEl>>,
}
#[derive(Serialize)]
pub struct BcmdataexportsExportExportEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_query: Option<Vec<BcmdataexportsExportExportElDataQueryEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_configurations:
        Option<Vec<BcmdataexportsExportExportElDestinationConfigurationsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    refresh_cadence: Option<Vec<BcmdataexportsExportExportElRefreshCadenceEl>>,
    dynamic: BcmdataexportsExportExportElDynamic,
}
impl BcmdataexportsExportExportEl {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `data_query`.\n"]
    pub fn set_data_query(
        mut self,
        v: impl Into<BlockAssignable<BcmdataexportsExportExportElDataQueryEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.data_query = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.data_query = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `destination_configurations`.\n"]
    pub fn set_destination_configurations(
        mut self,
        v: impl Into<BlockAssignable<BcmdataexportsExportExportElDestinationConfigurationsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.destination_configurations = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.destination_configurations = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `refresh_cadence`.\n"]
    pub fn set_refresh_cadence(
        mut self,
        v: impl Into<BlockAssignable<BcmdataexportsExportExportElRefreshCadenceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.refresh_cadence = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.refresh_cadence = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for BcmdataexportsExportExportEl {
    type O = BlockAssignable<BcmdataexportsExportExportEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBcmdataexportsExportExportEl {
    #[doc = ""]
    pub name: PrimField<String>,
}
impl BuildBcmdataexportsExportExportEl {
    pub fn build(self) -> BcmdataexportsExportExportEl {
        BcmdataexportsExportExportEl {
            description: core::default::Default::default(),
            name: self.name,
            data_query: core::default::Default::default(),
            destination_configurations: core::default::Default::default(),
            refresh_cadence: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct BcmdataexportsExportExportElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BcmdataexportsExportExportElRef {
    fn new(shared: StackShared, base: String) -> BcmdataexportsExportExportElRef {
        BcmdataexportsExportExportElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BcmdataexportsExportExportElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `export_arn` after provisioning.\n"]
    pub fn export_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.export_arn", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `data_query` after provisioning.\n"]
    pub fn data_query(&self) -> ListRef<BcmdataexportsExportExportElDataQueryElRef> {
        ListRef::new(self.shared().clone(), format!("{}.data_query", self.base))
    }
    #[doc = "Get a reference to the value of field `destination_configurations` after provisioning.\n"]
    pub fn destination_configurations(
        &self,
    ) -> ListRef<BcmdataexportsExportExportElDestinationConfigurationsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.destination_configurations", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `refresh_cadence` after provisioning.\n"]
    pub fn refresh_cadence(&self) -> ListRef<BcmdataexportsExportExportElRefreshCadenceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.refresh_cadence", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct BcmdataexportsExportTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl BcmdataexportsExportTimeoutsEl {
    #[doc = "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }
    #[doc = "Set the field `update`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}
impl ToListMappable for BcmdataexportsExportTimeoutsEl {
    type O = BlockAssignable<BcmdataexportsExportTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBcmdataexportsExportTimeoutsEl {}
impl BuildBcmdataexportsExportTimeoutsEl {
    pub fn build(self) -> BcmdataexportsExportTimeoutsEl {
        BcmdataexportsExportTimeoutsEl {
            create: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct BcmdataexportsExportTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BcmdataexportsExportTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> BcmdataexportsExportTimeoutsElRef {
        BcmdataexportsExportTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BcmdataexportsExportTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }
    #[doc = "Get a reference to the value of field `update` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
#[derive(Serialize, Default)]
struct BcmdataexportsExportDynamic {
    export: Option<DynamicBlock<BcmdataexportsExportExportEl>>,
}
