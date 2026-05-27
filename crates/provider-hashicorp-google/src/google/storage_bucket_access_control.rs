use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct StorageBucketAccessControlData {
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
    deletion_policy: Option<PrimField<String>>,
    entity: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    role: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<StorageBucketAccessControlTimeoutsEl>,
}
struct StorageBucketAccessControl_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<StorageBucketAccessControlData>,
}
#[derive(Clone)]
pub struct StorageBucketAccessControl(Rc<StorageBucketAccessControl_>);
impl StorageBucketAccessControl {
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
    #[doc = "Set the field `role`.\nThe access permission for the entity. Possible values: [\"OWNER\", \"READER\", \"WRITER\"]"]
    pub fn set_role(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().role = Some(v.into());
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<StorageBucketAccessControlTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `bucket` after provisioning.\nThe name of the bucket."]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bucket", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `domain` after provisioning.\nThe domain associated with the entity."]
    pub fn domain(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `email` after provisioning.\nThe email address associated with the entity."]
    pub fn email(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.email", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `entity` after provisioning.\nThe entity holding the permission, in one of the following forms:\n  user-userId\n  user-email\n  group-groupId\n  group-email\n  domain-domain\n  project-team-projectId\n  allUsers\n  allAuthenticatedUsers\nExamples:\n  The user liz@example.com would be user-liz@example.com.\n  The group example@googlegroups.com would be\n  group-example@googlegroups.com.\n  To refer to all members of the Google Apps for Business domain\n  example.com, the entity would be domain-example.com."]
    pub fn entity(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.entity", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `role` after provisioning.\nThe access permission for the entity. Possible values: [\"OWNER\", \"READER\", \"WRITER\"]"]
    pub fn role(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.role", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> StorageBucketAccessControlTimeoutsElRef {
        StorageBucketAccessControlTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for StorageBucketAccessControl {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for StorageBucketAccessControl {}
impl ToListMappable for StorageBucketAccessControl {
    type O = ListRef<StorageBucketAccessControlRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for StorageBucketAccessControl_ {
    fn extract_resource_type(&self) -> String {
        "google_storage_bucket_access_control".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildStorageBucketAccessControl {
    pub tf_id: String,
    #[doc = "The name of the bucket."]
    pub bucket: PrimField<String>,
    #[doc = "The entity holding the permission, in one of the following forms:\n  user-userId\n  user-email\n  group-groupId\n  group-email\n  domain-domain\n  project-team-projectId\n  allUsers\n  allAuthenticatedUsers\nExamples:\n  The user liz@example.com would be user-liz@example.com.\n  The group example@googlegroups.com would be\n  group-example@googlegroups.com.\n  To refer to all members of the Google Apps for Business domain\n  example.com, the entity would be domain-example.com."]
    pub entity: PrimField<String>,
}
impl BuildStorageBucketAccessControl {
    pub fn build(self, stack: &mut Stack) -> StorageBucketAccessControl {
        let out = StorageBucketAccessControl(Rc::new(StorageBucketAccessControl_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(StorageBucketAccessControlData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                bucket: self.bucket,
                deletion_policy: core::default::Default::default(),
                entity: self.entity,
                id: core::default::Default::default(),
                role: core::default::Default::default(),
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct StorageBucketAccessControlRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageBucketAccessControlRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl StorageBucketAccessControlRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bucket` after provisioning.\nThe name of the bucket."]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bucket", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `domain` after provisioning.\nThe domain associated with the entity."]
    pub fn domain(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `email` after provisioning.\nThe email address associated with the entity."]
    pub fn email(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.email", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `entity` after provisioning.\nThe entity holding the permission, in one of the following forms:\n  user-userId\n  user-email\n  group-groupId\n  group-email\n  domain-domain\n  project-team-projectId\n  allUsers\n  allAuthenticatedUsers\nExamples:\n  The user liz@example.com would be user-liz@example.com.\n  The group example@googlegroups.com would be\n  group-example@googlegroups.com.\n  To refer to all members of the Google Apps for Business domain\n  example.com, the entity would be domain-example.com."]
    pub fn entity(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.entity", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `role` after provisioning.\nThe access permission for the entity. Possible values: [\"OWNER\", \"READER\", \"WRITER\"]"]
    pub fn role(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.role", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> StorageBucketAccessControlTimeoutsElRef {
        StorageBucketAccessControlTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct StorageBucketAccessControlTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl StorageBucketAccessControlTimeoutsEl {
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
impl ToListMappable for StorageBucketAccessControlTimeoutsEl {
    type O = BlockAssignable<StorageBucketAccessControlTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageBucketAccessControlTimeoutsEl {}
impl BuildStorageBucketAccessControlTimeoutsEl {
    pub fn build(self) -> StorageBucketAccessControlTimeoutsEl {
        StorageBucketAccessControlTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct StorageBucketAccessControlTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageBucketAccessControlTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> StorageBucketAccessControlTimeoutsElRef {
        StorageBucketAccessControlTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageBucketAccessControlTimeoutsElRef {
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
