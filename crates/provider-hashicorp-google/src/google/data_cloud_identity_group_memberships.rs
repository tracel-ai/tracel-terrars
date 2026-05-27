use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataCloudIdentityGroupMembershipsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    group: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
}
struct DataCloudIdentityGroupMemberships_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataCloudIdentityGroupMembershipsData>,
}
#[derive(Clone)]
pub struct DataCloudIdentityGroupMemberships(Rc<DataCloudIdentityGroupMemberships_>);
impl DataCloudIdentityGroupMemberships {
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
    #[doc = "Get a reference to the value of field `group` after provisioning.\nThe name of the Group to get memberships from."]
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
    #[doc = "Get a reference to the value of field `memberships` after provisioning.\nList of Cloud Identity group memberships."]
    pub fn memberships(&self) -> ListRef<DataCloudIdentityGroupMembershipsMembershipsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.memberships", self.extract_ref()),
        )
    }
}
impl Referable for DataCloudIdentityGroupMemberships {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataCloudIdentityGroupMemberships {}
impl ToListMappable for DataCloudIdentityGroupMemberships {
    type O = ListRef<DataCloudIdentityGroupMembershipsRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataCloudIdentityGroupMemberships_ {
    fn extract_datasource_type(&self) -> String {
        "google_cloud_identity_group_memberships".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataCloudIdentityGroupMemberships {
    pub tf_id: String,
    #[doc = "The name of the Group to get memberships from."]
    pub group: PrimField<String>,
}
impl BuildDataCloudIdentityGroupMemberships {
    pub fn build(self, stack: &mut Stack) -> DataCloudIdentityGroupMemberships {
        let out = DataCloudIdentityGroupMemberships(Rc::new(DataCloudIdentityGroupMemberships_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataCloudIdentityGroupMembershipsData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                group: self.group,
                id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataCloudIdentityGroupMembershipsRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudIdentityGroupMembershipsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataCloudIdentityGroupMembershipsRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `group` after provisioning.\nThe name of the Group to get memberships from."]
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
    #[doc = "Get a reference to the value of field `memberships` after provisioning.\nList of Cloud Identity group memberships."]
    pub fn memberships(&self) -> ListRef<DataCloudIdentityGroupMembershipsMembershipsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.memberships", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataCloudIdentityGroupMembershipsMembershipsElPreferredMemberKeyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    namespace: Option<PrimField<String>>,
}
impl DataCloudIdentityGroupMembershipsMembershipsElPreferredMemberKeyEl {
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }
    #[doc = "Set the field `namespace`.\n"]
    pub fn set_namespace(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.namespace = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudIdentityGroupMembershipsMembershipsElPreferredMemberKeyEl {
    type O = BlockAssignable<DataCloudIdentityGroupMembershipsMembershipsElPreferredMemberKeyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudIdentityGroupMembershipsMembershipsElPreferredMemberKeyEl {}
impl BuildDataCloudIdentityGroupMembershipsMembershipsElPreferredMemberKeyEl {
    pub fn build(self) -> DataCloudIdentityGroupMembershipsMembershipsElPreferredMemberKeyEl {
        DataCloudIdentityGroupMembershipsMembershipsElPreferredMemberKeyEl {
            id: core::default::Default::default(),
            namespace: core::default::Default::default(),
        }
    }
}
pub struct DataCloudIdentityGroupMembershipsMembershipsElPreferredMemberKeyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudIdentityGroupMembershipsMembershipsElPreferredMemberKeyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudIdentityGroupMembershipsMembershipsElPreferredMemberKeyElRef {
        DataCloudIdentityGroupMembershipsMembershipsElPreferredMemberKeyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudIdentityGroupMembershipsMembershipsElPreferredMemberKeyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
    #[doc = "Get a reference to the value of field `namespace` after provisioning.\n"]
    pub fn namespace(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.namespace", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudIdentityGroupMembershipsMembershipsElRolesElExpiryDetailEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    expire_time: Option<PrimField<String>>,
}
impl DataCloudIdentityGroupMembershipsMembershipsElRolesElExpiryDetailEl {
    #[doc = "Set the field `expire_time`.\n"]
    pub fn set_expire_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.expire_time = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudIdentityGroupMembershipsMembershipsElRolesElExpiryDetailEl {
    type O = BlockAssignable<DataCloudIdentityGroupMembershipsMembershipsElRolesElExpiryDetailEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudIdentityGroupMembershipsMembershipsElRolesElExpiryDetailEl {}
impl BuildDataCloudIdentityGroupMembershipsMembershipsElRolesElExpiryDetailEl {
    pub fn build(self) -> DataCloudIdentityGroupMembershipsMembershipsElRolesElExpiryDetailEl {
        DataCloudIdentityGroupMembershipsMembershipsElRolesElExpiryDetailEl {
            expire_time: core::default::Default::default(),
        }
    }
}
pub struct DataCloudIdentityGroupMembershipsMembershipsElRolesElExpiryDetailElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudIdentityGroupMembershipsMembershipsElRolesElExpiryDetailElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudIdentityGroupMembershipsMembershipsElRolesElExpiryDetailElRef {
        DataCloudIdentityGroupMembershipsMembershipsElRolesElExpiryDetailElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudIdentityGroupMembershipsMembershipsElRolesElExpiryDetailElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `expire_time` after provisioning.\n"]
    pub fn expire_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expire_time", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudIdentityGroupMembershipsMembershipsElRolesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    expiry_detail:
        Option<ListField<DataCloudIdentityGroupMembershipsMembershipsElRolesElExpiryDetailEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}
impl DataCloudIdentityGroupMembershipsMembershipsElRolesEl {
    #[doc = "Set the field `expiry_detail`.\n"]
    pub fn set_expiry_detail(
        mut self,
        v: impl Into<ListField<DataCloudIdentityGroupMembershipsMembershipsElRolesElExpiryDetailEl>>,
    ) -> Self {
        self.expiry_detail = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudIdentityGroupMembershipsMembershipsElRolesEl {
    type O = BlockAssignable<DataCloudIdentityGroupMembershipsMembershipsElRolesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudIdentityGroupMembershipsMembershipsElRolesEl {}
impl BuildDataCloudIdentityGroupMembershipsMembershipsElRolesEl {
    pub fn build(self) -> DataCloudIdentityGroupMembershipsMembershipsElRolesEl {
        DataCloudIdentityGroupMembershipsMembershipsElRolesEl {
            expiry_detail: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}
pub struct DataCloudIdentityGroupMembershipsMembershipsElRolesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudIdentityGroupMembershipsMembershipsElRolesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudIdentityGroupMembershipsMembershipsElRolesElRef {
        DataCloudIdentityGroupMembershipsMembershipsElRolesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudIdentityGroupMembershipsMembershipsElRolesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `expiry_detail` after provisioning.\n"]
    pub fn expiry_detail(
        &self,
    ) -> ListRef<DataCloudIdentityGroupMembershipsMembershipsElRolesElExpiryDetailElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.expiry_detail", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudIdentityGroupMembershipsMembershipsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create_ignore_already_exists: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    create_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preferred_member_key:
        Option<ListField<DataCloudIdentityGroupMembershipsMembershipsElPreferredMemberKeyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    roles: Option<SetField<DataCloudIdentityGroupMembershipsMembershipsElRolesEl>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update_time: Option<PrimField<String>>,
}
impl DataCloudIdentityGroupMembershipsMembershipsEl {
    #[doc = "Set the field `create_ignore_already_exists`.\n"]
    pub fn set_create_ignore_already_exists(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.create_ignore_already_exists = Some(v.into());
        self
    }
    #[doc = "Set the field `create_time`.\n"]
    pub fn set_create_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create_time = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\n"]
    pub fn set_deletion_policy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `group`.\n"]
    pub fn set_group(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.group = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `preferred_member_key`.\n"]
    pub fn set_preferred_member_key(
        mut self,
        v: impl Into<ListField<DataCloudIdentityGroupMembershipsMembershipsElPreferredMemberKeyEl>>,
    ) -> Self {
        self.preferred_member_key = Some(v.into());
        self
    }
    #[doc = "Set the field `roles`.\n"]
    pub fn set_roles(
        mut self,
        v: impl Into<SetField<DataCloudIdentityGroupMembershipsMembershipsElRolesEl>>,
    ) -> Self {
        self.roles = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
    #[doc = "Set the field `update_time`.\n"]
    pub fn set_update_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update_time = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudIdentityGroupMembershipsMembershipsEl {
    type O = BlockAssignable<DataCloudIdentityGroupMembershipsMembershipsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudIdentityGroupMembershipsMembershipsEl {}
impl BuildDataCloudIdentityGroupMembershipsMembershipsEl {
    pub fn build(self) -> DataCloudIdentityGroupMembershipsMembershipsEl {
        DataCloudIdentityGroupMembershipsMembershipsEl {
            create_ignore_already_exists: core::default::Default::default(),
            create_time: core::default::Default::default(),
            deletion_policy: core::default::Default::default(),
            group: core::default::Default::default(),
            name: core::default::Default::default(),
            preferred_member_key: core::default::Default::default(),
            roles: core::default::Default::default(),
            type_: core::default::Default::default(),
            update_time: core::default::Default::default(),
        }
    }
}
pub struct DataCloudIdentityGroupMembershipsMembershipsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudIdentityGroupMembershipsMembershipsElRef {
    fn new(shared: StackShared, base: String) -> DataCloudIdentityGroupMembershipsMembershipsElRef {
        DataCloudIdentityGroupMembershipsMembershipsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudIdentityGroupMembershipsMembershipsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_ignore_already_exists` after provisioning.\n"]
    pub fn create_ignore_already_exists(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_ignore_already_exists", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.base))
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `group` after provisioning.\n"]
    pub fn group(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.group", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `preferred_member_key` after provisioning.\n"]
    pub fn preferred_member_key(
        &self,
    ) -> ListRef<DataCloudIdentityGroupMembershipsMembershipsElPreferredMemberKeyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.preferred_member_key", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `roles` after provisioning.\n"]
    pub fn roles(&self) -> SetRef<DataCloudIdentityGroupMembershipsMembershipsElRolesElRef> {
        SetRef::new(self.shared().clone(), format!("{}.roles", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n"]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }
}
