use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataCloudwatchLogDataProtectionPolicyDocumentData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    configuration: Option<Vec<DataCloudwatchLogDataProtectionPolicyDocumentConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    statement: Option<Vec<DataCloudwatchLogDataProtectionPolicyDocumentStatementEl>>,
    dynamic: DataCloudwatchLogDataProtectionPolicyDocumentDynamic,
}
struct DataCloudwatchLogDataProtectionPolicyDocument_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataCloudwatchLogDataProtectionPolicyDocumentData>,
}
#[derive(Clone)]
pub struct DataCloudwatchLogDataProtectionPolicyDocument(
    Rc<DataCloudwatchLogDataProtectionPolicyDocument_>,
);
impl DataCloudwatchLogDataProtectionPolicyDocument {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(&self, provider: &ProviderAws) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
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
    #[doc = "Set the field `version`.\n"]
    pub fn set_version(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().version = Some(v.into());
        self
    }
    #[doc = "Set the field `configuration`.\n"]
    pub fn set_configuration(
        self,
        v: impl Into<BlockAssignable<DataCloudwatchLogDataProtectionPolicyDocumentConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.configuration = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `statement`.\n"]
    pub fn set_statement(
        self,
        v: impl Into<BlockAssignable<DataCloudwatchLogDataProtectionPolicyDocumentStatementEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().statement = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.statement = Some(d);
            }
        }
        self
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
    #[doc = "Get a reference to the value of field `json` after provisioning.\n"]
    pub fn json(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.json", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(
        &self,
    ) -> ListRef<DataCloudwatchLogDataProtectionPolicyDocumentConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `statement` after provisioning.\n"]
    pub fn statement(
        &self,
    ) -> ListRef<DataCloudwatchLogDataProtectionPolicyDocumentStatementElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.statement", self.extract_ref()),
        )
    }
}
impl Referable for DataCloudwatchLogDataProtectionPolicyDocument {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataCloudwatchLogDataProtectionPolicyDocument {}
impl ToListMappable for DataCloudwatchLogDataProtectionPolicyDocument {
    type O = ListRef<DataCloudwatchLogDataProtectionPolicyDocumentRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataCloudwatchLogDataProtectionPolicyDocument_ {
    fn extract_datasource_type(&self) -> String {
        "aws_cloudwatch_log_data_protection_policy_document".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataCloudwatchLogDataProtectionPolicyDocument {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
}
impl BuildDataCloudwatchLogDataProtectionPolicyDocument {
    pub fn build(self, stack: &mut Stack) -> DataCloudwatchLogDataProtectionPolicyDocument {
        let out = DataCloudwatchLogDataProtectionPolicyDocument(Rc::new(
            DataCloudwatchLogDataProtectionPolicyDocument_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(DataCloudwatchLogDataProtectionPolicyDocumentData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    for_each: None,
                    description: core::default::Default::default(),
                    id: core::default::Default::default(),
                    name: self.name,
                    version: core::default::Default::default(),
                    configuration: core::default::Default::default(),
                    statement: core::default::Default::default(),
                    dynamic: Default::default(),
                }),
            },
        ));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataCloudwatchLogDataProtectionPolicyDocumentRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudwatchLogDataProtectionPolicyDocumentRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataCloudwatchLogDataProtectionPolicyDocumentRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
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
    #[doc = "Get a reference to the value of field `json` after provisioning.\n"]
    pub fn json(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.json", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(
        &self,
    ) -> ListRef<DataCloudwatchLogDataProtectionPolicyDocumentConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `statement` after provisioning.\n"]
    pub fn statement(
        &self,
    ) -> ListRef<DataCloudwatchLogDataProtectionPolicyDocumentStatementElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.statement", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataCloudwatchLogDataProtectionPolicyDocumentConfigurationElCustomDataIdentifierEl {
    name: PrimField<String>,
    regex: PrimField<String>,
}
impl DataCloudwatchLogDataProtectionPolicyDocumentConfigurationElCustomDataIdentifierEl {}
impl ToListMappable
    for DataCloudwatchLogDataProtectionPolicyDocumentConfigurationElCustomDataIdentifierEl
{
    type O = BlockAssignable<
        DataCloudwatchLogDataProtectionPolicyDocumentConfigurationElCustomDataIdentifierEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudwatchLogDataProtectionPolicyDocumentConfigurationElCustomDataIdentifierEl {
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub regex: PrimField<String>,
}
impl BuildDataCloudwatchLogDataProtectionPolicyDocumentConfigurationElCustomDataIdentifierEl {
    pub fn build(
        self,
    ) -> DataCloudwatchLogDataProtectionPolicyDocumentConfigurationElCustomDataIdentifierEl {
        DataCloudwatchLogDataProtectionPolicyDocumentConfigurationElCustomDataIdentifierEl {
            name: self.name,
            regex: self.regex,
        }
    }
}
pub struct DataCloudwatchLogDataProtectionPolicyDocumentConfigurationElCustomDataIdentifierElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudwatchLogDataProtectionPolicyDocumentConfigurationElCustomDataIdentifierElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudwatchLogDataProtectionPolicyDocumentConfigurationElCustomDataIdentifierElRef {
        DataCloudwatchLogDataProtectionPolicyDocumentConfigurationElCustomDataIdentifierElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudwatchLogDataProtectionPolicyDocumentConfigurationElCustomDataIdentifierElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `regex` after provisioning.\n"]
    pub fn regex(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.regex", self.base))
    }
}
#[derive(Serialize, Default)]
struct DataCloudwatchLogDataProtectionPolicyDocumentConfigurationElDynamic {
    custom_data_identifier: Option<
        DynamicBlock<
            DataCloudwatchLogDataProtectionPolicyDocumentConfigurationElCustomDataIdentifierEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct DataCloudwatchLogDataProtectionPolicyDocumentConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_data_identifier: Option<
        Vec<DataCloudwatchLogDataProtectionPolicyDocumentConfigurationElCustomDataIdentifierEl>,
    >,
    dynamic: DataCloudwatchLogDataProtectionPolicyDocumentConfigurationElDynamic,
}
impl DataCloudwatchLogDataProtectionPolicyDocumentConfigurationEl {
    #[doc = "Set the field `custom_data_identifier`.\n"]
    pub fn set_custom_data_identifier(
        mut self,
        v: impl Into<
            BlockAssignable<
                DataCloudwatchLogDataProtectionPolicyDocumentConfigurationElCustomDataIdentifierEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.custom_data_identifier = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_data_identifier = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataCloudwatchLogDataProtectionPolicyDocumentConfigurationEl {
    type O = BlockAssignable<DataCloudwatchLogDataProtectionPolicyDocumentConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudwatchLogDataProtectionPolicyDocumentConfigurationEl {}
impl BuildDataCloudwatchLogDataProtectionPolicyDocumentConfigurationEl {
    pub fn build(self) -> DataCloudwatchLogDataProtectionPolicyDocumentConfigurationEl {
        DataCloudwatchLogDataProtectionPolicyDocumentConfigurationEl {
            custom_data_identifier: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataCloudwatchLogDataProtectionPolicyDocumentConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudwatchLogDataProtectionPolicyDocumentConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudwatchLogDataProtectionPolicyDocumentConfigurationElRef {
        DataCloudwatchLogDataProtectionPolicyDocumentConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudwatchLogDataProtectionPolicyDocumentConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `custom_data_identifier` after provisioning.\n"]
    pub fn custom_data_identifier(
        &self,
    ) -> ListRef<
        DataCloudwatchLogDataProtectionPolicyDocumentConfigurationElCustomDataIdentifierElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_data_identifier", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElCloudwatchLogsEl
{
    log_group: PrimField<String>,
}
impl DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElCloudwatchLogsEl { }
impl ToListMappable for DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElCloudwatchLogsEl { type O = BlockAssignable < DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElCloudwatchLogsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElCloudwatchLogsEl
{
    #[doc = ""]
    pub log_group: PrimField<String>,
}
impl BuildDataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElCloudwatchLogsEl { pub fn build (self) -> DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElCloudwatchLogsEl { DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElCloudwatchLogsEl { log_group : self . log_group , } } }
pub struct DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElCloudwatchLogsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElCloudwatchLogsElRef { fn new (shared : StackShared , base : String) -> DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElCloudwatchLogsElRef { DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElCloudwatchLogsElRef { shared : shared , base : base . to_string () , } } }
impl DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElCloudwatchLogsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `log_group` after provisioning.\n"] pub fn log_group (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.log_group" , self . base)) } }
#[derive(Serialize)]
pub struct DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElFirehoseEl
{
    delivery_stream: PrimField<String>,
}
impl DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElFirehoseEl { }
impl ToListMappable for DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElFirehoseEl { type O = BlockAssignable < DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElFirehoseEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElFirehoseEl
{
    #[doc = ""]
    pub delivery_stream: PrimField<String>,
}
impl BuildDataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElFirehoseEl { pub fn build (self) -> DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElFirehoseEl { DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElFirehoseEl { delivery_stream : self . delivery_stream , } } }
pub struct DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElFirehoseElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElFirehoseElRef { fn new (shared : StackShared , base : String) -> DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElFirehoseElRef { DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElFirehoseElRef { shared : shared , base : base . to_string () , } } }
impl DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElFirehoseElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `delivery_stream` after provisioning.\n"] pub fn delivery_stream (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.delivery_stream" , self . base)) } }
#[derive(Serialize)]
pub struct DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElS3El
{
    bucket: PrimField<String>,
}
impl DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElS3El { }
impl ToListMappable for DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElS3El { type O = BlockAssignable < DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElS3El > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElS3El
{
    #[doc = ""]
    pub bucket: PrimField<String>,
}
impl BuildDataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElS3El { pub fn build (self) -> DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElS3El { DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElS3El { bucket : self . bucket , } } }
pub struct DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElS3ElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElS3ElRef { fn new (shared : StackShared , base : String) -> DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElS3ElRef { DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElS3ElRef { shared : shared , base : base . to_string () , } } }
impl DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElS3ElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `bucket` after provisioning.\n"] pub fn bucket (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.bucket" , self . base)) } }
#[derive(Serialize, Default)]
struct DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElDynamic { cloudwatch_logs : Option < DynamicBlock < DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElCloudwatchLogsEl >> , firehose : Option < DynamicBlock < DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElFirehoseEl >> , s3 : Option < DynamicBlock < DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElS3El >> , }
#[derive(Serialize)]
pub struct DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationEl { # [serde (skip_serializing_if = "Option::is_none")] cloudwatch_logs : Option < Vec < DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElCloudwatchLogsEl > > , # [serde (skip_serializing_if = "Option::is_none")] firehose : Option < Vec < DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElFirehoseEl > > , # [serde (skip_serializing_if = "Option::is_none")] s3 : Option < Vec < DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElS3El > > , dynamic : DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElDynamic , }
impl
    DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationEl
{
    #[doc = "Set the field `cloudwatch_logs`.\n"]
    pub fn set_cloudwatch_logs(
        mut self,
        v : impl Into < BlockAssignable < DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElCloudwatchLogsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cloudwatch_logs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cloudwatch_logs = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `firehose`.\n"]
    pub fn set_firehose(
        mut self,
        v : impl Into < BlockAssignable < DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElFirehoseEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.firehose = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.firehose = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `s3`.\n"]
    pub fn set_s3(
        mut self,
        v : impl Into < BlockAssignable < DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElS3El >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.s3 = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.s3 = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationEl { type O = BlockAssignable < DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationEl
{}
impl BuildDataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationEl { pub fn build (self) -> DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationEl { DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationEl { cloudwatch_logs : core :: default :: Default :: default () , firehose : core :: default :: Default :: default () , s3 : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElRef { fn new (shared : StackShared , base : String) -> DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElRef { DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElRef { shared : shared , base : base . to_string () , } } }
impl DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `cloudwatch_logs` after provisioning.\n"] pub fn cloudwatch_logs (& self) -> ListRef < DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElCloudwatchLogsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.cloudwatch_logs" , self . base)) } # [doc = "Get a reference to the value of field `firehose` after provisioning.\n"] pub fn firehose (& self) -> ListRef < DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElFirehoseElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.firehose" , self . base)) } # [doc = "Get a reference to the value of field `s3` after provisioning.\n"] pub fn s3 (& self) -> ListRef < DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElS3ElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.s3" , self . base)) } }
#[derive(Serialize, Default)]
struct DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElDynamic { findings_destination : Option < DynamicBlock < DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationEl >> , }
#[derive(Serialize)]
pub struct DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditEl { # [serde (skip_serializing_if = "Option::is_none")] findings_destination : Option < Vec < DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationEl > > , dynamic : DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElDynamic , }
impl DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditEl {
    #[doc = "Set the field `findings_destination`.\n"]
    pub fn set_findings_destination(
        mut self,
        v : impl Into < BlockAssignable < DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.findings_destination = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.findings_destination = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditEl {
    type O =
        BlockAssignable<DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditEl {}
impl BuildDataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditEl {
    pub fn build(
        self,
    ) -> DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditEl {
        DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditEl {
            findings_destination: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElRef {
        DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `findings_destination` after provisioning.\n"]    pub fn findings_destination (& self) -> ListRef < DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.findings_destination", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElDeidentifyElMaskConfigEl
{}
impl DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElDeidentifyElMaskConfigEl {}
impl ToListMappable
    for DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElDeidentifyElMaskConfigEl
{
    type O = BlockAssignable<
        DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElDeidentifyElMaskConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElDeidentifyElMaskConfigEl
{}
impl
    BuildDataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElDeidentifyElMaskConfigEl
{
    pub fn build(
        self,
    ) -> DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElDeidentifyElMaskConfigEl
    {
        DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElDeidentifyElMaskConfigEl { }
    }
}
pub struct DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElDeidentifyElMaskConfigElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElDeidentifyElMaskConfigElRef { fn new (shared : StackShared , base : String) -> DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElDeidentifyElMaskConfigElRef { DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElDeidentifyElMaskConfigElRef { shared : shared , base : base . to_string () , } } }
impl
    DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElDeidentifyElMaskConfigElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize, Default)]
struct DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElDeidentifyElDynamic { mask_config : Option < DynamicBlock < DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElDeidentifyElMaskConfigEl >> , }
#[derive(Serialize)]
pub struct DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElDeidentifyEl { # [serde (skip_serializing_if = "Option::is_none")] mask_config : Option < Vec < DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElDeidentifyElMaskConfigEl > > , dynamic : DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElDeidentifyElDynamic , }
impl DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElDeidentifyEl {
    #[doc = "Set the field `mask_config`.\n"]
    pub fn set_mask_config(
        mut self,
        v : impl Into < BlockAssignable < DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElDeidentifyElMaskConfigEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.mask_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.mask_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElDeidentifyEl
{
    type O = BlockAssignable<
        DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElDeidentifyEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElDeidentifyEl {}
impl BuildDataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElDeidentifyEl {
    pub fn build(
        self,
    ) -> DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElDeidentifyEl {
        DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElDeidentifyEl {
            mask_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElDeidentifyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElDeidentifyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElDeidentifyElRef {
        DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElDeidentifyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElDeidentifyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `mask_config` after provisioning.\n"]    pub fn mask_config (& self) -> ListRef < DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElDeidentifyElMaskConfigElRef >{
        ListRef::new(self.shared().clone(), format!("{}.mask_config", self.base))
    }
}
#[derive(Serialize, Default)]
struct DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElDynamic {
    audit: Option<
        DynamicBlock<DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditEl>,
    >,
    deidentify: Option<
        DynamicBlock<
            DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElDeidentifyEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    audit: Option<Vec<DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deidentify: Option<
        Vec<DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElDeidentifyEl>,
    >,
    dynamic: DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElDynamic,
}
impl DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationEl {
    #[doc = "Set the field `audit`.\n"]
    pub fn set_audit(
        mut self,
        v: impl Into<
            BlockAssignable<
                DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.audit = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.audit = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `deidentify`.\n"]
    pub fn set_deidentify(
        mut self,
        v: impl Into<
            BlockAssignable<
                DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElDeidentifyEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.deidentify = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.deidentify = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationEl {
    type O = BlockAssignable<DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationEl {}
impl BuildDataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationEl {
    pub fn build(self) -> DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationEl {
        DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationEl {
            audit: core::default::Default::default(),
            deidentify: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElRef {
        DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `audit` after provisioning.\n"]
    pub fn audit(
        &self,
    ) -> ListRef<DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.audit", self.base))
    }
    #[doc = "Get a reference to the value of field `deidentify` after provisioning.\n"]
    pub fn deidentify(
        &self,
    ) -> ListRef<DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElDeidentifyElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.deidentify", self.base))
    }
}
#[derive(Serialize, Default)]
struct DataCloudwatchLogDataProtectionPolicyDocumentStatementElDynamic {
    operation:
        Option<DynamicBlock<DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationEl>>,
}
#[derive(Serialize)]
pub struct DataCloudwatchLogDataProtectionPolicyDocumentStatementEl {
    data_identifiers: SetField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sid: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    operation: Option<Vec<DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationEl>>,
    dynamic: DataCloudwatchLogDataProtectionPolicyDocumentStatementElDynamic,
}
impl DataCloudwatchLogDataProtectionPolicyDocumentStatementEl {
    #[doc = "Set the field `sid`.\n"]
    pub fn set_sid(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sid = Some(v.into());
        self
    }
    #[doc = "Set the field `operation`.\n"]
    pub fn set_operation(
        mut self,
        v: impl Into<
            BlockAssignable<DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.operation = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.operation = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataCloudwatchLogDataProtectionPolicyDocumentStatementEl {
    type O = BlockAssignable<DataCloudwatchLogDataProtectionPolicyDocumentStatementEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudwatchLogDataProtectionPolicyDocumentStatementEl {
    #[doc = ""]
    pub data_identifiers: SetField<PrimField<String>>,
}
impl BuildDataCloudwatchLogDataProtectionPolicyDocumentStatementEl {
    pub fn build(self) -> DataCloudwatchLogDataProtectionPolicyDocumentStatementEl {
        DataCloudwatchLogDataProtectionPolicyDocumentStatementEl {
            data_identifiers: self.data_identifiers,
            sid: core::default::Default::default(),
            operation: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataCloudwatchLogDataProtectionPolicyDocumentStatementElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudwatchLogDataProtectionPolicyDocumentStatementElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudwatchLogDataProtectionPolicyDocumentStatementElRef {
        DataCloudwatchLogDataProtectionPolicyDocumentStatementElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudwatchLogDataProtectionPolicyDocumentStatementElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `data_identifiers` after provisioning.\n"]
    pub fn data_identifiers(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.data_identifiers", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `sid` after provisioning.\n"]
    pub fn sid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sid", self.base))
    }
    #[doc = "Get a reference to the value of field `operation` after provisioning.\n"]
    pub fn operation(
        &self,
    ) -> ListRef<DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.operation", self.base))
    }
}
#[derive(Serialize, Default)]
struct DataCloudwatchLogDataProtectionPolicyDocumentDynamic {
    configuration:
        Option<DynamicBlock<DataCloudwatchLogDataProtectionPolicyDocumentConfigurationEl>>,
    statement: Option<DynamicBlock<DataCloudwatchLogDataProtectionPolicyDocumentStatementEl>>,
}
