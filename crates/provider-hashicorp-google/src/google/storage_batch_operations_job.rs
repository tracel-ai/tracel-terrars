use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct StorageBatchOperationsJobData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete_protection: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    job_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_list: Option<Vec<StorageBatchOperationsJobBucketListEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete_object: Option<Vec<StorageBatchOperationsJobDeleteObjectEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    put_metadata: Option<Vec<StorageBatchOperationsJobPutMetadataEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    put_object_hold: Option<Vec<StorageBatchOperationsJobPutObjectHoldEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rewrite_object: Option<Vec<StorageBatchOperationsJobRewriteObjectEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<StorageBatchOperationsJobTimeoutsEl>,
    dynamic: StorageBatchOperationsJobDynamic,
}
struct StorageBatchOperationsJob_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<StorageBatchOperationsJobData>,
}
#[derive(Clone)]
pub struct StorageBatchOperationsJob(Rc<StorageBatchOperationsJob_>);
impl StorageBatchOperationsJob {
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
    #[doc = "Set the field `delete_protection`.\nIf set to 'true', the storage batch operation job will not be deleted and new job will be created."]
    pub fn set_delete_protection(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().delete_protection = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nA description provided by the user for the job. Its max length is 1024 bytes when Unicode-encoded."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `job_id`.\nThe ID of the job."]
    pub fn set_job_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().job_id = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `bucket_list`.\n"]
    pub fn set_bucket_list(
        self,
        v: impl Into<BlockAssignable<StorageBatchOperationsJobBucketListEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().bucket_list = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.bucket_list = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `delete_object`.\n"]
    pub fn set_delete_object(
        self,
        v: impl Into<BlockAssignable<StorageBatchOperationsJobDeleteObjectEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().delete_object = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.delete_object = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `put_metadata`.\n"]
    pub fn set_put_metadata(
        self,
        v: impl Into<BlockAssignable<StorageBatchOperationsJobPutMetadataEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().put_metadata = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.put_metadata = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `put_object_hold`.\n"]
    pub fn set_put_object_hold(
        self,
        v: impl Into<BlockAssignable<StorageBatchOperationsJobPutObjectHoldEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().put_object_hold = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.put_object_hold = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `rewrite_object`.\n"]
    pub fn set_rewrite_object(
        self,
        v: impl Into<BlockAssignable<StorageBatchOperationsJobRewriteObjectEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rewrite_object = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rewrite_object = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<StorageBatchOperationsJobTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `complete_time` after provisioning.\nThe time that the job was completed."]
    pub fn complete_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.complete_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp at which this storage batch operation was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `delete_protection` after provisioning.\nIf set to 'true', the storage batch operation job will not be deleted and new job will be created."]
    pub fn delete_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delete_protection", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA description provided by the user for the job. Its max length is 1024 bytes when Unicode-encoded."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `job_id` after provisioning.\nThe ID of the job."]
    pub fn job_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.job_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `schedule_time` after provisioning.\nThe time that the job was scheduled."]
    pub fn schedule_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.schedule_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nState of the job."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe timestamp at which this storage batch operation was most recently updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `bucket_list` after provisioning.\n"]
    pub fn bucket_list(&self) -> ListRef<StorageBatchOperationsJobBucketListElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.bucket_list", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `delete_object` after provisioning.\n"]
    pub fn delete_object(&self) -> ListRef<StorageBatchOperationsJobDeleteObjectElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.delete_object", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `put_metadata` after provisioning.\n"]
    pub fn put_metadata(&self) -> ListRef<StorageBatchOperationsJobPutMetadataElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.put_metadata", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `put_object_hold` after provisioning.\n"]
    pub fn put_object_hold(&self) -> ListRef<StorageBatchOperationsJobPutObjectHoldElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.put_object_hold", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rewrite_object` after provisioning.\n"]
    pub fn rewrite_object(&self) -> ListRef<StorageBatchOperationsJobRewriteObjectElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rewrite_object", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> StorageBatchOperationsJobTimeoutsElRef {
        StorageBatchOperationsJobTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for StorageBatchOperationsJob {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for StorageBatchOperationsJob {}
impl ToListMappable for StorageBatchOperationsJob {
    type O = ListRef<StorageBatchOperationsJobRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for StorageBatchOperationsJob_ {
    fn extract_resource_type(&self) -> String {
        "google_storage_batch_operations_job".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildStorageBatchOperationsJob {
    pub tf_id: String,
}
impl BuildStorageBatchOperationsJob {
    pub fn build(self, stack: &mut Stack) -> StorageBatchOperationsJob {
        let out = StorageBatchOperationsJob(Rc::new(StorageBatchOperationsJob_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(StorageBatchOperationsJobData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                delete_protection: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                job_id: core::default::Default::default(),
                project: core::default::Default::default(),
                bucket_list: core::default::Default::default(),
                delete_object: core::default::Default::default(),
                put_metadata: core::default::Default::default(),
                put_object_hold: core::default::Default::default(),
                rewrite_object: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct StorageBatchOperationsJobRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageBatchOperationsJobRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl StorageBatchOperationsJobRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `complete_time` after provisioning.\nThe time that the job was completed."]
    pub fn complete_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.complete_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp at which this storage batch operation was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `delete_protection` after provisioning.\nIf set to 'true', the storage batch operation job will not be deleted and new job will be created."]
    pub fn delete_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delete_protection", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA description provided by the user for the job. Its max length is 1024 bytes when Unicode-encoded."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `job_id` after provisioning.\nThe ID of the job."]
    pub fn job_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.job_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `schedule_time` after provisioning.\nThe time that the job was scheduled."]
    pub fn schedule_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.schedule_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nState of the job."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe timestamp at which this storage batch operation was most recently updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `bucket_list` after provisioning.\n"]
    pub fn bucket_list(&self) -> ListRef<StorageBatchOperationsJobBucketListElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.bucket_list", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `delete_object` after provisioning.\n"]
    pub fn delete_object(&self) -> ListRef<StorageBatchOperationsJobDeleteObjectElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.delete_object", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `put_metadata` after provisioning.\n"]
    pub fn put_metadata(&self) -> ListRef<StorageBatchOperationsJobPutMetadataElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.put_metadata", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `put_object_hold` after provisioning.\n"]
    pub fn put_object_hold(&self) -> ListRef<StorageBatchOperationsJobPutObjectHoldElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.put_object_hold", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rewrite_object` after provisioning.\n"]
    pub fn rewrite_object(&self) -> ListRef<StorageBatchOperationsJobRewriteObjectElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rewrite_object", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> StorageBatchOperationsJobTimeoutsElRef {
        StorageBatchOperationsJobTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct StorageBatchOperationsJobBucketListElBucketsElManifestEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    manifest_location: Option<PrimField<String>>,
}
impl StorageBatchOperationsJobBucketListElBucketsElManifestEl {
    #[doc = "Set the field `manifest_location`.\nSpecifies objects in a manifest file."]
    pub fn set_manifest_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.manifest_location = Some(v.into());
        self
    }
}
impl ToListMappable for StorageBatchOperationsJobBucketListElBucketsElManifestEl {
    type O = BlockAssignable<StorageBatchOperationsJobBucketListElBucketsElManifestEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageBatchOperationsJobBucketListElBucketsElManifestEl {}
impl BuildStorageBatchOperationsJobBucketListElBucketsElManifestEl {
    pub fn build(self) -> StorageBatchOperationsJobBucketListElBucketsElManifestEl {
        StorageBatchOperationsJobBucketListElBucketsElManifestEl {
            manifest_location: core::default::Default::default(),
        }
    }
}
pub struct StorageBatchOperationsJobBucketListElBucketsElManifestElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageBatchOperationsJobBucketListElBucketsElManifestElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> StorageBatchOperationsJobBucketListElBucketsElManifestElRef {
        StorageBatchOperationsJobBucketListElBucketsElManifestElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageBatchOperationsJobBucketListElBucketsElManifestElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `manifest_location` after provisioning.\nSpecifies objects in a manifest file."]
    pub fn manifest_location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.manifest_location", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct StorageBatchOperationsJobBucketListElBucketsElPrefixListEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    included_object_prefixes: Option<ListField<PrimField<String>>>,
}
impl StorageBatchOperationsJobBucketListElBucketsElPrefixListEl {
    #[doc = "Set the field `included_object_prefixes`.\n"]
    pub fn set_included_object_prefixes(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.included_object_prefixes = Some(v.into());
        self
    }
}
impl ToListMappable for StorageBatchOperationsJobBucketListElBucketsElPrefixListEl {
    type O = BlockAssignable<StorageBatchOperationsJobBucketListElBucketsElPrefixListEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageBatchOperationsJobBucketListElBucketsElPrefixListEl {}
impl BuildStorageBatchOperationsJobBucketListElBucketsElPrefixListEl {
    pub fn build(self) -> StorageBatchOperationsJobBucketListElBucketsElPrefixListEl {
        StorageBatchOperationsJobBucketListElBucketsElPrefixListEl {
            included_object_prefixes: core::default::Default::default(),
        }
    }
}
pub struct StorageBatchOperationsJobBucketListElBucketsElPrefixListElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageBatchOperationsJobBucketListElBucketsElPrefixListElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> StorageBatchOperationsJobBucketListElBucketsElPrefixListElRef {
        StorageBatchOperationsJobBucketListElBucketsElPrefixListElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageBatchOperationsJobBucketListElBucketsElPrefixListElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `included_object_prefixes` after provisioning.\n"]
    pub fn included_object_prefixes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.included_object_prefixes", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct StorageBatchOperationsJobBucketListElBucketsElDynamic {
    manifest: Option<DynamicBlock<StorageBatchOperationsJobBucketListElBucketsElManifestEl>>,
    prefix_list: Option<DynamicBlock<StorageBatchOperationsJobBucketListElBucketsElPrefixListEl>>,
}
#[derive(Serialize)]
pub struct StorageBatchOperationsJobBucketListElBucketsEl {
    bucket: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    manifest: Option<Vec<StorageBatchOperationsJobBucketListElBucketsElManifestEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix_list: Option<Vec<StorageBatchOperationsJobBucketListElBucketsElPrefixListEl>>,
    dynamic: StorageBatchOperationsJobBucketListElBucketsElDynamic,
}
impl StorageBatchOperationsJobBucketListElBucketsEl {
    #[doc = "Set the field `manifest`.\n"]
    pub fn set_manifest(
        mut self,
        v: impl Into<BlockAssignable<StorageBatchOperationsJobBucketListElBucketsElManifestEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.manifest = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.manifest = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `prefix_list`.\n"]
    pub fn set_prefix_list(
        mut self,
        v: impl Into<BlockAssignable<StorageBatchOperationsJobBucketListElBucketsElPrefixListEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.prefix_list = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.prefix_list = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for StorageBatchOperationsJobBucketListElBucketsEl {
    type O = BlockAssignable<StorageBatchOperationsJobBucketListElBucketsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageBatchOperationsJobBucketListElBucketsEl {
    #[doc = "Bucket name for the objects to be transformed."]
    pub bucket: PrimField<String>,
}
impl BuildStorageBatchOperationsJobBucketListElBucketsEl {
    pub fn build(self) -> StorageBatchOperationsJobBucketListElBucketsEl {
        StorageBatchOperationsJobBucketListElBucketsEl {
            bucket: self.bucket,
            manifest: core::default::Default::default(),
            prefix_list: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct StorageBatchOperationsJobBucketListElBucketsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageBatchOperationsJobBucketListElBucketsElRef {
    fn new(shared: StackShared, base: String) -> StorageBatchOperationsJobBucketListElBucketsElRef {
        StorageBatchOperationsJobBucketListElBucketsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageBatchOperationsJobBucketListElBucketsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bucket` after provisioning.\nBucket name for the objects to be transformed."]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }
    #[doc = "Get a reference to the value of field `manifest` after provisioning.\n"]
    pub fn manifest(&self) -> ListRef<StorageBatchOperationsJobBucketListElBucketsElManifestElRef> {
        ListRef::new(self.shared().clone(), format!("{}.manifest", self.base))
    }
    #[doc = "Get a reference to the value of field `prefix_list` after provisioning.\n"]
    pub fn prefix_list(
        &self,
    ) -> ListRef<StorageBatchOperationsJobBucketListElBucketsElPrefixListElRef> {
        ListRef::new(self.shared().clone(), format!("{}.prefix_list", self.base))
    }
}
#[derive(Serialize, Default)]
struct StorageBatchOperationsJobBucketListElDynamic {
    buckets: Option<DynamicBlock<StorageBatchOperationsJobBucketListElBucketsEl>>,
}
#[derive(Serialize)]
pub struct StorageBatchOperationsJobBucketListEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    buckets: Option<Vec<StorageBatchOperationsJobBucketListElBucketsEl>>,
    dynamic: StorageBatchOperationsJobBucketListElDynamic,
}
impl StorageBatchOperationsJobBucketListEl {
    #[doc = "Set the field `buckets`.\n"]
    pub fn set_buckets(
        mut self,
        v: impl Into<BlockAssignable<StorageBatchOperationsJobBucketListElBucketsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.buckets = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.buckets = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for StorageBatchOperationsJobBucketListEl {
    type O = BlockAssignable<StorageBatchOperationsJobBucketListEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageBatchOperationsJobBucketListEl {}
impl BuildStorageBatchOperationsJobBucketListEl {
    pub fn build(self) -> StorageBatchOperationsJobBucketListEl {
        StorageBatchOperationsJobBucketListEl {
            buckets: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct StorageBatchOperationsJobBucketListElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageBatchOperationsJobBucketListElRef {
    fn new(shared: StackShared, base: String) -> StorageBatchOperationsJobBucketListElRef {
        StorageBatchOperationsJobBucketListElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageBatchOperationsJobBucketListElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `buckets` after provisioning.\n"]
    pub fn buckets(&self) -> ListRef<StorageBatchOperationsJobBucketListElBucketsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.buckets", self.base))
    }
}
#[derive(Serialize)]
pub struct StorageBatchOperationsJobDeleteObjectEl {
    permanent_object_deletion_enabled: PrimField<bool>,
}
impl StorageBatchOperationsJobDeleteObjectEl {}
impl ToListMappable for StorageBatchOperationsJobDeleteObjectEl {
    type O = BlockAssignable<StorageBatchOperationsJobDeleteObjectEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageBatchOperationsJobDeleteObjectEl {
    #[doc = "enable flag to permanently delete object and all object versions if versioning is enabled on bucket."]
    pub permanent_object_deletion_enabled: PrimField<bool>,
}
impl BuildStorageBatchOperationsJobDeleteObjectEl {
    pub fn build(self) -> StorageBatchOperationsJobDeleteObjectEl {
        StorageBatchOperationsJobDeleteObjectEl {
            permanent_object_deletion_enabled: self.permanent_object_deletion_enabled,
        }
    }
}
pub struct StorageBatchOperationsJobDeleteObjectElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageBatchOperationsJobDeleteObjectElRef {
    fn new(shared: StackShared, base: String) -> StorageBatchOperationsJobDeleteObjectElRef {
        StorageBatchOperationsJobDeleteObjectElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageBatchOperationsJobDeleteObjectElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `permanent_object_deletion_enabled` after provisioning.\nenable flag to permanently delete object and all object versions if versioning is enabled on bucket."]
    pub fn permanent_object_deletion_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.permanent_object_deletion_enabled", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct StorageBatchOperationsJobPutMetadataEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_control: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content_disposition: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content_encoding: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content_language: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_metadata: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_time: Option<PrimField<String>>,
}
impl StorageBatchOperationsJobPutMetadataEl {
    #[doc = "Set the field `cache_control`.\nCache-Control directive to specify caching behavior of object data. If omitted and object is accessible to all anonymous users, the default will be public, max-age=3600"]
    pub fn set_cache_control(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cache_control = Some(v.into());
        self
    }
    #[doc = "Set the field `content_disposition`.\nContent-Disposition of the object data."]
    pub fn set_content_disposition(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.content_disposition = Some(v.into());
        self
    }
    #[doc = "Set the field `content_encoding`.\nContent Encoding of the object data."]
    pub fn set_content_encoding(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.content_encoding = Some(v.into());
        self
    }
    #[doc = "Set the field `content_language`.\nContent-Language of the object data."]
    pub fn set_content_language(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.content_language = Some(v.into());
        self
    }
    #[doc = "Set the field `content_type`.\nContent-Type of the object data."]
    pub fn set_content_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.content_type = Some(v.into());
        self
    }
    #[doc = "Set the field `custom_metadata`.\nUser-provided metadata, in key/value pairs."]
    pub fn set_custom_metadata(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.custom_metadata = Some(v.into());
        self
    }
    #[doc = "Set the field `custom_time`.\nUpdates the objects fixed custom time metadata."]
    pub fn set_custom_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.custom_time = Some(v.into());
        self
    }
}
impl ToListMappable for StorageBatchOperationsJobPutMetadataEl {
    type O = BlockAssignable<StorageBatchOperationsJobPutMetadataEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageBatchOperationsJobPutMetadataEl {}
impl BuildStorageBatchOperationsJobPutMetadataEl {
    pub fn build(self) -> StorageBatchOperationsJobPutMetadataEl {
        StorageBatchOperationsJobPutMetadataEl {
            cache_control: core::default::Default::default(),
            content_disposition: core::default::Default::default(),
            content_encoding: core::default::Default::default(),
            content_language: core::default::Default::default(),
            content_type: core::default::Default::default(),
            custom_metadata: core::default::Default::default(),
            custom_time: core::default::Default::default(),
        }
    }
}
pub struct StorageBatchOperationsJobPutMetadataElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageBatchOperationsJobPutMetadataElRef {
    fn new(shared: StackShared, base: String) -> StorageBatchOperationsJobPutMetadataElRef {
        StorageBatchOperationsJobPutMetadataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageBatchOperationsJobPutMetadataElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cache_control` after provisioning.\nCache-Control directive to specify caching behavior of object data. If omitted and object is accessible to all anonymous users, the default will be public, max-age=3600"]
    pub fn cache_control(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cache_control", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `content_disposition` after provisioning.\nContent-Disposition of the object data."]
    pub fn content_disposition(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.content_disposition", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `content_encoding` after provisioning.\nContent Encoding of the object data."]
    pub fn content_encoding(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.content_encoding", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `content_language` after provisioning.\nContent-Language of the object data."]
    pub fn content_language(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.content_language", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `content_type` after provisioning.\nContent-Type of the object data."]
    pub fn content_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content_type", self.base))
    }
    #[doc = "Get a reference to the value of field `custom_metadata` after provisioning.\nUser-provided metadata, in key/value pairs."]
    pub fn custom_metadata(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.custom_metadata", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `custom_time` after provisioning.\nUpdates the objects fixed custom time metadata."]
    pub fn custom_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.custom_time", self.base))
    }
}
#[derive(Serialize)]
pub struct StorageBatchOperationsJobPutObjectHoldEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    event_based_hold: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temporary_hold: Option<PrimField<String>>,
}
impl StorageBatchOperationsJobPutObjectHoldEl {
    #[doc = "Set the field `event_based_hold`.\nset/unset to update event based hold for objects."]
    pub fn set_event_based_hold(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.event_based_hold = Some(v.into());
        self
    }
    #[doc = "Set the field `temporary_hold`.\nset/unset to update temporary based hold for objects."]
    pub fn set_temporary_hold(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.temporary_hold = Some(v.into());
        self
    }
}
impl ToListMappable for StorageBatchOperationsJobPutObjectHoldEl {
    type O = BlockAssignable<StorageBatchOperationsJobPutObjectHoldEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageBatchOperationsJobPutObjectHoldEl {}
impl BuildStorageBatchOperationsJobPutObjectHoldEl {
    pub fn build(self) -> StorageBatchOperationsJobPutObjectHoldEl {
        StorageBatchOperationsJobPutObjectHoldEl {
            event_based_hold: core::default::Default::default(),
            temporary_hold: core::default::Default::default(),
        }
    }
}
pub struct StorageBatchOperationsJobPutObjectHoldElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageBatchOperationsJobPutObjectHoldElRef {
    fn new(shared: StackShared, base: String) -> StorageBatchOperationsJobPutObjectHoldElRef {
        StorageBatchOperationsJobPutObjectHoldElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageBatchOperationsJobPutObjectHoldElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `event_based_hold` after provisioning.\nset/unset to update event based hold for objects."]
    pub fn event_based_hold(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.event_based_hold", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `temporary_hold` after provisioning.\nset/unset to update temporary based hold for objects."]
    pub fn temporary_hold(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.temporary_hold", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct StorageBatchOperationsJobRewriteObjectEl {
    kms_key: PrimField<String>,
}
impl StorageBatchOperationsJobRewriteObjectEl {}
impl ToListMappable for StorageBatchOperationsJobRewriteObjectEl {
    type O = BlockAssignable<StorageBatchOperationsJobRewriteObjectEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageBatchOperationsJobRewriteObjectEl {
    #[doc = "valid kms key"]
    pub kms_key: PrimField<String>,
}
impl BuildStorageBatchOperationsJobRewriteObjectEl {
    pub fn build(self) -> StorageBatchOperationsJobRewriteObjectEl {
        StorageBatchOperationsJobRewriteObjectEl {
            kms_key: self.kms_key,
        }
    }
}
pub struct StorageBatchOperationsJobRewriteObjectElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageBatchOperationsJobRewriteObjectElRef {
    fn new(shared: StackShared, base: String) -> StorageBatchOperationsJobRewriteObjectElRef {
        StorageBatchOperationsJobRewriteObjectElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageBatchOperationsJobRewriteObjectElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `kms_key` after provisioning.\nvalid kms key"]
    pub fn kms_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key", self.base))
    }
}
#[derive(Serialize)]
pub struct StorageBatchOperationsJobTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl StorageBatchOperationsJobTimeoutsEl {
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
impl ToListMappable for StorageBatchOperationsJobTimeoutsEl {
    type O = BlockAssignable<StorageBatchOperationsJobTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageBatchOperationsJobTimeoutsEl {}
impl BuildStorageBatchOperationsJobTimeoutsEl {
    pub fn build(self) -> StorageBatchOperationsJobTimeoutsEl {
        StorageBatchOperationsJobTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct StorageBatchOperationsJobTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageBatchOperationsJobTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> StorageBatchOperationsJobTimeoutsElRef {
        StorageBatchOperationsJobTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageBatchOperationsJobTimeoutsElRef {
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
struct StorageBatchOperationsJobDynamic {
    bucket_list: Option<DynamicBlock<StorageBatchOperationsJobBucketListEl>>,
    delete_object: Option<DynamicBlock<StorageBatchOperationsJobDeleteObjectEl>>,
    put_metadata: Option<DynamicBlock<StorageBatchOperationsJobPutMetadataEl>>,
    put_object_hold: Option<DynamicBlock<StorageBatchOperationsJobPutObjectHoldEl>>,
    rewrite_object: Option<DynamicBlock<StorageBatchOperationsJobRewriteObjectEl>>,
}
