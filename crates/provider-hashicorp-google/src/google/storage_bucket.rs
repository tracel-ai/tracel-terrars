use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct StorageBucketData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_event_based_hold: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_object_retention: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    force_destroy: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    location: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    public_access_prevention: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    requester_pays: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rpo: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_class: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    uniform_bucket_level_access: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    autoclass: Option<Vec<StorageBucketAutoclassEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cors: Option<Vec<StorageBucketCorsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_placement_config: Option<Vec<StorageBucketCustomPlacementConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption: Option<Vec<StorageBucketEncryptionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hierarchical_namespace: Option<Vec<StorageBucketHierarchicalNamespaceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_filter: Option<Vec<StorageBucketIpFilterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle_rule: Option<Vec<StorageBucketLifecycleRuleEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logging: Option<Vec<StorageBucketLoggingEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retention_policy: Option<Vec<StorageBucketRetentionPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    soft_delete_policy: Option<Vec<StorageBucketSoftDeletePolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<StorageBucketTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    versioning: Option<Vec<StorageBucketVersioningEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    website: Option<Vec<StorageBucketWebsiteEl>>,
    dynamic: StorageBucketDynamic,
}
struct StorageBucket_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<StorageBucketData>,
}
#[derive(Clone)]
pub struct StorageBucket(Rc<StorageBucket_>);
impl StorageBucket {
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
    #[doc = "Set the field `default_event_based_hold`.\nWhether or not to automatically apply an eventBasedHold to new objects added to the bucket."]
    pub fn set_default_event_based_hold(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().default_event_based_hold = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_object_retention`.\nEnables each object in the bucket to have its own retention policy, which prevents deletion until stored for a specific length of time."]
    pub fn set_enable_object_retention(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_object_retention = Some(v.into());
        self
    }
    #[doc = "Set the field `force_destroy`.\nWhen true, before deleting a bucket, delete all objects within the bucket, or Anywhere Caches caching data for that bucket. Otherwise, buckets with objects/caches will fail. Anywhere Cache requires additional permissions to interact with and will be ignored when those are not present, attempting to delete anyways. This may result in the objects in the bucket getting destroyed but not the bucket itself if there is a cache in use with the bucket. Force deletion may take a long time to delete buckets with lots of objects or with any Anywhere Caches (80m+)."]
    pub fn set_force_destroy(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().force_destroy = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nA set of key/value label pairs to assign to the bucket."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `public_access_prevention`.\nPrevents public access to a bucket."]
    pub fn set_public_access_prevention(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().public_access_prevention = Some(v.into());
        self
    }
    #[doc = "Set the field `requester_pays`.\nEnables Requester Pays on a storage bucket."]
    pub fn set_requester_pays(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().requester_pays = Some(v.into());
        self
    }
    #[doc = "Set the field `rpo`.\nSpecifies the RPO setting of bucket. If set 'ASYNC_TURBO', The Turbo Replication will be enabled for the dual-region bucket. Value 'DEFAULT' will set RPO setting to default. Turbo Replication is only for buckets in dual-regions.See the docs for more details."]
    pub fn set_rpo(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().rpo = Some(v.into());
        self
    }
    #[doc = "Set the field `storage_class`.\nThe Storage Class of the new bucket. Supported values include: STANDARD, MULTI_REGIONAL, REGIONAL, NEARLINE, COLDLINE, ARCHIVE."]
    pub fn set_storage_class(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().storage_class = Some(v.into());
        self
    }
    #[doc = "Set the field `uniform_bucket_level_access`.\nEnables uniform bucket-level access on a bucket."]
    pub fn set_uniform_bucket_level_access(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().uniform_bucket_level_access = Some(v.into());
        self
    }
    #[doc = "Set the field `autoclass`.\n"]
    pub fn set_autoclass(self, v: impl Into<BlockAssignable<StorageBucketAutoclassEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().autoclass = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.autoclass = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `cors`.\n"]
    pub fn set_cors(self, v: impl Into<BlockAssignable<StorageBucketCorsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cors = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cors = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `custom_placement_config`.\n"]
    pub fn set_custom_placement_config(
        self,
        v: impl Into<BlockAssignable<StorageBucketCustomPlacementConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().custom_placement_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.custom_placement_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `encryption`.\n"]
    pub fn set_encryption(self, v: impl Into<BlockAssignable<StorageBucketEncryptionEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().encryption = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.encryption = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `hierarchical_namespace`.\n"]
    pub fn set_hierarchical_namespace(
        self,
        v: impl Into<BlockAssignable<StorageBucketHierarchicalNamespaceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().hierarchical_namespace = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.hierarchical_namespace = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `ip_filter`.\n"]
    pub fn set_ip_filter(self, v: impl Into<BlockAssignable<StorageBucketIpFilterEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ip_filter = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ip_filter = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `lifecycle_rule`.\n"]
    pub fn set_lifecycle_rule(
        self,
        v: impl Into<BlockAssignable<StorageBucketLifecycleRuleEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().lifecycle_rule = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.lifecycle_rule = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `logging`.\n"]
    pub fn set_logging(self, v: impl Into<BlockAssignable<StorageBucketLoggingEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().logging = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.logging = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `retention_policy`.\n"]
    pub fn set_retention_policy(
        self,
        v: impl Into<BlockAssignable<StorageBucketRetentionPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().retention_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.retention_policy = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `soft_delete_policy`.\n"]
    pub fn set_soft_delete_policy(
        self,
        v: impl Into<BlockAssignable<StorageBucketSoftDeletePolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().soft_delete_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.soft_delete_policy = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<StorageBucketTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Set the field `versioning`.\n"]
    pub fn set_versioning(self, v: impl Into<BlockAssignable<StorageBucketVersioningEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().versioning = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.versioning = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `website`.\n"]
    pub fn set_website(self, v: impl Into<BlockAssignable<StorageBucketWebsiteEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().website = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.website = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `default_event_based_hold` after provisioning.\nWhether or not to automatically apply an eventBasedHold to new objects added to the bucket."]
    pub fn default_event_based_hold(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_event_based_hold", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_object_retention` after provisioning.\nEnables each object in the bucket to have its own retention policy, which prevents deletion until stored for a specific length of time."]
    pub fn enable_object_retention(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_object_retention", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `force_destroy` after provisioning.\nWhen true, before deleting a bucket, delete all objects within the bucket, or Anywhere Caches caching data for that bucket. Otherwise, buckets with objects/caches will fail. Anywhere Cache requires additional permissions to interact with and will be ignored when those are not present, attempting to delete anyways. This may result in the objects in the bucket getting destroyed but not the bucket itself if there is a cache in use with the bucket. Force deletion may take a long time to delete buckets with lots of objects or with any Anywhere Caches (80m+)."]
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
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nA set of key/value label pairs to assign to the bucket."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe Google Cloud Storage location or region."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the bucket."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project_number` after provisioning.\nThe project number of the project in which the resource belongs."]
    pub fn project_number(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project_number", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `public_access_prevention` after provisioning.\nPrevents public access to a bucket."]
    pub fn public_access_prevention(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.public_access_prevention", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `requester_pays` after provisioning.\nEnables Requester Pays on a storage bucket."]
    pub fn requester_pays(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.requester_pays", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rpo` after provisioning.\nSpecifies the RPO setting of bucket. If set 'ASYNC_TURBO', The Turbo Replication will be enabled for the dual-region bucket. Value 'DEFAULT' will set RPO setting to default. Turbo Replication is only for buckets in dual-regions.See the docs for more details."]
    pub fn rpo(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.rpo", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\nThe URI of the created resource."]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `storage_class` after provisioning.\nThe Storage Class of the new bucket. Supported values include: STANDARD, MULTI_REGIONAL, REGIONAL, NEARLINE, COLDLINE, ARCHIVE."]
    pub fn storage_class(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.storage_class", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `time_created` after provisioning.\nThe creation time of the bucket in RFC 3339 format."]
    pub fn time_created(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.time_created", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uniform_bucket_level_access` after provisioning.\nEnables uniform bucket-level access on a bucket."]
    pub fn uniform_bucket_level_access(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.uniform_bucket_level_access", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `updated` after provisioning.\nThe time at which the bucket's metadata or IAM policy was last updated, in RFC 3339 format."]
    pub fn updated(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.updated", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `url` after provisioning.\nThe base URL of the bucket, in the format gs://<bucket-name>."]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `autoclass` after provisioning.\n"]
    pub fn autoclass(&self) -> ListRef<StorageBucketAutoclassElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.autoclass", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cors` after provisioning.\n"]
    pub fn cors(&self) -> ListRef<StorageBucketCorsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cors", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `custom_placement_config` after provisioning.\n"]
    pub fn custom_placement_config(&self) -> ListRef<StorageBucketCustomPlacementConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_placement_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `encryption` after provisioning.\n"]
    pub fn encryption(&self) -> ListRef<StorageBucketEncryptionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.encryption", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `hierarchical_namespace` after provisioning.\n"]
    pub fn hierarchical_namespace(&self) -> ListRef<StorageBucketHierarchicalNamespaceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.hierarchical_namespace", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ip_filter` after provisioning.\n"]
    pub fn ip_filter(&self) -> ListRef<StorageBucketIpFilterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ip_filter", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `lifecycle_rule` after provisioning.\n"]
    pub fn lifecycle_rule(&self) -> ListRef<StorageBucketLifecycleRuleElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.lifecycle_rule", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `logging` after provisioning.\n"]
    pub fn logging(&self) -> ListRef<StorageBucketLoggingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.logging", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `retention_policy` after provisioning.\n"]
    pub fn retention_policy(&self) -> ListRef<StorageBucketRetentionPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.retention_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `soft_delete_policy` after provisioning.\n"]
    pub fn soft_delete_policy(&self) -> ListRef<StorageBucketSoftDeletePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.soft_delete_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> StorageBucketTimeoutsElRef {
        StorageBucketTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `versioning` after provisioning.\n"]
    pub fn versioning(&self) -> ListRef<StorageBucketVersioningElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.versioning", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `website` after provisioning.\n"]
    pub fn website(&self) -> ListRef<StorageBucketWebsiteElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.website", self.extract_ref()),
        )
    }
}
impl Referable for StorageBucket {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for StorageBucket {}
impl ToListMappable for StorageBucket {
    type O = ListRef<StorageBucketRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for StorageBucket_ {
    fn extract_resource_type(&self) -> String {
        "google_storage_bucket".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildStorageBucket {
    pub tf_id: String,
    #[doc = "The Google Cloud Storage location or region."]
    pub location: PrimField<String>,
    #[doc = "The name of the bucket."]
    pub name: PrimField<String>,
}
impl BuildStorageBucket {
    pub fn build(self, stack: &mut Stack) -> StorageBucket {
        let out = StorageBucket(Rc::new(StorageBucket_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(StorageBucketData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                default_event_based_hold: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                enable_object_retention: core::default::Default::default(),
                force_destroy: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                name: self.name,
                project: core::default::Default::default(),
                public_access_prevention: core::default::Default::default(),
                requester_pays: core::default::Default::default(),
                rpo: core::default::Default::default(),
                storage_class: core::default::Default::default(),
                uniform_bucket_level_access: core::default::Default::default(),
                autoclass: core::default::Default::default(),
                cors: core::default::Default::default(),
                custom_placement_config: core::default::Default::default(),
                encryption: core::default::Default::default(),
                hierarchical_namespace: core::default::Default::default(),
                ip_filter: core::default::Default::default(),
                lifecycle_rule: core::default::Default::default(),
                logging: core::default::Default::default(),
                retention_policy: core::default::Default::default(),
                soft_delete_policy: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                versioning: core::default::Default::default(),
                website: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct StorageBucketRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageBucketRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl StorageBucketRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `default_event_based_hold` after provisioning.\nWhether or not to automatically apply an eventBasedHold to new objects added to the bucket."]
    pub fn default_event_based_hold(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_event_based_hold", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_object_retention` after provisioning.\nEnables each object in the bucket to have its own retention policy, which prevents deletion until stored for a specific length of time."]
    pub fn enable_object_retention(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_object_retention", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `force_destroy` after provisioning.\nWhen true, before deleting a bucket, delete all objects within the bucket, or Anywhere Caches caching data for that bucket. Otherwise, buckets with objects/caches will fail. Anywhere Cache requires additional permissions to interact with and will be ignored when those are not present, attempting to delete anyways. This may result in the objects in the bucket getting destroyed but not the bucket itself if there is a cache in use with the bucket. Force deletion may take a long time to delete buckets with lots of objects or with any Anywhere Caches (80m+)."]
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
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nA set of key/value label pairs to assign to the bucket."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe Google Cloud Storage location or region."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the bucket."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project_number` after provisioning.\nThe project number of the project in which the resource belongs."]
    pub fn project_number(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project_number", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `public_access_prevention` after provisioning.\nPrevents public access to a bucket."]
    pub fn public_access_prevention(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.public_access_prevention", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `requester_pays` after provisioning.\nEnables Requester Pays on a storage bucket."]
    pub fn requester_pays(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.requester_pays", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rpo` after provisioning.\nSpecifies the RPO setting of bucket. If set 'ASYNC_TURBO', The Turbo Replication will be enabled for the dual-region bucket. Value 'DEFAULT' will set RPO setting to default. Turbo Replication is only for buckets in dual-regions.See the docs for more details."]
    pub fn rpo(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.rpo", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\nThe URI of the created resource."]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `storage_class` after provisioning.\nThe Storage Class of the new bucket. Supported values include: STANDARD, MULTI_REGIONAL, REGIONAL, NEARLINE, COLDLINE, ARCHIVE."]
    pub fn storage_class(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.storage_class", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `time_created` after provisioning.\nThe creation time of the bucket in RFC 3339 format."]
    pub fn time_created(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.time_created", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uniform_bucket_level_access` after provisioning.\nEnables uniform bucket-level access on a bucket."]
    pub fn uniform_bucket_level_access(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.uniform_bucket_level_access", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `updated` after provisioning.\nThe time at which the bucket's metadata or IAM policy was last updated, in RFC 3339 format."]
    pub fn updated(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.updated", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `url` after provisioning.\nThe base URL of the bucket, in the format gs://<bucket-name>."]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `autoclass` after provisioning.\n"]
    pub fn autoclass(&self) -> ListRef<StorageBucketAutoclassElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.autoclass", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cors` after provisioning.\n"]
    pub fn cors(&self) -> ListRef<StorageBucketCorsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cors", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `custom_placement_config` after provisioning.\n"]
    pub fn custom_placement_config(&self) -> ListRef<StorageBucketCustomPlacementConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_placement_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `encryption` after provisioning.\n"]
    pub fn encryption(&self) -> ListRef<StorageBucketEncryptionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.encryption", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `hierarchical_namespace` after provisioning.\n"]
    pub fn hierarchical_namespace(&self) -> ListRef<StorageBucketHierarchicalNamespaceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.hierarchical_namespace", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ip_filter` after provisioning.\n"]
    pub fn ip_filter(&self) -> ListRef<StorageBucketIpFilterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ip_filter", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `lifecycle_rule` after provisioning.\n"]
    pub fn lifecycle_rule(&self) -> ListRef<StorageBucketLifecycleRuleElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.lifecycle_rule", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `logging` after provisioning.\n"]
    pub fn logging(&self) -> ListRef<StorageBucketLoggingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.logging", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `retention_policy` after provisioning.\n"]
    pub fn retention_policy(&self) -> ListRef<StorageBucketRetentionPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.retention_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `soft_delete_policy` after provisioning.\n"]
    pub fn soft_delete_policy(&self) -> ListRef<StorageBucketSoftDeletePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.soft_delete_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> StorageBucketTimeoutsElRef {
        StorageBucketTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `versioning` after provisioning.\n"]
    pub fn versioning(&self) -> ListRef<StorageBucketVersioningElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.versioning", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `website` after provisioning.\n"]
    pub fn website(&self) -> ListRef<StorageBucketWebsiteElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.website", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct StorageBucketAutoclassEl {
    enabled: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    terminal_storage_class: Option<PrimField<String>>,
}
impl StorageBucketAutoclassEl {
    #[doc = "Set the field `terminal_storage_class`.\nThe storage class that objects in the bucket eventually transition to if they are not read for a certain length of time. Supported values include: NEARLINE, ARCHIVE."]
    pub fn set_terminal_storage_class(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.terminal_storage_class = Some(v.into());
        self
    }
}
impl ToListMappable for StorageBucketAutoclassEl {
    type O = BlockAssignable<StorageBucketAutoclassEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageBucketAutoclassEl {
    #[doc = "While set to true, autoclass automatically transitions objects in your bucket to appropriate storage classes based on each object's access pattern."]
    pub enabled: PrimField<bool>,
}
impl BuildStorageBucketAutoclassEl {
    pub fn build(self) -> StorageBucketAutoclassEl {
        StorageBucketAutoclassEl {
            enabled: self.enabled,
            terminal_storage_class: core::default::Default::default(),
        }
    }
}
pub struct StorageBucketAutoclassElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageBucketAutoclassElRef {
    fn new(shared: StackShared, base: String) -> StorageBucketAutoclassElRef {
        StorageBucketAutoclassElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageBucketAutoclassElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nWhile set to true, autoclass automatically transitions objects in your bucket to appropriate storage classes based on each object's access pattern."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
    #[doc = "Get a reference to the value of field `terminal_storage_class` after provisioning.\nThe storage class that objects in the bucket eventually transition to if they are not read for a certain length of time. Supported values include: NEARLINE, ARCHIVE."]
    pub fn terminal_storage_class(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.terminal_storage_class", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct StorageBucketCorsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_age_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    method: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    origin: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_header: Option<ListField<PrimField<String>>>,
}
impl StorageBucketCorsEl {
    #[doc = "Set the field `max_age_seconds`.\nThe value, in seconds, to return in the Access-Control-Max-Age header used in preflight responses."]
    pub fn set_max_age_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_age_seconds = Some(v.into());
        self
    }
    #[doc = "Set the field `method`.\nThe list of HTTP methods on which to include CORS response headers, (GET, OPTIONS, POST, etc) Note: \"*\" is permitted in the list of methods, and means \"any method\"."]
    pub fn set_method(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.method = Some(v.into());
        self
    }
    #[doc = "Set the field `origin`.\nThe list of Origins eligible to receive CORS response headers. Note: \"*\" is permitted in the list of origins, and means \"any Origin\"."]
    pub fn set_origin(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.origin = Some(v.into());
        self
    }
    #[doc = "Set the field `response_header`.\nThe list of HTTP headers other than the simple response headers to give permission for the user-agent to share across domains."]
    pub fn set_response_header(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.response_header = Some(v.into());
        self
    }
}
impl ToListMappable for StorageBucketCorsEl {
    type O = BlockAssignable<StorageBucketCorsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageBucketCorsEl {}
impl BuildStorageBucketCorsEl {
    pub fn build(self) -> StorageBucketCorsEl {
        StorageBucketCorsEl {
            max_age_seconds: core::default::Default::default(),
            method: core::default::Default::default(),
            origin: core::default::Default::default(),
            response_header: core::default::Default::default(),
        }
    }
}
pub struct StorageBucketCorsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageBucketCorsElRef {
    fn new(shared: StackShared, base: String) -> StorageBucketCorsElRef {
        StorageBucketCorsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageBucketCorsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `max_age_seconds` after provisioning.\nThe value, in seconds, to return in the Access-Control-Max-Age header used in preflight responses."]
    pub fn max_age_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_age_seconds", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `method` after provisioning.\nThe list of HTTP methods on which to include CORS response headers, (GET, OPTIONS, POST, etc) Note: \"*\" is permitted in the list of methods, and means \"any method\"."]
    pub fn method(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.method", self.base))
    }
    #[doc = "Get a reference to the value of field `origin` after provisioning.\nThe list of Origins eligible to receive CORS response headers. Note: \"*\" is permitted in the list of origins, and means \"any Origin\"."]
    pub fn origin(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.origin", self.base))
    }
    #[doc = "Get a reference to the value of field `response_header` after provisioning.\nThe list of HTTP headers other than the simple response headers to give permission for the user-agent to share across domains."]
    pub fn response_header(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.response_header", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct StorageBucketCustomPlacementConfigEl {
    data_locations: SetField<PrimField<String>>,
}
impl StorageBucketCustomPlacementConfigEl {}
impl ToListMappable for StorageBucketCustomPlacementConfigEl {
    type O = BlockAssignable<StorageBucketCustomPlacementConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageBucketCustomPlacementConfigEl {
    #[doc = "The list of individual regions that comprise a dual-region bucket. See the docs for a list of acceptable regions. Note: If any of the data_locations changes, it will recreate the bucket."]
    pub data_locations: SetField<PrimField<String>>,
}
impl BuildStorageBucketCustomPlacementConfigEl {
    pub fn build(self) -> StorageBucketCustomPlacementConfigEl {
        StorageBucketCustomPlacementConfigEl {
            data_locations: self.data_locations,
        }
    }
}
pub struct StorageBucketCustomPlacementConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageBucketCustomPlacementConfigElRef {
    fn new(shared: StackShared, base: String) -> StorageBucketCustomPlacementConfigElRef {
        StorageBucketCustomPlacementConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageBucketCustomPlacementConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `data_locations` after provisioning.\nThe list of individual regions that comprise a dual-region bucket. See the docs for a list of acceptable regions. Note: If any of the data_locations changes, it will recreate the bucket."]
    pub fn data_locations(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.data_locations", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct StorageBucketEncryptionElCustomerManagedEncryptionEnforcementConfigEl {
    restriction_mode: PrimField<String>,
}
impl StorageBucketEncryptionElCustomerManagedEncryptionEnforcementConfigEl {}
impl ToListMappable for StorageBucketEncryptionElCustomerManagedEncryptionEnforcementConfigEl {
    type O = BlockAssignable<StorageBucketEncryptionElCustomerManagedEncryptionEnforcementConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageBucketEncryptionElCustomerManagedEncryptionEnforcementConfigEl {
    #[doc = "Whether CMEK is restricted for new objects within the bucket. If FullyRestricted, new objects can't be created using CMEK encryption. If NotRestricted or unset, creation of new objects with CMEK encryption is allowed."]
    pub restriction_mode: PrimField<String>,
}
impl BuildStorageBucketEncryptionElCustomerManagedEncryptionEnforcementConfigEl {
    pub fn build(self) -> StorageBucketEncryptionElCustomerManagedEncryptionEnforcementConfigEl {
        StorageBucketEncryptionElCustomerManagedEncryptionEnforcementConfigEl {
            restriction_mode: self.restriction_mode,
        }
    }
}
pub struct StorageBucketEncryptionElCustomerManagedEncryptionEnforcementConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageBucketEncryptionElCustomerManagedEncryptionEnforcementConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> StorageBucketEncryptionElCustomerManagedEncryptionEnforcementConfigElRef {
        StorageBucketEncryptionElCustomerManagedEncryptionEnforcementConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageBucketEncryptionElCustomerManagedEncryptionEnforcementConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `effective_time` after provisioning.\nTime from which the config was effective. This is service-provided."]
    pub fn effective_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.effective_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `restriction_mode` after provisioning.\nWhether CMEK is restricted for new objects within the bucket. If FullyRestricted, new objects can't be created using CMEK encryption. If NotRestricted or unset, creation of new objects with CMEK encryption is allowed."]
    pub fn restriction_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.restriction_mode", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct StorageBucketEncryptionElCustomerSuppliedEncryptionEnforcementConfigEl {
    restriction_mode: PrimField<String>,
}
impl StorageBucketEncryptionElCustomerSuppliedEncryptionEnforcementConfigEl {}
impl ToListMappable for StorageBucketEncryptionElCustomerSuppliedEncryptionEnforcementConfigEl {
    type O =
        BlockAssignable<StorageBucketEncryptionElCustomerSuppliedEncryptionEnforcementConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageBucketEncryptionElCustomerSuppliedEncryptionEnforcementConfigEl {
    #[doc = "Whether CSEK is restricted for new objects within the bucket. If FullyRestricted, new objects can't be created using CSEK encryption. If NotRestricted or unset, creation of new objects with CSEK encryption is allowed."]
    pub restriction_mode: PrimField<String>,
}
impl BuildStorageBucketEncryptionElCustomerSuppliedEncryptionEnforcementConfigEl {
    pub fn build(self) -> StorageBucketEncryptionElCustomerSuppliedEncryptionEnforcementConfigEl {
        StorageBucketEncryptionElCustomerSuppliedEncryptionEnforcementConfigEl {
            restriction_mode: self.restriction_mode,
        }
    }
}
pub struct StorageBucketEncryptionElCustomerSuppliedEncryptionEnforcementConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageBucketEncryptionElCustomerSuppliedEncryptionEnforcementConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> StorageBucketEncryptionElCustomerSuppliedEncryptionEnforcementConfigElRef {
        StorageBucketEncryptionElCustomerSuppliedEncryptionEnforcementConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageBucketEncryptionElCustomerSuppliedEncryptionEnforcementConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `effective_time` after provisioning.\nTime from which the config was effective. This is service-provided."]
    pub fn effective_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.effective_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `restriction_mode` after provisioning.\nWhether CSEK is restricted for new objects within the bucket. If FullyRestricted, new objects can't be created using CSEK encryption. If NotRestricted or unset, creation of new objects with CSEK encryption is allowed."]
    pub fn restriction_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.restriction_mode", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct StorageBucketEncryptionElGoogleManagedEncryptionEnforcementConfigEl {
    restriction_mode: PrimField<String>,
}
impl StorageBucketEncryptionElGoogleManagedEncryptionEnforcementConfigEl {}
impl ToListMappable for StorageBucketEncryptionElGoogleManagedEncryptionEnforcementConfigEl {
    type O = BlockAssignable<StorageBucketEncryptionElGoogleManagedEncryptionEnforcementConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageBucketEncryptionElGoogleManagedEncryptionEnforcementConfigEl {
    #[doc = "Whether GMEK is restricted for new objects within the bucket. If FullyRestricted, new objects can't be created using GMEK encryption. If NotRestricted or unset, creation of new objects with GMEK encryption is allowed."]
    pub restriction_mode: PrimField<String>,
}
impl BuildStorageBucketEncryptionElGoogleManagedEncryptionEnforcementConfigEl {
    pub fn build(self) -> StorageBucketEncryptionElGoogleManagedEncryptionEnforcementConfigEl {
        StorageBucketEncryptionElGoogleManagedEncryptionEnforcementConfigEl {
            restriction_mode: self.restriction_mode,
        }
    }
}
pub struct StorageBucketEncryptionElGoogleManagedEncryptionEnforcementConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageBucketEncryptionElGoogleManagedEncryptionEnforcementConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> StorageBucketEncryptionElGoogleManagedEncryptionEnforcementConfigElRef {
        StorageBucketEncryptionElGoogleManagedEncryptionEnforcementConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageBucketEncryptionElGoogleManagedEncryptionEnforcementConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `effective_time` after provisioning.\nTime from which the config was effective. This is service-provided."]
    pub fn effective_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.effective_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `restriction_mode` after provisioning.\nWhether GMEK is restricted for new objects within the bucket. If FullyRestricted, new objects can't be created using GMEK encryption. If NotRestricted or unset, creation of new objects with GMEK encryption is allowed."]
    pub fn restriction_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.restriction_mode", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct StorageBucketEncryptionElDynamic {
    customer_managed_encryption_enforcement_config:
        Option<DynamicBlock<StorageBucketEncryptionElCustomerManagedEncryptionEnforcementConfigEl>>,
    customer_supplied_encryption_enforcement_config: Option<
        DynamicBlock<StorageBucketEncryptionElCustomerSuppliedEncryptionEnforcementConfigEl>,
    >,
    google_managed_encryption_enforcement_config:
        Option<DynamicBlock<StorageBucketEncryptionElGoogleManagedEncryptionEnforcementConfigEl>>,
}
#[derive(Serialize)]
pub struct StorageBucketEncryptionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    default_kms_key_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_managed_encryption_enforcement_config:
        Option<Vec<StorageBucketEncryptionElCustomerManagedEncryptionEnforcementConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_supplied_encryption_enforcement_config:
        Option<Vec<StorageBucketEncryptionElCustomerSuppliedEncryptionEnforcementConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    google_managed_encryption_enforcement_config:
        Option<Vec<StorageBucketEncryptionElGoogleManagedEncryptionEnforcementConfigEl>>,
    dynamic: StorageBucketEncryptionElDynamic,
}
impl StorageBucketEncryptionEl {
    #[doc = "Set the field `default_kms_key_name`.\nA Cloud KMS key that will be used to encrypt objects inserted into this bucket, if no encryption method is specified. You must pay attention to whether the crypto key is available in the location that this bucket is created in. See the docs for more details."]
    pub fn set_default_kms_key_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.default_kms_key_name = Some(v.into());
        self
    }
    #[doc = "Set the field `customer_managed_encryption_enforcement_config`.\n"]
    pub fn set_customer_managed_encryption_enforcement_config(
        mut self,
        v: impl Into<
            BlockAssignable<StorageBucketEncryptionElCustomerManagedEncryptionEnforcementConfigEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.customer_managed_encryption_enforcement_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.customer_managed_encryption_enforcement_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `customer_supplied_encryption_enforcement_config`.\n"]
    pub fn set_customer_supplied_encryption_enforcement_config(
        mut self,
        v: impl Into<
            BlockAssignable<StorageBucketEncryptionElCustomerSuppliedEncryptionEnforcementConfigEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.customer_supplied_encryption_enforcement_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.customer_supplied_encryption_enforcement_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `google_managed_encryption_enforcement_config`.\n"]
    pub fn set_google_managed_encryption_enforcement_config(
        mut self,
        v: impl Into<
            BlockAssignable<StorageBucketEncryptionElGoogleManagedEncryptionEnforcementConfigEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.google_managed_encryption_enforcement_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.google_managed_encryption_enforcement_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for StorageBucketEncryptionEl {
    type O = BlockAssignable<StorageBucketEncryptionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageBucketEncryptionEl {}
impl BuildStorageBucketEncryptionEl {
    pub fn build(self) -> StorageBucketEncryptionEl {
        StorageBucketEncryptionEl {
            default_kms_key_name: core::default::Default::default(),
            customer_managed_encryption_enforcement_config: core::default::Default::default(),
            customer_supplied_encryption_enforcement_config: core::default::Default::default(),
            google_managed_encryption_enforcement_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct StorageBucketEncryptionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageBucketEncryptionElRef {
    fn new(shared: StackShared, base: String) -> StorageBucketEncryptionElRef {
        StorageBucketEncryptionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageBucketEncryptionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `default_kms_key_name` after provisioning.\nA Cloud KMS key that will be used to encrypt objects inserted into this bucket, if no encryption method is specified. You must pay attention to whether the crypto key is available in the location that this bucket is created in. See the docs for more details."]
    pub fn default_kms_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_kms_key_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `customer_managed_encryption_enforcement_config` after provisioning.\n"]
    pub fn customer_managed_encryption_enforcement_config(
        &self,
    ) -> ListRef<StorageBucketEncryptionElCustomerManagedEncryptionEnforcementConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.customer_managed_encryption_enforcement_config",
                self.base
            ),
        )
    }
    #[doc = "Get a reference to the value of field `customer_supplied_encryption_enforcement_config` after provisioning.\n"]
    pub fn customer_supplied_encryption_enforcement_config(
        &self,
    ) -> ListRef<StorageBucketEncryptionElCustomerSuppliedEncryptionEnforcementConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.customer_supplied_encryption_enforcement_config",
                self.base
            ),
        )
    }
    #[doc = "Get a reference to the value of field `google_managed_encryption_enforcement_config` after provisioning.\n"]
    pub fn google_managed_encryption_enforcement_config(
        &self,
    ) -> ListRef<StorageBucketEncryptionElGoogleManagedEncryptionEnforcementConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.google_managed_encryption_enforcement_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct StorageBucketHierarchicalNamespaceEl {
    enabled: PrimField<bool>,
}
impl StorageBucketHierarchicalNamespaceEl {}
impl ToListMappable for StorageBucketHierarchicalNamespaceEl {
    type O = BlockAssignable<StorageBucketHierarchicalNamespaceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageBucketHierarchicalNamespaceEl {
    #[doc = "Set this field true to organize bucket with logical file system structure."]
    pub enabled: PrimField<bool>,
}
impl BuildStorageBucketHierarchicalNamespaceEl {
    pub fn build(self) -> StorageBucketHierarchicalNamespaceEl {
        StorageBucketHierarchicalNamespaceEl {
            enabled: self.enabled,
        }
    }
}
pub struct StorageBucketHierarchicalNamespaceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageBucketHierarchicalNamespaceElRef {
    fn new(shared: StackShared, base: String) -> StorageBucketHierarchicalNamespaceElRef {
        StorageBucketHierarchicalNamespaceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageBucketHierarchicalNamespaceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nSet this field true to organize bucket with logical file system structure."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize)]
pub struct StorageBucketIpFilterElPublicNetworkSourceEl {
    allowed_ip_cidr_ranges: ListField<PrimField<String>>,
}
impl StorageBucketIpFilterElPublicNetworkSourceEl {}
impl ToListMappable for StorageBucketIpFilterElPublicNetworkSourceEl {
    type O = BlockAssignable<StorageBucketIpFilterElPublicNetworkSourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageBucketIpFilterElPublicNetworkSourceEl {
    #[doc = "The list of public IPv4, IPv6 cidr ranges that are allowed to access the bucket."]
    pub allowed_ip_cidr_ranges: ListField<PrimField<String>>,
}
impl BuildStorageBucketIpFilterElPublicNetworkSourceEl {
    pub fn build(self) -> StorageBucketIpFilterElPublicNetworkSourceEl {
        StorageBucketIpFilterElPublicNetworkSourceEl {
            allowed_ip_cidr_ranges: self.allowed_ip_cidr_ranges,
        }
    }
}
pub struct StorageBucketIpFilterElPublicNetworkSourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageBucketIpFilterElPublicNetworkSourceElRef {
    fn new(shared: StackShared, base: String) -> StorageBucketIpFilterElPublicNetworkSourceElRef {
        StorageBucketIpFilterElPublicNetworkSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageBucketIpFilterElPublicNetworkSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allowed_ip_cidr_ranges` after provisioning.\nThe list of public IPv4, IPv6 cidr ranges that are allowed to access the bucket."]
    pub fn allowed_ip_cidr_ranges(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allowed_ip_cidr_ranges", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct StorageBucketIpFilterElVpcNetworkSourcesEl {
    allowed_ip_cidr_ranges: ListField<PrimField<String>>,
    network: PrimField<String>,
}
impl StorageBucketIpFilterElVpcNetworkSourcesEl {}
impl ToListMappable for StorageBucketIpFilterElVpcNetworkSourcesEl {
    type O = BlockAssignable<StorageBucketIpFilterElVpcNetworkSourcesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageBucketIpFilterElVpcNetworkSourcesEl {
    #[doc = "The list of public or private IPv4 and IPv6 CIDR ranges that can access the bucket."]
    pub allowed_ip_cidr_ranges: ListField<PrimField<String>>,
    #[doc = "Name of the network. Format: projects/{PROJECT_ID}/global/networks/{NETWORK_NAME}"]
    pub network: PrimField<String>,
}
impl BuildStorageBucketIpFilterElVpcNetworkSourcesEl {
    pub fn build(self) -> StorageBucketIpFilterElVpcNetworkSourcesEl {
        StorageBucketIpFilterElVpcNetworkSourcesEl {
            allowed_ip_cidr_ranges: self.allowed_ip_cidr_ranges,
            network: self.network,
        }
    }
}
pub struct StorageBucketIpFilterElVpcNetworkSourcesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageBucketIpFilterElVpcNetworkSourcesElRef {
    fn new(shared: StackShared, base: String) -> StorageBucketIpFilterElVpcNetworkSourcesElRef {
        StorageBucketIpFilterElVpcNetworkSourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageBucketIpFilterElVpcNetworkSourcesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allowed_ip_cidr_ranges` after provisioning.\nThe list of public or private IPv4 and IPv6 CIDR ranges that can access the bucket."]
    pub fn allowed_ip_cidr_ranges(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allowed_ip_cidr_ranges", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\nName of the network. Format: projects/{PROJECT_ID}/global/networks/{NETWORK_NAME}"]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }
}
#[derive(Serialize, Default)]
struct StorageBucketIpFilterElDynamic {
    public_network_source: Option<DynamicBlock<StorageBucketIpFilterElPublicNetworkSourceEl>>,
    vpc_network_sources: Option<DynamicBlock<StorageBucketIpFilterElVpcNetworkSourcesEl>>,
}
#[derive(Serialize)]
pub struct StorageBucketIpFilterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_all_service_agent_access: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_cross_org_vpcs: Option<PrimField<bool>>,
    mode: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    public_network_source: Option<Vec<StorageBucketIpFilterElPublicNetworkSourceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_network_sources: Option<Vec<StorageBucketIpFilterElVpcNetworkSourcesEl>>,
    dynamic: StorageBucketIpFilterElDynamic,
}
impl StorageBucketIpFilterEl {
    #[doc = "Set the field `allow_all_service_agent_access`.\nWhether to allow all service agents to access the bucket regardless of the IP filter configuration."]
    pub fn set_allow_all_service_agent_access(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.allow_all_service_agent_access = Some(v.into());
        self
    }
    #[doc = "Set the field `allow_cross_org_vpcs`.\nWhether to allow cross-org VPCs in the bucket's IP filter configuration."]
    pub fn set_allow_cross_org_vpcs(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.allow_cross_org_vpcs = Some(v.into());
        self
    }
    #[doc = "Set the field `public_network_source`.\n"]
    pub fn set_public_network_source(
        mut self,
        v: impl Into<BlockAssignable<StorageBucketIpFilterElPublicNetworkSourceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.public_network_source = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.public_network_source = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `vpc_network_sources`.\n"]
    pub fn set_vpc_network_sources(
        mut self,
        v: impl Into<BlockAssignable<StorageBucketIpFilterElVpcNetworkSourcesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.vpc_network_sources = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.vpc_network_sources = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for StorageBucketIpFilterEl {
    type O = BlockAssignable<StorageBucketIpFilterEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageBucketIpFilterEl {
    #[doc = "The mode of the IP filter. Valid values are 'Enabled' and 'Disabled'."]
    pub mode: PrimField<String>,
}
impl BuildStorageBucketIpFilterEl {
    pub fn build(self) -> StorageBucketIpFilterEl {
        StorageBucketIpFilterEl {
            allow_all_service_agent_access: core::default::Default::default(),
            allow_cross_org_vpcs: core::default::Default::default(),
            mode: self.mode,
            public_network_source: core::default::Default::default(),
            vpc_network_sources: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct StorageBucketIpFilterElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageBucketIpFilterElRef {
    fn new(shared: StackShared, base: String) -> StorageBucketIpFilterElRef {
        StorageBucketIpFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageBucketIpFilterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allow_all_service_agent_access` after provisioning.\nWhether to allow all service agents to access the bucket regardless of the IP filter configuration."]
    pub fn allow_all_service_agent_access(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_all_service_agent_access", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `allow_cross_org_vpcs` after provisioning.\nWhether to allow cross-org VPCs in the bucket's IP filter configuration."]
    pub fn allow_cross_org_vpcs(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_cross_org_vpcs", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `mode` after provisioning.\nThe mode of the IP filter. Valid values are 'Enabled' and 'Disabled'."]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
    #[doc = "Get a reference to the value of field `public_network_source` after provisioning.\n"]
    pub fn public_network_source(
        &self,
    ) -> ListRef<StorageBucketIpFilterElPublicNetworkSourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.public_network_source", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `vpc_network_sources` after provisioning.\n"]
    pub fn vpc_network_sources(&self) -> ListRef<StorageBucketIpFilterElVpcNetworkSourcesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vpc_network_sources", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct StorageBucketLifecycleRuleElActionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_class: Option<PrimField<String>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}
impl StorageBucketLifecycleRuleElActionEl {
    #[doc = "Set the field `storage_class`.\nThe target Storage Class of objects affected by this Lifecycle Rule. Supported values include: MULTI_REGIONAL, REGIONAL, NEARLINE, COLDLINE, ARCHIVE."]
    pub fn set_storage_class(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.storage_class = Some(v.into());
        self
    }
}
impl ToListMappable for StorageBucketLifecycleRuleElActionEl {
    type O = BlockAssignable<StorageBucketLifecycleRuleElActionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageBucketLifecycleRuleElActionEl {
    #[doc = "The type of the action of this Lifecycle Rule. Supported values include: Delete, SetStorageClass and AbortIncompleteMultipartUpload."]
    pub type_: PrimField<String>,
}
impl BuildStorageBucketLifecycleRuleElActionEl {
    pub fn build(self) -> StorageBucketLifecycleRuleElActionEl {
        StorageBucketLifecycleRuleElActionEl {
            storage_class: core::default::Default::default(),
            type_: self.type_,
        }
    }
}
pub struct StorageBucketLifecycleRuleElActionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageBucketLifecycleRuleElActionElRef {
    fn new(shared: StackShared, base: String) -> StorageBucketLifecycleRuleElActionElRef {
        StorageBucketLifecycleRuleElActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageBucketLifecycleRuleElActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `storage_class` after provisioning.\nThe target Storage Class of objects affected by this Lifecycle Rule. Supported values include: MULTI_REGIONAL, REGIONAL, NEARLINE, COLDLINE, ARCHIVE."]
    pub fn storage_class(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.storage_class", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe type of the action of this Lifecycle Rule. Supported values include: Delete, SetStorageClass and AbortIncompleteMultipartUpload."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct StorageBucketLifecycleRuleElConditionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    age: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    created_before: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_time_before: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    days_since_custom_time: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    days_since_noncurrent_time: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    matches_prefix: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    matches_storage_class: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    matches_suffix: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    noncurrent_time_before: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    num_newer_versions: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    send_age_if_zero: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    send_days_since_custom_time_if_zero: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    send_days_since_noncurrent_time_if_zero: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    send_num_newer_versions_if_zero: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    with_state: Option<PrimField<String>>,
}
impl StorageBucketLifecycleRuleElConditionEl {
    #[doc = "Set the field `age`.\nMinimum age of an object in days to satisfy this condition."]
    pub fn set_age(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.age = Some(v.into());
        self
    }
    #[doc = "Set the field `created_before`.\nCreation date of an object in RFC 3339 (e.g. 2017-06-13) to satisfy this condition."]
    pub fn set_created_before(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.created_before = Some(v.into());
        self
    }
    #[doc = "Set the field `custom_time_before`.\nCreation date of an object in RFC 3339 (e.g. 2017-06-13) to satisfy this condition."]
    pub fn set_custom_time_before(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.custom_time_before = Some(v.into());
        self
    }
    #[doc = "Set the field `days_since_custom_time`.\nNumber of days elapsed since the user-specified timestamp set on an object."]
    pub fn set_days_since_custom_time(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.days_since_custom_time = Some(v.into());
        self
    }
    #[doc = "Set the field `days_since_noncurrent_time`.\nNumber of days elapsed since the noncurrent timestamp of an object. This\n\t\t\t\t\t\t\t\t\t\tcondition is relevant only for versioned objects."]
    pub fn set_days_since_noncurrent_time(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.days_since_noncurrent_time = Some(v.into());
        self
    }
    #[doc = "Set the field `matches_prefix`.\nOne or more matching name prefixes to satisfy this condition."]
    pub fn set_matches_prefix(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.matches_prefix = Some(v.into());
        self
    }
    #[doc = "Set the field `matches_storage_class`.\nStorage Class of objects to satisfy this condition. Supported values include: MULTI_REGIONAL, REGIONAL, NEARLINE, COLDLINE, ARCHIVE, STANDARD, DURABLE_REDUCED_AVAILABILITY."]
    pub fn set_matches_storage_class(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.matches_storage_class = Some(v.into());
        self
    }
    #[doc = "Set the field `matches_suffix`.\nOne or more matching name suffixes to satisfy this condition."]
    pub fn set_matches_suffix(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.matches_suffix = Some(v.into());
        self
    }
    #[doc = "Set the field `noncurrent_time_before`.\nCreation date of an object in RFC 3339 (e.g. 2017-06-13) to satisfy this condition."]
    pub fn set_noncurrent_time_before(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.noncurrent_time_before = Some(v.into());
        self
    }
    #[doc = "Set the field `num_newer_versions`.\nRelevant only for versioned objects. The number of newer versions of an object to satisfy this condition."]
    pub fn set_num_newer_versions(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.num_newer_versions = Some(v.into());
        self
    }
    #[doc = "Set the field `send_age_if_zero`.\nWhile set true, age value will be sent in the request even for zero value of the field. This field is only useful for setting 0 value to the age field. It can be used alone or together with age."]
    pub fn set_send_age_if_zero(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.send_age_if_zero = Some(v.into());
        self
    }
    #[doc = "Set the field `send_days_since_custom_time_if_zero`.\nWhile set true, days_since_custom_time value will be sent in the request even for zero value of the field. This field is only useful for setting 0 value to the days_since_custom_time field. It can be used alone or together with days_since_custom_time."]
    pub fn set_send_days_since_custom_time_if_zero(
        mut self,
        v: impl Into<PrimField<bool>>,
    ) -> Self {
        self.send_days_since_custom_time_if_zero = Some(v.into());
        self
    }
    #[doc = "Set the field `send_days_since_noncurrent_time_if_zero`.\nWhile set true, days_since_noncurrent_time value will be sent in the request even for zero value of the field. This field is only useful for setting 0 value to the days_since_noncurrent_time field. It can be used alone or together with days_since_noncurrent_time."]
    pub fn set_send_days_since_noncurrent_time_if_zero(
        mut self,
        v: impl Into<PrimField<bool>>,
    ) -> Self {
        self.send_days_since_noncurrent_time_if_zero = Some(v.into());
        self
    }
    #[doc = "Set the field `send_num_newer_versions_if_zero`.\nWhile set true, num_newer_versions value will be sent in the request even for zero value of the field. This field is only useful for setting 0 value to the num_newer_versions field. It can be used alone or together with num_newer_versions."]
    pub fn set_send_num_newer_versions_if_zero(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.send_num_newer_versions_if_zero = Some(v.into());
        self
    }
    #[doc = "Set the field `with_state`.\nMatch to live and/or archived objects. Unversioned buckets have only live objects. Supported values include: \"LIVE\", \"ARCHIVED\", \"ANY\"."]
    pub fn set_with_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.with_state = Some(v.into());
        self
    }
}
impl ToListMappable for StorageBucketLifecycleRuleElConditionEl {
    type O = BlockAssignable<StorageBucketLifecycleRuleElConditionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageBucketLifecycleRuleElConditionEl {}
impl BuildStorageBucketLifecycleRuleElConditionEl {
    pub fn build(self) -> StorageBucketLifecycleRuleElConditionEl {
        StorageBucketLifecycleRuleElConditionEl {
            age: core::default::Default::default(),
            created_before: core::default::Default::default(),
            custom_time_before: core::default::Default::default(),
            days_since_custom_time: core::default::Default::default(),
            days_since_noncurrent_time: core::default::Default::default(),
            matches_prefix: core::default::Default::default(),
            matches_storage_class: core::default::Default::default(),
            matches_suffix: core::default::Default::default(),
            noncurrent_time_before: core::default::Default::default(),
            num_newer_versions: core::default::Default::default(),
            send_age_if_zero: core::default::Default::default(),
            send_days_since_custom_time_if_zero: core::default::Default::default(),
            send_days_since_noncurrent_time_if_zero: core::default::Default::default(),
            send_num_newer_versions_if_zero: core::default::Default::default(),
            with_state: core::default::Default::default(),
        }
    }
}
pub struct StorageBucketLifecycleRuleElConditionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageBucketLifecycleRuleElConditionElRef {
    fn new(shared: StackShared, base: String) -> StorageBucketLifecycleRuleElConditionElRef {
        StorageBucketLifecycleRuleElConditionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageBucketLifecycleRuleElConditionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `age` after provisioning.\nMinimum age of an object in days to satisfy this condition."]
    pub fn age(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.age", self.base))
    }
    #[doc = "Get a reference to the value of field `created_before` after provisioning.\nCreation date of an object in RFC 3339 (e.g. 2017-06-13) to satisfy this condition."]
    pub fn created_before(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created_before", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `custom_time_before` after provisioning.\nCreation date of an object in RFC 3339 (e.g. 2017-06-13) to satisfy this condition."]
    pub fn custom_time_before(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.custom_time_before", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `days_since_custom_time` after provisioning.\nNumber of days elapsed since the user-specified timestamp set on an object."]
    pub fn days_since_custom_time(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.days_since_custom_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `days_since_noncurrent_time` after provisioning.\nNumber of days elapsed since the noncurrent timestamp of an object. This\n\t\t\t\t\t\t\t\t\t\tcondition is relevant only for versioned objects."]
    pub fn days_since_noncurrent_time(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.days_since_noncurrent_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `matches_prefix` after provisioning.\nOne or more matching name prefixes to satisfy this condition."]
    pub fn matches_prefix(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.matches_prefix", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `matches_storage_class` after provisioning.\nStorage Class of objects to satisfy this condition. Supported values include: MULTI_REGIONAL, REGIONAL, NEARLINE, COLDLINE, ARCHIVE, STANDARD, DURABLE_REDUCED_AVAILABILITY."]
    pub fn matches_storage_class(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.matches_storage_class", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `matches_suffix` after provisioning.\nOne or more matching name suffixes to satisfy this condition."]
    pub fn matches_suffix(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.matches_suffix", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `noncurrent_time_before` after provisioning.\nCreation date of an object in RFC 3339 (e.g. 2017-06-13) to satisfy this condition."]
    pub fn noncurrent_time_before(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.noncurrent_time_before", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `num_newer_versions` after provisioning.\nRelevant only for versioned objects. The number of newer versions of an object to satisfy this condition."]
    pub fn num_newer_versions(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.num_newer_versions", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `send_age_if_zero` after provisioning.\nWhile set true, age value will be sent in the request even for zero value of the field. This field is only useful for setting 0 value to the age field. It can be used alone or together with age."]
    pub fn send_age_if_zero(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.send_age_if_zero", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `send_days_since_custom_time_if_zero` after provisioning.\nWhile set true, days_since_custom_time value will be sent in the request even for zero value of the field. This field is only useful for setting 0 value to the days_since_custom_time field. It can be used alone or together with days_since_custom_time."]
    pub fn send_days_since_custom_time_if_zero(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.send_days_since_custom_time_if_zero", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `send_days_since_noncurrent_time_if_zero` after provisioning.\nWhile set true, days_since_noncurrent_time value will be sent in the request even for zero value of the field. This field is only useful for setting 0 value to the days_since_noncurrent_time field. It can be used alone or together with days_since_noncurrent_time."]
    pub fn send_days_since_noncurrent_time_if_zero(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.send_days_since_noncurrent_time_if_zero", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `send_num_newer_versions_if_zero` after provisioning.\nWhile set true, num_newer_versions value will be sent in the request even for zero value of the field. This field is only useful for setting 0 value to the num_newer_versions field. It can be used alone or together with num_newer_versions."]
    pub fn send_num_newer_versions_if_zero(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.send_num_newer_versions_if_zero", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `with_state` after provisioning.\nMatch to live and/or archived objects. Unversioned buckets have only live objects. Supported values include: \"LIVE\", \"ARCHIVED\", \"ANY\"."]
    pub fn with_state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.with_state", self.base))
    }
}
#[derive(Serialize, Default)]
struct StorageBucketLifecycleRuleElDynamic {
    action: Option<DynamicBlock<StorageBucketLifecycleRuleElActionEl>>,
    condition: Option<DynamicBlock<StorageBucketLifecycleRuleElConditionEl>>,
}
#[derive(Serialize)]
pub struct StorageBucketLifecycleRuleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<Vec<StorageBucketLifecycleRuleElActionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    condition: Option<Vec<StorageBucketLifecycleRuleElConditionEl>>,
    dynamic: StorageBucketLifecycleRuleElDynamic,
}
impl StorageBucketLifecycleRuleEl {
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(
        mut self,
        v: impl Into<BlockAssignable<StorageBucketLifecycleRuleElActionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.action = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.action = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `condition`.\n"]
    pub fn set_condition(
        mut self,
        v: impl Into<BlockAssignable<StorageBucketLifecycleRuleElConditionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.condition = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.condition = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for StorageBucketLifecycleRuleEl {
    type O = BlockAssignable<StorageBucketLifecycleRuleEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageBucketLifecycleRuleEl {}
impl BuildStorageBucketLifecycleRuleEl {
    pub fn build(self) -> StorageBucketLifecycleRuleEl {
        StorageBucketLifecycleRuleEl {
            action: core::default::Default::default(),
            condition: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct StorageBucketLifecycleRuleElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageBucketLifecycleRuleElRef {
    fn new(shared: StackShared, base: String) -> StorageBucketLifecycleRuleElRef {
        StorageBucketLifecycleRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageBucketLifecycleRuleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct StorageBucketLoggingEl {
    log_bucket: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_object_prefix: Option<PrimField<String>>,
}
impl StorageBucketLoggingEl {
    #[doc = "Set the field `log_object_prefix`.\nThe object prefix for log objects. If it's not provided, by default Google Cloud Storage sets this to this bucket's name."]
    pub fn set_log_object_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.log_object_prefix = Some(v.into());
        self
    }
}
impl ToListMappable for StorageBucketLoggingEl {
    type O = BlockAssignable<StorageBucketLoggingEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageBucketLoggingEl {
    #[doc = "The bucket that will receive log objects."]
    pub log_bucket: PrimField<String>,
}
impl BuildStorageBucketLoggingEl {
    pub fn build(self) -> StorageBucketLoggingEl {
        StorageBucketLoggingEl {
            log_bucket: self.log_bucket,
            log_object_prefix: core::default::Default::default(),
        }
    }
}
pub struct StorageBucketLoggingElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageBucketLoggingElRef {
    fn new(shared: StackShared, base: String) -> StorageBucketLoggingElRef {
        StorageBucketLoggingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageBucketLoggingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `log_bucket` after provisioning.\nThe bucket that will receive log objects."]
    pub fn log_bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_bucket", self.base))
    }
    #[doc = "Get a reference to the value of field `log_object_prefix` after provisioning.\nThe object prefix for log objects. If it's not provided, by default Google Cloud Storage sets this to this bucket's name."]
    pub fn log_object_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.log_object_prefix", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct StorageBucketRetentionPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    is_locked: Option<PrimField<bool>>,
    retention_period: PrimField<String>,
}
impl StorageBucketRetentionPolicyEl {
    #[doc = "Set the field `is_locked`.\nIf set to true, the bucket will be locked and permanently restrict edits to the bucket's retention policy.  Caution: Locking a bucket is an irreversible action."]
    pub fn set_is_locked(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.is_locked = Some(v.into());
        self
    }
}
impl ToListMappable for StorageBucketRetentionPolicyEl {
    type O = BlockAssignable<StorageBucketRetentionPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageBucketRetentionPolicyEl {
    #[doc = "The period of time, in seconds, that objects in the bucket must be retained and cannot be deleted, overwritten, or archived. The value must be less than 3,155,760,000 seconds."]
    pub retention_period: PrimField<String>,
}
impl BuildStorageBucketRetentionPolicyEl {
    pub fn build(self) -> StorageBucketRetentionPolicyEl {
        StorageBucketRetentionPolicyEl {
            is_locked: core::default::Default::default(),
            retention_period: self.retention_period,
        }
    }
}
pub struct StorageBucketRetentionPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageBucketRetentionPolicyElRef {
    fn new(shared: StackShared, base: String) -> StorageBucketRetentionPolicyElRef {
        StorageBucketRetentionPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageBucketRetentionPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `is_locked` after provisioning.\nIf set to true, the bucket will be locked and permanently restrict edits to the bucket's retention policy.  Caution: Locking a bucket is an irreversible action."]
    pub fn is_locked(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.is_locked", self.base))
    }
    #[doc = "Get a reference to the value of field `retention_period` after provisioning.\nThe period of time, in seconds, that objects in the bucket must be retained and cannot be deleted, overwritten, or archived. The value must be less than 3,155,760,000 seconds."]
    pub fn retention_period(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.retention_period", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct StorageBucketSoftDeletePolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    retention_duration_seconds: Option<PrimField<f64>>,
}
impl StorageBucketSoftDeletePolicyEl {
    #[doc = "Set the field `retention_duration_seconds`.\nThe duration in seconds that soft-deleted objects in the bucket will be retained and cannot be permanently deleted. Default value is 604800."]
    pub fn set_retention_duration_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.retention_duration_seconds = Some(v.into());
        self
    }
}
impl ToListMappable for StorageBucketSoftDeletePolicyEl {
    type O = BlockAssignable<StorageBucketSoftDeletePolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageBucketSoftDeletePolicyEl {}
impl BuildStorageBucketSoftDeletePolicyEl {
    pub fn build(self) -> StorageBucketSoftDeletePolicyEl {
        StorageBucketSoftDeletePolicyEl {
            retention_duration_seconds: core::default::Default::default(),
        }
    }
}
pub struct StorageBucketSoftDeletePolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageBucketSoftDeletePolicyElRef {
    fn new(shared: StackShared, base: String) -> StorageBucketSoftDeletePolicyElRef {
        StorageBucketSoftDeletePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageBucketSoftDeletePolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `effective_time` after provisioning.\nServer-determined value that indicates the time from which the policy, or one with a greater retention, was effective. This value is in RFC 3339 format."]
    pub fn effective_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.effective_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `retention_duration_seconds` after provisioning.\nThe duration in seconds that soft-deleted objects in the bucket will be retained and cannot be permanently deleted. Default value is 604800."]
    pub fn retention_duration_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.retention_duration_seconds", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct StorageBucketTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    read: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl StorageBucketTimeoutsEl {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }
    #[doc = "Set the field `read`.\n"]
    pub fn set_read(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.read = Some(v.into());
        self
    }
    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}
impl ToListMappable for StorageBucketTimeoutsEl {
    type O = BlockAssignable<StorageBucketTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageBucketTimeoutsEl {}
impl BuildStorageBucketTimeoutsEl {
    pub fn build(self) -> StorageBucketTimeoutsEl {
        StorageBucketTimeoutsEl {
            create: core::default::Default::default(),
            read: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct StorageBucketTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageBucketTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> StorageBucketTimeoutsElRef {
        StorageBucketTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageBucketTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }
    #[doc = "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }
    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
#[derive(Serialize)]
pub struct StorageBucketVersioningEl {
    enabled: PrimField<bool>,
}
impl StorageBucketVersioningEl {}
impl ToListMappable for StorageBucketVersioningEl {
    type O = BlockAssignable<StorageBucketVersioningEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageBucketVersioningEl {
    #[doc = "While set to true, versioning is fully enabled for this bucket."]
    pub enabled: PrimField<bool>,
}
impl BuildStorageBucketVersioningEl {
    pub fn build(self) -> StorageBucketVersioningEl {
        StorageBucketVersioningEl {
            enabled: self.enabled,
        }
    }
}
pub struct StorageBucketVersioningElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageBucketVersioningElRef {
    fn new(shared: StackShared, base: String) -> StorageBucketVersioningElRef {
        StorageBucketVersioningElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageBucketVersioningElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nWhile set to true, versioning is fully enabled for this bucket."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize)]
pub struct StorageBucketWebsiteEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    main_page_suffix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    not_found_page: Option<PrimField<String>>,
}
impl StorageBucketWebsiteEl {
    #[doc = "Set the field `main_page_suffix`.\nBehaves as the bucket's directory index where missing objects are treated as potential directories."]
    pub fn set_main_page_suffix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.main_page_suffix = Some(v.into());
        self
    }
    #[doc = "Set the field `not_found_page`.\nThe custom object to return when a requested resource is not found."]
    pub fn set_not_found_page(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.not_found_page = Some(v.into());
        self
    }
}
impl ToListMappable for StorageBucketWebsiteEl {
    type O = BlockAssignable<StorageBucketWebsiteEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageBucketWebsiteEl {}
impl BuildStorageBucketWebsiteEl {
    pub fn build(self) -> StorageBucketWebsiteEl {
        StorageBucketWebsiteEl {
            main_page_suffix: core::default::Default::default(),
            not_found_page: core::default::Default::default(),
        }
    }
}
pub struct StorageBucketWebsiteElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageBucketWebsiteElRef {
    fn new(shared: StackShared, base: String) -> StorageBucketWebsiteElRef {
        StorageBucketWebsiteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageBucketWebsiteElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `main_page_suffix` after provisioning.\nBehaves as the bucket's directory index where missing objects are treated as potential directories."]
    pub fn main_page_suffix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.main_page_suffix", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `not_found_page` after provisioning.\nThe custom object to return when a requested resource is not found."]
    pub fn not_found_page(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.not_found_page", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct StorageBucketDynamic {
    autoclass: Option<DynamicBlock<StorageBucketAutoclassEl>>,
    cors: Option<DynamicBlock<StorageBucketCorsEl>>,
    custom_placement_config: Option<DynamicBlock<StorageBucketCustomPlacementConfigEl>>,
    encryption: Option<DynamicBlock<StorageBucketEncryptionEl>>,
    hierarchical_namespace: Option<DynamicBlock<StorageBucketHierarchicalNamespaceEl>>,
    ip_filter: Option<DynamicBlock<StorageBucketIpFilterEl>>,
    lifecycle_rule: Option<DynamicBlock<StorageBucketLifecycleRuleEl>>,
    logging: Option<DynamicBlock<StorageBucketLoggingEl>>,
    retention_policy: Option<DynamicBlock<StorageBucketRetentionPolicyEl>>,
    soft_delete_policy: Option<DynamicBlock<StorageBucketSoftDeletePolicyEl>>,
    versioning: Option<DynamicBlock<StorageBucketVersioningEl>>,
    website: Option<DynamicBlock<StorageBucketWebsiteEl>>,
}
