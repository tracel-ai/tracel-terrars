use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct BigtableSchemaBundleData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ignore_warnings: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    schema_bundle_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    table: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    proto_schema: Option<Vec<BigtableSchemaBundleProtoSchemaEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<BigtableSchemaBundleTimeoutsEl>,
    dynamic: BigtableSchemaBundleDynamic,
}
struct BigtableSchemaBundle_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<BigtableSchemaBundleData>,
}
#[derive(Clone)]
pub struct BigtableSchemaBundle(Rc<BigtableSchemaBundle_>);
impl BigtableSchemaBundle {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(self, provider: &ProviderGoogle) -> Self {
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
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `ignore_warnings`.\nIf true, allow backwards incompatible changes."]
    pub fn set_ignore_warnings(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().ignore_warnings = Some(v.into());
        self
    }
    #[doc = "Set the field `instance`.\nThe name of the instance to create the schema bundle within."]
    pub fn set_instance(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().instance = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `table`.\nThe name of the table to create the schema bundle within."]
    pub fn set_table(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().table = Some(v.into());
        self
    }
    #[doc = "Set the field `proto_schema`.\n"]
    pub fn set_proto_schema(
        self,
        v: impl Into<BlockAssignable<BigtableSchemaBundleProtoSchemaEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().proto_schema = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.proto_schema = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<BigtableSchemaBundleTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\netag is used for optimistic concurrency control as a way to help prevent simultaneous\nupdates of a schema bundle from overwriting each other. This may be sent on update and delete\nrequests to ensure the client has an update-to-date value before proceeding. The server returns\nan ABORTED error on a mismatched etag."]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.etag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `ignore_warnings` after provisioning.\nIf true, allow backwards incompatible changes."]
    pub fn ignore_warnings(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ignore_warnings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `instance` after provisioning.\nThe name of the instance to create the schema bundle within."]
    pub fn instance(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe unique name of the requested schema bundle. Values are of the form 'projects/<project>/instances/<instance>/tables/<table>/schemaBundles/<schemaBundleId>'."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `schema_bundle_id` after provisioning.\nThe unique name of the schema bundle in the form '[_a-zA-Z0-9][-_.a-zA-Z0-9]*'."]
    pub fn schema_bundle_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.schema_bundle_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `table` after provisioning.\nThe name of the table to create the schema bundle within."]
    pub fn table(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.table", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `proto_schema` after provisioning.\n"]
    pub fn proto_schema(&self) -> ListRef<BigtableSchemaBundleProtoSchemaElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.proto_schema", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BigtableSchemaBundleTimeoutsElRef {
        BigtableSchemaBundleTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for BigtableSchemaBundle {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for BigtableSchemaBundle {}
impl ToListMappable for BigtableSchemaBundle {
    type O = ListRef<BigtableSchemaBundleRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for BigtableSchemaBundle_ {
    fn extract_resource_type(&self) -> String {
        "google_bigtable_schema_bundle".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildBigtableSchemaBundle {
    pub tf_id: String,
    #[doc = "The unique name of the schema bundle in the form '[_a-zA-Z0-9][-_.a-zA-Z0-9]*'."]
    pub schema_bundle_id: PrimField<String>,
}
impl BuildBigtableSchemaBundle {
    pub fn build(self, stack: &mut Stack) -> BigtableSchemaBundle {
        let out = BigtableSchemaBundle(Rc::new(BigtableSchemaBundle_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(BigtableSchemaBundleData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                ignore_warnings: core::default::Default::default(),
                instance: core::default::Default::default(),
                project: core::default::Default::default(),
                schema_bundle_id: self.schema_bundle_id,
                table: core::default::Default::default(),
                proto_schema: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct BigtableSchemaBundleRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigtableSchemaBundleRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl BigtableSchemaBundleRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\netag is used for optimistic concurrency control as a way to help prevent simultaneous\nupdates of a schema bundle from overwriting each other. This may be sent on update and delete\nrequests to ensure the client has an update-to-date value before proceeding. The server returns\nan ABORTED error on a mismatched etag."]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.etag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `ignore_warnings` after provisioning.\nIf true, allow backwards incompatible changes."]
    pub fn ignore_warnings(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ignore_warnings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `instance` after provisioning.\nThe name of the instance to create the schema bundle within."]
    pub fn instance(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe unique name of the requested schema bundle. Values are of the form 'projects/<project>/instances/<instance>/tables/<table>/schemaBundles/<schemaBundleId>'."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `schema_bundle_id` after provisioning.\nThe unique name of the schema bundle in the form '[_a-zA-Z0-9][-_.a-zA-Z0-9]*'."]
    pub fn schema_bundle_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.schema_bundle_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `table` after provisioning.\nThe name of the table to create the schema bundle within."]
    pub fn table(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.table", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `proto_schema` after provisioning.\n"]
    pub fn proto_schema(&self) -> ListRef<BigtableSchemaBundleProtoSchemaElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.proto_schema", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BigtableSchemaBundleTimeoutsElRef {
        BigtableSchemaBundleTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct BigtableSchemaBundleProtoSchemaEl {
    proto_descriptors: PrimField<String>,
}
impl BigtableSchemaBundleProtoSchemaEl {}
impl ToListMappable for BigtableSchemaBundleProtoSchemaEl {
    type O = BlockAssignable<BigtableSchemaBundleProtoSchemaEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBigtableSchemaBundleProtoSchemaEl {
    #[doc = "Base64 encoded content of the file."]
    pub proto_descriptors: PrimField<String>,
}
impl BuildBigtableSchemaBundleProtoSchemaEl {
    pub fn build(self) -> BigtableSchemaBundleProtoSchemaEl {
        BigtableSchemaBundleProtoSchemaEl {
            proto_descriptors: self.proto_descriptors,
        }
    }
}
pub struct BigtableSchemaBundleProtoSchemaElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigtableSchemaBundleProtoSchemaElRef {
    fn new(shared: StackShared, base: String) -> BigtableSchemaBundleProtoSchemaElRef {
        BigtableSchemaBundleProtoSchemaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BigtableSchemaBundleProtoSchemaElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `proto_descriptors` after provisioning.\nBase64 encoded content of the file."]
    pub fn proto_descriptors(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.proto_descriptors", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct BigtableSchemaBundleTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl BigtableSchemaBundleTimeoutsEl {
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
impl ToListMappable for BigtableSchemaBundleTimeoutsEl {
    type O = BlockAssignable<BigtableSchemaBundleTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBigtableSchemaBundleTimeoutsEl {}
impl BuildBigtableSchemaBundleTimeoutsEl {
    pub fn build(self) -> BigtableSchemaBundleTimeoutsEl {
        BigtableSchemaBundleTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct BigtableSchemaBundleTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigtableSchemaBundleTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> BigtableSchemaBundleTimeoutsElRef {
        BigtableSchemaBundleTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BigtableSchemaBundleTimeoutsElRef {
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
#[derive(Serialize, Default)]
struct BigtableSchemaBundleDynamic {
    proto_schema: Option<DynamicBlock<BigtableSchemaBundleProtoSchemaEl>>,
}
