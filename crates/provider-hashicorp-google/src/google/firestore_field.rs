use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct FirestoreFieldData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    collection: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    database: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    field: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    index_config: Option<Vec<FirestoreFieldIndexConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<FirestoreFieldTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ttl_config: Option<Vec<FirestoreFieldTtlConfigEl>>,
    dynamic: FirestoreFieldDynamic,
}
struct FirestoreField_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<FirestoreFieldData>,
}
#[derive(Clone)]
pub struct FirestoreField(Rc<FirestoreField_>);
impl FirestoreField {
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
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `index_config`.\n"]
    pub fn set_index_config(
        self,
        v: impl Into<BlockAssignable<FirestoreFieldIndexConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().index_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.index_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<FirestoreFieldTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Set the field `ttl_config`.\n"]
    pub fn set_ttl_config(self, v: impl Into<BlockAssignable<FirestoreFieldTtlConfigEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ttl_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ttl_config = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `collection` after provisioning.\nThe id of the collection group to configure."]
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
    #[doc = "Get a reference to the value of field `field` after provisioning.\nThe id of the field to configure."]
    pub fn field(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.field", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of this field. Format:\n'projects/{{project}}/databases/{{database}}/collectionGroups/{{collection}}/fields/{{field}}'"]
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
    #[doc = "Get a reference to the value of field `index_config` after provisioning.\n"]
    pub fn index_config(&self) -> ListRef<FirestoreFieldIndexConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.index_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FirestoreFieldTimeoutsElRef {
        FirestoreFieldTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ttl_config` after provisioning.\n"]
    pub fn ttl_config(&self) -> ListRef<FirestoreFieldTtlConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ttl_config", self.extract_ref()),
        )
    }
}
impl Referable for FirestoreField {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for FirestoreField {}
impl ToListMappable for FirestoreField {
    type O = ListRef<FirestoreFieldRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for FirestoreField_ {
    fn extract_resource_type(&self) -> String {
        "google_firestore_field".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildFirestoreField {
    pub tf_id: String,
    #[doc = "The id of the collection group to configure."]
    pub collection: PrimField<String>,
    #[doc = "The id of the field to configure."]
    pub field: PrimField<String>,
}
impl BuildFirestoreField {
    pub fn build(self, stack: &mut Stack) -> FirestoreField {
        let out = FirestoreField(Rc::new(FirestoreField_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(FirestoreFieldData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                collection: self.collection,
                database: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                field: self.field,
                id: core::default::Default::default(),
                project: core::default::Default::default(),
                index_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                ttl_config: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct FirestoreFieldRef {
    shared: StackShared,
    base: String,
}
impl Ref for FirestoreFieldRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl FirestoreFieldRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `collection` after provisioning.\nThe id of the collection group to configure."]
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
    #[doc = "Get a reference to the value of field `field` after provisioning.\nThe id of the field to configure."]
    pub fn field(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.field", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of this field. Format:\n'projects/{{project}}/databases/{{database}}/collectionGroups/{{collection}}/fields/{{field}}'"]
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
    #[doc = "Get a reference to the value of field `index_config` after provisioning.\n"]
    pub fn index_config(&self) -> ListRef<FirestoreFieldIndexConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.index_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FirestoreFieldTimeoutsElRef {
        FirestoreFieldTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ttl_config` after provisioning.\n"]
    pub fn ttl_config(&self) -> ListRef<FirestoreFieldTtlConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ttl_config", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct FirestoreFieldIndexConfigElIndexesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    array_config: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    order: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    query_scope: Option<PrimField<String>>,
}
impl FirestoreFieldIndexConfigElIndexesEl {
    #[doc = "Set the field `array_config`.\nIndicates that this field supports operations on arrayValues. Only one of 'order' and 'arrayConfig' can\nbe specified. Possible values: [\"CONTAINS\"]"]
    pub fn set_array_config(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.array_config = Some(v.into());
        self
    }
    #[doc = "Set the field `order`.\nIndicates that this field supports ordering by the specified order or comparing using =, <, <=, >, >=, !=.\nOnly one of 'order' and 'arrayConfig' can be specified. Possible values: [\"ASCENDING\", \"DESCENDING\"]"]
    pub fn set_order(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.order = Some(v.into());
        self
    }
    #[doc = "Set the field `query_scope`.\nThe scope at which a query is run. Collection scoped queries require you specify\nthe collection at query time. Collection group scope allows queries across all\ncollections with the same id. Default value: \"COLLECTION\" Possible values: [\"COLLECTION\", \"COLLECTION_GROUP\"]"]
    pub fn set_query_scope(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.query_scope = Some(v.into());
        self
    }
}
impl ToListMappable for FirestoreFieldIndexConfigElIndexesEl {
    type O = BlockAssignable<FirestoreFieldIndexConfigElIndexesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFirestoreFieldIndexConfigElIndexesEl {}
impl BuildFirestoreFieldIndexConfigElIndexesEl {
    pub fn build(self) -> FirestoreFieldIndexConfigElIndexesEl {
        FirestoreFieldIndexConfigElIndexesEl {
            array_config: core::default::Default::default(),
            order: core::default::Default::default(),
            query_scope: core::default::Default::default(),
        }
    }
}
pub struct FirestoreFieldIndexConfigElIndexesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FirestoreFieldIndexConfigElIndexesElRef {
    fn new(shared: StackShared, base: String) -> FirestoreFieldIndexConfigElIndexesElRef {
        FirestoreFieldIndexConfigElIndexesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FirestoreFieldIndexConfigElIndexesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `array_config` after provisioning.\nIndicates that this field supports operations on arrayValues. Only one of 'order' and 'arrayConfig' can\nbe specified. Possible values: [\"CONTAINS\"]"]
    pub fn array_config(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.array_config", self.base))
    }
    #[doc = "Get a reference to the value of field `order` after provisioning.\nIndicates that this field supports ordering by the specified order or comparing using =, <, <=, >, >=, !=.\nOnly one of 'order' and 'arrayConfig' can be specified. Possible values: [\"ASCENDING\", \"DESCENDING\"]"]
    pub fn order(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.order", self.base))
    }
    #[doc = "Get a reference to the value of field `query_scope` after provisioning.\nThe scope at which a query is run. Collection scoped queries require you specify\nthe collection at query time. Collection group scope allows queries across all\ncollections with the same id. Default value: \"COLLECTION\" Possible values: [\"COLLECTION\", \"COLLECTION_GROUP\"]"]
    pub fn query_scope(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.query_scope", self.base))
    }
}
#[derive(Serialize, Default)]
struct FirestoreFieldIndexConfigElDynamic {
    indexes: Option<DynamicBlock<FirestoreFieldIndexConfigElIndexesEl>>,
}
#[derive(Serialize)]
pub struct FirestoreFieldIndexConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    indexes: Option<Vec<FirestoreFieldIndexConfigElIndexesEl>>,
    dynamic: FirestoreFieldIndexConfigElDynamic,
}
impl FirestoreFieldIndexConfigEl {
    #[doc = "Set the field `indexes`.\n"]
    pub fn set_indexes(
        mut self,
        v: impl Into<BlockAssignable<FirestoreFieldIndexConfigElIndexesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.indexes = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.indexes = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for FirestoreFieldIndexConfigEl {
    type O = BlockAssignable<FirestoreFieldIndexConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFirestoreFieldIndexConfigEl {}
impl BuildFirestoreFieldIndexConfigEl {
    pub fn build(self) -> FirestoreFieldIndexConfigEl {
        FirestoreFieldIndexConfigEl {
            indexes: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct FirestoreFieldIndexConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FirestoreFieldIndexConfigElRef {
    fn new(shared: StackShared, base: String) -> FirestoreFieldIndexConfigElRef {
        FirestoreFieldIndexConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FirestoreFieldIndexConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct FirestoreFieldTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl FirestoreFieldTimeoutsEl {
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
impl ToListMappable for FirestoreFieldTimeoutsEl {
    type O = BlockAssignable<FirestoreFieldTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFirestoreFieldTimeoutsEl {}
impl BuildFirestoreFieldTimeoutsEl {
    pub fn build(self) -> FirestoreFieldTimeoutsEl {
        FirestoreFieldTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct FirestoreFieldTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FirestoreFieldTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> FirestoreFieldTimeoutsElRef {
        FirestoreFieldTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FirestoreFieldTimeoutsElRef {
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
pub struct FirestoreFieldTtlConfigEl {}
impl FirestoreFieldTtlConfigEl {}
impl ToListMappable for FirestoreFieldTtlConfigEl {
    type O = BlockAssignable<FirestoreFieldTtlConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFirestoreFieldTtlConfigEl {}
impl BuildFirestoreFieldTtlConfigEl {
    pub fn build(self) -> FirestoreFieldTtlConfigEl {
        FirestoreFieldTtlConfigEl {}
    }
}
pub struct FirestoreFieldTtlConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FirestoreFieldTtlConfigElRef {
    fn new(shared: StackShared, base: String) -> FirestoreFieldTtlConfigElRef {
        FirestoreFieldTtlConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FirestoreFieldTtlConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe state of TTL (time-to-live) configuration for documents that have this Field set."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}
#[derive(Serialize, Default)]
struct FirestoreFieldDynamic {
    index_config: Option<DynamicBlock<FirestoreFieldIndexConfigEl>>,
    ttl_config: Option<DynamicBlock<FirestoreFieldTtlConfigEl>>,
}
