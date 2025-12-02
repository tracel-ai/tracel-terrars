use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct KeyspacesTableData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_time_to_live: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    keyspace_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    table_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    capacity_specification: Option<Vec<KeyspacesTableCapacitySpecificationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_side_timestamps: Option<Vec<KeyspacesTableClientSideTimestampsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    comment: Option<Vec<KeyspacesTableCommentEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_specification: Option<Vec<KeyspacesTableEncryptionSpecificationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    point_in_time_recovery: Option<Vec<KeyspacesTablePointInTimeRecoveryEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schema_definition: Option<Vec<KeyspacesTableSchemaDefinitionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<KeyspacesTableTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ttl: Option<Vec<KeyspacesTableTtlEl>>,
    dynamic: KeyspacesTableDynamic,
}
struct KeyspacesTable_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<KeyspacesTableData>,
}
#[derive(Clone)]
pub struct KeyspacesTable(Rc<KeyspacesTable_>);
impl KeyspacesTable {
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
    #[doc = "Set the field `default_time_to_live`.\n"]
    pub fn set_default_time_to_live(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().default_time_to_live = Some(v.into());
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
    #[doc = "Set the field `capacity_specification`.\n"]
    pub fn set_capacity_specification(
        self,
        v: impl Into<BlockAssignable<KeyspacesTableCapacitySpecificationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().capacity_specification = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.capacity_specification = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `client_side_timestamps`.\n"]
    pub fn set_client_side_timestamps(
        self,
        v: impl Into<BlockAssignable<KeyspacesTableClientSideTimestampsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().client_side_timestamps = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.client_side_timestamps = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `comment`.\n"]
    pub fn set_comment(self, v: impl Into<BlockAssignable<KeyspacesTableCommentEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().comment = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.comment = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `encryption_specification`.\n"]
    pub fn set_encryption_specification(
        self,
        v: impl Into<BlockAssignable<KeyspacesTableEncryptionSpecificationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().encryption_specification = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.encryption_specification = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `point_in_time_recovery`.\n"]
    pub fn set_point_in_time_recovery(
        self,
        v: impl Into<BlockAssignable<KeyspacesTablePointInTimeRecoveryEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().point_in_time_recovery = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.point_in_time_recovery = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `schema_definition`.\n"]
    pub fn set_schema_definition(
        self,
        v: impl Into<BlockAssignable<KeyspacesTableSchemaDefinitionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().schema_definition = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.schema_definition = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<KeyspacesTableTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Set the field `ttl`.\n"]
    pub fn set_ttl(self, v: impl Into<BlockAssignable<KeyspacesTableTtlEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ttl = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ttl = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `default_time_to_live` after provisioning.\n"]
    pub fn default_time_to_live(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_time_to_live", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `keyspace_name` after provisioning.\n"]
    pub fn keyspace_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.keyspace_name", self.extract_ref()),
        )
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
    #[doc = "Get a reference to the value of field `capacity_specification` after provisioning.\n"]
    pub fn capacity_specification(&self) -> ListRef<KeyspacesTableCapacitySpecificationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.capacity_specification", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `client_side_timestamps` after provisioning.\n"]
    pub fn client_side_timestamps(&self) -> ListRef<KeyspacesTableClientSideTimestampsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.client_side_timestamps", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `comment` after provisioning.\n"]
    pub fn comment(&self) -> ListRef<KeyspacesTableCommentElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.comment", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `encryption_specification` after provisioning.\n"]
    pub fn encryption_specification(&self) -> ListRef<KeyspacesTableEncryptionSpecificationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.encryption_specification", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `point_in_time_recovery` after provisioning.\n"]
    pub fn point_in_time_recovery(&self) -> ListRef<KeyspacesTablePointInTimeRecoveryElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.point_in_time_recovery", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `schema_definition` after provisioning.\n"]
    pub fn schema_definition(&self) -> ListRef<KeyspacesTableSchemaDefinitionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.schema_definition", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> KeyspacesTableTimeoutsElRef {
        KeyspacesTableTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ttl` after provisioning.\n"]
    pub fn ttl(&self) -> ListRef<KeyspacesTableTtlElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ttl", self.extract_ref()))
    }
}
impl Referable for KeyspacesTable {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for KeyspacesTable {}
impl ToListMappable for KeyspacesTable {
    type O = ListRef<KeyspacesTableRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for KeyspacesTable_ {
    fn extract_resource_type(&self) -> String {
        "aws_keyspaces_table".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildKeyspacesTable {
    pub tf_id: String,
    #[doc = ""]
    pub keyspace_name: PrimField<String>,
    #[doc = ""]
    pub table_name: PrimField<String>,
}
impl BuildKeyspacesTable {
    pub fn build(self, stack: &mut Stack) -> KeyspacesTable {
        let out = KeyspacesTable(Rc::new(KeyspacesTable_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(KeyspacesTableData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                default_time_to_live: core::default::Default::default(),
                id: core::default::Default::default(),
                keyspace_name: self.keyspace_name,
                region: core::default::Default::default(),
                table_name: self.table_name,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                capacity_specification: core::default::Default::default(),
                client_side_timestamps: core::default::Default::default(),
                comment: core::default::Default::default(),
                encryption_specification: core::default::Default::default(),
                point_in_time_recovery: core::default::Default::default(),
                schema_definition: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                ttl: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct KeyspacesTableRef {
    shared: StackShared,
    base: String,
}
impl Ref for KeyspacesTableRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl KeyspacesTableRef {
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
    #[doc = "Get a reference to the value of field `default_time_to_live` after provisioning.\n"]
    pub fn default_time_to_live(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_time_to_live", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `keyspace_name` after provisioning.\n"]
    pub fn keyspace_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.keyspace_name", self.extract_ref()),
        )
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
    #[doc = "Get a reference to the value of field `capacity_specification` after provisioning.\n"]
    pub fn capacity_specification(&self) -> ListRef<KeyspacesTableCapacitySpecificationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.capacity_specification", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `client_side_timestamps` after provisioning.\n"]
    pub fn client_side_timestamps(&self) -> ListRef<KeyspacesTableClientSideTimestampsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.client_side_timestamps", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `comment` after provisioning.\n"]
    pub fn comment(&self) -> ListRef<KeyspacesTableCommentElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.comment", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `encryption_specification` after provisioning.\n"]
    pub fn encryption_specification(&self) -> ListRef<KeyspacesTableEncryptionSpecificationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.encryption_specification", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `point_in_time_recovery` after provisioning.\n"]
    pub fn point_in_time_recovery(&self) -> ListRef<KeyspacesTablePointInTimeRecoveryElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.point_in_time_recovery", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `schema_definition` after provisioning.\n"]
    pub fn schema_definition(&self) -> ListRef<KeyspacesTableSchemaDefinitionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.schema_definition", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> KeyspacesTableTimeoutsElRef {
        KeyspacesTableTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ttl` after provisioning.\n"]
    pub fn ttl(&self) -> ListRef<KeyspacesTableTtlElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ttl", self.extract_ref()))
    }
}
#[derive(Serialize)]
pub struct KeyspacesTableCapacitySpecificationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    read_capacity_units: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    throughput_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    write_capacity_units: Option<PrimField<f64>>,
}
impl KeyspacesTableCapacitySpecificationEl {
    #[doc = "Set the field `read_capacity_units`.\n"]
    pub fn set_read_capacity_units(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.read_capacity_units = Some(v.into());
        self
    }
    #[doc = "Set the field `throughput_mode`.\n"]
    pub fn set_throughput_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.throughput_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `write_capacity_units`.\n"]
    pub fn set_write_capacity_units(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.write_capacity_units = Some(v.into());
        self
    }
}
impl ToListMappable for KeyspacesTableCapacitySpecificationEl {
    type O = BlockAssignable<KeyspacesTableCapacitySpecificationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildKeyspacesTableCapacitySpecificationEl {}
impl BuildKeyspacesTableCapacitySpecificationEl {
    pub fn build(self) -> KeyspacesTableCapacitySpecificationEl {
        KeyspacesTableCapacitySpecificationEl {
            read_capacity_units: core::default::Default::default(),
            throughput_mode: core::default::Default::default(),
            write_capacity_units: core::default::Default::default(),
        }
    }
}
pub struct KeyspacesTableCapacitySpecificationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for KeyspacesTableCapacitySpecificationElRef {
    fn new(shared: StackShared, base: String) -> KeyspacesTableCapacitySpecificationElRef {
        KeyspacesTableCapacitySpecificationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl KeyspacesTableCapacitySpecificationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `read_capacity_units` after provisioning.\n"]
    pub fn read_capacity_units(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.read_capacity_units", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `throughput_mode` after provisioning.\n"]
    pub fn throughput_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.throughput_mode", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `write_capacity_units` after provisioning.\n"]
    pub fn write_capacity_units(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.write_capacity_units", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct KeyspacesTableClientSideTimestampsEl {
    status: PrimField<String>,
}
impl KeyspacesTableClientSideTimestampsEl {}
impl ToListMappable for KeyspacesTableClientSideTimestampsEl {
    type O = BlockAssignable<KeyspacesTableClientSideTimestampsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildKeyspacesTableClientSideTimestampsEl {
    #[doc = ""]
    pub status: PrimField<String>,
}
impl BuildKeyspacesTableClientSideTimestampsEl {
    pub fn build(self) -> KeyspacesTableClientSideTimestampsEl {
        KeyspacesTableClientSideTimestampsEl {
            status: self.status,
        }
    }
}
pub struct KeyspacesTableClientSideTimestampsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for KeyspacesTableClientSideTimestampsElRef {
    fn new(shared: StackShared, base: String) -> KeyspacesTableClientSideTimestampsElRef {
        KeyspacesTableClientSideTimestampsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl KeyspacesTableClientSideTimestampsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}
#[derive(Serialize)]
pub struct KeyspacesTableCommentEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<PrimField<String>>,
}
impl KeyspacesTableCommentEl {
    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message = Some(v.into());
        self
    }
}
impl ToListMappable for KeyspacesTableCommentEl {
    type O = BlockAssignable<KeyspacesTableCommentEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildKeyspacesTableCommentEl {}
impl BuildKeyspacesTableCommentEl {
    pub fn build(self) -> KeyspacesTableCommentEl {
        KeyspacesTableCommentEl {
            message: core::default::Default::default(),
        }
    }
}
pub struct KeyspacesTableCommentElRef {
    shared: StackShared,
    base: String,
}
impl Ref for KeyspacesTableCommentElRef {
    fn new(shared: StackShared, base: String) -> KeyspacesTableCommentElRef {
        KeyspacesTableCommentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl KeyspacesTableCommentElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
}
#[derive(Serialize)]
pub struct KeyspacesTableEncryptionSpecificationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_identifier: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl KeyspacesTableEncryptionSpecificationEl {
    #[doc = "Set the field `kms_key_identifier`.\n"]
    pub fn set_kms_key_identifier(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_identifier = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable for KeyspacesTableEncryptionSpecificationEl {
    type O = BlockAssignable<KeyspacesTableEncryptionSpecificationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildKeyspacesTableEncryptionSpecificationEl {}
impl BuildKeyspacesTableEncryptionSpecificationEl {
    pub fn build(self) -> KeyspacesTableEncryptionSpecificationEl {
        KeyspacesTableEncryptionSpecificationEl {
            kms_key_identifier: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}
pub struct KeyspacesTableEncryptionSpecificationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for KeyspacesTableEncryptionSpecificationElRef {
    fn new(shared: StackShared, base: String) -> KeyspacesTableEncryptionSpecificationElRef {
        KeyspacesTableEncryptionSpecificationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl KeyspacesTableEncryptionSpecificationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `kms_key_identifier` after provisioning.\n"]
    pub fn kms_key_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_identifier", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct KeyspacesTablePointInTimeRecoveryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
}
impl KeyspacesTablePointInTimeRecoveryEl {
    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status = Some(v.into());
        self
    }
}
impl ToListMappable for KeyspacesTablePointInTimeRecoveryEl {
    type O = BlockAssignable<KeyspacesTablePointInTimeRecoveryEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildKeyspacesTablePointInTimeRecoveryEl {}
impl BuildKeyspacesTablePointInTimeRecoveryEl {
    pub fn build(self) -> KeyspacesTablePointInTimeRecoveryEl {
        KeyspacesTablePointInTimeRecoveryEl {
            status: core::default::Default::default(),
        }
    }
}
pub struct KeyspacesTablePointInTimeRecoveryElRef {
    shared: StackShared,
    base: String,
}
impl Ref for KeyspacesTablePointInTimeRecoveryElRef {
    fn new(shared: StackShared, base: String) -> KeyspacesTablePointInTimeRecoveryElRef {
        KeyspacesTablePointInTimeRecoveryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl KeyspacesTablePointInTimeRecoveryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}
#[derive(Serialize)]
pub struct KeyspacesTableSchemaDefinitionElClusteringKeyEl {
    name: PrimField<String>,
    order_by: PrimField<String>,
}
impl KeyspacesTableSchemaDefinitionElClusteringKeyEl {}
impl ToListMappable for KeyspacesTableSchemaDefinitionElClusteringKeyEl {
    type O = BlockAssignable<KeyspacesTableSchemaDefinitionElClusteringKeyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildKeyspacesTableSchemaDefinitionElClusteringKeyEl {
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub order_by: PrimField<String>,
}
impl BuildKeyspacesTableSchemaDefinitionElClusteringKeyEl {
    pub fn build(self) -> KeyspacesTableSchemaDefinitionElClusteringKeyEl {
        KeyspacesTableSchemaDefinitionElClusteringKeyEl {
            name: self.name,
            order_by: self.order_by,
        }
    }
}
pub struct KeyspacesTableSchemaDefinitionElClusteringKeyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for KeyspacesTableSchemaDefinitionElClusteringKeyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> KeyspacesTableSchemaDefinitionElClusteringKeyElRef {
        KeyspacesTableSchemaDefinitionElClusteringKeyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl KeyspacesTableSchemaDefinitionElClusteringKeyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `order_by` after provisioning.\n"]
    pub fn order_by(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.order_by", self.base))
    }
}
#[derive(Serialize)]
pub struct KeyspacesTableSchemaDefinitionElColumnEl {
    name: PrimField<String>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}
impl KeyspacesTableSchemaDefinitionElColumnEl {}
impl ToListMappable for KeyspacesTableSchemaDefinitionElColumnEl {
    type O = BlockAssignable<KeyspacesTableSchemaDefinitionElColumnEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildKeyspacesTableSchemaDefinitionElColumnEl {
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub type_: PrimField<String>,
}
impl BuildKeyspacesTableSchemaDefinitionElColumnEl {
    pub fn build(self) -> KeyspacesTableSchemaDefinitionElColumnEl {
        KeyspacesTableSchemaDefinitionElColumnEl {
            name: self.name,
            type_: self.type_,
        }
    }
}
pub struct KeyspacesTableSchemaDefinitionElColumnElRef {
    shared: StackShared,
    base: String,
}
impl Ref for KeyspacesTableSchemaDefinitionElColumnElRef {
    fn new(shared: StackShared, base: String) -> KeyspacesTableSchemaDefinitionElColumnElRef {
        KeyspacesTableSchemaDefinitionElColumnElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl KeyspacesTableSchemaDefinitionElColumnElRef {
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
pub struct KeyspacesTableSchemaDefinitionElPartitionKeyEl {
    name: PrimField<String>,
}
impl KeyspacesTableSchemaDefinitionElPartitionKeyEl {}
impl ToListMappable for KeyspacesTableSchemaDefinitionElPartitionKeyEl {
    type O = BlockAssignable<KeyspacesTableSchemaDefinitionElPartitionKeyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildKeyspacesTableSchemaDefinitionElPartitionKeyEl {
    #[doc = ""]
    pub name: PrimField<String>,
}
impl BuildKeyspacesTableSchemaDefinitionElPartitionKeyEl {
    pub fn build(self) -> KeyspacesTableSchemaDefinitionElPartitionKeyEl {
        KeyspacesTableSchemaDefinitionElPartitionKeyEl { name: self.name }
    }
}
pub struct KeyspacesTableSchemaDefinitionElPartitionKeyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for KeyspacesTableSchemaDefinitionElPartitionKeyElRef {
    fn new(shared: StackShared, base: String) -> KeyspacesTableSchemaDefinitionElPartitionKeyElRef {
        KeyspacesTableSchemaDefinitionElPartitionKeyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl KeyspacesTableSchemaDefinitionElPartitionKeyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
#[derive(Serialize)]
pub struct KeyspacesTableSchemaDefinitionElStaticColumnEl {
    name: PrimField<String>,
}
impl KeyspacesTableSchemaDefinitionElStaticColumnEl {}
impl ToListMappable for KeyspacesTableSchemaDefinitionElStaticColumnEl {
    type O = BlockAssignable<KeyspacesTableSchemaDefinitionElStaticColumnEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildKeyspacesTableSchemaDefinitionElStaticColumnEl {
    #[doc = ""]
    pub name: PrimField<String>,
}
impl BuildKeyspacesTableSchemaDefinitionElStaticColumnEl {
    pub fn build(self) -> KeyspacesTableSchemaDefinitionElStaticColumnEl {
        KeyspacesTableSchemaDefinitionElStaticColumnEl { name: self.name }
    }
}
pub struct KeyspacesTableSchemaDefinitionElStaticColumnElRef {
    shared: StackShared,
    base: String,
}
impl Ref for KeyspacesTableSchemaDefinitionElStaticColumnElRef {
    fn new(shared: StackShared, base: String) -> KeyspacesTableSchemaDefinitionElStaticColumnElRef {
        KeyspacesTableSchemaDefinitionElStaticColumnElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl KeyspacesTableSchemaDefinitionElStaticColumnElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
#[derive(Serialize, Default)]
struct KeyspacesTableSchemaDefinitionElDynamic {
    clustering_key: Option<DynamicBlock<KeyspacesTableSchemaDefinitionElClusteringKeyEl>>,
    column: Option<DynamicBlock<KeyspacesTableSchemaDefinitionElColumnEl>>,
    partition_key: Option<DynamicBlock<KeyspacesTableSchemaDefinitionElPartitionKeyEl>>,
    static_column: Option<DynamicBlock<KeyspacesTableSchemaDefinitionElStaticColumnEl>>,
}
#[derive(Serialize)]
pub struct KeyspacesTableSchemaDefinitionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    clustering_key: Option<Vec<KeyspacesTableSchemaDefinitionElClusteringKeyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    column: Option<Vec<KeyspacesTableSchemaDefinitionElColumnEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    partition_key: Option<Vec<KeyspacesTableSchemaDefinitionElPartitionKeyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    static_column: Option<Vec<KeyspacesTableSchemaDefinitionElStaticColumnEl>>,
    dynamic: KeyspacesTableSchemaDefinitionElDynamic,
}
impl KeyspacesTableSchemaDefinitionEl {
    #[doc = "Set the field `clustering_key`.\n"]
    pub fn set_clustering_key(
        mut self,
        v: impl Into<BlockAssignable<KeyspacesTableSchemaDefinitionElClusteringKeyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.clustering_key = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.clustering_key = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `column`.\n"]
    pub fn set_column(
        mut self,
        v: impl Into<BlockAssignable<KeyspacesTableSchemaDefinitionElColumnEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.column = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.column = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `partition_key`.\n"]
    pub fn set_partition_key(
        mut self,
        v: impl Into<BlockAssignable<KeyspacesTableSchemaDefinitionElPartitionKeyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.partition_key = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.partition_key = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `static_column`.\n"]
    pub fn set_static_column(
        mut self,
        v: impl Into<BlockAssignable<KeyspacesTableSchemaDefinitionElStaticColumnEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.static_column = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.static_column = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for KeyspacesTableSchemaDefinitionEl {
    type O = BlockAssignable<KeyspacesTableSchemaDefinitionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildKeyspacesTableSchemaDefinitionEl {}
impl BuildKeyspacesTableSchemaDefinitionEl {
    pub fn build(self) -> KeyspacesTableSchemaDefinitionEl {
        KeyspacesTableSchemaDefinitionEl {
            clustering_key: core::default::Default::default(),
            column: core::default::Default::default(),
            partition_key: core::default::Default::default(),
            static_column: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct KeyspacesTableSchemaDefinitionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for KeyspacesTableSchemaDefinitionElRef {
    fn new(shared: StackShared, base: String) -> KeyspacesTableSchemaDefinitionElRef {
        KeyspacesTableSchemaDefinitionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl KeyspacesTableSchemaDefinitionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `clustering_key` after provisioning.\n"]
    pub fn clustering_key(&self) -> ListRef<KeyspacesTableSchemaDefinitionElClusteringKeyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.clustering_key", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `partition_key` after provisioning.\n"]
    pub fn partition_key(&self) -> ListRef<KeyspacesTableSchemaDefinitionElPartitionKeyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.partition_key", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct KeyspacesTableTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl KeyspacesTableTimeoutsEl {
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
    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}
impl ToListMappable for KeyspacesTableTimeoutsEl {
    type O = BlockAssignable<KeyspacesTableTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildKeyspacesTableTimeoutsEl {}
impl BuildKeyspacesTableTimeoutsEl {
    pub fn build(self) -> KeyspacesTableTimeoutsEl {
        KeyspacesTableTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct KeyspacesTableTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for KeyspacesTableTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> KeyspacesTableTimeoutsElRef {
        KeyspacesTableTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl KeyspacesTableTimeoutsElRef {
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
    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
#[derive(Serialize)]
pub struct KeyspacesTableTtlEl {
    status: PrimField<String>,
}
impl KeyspacesTableTtlEl {}
impl ToListMappable for KeyspacesTableTtlEl {
    type O = BlockAssignable<KeyspacesTableTtlEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildKeyspacesTableTtlEl {
    #[doc = ""]
    pub status: PrimField<String>,
}
impl BuildKeyspacesTableTtlEl {
    pub fn build(self) -> KeyspacesTableTtlEl {
        KeyspacesTableTtlEl {
            status: self.status,
        }
    }
}
pub struct KeyspacesTableTtlElRef {
    shared: StackShared,
    base: String,
}
impl Ref for KeyspacesTableTtlElRef {
    fn new(shared: StackShared, base: String) -> KeyspacesTableTtlElRef {
        KeyspacesTableTtlElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl KeyspacesTableTtlElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}
#[derive(Serialize, Default)]
struct KeyspacesTableDynamic {
    capacity_specification: Option<DynamicBlock<KeyspacesTableCapacitySpecificationEl>>,
    client_side_timestamps: Option<DynamicBlock<KeyspacesTableClientSideTimestampsEl>>,
    comment: Option<DynamicBlock<KeyspacesTableCommentEl>>,
    encryption_specification: Option<DynamicBlock<KeyspacesTableEncryptionSpecificationEl>>,
    point_in_time_recovery: Option<DynamicBlock<KeyspacesTablePointInTimeRecoveryEl>>,
    schema_definition: Option<DynamicBlock<KeyspacesTableSchemaDefinitionEl>>,
    ttl: Option<DynamicBlock<KeyspacesTableTtlEl>>,
}
