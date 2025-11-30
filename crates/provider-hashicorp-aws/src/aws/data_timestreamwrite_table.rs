use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct DataTimestreamwriteTableData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    database_name: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}

struct DataTimestreamwriteTable_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataTimestreamwriteTableData>,
}

#[derive(Clone)]
pub struct DataTimestreamwriteTable(Rc<DataTimestreamwriteTable_>);

impl DataTimestreamwriteTable {
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

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `creation_time` after provisioning.\n"]
    pub fn creation_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creation_time", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `database_name` after provisioning.\n"]
    pub fn database_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.database_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `last_updated_time` after provisioning.\n"]
    pub fn last_updated_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_updated_time", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `magnetic_store_write_properties` after provisioning.\n"]
    pub fn magnetic_store_write_properties(
        &self,
    ) -> ListRef<DataTimestreamwriteTableMagneticStoreWritePropertiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.magnetic_store_write_properties", self.extract_ref()),
        )
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

    #[doc = "Get a reference to the value of field `retention_properties` after provisioning.\n"]
    pub fn retention_properties(
        &self,
    ) -> ListRef<DataTimestreamwriteTableRetentionPropertiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.retention_properties", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `schema` after provisioning.\n"]
    pub fn schema(&self) -> ListRef<DataTimestreamwriteTableSchemaElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.schema", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `table_status` after provisioning.\n"]
    pub fn table_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.table_status", self.extract_ref()),
        )
    }
}

impl Referable for DataTimestreamwriteTable {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Datasource for DataTimestreamwriteTable {}

impl ToListMappable for DataTimestreamwriteTable {
    type O = ListRef<DataTimestreamwriteTableRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataTimestreamwriteTable_ {
    fn extract_datasource_type(&self) -> String {
        "aws_timestreamwrite_table".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataTimestreamwriteTable {
    pub tf_id: String,
    #[doc = ""]
    pub database_name: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildDataTimestreamwriteTable {
    pub fn build(self, stack: &mut Stack) -> DataTimestreamwriteTable {
        let out = DataTimestreamwriteTable(Rc::new(DataTimestreamwriteTable_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataTimestreamwriteTableData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                database_name: self.database_name,
                name: self.name,
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataTimestreamwriteTableRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataTimestreamwriteTableRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl DataTimestreamwriteTableRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `creation_time` after provisioning.\n"]
    pub fn creation_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creation_time", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `database_name` after provisioning.\n"]
    pub fn database_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.database_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `last_updated_time` after provisioning.\n"]
    pub fn last_updated_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_updated_time", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `magnetic_store_write_properties` after provisioning.\n"]
    pub fn magnetic_store_write_properties(
        &self,
    ) -> ListRef<DataTimestreamwriteTableMagneticStoreWritePropertiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.magnetic_store_write_properties", self.extract_ref()),
        )
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

    #[doc = "Get a reference to the value of field `retention_properties` after provisioning.\n"]
    pub fn retention_properties(
        &self,
    ) -> ListRef<DataTimestreamwriteTableRetentionPropertiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.retention_properties", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `schema` after provisioning.\n"]
    pub fn schema(&self) -> ListRef<DataTimestreamwriteTableSchemaElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.schema", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `table_status` after provisioning.\n"]
    pub fn table_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.table_status", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DataTimestreamwriteTableMagneticStoreWritePropertiesElMagneticStoreRejectedDataLocationElS3ConfigurationEl
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

impl DataTimestreamwriteTableMagneticStoreWritePropertiesElMagneticStoreRejectedDataLocationElS3ConfigurationEl {
    #[doc = "Set the field `bucket_name`.\n"]
    pub fn set_bucket_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket_name = Some(v.into());
        self
    }

    #[doc = "Set the field `encryption_option`.\n"]
    pub fn set_encryption_option(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.encryption_option = Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_id = Some(v.into());
        self
    }

    #[doc = "Set the field `object_key_prefix`.\n"]
    pub fn set_object_key_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.object_key_prefix = Some(v.into());
        self
    }
}

impl ToListMappable for DataTimestreamwriteTableMagneticStoreWritePropertiesElMagneticStoreRejectedDataLocationElS3ConfigurationEl {
    type O =
        BlockAssignable<
            DataTimestreamwriteTableMagneticStoreWritePropertiesElMagneticStoreRejectedDataLocationElS3ConfigurationEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataTimestreamwriteTableMagneticStoreWritePropertiesElMagneticStoreRejectedDataLocationElS3ConfigurationEl
{}

impl BuildDataTimestreamwriteTableMagneticStoreWritePropertiesElMagneticStoreRejectedDataLocationElS3ConfigurationEl {
    pub fn build(
        self,
    ) -> DataTimestreamwriteTableMagneticStoreWritePropertiesElMagneticStoreRejectedDataLocationElS3ConfigurationEl {
        DataTimestreamwriteTableMagneticStoreWritePropertiesElMagneticStoreRejectedDataLocationElS3ConfigurationEl {
            bucket_name: core::default::Default::default(),
            encryption_option: core::default::Default::default(),
            kms_key_id: core::default::Default::default(),
            object_key_prefix: core::default::Default::default(),
        }
    }
}

pub struct DataTimestreamwriteTableMagneticStoreWritePropertiesElMagneticStoreRejectedDataLocationElS3ConfigurationElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for DataTimestreamwriteTableMagneticStoreWritePropertiesElMagneticStoreRejectedDataLocationElS3ConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataTimestreamwriteTableMagneticStoreWritePropertiesElMagneticStoreRejectedDataLocationElS3ConfigurationElRef {
        DataTimestreamwriteTableMagneticStoreWritePropertiesElMagneticStoreRejectedDataLocationElS3ConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataTimestreamwriteTableMagneticStoreWritePropertiesElMagneticStoreRejectedDataLocationElS3ConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `encryption_option` after provisioning.\n"]
    pub fn encryption_option(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.encryption_option", self.base))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.base))
    }

