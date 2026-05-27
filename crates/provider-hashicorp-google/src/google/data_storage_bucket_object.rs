use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataStorageBucketObjectData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}
struct DataStorageBucketObject_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataStorageBucketObjectData>,
}
#[derive(Clone)]
pub struct DataStorageBucketObject(Rc<DataStorageBucketObject_>);
impl DataStorageBucketObject {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(&self, provider: &ProviderGoogle) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }
    #[doc = "Set the field `bucket`.\nThe name of the containing bucket."]
    pub fn set_bucket(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().bucket = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\nThe name of the object. If you're interpolating the name of this object, see output_name instead."]
    pub fn set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name = Some(v.into());
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
    #[doc = "Get a reference to the value of field `contexts` after provisioning.\nContexts attached to an object, in key-value pairs."]
    pub fn contexts(&self) -> ListRef<DataStorageBucketObjectContextsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.contexts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `crc32c` after provisioning.\nBase 64 CRC32 hash of the uploaded data."]
    pub fn crc32c(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.crc32c", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `customer_encryption` after provisioning.\nEncryption key; encoded using base64."]
    pub fn customer_encryption(&self) -> ListRef<DataStorageBucketObjectCustomerEncryptionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.customer_encryption", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `retention` after provisioning.\nObject level retention configuration."]
    pub fn retention(&self) -> ListRef<DataStorageBucketObjectRetentionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.retention", self.extract_ref()),
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
}
impl Referable for DataStorageBucketObject {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataStorageBucketObject {}
impl ToListMappable for DataStorageBucketObject {
    type O = ListRef<DataStorageBucketObjectRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataStorageBucketObject_ {
    fn extract_datasource_type(&self) -> String {
        "google_storage_bucket_object".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataStorageBucketObject {
    pub tf_id: String,
}
impl BuildDataStorageBucketObject {
    pub fn build(self, stack: &mut Stack) -> DataStorageBucketObject {
        let out = DataStorageBucketObject(Rc::new(DataStorageBucketObject_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataStorageBucketObjectData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                bucket: core::default::Default::default(),
                id: core::default::Default::default(),
                name: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataStorageBucketObjectRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataStorageBucketObjectRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataStorageBucketObjectRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
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
    #[doc = "Get a reference to the value of field `contexts` after provisioning.\nContexts attached to an object, in key-value pairs."]
    pub fn contexts(&self) -> ListRef<DataStorageBucketObjectContextsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.contexts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `crc32c` after provisioning.\nBase 64 CRC32 hash of the uploaded data."]
    pub fn crc32c(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.crc32c", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `customer_encryption` after provisioning.\nEncryption key; encoded using base64."]
    pub fn customer_encryption(&self) -> ListRef<DataStorageBucketObjectCustomerEncryptionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.customer_encryption", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `retention` after provisioning.\nObject level retention configuration."]
    pub fn retention(&self) -> ListRef<DataStorageBucketObjectRetentionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.retention", self.extract_ref()),
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
}
#[derive(Serialize)]
pub struct DataStorageBucketObjectContextsElCustomEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl DataStorageBucketObjectContextsElCustomEl {
    #[doc = "Set the field `create_time`.\n"]
    pub fn set_create_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create_time = Some(v.into());
        self
    }
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }
    #[doc = "Set the field `update_time`.\n"]
    pub fn set_update_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update_time = Some(v.into());
        self
    }
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable for DataStorageBucketObjectContextsElCustomEl {
    type O = BlockAssignable<DataStorageBucketObjectContextsElCustomEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataStorageBucketObjectContextsElCustomEl {}
impl BuildDataStorageBucketObjectContextsElCustomEl {
    pub fn build(self) -> DataStorageBucketObjectContextsElCustomEl {
        DataStorageBucketObjectContextsElCustomEl {
            create_time: core::default::Default::default(),
            key: core::default::Default::default(),
            update_time: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}
pub struct DataStorageBucketObjectContextsElCustomElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataStorageBucketObjectContextsElCustomElRef {
    fn new(shared: StackShared, base: String) -> DataStorageBucketObjectContextsElCustomElRef {
        DataStorageBucketObjectContextsElCustomElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataStorageBucketObjectContextsElCustomElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.base))
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n"]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct DataStorageBucketObjectContextsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    custom: Option<ListField<DataStorageBucketObjectContextsElCustomEl>>,
}
impl DataStorageBucketObjectContextsEl {
    #[doc = "Set the field `custom`.\n"]
    pub fn set_custom(
        mut self,
        v: impl Into<ListField<DataStorageBucketObjectContextsElCustomEl>>,
    ) -> Self {
        self.custom = Some(v.into());
        self
    }
}
impl ToListMappable for DataStorageBucketObjectContextsEl {
    type O = BlockAssignable<DataStorageBucketObjectContextsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataStorageBucketObjectContextsEl {}
impl BuildDataStorageBucketObjectContextsEl {
    pub fn build(self) -> DataStorageBucketObjectContextsEl {
        DataStorageBucketObjectContextsEl {
            custom: core::default::Default::default(),
        }
    }
}
pub struct DataStorageBucketObjectContextsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataStorageBucketObjectContextsElRef {
    fn new(shared: StackShared, base: String) -> DataStorageBucketObjectContextsElRef {
        DataStorageBucketObjectContextsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataStorageBucketObjectContextsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `custom` after provisioning.\n"]
    pub fn custom(&self) -> ListRef<DataStorageBucketObjectContextsElCustomElRef> {
        ListRef::new(self.shared().clone(), format!("{}.custom", self.base))
    }
}
#[derive(Serialize)]
pub struct DataStorageBucketObjectCustomerEncryptionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_algorithm: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_key: Option<PrimField<String>>,
}
impl DataStorageBucketObjectCustomerEncryptionEl {
    #[doc = "Set the field `encryption_algorithm`.\n"]
    pub fn set_encryption_algorithm(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.encryption_algorithm = Some(v.into());
        self
    }
    #[doc = "Set the field `encryption_key`.\n"]
    pub fn set_encryption_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.encryption_key = Some(v.into());
        self
    }
}
impl ToListMappable for DataStorageBucketObjectCustomerEncryptionEl {
    type O = BlockAssignable<DataStorageBucketObjectCustomerEncryptionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataStorageBucketObjectCustomerEncryptionEl {}
impl BuildDataStorageBucketObjectCustomerEncryptionEl {
    pub fn build(self) -> DataStorageBucketObjectCustomerEncryptionEl {
        DataStorageBucketObjectCustomerEncryptionEl {
            encryption_algorithm: core::default::Default::default(),
            encryption_key: core::default::Default::default(),
        }
    }
}
pub struct DataStorageBucketObjectCustomerEncryptionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataStorageBucketObjectCustomerEncryptionElRef {
    fn new(shared: StackShared, base: String) -> DataStorageBucketObjectCustomerEncryptionElRef {
        DataStorageBucketObjectCustomerEncryptionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataStorageBucketObjectCustomerEncryptionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `encryption_algorithm` after provisioning.\n"]
    pub fn encryption_algorithm(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.encryption_algorithm", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `encryption_key` after provisioning.\n"]
    pub fn encryption_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.encryption_key", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataStorageBucketObjectRetentionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retain_until_time: Option<PrimField<String>>,
}
impl DataStorageBucketObjectRetentionEl {
    #[doc = "Set the field `mode`.\n"]
    pub fn set_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mode = Some(v.into());
        self
    }
    #[doc = "Set the field `retain_until_time`.\n"]
    pub fn set_retain_until_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.retain_until_time = Some(v.into());
        self
    }
}
impl ToListMappable for DataStorageBucketObjectRetentionEl {
    type O = BlockAssignable<DataStorageBucketObjectRetentionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataStorageBucketObjectRetentionEl {}
impl BuildDataStorageBucketObjectRetentionEl {
    pub fn build(self) -> DataStorageBucketObjectRetentionEl {
        DataStorageBucketObjectRetentionEl {
            mode: core::default::Default::default(),
            retain_until_time: core::default::Default::default(),
        }
    }
}
pub struct DataStorageBucketObjectRetentionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataStorageBucketObjectRetentionElRef {
    fn new(shared: StackShared, base: String) -> DataStorageBucketObjectRetentionElRef {
        DataStorageBucketObjectRetentionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataStorageBucketObjectRetentionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
    #[doc = "Get a reference to the value of field `retain_until_time` after provisioning.\n"]
    pub fn retain_until_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.retain_until_time", self.base),
        )
    }
}
