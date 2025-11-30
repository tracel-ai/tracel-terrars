use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct DataDynamodbTableData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    server_side_encryption: Option<Vec<DataDynamodbTableServerSideEncryptionEl>>,
    dynamic: DataDynamodbTableDynamic,
}

struct DataDynamodbTable_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataDynamodbTableData>,
}

#[derive(Clone)]
pub struct DataDynamodbTable(Rc<DataDynamodbTable_>);

impl DataDynamodbTable {
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

    #[doc = "Set the field `server_side_encryption`.\n"]
    pub fn set_server_side_encryption(
        self,
        v: impl Into<BlockAssignable<DataDynamodbTableServerSideEncryptionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().server_side_encryption = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.server_side_encryption = Some(d);
            }
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `attribute` after provisioning.\n"]
    pub fn attribute(&self) -> SetRef<DataDynamodbTableAttributeElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.attribute", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `billing_mode` after provisioning.\n"]
    pub fn billing_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.billing_mode", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `deletion_protection_enabled` after provisioning.\n"]
    pub fn deletion_protection_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_protection_enabled", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `global_secondary_index` after provisioning.\n"]
    pub fn global_secondary_index(&self) -> SetRef<DataDynamodbTableGlobalSecondaryIndexElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.global_secondary_index", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `hash_key` after provisioning.\n"]
    pub fn hash_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.hash_key", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `local_secondary_index` after provisioning.\n"]
    pub fn local_secondary_index(&self) -> SetRef<DataDynamodbTableLocalSecondaryIndexElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.local_secondary_index", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `on_demand_throughput` after provisioning.\n"]
    pub fn on_demand_throughput(&self) -> ListRef<DataDynamodbTableOnDemandThroughputElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.on_demand_throughput", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `point_in_time_recovery` after provisioning.\n"]
    pub fn point_in_time_recovery(&self) -> ListRef<DataDynamodbTablePointInTimeRecoveryElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.point_in_time_recovery", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `range_key` after provisioning.\n"]
    pub fn range_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.range_key", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `read_capacity` after provisioning.\n"]
    pub fn read_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.read_capacity", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `replica` after provisioning.\n"]
    pub fn replica(&self) -> SetRef<DataDynamodbTableReplicaElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.replica", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `stream_arn` after provisioning.\n"]
    pub fn stream_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.stream_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `stream_enabled` after provisioning.\n"]
    pub fn stream_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.stream_enabled", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `stream_label` after provisioning.\n"]
    pub fn stream_label(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.stream_label", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `stream_view_type` after provisioning.\n"]
    pub fn stream_view_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.stream_view_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `table_class` after provisioning.\n"]
    pub fn table_class(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.table_class", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `ttl` after provisioning.\n"]
    pub fn ttl(&self) -> SetRef<DataDynamodbTableTtlElRef> {
        SetRef::new(self.shared().clone(), format!("{}.ttl", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `warm_throughput` after provisioning.\n"]
    pub fn warm_throughput(&self) -> ListRef<DataDynamodbTableWarmThroughputElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.warm_throughput", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `write_capacity` after provisioning.\n"]
    pub fn write_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.write_capacity", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `server_side_encryption` after provisioning.\n"]
    pub fn server_side_encryption(&self) -> ListRef<DataDynamodbTableServerSideEncryptionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.server_side_encryption", self.extract_ref()),
        )
    }
}

