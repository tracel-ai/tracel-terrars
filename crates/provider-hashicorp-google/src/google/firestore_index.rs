use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct FirestoreIndexData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    api_scope: Option<PrimField<String>>,
    collection: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    database: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    density: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    multikey: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    query_scope: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    skip_wait: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unique: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fields: Option<Vec<FirestoreIndexFieldsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<FirestoreIndexTimeoutsEl>,
    dynamic: FirestoreIndexDynamic,
}
struct FirestoreIndex_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<FirestoreIndexData>,
}
#[derive(Clone)]
pub struct FirestoreIndex(Rc<FirestoreIndex_>);
impl FirestoreIndex {
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
    #[doc = "Set the field `api_scope`.\nThe API scope at which a query is run. Default value: \"ANY_API\" Possible values: [\"ANY_API\", \"DATASTORE_MODE_API\", \"MONGODB_COMPATIBLE_API\"]"]
    pub fn set_api_scope(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().api_scope = Some(v.into());
        self
    }
    #[doc = "Set the field `database`.\nThe Firestore database id. Defaults to '\"(default)\"'."]
    pub fn set_database(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().database = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `density`.\nThe density configuration for this index. Possible values: [\"SPARSE_ALL\", \"SPARSE_ANY\", \"DENSE\"]"]
    pub fn set_density(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().density = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `multikey`.\nOptional. Whether the index is multikey. By default, the index is not multikey. For non-multikey indexes, none of the paths in the index definition reach or traverse an array, except via an explicit array index. For multikey indexes, at most one of the paths in the index definition reach or traverse an array, except via an explicit array index. Violations will result in errors. Note this field only applies to indexes with MONGODB_COMPATIBLE_API ApiScope."]
    pub fn set_multikey(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().multikey = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `query_scope`.\nThe scope at which a query is run. Default value: \"COLLECTION\" Possible values: [\"COLLECTION\", \"COLLECTION_GROUP\", \"COLLECTION_RECURSIVE\"]"]
    pub fn set_query_scope(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().query_scope = Some(v.into());
        self
    }
    #[doc = "Set the field `skip_wait`.\nWhether to skip waiting for the index to be created."]
    pub fn set_skip_wait(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().skip_wait = Some(v.into());
        self
    }
    #[doc = "Set the field `unique`.\nWhether it is an unique index. Unique index ensures all values for the indexed field(s) are unique across documents."]
    pub fn set_unique(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().unique = Some(v.into());
        self
    }
    #[doc = "Set the field `fields`.\n"]
    pub fn set_fields(self, v: impl Into<BlockAssignable<FirestoreIndexFieldsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().fields = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.fields = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<FirestoreIndexTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `api_scope` after provisioning.\nThe API scope at which a query is run. Default value: \"ANY_API\" Possible values: [\"ANY_API\", \"DATASTORE_MODE_API\", \"MONGODB_COMPATIBLE_API\"]"]
    pub fn api_scope(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.api_scope", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `collection` after provisioning.\nThe collection being indexed."]
    pub fn collection(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.collection", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `database` after provisioning.\nThe Firestore database id. Defaults to '\"(default)\"'."]
    pub fn database(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.database", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `density` after provisioning.\nThe density configuration for this index. Possible values: [\"SPARSE_ALL\", \"SPARSE_ANY\", \"DENSE\"]"]
    pub fn density(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.density", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `multikey` after provisioning.\nOptional. Whether the index is multikey. By default, the index is not multikey. For non-multikey indexes, none of the paths in the index definition reach or traverse an array, except via an explicit array index. For multikey indexes, at most one of the paths in the index definition reach or traverse an array, except via an explicit array index. Violations will result in errors. Note this field only applies to indexes with MONGODB_COMPATIBLE_API ApiScope."]
    pub fn multikey(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.multikey", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nA server defined name for this index. Format:\n'projects/{{project}}/databases/{{database}}/collectionGroups/{{collection}}/indexes/{{server_generated_id}}'"]
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
    #[doc = "Get a reference to the value of field `query_scope` after provisioning.\nThe scope at which a query is run. Default value: \"COLLECTION\" Possible values: [\"COLLECTION\", \"COLLECTION_GROUP\", \"COLLECTION_RECURSIVE\"]"]
    pub fn query_scope(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.query_scope", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `skip_wait` after provisioning.\nWhether to skip waiting for the index to be created."]
    pub fn skip_wait(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.skip_wait", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `unique` after provisioning.\nWhether it is an unique index. Unique index ensures all values for the indexed field(s) are unique across documents."]
    pub fn unique(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.unique", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `fields` after provisioning.\n"]
    pub fn fields(&self) -> ListRef<FirestoreIndexFieldsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.fields", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FirestoreIndexTimeoutsElRef {
        FirestoreIndexTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for FirestoreIndex {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for FirestoreIndex {}
impl ToListMappable for FirestoreIndex {
    type O = ListRef<FirestoreIndexRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for FirestoreIndex_ {
    fn extract_resource_type(&self) -> String {
        "google_firestore_index".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildFirestoreIndex {
    pub tf_id: String,
    #[doc = "The collection being indexed."]
    pub collection: PrimField<String>,
}
impl BuildFirestoreIndex {
    pub fn build(self, stack: &mut Stack) -> FirestoreIndex {
        let out = FirestoreIndex(Rc::new(FirestoreIndex_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(FirestoreIndexData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                api_scope: core::default::Default::default(),
                collection: self.collection,
                database: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                density: core::default::Default::default(),
                id: core::default::Default::default(),
                multikey: core::default::Default::default(),
                project: core::default::Default::default(),
                query_scope: core::default::Default::default(),
                skip_wait: core::default::Default::default(),
                unique: core::default::Default::default(),
                fields: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct FirestoreIndexRef {
    shared: StackShared,
    base: String,
}
impl Ref for FirestoreIndexRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl FirestoreIndexRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `api_scope` after provisioning.\nThe API scope at which a query is run. Default value: \"ANY_API\" Possible values: [\"ANY_API\", \"DATASTORE_MODE_API\", \"MONGODB_COMPATIBLE_API\"]"]
    pub fn api_scope(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.api_scope", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `collection` after provisioning.\nThe collection being indexed."]
    pub fn collection(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.collection", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `database` after provisioning.\nThe Firestore database id. Defaults to '\"(default)\"'."]
    pub fn database(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.database", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `density` after provisioning.\nThe density configuration for this index. Possible values: [\"SPARSE_ALL\", \"SPARSE_ANY\", \"DENSE\"]"]
    pub fn density(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.density", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `multikey` after provisioning.\nOptional. Whether the index is multikey. By default, the index is not multikey. For non-multikey indexes, none of the paths in the index definition reach or traverse an array, except via an explicit array index. For multikey indexes, at most one of the paths in the index definition reach or traverse an array, except via an explicit array index. Violations will result in errors. Note this field only applies to indexes with MONGODB_COMPATIBLE_API ApiScope."]
    pub fn multikey(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.multikey", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nA server defined name for this index. Format:\n'projects/{{project}}/databases/{{database}}/collectionGroups/{{collection}}/indexes/{{server_generated_id}}'"]
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
    #[doc = "Get a reference to the value of field `query_scope` after provisioning.\nThe scope at which a query is run. Default value: \"COLLECTION\" Possible values: [\"COLLECTION\", \"COLLECTION_GROUP\", \"COLLECTION_RECURSIVE\"]"]
    pub fn query_scope(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.query_scope", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `skip_wait` after provisioning.\nWhether to skip waiting for the index to be created."]
    pub fn skip_wait(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.skip_wait", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `unique` after provisioning.\nWhether it is an unique index. Unique index ensures all values for the indexed field(s) are unique across documents."]
    pub fn unique(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.unique", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `fields` after provisioning.\n"]
    pub fn fields(&self) -> ListRef<FirestoreIndexFieldsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.fields", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FirestoreIndexTimeoutsElRef {
        FirestoreIndexTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct FirestoreIndexFieldsElSearchConfigElGeoSpecEl {
    geo_json_indexing_disabled: PrimField<bool>,
}
impl FirestoreIndexFieldsElSearchConfigElGeoSpecEl {}
impl ToListMappable for FirestoreIndexFieldsElSearchConfigElGeoSpecEl {
    type O = BlockAssignable<FirestoreIndexFieldsElSearchConfigElGeoSpecEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFirestoreIndexFieldsElSearchConfigElGeoSpecEl {
    #[doc = "If true, disables GeoJSON indexing for the field. By default, GeoJSON points are indexed.\nFirestore GeoPoints are indexed regardless of the value of this field."]
    pub geo_json_indexing_disabled: PrimField<bool>,
}
impl BuildFirestoreIndexFieldsElSearchConfigElGeoSpecEl {
    pub fn build(self) -> FirestoreIndexFieldsElSearchConfigElGeoSpecEl {
        FirestoreIndexFieldsElSearchConfigElGeoSpecEl {
            geo_json_indexing_disabled: self.geo_json_indexing_disabled,
        }
    }
}
pub struct FirestoreIndexFieldsElSearchConfigElGeoSpecElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FirestoreIndexFieldsElSearchConfigElGeoSpecElRef {
    fn new(shared: StackShared, base: String) -> FirestoreIndexFieldsElSearchConfigElGeoSpecElRef {
        FirestoreIndexFieldsElSearchConfigElGeoSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FirestoreIndexFieldsElSearchConfigElGeoSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `geo_json_indexing_disabled` after provisioning.\nIf true, disables GeoJSON indexing for the field. By default, GeoJSON points are indexed.\nFirestore GeoPoints are indexed regardless of the value of this field."]
    pub fn geo_json_indexing_disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.geo_json_indexing_disabled", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct FirestoreIndexFieldsElSearchConfigElTextSpecElIndexSpecsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    index_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_type: Option<PrimField<String>>,
}
impl FirestoreIndexFieldsElSearchConfigElTextSpecElIndexSpecsEl {
    #[doc = "Set the field `index_type`.\nWays to index the text field value."]
    pub fn set_index_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.index_type = Some(v.into());
        self
    }
    #[doc = "Set the field `match_type`.\nHow to match the text field value."]
    pub fn set_match_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.match_type = Some(v.into());
        self
    }
}
impl ToListMappable for FirestoreIndexFieldsElSearchConfigElTextSpecElIndexSpecsEl {
    type O = BlockAssignable<FirestoreIndexFieldsElSearchConfigElTextSpecElIndexSpecsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFirestoreIndexFieldsElSearchConfigElTextSpecElIndexSpecsEl {}
impl BuildFirestoreIndexFieldsElSearchConfigElTextSpecElIndexSpecsEl {
    pub fn build(self) -> FirestoreIndexFieldsElSearchConfigElTextSpecElIndexSpecsEl {
        FirestoreIndexFieldsElSearchConfigElTextSpecElIndexSpecsEl {
            index_type: core::default::Default::default(),
            match_type: core::default::Default::default(),
        }
    }
}
pub struct FirestoreIndexFieldsElSearchConfigElTextSpecElIndexSpecsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FirestoreIndexFieldsElSearchConfigElTextSpecElIndexSpecsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> FirestoreIndexFieldsElSearchConfigElTextSpecElIndexSpecsElRef {
        FirestoreIndexFieldsElSearchConfigElTextSpecElIndexSpecsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FirestoreIndexFieldsElSearchConfigElTextSpecElIndexSpecsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `index_type` after provisioning.\nWays to index the text field value."]
    pub fn index_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.index_type", self.base))
    }
    #[doc = "Get a reference to the value of field `match_type` after provisioning.\nHow to match the text field value."]
    pub fn match_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.match_type", self.base))
    }
}
#[derive(Serialize, Default)]
struct FirestoreIndexFieldsElSearchConfigElTextSpecElDynamic {
    index_specs: Option<DynamicBlock<FirestoreIndexFieldsElSearchConfigElTextSpecElIndexSpecsEl>>,
}
#[derive(Serialize)]
pub struct FirestoreIndexFieldsElSearchConfigElTextSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    index_specs: Option<Vec<FirestoreIndexFieldsElSearchConfigElTextSpecElIndexSpecsEl>>,
    dynamic: FirestoreIndexFieldsElSearchConfigElTextSpecElDynamic,
}
impl FirestoreIndexFieldsElSearchConfigElTextSpecEl {
    #[doc = "Set the field `index_specs`.\n"]
    pub fn set_index_specs(
        mut self,
        v: impl Into<BlockAssignable<FirestoreIndexFieldsElSearchConfigElTextSpecElIndexSpecsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.index_specs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.index_specs = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for FirestoreIndexFieldsElSearchConfigElTextSpecEl {
    type O = BlockAssignable<FirestoreIndexFieldsElSearchConfigElTextSpecEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFirestoreIndexFieldsElSearchConfigElTextSpecEl {}
impl BuildFirestoreIndexFieldsElSearchConfigElTextSpecEl {
    pub fn build(self) -> FirestoreIndexFieldsElSearchConfigElTextSpecEl {
        FirestoreIndexFieldsElSearchConfigElTextSpecEl {
            index_specs: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct FirestoreIndexFieldsElSearchConfigElTextSpecElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FirestoreIndexFieldsElSearchConfigElTextSpecElRef {
    fn new(shared: StackShared, base: String) -> FirestoreIndexFieldsElSearchConfigElTextSpecElRef {
        FirestoreIndexFieldsElSearchConfigElTextSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FirestoreIndexFieldsElSearchConfigElTextSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `index_specs` after provisioning.\n"]
    pub fn index_specs(
        &self,
    ) -> ListRef<FirestoreIndexFieldsElSearchConfigElTextSpecElIndexSpecsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.index_specs", self.base))
    }
}
#[derive(Serialize, Default)]
struct FirestoreIndexFieldsElSearchConfigElDynamic {
    geo_spec: Option<DynamicBlock<FirestoreIndexFieldsElSearchConfigElGeoSpecEl>>,
    text_spec: Option<DynamicBlock<FirestoreIndexFieldsElSearchConfigElTextSpecEl>>,
}
#[derive(Serialize)]
pub struct FirestoreIndexFieldsElSearchConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    geo_spec: Option<Vec<FirestoreIndexFieldsElSearchConfigElGeoSpecEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text_spec: Option<Vec<FirestoreIndexFieldsElSearchConfigElTextSpecEl>>,
    dynamic: FirestoreIndexFieldsElSearchConfigElDynamic,
}
impl FirestoreIndexFieldsElSearchConfigEl {
    #[doc = "Set the field `geo_spec`.\n"]
    pub fn set_geo_spec(
        mut self,
        v: impl Into<BlockAssignable<FirestoreIndexFieldsElSearchConfigElGeoSpecEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.geo_spec = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.geo_spec = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `text_spec`.\n"]
    pub fn set_text_spec(
        mut self,
        v: impl Into<BlockAssignable<FirestoreIndexFieldsElSearchConfigElTextSpecEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.text_spec = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.text_spec = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for FirestoreIndexFieldsElSearchConfigEl {
    type O = BlockAssignable<FirestoreIndexFieldsElSearchConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFirestoreIndexFieldsElSearchConfigEl {}
impl BuildFirestoreIndexFieldsElSearchConfigEl {
    pub fn build(self) -> FirestoreIndexFieldsElSearchConfigEl {
        FirestoreIndexFieldsElSearchConfigEl {
            geo_spec: core::default::Default::default(),
            text_spec: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct FirestoreIndexFieldsElSearchConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FirestoreIndexFieldsElSearchConfigElRef {
    fn new(shared: StackShared, base: String) -> FirestoreIndexFieldsElSearchConfigElRef {
        FirestoreIndexFieldsElSearchConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FirestoreIndexFieldsElSearchConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `geo_spec` after provisioning.\n"]
    pub fn geo_spec(&self) -> ListRef<FirestoreIndexFieldsElSearchConfigElGeoSpecElRef> {
        ListRef::new(self.shared().clone(), format!("{}.geo_spec", self.base))
    }
    #[doc = "Get a reference to the value of field `text_spec` after provisioning.\n"]
    pub fn text_spec(&self) -> ListRef<FirestoreIndexFieldsElSearchConfigElTextSpecElRef> {
        ListRef::new(self.shared().clone(), format!("{}.text_spec", self.base))
    }
}
#[derive(Serialize)]
pub struct FirestoreIndexFieldsElVectorConfigElFlatEl {}
impl FirestoreIndexFieldsElVectorConfigElFlatEl {}
impl ToListMappable for FirestoreIndexFieldsElVectorConfigElFlatEl {
    type O = BlockAssignable<FirestoreIndexFieldsElVectorConfigElFlatEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFirestoreIndexFieldsElVectorConfigElFlatEl {}
impl BuildFirestoreIndexFieldsElVectorConfigElFlatEl {
    pub fn build(self) -> FirestoreIndexFieldsElVectorConfigElFlatEl {
        FirestoreIndexFieldsElVectorConfigElFlatEl {}
    }
}
pub struct FirestoreIndexFieldsElVectorConfigElFlatElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FirestoreIndexFieldsElVectorConfigElFlatElRef {
    fn new(shared: StackShared, base: String) -> FirestoreIndexFieldsElVectorConfigElFlatElRef {
        FirestoreIndexFieldsElVectorConfigElFlatElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FirestoreIndexFieldsElVectorConfigElFlatElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize, Default)]
struct FirestoreIndexFieldsElVectorConfigElDynamic {
    flat: Option<DynamicBlock<FirestoreIndexFieldsElVectorConfigElFlatEl>>,
}
#[derive(Serialize)]
pub struct FirestoreIndexFieldsElVectorConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dimension: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    flat: Option<Vec<FirestoreIndexFieldsElVectorConfigElFlatEl>>,
    dynamic: FirestoreIndexFieldsElVectorConfigElDynamic,
}
impl FirestoreIndexFieldsElVectorConfigEl {
    #[doc = "Set the field `dimension`.\nThe resulting index will only include vectors of this dimension, and can be used for vector search\nwith the same dimension."]
    pub fn set_dimension(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.dimension = Some(v.into());
        self
    }
    #[doc = "Set the field `flat`.\n"]
    pub fn set_flat(
        mut self,
        v: impl Into<BlockAssignable<FirestoreIndexFieldsElVectorConfigElFlatEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.flat = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.flat = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for FirestoreIndexFieldsElVectorConfigEl {
    type O = BlockAssignable<FirestoreIndexFieldsElVectorConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFirestoreIndexFieldsElVectorConfigEl {}
impl BuildFirestoreIndexFieldsElVectorConfigEl {
    pub fn build(self) -> FirestoreIndexFieldsElVectorConfigEl {
        FirestoreIndexFieldsElVectorConfigEl {
            dimension: core::default::Default::default(),
            flat: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct FirestoreIndexFieldsElVectorConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FirestoreIndexFieldsElVectorConfigElRef {
    fn new(shared: StackShared, base: String) -> FirestoreIndexFieldsElVectorConfigElRef {
        FirestoreIndexFieldsElVectorConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FirestoreIndexFieldsElVectorConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `dimension` after provisioning.\nThe resulting index will only include vectors of this dimension, and can be used for vector search\nwith the same dimension."]
    pub fn dimension(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.dimension", self.base))
    }
    #[doc = "Get a reference to the value of field `flat` after provisioning.\n"]
    pub fn flat(&self) -> ListRef<FirestoreIndexFieldsElVectorConfigElFlatElRef> {
        ListRef::new(self.shared().clone(), format!("{}.flat", self.base))
    }
}
#[derive(Serialize, Default)]
struct FirestoreIndexFieldsElDynamic {
    search_config: Option<DynamicBlock<FirestoreIndexFieldsElSearchConfigEl>>,
    vector_config: Option<DynamicBlock<FirestoreIndexFieldsElVectorConfigEl>>,
}
#[derive(Serialize)]
pub struct FirestoreIndexFieldsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    array_config: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    field_path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    order: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    search_config: Option<Vec<FirestoreIndexFieldsElSearchConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vector_config: Option<Vec<FirestoreIndexFieldsElVectorConfigEl>>,
    dynamic: FirestoreIndexFieldsElDynamic,
}
impl FirestoreIndexFieldsEl {
    #[doc = "Set the field `array_config`.\nIndicates that this field supports operations on arrayValues. Only one of 'order', 'arrayConfig', 'searchConfig' and\n'vectorConfig' can be specified. Possible values: [\"CONTAINS\"]"]
    pub fn set_array_config(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.array_config = Some(v.into());
        self
    }
    #[doc = "Set the field `field_path`.\nName of the field."]
    pub fn set_field_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.field_path = Some(v.into());
        self
    }
    #[doc = "Set the field `order`.\nIndicates that this field supports ordering by the specified order or comparing using =, <, <=, >, >=.\nOnly one of 'order', 'arrayConfig', 'searchConfig' and 'vectorConfig' can be specified. Possible values: [\"ASCENDING\", \"DESCENDING\"]"]
    pub fn set_order(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.order = Some(v.into());
        self
    }
    #[doc = "Set the field `search_config`.\n"]
    pub fn set_search_config(
        mut self,
        v: impl Into<BlockAssignable<FirestoreIndexFieldsElSearchConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.search_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.search_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `vector_config`.\n"]
    pub fn set_vector_config(
        mut self,
        v: impl Into<BlockAssignable<FirestoreIndexFieldsElVectorConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.vector_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.vector_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for FirestoreIndexFieldsEl {
    type O = BlockAssignable<FirestoreIndexFieldsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFirestoreIndexFieldsEl {}
impl BuildFirestoreIndexFieldsEl {
    pub fn build(self) -> FirestoreIndexFieldsEl {
        FirestoreIndexFieldsEl {
            array_config: core::default::Default::default(),
            field_path: core::default::Default::default(),
            order: core::default::Default::default(),
            search_config: core::default::Default::default(),
            vector_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct FirestoreIndexFieldsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FirestoreIndexFieldsElRef {
    fn new(shared: StackShared, base: String) -> FirestoreIndexFieldsElRef {
        FirestoreIndexFieldsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FirestoreIndexFieldsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `array_config` after provisioning.\nIndicates that this field supports operations on arrayValues. Only one of 'order', 'arrayConfig', 'searchConfig' and\n'vectorConfig' can be specified. Possible values: [\"CONTAINS\"]"]
    pub fn array_config(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.array_config", self.base))
    }
    #[doc = "Get a reference to the value of field `field_path` after provisioning.\nName of the field."]
    pub fn field_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.field_path", self.base))
    }
    #[doc = "Get a reference to the value of field `order` after provisioning.\nIndicates that this field supports ordering by the specified order or comparing using =, <, <=, >, >=.\nOnly one of 'order', 'arrayConfig', 'searchConfig' and 'vectorConfig' can be specified. Possible values: [\"ASCENDING\", \"DESCENDING\"]"]
    pub fn order(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.order", self.base))
    }
    #[doc = "Get a reference to the value of field `search_config` after provisioning.\n"]
    pub fn search_config(&self) -> ListRef<FirestoreIndexFieldsElSearchConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.search_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `vector_config` after provisioning.\n"]
    pub fn vector_config(&self) -> ListRef<FirestoreIndexFieldsElVectorConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vector_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct FirestoreIndexTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl FirestoreIndexTimeoutsEl {
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
impl ToListMappable for FirestoreIndexTimeoutsEl {
    type O = BlockAssignable<FirestoreIndexTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFirestoreIndexTimeoutsEl {}
impl BuildFirestoreIndexTimeoutsEl {
    pub fn build(self) -> FirestoreIndexTimeoutsEl {
        FirestoreIndexTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct FirestoreIndexTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FirestoreIndexTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> FirestoreIndexTimeoutsElRef {
        FirestoreIndexTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FirestoreIndexTimeoutsElRef {
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
struct FirestoreIndexDynamic {
    fields: Option<DynamicBlock<FirestoreIndexFieldsEl>>,
}
