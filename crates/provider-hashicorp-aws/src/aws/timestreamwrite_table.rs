use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct TimestreamwriteTableData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    database_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    table_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    magnetic_store_write_properties:
        Option<Vec<TimestreamwriteTableMagneticStoreWritePropertiesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retention_properties: Option<Vec<TimestreamwriteTableRetentionPropertiesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schema: Option<Vec<TimestreamwriteTableSchemaEl>>,
    dynamic: TimestreamwriteTableDynamic,
}
struct TimestreamwriteTable_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<TimestreamwriteTableData>,
}
#[derive(Clone)]
pub struct TimestreamwriteTable(Rc<TimestreamwriteTable_>);
impl TimestreamwriteTable {
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
    #[doc = "Set the field `magnetic_store_write_properties`.\n"]
    pub fn set_magnetic_store_write_properties(
        self,
        v: impl Into<BlockAssignable<TimestreamwriteTableMagneticStoreWritePropertiesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().magnetic_store_write_properties = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .magnetic_store_write_properties = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `retention_properties`.\n"]
    pub fn set_retention_properties(
        self,
        v: impl Into<BlockAssignable<TimestreamwriteTableRetentionPropertiesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().retention_properties = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.retention_properties = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `schema`.\n"]
    pub fn set_schema(self, v: impl Into<BlockAssignable<TimestreamwriteTableSchemaEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().schema = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.schema = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `database_name` after provisioning.\n"]
    pub fn database_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.database_name", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `table_name` after provisioning.\n"]
    pub fn table_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.table_name", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `magnetic_store_write_properties` after provisioning.\n"]
    pub fn magnetic_store_write_properties(
        &self,
    ) -> ListRef<TimestreamwriteTableMagneticStoreWritePropertiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.magnetic_store_write_properties", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `retention_properties` after provisioning.\n"]
    pub fn retention_properties(&self) -> ListRef<TimestreamwriteTableRetentionPropertiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.retention_properties", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `schema` after provisioning.\n"]
    pub fn schema(&self) -> ListRef<TimestreamwriteTableSchemaElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.schema", self.extract_ref()),
        )
    }
}
impl Referable for TimestreamwriteTable {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for TimestreamwriteTable {}
impl ToListMappable for TimestreamwriteTable {
    type O = ListRef<TimestreamwriteTableRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for TimestreamwriteTable_ {
    fn extract_resource_type(&self) -> String {
        "aws_timestreamwrite_table".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildTimestreamwriteTable {
    pub tf_id: String,
    #[doc = ""]
    pub database_name: PrimField<String>,
    #[doc = ""]
    pub table_name: PrimField<String>,
}
impl BuildTimestreamwriteTable {
    pub fn build(self, stack: &mut Stack) -> TimestreamwriteTable {
        let out = TimestreamwriteTable(Rc::new(TimestreamwriteTable_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(TimestreamwriteTableData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                database_name: self.database_name,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                table_name: self.table_name,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                magnetic_store_write_properties: core::default::Default::default(),
                retention_properties: core::default::Default::default(),
                schema: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct TimestreamwriteTableRef {
    shared: StackShared,
    base: String,
}
impl Ref for TimestreamwriteTableRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl TimestreamwriteTableRef {
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
    #[doc = "Get a reference to the value of field `database_name` after provisioning.\n"]
    pub fn database_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.database_name", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `table_name` after provisioning.\n"]
    pub fn table_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.table_name", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `magnetic_store_write_properties` after provisioning.\n"]
    pub fn magnetic_store_write_properties(
        &self,
    ) -> ListRef<TimestreamwriteTableMagneticStoreWritePropertiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.magnetic_store_write_properties", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `retention_properties` after provisioning.\n"]
    pub fn retention_properties(&self) -> ListRef<TimestreamwriteTableRetentionPropertiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.retention_properties", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `schema` after provisioning.\n"]
    pub fn schema(&self) -> ListRef<TimestreamwriteTableSchemaElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.schema", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct TimestreamwriteTableMagneticStoreWritePropertiesElMagneticStoreRejectedDataLocationElS3ConfigurationEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_option: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    object_key_prefix: Option<PrimField<String>>,
}
impl TimestreamwriteTableMagneticStoreWritePropertiesElMagneticStoreRejectedDataLocationElS3ConfigurationEl { # [doc = "Set the field `bucket_name`.\n"] pub fn set_bucket_name (mut self , v : impl Into < PrimField < String > >) -> Self { self . bucket_name = Some (v . into ()) ; self } # [doc = "Set the field `encryption_option`.\n"] pub fn set_encryption_option (mut self , v : impl Into < PrimField < String > >) -> Self { self . encryption_option = Some (v . into ()) ; self } # [doc = "Set the field `kms_key_id`.\n"] pub fn set_kms_key_id (mut self , v : impl Into < PrimField < String > >) -> Self { self . kms_key_id = Some (v . into ()) ; self } # [doc = "Set the field `object_key_prefix`.\n"] pub fn set_object_key_prefix (mut self , v : impl Into < PrimField < String > >) -> Self { self . object_key_prefix = Some (v . into ()) ; self } }
impl ToListMappable for TimestreamwriteTableMagneticStoreWritePropertiesElMagneticStoreRejectedDataLocationElS3ConfigurationEl { type O = BlockAssignable < TimestreamwriteTableMagneticStoreWritePropertiesElMagneticStoreRejectedDataLocationElS3ConfigurationEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildTimestreamwriteTableMagneticStoreWritePropertiesElMagneticStoreRejectedDataLocationElS3ConfigurationEl
{}
impl BuildTimestreamwriteTableMagneticStoreWritePropertiesElMagneticStoreRejectedDataLocationElS3ConfigurationEl { pub fn build (self) -> TimestreamwriteTableMagneticStoreWritePropertiesElMagneticStoreRejectedDataLocationElS3ConfigurationEl { TimestreamwriteTableMagneticStoreWritePropertiesElMagneticStoreRejectedDataLocationElS3ConfigurationEl { bucket_name : core :: default :: Default :: default () , encryption_option : core :: default :: Default :: default () , kms_key_id : core :: default :: Default :: default () , object_key_prefix : core :: default :: Default :: default () , } } }
pub struct TimestreamwriteTableMagneticStoreWritePropertiesElMagneticStoreRejectedDataLocationElS3ConfigurationElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for TimestreamwriteTableMagneticStoreWritePropertiesElMagneticStoreRejectedDataLocationElS3ConfigurationElRef { fn new (shared : StackShared , base : String) -> TimestreamwriteTableMagneticStoreWritePropertiesElMagneticStoreRejectedDataLocationElS3ConfigurationElRef { TimestreamwriteTableMagneticStoreWritePropertiesElMagneticStoreRejectedDataLocationElS3ConfigurationElRef { shared : shared , base : base . to_string () , } } }
impl TimestreamwriteTableMagneticStoreWritePropertiesElMagneticStoreRejectedDataLocationElS3ConfigurationElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"] pub fn bucket_name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.bucket_name" , self . base)) } # [doc = "Get a reference to the value of field `encryption_option` after provisioning.\n"] pub fn encryption_option (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.encryption_option" , self . base)) } # [doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"] pub fn kms_key_id (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.kms_key_id" , self . base)) } # [doc = "Get a reference to the value of field `object_key_prefix` after provisioning.\n"] pub fn object_key_prefix (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.object_key_prefix" , self . base)) } }
#[derive(Serialize, Default)]
struct TimestreamwriteTableMagneticStoreWritePropertiesElMagneticStoreRejectedDataLocationElDynamic { s3_configuration : Option < DynamicBlock < TimestreamwriteTableMagneticStoreWritePropertiesElMagneticStoreRejectedDataLocationElS3ConfigurationEl >> , }
#[derive(Serialize)]
pub struct TimestreamwriteTableMagneticStoreWritePropertiesElMagneticStoreRejectedDataLocationEl { # [serde (skip_serializing_if = "Option::is_none")] s3_configuration : Option < Vec < TimestreamwriteTableMagneticStoreWritePropertiesElMagneticStoreRejectedDataLocationElS3ConfigurationEl > > , dynamic : TimestreamwriteTableMagneticStoreWritePropertiesElMagneticStoreRejectedDataLocationElDynamic , }
impl TimestreamwriteTableMagneticStoreWritePropertiesElMagneticStoreRejectedDataLocationEl {
    #[doc = "Set the field `s3_configuration`.\n"]
    pub fn set_s3_configuration(
        mut self,
        v : impl Into < BlockAssignable < TimestreamwriteTableMagneticStoreWritePropertiesElMagneticStoreRejectedDataLocationElS3ConfigurationEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.s3_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_configuration = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for TimestreamwriteTableMagneticStoreWritePropertiesElMagneticStoreRejectedDataLocationEl
{
    type O = BlockAssignable<
        TimestreamwriteTableMagneticStoreWritePropertiesElMagneticStoreRejectedDataLocationEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildTimestreamwriteTableMagneticStoreWritePropertiesElMagneticStoreRejectedDataLocationEl
{}
impl BuildTimestreamwriteTableMagneticStoreWritePropertiesElMagneticStoreRejectedDataLocationEl {
    pub fn build(
        self,
    ) -> TimestreamwriteTableMagneticStoreWritePropertiesElMagneticStoreRejectedDataLocationEl {
        TimestreamwriteTableMagneticStoreWritePropertiesElMagneticStoreRejectedDataLocationEl {
            s3_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct TimestreamwriteTableMagneticStoreWritePropertiesElMagneticStoreRejectedDataLocationElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for TimestreamwriteTableMagneticStoreWritePropertiesElMagneticStoreRejectedDataLocationElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> TimestreamwriteTableMagneticStoreWritePropertiesElMagneticStoreRejectedDataLocationElRef
    {
        TimestreamwriteTableMagneticStoreWritePropertiesElMagneticStoreRejectedDataLocationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl TimestreamwriteTableMagneticStoreWritePropertiesElMagneticStoreRejectedDataLocationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `s3_configuration` after provisioning.\n"]    pub fn s3_configuration (& self) -> ListRef < TimestreamwriteTableMagneticStoreWritePropertiesElMagneticStoreRejectedDataLocationElS3ConfigurationElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.s3_configuration", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct TimestreamwriteTableMagneticStoreWritePropertiesElDynamic {
    magnetic_store_rejected_data_location: Option<
        DynamicBlock<
            TimestreamwriteTableMagneticStoreWritePropertiesElMagneticStoreRejectedDataLocationEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct TimestreamwriteTableMagneticStoreWritePropertiesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_magnetic_store_writes: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    magnetic_store_rejected_data_location: Option<
        Vec<TimestreamwriteTableMagneticStoreWritePropertiesElMagneticStoreRejectedDataLocationEl>,
    >,
    dynamic: TimestreamwriteTableMagneticStoreWritePropertiesElDynamic,
}
impl TimestreamwriteTableMagneticStoreWritePropertiesEl {
    #[doc = "Set the field `enable_magnetic_store_writes`.\n"]
    pub fn set_enable_magnetic_store_writes(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_magnetic_store_writes = Some(v.into());
        self
    }
    #[doc = "Set the field `magnetic_store_rejected_data_location`.\n"]
    pub fn set_magnetic_store_rejected_data_location(
        mut self,
        v : impl Into < BlockAssignable < TimestreamwriteTableMagneticStoreWritePropertiesElMagneticStoreRejectedDataLocationEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.magnetic_store_rejected_data_location = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.magnetic_store_rejected_data_location = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for TimestreamwriteTableMagneticStoreWritePropertiesEl {
    type O = BlockAssignable<TimestreamwriteTableMagneticStoreWritePropertiesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildTimestreamwriteTableMagneticStoreWritePropertiesEl {}
impl BuildTimestreamwriteTableMagneticStoreWritePropertiesEl {
    pub fn build(self) -> TimestreamwriteTableMagneticStoreWritePropertiesEl {
        TimestreamwriteTableMagneticStoreWritePropertiesEl {
            enable_magnetic_store_writes: core::default::Default::default(),
            magnetic_store_rejected_data_location: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct TimestreamwriteTableMagneticStoreWritePropertiesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for TimestreamwriteTableMagneticStoreWritePropertiesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> TimestreamwriteTableMagneticStoreWritePropertiesElRef {
        TimestreamwriteTableMagneticStoreWritePropertiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl TimestreamwriteTableMagneticStoreWritePropertiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enable_magnetic_store_writes` after provisioning.\n"]
    pub fn enable_magnetic_store_writes(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_magnetic_store_writes", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `magnetic_store_rejected_data_location` after provisioning.\n"]
    pub fn magnetic_store_rejected_data_location(
        &self,
    ) -> ListRef<
        TimestreamwriteTableMagneticStoreWritePropertiesElMagneticStoreRejectedDataLocationElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.magnetic_store_rejected_data_location", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct TimestreamwriteTableRetentionPropertiesEl {
    magnetic_store_retention_period_in_days: PrimField<f64>,
    memory_store_retention_period_in_hours: PrimField<f64>,
}
impl TimestreamwriteTableRetentionPropertiesEl {}
impl ToListMappable for TimestreamwriteTableRetentionPropertiesEl {
    type O = BlockAssignable<TimestreamwriteTableRetentionPropertiesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildTimestreamwriteTableRetentionPropertiesEl {
    #[doc = ""]
    pub magnetic_store_retention_period_in_days: PrimField<f64>,
    #[doc = ""]
    pub memory_store_retention_period_in_hours: PrimField<f64>,
}
impl BuildTimestreamwriteTableRetentionPropertiesEl {
    pub fn build(self) -> TimestreamwriteTableRetentionPropertiesEl {
        TimestreamwriteTableRetentionPropertiesEl {
            magnetic_store_retention_period_in_days: self.magnetic_store_retention_period_in_days,
            memory_store_retention_period_in_hours: self.memory_store_retention_period_in_hours,
        }
    }
}
pub struct TimestreamwriteTableRetentionPropertiesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for TimestreamwriteTableRetentionPropertiesElRef {
    fn new(shared: StackShared, base: String) -> TimestreamwriteTableRetentionPropertiesElRef {
        TimestreamwriteTableRetentionPropertiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl TimestreamwriteTableRetentionPropertiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `magnetic_store_retention_period_in_days` after provisioning.\n"]
    pub fn magnetic_store_retention_period_in_days(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.magnetic_store_retention_period_in_days", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `memory_store_retention_period_in_hours` after provisioning.\n"]
    pub fn memory_store_retention_period_in_hours(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.memory_store_retention_period_in_hours", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct TimestreamwriteTableSchemaElCompositePartitionKeyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enforcement_in_record: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}
impl TimestreamwriteTableSchemaElCompositePartitionKeyEl {
    #[doc = "Set the field `enforcement_in_record`.\n"]
    pub fn set_enforcement_in_record(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.enforcement_in_record = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}
impl ToListMappable for TimestreamwriteTableSchemaElCompositePartitionKeyEl {
    type O = BlockAssignable<TimestreamwriteTableSchemaElCompositePartitionKeyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildTimestreamwriteTableSchemaElCompositePartitionKeyEl {
    #[doc = ""]
    pub type_: PrimField<String>,
}
impl BuildTimestreamwriteTableSchemaElCompositePartitionKeyEl {
    pub fn build(self) -> TimestreamwriteTableSchemaElCompositePartitionKeyEl {
        TimestreamwriteTableSchemaElCompositePartitionKeyEl {
            enforcement_in_record: core::default::Default::default(),
            name: core::default::Default::default(),
            type_: self.type_,
        }
    }
}
pub struct TimestreamwriteTableSchemaElCompositePartitionKeyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for TimestreamwriteTableSchemaElCompositePartitionKeyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> TimestreamwriteTableSchemaElCompositePartitionKeyElRef {
        TimestreamwriteTableSchemaElCompositePartitionKeyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl TimestreamwriteTableSchemaElCompositePartitionKeyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enforcement_in_record` after provisioning.\n"]
    pub fn enforcement_in_record(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enforcement_in_record", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize, Default)]
struct TimestreamwriteTableSchemaElDynamic {
    composite_partition_key:
        Option<DynamicBlock<TimestreamwriteTableSchemaElCompositePartitionKeyEl>>,
}
#[derive(Serialize)]
pub struct TimestreamwriteTableSchemaEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    composite_partition_key: Option<Vec<TimestreamwriteTableSchemaElCompositePartitionKeyEl>>,
    dynamic: TimestreamwriteTableSchemaElDynamic,
}
impl TimestreamwriteTableSchemaEl {
    #[doc = "Set the field `composite_partition_key`.\n"]
    pub fn set_composite_partition_key(
        mut self,
        v: impl Into<BlockAssignable<TimestreamwriteTableSchemaElCompositePartitionKeyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.composite_partition_key = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.composite_partition_key = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for TimestreamwriteTableSchemaEl {
    type O = BlockAssignable<TimestreamwriteTableSchemaEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildTimestreamwriteTableSchemaEl {}
impl BuildTimestreamwriteTableSchemaEl {
    pub fn build(self) -> TimestreamwriteTableSchemaEl {
        TimestreamwriteTableSchemaEl {
            composite_partition_key: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct TimestreamwriteTableSchemaElRef {
    shared: StackShared,
    base: String,
}
impl Ref for TimestreamwriteTableSchemaElRef {
    fn new(shared: StackShared, base: String) -> TimestreamwriteTableSchemaElRef {
        TimestreamwriteTableSchemaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl TimestreamwriteTableSchemaElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `composite_partition_key` after provisioning.\n"]
    pub fn composite_partition_key(
        &self,
    ) -> ListRef<TimestreamwriteTableSchemaElCompositePartitionKeyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.composite_partition_key", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct TimestreamwriteTableDynamic {
    magnetic_store_write_properties:
        Option<DynamicBlock<TimestreamwriteTableMagneticStoreWritePropertiesEl>>,
    retention_properties: Option<DynamicBlock<TimestreamwriteTableRetentionPropertiesEl>>,
    schema: Option<DynamicBlock<TimestreamwriteTableSchemaEl>>,
}