impl Referable for DataDynamodbTable {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Datasource for DataDynamodbTable {}

impl ToListMappable for DataDynamodbTable {
    type O = ListRef<DataDynamodbTableRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataDynamodbTable_ {
    fn extract_datasource_type(&self) -> String {
        "aws_dynamodb_table".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataDynamodbTable {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildDataDynamodbTable {
    pub fn build(self, stack: &mut Stack) -> DataDynamodbTable {
        let out = DataDynamodbTable(Rc::new(DataDynamodbTable_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataDynamodbTableData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                server_side_encryption: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataDynamodbTableRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataDynamodbTableRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl DataDynamodbTableRef {
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

    #[doc = "Get a reference to the value of field `attribute` after provisioning.\n"]
    pub fn attribute(&self) -> SetRef<DataDynamodbTableAttributeElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.attribute", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `billing_mode` after provisioning.\n"]
    pub fn billing_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.billing_mode", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `deletion_protection_enabled` after provisioning.\n"]
    pub fn deletion_protection_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_protection_enabled", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `global_secondary_index` after provisioning.\n"]
    pub fn global_secondary_index(&self) -> SetRef<DataDynamodbTableGlobalSecondaryIndexElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.global_secondary_index", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `hash_key` after provisioning.\n"]
    pub fn hash_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.hash_key", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `local_secondary_index` after provisioning.\n"]
    pub fn local_secondary_index(&self) -> SetRef<DataDynamodbTableLocalSecondaryIndexElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.local_secondary_index", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `on_demand_throughput` after provisioning.\n"]
    pub fn on_demand_throughput(&self) -> ListRef<DataDynamodbTableOnDemandThroughputElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.on_demand_throughput", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `point_in_time_recovery` after provisioning.\n"]
    pub fn point_in_time_recovery(&self) -> ListRef<DataDynamodbTablePointInTimeRecoveryElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.point_in_time_recovery", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `range_key` after provisioning.\n"]
    pub fn range_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.range_key", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `read_capacity` after provisioning.\n"]
    pub fn read_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.read_capacity", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `replica` after provisioning.\n"]
    pub fn replica(&self) -> SetRef<DataDynamodbTableReplicaElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.replica", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `stream_arn` after provisioning.\n"]
    pub fn stream_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.stream_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `stream_enabled` after provisioning.\n"]
    pub fn stream_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.stream_enabled", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `stream_label` after provisioning.\n"]
    pub fn stream_label(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.stream_label", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `stream_view_type` after provisioning.\n"]
    pub fn stream_view_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.stream_view_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `table_class` after provisioning.\n"]
    pub fn table_class(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.table_class", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `ttl` after provisioning.\n"]
    pub fn ttl(&self) -> SetRef<DataDynamodbTableTtlElRef> {
        SetRef::new(self.shared().clone(), format!("{}.ttl", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `warm_throughput` after provisioning.\n"]
    pub fn warm_throughput(&self) -> ListRef<DataDynamodbTableWarmThroughputElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.warm_throughput", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `write_capacity` after provisioning.\n"]
    pub fn write_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.write_capacity", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `server_side_encryption` after provisioning.\n"]
    pub fn server_side_encryption(&self) -> ListRef<DataDynamodbTableServerSideEncryptionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.server_side_encryption", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DataDynamodbTableAttributeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}

impl DataDynamodbTableAttributeEl {
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

impl ToListMappable for DataDynamodbTableAttributeEl {
    type O = BlockAssignable<DataDynamodbTableAttributeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDynamodbTableAttributeEl {}

impl BuildDataDynamodbTableAttributeEl {
    pub fn build(self) -> DataDynamodbTableAttributeEl {
        DataDynamodbTableAttributeEl {
            name: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct DataDynamodbTableAttributeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataDynamodbTableAttributeElRef {
    fn new(shared: StackShared, base: String) -> DataDynamodbTableAttributeElRef {
        DataDynamodbTableAttributeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDynamodbTableAttributeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
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
pub struct DataDynamodbTableGlobalSecondaryIndexElOnDemandThroughputEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_read_request_units: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_write_request_units: Option<PrimField<f64>>,
}

impl DataDynamodbTableGlobalSecondaryIndexElOnDemandThroughputEl {
    #[doc = "Set the field `max_read_request_units`.\n"]
    pub fn set_max_read_request_units(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_read_request_units = Some(v.into());
        self
    }

    #[doc = "Set the field `max_write_request_units`.\n"]
    pub fn set_max_write_request_units(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_write_request_units = Some(v.into());
        self
    }
}

impl ToListMappable for DataDynamodbTableGlobalSecondaryIndexElOnDemandThroughputEl {
    type O = BlockAssignable<DataDynamodbTableGlobalSecondaryIndexElOnDemandThroughputEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDynamodbTableGlobalSecondaryIndexElOnDemandThroughputEl {}

impl BuildDataDynamodbTableGlobalSecondaryIndexElOnDemandThroughputEl {
    pub fn build(self) -> DataDynamodbTableGlobalSecondaryIndexElOnDemandThroughputEl {
        DataDynamodbTableGlobalSecondaryIndexElOnDemandThroughputEl {
            max_read_request_units: core::default::Default::default(),
            max_write_request_units: core::default::Default::default(),
        }
    }
}

pub struct DataDynamodbTableGlobalSecondaryIndexElOnDemandThroughputElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataDynamodbTableGlobalSecondaryIndexElOnDemandThroughputElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataDynamodbTableGlobalSecondaryIndexElOnDemandThroughputElRef {
        DataDynamodbTableGlobalSecondaryIndexElOnDemandThroughputElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDynamodbTableGlobalSecondaryIndexElOnDemandThroughputElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_read_request_units` after provisioning.\n"]
    pub fn max_read_request_units(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_read_request_units", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `max_write_request_units` after provisioning.\n"]
    pub fn max_write_request_units(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_write_request_units", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataDynamodbTableGlobalSecondaryIndexElWarmThroughputEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    read_units_per_second: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    write_units_per_second: Option<PrimField<f64>>,
}

impl DataDynamodbTableGlobalSecondaryIndexElWarmThroughputEl {
    #[doc = "Set the field `read_units_per_second`.\n"]
    pub fn set_read_units_per_second(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.read_units_per_second = Some(v.into());
        self
    }

    #[doc = "Set the field `write_units_per_second`.\n"]
    pub fn set_write_units_per_second(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.write_units_per_second = Some(v.into());
        self
    }
}

impl ToListMappable for DataDynamodbTableGlobalSecondaryIndexElWarmThroughputEl {
    type O = BlockAssignable<DataDynamodbTableGlobalSecondaryIndexElWarmThroughputEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDynamodbTableGlobalSecondaryIndexElWarmThroughputEl {}

impl BuildDataDynamodbTableGlobalSecondaryIndexElWarmThroughputEl {
    pub fn build(self) -> DataDynamodbTableGlobalSecondaryIndexElWarmThroughputEl {
        DataDynamodbTableGlobalSecondaryIndexElWarmThroughputEl {
            read_units_per_second: core::default::Default::default(),
            write_units_per_second: core::default::Default::default(),
        }
    }
}

pub struct DataDynamodbTableGlobalSecondaryIndexElWarmThroughputElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataDynamodbTableGlobalSecondaryIndexElWarmThroughputElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataDynamodbTableGlobalSecondaryIndexElWarmThroughputElRef {
        DataDynamodbTableGlobalSecondaryIndexElWarmThroughputElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDynamodbTableGlobalSecondaryIndexElWarmThroughputElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `read_units_per_second` after provisioning.\n"]
    pub fn read_units_per_second(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.read_units_per_second", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `write_units_per_second` after provisioning.\n"]
    pub fn write_units_per_second(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.write_units_per_second", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataDynamodbTableGlobalSecondaryIndexEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    hash_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    non_key_attributes: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    on_demand_throughput:
        Option<ListField<DataDynamodbTableGlobalSecondaryIndexElOnDemandThroughputEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    projection_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    range_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    read_capacity: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    warm_throughput: Option<ListField<DataDynamodbTableGlobalSecondaryIndexElWarmThroughputEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    write_capacity: Option<PrimField<f64>>,
}

impl DataDynamodbTableGlobalSecondaryIndexEl {
    #[doc = "Set the field `hash_key`.\n"]
    pub fn set_hash_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.hash_key = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc = "Set the field `non_key_attributes`.\n"]
    pub fn set_non_key_attributes(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.non_key_attributes = Some(v.into());
        self
    }

    #[doc = "Set the field `on_demand_throughput`.\n"]
    pub fn set_on_demand_throughput(
        mut self,
        v: impl Into<ListField<DataDynamodbTableGlobalSecondaryIndexElOnDemandThroughputEl>>,
    ) -> Self {
        self.on_demand_throughput = Some(v.into());
        self
    }

    #[doc = "Set the field `projection_type`.\n"]
    pub fn set_projection_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.projection_type = Some(v.into());
        self
    }

    #[doc = "Set the field `range_key`.\n"]
    pub fn set_range_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.range_key = Some(v.into());
        self
    }

    #[doc = "Set the field `read_capacity`.\n"]
    pub fn set_read_capacity(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.read_capacity = Some(v.into());
        self
    }

    #[doc = "Set the field `warm_throughput`.\n"]
    pub fn set_warm_throughput(
        mut self,
        v: impl Into<ListField<DataDynamodbTableGlobalSecondaryIndexElWarmThroughputEl>>,
    ) -> Self {
        self.warm_throughput = Some(v.into());
        self
    }

    #[doc = "Set the field `write_capacity`.\n"]
    pub fn set_write_capacity(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.write_capacity = Some(v.into());
        self
    }
}

impl ToListMappable for DataDynamodbTableGlobalSecondaryIndexEl {
    type O = BlockAssignable<DataDynamodbTableGlobalSecondaryIndexEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDynamodbTableGlobalSecondaryIndexEl {}

impl BuildDataDynamodbTableGlobalSecondaryIndexEl {
    pub fn build(self) -> DataDynamodbTableGlobalSecondaryIndexEl {
        DataDynamodbTableGlobalSecondaryIndexEl {
            hash_key: core::default::Default::default(),
            name: core::default::Default::default(),
            non_key_attributes: core::default::Default::default(),
            on_demand_throughput: core::default::Default::default(),
            projection_type: core::default::Default::default(),
            range_key: core::default::Default::default(),
            read_capacity: core::default::Default::default(),
            warm_throughput: core::default::Default::default(),
            write_capacity: core::default::Default::default(),
        }
    }
}

pub struct DataDynamodbTableGlobalSecondaryIndexElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataDynamodbTableGlobalSecondaryIndexElRef {
    fn new(shared: StackShared, base: String) -> DataDynamodbTableGlobalSecondaryIndexElRef {
        DataDynamodbTableGlobalSecondaryIndexElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDynamodbTableGlobalSecondaryIndexElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hash_key` after provisioning.\n"]
    pub fn hash_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.hash_key", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `non_key_attributes` after provisioning.\n"]
    pub fn non_key_attributes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.non_key_attributes", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `on_demand_throughput` after provisioning.\n"]
    pub fn on_demand_throughput(
        &self,
    ) -> ListRef<DataDynamodbTableGlobalSecondaryIndexElOnDemandThroughputElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.on_demand_throughput", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `projection_type` after provisioning.\n"]
    pub fn projection_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.projection_type", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `range_key` after provisioning.\n"]
    pub fn range_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.range_key", self.base))
    }

    #[doc = "Get a reference to the value of field `read_capacity` after provisioning.\n"]
    pub fn read_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.read_capacity", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `warm_throughput` after provisioning.\n"]
    pub fn warm_throughput(
        &self,
    ) -> ListRef<DataDynamodbTableGlobalSecondaryIndexElWarmThroughputElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.warm_throughput", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `write_capacity` after provisioning.\n"]
    pub fn write_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.write_capacity", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataDynamodbTableLocalSecondaryIndexEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    non_key_attributes: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    projection_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    range_key: Option<PrimField<String>>,
}

impl DataDynamodbTableLocalSecondaryIndexEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc = "Set the field `non_key_attributes`.\n"]
    pub fn set_non_key_attributes(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.non_key_attributes = Some(v.into());
        self
    }

    #[doc = "Set the field `projection_type`.\n"]
    pub fn set_projection_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.projection_type = Some(v.into());
        self
    }

    #[doc = "Set the field `range_key`.\n"]
    pub fn set_range_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.range_key = Some(v.into());
        self
    }
}

