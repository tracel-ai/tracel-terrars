use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct StorageAnywhereCacheData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    admission_policy: Option<PrimField<String>>,
    bucket: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ingest_on_write: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ttl: Option<PrimField<String>>,
    zone: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<StorageAnywhereCacheTimeoutsEl>,
}
struct StorageAnywhereCache_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<StorageAnywhereCacheData>,
}
#[derive(Clone)]
pub struct StorageAnywhereCache(Rc<StorageAnywhereCache_>);
impl StorageAnywhereCache {
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
    #[doc = "Set the field `admission_policy`.\nThe cache admission policy dictates whether a block should be inserted upon a cache miss. Default value: \"admit-on-first-miss\" Possible values: [\"admit-on-first-miss\", \"admit-on-second-miss\"]"]
    pub fn set_admission_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().admission_policy = Some(v.into());
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
    #[doc = "Set the field `ingest_on_write`.\nWhether or not the cache ingests data as the data is written to the bucket."]
    pub fn set_ingest_on_write(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().ingest_on_write = Some(v.into());
        self
    }
    #[doc = "Set the field `ttl`.\nThe TTL of all cache entries in whole seconds. e.g., \"7200s\". It defaults to '86400s'"]
    pub fn set_ttl(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().ttl = Some(v.into());
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<StorageAnywhereCacheTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `admission_policy` after provisioning.\nThe cache admission policy dictates whether a block should be inserted upon a cache miss. Default value: \"admit-on-first-miss\" Possible values: [\"admit-on-first-miss\", \"admit-on-second-miss\"]"]
    pub fn admission_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.admission_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `anywhere_cache_id` after provisioning.\nThe ID of the Anywhere cache instance."]
    pub fn anywhere_cache_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.anywhere_cache_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `bucket` after provisioning.\nA reference to Bucket resource"]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bucket", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe creation time of the cache instance in RFC 3339 format."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `ingest_on_write` after provisioning.\nWhether or not the cache ingests data as the data is written to the bucket."]
    pub fn ingest_on_write(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ingest_on_write", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `pending_update` after provisioning.\nTrue if the cache instance has an active Update long-running operation."]
    pub fn pending_update(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pending_update", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe current state of the cache instance."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ttl` after provisioning.\nThe TTL of all cache entries in whole seconds. e.g., \"7200s\". It defaults to '86400s'"]
    pub fn ttl(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ttl", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe modification time of the cache instance metadata in RFC 3339 format."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `zone` after provisioning.\nThe zone in which the cache instance needs to be created. For example, 'us-central1-a.'"]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.zone", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> StorageAnywhereCacheTimeoutsElRef {
        StorageAnywhereCacheTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for StorageAnywhereCache {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for StorageAnywhereCache {}
impl ToListMappable for StorageAnywhereCache {
    type O = ListRef<StorageAnywhereCacheRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for StorageAnywhereCache_ {
    fn extract_resource_type(&self) -> String {
        "google_storage_anywhere_cache".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildStorageAnywhereCache {
    pub tf_id: String,
    #[doc = "A reference to Bucket resource"]
    pub bucket: PrimField<String>,
    #[doc = "The zone in which the cache instance needs to be created. For example, 'us-central1-a.'"]
    pub zone: PrimField<String>,
}
impl BuildStorageAnywhereCache {
    pub fn build(self, stack: &mut Stack) -> StorageAnywhereCache {
        let out = StorageAnywhereCache(Rc::new(StorageAnywhereCache_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(StorageAnywhereCacheData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                admission_policy: core::default::Default::default(),
                bucket: self.bucket,
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                ingest_on_write: core::default::Default::default(),
                ttl: core::default::Default::default(),
                zone: self.zone,
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct StorageAnywhereCacheRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageAnywhereCacheRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl StorageAnywhereCacheRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `admission_policy` after provisioning.\nThe cache admission policy dictates whether a block should be inserted upon a cache miss. Default value: \"admit-on-first-miss\" Possible values: [\"admit-on-first-miss\", \"admit-on-second-miss\"]"]
    pub fn admission_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.admission_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `anywhere_cache_id` after provisioning.\nThe ID of the Anywhere cache instance."]
    pub fn anywhere_cache_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.anywhere_cache_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `bucket` after provisioning.\nA reference to Bucket resource"]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bucket", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe creation time of the cache instance in RFC 3339 format."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `ingest_on_write` after provisioning.\nWhether or not the cache ingests data as the data is written to the bucket."]
    pub fn ingest_on_write(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ingest_on_write", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `pending_update` after provisioning.\nTrue if the cache instance has an active Update long-running operation."]
    pub fn pending_update(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pending_update", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe current state of the cache instance."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ttl` after provisioning.\nThe TTL of all cache entries in whole seconds. e.g., \"7200s\". It defaults to '86400s'"]
    pub fn ttl(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ttl", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe modification time of the cache instance metadata in RFC 3339 format."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `zone` after provisioning.\nThe zone in which the cache instance needs to be created. For example, 'us-central1-a.'"]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.zone", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> StorageAnywhereCacheTimeoutsElRef {
        StorageAnywhereCacheTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct StorageAnywhereCacheTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl StorageAnywhereCacheTimeoutsEl {
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
impl ToListMappable for StorageAnywhereCacheTimeoutsEl {
    type O = BlockAssignable<StorageAnywhereCacheTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageAnywhereCacheTimeoutsEl {}
impl BuildStorageAnywhereCacheTimeoutsEl {
    pub fn build(self) -> StorageAnywhereCacheTimeoutsEl {
        StorageAnywhereCacheTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct StorageAnywhereCacheTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageAnywhereCacheTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> StorageAnywhereCacheTimeoutsElRef {
        StorageAnywhereCacheTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageAnywhereCacheTimeoutsElRef {
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
