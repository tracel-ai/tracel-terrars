use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct StorageBucketObjectData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    bucket: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_control: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content_disposition: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content_encoding: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content_language: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    detect_md5hash: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    event_based_hold: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    force_empty_content_type: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<RecField<PrimField<String>>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_md5hash: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_class: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temporary_hold: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    contexts: Option<Vec<StorageBucketObjectContextsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_encryption: Option<Vec<StorageBucketObjectCustomerEncryptionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retention: Option<Vec<StorageBucketObjectRetentionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<StorageBucketObjectTimeoutsEl>,
    dynamic: StorageBucketObjectDynamic,
}
struct StorageBucketObject_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<StorageBucketObjectData>,
}
#[derive(Clone)]
pub struct StorageBucketObject(Rc<StorageBucketObject_>);
impl StorageBucketObject {
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
    #[doc = "Set the field `cache_control`.\nCache-Control directive to specify caching behavior of object data. If omitted and object is accessible to all anonymous users, the default will be public, max-age=3600"]
    pub fn set_cache_control(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().cache_control = Some(v.into());
        self
    }
    #[doc = "Set the field `content`.\nData as string to be uploaded. Must be defined if source is not. Note: The content field is marked as sensitive. To view the raw contents of the object, please define an output."]
    pub fn set_content(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().content = Some(v.into());
        self
    }
    #[doc = "Set the field `content_disposition`.\nContent-Disposition of the object data."]
    pub fn set_content_disposition(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().content_disposition = Some(v.into());
        self
    }
    #[doc = "Set the field `content_encoding`.\nContent-Encoding of the object data."]
    pub fn set_content_encoding(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().content_encoding = Some(v.into());
        self
    }
    #[doc = "Set the field `content_language`.\nContent-Language of the object data."]
    pub fn set_content_language(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().content_language = Some(v.into());
        self
    }
    #[doc = "Set the field `content_type`.\nContent-Type of the object data. Defaults to \"application/octet-stream\" or \"text/plain; charset=utf-8\"."]
    pub fn set_content_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().content_type = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `detect_md5hash`.\n"]
    pub fn set_detect_md5hash(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().detect_md5hash = Some(v.into());
        self
    }
    #[doc = "Set the field `event_based_hold`.\nWhether an object is under event-based hold. Event-based hold is a way to retain objects until an event occurs, which is signified by the hold's release (i.e. this value is set to false). After being released (set to false), such objects will be subject to bucket-level retention (if any)."]
    pub fn set_event_based_hold(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().event_based_hold = Some(v.into());
        self
    }
    #[doc = "Set the field `force_empty_content_type`.\nFlag to set empty Content-Type."]
    pub fn set_force_empty_content_type(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().force_empty_content_type = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `kms_key_name`.\nResource name of the Cloud KMS key that will be used to encrypt the object. Overrides the object metadata's kmsKeyName value, if any."]
    pub fn set_kms_key_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().kms_key_name = Some(v.into());
        self
    }
    #[doc = "Set the field `metadata`.\nUser-provided metadata, in key/value pairs."]
    pub fn set_metadata(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().metadata = Some(v.into());
        self
    }
    #[doc = "Set the field `source`.\nA path to the data you want to upload. Must be defined if content is not."]
    pub fn set_source(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().source = Some(v.into());
        self
    }
    #[doc = "Set the field `source_md5hash`.\nUser-provided md5hash, Base 64 MD5 hash of the object data."]
    pub fn set_source_md5hash(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().source_md5hash = Some(v.into());
        self
    }
    #[doc = "Set the field `storage_class`.\nThe StorageClass of the new bucket object. Supported values include: MULTI_REGIONAL, REGIONAL, NEARLINE, COLDLINE, ARCHIVE. If not provided, this defaults to the bucket's default storage class or to a standard class."]
    pub fn set_storage_class(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().storage_class = Some(v.into());
        self
    }
    #[doc = "Set the field `temporary_hold`.\nWhether an object is under temporary hold. While this flag is set to true, the object is protected against deletion and overwrites."]
    pub fn set_temporary_hold(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().temporary_hold = Some(v.into());
        self
    }
    #[doc = "Set the field `contexts`.\n"]
    pub fn set_contexts(
        self,
        v: impl Into<BlockAssignable<StorageBucketObjectContextsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().contexts = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.contexts = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `customer_encryption`.\n"]
    pub fn set_customer_encryption(
        self,
        v: impl Into<BlockAssignable<StorageBucketObjectCustomerEncryptionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().customer_encryption = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.customer_encryption = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `retention`.\n"]
    pub fn set_retention(
        self,
        v: impl Into<BlockAssignable<StorageBucketObjectRetentionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().retention = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.retention = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<StorageBucketObjectTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `bucket` after provisioning.\nThe name of the containing bucket."]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bucket", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cache_control` after provisioning.\nCache-Control directive to specify caching behavior of object data. If omitted and object is accessible to all anonymous users, the default will be public, max-age=3600"]
    pub fn cache_control(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cache_control", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `content` after provisioning.\nData as string to be uploaded. Must be defined if source is not. Note: The content field is marked as sensitive. To view the raw contents of the object, please define an output."]
    pub fn content(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.content", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `content_disposition` after provisioning.\nContent-Disposition of the object data."]
    pub fn content_disposition(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.content_disposition", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `content_encoding` after provisioning.\nContent-Encoding of the object data."]
    pub fn content_encoding(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.content_encoding", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `content_language` after provisioning.\nContent-Language of the object data."]
    pub fn content_language(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.content_language", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `content_type` after provisioning.\nContent-Type of the object data. Defaults to \"application/octet-stream\" or \"text/plain; charset=utf-8\"."]
    pub fn content_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.content_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `crc32c` after provisioning.\nBase 64 CRC32 hash of the uploaded data."]
    pub fn crc32c(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.crc32c", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `detect_md5hash` after provisioning.\n"]
    pub fn detect_md5hash(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.detect_md5hash", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `event_based_hold` after provisioning.\nWhether an object is under event-based hold. Event-based hold is a way to retain objects until an event occurs, which is signified by the hold's release (i.e. this value is set to false). After being released (set to false), such objects will be subject to bucket-level retention (if any)."]
    pub fn event_based_hold(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.event_based_hold", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `force_empty_content_type` after provisioning.\nFlag to set empty Content-Type."]
    pub fn force_empty_content_type(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.force_empty_content_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `generation` after provisioning.\nThe content generation of this object. Used for object versioning and soft delete."]
    pub fn generation(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.generation", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `kms_key_name` after provisioning.\nResource name of the Cloud KMS key that will be used to encrypt the object. Overrides the object metadata's kmsKeyName value, if any."]
    pub fn kms_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `md5hash` after provisioning.\nBase 64 MD5 hash of the uploaded data."]
    pub fn md5hash(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.md5hash", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `md5hexhash` after provisioning.\nHex value of md5hash"]
    pub fn md5hexhash(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.md5hexhash", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `media_link` after provisioning.\nA url reference to download this object."]
    pub fn media_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.media_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `metadata` after provisioning.\nUser-provided metadata, in key/value pairs."]
    pub fn metadata(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.metadata", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the object. If you're interpolating the name of this object, see output_name instead."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `output_name` after provisioning.\nThe name of the object. Use this field in interpolations with google_storage_object_acl to recreate google_storage_object_acl resources when your google_storage_bucket_object is recreated."]
    pub fn output_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.output_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\nA url reference to this object."]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source` after provisioning.\nA path to the data you want to upload. Must be defined if content is not."]
    pub fn source(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_md5hash` after provisioning.\nUser-provided md5hash, Base 64 MD5 hash of the object data."]
    pub fn source_md5hash(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_md5hash", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `storage_class` after provisioning.\nThe StorageClass of the new bucket object. Supported values include: MULTI_REGIONAL, REGIONAL, NEARLINE, COLDLINE, ARCHIVE. If not provided, this defaults to the bucket's default storage class or to a standard class."]
    pub fn storage_class(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.storage_class", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `temporary_hold` after provisioning.\nWhether an object is under temporary hold. While this flag is set to true, the object is protected against deletion and overwrites."]
    pub fn temporary_hold(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.temporary_hold", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `contexts` after provisioning.\n"]
    pub fn contexts(&self) -> ListRef<StorageBucketObjectContextsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.contexts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `customer_encryption` after provisioning.\n"]
    pub fn customer_encryption(&self) -> ListRef<StorageBucketObjectCustomerEncryptionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.customer_encryption", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `retention` after provisioning.\n"]
    pub fn retention(&self) -> ListRef<StorageBucketObjectRetentionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.retention", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> StorageBucketObjectTimeoutsElRef {
        StorageBucketObjectTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for StorageBucketObject {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for StorageBucketObject {}
impl ToListMappable for StorageBucketObject {
    type O = ListRef<StorageBucketObjectRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for StorageBucketObject_ {
    fn extract_resource_type(&self) -> String {
        "google_storage_bucket_object".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildStorageBucketObject {
    pub tf_id: String,
    #[doc = "The name of the containing bucket."]
    pub bucket: PrimField<String>,
    #[doc = "The name of the object. If you're interpolating the name of this object, see output_name instead."]
    pub name: PrimField<String>,
}
impl BuildStorageBucketObject {
    pub fn build(self, stack: &mut Stack) -> StorageBucketObject {
        let out = StorageBucketObject(Rc::new(StorageBucketObject_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(StorageBucketObjectData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                bucket: self.bucket,
                cache_control: core::default::Default::default(),
                content: core::default::Default::default(),
                content_disposition: core::default::Default::default(),
                content_encoding: core::default::Default::default(),
                content_language: core::default::Default::default(),
                content_type: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                detect_md5hash: core::default::Default::default(),
                event_based_hold: core::default::Default::default(),
                force_empty_content_type: core::default::Default::default(),
                id: core::default::Default::default(),
                kms_key_name: core::default::Default::default(),
                metadata: core::default::Default::default(),
                name: self.name,
                source: core::default::Default::default(),
                source_md5hash: core::default::Default::default(),
                storage_class: core::default::Default::default(),
                temporary_hold: core::default::Default::default(),
                contexts: core::default::Default::default(),
                customer_encryption: core::default::Default::default(),
                retention: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct StorageBucketObjectRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageBucketObjectRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl StorageBucketObjectRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bucket` after provisioning.\nThe name of the containing bucket."]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bucket", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cache_control` after provisioning.\nCache-Control directive to specify caching behavior of object data. If omitted and object is accessible to all anonymous users, the default will be public, max-age=3600"]
    pub fn cache_control(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cache_control", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `content` after provisioning.\nData as string to be uploaded. Must be defined if source is not. Note: The content field is marked as sensitive. To view the raw contents of the object, please define an output."]
    pub fn content(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.content", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `content_disposition` after provisioning.\nContent-Disposition of the object data."]
    pub fn content_disposition(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.content_disposition", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `content_encoding` after provisioning.\nContent-Encoding of the object data."]
    pub fn content_encoding(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.content_encoding", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `content_language` after provisioning.\nContent-Language of the object data."]
    pub fn content_language(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.content_language", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `content_type` after provisioning.\nContent-Type of the object data. Defaults to \"application/octet-stream\" or \"text/plain; charset=utf-8\"."]
    pub fn content_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.content_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `crc32c` after provisioning.\nBase 64 CRC32 hash of the uploaded data."]
    pub fn crc32c(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.crc32c", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `detect_md5hash` after provisioning.\n"]
    pub fn detect_md5hash(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.detect_md5hash", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `event_based_hold` after provisioning.\nWhether an object is under event-based hold. Event-based hold is a way to retain objects until an event occurs, which is signified by the hold's release (i.e. this value is set to false). After being released (set to false), such objects will be subject to bucket-level retention (if any)."]
    pub fn event_based_hold(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.event_based_hold", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `force_empty_content_type` after provisioning.\nFlag to set empty Content-Type."]
    pub fn force_empty_content_type(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.force_empty_content_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `generation` after provisioning.\nThe content generation of this object. Used for object versioning and soft delete."]
    pub fn generation(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.generation", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `kms_key_name` after provisioning.\nResource name of the Cloud KMS key that will be used to encrypt the object. Overrides the object metadata's kmsKeyName value, if any."]
    pub fn kms_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `md5hash` after provisioning.\nBase 64 MD5 hash of the uploaded data."]
    pub fn md5hash(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.md5hash", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `md5hexhash` after provisioning.\nHex value of md5hash"]
    pub fn md5hexhash(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.md5hexhash", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `media_link` after provisioning.\nA url reference to download this object."]
    pub fn media_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.media_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `metadata` after provisioning.\nUser-provided metadata, in key/value pairs."]
    pub fn metadata(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.metadata", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the object. If you're interpolating the name of this object, see output_name instead."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `output_name` after provisioning.\nThe name of the object. Use this field in interpolations with google_storage_object_acl to recreate google_storage_object_acl resources when your google_storage_bucket_object is recreated."]
    pub fn output_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.output_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\nA url reference to this object."]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source` after provisioning.\nA path to the data you want to upload. Must be defined if content is not."]
    pub fn source(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_md5hash` after provisioning.\nUser-provided md5hash, Base 64 MD5 hash of the object data."]
    pub fn source_md5hash(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_md5hash", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `storage_class` after provisioning.\nThe StorageClass of the new bucket object. Supported values include: MULTI_REGIONAL, REGIONAL, NEARLINE, COLDLINE, ARCHIVE. If not provided, this defaults to the bucket's default storage class or to a standard class."]
    pub fn storage_class(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.storage_class", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `temporary_hold` after provisioning.\nWhether an object is under temporary hold. While this flag is set to true, the object is protected against deletion and overwrites."]
    pub fn temporary_hold(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.temporary_hold", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `contexts` after provisioning.\n"]
    pub fn contexts(&self) -> ListRef<StorageBucketObjectContextsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.contexts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `customer_encryption` after provisioning.\n"]
    pub fn customer_encryption(&self) -> ListRef<StorageBucketObjectCustomerEncryptionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.customer_encryption", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `retention` after provisioning.\n"]
    pub fn retention(&self) -> ListRef<StorageBucketObjectRetentionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.retention", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> StorageBucketObjectTimeoutsElRef {
        StorageBucketObjectTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct StorageBucketObjectContextsElCustomEl {
    key: PrimField<String>,
    value: PrimField<String>,
}
impl StorageBucketObjectContextsElCustomEl {}
impl ToListMappable for StorageBucketObjectContextsElCustomEl {
    type O = BlockAssignable<StorageBucketObjectContextsElCustomEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageBucketObjectContextsElCustomEl {
    #[doc = "An individual object context. Context keys and their corresponding values must start with an alphanumeric character."]
    pub key: PrimField<String>,
    #[doc = "The value associated with this context. This field holds the primary information for the given context key."]
    pub value: PrimField<String>,
}
impl BuildStorageBucketObjectContextsElCustomEl {
    pub fn build(self) -> StorageBucketObjectContextsElCustomEl {
        StorageBucketObjectContextsElCustomEl {
            key: self.key,
            value: self.value,
        }
    }
}
pub struct StorageBucketObjectContextsElCustomElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageBucketObjectContextsElCustomElRef {
    fn new(shared: StackShared, base: String) -> StorageBucketObjectContextsElCustomElRef {
        StorageBucketObjectContextsElCustomElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageBucketObjectContextsElCustomElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe time when context was first added to the storage#object in RFC 3339 format."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.base))
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\nAn individual object context. Context keys and their corresponding values must start with an alphanumeric character."]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe time when context was last updated in RFC 3339 format."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\nThe value associated with this context. This field holds the primary information for the given context key."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize, Default)]
struct StorageBucketObjectContextsElDynamic {
    custom: Option<DynamicBlock<StorageBucketObjectContextsElCustomEl>>,
}
#[derive(Serialize)]
pub struct StorageBucketObjectContextsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    custom: Option<Vec<StorageBucketObjectContextsElCustomEl>>,
    dynamic: StorageBucketObjectContextsElDynamic,
}
impl StorageBucketObjectContextsEl {
    #[doc = "Set the field `custom`.\n"]
    pub fn set_custom(
        mut self,
        v: impl Into<BlockAssignable<StorageBucketObjectContextsElCustomEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.custom = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.custom = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for StorageBucketObjectContextsEl {
    type O = BlockAssignable<StorageBucketObjectContextsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageBucketObjectContextsEl {}
impl BuildStorageBucketObjectContextsEl {
    pub fn build(self) -> StorageBucketObjectContextsEl {
        StorageBucketObjectContextsEl {
            custom: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct StorageBucketObjectContextsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageBucketObjectContextsElRef {
    fn new(shared: StackShared, base: String) -> StorageBucketObjectContextsElRef {
        StorageBucketObjectContextsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageBucketObjectContextsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `custom` after provisioning.\n"]
    pub fn custom(&self) -> ListRef<StorageBucketObjectContextsElCustomElRef> {
        ListRef::new(self.shared().clone(), format!("{}.custom", self.base))
    }
}
#[derive(Serialize)]
pub struct StorageBucketObjectCustomerEncryptionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_algorithm: Option<PrimField<String>>,
    encryption_key: PrimField<String>,
}
impl StorageBucketObjectCustomerEncryptionEl {
    #[doc = "Set the field `encryption_algorithm`.\nThe encryption algorithm. Default: AES256"]
    pub fn set_encryption_algorithm(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.encryption_algorithm = Some(v.into());
        self
    }
}
impl ToListMappable for StorageBucketObjectCustomerEncryptionEl {
    type O = BlockAssignable<StorageBucketObjectCustomerEncryptionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageBucketObjectCustomerEncryptionEl {
    #[doc = "Base64 encoded customer supplied encryption key."]
    pub encryption_key: PrimField<String>,
}
impl BuildStorageBucketObjectCustomerEncryptionEl {
    pub fn build(self) -> StorageBucketObjectCustomerEncryptionEl {
        StorageBucketObjectCustomerEncryptionEl {
            encryption_algorithm: core::default::Default::default(),
            encryption_key: self.encryption_key,
        }
    }
}
pub struct StorageBucketObjectCustomerEncryptionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageBucketObjectCustomerEncryptionElRef {
    fn new(shared: StackShared, base: String) -> StorageBucketObjectCustomerEncryptionElRef {
        StorageBucketObjectCustomerEncryptionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageBucketObjectCustomerEncryptionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `encryption_algorithm` after provisioning.\nThe encryption algorithm. Default: AES256"]
    pub fn encryption_algorithm(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.encryption_algorithm", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `encryption_key` after provisioning.\nBase64 encoded customer supplied encryption key."]
    pub fn encryption_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.encryption_key", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct StorageBucketObjectRetentionEl {
    mode: PrimField<String>,
    retain_until_time: PrimField<String>,
}
impl StorageBucketObjectRetentionEl {}
impl ToListMappable for StorageBucketObjectRetentionEl {
    type O = BlockAssignable<StorageBucketObjectRetentionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageBucketObjectRetentionEl {
    #[doc = "The object retention mode. Supported values include: \"Unlocked\", \"Locked\"."]
    pub mode: PrimField<String>,
    #[doc = "Time in RFC 3339 (e.g. 2030-01-01T02:03:04Z) until which object retention protects this object."]
    pub retain_until_time: PrimField<String>,
}
impl BuildStorageBucketObjectRetentionEl {
    pub fn build(self) -> StorageBucketObjectRetentionEl {
        StorageBucketObjectRetentionEl {
            mode: self.mode,
            retain_until_time: self.retain_until_time,
        }
    }
}
pub struct StorageBucketObjectRetentionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageBucketObjectRetentionElRef {
    fn new(shared: StackShared, base: String) -> StorageBucketObjectRetentionElRef {
        StorageBucketObjectRetentionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageBucketObjectRetentionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `mode` after provisioning.\nThe object retention mode. Supported values include: \"Unlocked\", \"Locked\"."]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
    #[doc = "Get a reference to the value of field `retain_until_time` after provisioning.\nTime in RFC 3339 (e.g. 2030-01-01T02:03:04Z) until which object retention protects this object."]
    pub fn retain_until_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.retain_until_time", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct StorageBucketObjectTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl StorageBucketObjectTimeoutsEl {
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
impl ToListMappable for StorageBucketObjectTimeoutsEl {
    type O = BlockAssignable<StorageBucketObjectTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageBucketObjectTimeoutsEl {}
impl BuildStorageBucketObjectTimeoutsEl {
    pub fn build(self) -> StorageBucketObjectTimeoutsEl {
        StorageBucketObjectTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct StorageBucketObjectTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageBucketObjectTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> StorageBucketObjectTimeoutsElRef {
        StorageBucketObjectTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageBucketObjectTimeoutsElRef {
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
struct StorageBucketObjectDynamic {
    contexts: Option<DynamicBlock<StorageBucketObjectContextsEl>>,
    customer_encryption: Option<DynamicBlock<StorageBucketObjectCustomerEncryptionEl>>,
    retention: Option<DynamicBlock<StorageBucketObjectRetentionEl>>,
}
