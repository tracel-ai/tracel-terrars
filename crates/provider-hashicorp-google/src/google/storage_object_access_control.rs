use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct StorageObjectAccessControlData {
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
    object: PrimField<String>,
    role: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<StorageObjectAccessControlTimeoutsEl>,
}
struct StorageObjectAccessControl_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<StorageObjectAccessControlData>,
}
#[derive(Clone)]
pub struct StorageObjectAccessControl(Rc<StorageObjectAccessControl_>);
impl StorageObjectAccessControl {
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
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<StorageObjectAccessControlTimeoutsEl>) -> Self {
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
    #[doc = "Get a reference to the value of field `entity` after provisioning.\nThe entity holding the permission, in one of the following forms:\n  * user-{{userId}}\n  * user-{{email}} (such as \"user-liz@example.com\")\n  * group-{{groupId}}\n  * group-{{email}} (such as \"group-example@googlegroups.com\")\n  * domain-{{domain}} (such as \"domain-example.com\")\n  * project-team-{{projectId}}\n  * allUsers\n  * allAuthenticatedUsers"]
    pub fn entity(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.entity", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `entity_id` after provisioning.\nThe ID for the entity"]
    pub fn entity_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.entity_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `generation` after provisioning.\nThe content generation of the object, if applied to an object."]
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
    #[doc = "Get a reference to the value of field `object` after provisioning.\nThe name of the object to apply the access control to."]
    pub fn object(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.object", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project_team` after provisioning.\nThe project team associated with the entity"]
    pub fn project_team(&self) -> ListRef<StorageObjectAccessControlProjectTeamElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.project_team", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `role` after provisioning.\nThe access permission for the entity. Possible values: [\"OWNER\", \"READER\"]"]
    pub fn role(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.role", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> StorageObjectAccessControlTimeoutsElRef {
        StorageObjectAccessControlTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for StorageObjectAccessControl {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for StorageObjectAccessControl {}
impl ToListMappable for StorageObjectAccessControl {
    type O = ListRef<StorageObjectAccessControlRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for StorageObjectAccessControl_ {
    fn extract_resource_type(&self) -> String {
        "google_storage_object_access_control".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildStorageObjectAccessControl {
    pub tf_id: String,
    #[doc = "The name of the bucket."]
    pub bucket: PrimField<String>,
    #[doc = "The entity holding the permission, in one of the following forms:\n  * user-{{userId}}\n  * user-{{email}} (such as \"user-liz@example.com\")\n  * group-{{groupId}}\n  * group-{{email}} (such as \"group-example@googlegroups.com\")\n  * domain-{{domain}} (such as \"domain-example.com\")\n  * project-team-{{projectId}}\n  * allUsers\n  * allAuthenticatedUsers"]
    pub entity: PrimField<String>,
    #[doc = "The name of the object to apply the access control to."]
    pub object: PrimField<String>,
    #[doc = "The access permission for the entity. Possible values: [\"OWNER\", \"READER\"]"]
    pub role: PrimField<String>,
}
impl BuildStorageObjectAccessControl {
    pub fn build(self, stack: &mut Stack) -> StorageObjectAccessControl {
        let out = StorageObjectAccessControl(Rc::new(StorageObjectAccessControl_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(StorageObjectAccessControlData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                bucket: self.bucket,
                deletion_policy: core::default::Default::default(),
                entity: self.entity,
                id: core::default::Default::default(),
                object: self.object,
                role: self.role,
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct StorageObjectAccessControlRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageObjectAccessControlRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl StorageObjectAccessControlRef {
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
    #[doc = "Get a reference to the value of field `entity` after provisioning.\nThe entity holding the permission, in one of the following forms:\n  * user-{{userId}}\n  * user-{{email}} (such as \"user-liz@example.com\")\n  * group-{{groupId}}\n  * group-{{email}} (such as \"group-example@googlegroups.com\")\n  * domain-{{domain}} (such as \"domain-example.com\")\n  * project-team-{{projectId}}\n  * allUsers\n  * allAuthenticatedUsers"]
    pub fn entity(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.entity", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `entity_id` after provisioning.\nThe ID for the entity"]
    pub fn entity_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.entity_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `generation` after provisioning.\nThe content generation of the object, if applied to an object."]
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
    #[doc = "Get a reference to the value of field `object` after provisioning.\nThe name of the object to apply the access control to."]
    pub fn object(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.object", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project_team` after provisioning.\nThe project team associated with the entity"]
    pub fn project_team(&self) -> ListRef<StorageObjectAccessControlProjectTeamElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.project_team", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `role` after provisioning.\nThe access permission for the entity. Possible values: [\"OWNER\", \"READER\"]"]
    pub fn role(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.role", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> StorageObjectAccessControlTimeoutsElRef {
        StorageObjectAccessControlTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct StorageObjectAccessControlProjectTeamEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    project_number: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    team: Option<PrimField<String>>,
}
impl StorageObjectAccessControlProjectTeamEl {
    #[doc = "Set the field `project_number`.\n"]
    pub fn set_project_number(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.project_number = Some(v.into());
        self
    }
    #[doc = "Set the field `team`.\n"]
    pub fn set_team(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.team = Some(v.into());
        self
    }
}
impl ToListMappable for StorageObjectAccessControlProjectTeamEl {
    type O = BlockAssignable<StorageObjectAccessControlProjectTeamEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageObjectAccessControlProjectTeamEl {}
impl BuildStorageObjectAccessControlProjectTeamEl {
    pub fn build(self) -> StorageObjectAccessControlProjectTeamEl {
        StorageObjectAccessControlProjectTeamEl {
            project_number: core::default::Default::default(),
            team: core::default::Default::default(),
        }
    }
}
pub struct StorageObjectAccessControlProjectTeamElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageObjectAccessControlProjectTeamElRef {
    fn new(shared: StackShared, base: String) -> StorageObjectAccessControlProjectTeamElRef {
        StorageObjectAccessControlProjectTeamElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageObjectAccessControlProjectTeamElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `project_number` after provisioning.\n"]
    pub fn project_number(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project_number", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `team` after provisioning.\n"]
    pub fn team(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.team", self.base))
    }
}
#[derive(Serialize)]
pub struct StorageObjectAccessControlTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl StorageObjectAccessControlTimeoutsEl {
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
impl ToListMappable for StorageObjectAccessControlTimeoutsEl {
    type O = BlockAssignable<StorageObjectAccessControlTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageObjectAccessControlTimeoutsEl {}
impl BuildStorageObjectAccessControlTimeoutsEl {
    pub fn build(self) -> StorageObjectAccessControlTimeoutsEl {
        StorageObjectAccessControlTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct StorageObjectAccessControlTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageObjectAccessControlTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> StorageObjectAccessControlTimeoutsElRef {
        StorageObjectAccessControlTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageObjectAccessControlTimeoutsElRef {
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
