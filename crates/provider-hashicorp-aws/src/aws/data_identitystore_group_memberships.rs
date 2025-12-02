use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataIdentitystoreGroupMembershipsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    group_id: PrimField<String>,
    identity_store_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}
struct DataIdentitystoreGroupMemberships_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataIdentitystoreGroupMembershipsData>,
}
#[derive(Clone)]
pub struct DataIdentitystoreGroupMemberships(Rc<DataIdentitystoreGroupMemberships_>);
impl DataIdentitystoreGroupMemberships {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(&self, provider: &ProviderAws) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `group_id` after provisioning.\n"]
    pub fn group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.group_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `group_memberships` after provisioning.\n"]
    pub fn group_memberships(
        &self,
    ) -> ListRef<DataIdentitystoreGroupMembershipsGroupMembershipsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.group_memberships", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `identity_store_id` after provisioning.\n"]
    pub fn identity_store_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.identity_store_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
}
impl Referable for DataIdentitystoreGroupMemberships {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataIdentitystoreGroupMemberships {}
impl ToListMappable for DataIdentitystoreGroupMemberships {
    type O = ListRef<DataIdentitystoreGroupMembershipsRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataIdentitystoreGroupMemberships_ {
    fn extract_datasource_type(&self) -> String {
        "aws_identitystore_group_memberships".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataIdentitystoreGroupMemberships {
    pub tf_id: String,
    #[doc = ""]
    pub group_id: PrimField<String>,
    #[doc = ""]
    pub identity_store_id: PrimField<String>,
}
impl BuildDataIdentitystoreGroupMemberships {
    pub fn build(self, stack: &mut Stack) -> DataIdentitystoreGroupMemberships {
        let out = DataIdentitystoreGroupMemberships(Rc::new(DataIdentitystoreGroupMemberships_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataIdentitystoreGroupMembershipsData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                group_id: self.group_id,
                identity_store_id: self.identity_store_id,
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataIdentitystoreGroupMembershipsRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataIdentitystoreGroupMembershipsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataIdentitystoreGroupMembershipsRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `group_id` after provisioning.\n"]
    pub fn group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.group_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `group_memberships` after provisioning.\n"]
    pub fn group_memberships(
        &self,
    ) -> ListRef<DataIdentitystoreGroupMembershipsGroupMembershipsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.group_memberships", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `identity_store_id` after provisioning.\n"]
    pub fn identity_store_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.identity_store_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataIdentitystoreGroupMembershipsGroupMembershipsElMemberId {
    #[serde(skip_serializing_if = "Option::is_none")]
    user_id: Option<PrimField<String>>,
}
impl DataIdentitystoreGroupMembershipsGroupMembershipsElMemberId {
    #[doc = "Set the field `user_id`.\n"]
    pub fn set_user_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.user_id = Some(v.into());
        self
    }
}
impl ToListMappable for DataIdentitystoreGroupMembershipsGroupMembershipsElMemberId {
    type O = BlockAssignable<DataIdentitystoreGroupMembershipsGroupMembershipsElMemberId>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataIdentitystoreGroupMembershipsGroupMembershipsElMemberId {}
impl BuildDataIdentitystoreGroupMembershipsGroupMembershipsElMemberId {
    pub fn build(self) -> DataIdentitystoreGroupMembershipsGroupMembershipsElMemberId {
        DataIdentitystoreGroupMembershipsGroupMembershipsElMemberId {
            user_id: core::default::Default::default(),
        }
    }
}
pub struct DataIdentitystoreGroupMembershipsGroupMembershipsElMemberIdRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataIdentitystoreGroupMembershipsGroupMembershipsElMemberIdRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataIdentitystoreGroupMembershipsGroupMembershipsElMemberIdRef {
        DataIdentitystoreGroupMembershipsGroupMembershipsElMemberIdRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataIdentitystoreGroupMembershipsGroupMembershipsElMemberIdRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `user_id` after provisioning.\n"]
    pub fn user_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_id", self.base))
    }
}
#[derive(Serialize)]
pub struct DataIdentitystoreGroupMembershipsGroupMembershipsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    group_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    identity_store_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    member_id: Option<DataIdentitystoreGroupMembershipsGroupMembershipsElMemberId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    membership_id: Option<PrimField<String>>,
}
impl DataIdentitystoreGroupMembershipsGroupMembershipsEl {
    #[doc = "Set the field `group_id`.\n"]
    pub fn set_group_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.group_id = Some(v.into());
        self
    }
    #[doc = "Set the field `identity_store_id`.\n"]
    pub fn set_identity_store_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.identity_store_id = Some(v.into());
        self
    }
    #[doc = "Set the field `member_id`.\n"]
    pub fn set_member_id(
        mut self,
        v: impl Into<DataIdentitystoreGroupMembershipsGroupMembershipsElMemberId>,
    ) -> Self {
        self.member_id = Some(v.into());
        self
    }
    #[doc = "Set the field `membership_id`.\n"]
    pub fn set_membership_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.membership_id = Some(v.into());
        self
    }
}
impl ToListMappable for DataIdentitystoreGroupMembershipsGroupMembershipsEl {
    type O = BlockAssignable<DataIdentitystoreGroupMembershipsGroupMembershipsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataIdentitystoreGroupMembershipsGroupMembershipsEl {}
impl BuildDataIdentitystoreGroupMembershipsGroupMembershipsEl {
    pub fn build(self) -> DataIdentitystoreGroupMembershipsGroupMembershipsEl {
        DataIdentitystoreGroupMembershipsGroupMembershipsEl {
            group_id: core::default::Default::default(),
            identity_store_id: core::default::Default::default(),
            member_id: core::default::Default::default(),
            membership_id: core::default::Default::default(),
        }
    }
}
pub struct DataIdentitystoreGroupMembershipsGroupMembershipsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataIdentitystoreGroupMembershipsGroupMembershipsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataIdentitystoreGroupMembershipsGroupMembershipsElRef {
        DataIdentitystoreGroupMembershipsGroupMembershipsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataIdentitystoreGroupMembershipsGroupMembershipsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `group_id` after provisioning.\n"]
    pub fn group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_id", self.base))
    }
    #[doc = "Get a reference to the value of field `identity_store_id` after provisioning.\n"]
    pub fn identity_store_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.identity_store_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `member_id` after provisioning.\n"]
    pub fn member_id(&self) -> DataIdentitystoreGroupMembershipsGroupMembershipsElMemberIdRef {
        DataIdentitystoreGroupMembershipsGroupMembershipsElMemberIdRef::new(
            self.shared().clone(),
            format!("{}.member_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `membership_id` after provisioning.\n"]
    pub fn membership_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.membership_id", self.base),
        )
    }
}
