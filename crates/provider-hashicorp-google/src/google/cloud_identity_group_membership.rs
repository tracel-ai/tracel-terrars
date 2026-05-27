use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct CloudIdentityGroupMembershipData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    create_ignore_already_exists: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    group: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preferred_member_key: Option<Vec<CloudIdentityGroupMembershipPreferredMemberKeyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    roles: Option<Vec<CloudIdentityGroupMembershipRolesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<CloudIdentityGroupMembershipTimeoutsEl>,
    dynamic: CloudIdentityGroupMembershipDynamic,
}
struct CloudIdentityGroupMembership_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CloudIdentityGroupMembershipData>,
}
#[derive(Clone)]
pub struct CloudIdentityGroupMembership(Rc<CloudIdentityGroupMembership_>);
impl CloudIdentityGroupMembership {
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
    #[doc = "Set the field `create_ignore_already_exists`.\nIf set to true, skip group member creation if a membership with the same name already exists. Defaults to false."]
    pub fn set_create_ignore_already_exists(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().create_ignore_already_exists = Some(v.into());
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
    #[doc = "Set the field `preferred_member_key`.\n"]
    pub fn set_preferred_member_key(
        self,
        v: impl Into<BlockAssignable<CloudIdentityGroupMembershipPreferredMemberKeyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().preferred_member_key = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.preferred_member_key = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `roles`.\n"]
    pub fn set_roles(
        self,
        v: impl Into<BlockAssignable<CloudIdentityGroupMembershipRolesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().roles = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.roles = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<CloudIdentityGroupMembershipTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `create_ignore_already_exists` after provisioning.\nIf set to true, skip group member creation if a membership with the same name already exists. Defaults to false."]
    pub fn create_ignore_already_exists(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_ignore_already_exists", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe time when the Membership was created."]
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
    #[doc = "Get a reference to the value of field `group` after provisioning.\nThe name of the Group to create this membership in."]
    pub fn group(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.group", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the Membership, of the form groups/{group_id}/memberships/{membership_id}."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe type of the membership."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe time when the Membership was last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `preferred_member_key` after provisioning.\n"]
    pub fn preferred_member_key(
        &self,
    ) -> ListRef<CloudIdentityGroupMembershipPreferredMemberKeyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.preferred_member_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudIdentityGroupMembershipTimeoutsElRef {
        CloudIdentityGroupMembershipTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for CloudIdentityGroupMembership {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for CloudIdentityGroupMembership {}
impl ToListMappable for CloudIdentityGroupMembership {
    type O = ListRef<CloudIdentityGroupMembershipRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for CloudIdentityGroupMembership_ {
    fn extract_resource_type(&self) -> String {
        "google_cloud_identity_group_membership".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildCloudIdentityGroupMembership {
    pub tf_id: String,
    #[doc = "The name of the Group to create this membership in."]
    pub group: PrimField<String>,
}
impl BuildCloudIdentityGroupMembership {
    pub fn build(self, stack: &mut Stack) -> CloudIdentityGroupMembership {
        let out = CloudIdentityGroupMembership(Rc::new(CloudIdentityGroupMembership_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(CloudIdentityGroupMembershipData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                create_ignore_already_exists: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                group: self.group,
                id: core::default::Default::default(),
                preferred_member_key: core::default::Default::default(),
                roles: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct CloudIdentityGroupMembershipRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudIdentityGroupMembershipRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl CloudIdentityGroupMembershipRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_ignore_already_exists` after provisioning.\nIf set to true, skip group member creation if a membership with the same name already exists. Defaults to false."]
    pub fn create_ignore_already_exists(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_ignore_already_exists", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe time when the Membership was created."]
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
    #[doc = "Get a reference to the value of field `group` after provisioning.\nThe name of the Group to create this membership in."]
    pub fn group(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.group", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the Membership, of the form groups/{group_id}/memberships/{membership_id}."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe type of the membership."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe time when the Membership was last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `preferred_member_key` after provisioning.\n"]
    pub fn preferred_member_key(
        &self,
    ) -> ListRef<CloudIdentityGroupMembershipPreferredMemberKeyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.preferred_member_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudIdentityGroupMembershipTimeoutsElRef {
        CloudIdentityGroupMembershipTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct CloudIdentityGroupMembershipPreferredMemberKeyEl {
    id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    namespace: Option<PrimField<String>>,
}
impl CloudIdentityGroupMembershipPreferredMemberKeyEl {
    #[doc = "Set the field `namespace`.\nThe namespace in which the entity exists.\n\nIf not specified, the EntityKey represents a Google-managed entity\nsuch as a Google user or a Google Group.\n\nIf specified, the EntityKey represents an external-identity-mapped group.\nThe namespace must correspond to an identity source created in Admin Console\nand must be in the form of 'identitysources/{identity_source_id}'."]
    pub fn set_namespace(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.namespace = Some(v.into());
        self
    }
}
impl ToListMappable for CloudIdentityGroupMembershipPreferredMemberKeyEl {
    type O = BlockAssignable<CloudIdentityGroupMembershipPreferredMemberKeyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudIdentityGroupMembershipPreferredMemberKeyEl {
    #[doc = "The ID of the entity.\n\nFor Google-managed entities, the id must be the email address of an existing\ngroup or user.\n\nFor external-identity-mapped entities, the id must be a string conforming\nto the Identity Source's requirements.\n\nMust be unique within a namespace."]
    pub id: PrimField<String>,
}
impl BuildCloudIdentityGroupMembershipPreferredMemberKeyEl {
    pub fn build(self) -> CloudIdentityGroupMembershipPreferredMemberKeyEl {
        CloudIdentityGroupMembershipPreferredMemberKeyEl {
            id: self.id,
            namespace: core::default::Default::default(),
        }
    }
}
pub struct CloudIdentityGroupMembershipPreferredMemberKeyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudIdentityGroupMembershipPreferredMemberKeyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudIdentityGroupMembershipPreferredMemberKeyElRef {
        CloudIdentityGroupMembershipPreferredMemberKeyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudIdentityGroupMembershipPreferredMemberKeyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe ID of the entity.\n\nFor Google-managed entities, the id must be the email address of an existing\ngroup or user.\n\nFor external-identity-mapped entities, the id must be a string conforming\nto the Identity Source's requirements.\n\nMust be unique within a namespace."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
    #[doc = "Get a reference to the value of field `namespace` after provisioning.\nThe namespace in which the entity exists.\n\nIf not specified, the EntityKey represents a Google-managed entity\nsuch as a Google user or a Google Group.\n\nIf specified, the EntityKey represents an external-identity-mapped group.\nThe namespace must correspond to an identity source created in Admin Console\nand must be in the form of 'identitysources/{identity_source_id}'."]
    pub fn namespace(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.namespace", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudIdentityGroupMembershipRolesElExpiryDetailEl {
    expire_time: PrimField<String>,
}
impl CloudIdentityGroupMembershipRolesElExpiryDetailEl {}
impl ToListMappable for CloudIdentityGroupMembershipRolesElExpiryDetailEl {
    type O = BlockAssignable<CloudIdentityGroupMembershipRolesElExpiryDetailEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudIdentityGroupMembershipRolesElExpiryDetailEl {
    #[doc = "The time at which the MembershipRole will expire.\n\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond\nresolution and up to nine fractional digits.\n\nExamples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub expire_time: PrimField<String>,
}
impl BuildCloudIdentityGroupMembershipRolesElExpiryDetailEl {
    pub fn build(self) -> CloudIdentityGroupMembershipRolesElExpiryDetailEl {
        CloudIdentityGroupMembershipRolesElExpiryDetailEl {
            expire_time: self.expire_time,
        }
    }
}
pub struct CloudIdentityGroupMembershipRolesElExpiryDetailElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudIdentityGroupMembershipRolesElExpiryDetailElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudIdentityGroupMembershipRolesElExpiryDetailElRef {
        CloudIdentityGroupMembershipRolesElExpiryDetailElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudIdentityGroupMembershipRolesElExpiryDetailElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `expire_time` after provisioning.\nThe time at which the MembershipRole will expire.\n\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond\nresolution and up to nine fractional digits.\n\nExamples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn expire_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expire_time", self.base))
    }
}
#[derive(Serialize, Default)]
struct CloudIdentityGroupMembershipRolesElDynamic {
    expiry_detail: Option<DynamicBlock<CloudIdentityGroupMembershipRolesElExpiryDetailEl>>,
}
#[derive(Serialize)]
pub struct CloudIdentityGroupMembershipRolesEl {
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expiry_detail: Option<Vec<CloudIdentityGroupMembershipRolesElExpiryDetailEl>>,
    dynamic: CloudIdentityGroupMembershipRolesElDynamic,
}
impl CloudIdentityGroupMembershipRolesEl {
    #[doc = "Set the field `expiry_detail`.\n"]
    pub fn set_expiry_detail(
        mut self,
        v: impl Into<BlockAssignable<CloudIdentityGroupMembershipRolesElExpiryDetailEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.expiry_detail = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.expiry_detail = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CloudIdentityGroupMembershipRolesEl {
    type O = BlockAssignable<CloudIdentityGroupMembershipRolesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudIdentityGroupMembershipRolesEl {
    #[doc = "The name of the MembershipRole. Must be one of OWNER, MANAGER, MEMBER. Possible values: [\"OWNER\", \"MANAGER\", \"MEMBER\"]"]
    pub name: PrimField<String>,
}
impl BuildCloudIdentityGroupMembershipRolesEl {
    pub fn build(self) -> CloudIdentityGroupMembershipRolesEl {
        CloudIdentityGroupMembershipRolesEl {
            name: self.name,
            expiry_detail: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CloudIdentityGroupMembershipRolesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudIdentityGroupMembershipRolesElRef {
    fn new(shared: StackShared, base: String) -> CloudIdentityGroupMembershipRolesElRef {
        CloudIdentityGroupMembershipRolesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudIdentityGroupMembershipRolesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the MembershipRole. Must be one of OWNER, MANAGER, MEMBER. Possible values: [\"OWNER\", \"MANAGER\", \"MEMBER\"]"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `expiry_detail` after provisioning.\n"]
    pub fn expiry_detail(&self) -> ListRef<CloudIdentityGroupMembershipRolesElExpiryDetailElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.expiry_detail", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CloudIdentityGroupMembershipTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl CloudIdentityGroupMembershipTimeoutsEl {
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
impl ToListMappable for CloudIdentityGroupMembershipTimeoutsEl {
    type O = BlockAssignable<CloudIdentityGroupMembershipTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudIdentityGroupMembershipTimeoutsEl {}
impl BuildCloudIdentityGroupMembershipTimeoutsEl {
    pub fn build(self) -> CloudIdentityGroupMembershipTimeoutsEl {
        CloudIdentityGroupMembershipTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct CloudIdentityGroupMembershipTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudIdentityGroupMembershipTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> CloudIdentityGroupMembershipTimeoutsElRef {
        CloudIdentityGroupMembershipTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudIdentityGroupMembershipTimeoutsElRef {
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
struct CloudIdentityGroupMembershipDynamic {
    preferred_member_key: Option<DynamicBlock<CloudIdentityGroupMembershipPreferredMemberKeyEl>>,
    roles: Option<DynamicBlock<CloudIdentityGroupMembershipRolesEl>>,
}