impl ToListMappable for DataDynamodbTableLocalSecondaryIndexEl {
    type O = BlockAssignable<DataDynamodbTableLocalSecondaryIndexEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDynamodbTableLocalSecondaryIndexEl {}

impl BuildDataDynamodbTableLocalSecondaryIndexEl {
    pub fn build(self) -> DataDynamodbTableLocalSecondaryIndexEl {
        DataDynamodbTableLocalSecondaryIndexEl {
            name: core::default::Default::default(),
            non_key_attributes: core::default::Default::default(),
            projection_type: core::default::Default::default(),
            range_key: core::default::Default::default(),
        }
    }
}

pub struct DataDynamodbTableLocalSecondaryIndexElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataDynamodbTableLocalSecondaryIndexElRef {
    fn new(shared: StackShared, base: String) -> DataDynamodbTableLocalSecondaryIndexElRef {
        DataDynamodbTableLocalSecondaryIndexElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDynamodbTableLocalSecondaryIndexElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `non_key_attributes` after provisioning.\n"]
    pub fn non_key_attributes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.non_key_attributes", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `projection_type` after provisioning.\n"]
    pub fn projection_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.projection_type", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `range_key` after provisioning.\n"]
    pub fn range_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.range_key", self.base))
    }
}

#[derive(Serialize)]
pub struct DataDynamodbTableOnDemandThroughputEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_read_request_units: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_write_request_units: Option<PrimField<f64>>,
}

