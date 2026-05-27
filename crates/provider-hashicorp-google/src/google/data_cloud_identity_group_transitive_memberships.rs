use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataCloudIdentityGroupTransitiveMembershipsData {
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
struct DataCloudIdentityGroupTransitiveMemberships_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataCloudIdentityGroupTransitiveMembershipsData>,
}
#[derive(Clone)]
pub struct DataCloudIdentityGroupTransitiveMemberships(
    Rc<DataCloudIdentityGroupTransitiveMemberships_>,
);
impl DataCloudIdentityGroupTransitiveMemberships {
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
    pub fn memberships(
        &self,
    ) -> ListRef<DataCloudIdentityGroupTransitiveMembershipsMembershipsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.memberships", self.extract_ref()),
        )
    }
}
impl Referable for DataCloudIdentityGroupTransitiveMemberships {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataCloudIdentityGroupTransitiveMemberships {}
impl ToListMappable for DataCloudIdentityGroupTransitiveMemberships {
    type O = ListRef<DataCloudIdentityGroupTransitiveMembershipsRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataCloudIdentityGroupTransitiveMemberships_ {
    fn extract_datasource_type(&self) -> String {
        "google_cloud_identity_group_transitive_memberships".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataCloudIdentityGroupTransitiveMemberships {
    pub tf_id: String,
    #[doc = "The name of the Group to get memberships from."]
    pub group: PrimField<String>,
}
impl BuildDataCloudIdentityGroupTransitiveMemberships {
    pub fn build(self, stack: &mut Stack) -> DataCloudIdentityGroupTransitiveMemberships {
        let out = DataCloudIdentityGroupTransitiveMemberships(Rc::new(
            DataCloudIdentityGroupTransitiveMemberships_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(DataCloudIdentityGroupTransitiveMembershipsData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    for_each: None,
                    group: self.group,
                    id: core::default::Default::default(),
                }),
            },
        ));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataCloudIdentityGroupTransitiveMembershipsRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudIdentityGroupTransitiveMembershipsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataCloudIdentityGroupTransitiveMembershipsRef {
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
    pub fn memberships(
        &self,
    ) -> ListRef<DataCloudIdentityGroupTransitiveMembershipsMembershipsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.memberships", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataCloudIdentityGroupTransitiveMembershipsMembershipsElPreferredMemberKeyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    namespace: Option<PrimField<String>>,
}
impl DataCloudIdentityGroupTransitiveMembershipsMembershipsElPreferredMemberKeyEl {
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
impl ToListMappable
    for DataCloudIdentityGroupTransitiveMembershipsMembershipsElPreferredMemberKeyEl
{
    type O = BlockAssignable<
        DataCloudIdentityGroupTransitiveMembershipsMembershipsElPreferredMemberKeyEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudIdentityGroupTransitiveMembershipsMembershipsElPreferredMemberKeyEl {}
impl BuildDataCloudIdentityGroupTransitiveMembershipsMembershipsElPreferredMemberKeyEl {
    pub fn build(
        self,
    ) -> DataCloudIdentityGroupTransitiveMembershipsMembershipsElPreferredMemberKeyEl {
        DataCloudIdentityGroupTransitiveMembershipsMembershipsElPreferredMemberKeyEl {
            id: core::default::Default::default(),
            namespace: core::default::Default::default(),
        }
    }
}
pub struct DataCloudIdentityGroupTransitiveMembershipsMembershipsElPreferredMemberKeyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudIdentityGroupTransitiveMembershipsMembershipsElPreferredMemberKeyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudIdentityGroupTransitiveMembershipsMembershipsElPreferredMemberKeyElRef {
        DataCloudIdentityGroupTransitiveMembershipsMembershipsElPreferredMemberKeyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudIdentityGroupTransitiveMembershipsMembershipsElPreferredMemberKeyElRef {
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
pub struct DataCloudIdentityGroupTransitiveMembershipsMembershipsElRolesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    role: Option<PrimField<String>>,
}
impl DataCloudIdentityGroupTransitiveMembershipsMembershipsElRolesEl {
    #[doc = "Set the field `role`.\n"]
    pub fn set_role(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.role = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudIdentityGroupTransitiveMembershipsMembershipsElRolesEl {
    type O = BlockAssignable<DataCloudIdentityGroupTransitiveMembershipsMembershipsElRolesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudIdentityGroupTransitiveMembershipsMembershipsElRolesEl {}
impl BuildDataCloudIdentityGroupTransitiveMembershipsMembershipsElRolesEl {
    pub fn build(self) -> DataCloudIdentityGroupTransitiveMembershipsMembershipsElRolesEl {
        DataCloudIdentityGroupTransitiveMembershipsMembershipsElRolesEl {
            role: core::default::Default::default(),
        }
    }
}
pub struct DataCloudIdentityGroupTransitiveMembershipsMembershipsElRolesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudIdentityGroupTransitiveMembershipsMembershipsElRolesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudIdentityGroupTransitiveMembershipsMembershipsElRolesElRef {
        DataCloudIdentityGroupTransitiveMembershipsMembershipsElRolesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudIdentityGroupTransitiveMembershipsMembershipsElRolesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `role` after provisioning.\n"]
    pub fn role(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudIdentityGroupTransitiveMembershipsMembershipsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    member: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preferred_member_key: Option<
        ListField<DataCloudIdentityGroupTransitiveMembershipsMembershipsElPreferredMemberKeyEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    relation_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    roles: Option<SetField<DataCloudIdentityGroupTransitiveMembershipsMembershipsElRolesEl>>,
}
impl DataCloudIdentityGroupTransitiveMembershipsMembershipsEl {
    #[doc = "Set the field `member`.\n"]
    pub fn set_member(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.member = Some(v.into());
        self
    }
    #[doc = "Set the field `preferred_member_key`.\n"]
    pub fn set_preferred_member_key(
        mut self,
        v: impl Into<
            ListField<DataCloudIdentityGroupTransitiveMembershipsMembershipsElPreferredMemberKeyEl>,
        >,
    ) -> Self {
        self.preferred_member_key = Some(v.into());
        self
    }
    #[doc = "Set the field `relation_type`.\n"]
    pub fn set_relation_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.relation_type = Some(v.into());
        self
    }
    #[doc = "Set the field `roles`.\n"]
    pub fn set_roles(
        mut self,
        v: impl Into<SetField<DataCloudIdentityGroupTransitiveMembershipsMembershipsElRolesEl>>,
    ) -> Self {
        self.roles = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudIdentityGroupTransitiveMembershipsMembershipsEl {
    type O = BlockAssignable<DataCloudIdentityGroupTransitiveMembershipsMembershipsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudIdentityGroupTransitiveMembershipsMembershipsEl {}
impl BuildDataCloudIdentityGroupTransitiveMembershipsMembershipsEl {
    pub fn build(self) -> DataCloudIdentityGroupTransitiveMembershipsMembershipsEl {
        DataCloudIdentityGroupTransitiveMembershipsMembershipsEl {
            member: core::default::Default::default(),
            preferred_member_key: core::default::Default::default(),
            relation_type: core::default::Default::default(),
            roles: core::default::Default::default(),
        }
    }
}
pub struct DataCloudIdentityGroupTransitiveMembershipsMembershipsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudIdentityGroupTransitiveMembershipsMembershipsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudIdentityGroupTransitiveMembershipsMembershipsElRef {
        DataCloudIdentityGroupTransitiveMembershipsMembershipsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudIdentityGroupTransitiveMembershipsMembershipsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `member` after provisioning.\n"]
    pub fn member(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.member", self.base))
    }
    #[doc = "Get a reference to the value of field `preferred_member_key` after provisioning.\n"]
    pub fn preferred_member_key(
        &self,
    ) -> ListRef<DataCloudIdentityGroupTransitiveMembershipsMembershipsElPreferredMemberKeyElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.preferred_member_key", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `relation_type` after provisioning.\n"]
    pub fn relation_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.relation_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `roles` after provisioning.\n"]
    pub fn roles(
        &self,
    ) -> SetRef<DataCloudIdentityGroupTransitiveMembershipsMembershipsElRolesElRef> {
        SetRef::new(self.shared().clone(), format!("{}.roles", self.base))
    }
}
