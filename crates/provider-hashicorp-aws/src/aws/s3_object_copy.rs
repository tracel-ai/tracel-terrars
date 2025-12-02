use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct S3ObjectCopyData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    acl: Option<PrimField<String>>,
    bucket: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_key_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_control: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    checksum_algorithm: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content_disposition: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content_encoding: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content_language: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    copy_if_match: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    copy_if_modified_since: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    copy_if_none_match: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    copy_if_unmodified_since: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_algorithm: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_key_md5: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expected_bucket_owner: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expected_source_bucket_owner: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expires: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    force_destroy: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    key: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_encryption_context: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata_directive: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    object_lock_legal_hold_status: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    object_lock_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    object_lock_retain_until_date: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_payer: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    server_side_encryption: Option<PrimField<String>>,
    source: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_customer_algorithm: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_customer_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_customer_key_md5: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_class: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tagging_directive: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    website_redirect: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    grant: Option<Vec<S3ObjectCopyGrantEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    override_provider: Option<Vec<S3ObjectCopyOverrideProviderEl>>,
    dynamic: S3ObjectCopyDynamic,
}
struct S3ObjectCopy_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<S3ObjectCopyData>,
}
#[derive(Clone)]
pub struct S3ObjectCopy(Rc<S3ObjectCopy_>);
impl S3ObjectCopy {
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
    #[doc = "Set the field `acl`.\n"]
    pub fn set_acl(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().acl = Some(v.into());
        self
    }
    #[doc = "Set the field `bucket_key_enabled`.\n"]
    pub fn set_bucket_key_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().bucket_key_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `cache_control`.\n"]
    pub fn set_cache_control(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().cache_control = Some(v.into());
        self
    }
    #[doc = "Set the field `checksum_algorithm`.\n"]
    pub fn set_checksum_algorithm(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().checksum_algorithm = Some(v.into());
        self
    }
    #[doc = "Set the field `content_disposition`.\n"]
    pub fn set_content_disposition(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().content_disposition = Some(v.into());
        self
    }
    #[doc = "Set the field `content_encoding`.\n"]
    pub fn set_content_encoding(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().content_encoding = Some(v.into());
        self
    }
    #[doc = "Set the field `content_language`.\n"]
    pub fn set_content_language(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().content_language = Some(v.into());
        self
    }
    #[doc = "Set the field `content_type`.\n"]
    pub fn set_content_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().content_type = Some(v.into());
        self
    }
    #[doc = "Set the field `copy_if_match`.\n"]
    pub fn set_copy_if_match(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().copy_if_match = Some(v.into());
        self
    }
    #[doc = "Set the field `copy_if_modified_since`.\n"]
    pub fn set_copy_if_modified_since(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().copy_if_modified_since = Some(v.into());
        self
    }
    #[doc = "Set the field `copy_if_none_match`.\n"]
    pub fn set_copy_if_none_match(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().copy_if_none_match = Some(v.into());
        self
    }
    #[doc = "Set the field `copy_if_unmodified_since`.\n"]
    pub fn set_copy_if_unmodified_since(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().copy_if_unmodified_since = Some(v.into());
        self
    }
    #[doc = "Set the field `customer_algorithm`.\n"]
    pub fn set_customer_algorithm(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().customer_algorithm = Some(v.into());
        self
    }
    #[doc = "Set the field `customer_key`.\n"]
    pub fn set_customer_key(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().customer_key = Some(v.into());
        self
    }
    #[doc = "Set the field `customer_key_md5`.\n"]
    pub fn set_customer_key_md5(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().customer_key_md5 = Some(v.into());
        self
    }
    #[doc = "Set the field `expected_bucket_owner`.\n"]
    pub fn set_expected_bucket_owner(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().expected_bucket_owner = Some(v.into());
        self
    }
    #[doc = "Set the field `expected_source_bucket_owner`.\n"]
    pub fn set_expected_source_bucket_owner(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().expected_source_bucket_owner = Some(v.into());
        self
    }
    #[doc = "Set the field `expires`.\n"]
    pub fn set_expires(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().expires = Some(v.into());
        self
    }
    #[doc = "Set the field `force_destroy`.\n"]
    pub fn set_force_destroy(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().force_destroy = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `kms_encryption_context`.\n"]
    pub fn set_kms_encryption_context(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().kms_encryption_context = Some(v.into());
        self
    }
    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().kms_key_id = Some(v.into());
        self
    }
    #[doc = "Set the field `metadata`.\n"]
    pub fn set_metadata(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().metadata = Some(v.into());
        self
    }
    #[doc = "Set the field `metadata_directive`.\n"]
    pub fn set_metadata_directive(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().metadata_directive = Some(v.into());
        self
    }
    #[doc = "Set the field `object_lock_legal_hold_status`.\n"]
    pub fn set_object_lock_legal_hold_status(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().object_lock_legal_hold_status = Some(v.into());
        self
    }
    #[doc = "Set the field `object_lock_mode`.\n"]
    pub fn set_object_lock_mode(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().object_lock_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `object_lock_retain_until_date`.\n"]
    pub fn set_object_lock_retain_until_date(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().object_lock_retain_until_date = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `request_payer`.\n"]
    pub fn set_request_payer(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().request_payer = Some(v.into());
        self
    }
    #[doc = "Set the field `server_side_encryption`.\n"]
    pub fn set_server_side_encryption(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().server_side_encryption = Some(v.into());
        self
    }
    #[doc = "Set the field `source_customer_algorithm`.\n"]
    pub fn set_source_customer_algorithm(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().source_customer_algorithm = Some(v.into());
        self
    }
    #[doc = "Set the field `source_customer_key`.\n"]
    pub fn set_source_customer_key(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().source_customer_key = Some(v.into());
        self
    }
    #[doc = "Set the field `source_customer_key_md5`.\n"]
    pub fn set_source_customer_key_md5(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().source_customer_key_md5 = Some(v.into());
        self
    }
    #[doc = "Set the field `storage_class`.\n"]
    pub fn set_storage_class(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().storage_class = Some(v.into());
        self
    }
    #[doc = "Set the field `tagging_directive`.\n"]
    pub fn set_tagging_directive(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().tagging_directive = Some(v.into());
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
    #[doc = "Set the field `website_redirect`.\n"]
    pub fn set_website_redirect(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().website_redirect = Some(v.into());
        self
    }
    #[doc = "Set the field `grant`.\n"]
    pub fn set_grant(self, v: impl Into<BlockAssignable<S3ObjectCopyGrantEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().grant = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.grant = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `override_provider`.\n"]
    pub fn set_override_provider(
        self,
        v: impl Into<BlockAssignable<S3ObjectCopyOverrideProviderEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().override_provider = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.override_provider = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `acl` after provisioning.\n"]
    pub fn acl(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.acl", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bucket", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `bucket_key_enabled` after provisioning.\n"]
    pub fn bucket_key_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bucket_key_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cache_control` after provisioning.\n"]
    pub fn cache_control(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cache_control", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `checksum_algorithm` after provisioning.\n"]
    pub fn checksum_algorithm(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.checksum_algorithm", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `checksum_crc32` after provisioning.\n"]
    pub fn checksum_crc32(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.checksum_crc32", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `checksum_crc32c` after provisioning.\n"]
    pub fn checksum_crc32c(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.checksum_crc32c", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `checksum_crc64nvme` after provisioning.\n"]
    pub fn checksum_crc64nvme(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.checksum_crc64nvme", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `checksum_sha1` after provisioning.\n"]
    pub fn checksum_sha1(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.checksum_sha1", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `checksum_sha256` after provisioning.\n"]
    pub fn checksum_sha256(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.checksum_sha256", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `content_disposition` after provisioning.\n"]
    pub fn content_disposition(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.content_disposition", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `content_encoding` after provisioning.\n"]
    pub fn content_encoding(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.content_encoding", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `content_language` after provisioning.\n"]
    pub fn content_language(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.content_language", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `content_type` after provisioning.\n"]
    pub fn content_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.content_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `copy_if_match` after provisioning.\n"]
    pub fn copy_if_match(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.copy_if_match", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `copy_if_modified_since` after provisioning.\n"]
    pub fn copy_if_modified_since(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.copy_if_modified_since", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `copy_if_none_match` after provisioning.\n"]
    pub fn copy_if_none_match(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.copy_if_none_match", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `copy_if_unmodified_since` after provisioning.\n"]
    pub fn copy_if_unmodified_since(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.copy_if_unmodified_since", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `customer_algorithm` after provisioning.\n"]
    pub fn customer_algorithm(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.customer_algorithm", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `customer_key` after provisioning.\n"]
    pub fn customer_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.customer_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `customer_key_md5` after provisioning.\n"]
    pub fn customer_key_md5(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.customer_key_md5", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\n"]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.etag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `expected_bucket_owner` after provisioning.\n"]
    pub fn expected_bucket_owner(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.expected_bucket_owner", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `expected_source_bucket_owner` after provisioning.\n"]
    pub fn expected_source_bucket_owner(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.expected_source_bucket_owner", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `expiration` after provisioning.\n"]
    pub fn expiration(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.expiration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `expires` after provisioning.\n"]
    pub fn expires(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.expires", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `force_destroy` after provisioning.\n"]
    pub fn force_destroy(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.force_destroy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `kms_encryption_context` after provisioning.\n"]
    pub fn kms_encryption_context(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_encryption_context", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `last_modified` after provisioning.\n"]
    pub fn last_modified(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_modified", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `metadata` after provisioning.\n"]
    pub fn metadata(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.metadata", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `metadata_directive` after provisioning.\n"]
    pub fn metadata_directive(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.metadata_directive", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `object_lock_legal_hold_status` after provisioning.\n"]
    pub fn object_lock_legal_hold_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.object_lock_legal_hold_status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `object_lock_mode` after provisioning.\n"]
    pub fn object_lock_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.object_lock_mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `object_lock_retain_until_date` after provisioning.\n"]
    pub fn object_lock_retain_until_date(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.object_lock_retain_until_date", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `request_charged` after provisioning.\n"]
    pub fn request_charged(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.request_charged", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `request_payer` after provisioning.\n"]
    pub fn request_payer(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.request_payer", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `server_side_encryption` after provisioning.\n"]
    pub fn server_side_encryption(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.server_side_encryption", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_customer_algorithm` after provisioning.\n"]
    pub fn source_customer_algorithm(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_customer_algorithm", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_customer_key` after provisioning.\n"]
    pub fn source_customer_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_customer_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_customer_key_md5` after provisioning.\n"]
    pub fn source_customer_key_md5(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_customer_key_md5", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_version_id` after provisioning.\n"]
    pub fn source_version_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_version_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `storage_class` after provisioning.\n"]
    pub fn storage_class(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.storage_class", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tagging_directive` after provisioning.\n"]
    pub fn tagging_directive(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.tagging_directive", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `version_id` after provisioning.\n"]
    pub fn version_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `website_redirect` after provisioning.\n"]
    pub fn website_redirect(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.website_redirect", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `override_provider` after provisioning.\n"]
    pub fn override_provider(&self) -> ListRef<S3ObjectCopyOverrideProviderElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.override_provider", self.extract_ref()),
        )
    }
}
impl Referable for S3ObjectCopy {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for S3ObjectCopy {}
impl ToListMappable for S3ObjectCopy {
    type O = ListRef<S3ObjectCopyRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for S3ObjectCopy_ {
    fn extract_resource_type(&self) -> String {
        "aws_s3_object_copy".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildS3ObjectCopy {
    pub tf_id: String,
    #[doc = ""]
    pub bucket: PrimField<String>,
    #[doc = ""]
    pub key: PrimField<String>,
    #[doc = ""]
    pub source: PrimField<String>,
}
impl BuildS3ObjectCopy {
    pub fn build(self, stack: &mut Stack) -> S3ObjectCopy {
        let out = S3ObjectCopy(Rc::new(S3ObjectCopy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(S3ObjectCopyData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                acl: core::default::Default::default(),
                bucket: self.bucket,
                bucket_key_enabled: core::default::Default::default(),
                cache_control: core::default::Default::default(),
                checksum_algorithm: core::default::Default::default(),
                content_disposition: core::default::Default::default(),
                content_encoding: core::default::Default::default(),
                content_language: core::default::Default::default(),
                content_type: core::default::Default::default(),
                copy_if_match: core::default::Default::default(),
                copy_if_modified_since: core::default::Default::default(),
                copy_if_none_match: core::default::Default::default(),
                copy_if_unmodified_since: core::default::Default::default(),
                customer_algorithm: core::default::Default::default(),
                customer_key: core::default::Default::default(),
                customer_key_md5: core::default::Default::default(),
                expected_bucket_owner: core::default::Default::default(),
                expected_source_bucket_owner: core::default::Default::default(),
                expires: core::default::Default::default(),
                force_destroy: core::default::Default::default(),
                id: core::default::Default::default(),
                key: self.key,
                kms_encryption_context: core::default::Default::default(),
                kms_key_id: core::default::Default::default(),
                metadata: core::default::Default::default(),
                metadata_directive: core::default::Default::default(),
                object_lock_legal_hold_status: core::default::Default::default(),
                object_lock_mode: core::default::Default::default(),
                object_lock_retain_until_date: core::default::Default::default(),
                region: core::default::Default::default(),
                request_payer: core::default::Default::default(),
                server_side_encryption: core::default::Default::default(),
                source: self.source,
                source_customer_algorithm: core::default::Default::default(),
                source_customer_key: core::default::Default::default(),
                source_customer_key_md5: core::default::Default::default(),
                storage_class: core::default::Default::default(),
                tagging_directive: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                website_redirect: core::default::Default::default(),
                grant: core::default::Default::default(),
                override_provider: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct S3ObjectCopyRef {
    shared: StackShared,
    base: String,
}
impl Ref for S3ObjectCopyRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl S3ObjectCopyRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `acl` after provisioning.\n"]
    pub fn acl(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.acl", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bucket", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `bucket_key_enabled` after provisioning.\n"]
    pub fn bucket_key_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bucket_key_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cache_control` after provisioning.\n"]
    pub fn cache_control(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cache_control", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `checksum_algorithm` after provisioning.\n"]
    pub fn checksum_algorithm(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.checksum_algorithm", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `checksum_crc32` after provisioning.\n"]
    pub fn checksum_crc32(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.checksum_crc32", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `checksum_crc32c` after provisioning.\n"]
    pub fn checksum_crc32c(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.checksum_crc32c", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `checksum_crc64nvme` after provisioning.\n"]
    pub fn checksum_crc64nvme(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.checksum_crc64nvme", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `checksum_sha1` after provisioning.\n"]
    pub fn checksum_sha1(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.checksum_sha1", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `checksum_sha256` after provisioning.\n"]
    pub fn checksum_sha256(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.checksum_sha256", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `content_disposition` after provisioning.\n"]
    pub fn content_disposition(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.content_disposition", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `content_encoding` after provisioning.\n"]
    pub fn content_encoding(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.content_encoding", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `content_language` after provisioning.\n"]
    pub fn content_language(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.content_language", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `content_type` after provisioning.\n"]
    pub fn content_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.content_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `copy_if_match` after provisioning.\n"]
    pub fn copy_if_match(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.copy_if_match", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `copy_if_modified_since` after provisioning.\n"]
    pub fn copy_if_modified_since(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.copy_if_modified_since", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `copy_if_none_match` after provisioning.\n"]
    pub fn copy_if_none_match(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.copy_if_none_match", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `copy_if_unmodified_since` after provisioning.\n"]
    pub fn copy_if_unmodified_since(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.copy_if_unmodified_since", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `customer_algorithm` after provisioning.\n"]
    pub fn customer_algorithm(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.customer_algorithm", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `customer_key` after provisioning.\n"]
    pub fn customer_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.customer_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `customer_key_md5` after provisioning.\n"]
    pub fn customer_key_md5(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.customer_key_md5", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\n"]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.etag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `expected_bucket_owner` after provisioning.\n"]
    pub fn expected_bucket_owner(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.expected_bucket_owner", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `expected_source_bucket_owner` after provisioning.\n"]
    pub fn expected_source_bucket_owner(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.expected_source_bucket_owner", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `expiration` after provisioning.\n"]
    pub fn expiration(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.expiration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `expires` after provisioning.\n"]
    pub fn expires(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.expires", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `force_destroy` after provisioning.\n"]
    pub fn force_destroy(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.force_destroy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `kms_encryption_context` after provisioning.\n"]
    pub fn kms_encryption_context(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_encryption_context", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `last_modified` after provisioning.\n"]
    pub fn last_modified(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_modified", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `metadata` after provisioning.\n"]
    pub fn metadata(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.metadata", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `metadata_directive` after provisioning.\n"]
    pub fn metadata_directive(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.metadata_directive", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `object_lock_legal_hold_status` after provisioning.\n"]
    pub fn object_lock_legal_hold_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.object_lock_legal_hold_status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `object_lock_mode` after provisioning.\n"]
    pub fn object_lock_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.object_lock_mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `object_lock_retain_until_date` after provisioning.\n"]
    pub fn object_lock_retain_until_date(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.object_lock_retain_until_date", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `request_charged` after provisioning.\n"]
    pub fn request_charged(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.request_charged", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `request_payer` after provisioning.\n"]
    pub fn request_payer(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.request_payer", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `server_side_encryption` after provisioning.\n"]
    pub fn server_side_encryption(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.server_side_encryption", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_customer_algorithm` after provisioning.\n"]
    pub fn source_customer_algorithm(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_customer_algorithm", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_customer_key` after provisioning.\n"]
    pub fn source_customer_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_customer_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_customer_key_md5` after provisioning.\n"]
    pub fn source_customer_key_md5(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_customer_key_md5", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_version_id` after provisioning.\n"]
    pub fn source_version_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_version_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `storage_class` after provisioning.\n"]
    pub fn storage_class(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.storage_class", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tagging_directive` after provisioning.\n"]
    pub fn tagging_directive(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.tagging_directive", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `version_id` after provisioning.\n"]
    pub fn version_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `website_redirect` after provisioning.\n"]
    pub fn website_redirect(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.website_redirect", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `override_provider` after provisioning.\n"]
    pub fn override_provider(&self) -> ListRef<S3ObjectCopyOverrideProviderElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.override_provider", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct S3ObjectCopyGrantEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    permissions: SetField<PrimField<String>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    uri: Option<PrimField<String>>,
}
impl S3ObjectCopyGrantEl {
    #[doc = "Set the field `email`.\n"]
    pub fn set_email(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.email = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }
    #[doc = "Set the field `uri`.\n"]
    pub fn set_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.uri = Some(v.into());
        self
    }
}
impl ToListMappable for S3ObjectCopyGrantEl {
    type O = BlockAssignable<S3ObjectCopyGrantEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildS3ObjectCopyGrantEl {
    #[doc = ""]
    pub permissions: SetField<PrimField<String>>,
    #[doc = ""]
    pub type_: PrimField<String>,
}
impl BuildS3ObjectCopyGrantEl {
    pub fn build(self) -> S3ObjectCopyGrantEl {
        S3ObjectCopyGrantEl {
            email: core::default::Default::default(),
            id: core::default::Default::default(),
            permissions: self.permissions,
            type_: self.type_,
            uri: core::default::Default::default(),
        }
    }
}
pub struct S3ObjectCopyGrantElRef {
    shared: StackShared,
    base: String,
}
impl Ref for S3ObjectCopyGrantElRef {
    fn new(shared: StackShared, base: String) -> S3ObjectCopyGrantElRef {
        S3ObjectCopyGrantElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl S3ObjectCopyGrantElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.email", self.base))
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
    #[doc = "Get a reference to the value of field `permissions` after provisioning.\n"]
    pub fn permissions(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.permissions", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
    #[doc = "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}
#[derive(Serialize)]
pub struct S3ObjectCopyOverrideProviderElDefaultTagsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
}
impl S3ObjectCopyOverrideProviderElDefaultTagsEl {
    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.tags = Some(v.into());
        self
    }
}
impl ToListMappable for S3ObjectCopyOverrideProviderElDefaultTagsEl {
    type O = BlockAssignable<S3ObjectCopyOverrideProviderElDefaultTagsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildS3ObjectCopyOverrideProviderElDefaultTagsEl {}
impl BuildS3ObjectCopyOverrideProviderElDefaultTagsEl {
    pub fn build(self) -> S3ObjectCopyOverrideProviderElDefaultTagsEl {
        S3ObjectCopyOverrideProviderElDefaultTagsEl {
            tags: core::default::Default::default(),
        }
    }
}
pub struct S3ObjectCopyOverrideProviderElDefaultTagsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for S3ObjectCopyOverrideProviderElDefaultTagsElRef {
    fn new(shared: StackShared, base: String) -> S3ObjectCopyOverrideProviderElDefaultTagsElRef {
        S3ObjectCopyOverrideProviderElDefaultTagsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl S3ObjectCopyOverrideProviderElDefaultTagsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}
#[derive(Serialize, Default)]
struct S3ObjectCopyOverrideProviderElDynamic {
    default_tags: Option<DynamicBlock<S3ObjectCopyOverrideProviderElDefaultTagsEl>>,
}
#[derive(Serialize)]
pub struct S3ObjectCopyOverrideProviderEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    default_tags: Option<Vec<S3ObjectCopyOverrideProviderElDefaultTagsEl>>,
    dynamic: S3ObjectCopyOverrideProviderElDynamic,
}
impl S3ObjectCopyOverrideProviderEl {
    #[doc = "Set the field `default_tags`.\n"]
    pub fn set_default_tags(
        mut self,
        v: impl Into<BlockAssignable<S3ObjectCopyOverrideProviderElDefaultTagsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.default_tags = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.default_tags = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for S3ObjectCopyOverrideProviderEl {
    type O = BlockAssignable<S3ObjectCopyOverrideProviderEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildS3ObjectCopyOverrideProviderEl {}
impl BuildS3ObjectCopyOverrideProviderEl {
    pub fn build(self) -> S3ObjectCopyOverrideProviderEl {
        S3ObjectCopyOverrideProviderEl {
            default_tags: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct S3ObjectCopyOverrideProviderElRef {
    shared: StackShared,
    base: String,
}
impl Ref for S3ObjectCopyOverrideProviderElRef {
    fn new(shared: StackShared, base: String) -> S3ObjectCopyOverrideProviderElRef {
        S3ObjectCopyOverrideProviderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl S3ObjectCopyOverrideProviderElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `default_tags` after provisioning.\n"]
    pub fn default_tags(&self) -> ListRef<S3ObjectCopyOverrideProviderElDefaultTagsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.default_tags", self.base))
    }
}
#[derive(Serialize, Default)]
struct S3ObjectCopyDynamic {
    grant: Option<DynamicBlock<S3ObjectCopyGrantEl>>,
    override_provider: Option<DynamicBlock<S3ObjectCopyOverrideProviderEl>>,
}