impl DataDynamodbTableOnDemandThroughputEl {
    #[doc = "Set the field `max_read_request_units`.\n"]
    pub fn set_max_read_request_units(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_read_request_units = Some(v.into());
        self
    }

    #[doc = "Set the field `max_write_request_units`.\n"]
    pub fn set_max_write_request_units(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_write_request_units = Some(v.into());
        self
    }
}

impl ToListMappable for DataDynamodbTableOnDemandThroughputEl {
    type O = BlockAssignable<DataDynamodbTableOnDemandThroughputEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDynamodbTableOnDemandThroughputEl {}

impl BuildDataDynamodbTableOnDemandThroughputEl {
    pub fn build(self) -> DataDynamodbTableOnDemandThroughputEl {
        DataDynamodbTableOnDemandThroughputEl {
            max_read_request_units: core::default::Default::default(),
            max_write_request_units: core::default::Default::default(),
        }
    }
}

pub struct DataDynamodbTableOnDemandThroughputElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataDynamodbTableOnDemandThroughputElRef {
    fn new(shared: StackShared, base: String) -> DataDynamodbTableOnDemandThroughputElRef {
        DataDynamodbTableOnDemandThroughputElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDynamodbTableOnDemandThroughputElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_read_request_units` after provisioning.\n"]
    pub fn max_read_request_units(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_read_request_units", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `max_write_request_units` after provisioning.\n"]
    pub fn max_write_request_units(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_write_request_units", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataDynamodbTablePointInTimeRecoveryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recovery_period_in_days: Option<PrimField<f64>>,
}

impl DataDynamodbTablePointInTimeRecoveryEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `recovery_period_in_days`.\n"]
    pub fn set_recovery_period_in_days(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.recovery_period_in_days = Some(v.into());
        self
    }
}

impl ToListMappable for DataDynamodbTablePointInTimeRecoveryEl {
    type O = BlockAssignable<DataDynamodbTablePointInTimeRecoveryEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDynamodbTablePointInTimeRecoveryEl {}

impl BuildDataDynamodbTablePointInTimeRecoveryEl {
    pub fn build(self) -> DataDynamodbTablePointInTimeRecoveryEl {
        DataDynamodbTablePointInTimeRecoveryEl {
            enabled: core::default::Default::default(),
            recovery_period_in_days: core::default::Default::default(),
        }
    }
}

pub struct DataDynamodbTablePointInTimeRecoveryElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataDynamodbTablePointInTimeRecoveryElRef {
    fn new(shared: StackShared, base: String) -> DataDynamodbTablePointInTimeRecoveryElRef {
        DataDynamodbTablePointInTimeRecoveryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDynamodbTablePointInTimeRecoveryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `recovery_period_in_days` after provisioning.\n"]
    pub fn recovery_period_in_days(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.recovery_period_in_days", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataDynamodbTableReplicaEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region_name: Option<PrimField<String>>,
}

impl DataDynamodbTableReplicaEl {
    #[doc = "Set the field `kms_key_arn`.\n"]
    pub fn set_kms_key_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `region_name`.\n"]
    pub fn set_region_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.region_name = Some(v.into());
        self
    }
}

impl ToListMappable for DataDynamodbTableReplicaEl {
    type O = BlockAssignable<DataDynamodbTableReplicaEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDynamodbTableReplicaEl {}

impl BuildDataDynamodbTableReplicaEl {
    pub fn build(self) -> DataDynamodbTableReplicaEl {
        DataDynamodbTableReplicaEl {
            kms_key_arn: core::default::Default::default(),
            region_name: core::default::Default::default(),
        }
    }
}

pub struct DataDynamodbTableReplicaElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataDynamodbTableReplicaElRef {
    fn new(shared: StackShared, base: String) -> DataDynamodbTableReplicaElRef {
        DataDynamodbTableReplicaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDynamodbTableReplicaElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `region_name` after provisioning.\n"]
    pub fn region_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataDynamodbTableTtlEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    attribute_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}

impl DataDynamodbTableTtlEl {
    #[doc = "Set the field `attribute_name`.\n"]
    pub fn set_attribute_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.attribute_name = Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}

impl ToListMappable for DataDynamodbTableTtlEl {
    type O = BlockAssignable<DataDynamodbTableTtlEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDynamodbTableTtlEl {}

impl BuildDataDynamodbTableTtlEl {
    pub fn build(self) -> DataDynamodbTableTtlEl {
        DataDynamodbTableTtlEl {
            attribute_name: core::default::Default::default(),
            enabled: core::default::Default::default(),
        }
    }
}

pub struct DataDynamodbTableTtlElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataDynamodbTableTtlElRef {
    fn new(shared: StackShared, base: String) -> DataDynamodbTableTtlElRef {
        DataDynamodbTableTtlElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDynamodbTableTtlElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `attribute_name` after provisioning.\n"]
    pub fn attribute_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.attribute_name", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataDynamodbTableWarmThroughputEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    read_units_per_second: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    write_units_per_second: Option<PrimField<f64>>,
}

impl DataDynamodbTableWarmThroughputEl {
    #[doc = "Set the field `read_units_per_second`.\n"]
    pub fn set_read_units_per_second(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.read_units_per_second = Some(v.into());
        self
    }

    #[doc = "Set the field `write_units_per_second`.\n"]
    pub fn set_write_units_per_second(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.write_units_per_second = Some(v.into());
        self
    }
}

impl ToListMappable for DataDynamodbTableWarmThroughputEl {
    type O = BlockAssignable<DataDynamodbTableWarmThroughputEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDynamodbTableWarmThroughputEl {}

impl BuildDataDynamodbTableWarmThroughputEl {
    pub fn build(self) -> DataDynamodbTableWarmThroughputEl {
        DataDynamodbTableWarmThroughputEl {
            read_units_per_second: core::default::Default::default(),
            write_units_per_second: core::default::Default::default(),
        }
    }
}

pub struct DataDynamodbTableWarmThroughputElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataDynamodbTableWarmThroughputElRef {
    fn new(shared: StackShared, base: String) -> DataDynamodbTableWarmThroughputElRef {
        DataDynamodbTableWarmThroughputElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDynamodbTableWarmThroughputElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `read_units_per_second` after provisioning.\n"]
    pub fn read_units_per_second(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.read_units_per_second", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `write_units_per_second` after provisioning.\n"]
    pub fn write_units_per_second(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.write_units_per_second", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataDynamodbTableServerSideEncryptionEl {}

impl DataDynamodbTableServerSideEncryptionEl {}

impl ToListMappable for DataDynamodbTableServerSideEncryptionEl {
    type O = BlockAssignable<DataDynamodbTableServerSideEncryptionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDynamodbTableServerSideEncryptionEl {}

impl BuildDataDynamodbTableServerSideEncryptionEl {
    pub fn build(self) -> DataDynamodbTableServerSideEncryptionEl {
        DataDynamodbTableServerSideEncryptionEl {}
    }
}

pub struct DataDynamodbTableServerSideEncryptionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataDynamodbTableServerSideEncryptionElRef {
    fn new(shared: StackShared, base: String) -> DataDynamodbTableServerSideEncryptionElRef {
        DataDynamodbTableServerSideEncryptionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDynamodbTableServerSideEncryptionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataDynamodbTableDynamic {
    server_side_encryption: Option<DynamicBlock<DataDynamodbTableServerSideEncryptionEl>>,
}