    #[doc = "Get a reference to the value of field `object_key_prefix` after provisioning.\n"]
    pub fn object_key_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.object_key_prefix", self.base))
    }
}

#[derive(Serialize)]
pub struct DataTimestreamwriteTableMagneticStoreWritePropertiesElMagneticStoreRejectedDataLocationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_configuration: Option<
        ListField<
            DataTimestreamwriteTableMagneticStoreWritePropertiesElMagneticStoreRejectedDataLocationElS3ConfigurationEl,
        >,
    >,
}

impl DataTimestreamwriteTableMagneticStoreWritePropertiesElMagneticStoreRejectedDataLocationEl {
    #[doc = "Set the field `s3_configuration`.\n"]
    pub fn set_s3_configuration(
        mut self,
        v:
            impl

                    Into<
                        ListField<
                            DataTimestreamwriteTableMagneticStoreWritePropertiesElMagneticStoreRejectedDataLocationElS3ConfigurationEl,
                        >,
                    >,
    ) -> Self {
        self.s3_configuration = Some(v.into());
        self
    }
}

impl ToListMappable
    for DataTimestreamwriteTableMagneticStoreWritePropertiesElMagneticStoreRejectedDataLocationEl
{
    type O = BlockAssignable<
        DataTimestreamwriteTableMagneticStoreWritePropertiesElMagneticStoreRejectedDataLocationEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataTimestreamwriteTableMagneticStoreWritePropertiesElMagneticStoreRejectedDataLocationEl
{}

impl
    BuildDataTimestreamwriteTableMagneticStoreWritePropertiesElMagneticStoreRejectedDataLocationEl
{
    pub fn build(
        self,
    ) -> DataTimestreamwriteTableMagneticStoreWritePropertiesElMagneticStoreRejectedDataLocationEl
    {
        DataTimestreamwriteTableMagneticStoreWritePropertiesElMagneticStoreRejectedDataLocationEl {
            s3_configuration: core::default::Default::default(),
        }
    }
}

pub struct DataTimestreamwriteTableMagneticStoreWritePropertiesElMagneticStoreRejectedDataLocationElRef
{
    shared: StackShared,
    base: String,
}

impl Ref
    for DataTimestreamwriteTableMagneticStoreWritePropertiesElMagneticStoreRejectedDataLocationElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataTimestreamwriteTableMagneticStoreWritePropertiesElMagneticStoreRejectedDataLocationElRef
    {
        DataTimestreamwriteTableMagneticStoreWritePropertiesElMagneticStoreRejectedDataLocationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataTimestreamwriteTableMagneticStoreWritePropertiesElMagneticStoreRejectedDataLocationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3_configuration` after provisioning.\n"]
    pub fn s3_configuration(
        &self,
    ) -> ListRef<
        DataTimestreamwriteTableMagneticStoreWritePropertiesElMagneticStoreRejectedDataLocationElS3ConfigurationElRef,
    >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.s3_configuration", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataTimestreamwriteTableMagneticStoreWritePropertiesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_magnetic_store_writes: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    magnetic_store_rejected_data_location: Option<
        ListField<DataTimestreamwriteTableMagneticStoreWritePropertiesElMagneticStoreRejectedDataLocationEl>,
    >,
}

impl DataTimestreamwriteTableMagneticStoreWritePropertiesEl {
    #[doc = "Set the field `enable_magnetic_store_writes`.\n"]
    pub fn set_enable_magnetic_store_writes(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_magnetic_store_writes = Some(v.into());
        self
    }

    #[doc = "Set the field `magnetic_store_rejected_data_location`.\n"]
    pub fn set_magnetic_store_rejected_data_location(
        mut self,
        v:
            impl

                    Into<
                        ListField<
                            DataTimestreamwriteTableMagneticStoreWritePropertiesElMagneticStoreRejectedDataLocationEl,
                        >,
                    >,
    ) -> Self {
        self.magnetic_store_rejected_data_location = Some(v.into());
        self
    }
}

impl ToListMappable for DataTimestreamwriteTableMagneticStoreWritePropertiesEl {
    type O = BlockAssignable<DataTimestreamwriteTableMagneticStoreWritePropertiesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataTimestreamwriteTableMagneticStoreWritePropertiesEl {}

impl BuildDataTimestreamwriteTableMagneticStoreWritePropertiesEl {
    pub fn build(self) -> DataTimestreamwriteTableMagneticStoreWritePropertiesEl {
        DataTimestreamwriteTableMagneticStoreWritePropertiesEl {
            enable_magnetic_store_writes: core::default::Default::default(),
            magnetic_store_rejected_data_location: core::default::Default::default(),
        }
    }
}

pub struct DataTimestreamwriteTableMagneticStoreWritePropertiesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataTimestreamwriteTableMagneticStoreWritePropertiesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataTimestreamwriteTableMagneticStoreWritePropertiesElRef {
        DataTimestreamwriteTableMagneticStoreWritePropertiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataTimestreamwriteTableMagneticStoreWritePropertiesElRef {
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
    ) -> ListRef<DataTimestreamwriteTableMagneticStoreWritePropertiesElMagneticStoreRejectedDataLocationElRef>{
        ListRef::new(
            self.shared().clone(),
            format!("{}.magnetic_store_rejected_data_location", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataTimestreamwriteTableRetentionPropertiesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    magnetic_store_retention_period_in_days: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memory_store_retention_period_in_hours: Option<PrimField<f64>>,
}

impl DataTimestreamwriteTableRetentionPropertiesEl {
    #[doc = "Set the field `magnetic_store_retention_period_in_days`.\n"]
    pub fn set_magnetic_store_retention_period_in_days(
        mut self,
        v: impl Into<PrimField<f64>>,
    ) -> Self {
        self.magnetic_store_retention_period_in_days = Some(v.into());
        self
    }

    #[doc = "Set the field `memory_store_retention_period_in_hours`.\n"]
    pub fn set_memory_store_retention_period_in_hours(
        mut self,
        v: impl Into<PrimField<f64>>,
    ) -> Self {
        self.memory_store_retention_period_in_hours = Some(v.into());
        self
    }
}

impl ToListMappable for DataTimestreamwriteTableRetentionPropertiesEl {
    type O = BlockAssignable<DataTimestreamwriteTableRetentionPropertiesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataTimestreamwriteTableRetentionPropertiesEl {}

impl BuildDataTimestreamwriteTableRetentionPropertiesEl {
    pub fn build(self) -> DataTimestreamwriteTableRetentionPropertiesEl {
        DataTimestreamwriteTableRetentionPropertiesEl {
            magnetic_store_retention_period_in_days: core::default::Default::default(),
            memory_store_retention_period_in_hours: core::default::Default::default(),
        }
    }
}

pub struct DataTimestreamwriteTableRetentionPropertiesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataTimestreamwriteTableRetentionPropertiesElRef {
    fn new(shared: StackShared, base: String) -> DataTimestreamwriteTableRetentionPropertiesElRef {
        DataTimestreamwriteTableRetentionPropertiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataTimestreamwriteTableRetentionPropertiesElRef {
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
pub struct DataTimestreamwriteTableSchemaElCompositePartitionKeyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enforcement_in_record: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}

impl DataTimestreamwriteTableSchemaElCompositePartitionKeyEl {
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

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}

impl ToListMappable for DataTimestreamwriteTableSchemaElCompositePartitionKeyEl {
    type O = BlockAssignable<DataTimestreamwriteTableSchemaElCompositePartitionKeyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataTimestreamwriteTableSchemaElCompositePartitionKeyEl {}

impl BuildDataTimestreamwriteTableSchemaElCompositePartitionKeyEl {
    pub fn build(self) -> DataTimestreamwriteTableSchemaElCompositePartitionKeyEl {
        DataTimestreamwriteTableSchemaElCompositePartitionKeyEl {
            enforcement_in_record: core::default::Default::default(),
            name: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct DataTimestreamwriteTableSchemaElCompositePartitionKeyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataTimestreamwriteTableSchemaElCompositePartitionKeyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataTimestreamwriteTableSchemaElCompositePartitionKeyElRef {
        DataTimestreamwriteTableSchemaElCompositePartitionKeyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataTimestreamwriteTableSchemaElCompositePartitionKeyElRef {
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

#[derive(Serialize)]
pub struct DataTimestreamwriteTableSchemaEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    composite_partition_key:
        Option<ListField<DataTimestreamwriteTableSchemaElCompositePartitionKeyEl>>,
}

impl DataTimestreamwriteTableSchemaEl {
    #[doc = "Set the field `composite_partition_key`.\n"]
    pub fn set_composite_partition_key(
        mut self,
        v: impl Into<ListField<DataTimestreamwriteTableSchemaElCompositePartitionKeyEl>>,
    ) -> Self {
        self.composite_partition_key = Some(v.into());
        self
    }
}

impl ToListMappable for DataTimestreamwriteTableSchemaEl {
    type O = BlockAssignable<DataTimestreamwriteTableSchemaEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataTimestreamwriteTableSchemaEl {}

impl BuildDataTimestreamwriteTableSchemaEl {
    pub fn build(self) -> DataTimestreamwriteTableSchemaEl {
        DataTimestreamwriteTableSchemaEl {
            composite_partition_key: core::default::Default::default(),
        }
    }
}

pub struct DataTimestreamwriteTableSchemaElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataTimestreamwriteTableSchemaElRef {
    fn new(shared: StackShared, base: String) -> DataTimestreamwriteTableSchemaElRef {
        DataTimestreamwriteTableSchemaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataTimestreamwriteTableSchemaElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `composite_partition_key` after provisioning.\n"]
    pub fn composite_partition_key(
        &self,
    ) -> ListRef<DataTimestreamwriteTableSchemaElCompositePartitionKeyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.composite_partition_key", self.base),
        )
    }
}
