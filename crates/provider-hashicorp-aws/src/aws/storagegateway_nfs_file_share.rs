use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct StoragegatewayNfsFileShareData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    audit_destination_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_region: Option<PrimField<String>>,
    client_list: SetField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_storage_class: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file_share_name: Option<PrimField<String>>,
    gateway_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    guess_mime_type_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_encrypted: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_arn: Option<PrimField<String>>,
    location_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notification_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    object_acl: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    read_only: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    requester_pays: Option<PrimField<bool>>,
    role_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    squash: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_endpoint_dns_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_attributes: Option<Vec<StoragegatewayNfsFileShareCacheAttributesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nfs_file_share_defaults: Option<Vec<StoragegatewayNfsFileShareNfsFileShareDefaultsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<StoragegatewayNfsFileShareTimeoutsEl>,
    dynamic: StoragegatewayNfsFileShareDynamic,
}

struct StoragegatewayNfsFileShare_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<StoragegatewayNfsFileShareData>,
}

#[derive(Clone)]
pub struct StoragegatewayNfsFileShare(Rc<StoragegatewayNfsFileShare_>);

impl StoragegatewayNfsFileShare {
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

    #[doc = "Set the field `audit_destination_arn`.\n"]
    pub fn set_audit_destination_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().audit_destination_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `bucket_region`.\n"]
    pub fn set_bucket_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().bucket_region = Some(v.into());
        self
    }

    #[doc = "Set the field `default_storage_class`.\n"]
    pub fn set_default_storage_class(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().default_storage_class = Some(v.into());
        self
    }

    #[doc = "Set the field `file_share_name`.\n"]
    pub fn set_file_share_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().file_share_name = Some(v.into());
        self
    }

    #[doc = "Set the field `guess_mime_type_enabled`.\n"]
    pub fn set_guess_mime_type_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().guess_mime_type_enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `kms_encrypted`.\n"]
    pub fn set_kms_encrypted(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().kms_encrypted = Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_arn`.\n"]
    pub fn set_kms_key_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().kms_key_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `notification_policy`.\n"]
    pub fn set_notification_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().notification_policy = Some(v.into());
        self
    }

    #[doc = "Set the field `object_acl`.\n"]
    pub fn set_object_acl(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().object_acl = Some(v.into());
        self
    }

    #[doc = "Set the field `read_only`.\n"]
    pub fn set_read_only(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().read_only = Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `requester_pays`.\n"]
    pub fn set_requester_pays(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().requester_pays = Some(v.into());
        self
    }

    #[doc = "Set the field `squash`.\n"]
    pub fn set_squash(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().squash = Some(v.into());
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

    #[doc = "Set the field `vpc_endpoint_dns_name`.\n"]
    pub fn set_vpc_endpoint_dns_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().vpc_endpoint_dns_name = Some(v.into());
        self
    }

    #[doc = "Set the field `cache_attributes`.\n"]
    pub fn set_cache_attributes(
        self,
        v: impl Into<BlockAssignable<StoragegatewayNfsFileShareCacheAttributesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cache_attributes = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cache_attributes = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `nfs_file_share_defaults`.\n"]
    pub fn set_nfs_file_share_defaults(
        self,
        v: impl Into<BlockAssignable<StoragegatewayNfsFileShareNfsFileShareDefaultsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().nfs_file_share_defaults = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.nfs_file_share_defaults = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<StoragegatewayNfsFileShareTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `audit_destination_arn` after provisioning.\n"]
    pub fn audit_destination_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.audit_destination_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `bucket_region` after provisioning.\n"]
    pub fn bucket_region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bucket_region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `client_list` after provisioning.\n"]
    pub fn client_list(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.client_list", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `default_storage_class` after provisioning.\n"]
    pub fn default_storage_class(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_storage_class", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `file_share_name` after provisioning.\n"]
    pub fn file_share_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.file_share_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `fileshare_id` after provisioning.\n"]
    pub fn fileshare_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.fileshare_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `gateway_arn` after provisioning.\n"]
    pub fn gateway_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gateway_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `guess_mime_type_enabled` after provisioning.\n"]
    pub fn guess_mime_type_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.guess_mime_type_enabled", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_encrypted` after provisioning.\n"]
    pub fn kms_encrypted(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_encrypted", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `location_arn` after provisioning.\n"]
    pub fn location_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `notification_policy` after provisioning.\n"]
    pub fn notification_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.notification_policy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `object_acl` after provisioning.\n"]
    pub fn object_acl(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.object_acl", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.path", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `read_only` after provisioning.\n"]
    pub fn read_only(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.read_only", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `requester_pays` after provisioning.\n"]
    pub fn requester_pays(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.requester_pays", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.role_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `squash` after provisioning.\n"]
    pub fn squash(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.squash", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `vpc_endpoint_dns_name` after provisioning.\n"]
    pub fn vpc_endpoint_dns_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vpc_endpoint_dns_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `cache_attributes` after provisioning.\n"]
    pub fn cache_attributes(&self) -> ListRef<StoragegatewayNfsFileShareCacheAttributesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cache_attributes", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `nfs_file_share_defaults` after provisioning.\n"]
    pub fn nfs_file_share_defaults(
        &self,
    ) -> ListRef<StoragegatewayNfsFileShareNfsFileShareDefaultsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.nfs_file_share_defaults", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> StoragegatewayNfsFileShareTimeoutsElRef {
        StoragegatewayNfsFileShareTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for StoragegatewayNfsFileShare {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for StoragegatewayNfsFileShare {}

impl ToListMappable for StoragegatewayNfsFileShare {
    type O = ListRef<StoragegatewayNfsFileShareRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for StoragegatewayNfsFileShare_ {
    fn extract_resource_type(&self) -> String {
        "aws_storagegateway_nfs_file_share".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildStoragegatewayNfsFileShare {
    pub tf_id: String,
    #[doc = ""]
    pub client_list: SetField<PrimField<String>>,
    #[doc = ""]
    pub gateway_arn: PrimField<String>,
    #[doc = ""]
    pub location_arn: PrimField<String>,
    #[doc = ""]
    pub role_arn: PrimField<String>,
}

impl BuildStoragegatewayNfsFileShare {
    pub fn build(self, stack: &mut Stack) -> StoragegatewayNfsFileShare {
        let out = StoragegatewayNfsFileShare(Rc::new(StoragegatewayNfsFileShare_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(StoragegatewayNfsFileShareData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                audit_destination_arn: core::default::Default::default(),
                bucket_region: core::default::Default::default(),
                client_list: self.client_list,
                default_storage_class: core::default::Default::default(),
                file_share_name: core::default::Default::default(),
                gateway_arn: self.gateway_arn,
                guess_mime_type_enabled: core::default::Default::default(),
                id: core::default::Default::default(),
                kms_encrypted: core::default::Default::default(),
                kms_key_arn: core::default::Default::default(),
                location_arn: self.location_arn,
                notification_policy: core::default::Default::default(),
                object_acl: core::default::Default::default(),
                read_only: core::default::Default::default(),
                region: core::default::Default::default(),
                requester_pays: core::default::Default::default(),
                role_arn: self.role_arn,
                squash: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                vpc_endpoint_dns_name: core::default::Default::default(),
                cache_attributes: core::default::Default::default(),
                nfs_file_share_defaults: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct StoragegatewayNfsFileShareRef {
    shared: StackShared,
    base: String,
}

impl Ref for StoragegatewayNfsFileShareRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl StoragegatewayNfsFileShareRef {
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

    #[doc = "Get a reference to the value of field `audit_destination_arn` after provisioning.\n"]
    pub fn audit_destination_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.audit_destination_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `bucket_region` after provisioning.\n"]
    pub fn bucket_region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bucket_region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `client_list` after provisioning.\n"]
    pub fn client_list(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.client_list", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `default_storage_class` after provisioning.\n"]
    pub fn default_storage_class(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_storage_class", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `file_share_name` after provisioning.\n"]
    pub fn file_share_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.file_share_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `fileshare_id` after provisioning.\n"]
    pub fn fileshare_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.fileshare_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `gateway_arn` after provisioning.\n"]
    pub fn gateway_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gateway_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `guess_mime_type_enabled` after provisioning.\n"]
    pub fn guess_mime_type_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.guess_mime_type_enabled", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_encrypted` after provisioning.\n"]
    pub fn kms_encrypted(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_encrypted", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `location_arn` after provisioning.\n"]
    pub fn location_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `notification_policy` after provisioning.\n"]
    pub fn notification_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.notification_policy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `object_acl` after provisioning.\n"]
    pub fn object_acl(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.object_acl", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.path", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `read_only` after provisioning.\n"]
    pub fn read_only(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.read_only", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `requester_pays` after provisioning.\n"]
    pub fn requester_pays(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.requester_pays", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.role_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `squash` after provisioning.\n"]
    pub fn squash(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.squash", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `vpc_endpoint_dns_name` after provisioning.\n"]
    pub fn vpc_endpoint_dns_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vpc_endpoint_dns_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `cache_attributes` after provisioning.\n"]
    pub fn cache_attributes(&self) -> ListRef<StoragegatewayNfsFileShareCacheAttributesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cache_attributes", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `nfs_file_share_defaults` after provisioning.\n"]
    pub fn nfs_file_share_defaults(
        &self,
    ) -> ListRef<StoragegatewayNfsFileShareNfsFileShareDefaultsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.nfs_file_share_defaults", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> StoragegatewayNfsFileShareTimeoutsElRef {
        StoragegatewayNfsFileShareTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct StoragegatewayNfsFileShareCacheAttributesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_stale_timeout_in_seconds: Option<PrimField<f64>>,
}

impl StoragegatewayNfsFileShareCacheAttributesEl {
    #[doc = "Set the field `cache_stale_timeout_in_seconds`.\n"]
    pub fn set_cache_stale_timeout_in_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.cache_stale_timeout_in_seconds = Some(v.into());
        self
    }
}

impl ToListMappable for StoragegatewayNfsFileShareCacheAttributesEl {
    type O = BlockAssignable<StoragegatewayNfsFileShareCacheAttributesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStoragegatewayNfsFileShareCacheAttributesEl {}

impl BuildStoragegatewayNfsFileShareCacheAttributesEl {
    pub fn build(self) -> StoragegatewayNfsFileShareCacheAttributesEl {
        StoragegatewayNfsFileShareCacheAttributesEl {
            cache_stale_timeout_in_seconds: core::default::Default::default(),
        }
    }
}

pub struct StoragegatewayNfsFileShareCacheAttributesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for StoragegatewayNfsFileShareCacheAttributesElRef {
    fn new(shared: StackShared, base: String) -> StoragegatewayNfsFileShareCacheAttributesElRef {
        StoragegatewayNfsFileShareCacheAttributesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StoragegatewayNfsFileShareCacheAttributesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cache_stale_timeout_in_seconds` after provisioning.\n"]
    pub fn cache_stale_timeout_in_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cache_stale_timeout_in_seconds", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct StoragegatewayNfsFileShareNfsFileShareDefaultsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    directory_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    owner_id: Option<PrimField<String>>,
}

impl StoragegatewayNfsFileShareNfsFileShareDefaultsEl {
    #[doc = "Set the field `directory_mode`.\n"]
    pub fn set_directory_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.directory_mode = Some(v.into());
        self
    }

    #[doc = "Set the field `file_mode`.\n"]
    pub fn set_file_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.file_mode = Some(v.into());
        self
    }

    #[doc = "Set the field `group_id`.\n"]
    pub fn set_group_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.group_id = Some(v.into());
        self
    }

    #[doc = "Set the field `owner_id`.\n"]
    pub fn set_owner_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.owner_id = Some(v.into());
        self
    }
}

impl ToListMappable for StoragegatewayNfsFileShareNfsFileShareDefaultsEl {
    type O = BlockAssignable<StoragegatewayNfsFileShareNfsFileShareDefaultsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStoragegatewayNfsFileShareNfsFileShareDefaultsEl {}

impl BuildStoragegatewayNfsFileShareNfsFileShareDefaultsEl {
    pub fn build(self) -> StoragegatewayNfsFileShareNfsFileShareDefaultsEl {
        StoragegatewayNfsFileShareNfsFileShareDefaultsEl {
            directory_mode: core::default::Default::default(),
            file_mode: core::default::Default::default(),
            group_id: core::default::Default::default(),
            owner_id: core::default::Default::default(),
        }
    }
}

pub struct StoragegatewayNfsFileShareNfsFileShareDefaultsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for StoragegatewayNfsFileShareNfsFileShareDefaultsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> StoragegatewayNfsFileShareNfsFileShareDefaultsElRef {
        StoragegatewayNfsFileShareNfsFileShareDefaultsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StoragegatewayNfsFileShareNfsFileShareDefaultsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `directory_mode` after provisioning.\n"]
    pub fn directory_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.directory_mode", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `file_mode` after provisioning.\n"]
    pub fn file_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.file_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `group_id` after provisioning.\n"]
    pub fn group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_id", self.base))
    }

    #[doc = "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.owner_id", self.base))
    }
}

#[derive(Serialize)]
pub struct StoragegatewayNfsFileShareTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl StoragegatewayNfsFileShareTimeoutsEl {
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

impl ToListMappable for StoragegatewayNfsFileShareTimeoutsEl {
    type O = BlockAssignable<StoragegatewayNfsFileShareTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStoragegatewayNfsFileShareTimeoutsEl {}

impl BuildStoragegatewayNfsFileShareTimeoutsEl {
    pub fn build(self) -> StoragegatewayNfsFileShareTimeoutsEl {
        StoragegatewayNfsFileShareTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct StoragegatewayNfsFileShareTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for StoragegatewayNfsFileShareTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> StoragegatewayNfsFileShareTimeoutsElRef {
        StoragegatewayNfsFileShareTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StoragegatewayNfsFileShareTimeoutsElRef {
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
struct StoragegatewayNfsFileShareDynamic {
    cache_attributes: Option<DynamicBlock<StoragegatewayNfsFileShareCacheAttributesEl>>,
    nfs_file_share_defaults: Option<DynamicBlock<StoragegatewayNfsFileShareNfsFileShareDefaultsEl>>,
}
