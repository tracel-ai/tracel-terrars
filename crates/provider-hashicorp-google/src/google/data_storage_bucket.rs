use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataStorageBucketData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
}
struct DataStorageBucket_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataStorageBucketData>,
}
#[derive(Clone)]
pub struct DataStorageBucket(Rc<DataStorageBucket_>);
impl DataStorageBucket {
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
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `autoclass` after provisioning.\nThe bucket's autoclass configuration."]
    pub fn autoclass(&self) -> ListRef<DataStorageBucketAutoclassElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.autoclass", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cors` after provisioning.\nThe bucket's Cross-Origin Resource Sharing (CORS) configuration."]
    pub fn cors(&self) -> ListRef<DataStorageBucketCorsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cors", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `custom_placement_config` after provisioning.\nThe bucket's custom location configuration, which specifies the individual regions that comprise a dual-region bucket. If the bucket is designated a single or multi-region, the parameters are empty."]
    pub fn custom_placement_config(&self) -> ListRef<DataStorageBucketCustomPlacementConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_placement_config", self.extract_ref()),
        )
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
    #[doc = "Get a reference to the value of field `encryption` after provisioning.\nThe bucket's encryption configuration."]
    pub fn encryption(&self) -> ListRef<DataStorageBucketEncryptionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.encryption", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `force_destroy` after provisioning.\nWhen true, before deleting a bucket, delete all objects within the bucket, or Anywhere Caches caching data for that bucket. Otherwise, buckets with objects/caches will fail. Anywhere Cache requires additional permissions to interact with and will be ignored when those are not present, attempting to delete anyways. This may result in the objects in the bucket getting destroyed but not the bucket itself if there is a cache in use with the bucket. Force deletion may take a long time to delete buckets with lots of objects or with any Anywhere Caches (80m+)."]
    pub fn force_destroy(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.force_destroy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `hierarchical_namespace` after provisioning.\nThe bucket's HNS configuration, which defines bucket can organize folders in logical file system structure."]
    pub fn hierarchical_namespace(&self) -> ListRef<DataStorageBucketHierarchicalNamespaceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.hierarchical_namespace", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `ip_filter` after provisioning.\nThe bucket IP filtering configuration."]
    pub fn ip_filter(&self) -> ListRef<DataStorageBucketIpFilterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ip_filter", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nA set of key/value label pairs to assign to the bucket."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `lifecycle_rule` after provisioning.\nThe bucket's Lifecycle Rules configuration."]
    pub fn lifecycle_rule(&self) -> ListRef<DataStorageBucketLifecycleRuleElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.lifecycle_rule", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe Google Cloud Storage location or region."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `logging` after provisioning.\nThe bucket's Access & Storage Logs configuration."]
    pub fn logging(&self) -> ListRef<DataStorageBucketLoggingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.logging", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `retention_policy` after provisioning.\nConfiguration of the bucket's data retention policy for how long objects in the bucket should be retained."]
    pub fn retention_policy(&self) -> ListRef<DataStorageBucketRetentionPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.retention_policy", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `soft_delete_policy` after provisioning.\nThe bucket's soft delete policy, which defines the period of time that soft-deleted objects will be retained, and cannot be permanently deleted. If it is not provided, by default Google Cloud Storage sets this to default soft delete policy"]
    pub fn soft_delete_policy(&self) -> ListRef<DataStorageBucketSoftDeletePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.soft_delete_policy", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `versioning` after provisioning.\nThe bucket's Versioning configuration."]
    pub fn versioning(&self) -> ListRef<DataStorageBucketVersioningElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.versioning", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `website` after provisioning.\nConfiguration if the bucket acts as a website."]
    pub fn website(&self) -> ListRef<DataStorageBucketWebsiteElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.website", self.extract_ref()),
        )
    }
}
impl Referable for DataStorageBucket {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataStorageBucket {}
impl ToListMappable for DataStorageBucket {
    type O = ListRef<DataStorageBucketRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataStorageBucket_ {
    fn extract_datasource_type(&self) -> String {
        "google_storage_bucket".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataStorageBucket {
    pub tf_id: String,
    #[doc = "The name of the bucket."]
    pub name: PrimField<String>,
}
impl BuildDataStorageBucket {
    pub fn build(self, stack: &mut Stack) -> DataStorageBucket {
        let out = DataStorageBucket(Rc::new(DataStorageBucket_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataStorageBucketData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataStorageBucketRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataStorageBucketRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataStorageBucketRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `autoclass` after provisioning.\nThe bucket's autoclass configuration."]
    pub fn autoclass(&self) -> ListRef<DataStorageBucketAutoclassElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.autoclass", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cors` after provisioning.\nThe bucket's Cross-Origin Resource Sharing (CORS) configuration."]
    pub fn cors(&self) -> ListRef<DataStorageBucketCorsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cors", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `custom_placement_config` after provisioning.\nThe bucket's custom location configuration, which specifies the individual regions that comprise a dual-region bucket. If the bucket is designated a single or multi-region, the parameters are empty."]
    pub fn custom_placement_config(&self) -> ListRef<DataStorageBucketCustomPlacementConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_placement_config", self.extract_ref()),
        )
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
    #[doc = "Get a reference to the value of field `encryption` after provisioning.\nThe bucket's encryption configuration."]
    pub fn encryption(&self) -> ListRef<DataStorageBucketEncryptionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.encryption", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `force_destroy` after provisioning.\nWhen true, before deleting a bucket, delete all objects within the bucket, or Anywhere Caches caching data for that bucket. Otherwise, buckets with objects/caches will fail. Anywhere Cache requires additional permissions to interact with and will be ignored when those are not present, attempting to delete anyways. This may result in the objects in the bucket getting destroyed but not the bucket itself if there is a cache in use with the bucket. Force deletion may take a long time to delete buckets with lots of objects or with any Anywhere Caches (80m+)."]
    pub fn force_destroy(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.force_destroy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `hierarchical_namespace` after provisioning.\nThe bucket's HNS configuration, which defines bucket can organize folders in logical file system structure."]
    pub fn hierarchical_namespace(&self) -> ListRef<DataStorageBucketHierarchicalNamespaceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.hierarchical_namespace", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `ip_filter` after provisioning.\nThe bucket IP filtering configuration."]
    pub fn ip_filter(&self) -> ListRef<DataStorageBucketIpFilterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ip_filter", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nA set of key/value label pairs to assign to the bucket."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `lifecycle_rule` after provisioning.\nThe bucket's Lifecycle Rules configuration."]
    pub fn lifecycle_rule(&self) -> ListRef<DataStorageBucketLifecycleRuleElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.lifecycle_rule", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe Google Cloud Storage location or region."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `logging` after provisioning.\nThe bucket's Access & Storage Logs configuration."]
    pub fn logging(&self) -> ListRef<DataStorageBucketLoggingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.logging", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `retention_policy` after provisioning.\nConfiguration of the bucket's data retention policy for how long objects in the bucket should be retained."]
    pub fn retention_policy(&self) -> ListRef<DataStorageBucketRetentionPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.retention_policy", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `soft_delete_policy` after provisioning.\nThe bucket's soft delete policy, which defines the period of time that soft-deleted objects will be retained, and cannot be permanently deleted. If it is not provided, by default Google Cloud Storage sets this to default soft delete policy"]
    pub fn soft_delete_policy(&self) -> ListRef<DataStorageBucketSoftDeletePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.soft_delete_policy", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `versioning` after provisioning.\nThe bucket's Versioning configuration."]
    pub fn versioning(&self) -> ListRef<DataStorageBucketVersioningElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.versioning", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `website` after provisioning.\nConfiguration if the bucket acts as a website."]
    pub fn website(&self) -> ListRef<DataStorageBucketWebsiteElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.website", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataStorageBucketAutoclassEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    terminal_storage_class: Option<PrimField<String>>,
}
impl DataStorageBucketAutoclassEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `terminal_storage_class`.\n"]
    pub fn set_terminal_storage_class(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.terminal_storage_class = Some(v.into());
        self
    }
}
impl ToListMappable for DataStorageBucketAutoclassEl {
    type O = BlockAssignable<DataStorageBucketAutoclassEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataStorageBucketAutoclassEl {}
impl BuildDataStorageBucketAutoclassEl {
    pub fn build(self) -> DataStorageBucketAutoclassEl {
        DataStorageBucketAutoclassEl {
            enabled: core::default::Default::default(),
            terminal_storage_class: core::default::Default::default(),
        }
    }
}
pub struct DataStorageBucketAutoclassElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataStorageBucketAutoclassElRef {
    fn new(shared: StackShared, base: String) -> DataStorageBucketAutoclassElRef {
        DataStorageBucketAutoclassElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataStorageBucketAutoclassElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
    #[doc = "Get a reference to the value of field `terminal_storage_class` after provisioning.\n"]
    pub fn terminal_storage_class(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.terminal_storage_class", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataStorageBucketCorsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_age_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    method: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    origin: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_header: Option<ListField<PrimField<String>>>,
}
impl DataStorageBucketCorsEl {
    #[doc = "Set the field `max_age_seconds`.\n"]
    pub fn set_max_age_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_age_seconds = Some(v.into());
        self
    }
    #[doc = "Set the field `method`.\n"]
    pub fn set_method(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.method = Some(v.into());
        self
    }
    #[doc = "Set the field `origin`.\n"]
    pub fn set_origin(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.origin = Some(v.into());
        self
    }
    #[doc = "Set the field `response_header`.\n"]
    pub fn set_response_header(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.response_header = Some(v.into());
        self
    }
}
impl ToListMappable for DataStorageBucketCorsEl {
    type O = BlockAssignable<DataStorageBucketCorsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataStorageBucketCorsEl {}
impl BuildDataStorageBucketCorsEl {
    pub fn build(self) -> DataStorageBucketCorsEl {
        DataStorageBucketCorsEl {
            max_age_seconds: core::default::Default::default(),
            method: core::default::Default::default(),
            origin: core::default::Default::default(),
            response_header: core::default::Default::default(),
        }
    }
}
pub struct DataStorageBucketCorsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataStorageBucketCorsElRef {
    fn new(shared: StackShared, base: String) -> DataStorageBucketCorsElRef {
        DataStorageBucketCorsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataStorageBucketCorsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `max_age_seconds` after provisioning.\n"]
    pub fn max_age_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_age_seconds", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `method` after provisioning.\n"]
    pub fn method(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.method", self.base))
    }
    #[doc = "Get a reference to the value of field `origin` after provisioning.\n"]
    pub fn origin(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.origin", self.base))
    }
    #[doc = "Get a reference to the value of field `response_header` after provisioning.\n"]
    pub fn response_header(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.response_header", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataStorageBucketCustomPlacementConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    data_locations: Option<SetField<PrimField<String>>>,
}
impl DataStorageBucketCustomPlacementConfigEl {
    #[doc = "Set the field `data_locations`.\n"]
    pub fn set_data_locations(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.data_locations = Some(v.into());
        self
    }
}
impl ToListMappable for DataStorageBucketCustomPlacementConfigEl {
    type O = BlockAssignable<DataStorageBucketCustomPlacementConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataStorageBucketCustomPlacementConfigEl {}
impl BuildDataStorageBucketCustomPlacementConfigEl {
    pub fn build(self) -> DataStorageBucketCustomPlacementConfigEl {
        DataStorageBucketCustomPlacementConfigEl {
            data_locations: core::default::Default::default(),
        }
    }
}
pub struct DataStorageBucketCustomPlacementConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataStorageBucketCustomPlacementConfigElRef {
    fn new(shared: StackShared, base: String) -> DataStorageBucketCustomPlacementConfigElRef {
        DataStorageBucketCustomPlacementConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataStorageBucketCustomPlacementConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `data_locations` after provisioning.\n"]
    pub fn data_locations(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.data_locations", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataStorageBucketEncryptionElCustomerManagedEncryptionEnforcementConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    effective_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    restriction_mode: Option<PrimField<String>>,
}
impl DataStorageBucketEncryptionElCustomerManagedEncryptionEnforcementConfigEl {
    #[doc = "Set the field `effective_time`.\n"]
    pub fn set_effective_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.effective_time = Some(v.into());
        self
    }
    #[doc = "Set the field `restriction_mode`.\n"]
    pub fn set_restriction_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.restriction_mode = Some(v.into());
        self
    }
}
impl ToListMappable for DataStorageBucketEncryptionElCustomerManagedEncryptionEnforcementConfigEl {
    type O =
        BlockAssignable<DataStorageBucketEncryptionElCustomerManagedEncryptionEnforcementConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataStorageBucketEncryptionElCustomerManagedEncryptionEnforcementConfigEl {}
impl BuildDataStorageBucketEncryptionElCustomerManagedEncryptionEnforcementConfigEl {
    pub fn build(
        self,
    ) -> DataStorageBucketEncryptionElCustomerManagedEncryptionEnforcementConfigEl {
        DataStorageBucketEncryptionElCustomerManagedEncryptionEnforcementConfigEl {
            effective_time: core::default::Default::default(),
            restriction_mode: core::default::Default::default(),
        }
    }
}
pub struct DataStorageBucketEncryptionElCustomerManagedEncryptionEnforcementConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataStorageBucketEncryptionElCustomerManagedEncryptionEnforcementConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataStorageBucketEncryptionElCustomerManagedEncryptionEnforcementConfigElRef {
        DataStorageBucketEncryptionElCustomerManagedEncryptionEnforcementConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataStorageBucketEncryptionElCustomerManagedEncryptionEnforcementConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `effective_time` after provisioning.\n"]
    pub fn effective_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.effective_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `restriction_mode` after provisioning.\n"]
    pub fn restriction_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.restriction_mode", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataStorageBucketEncryptionElCustomerSuppliedEncryptionEnforcementConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    effective_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    restriction_mode: Option<PrimField<String>>,
}
impl DataStorageBucketEncryptionElCustomerSuppliedEncryptionEnforcementConfigEl {
    #[doc = "Set the field `effective_time`.\n"]
    pub fn set_effective_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.effective_time = Some(v.into());
        self
    }
    #[doc = "Set the field `restriction_mode`.\n"]
    pub fn set_restriction_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.restriction_mode = Some(v.into());
        self
    }
}
impl ToListMappable for DataStorageBucketEncryptionElCustomerSuppliedEncryptionEnforcementConfigEl {
    type O =
        BlockAssignable<DataStorageBucketEncryptionElCustomerSuppliedEncryptionEnforcementConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataStorageBucketEncryptionElCustomerSuppliedEncryptionEnforcementConfigEl {}
impl BuildDataStorageBucketEncryptionElCustomerSuppliedEncryptionEnforcementConfigEl {
    pub fn build(
        self,
    ) -> DataStorageBucketEncryptionElCustomerSuppliedEncryptionEnforcementConfigEl {
        DataStorageBucketEncryptionElCustomerSuppliedEncryptionEnforcementConfigEl {
            effective_time: core::default::Default::default(),
            restriction_mode: core::default::Default::default(),
        }
    }
}
pub struct DataStorageBucketEncryptionElCustomerSuppliedEncryptionEnforcementConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataStorageBucketEncryptionElCustomerSuppliedEncryptionEnforcementConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataStorageBucketEncryptionElCustomerSuppliedEncryptionEnforcementConfigElRef {
        DataStorageBucketEncryptionElCustomerSuppliedEncryptionEnforcementConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataStorageBucketEncryptionElCustomerSuppliedEncryptionEnforcementConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `effective_time` after provisioning.\n"]
    pub fn effective_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.effective_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `restriction_mode` after provisioning.\n"]
    pub fn restriction_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.restriction_mode", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataStorageBucketEncryptionElGoogleManagedEncryptionEnforcementConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    effective_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    restriction_mode: Option<PrimField<String>>,
}
impl DataStorageBucketEncryptionElGoogleManagedEncryptionEnforcementConfigEl {
    #[doc = "Set the field `effective_time`.\n"]
    pub fn set_effective_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.effective_time = Some(v.into());
        self
    }
    #[doc = "Set the field `restriction_mode`.\n"]
    pub fn set_restriction_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.restriction_mode = Some(v.into());
        self
    }
}
impl ToListMappable for DataStorageBucketEncryptionElGoogleManagedEncryptionEnforcementConfigEl {
    type O =
        BlockAssignable<DataStorageBucketEncryptionElGoogleManagedEncryptionEnforcementConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataStorageBucketEncryptionElGoogleManagedEncryptionEnforcementConfigEl {}
impl BuildDataStorageBucketEncryptionElGoogleManagedEncryptionEnforcementConfigEl {
    pub fn build(self) -> DataStorageBucketEncryptionElGoogleManagedEncryptionEnforcementConfigEl {
        DataStorageBucketEncryptionElGoogleManagedEncryptionEnforcementConfigEl {
            effective_time: core::default::Default::default(),
            restriction_mode: core::default::Default::default(),
        }
    }
}
pub struct DataStorageBucketEncryptionElGoogleManagedEncryptionEnforcementConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataStorageBucketEncryptionElGoogleManagedEncryptionEnforcementConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataStorageBucketEncryptionElGoogleManagedEncryptionEnforcementConfigElRef {
        DataStorageBucketEncryptionElGoogleManagedEncryptionEnforcementConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataStorageBucketEncryptionElGoogleManagedEncryptionEnforcementConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `effective_time` after provisioning.\n"]
    pub fn effective_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.effective_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `restriction_mode` after provisioning.\n"]
    pub fn restriction_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.restriction_mode", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataStorageBucketEncryptionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_managed_encryption_enforcement_config: Option<
        ListField<DataStorageBucketEncryptionElCustomerManagedEncryptionEnforcementConfigEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_supplied_encryption_enforcement_config: Option<
        ListField<DataStorageBucketEncryptionElCustomerSuppliedEncryptionEnforcementConfigEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_kms_key_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    google_managed_encryption_enforcement_config:
        Option<ListField<DataStorageBucketEncryptionElGoogleManagedEncryptionEnforcementConfigEl>>,
}
impl DataStorageBucketEncryptionEl {
    #[doc = "Set the field `customer_managed_encryption_enforcement_config`.\n"]
    pub fn set_customer_managed_encryption_enforcement_config(
        mut self,
        v: impl Into<
            ListField<DataStorageBucketEncryptionElCustomerManagedEncryptionEnforcementConfigEl>,
        >,
    ) -> Self {
        self.customer_managed_encryption_enforcement_config = Some(v.into());
        self
    }
    #[doc = "Set the field `customer_supplied_encryption_enforcement_config`.\n"]
    pub fn set_customer_supplied_encryption_enforcement_config(
        mut self,
        v: impl Into<
            ListField<DataStorageBucketEncryptionElCustomerSuppliedEncryptionEnforcementConfigEl>,
        >,
    ) -> Self {
        self.customer_supplied_encryption_enforcement_config = Some(v.into());
        self
    }
    #[doc = "Set the field `default_kms_key_name`.\n"]
    pub fn set_default_kms_key_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.default_kms_key_name = Some(v.into());
        self
    }
    #[doc = "Set the field `google_managed_encryption_enforcement_config`.\n"]
    pub fn set_google_managed_encryption_enforcement_config(
        mut self,
        v: impl Into<ListField<DataStorageBucketEncryptionElGoogleManagedEncryptionEnforcementConfigEl>>,
    ) -> Self {
        self.google_managed_encryption_enforcement_config = Some(v.into());
        self
    }
}
impl ToListMappable for DataStorageBucketEncryptionEl {
    type O = BlockAssignable<DataStorageBucketEncryptionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataStorageBucketEncryptionEl {}
impl BuildDataStorageBucketEncryptionEl {
    pub fn build(self) -> DataStorageBucketEncryptionEl {
        DataStorageBucketEncryptionEl {
            customer_managed_encryption_enforcement_config: core::default::Default::default(),
            customer_supplied_encryption_enforcement_config: core::default::Default::default(),
            default_kms_key_name: core::default::Default::default(),
            google_managed_encryption_enforcement_config: core::default::Default::default(),
        }
    }
}
pub struct DataStorageBucketEncryptionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataStorageBucketEncryptionElRef {
    fn new(shared: StackShared, base: String) -> DataStorageBucketEncryptionElRef {
        DataStorageBucketEncryptionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataStorageBucketEncryptionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `customer_managed_encryption_enforcement_config` after provisioning.\n"]
    pub fn customer_managed_encryption_enforcement_config(
        &self,
    ) -> ListRef<DataStorageBucketEncryptionElCustomerManagedEncryptionEnforcementConfigElRef> {
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
    ) -> ListRef<DataStorageBucketEncryptionElCustomerSuppliedEncryptionEnforcementConfigElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.customer_supplied_encryption_enforcement_config",
                self.base
            ),
        )
    }
    #[doc = "Get a reference to the value of field `default_kms_key_name` after provisioning.\n"]
    pub fn default_kms_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_kms_key_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `google_managed_encryption_enforcement_config` after provisioning.\n"]
    pub fn google_managed_encryption_enforcement_config(
        &self,
    ) -> ListRef<DataStorageBucketEncryptionElGoogleManagedEncryptionEnforcementConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.google_managed_encryption_enforcement_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataStorageBucketHierarchicalNamespaceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}
impl DataStorageBucketHierarchicalNamespaceEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}
impl ToListMappable for DataStorageBucketHierarchicalNamespaceEl {
    type O = BlockAssignable<DataStorageBucketHierarchicalNamespaceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataStorageBucketHierarchicalNamespaceEl {}
impl BuildDataStorageBucketHierarchicalNamespaceEl {
    pub fn build(self) -> DataStorageBucketHierarchicalNamespaceEl {
        DataStorageBucketHierarchicalNamespaceEl {
            enabled: core::default::Default::default(),
        }
    }
}
pub struct DataStorageBucketHierarchicalNamespaceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataStorageBucketHierarchicalNamespaceElRef {
    fn new(shared: StackShared, base: String) -> DataStorageBucketHierarchicalNamespaceElRef {
        DataStorageBucketHierarchicalNamespaceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataStorageBucketHierarchicalNamespaceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize)]
pub struct DataStorageBucketIpFilterElPublicNetworkSourceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_ip_cidr_ranges: Option<ListField<PrimField<String>>>,
}
impl DataStorageBucketIpFilterElPublicNetworkSourceEl {
    #[doc = "Set the field `allowed_ip_cidr_ranges`.\n"]
    pub fn set_allowed_ip_cidr_ranges(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.allowed_ip_cidr_ranges = Some(v.into());
        self
    }
}
impl ToListMappable for DataStorageBucketIpFilterElPublicNetworkSourceEl {
    type O = BlockAssignable<DataStorageBucketIpFilterElPublicNetworkSourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataStorageBucketIpFilterElPublicNetworkSourceEl {}
impl BuildDataStorageBucketIpFilterElPublicNetworkSourceEl {
    pub fn build(self) -> DataStorageBucketIpFilterElPublicNetworkSourceEl {
        DataStorageBucketIpFilterElPublicNetworkSourceEl {
            allowed_ip_cidr_ranges: core::default::Default::default(),
        }
    }
}
pub struct DataStorageBucketIpFilterElPublicNetworkSourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataStorageBucketIpFilterElPublicNetworkSourceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataStorageBucketIpFilterElPublicNetworkSourceElRef {
        DataStorageBucketIpFilterElPublicNetworkSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataStorageBucketIpFilterElPublicNetworkSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allowed_ip_cidr_ranges` after provisioning.\n"]
    pub fn allowed_ip_cidr_ranges(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allowed_ip_cidr_ranges", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataStorageBucketIpFilterElVpcNetworkSourcesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_ip_cidr_ranges: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<PrimField<String>>,
}
impl DataStorageBucketIpFilterElVpcNetworkSourcesEl {
    #[doc = "Set the field `allowed_ip_cidr_ranges`.\n"]
    pub fn set_allowed_ip_cidr_ranges(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.allowed_ip_cidr_ranges = Some(v.into());
        self
    }
    #[doc = "Set the field `network`.\n"]
    pub fn set_network(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network = Some(v.into());
        self
    }
}
impl ToListMappable for DataStorageBucketIpFilterElVpcNetworkSourcesEl {
    type O = BlockAssignable<DataStorageBucketIpFilterElVpcNetworkSourcesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataStorageBucketIpFilterElVpcNetworkSourcesEl {}
impl BuildDataStorageBucketIpFilterElVpcNetworkSourcesEl {
    pub fn build(self) -> DataStorageBucketIpFilterElVpcNetworkSourcesEl {
        DataStorageBucketIpFilterElVpcNetworkSourcesEl {
            allowed_ip_cidr_ranges: core::default::Default::default(),
            network: core::default::Default::default(),
        }
    }
}
pub struct DataStorageBucketIpFilterElVpcNetworkSourcesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataStorageBucketIpFilterElVpcNetworkSourcesElRef {
    fn new(shared: StackShared, base: String) -> DataStorageBucketIpFilterElVpcNetworkSourcesElRef {
        DataStorageBucketIpFilterElVpcNetworkSourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataStorageBucketIpFilterElVpcNetworkSourcesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allowed_ip_cidr_ranges` after provisioning.\n"]
    pub fn allowed_ip_cidr_ranges(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allowed_ip_cidr_ranges", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\n"]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }
}
#[derive(Serialize)]
pub struct DataStorageBucketIpFilterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_all_service_agent_access: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_cross_org_vpcs: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    public_network_source: Option<ListField<DataStorageBucketIpFilterElPublicNetworkSourceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_network_sources: Option<ListField<DataStorageBucketIpFilterElVpcNetworkSourcesEl>>,
}
impl DataStorageBucketIpFilterEl {
    #[doc = "Set the field `allow_all_service_agent_access`.\n"]
    pub fn set_allow_all_service_agent_access(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.allow_all_service_agent_access = Some(v.into());
        self
    }
    #[doc = "Set the field `allow_cross_org_vpcs`.\n"]
    pub fn set_allow_cross_org_vpcs(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.allow_cross_org_vpcs = Some(v.into());
        self
    }
    #[doc = "Set the field `mode`.\n"]
    pub fn set_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mode = Some(v.into());
        self
    }
    #[doc = "Set the field `public_network_source`.\n"]
    pub fn set_public_network_source(
        mut self,
        v: impl Into<ListField<DataStorageBucketIpFilterElPublicNetworkSourceEl>>,
    ) -> Self {
        self.public_network_source = Some(v.into());
        self
    }
    #[doc = "Set the field `vpc_network_sources`.\n"]
    pub fn set_vpc_network_sources(
        mut self,
        v: impl Into<ListField<DataStorageBucketIpFilterElVpcNetworkSourcesEl>>,
    ) -> Self {
        self.vpc_network_sources = Some(v.into());
        self
    }
}
impl ToListMappable for DataStorageBucketIpFilterEl {
    type O = BlockAssignable<DataStorageBucketIpFilterEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataStorageBucketIpFilterEl {}
impl BuildDataStorageBucketIpFilterEl {
    pub fn build(self) -> DataStorageBucketIpFilterEl {
        DataStorageBucketIpFilterEl {
            allow_all_service_agent_access: core::default::Default::default(),
            allow_cross_org_vpcs: core::default::Default::default(),
            mode: core::default::Default::default(),
            public_network_source: core::default::Default::default(),
            vpc_network_sources: core::default::Default::default(),
        }
    }
}
pub struct DataStorageBucketIpFilterElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataStorageBucketIpFilterElRef {
    fn new(shared: StackShared, base: String) -> DataStorageBucketIpFilterElRef {
        DataStorageBucketIpFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataStorageBucketIpFilterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allow_all_service_agent_access` after provisioning.\n"]
    pub fn allow_all_service_agent_access(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_all_service_agent_access", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `allow_cross_org_vpcs` after provisioning.\n"]
    pub fn allow_cross_org_vpcs(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_cross_org_vpcs", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
    #[doc = "Get a reference to the value of field `public_network_source` after provisioning.\n"]
    pub fn public_network_source(
        &self,
    ) -> ListRef<DataStorageBucketIpFilterElPublicNetworkSourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.public_network_source", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `vpc_network_sources` after provisioning.\n"]
    pub fn vpc_network_sources(
        &self,
    ) -> ListRef<DataStorageBucketIpFilterElVpcNetworkSourcesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vpc_network_sources", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataStorageBucketLifecycleRuleElActionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_class: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl DataStorageBucketLifecycleRuleElActionEl {
    #[doc = "Set the field `storage_class`.\n"]
    pub fn set_storage_class(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.storage_class = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable for DataStorageBucketLifecycleRuleElActionEl {
    type O = BlockAssignable<DataStorageBucketLifecycleRuleElActionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataStorageBucketLifecycleRuleElActionEl {}
impl BuildDataStorageBucketLifecycleRuleElActionEl {
    pub fn build(self) -> DataStorageBucketLifecycleRuleElActionEl {
        DataStorageBucketLifecycleRuleElActionEl {
            storage_class: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}
pub struct DataStorageBucketLifecycleRuleElActionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataStorageBucketLifecycleRuleElActionElRef {
    fn new(shared: StackShared, base: String) -> DataStorageBucketLifecycleRuleElActionElRef {
        DataStorageBucketLifecycleRuleElActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataStorageBucketLifecycleRuleElActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `storage_class` after provisioning.\n"]
    pub fn storage_class(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.storage_class", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct DataStorageBucketLifecycleRuleElConditionEl {
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
impl DataStorageBucketLifecycleRuleElConditionEl {
    #[doc = "Set the field `age`.\n"]
    pub fn set_age(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.age = Some(v.into());
        self
    }
    #[doc = "Set the field `created_before`.\n"]
    pub fn set_created_before(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.created_before = Some(v.into());
        self
    }
    #[doc = "Set the field `custom_time_before`.\n"]
    pub fn set_custom_time_before(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.custom_time_before = Some(v.into());
        self
    }
    #[doc = "Set the field `days_since_custom_time`.\n"]
    pub fn set_days_since_custom_time(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.days_since_custom_time = Some(v.into());
        self
    }
    #[doc = "Set the field `days_since_noncurrent_time`.\n"]
    pub fn set_days_since_noncurrent_time(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.days_since_noncurrent_time = Some(v.into());
        self
    }
    #[doc = "Set the field `matches_prefix`.\n"]
    pub fn set_matches_prefix(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.matches_prefix = Some(v.into());
        self
    }
    #[doc = "Set the field `matches_storage_class`.\n"]
    pub fn set_matches_storage_class(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.matches_storage_class = Some(v.into());
        self
    }
    #[doc = "Set the field `matches_suffix`.\n"]
    pub fn set_matches_suffix(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.matches_suffix = Some(v.into());
        self
    }
    #[doc = "Set the field `noncurrent_time_before`.\n"]
    pub fn set_noncurrent_time_before(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.noncurrent_time_before = Some(v.into());
        self
    }
    #[doc = "Set the field `num_newer_versions`.\n"]
    pub fn set_num_newer_versions(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.num_newer_versions = Some(v.into());
        self
    }
    #[doc = "Set the field `send_age_if_zero`.\n"]
    pub fn set_send_age_if_zero(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.send_age_if_zero = Some(v.into());
        self
    }
    #[doc = "Set the field `send_days_since_custom_time_if_zero`.\n"]
    pub fn set_send_days_since_custom_time_if_zero(
        mut self,
        v: impl Into<PrimField<bool>>,
    ) -> Self {
        self.send_days_since_custom_time_if_zero = Some(v.into());
        self
    }
    #[doc = "Set the field `send_days_since_noncurrent_time_if_zero`.\n"]
    pub fn set_send_days_since_noncurrent_time_if_zero(
        mut self,
        v: impl Into<PrimField<bool>>,
    ) -> Self {
        self.send_days_since_noncurrent_time_if_zero = Some(v.into());
        self
    }
    #[doc = "Set the field `send_num_newer_versions_if_zero`.\n"]
    pub fn set_send_num_newer_versions_if_zero(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.send_num_newer_versions_if_zero = Some(v.into());
        self
    }
    #[doc = "Set the field `with_state`.\n"]
    pub fn set_with_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.with_state = Some(v.into());
        self
    }
}
impl ToListMappable for DataStorageBucketLifecycleRuleElConditionEl {
    type O = BlockAssignable<DataStorageBucketLifecycleRuleElConditionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataStorageBucketLifecycleRuleElConditionEl {}
impl BuildDataStorageBucketLifecycleRuleElConditionEl {
    pub fn build(self) -> DataStorageBucketLifecycleRuleElConditionEl {
        DataStorageBucketLifecycleRuleElConditionEl {
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
pub struct DataStorageBucketLifecycleRuleElConditionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataStorageBucketLifecycleRuleElConditionElRef {
    fn new(shared: StackShared, base: String) -> DataStorageBucketLifecycleRuleElConditionElRef {
        DataStorageBucketLifecycleRuleElConditionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataStorageBucketLifecycleRuleElConditionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `age` after provisioning.\n"]
    pub fn age(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.age", self.base))
    }
    #[doc = "Get a reference to the value of field `created_before` after provisioning.\n"]
    pub fn created_before(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created_before", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `custom_time_before` after provisioning.\n"]
    pub fn custom_time_before(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.custom_time_before", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `days_since_custom_time` after provisioning.\n"]
    pub fn days_since_custom_time(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.days_since_custom_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `days_since_noncurrent_time` after provisioning.\n"]
    pub fn days_since_noncurrent_time(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.days_since_noncurrent_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `matches_prefix` after provisioning.\n"]
    pub fn matches_prefix(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.matches_prefix", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `matches_storage_class` after provisioning.\n"]
    pub fn matches_storage_class(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.matches_storage_class", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `matches_suffix` after provisioning.\n"]
    pub fn matches_suffix(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.matches_suffix", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `noncurrent_time_before` after provisioning.\n"]
    pub fn noncurrent_time_before(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.noncurrent_time_before", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `num_newer_versions` after provisioning.\n"]
    pub fn num_newer_versions(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.num_newer_versions", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `send_age_if_zero` after provisioning.\n"]
    pub fn send_age_if_zero(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.send_age_if_zero", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `send_days_since_custom_time_if_zero` after provisioning.\n"]
    pub fn send_days_since_custom_time_if_zero(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.send_days_since_custom_time_if_zero", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `send_days_since_noncurrent_time_if_zero` after provisioning.\n"]
    pub fn send_days_since_noncurrent_time_if_zero(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.send_days_since_noncurrent_time_if_zero", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `send_num_newer_versions_if_zero` after provisioning.\n"]
    pub fn send_num_newer_versions_if_zero(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.send_num_newer_versions_if_zero", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `with_state` after provisioning.\n"]
    pub fn with_state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.with_state", self.base))
    }
}
#[derive(Serialize)]
pub struct DataStorageBucketLifecycleRuleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<SetField<DataStorageBucketLifecycleRuleElActionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    condition: Option<SetField<DataStorageBucketLifecycleRuleElConditionEl>>,
}
impl DataStorageBucketLifecycleRuleEl {
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(
        mut self,
        v: impl Into<SetField<DataStorageBucketLifecycleRuleElActionEl>>,
    ) -> Self {
        self.action = Some(v.into());
        self
    }
    #[doc = "Set the field `condition`.\n"]
    pub fn set_condition(
        mut self,
        v: impl Into<SetField<DataStorageBucketLifecycleRuleElConditionEl>>,
    ) -> Self {
        self.condition = Some(v.into());
        self
    }
}
impl ToListMappable for DataStorageBucketLifecycleRuleEl {
    type O = BlockAssignable<DataStorageBucketLifecycleRuleEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataStorageBucketLifecycleRuleEl {}
impl BuildDataStorageBucketLifecycleRuleEl {
    pub fn build(self) -> DataStorageBucketLifecycleRuleEl {
        DataStorageBucketLifecycleRuleEl {
            action: core::default::Default::default(),
            condition: core::default::Default::default(),
        }
    }
}
pub struct DataStorageBucketLifecycleRuleElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataStorageBucketLifecycleRuleElRef {
    fn new(shared: StackShared, base: String) -> DataStorageBucketLifecycleRuleElRef {
        DataStorageBucketLifecycleRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataStorageBucketLifecycleRuleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> SetRef<DataStorageBucketLifecycleRuleElActionElRef> {
        SetRef::new(self.shared().clone(), format!("{}.action", self.base))
    }
    #[doc = "Get a reference to the value of field `condition` after provisioning.\n"]
    pub fn condition(&self) -> SetRef<DataStorageBucketLifecycleRuleElConditionElRef> {
        SetRef::new(self.shared().clone(), format!("{}.condition", self.base))
    }
}
#[derive(Serialize)]
pub struct DataStorageBucketLoggingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    log_bucket: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_object_prefix: Option<PrimField<String>>,
}
impl DataStorageBucketLoggingEl {
    #[doc = "Set the field `log_bucket`.\n"]
    pub fn set_log_bucket(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.log_bucket = Some(v.into());
        self
    }
    #[doc = "Set the field `log_object_prefix`.\n"]
    pub fn set_log_object_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.log_object_prefix = Some(v.into());
        self
    }
}
impl ToListMappable for DataStorageBucketLoggingEl {
    type O = BlockAssignable<DataStorageBucketLoggingEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataStorageBucketLoggingEl {}
impl BuildDataStorageBucketLoggingEl {
    pub fn build(self) -> DataStorageBucketLoggingEl {
        DataStorageBucketLoggingEl {
            log_bucket: core::default::Default::default(),
            log_object_prefix: core::default::Default::default(),
        }
    }
}
pub struct DataStorageBucketLoggingElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataStorageBucketLoggingElRef {
    fn new(shared: StackShared, base: String) -> DataStorageBucketLoggingElRef {
        DataStorageBucketLoggingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataStorageBucketLoggingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `log_bucket` after provisioning.\n"]
    pub fn log_bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_bucket", self.base))
    }
    #[doc = "Get a reference to the value of field `log_object_prefix` after provisioning.\n"]
    pub fn log_object_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.log_object_prefix", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataStorageBucketRetentionPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    is_locked: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retention_period: Option<PrimField<String>>,
}
impl DataStorageBucketRetentionPolicyEl {
    #[doc = "Set the field `is_locked`.\n"]
    pub fn set_is_locked(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.is_locked = Some(v.into());
        self
    }
    #[doc = "Set the field `retention_period`.\n"]
    pub fn set_retention_period(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.retention_period = Some(v.into());
        self
    }
}
impl ToListMappable for DataStorageBucketRetentionPolicyEl {
    type O = BlockAssignable<DataStorageBucketRetentionPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataStorageBucketRetentionPolicyEl {}
impl BuildDataStorageBucketRetentionPolicyEl {
    pub fn build(self) -> DataStorageBucketRetentionPolicyEl {
        DataStorageBucketRetentionPolicyEl {
            is_locked: core::default::Default::default(),
            retention_period: core::default::Default::default(),
        }
    }
}
pub struct DataStorageBucketRetentionPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataStorageBucketRetentionPolicyElRef {
    fn new(shared: StackShared, base: String) -> DataStorageBucketRetentionPolicyElRef {
        DataStorageBucketRetentionPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataStorageBucketRetentionPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `is_locked` after provisioning.\n"]
    pub fn is_locked(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.is_locked", self.base))
    }
    #[doc = "Get a reference to the value of field `retention_period` after provisioning.\n"]
    pub fn retention_period(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.retention_period", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataStorageBucketSoftDeletePolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    effective_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retention_duration_seconds: Option<PrimField<f64>>,
}
impl DataStorageBucketSoftDeletePolicyEl {
    #[doc = "Set the field `effective_time`.\n"]
    pub fn set_effective_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.effective_time = Some(v.into());
        self
    }
    #[doc = "Set the field `retention_duration_seconds`.\n"]
    pub fn set_retention_duration_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.retention_duration_seconds = Some(v.into());
        self
    }
}
impl ToListMappable for DataStorageBucketSoftDeletePolicyEl {
    type O = BlockAssignable<DataStorageBucketSoftDeletePolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataStorageBucketSoftDeletePolicyEl {}
impl BuildDataStorageBucketSoftDeletePolicyEl {
    pub fn build(self) -> DataStorageBucketSoftDeletePolicyEl {
        DataStorageBucketSoftDeletePolicyEl {
            effective_time: core::default::Default::default(),
            retention_duration_seconds: core::default::Default::default(),
        }
    }
}
pub struct DataStorageBucketSoftDeletePolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataStorageBucketSoftDeletePolicyElRef {
    fn new(shared: StackShared, base: String) -> DataStorageBucketSoftDeletePolicyElRef {
        DataStorageBucketSoftDeletePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataStorageBucketSoftDeletePolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `effective_time` after provisioning.\n"]
    pub fn effective_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.effective_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `retention_duration_seconds` after provisioning.\n"]
    pub fn retention_duration_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.retention_duration_seconds", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataStorageBucketVersioningEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}
impl DataStorageBucketVersioningEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}
impl ToListMappable for DataStorageBucketVersioningEl {
    type O = BlockAssignable<DataStorageBucketVersioningEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataStorageBucketVersioningEl {}
impl BuildDataStorageBucketVersioningEl {
    pub fn build(self) -> DataStorageBucketVersioningEl {
        DataStorageBucketVersioningEl {
            enabled: core::default::Default::default(),
        }
    }
}
pub struct DataStorageBucketVersioningElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataStorageBucketVersioningElRef {
    fn new(shared: StackShared, base: String) -> DataStorageBucketVersioningElRef {
        DataStorageBucketVersioningElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataStorageBucketVersioningElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize)]
pub struct DataStorageBucketWebsiteEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    main_page_suffix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    not_found_page: Option<PrimField<String>>,
}
impl DataStorageBucketWebsiteEl {
    #[doc = "Set the field `main_page_suffix`.\n"]
    pub fn set_main_page_suffix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.main_page_suffix = Some(v.into());
        self
    }
    #[doc = "Set the field `not_found_page`.\n"]
    pub fn set_not_found_page(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.not_found_page = Some(v.into());
        self
    }
}
impl ToListMappable for DataStorageBucketWebsiteEl {
    type O = BlockAssignable<DataStorageBucketWebsiteEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataStorageBucketWebsiteEl {}
impl BuildDataStorageBucketWebsiteEl {
    pub fn build(self) -> DataStorageBucketWebsiteEl {
        DataStorageBucketWebsiteEl {
            main_page_suffix: core::default::Default::default(),
            not_found_page: core::default::Default::default(),
        }
    }
}
pub struct DataStorageBucketWebsiteElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataStorageBucketWebsiteElRef {
    fn new(shared: StackShared, base: String) -> DataStorageBucketWebsiteElRef {
        DataStorageBucketWebsiteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataStorageBucketWebsiteElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `main_page_suffix` after provisioning.\n"]
    pub fn main_page_suffix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.main_page_suffix", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `not_found_page` after provisioning.\n"]
    pub fn not_found_page(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.not_found_page", self.base),
        )
    }
}
