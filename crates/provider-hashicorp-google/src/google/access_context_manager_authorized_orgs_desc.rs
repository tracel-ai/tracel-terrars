use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct AccessContextManagerAuthorizedOrgsDescData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    asset_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authorization_direction: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authorization_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    orgs: Option<ListField<PrimField<String>>>,
    parent: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<AccessContextManagerAuthorizedOrgsDescTimeoutsEl>,
}
struct AccessContextManagerAuthorizedOrgsDesc_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<AccessContextManagerAuthorizedOrgsDescData>,
}
#[derive(Clone)]
pub struct AccessContextManagerAuthorizedOrgsDesc(Rc<AccessContextManagerAuthorizedOrgsDesc_>);
impl AccessContextManagerAuthorizedOrgsDesc {
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
    #[doc = "Set the field `asset_type`.\nThe type of entities that need to use the authorization relationship during\nevaluation, such as a device. Valid values are \"ASSET_TYPE_DEVICE\" and\n\"ASSET_TYPE_CREDENTIAL_STRENGTH\". Possible values: [\"ASSET_TYPE_DEVICE\", \"ASSET_TYPE_CREDENTIAL_STRENGTH\"]"]
    pub fn set_asset_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().asset_type = Some(v.into());
        self
    }
    #[doc = "Set the field `authorization_direction`.\nThe direction of the authorization relationship between this organization\nand the organizations listed in the \"orgs\" field. The valid values for this\nfield include the following:\n\nAUTHORIZATION_DIRECTION_FROM: Allows this organization to evaluate traffic\nin the organizations listed in the 'orgs' field.\n\nAUTHORIZATION_DIRECTION_TO: Allows the organizations listed in the 'orgs'\nfield to evaluate the traffic in this organization.\n\nFor the authorization relationship to take effect, all of the organizations\nmust authorize and specify the appropriate relationship direction. For\nexample, if organization A authorized organization B and C to evaluate its\ntraffic, by specifying \"AUTHORIZATION_DIRECTION_TO\" as the authorization\ndirection, organizations B and C must specify\n\"AUTHORIZATION_DIRECTION_FROM\" as the authorization direction in their\n\"AuthorizedOrgsDesc\" resource. Possible values: [\"AUTHORIZATION_DIRECTION_TO\", \"AUTHORIZATION_DIRECTION_FROM\"]"]
    pub fn set_authorization_direction(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().authorization_direction = Some(v.into());
        self
    }
    #[doc = "Set the field `authorization_type`.\nA granular control type for authorization levels. Valid value is \"AUTHORIZATION_TYPE_TRUST\". Possible values: [\"AUTHORIZATION_TYPE_TRUST\"]"]
    pub fn set_authorization_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().authorization_type = Some(v.into());
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
    #[doc = "Set the field `orgs`.\nThe list of organization ids in this AuthorizedOrgsDesc.\nFormat: 'organizations/<org_number>'\nExample: 'organizations/123456'"]
    pub fn set_orgs(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().orgs = Some(v.into());
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(
        self,
        v: impl Into<AccessContextManagerAuthorizedOrgsDescTimeoutsEl>,
    ) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `asset_type` after provisioning.\nThe type of entities that need to use the authorization relationship during\nevaluation, such as a device. Valid values are \"ASSET_TYPE_DEVICE\" and\n\"ASSET_TYPE_CREDENTIAL_STRENGTH\". Possible values: [\"ASSET_TYPE_DEVICE\", \"ASSET_TYPE_CREDENTIAL_STRENGTH\"]"]
    pub fn asset_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.asset_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `authorization_direction` after provisioning.\nThe direction of the authorization relationship between this organization\nand the organizations listed in the \"orgs\" field. The valid values for this\nfield include the following:\n\nAUTHORIZATION_DIRECTION_FROM: Allows this organization to evaluate traffic\nin the organizations listed in the 'orgs' field.\n\nAUTHORIZATION_DIRECTION_TO: Allows the organizations listed in the 'orgs'\nfield to evaluate the traffic in this organization.\n\nFor the authorization relationship to take effect, all of the organizations\nmust authorize and specify the appropriate relationship direction. For\nexample, if organization A authorized organization B and C to evaluate its\ntraffic, by specifying \"AUTHORIZATION_DIRECTION_TO\" as the authorization\ndirection, organizations B and C must specify\n\"AUTHORIZATION_DIRECTION_FROM\" as the authorization direction in their\n\"AuthorizedOrgsDesc\" resource. Possible values: [\"AUTHORIZATION_DIRECTION_TO\", \"AUTHORIZATION_DIRECTION_FROM\"]"]
    pub fn authorization_direction(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.authorization_direction", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `authorization_type` after provisioning.\nA granular control type for authorization levels. Valid value is \"AUTHORIZATION_TYPE_TRUST\". Possible values: [\"AUTHORIZATION_TYPE_TRUST\"]"]
    pub fn authorization_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.authorization_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTime the AuthorizedOrgsDesc was created in UTC."]
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
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nResource name for the 'AuthorizedOrgsDesc'. Format:\n'accessPolicies/{access_policy}/authorizedOrgsDescs/{authorized_orgs_desc}'.\nThe 'authorized_orgs_desc' component must begin with a letter, followed by\nalphanumeric characters or '_'.\nAfter you create an 'AuthorizedOrgsDesc', you cannot change its 'name'."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `orgs` after provisioning.\nThe list of organization ids in this AuthorizedOrgsDesc.\nFormat: 'organizations/<org_number>'\nExample: 'organizations/123456'"]
    pub fn orgs(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.orgs", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\nRequired. Resource name for the access policy which owns this 'AuthorizedOrgsDesc'."]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nTime the AuthorizedOrgsDesc was updated in UTC."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AccessContextManagerAuthorizedOrgsDescTimeoutsElRef {
        AccessContextManagerAuthorizedOrgsDescTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for AccessContextManagerAuthorizedOrgsDesc {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for AccessContextManagerAuthorizedOrgsDesc {}
impl ToListMappable for AccessContextManagerAuthorizedOrgsDesc {
    type O = ListRef<AccessContextManagerAuthorizedOrgsDescRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for AccessContextManagerAuthorizedOrgsDesc_ {
    fn extract_resource_type(&self) -> String {
        "google_access_context_manager_authorized_orgs_desc".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildAccessContextManagerAuthorizedOrgsDesc {
    pub tf_id: String,
    #[doc = "Resource name for the 'AuthorizedOrgsDesc'. Format:\n'accessPolicies/{access_policy}/authorizedOrgsDescs/{authorized_orgs_desc}'.\nThe 'authorized_orgs_desc' component must begin with a letter, followed by\nalphanumeric characters or '_'.\nAfter you create an 'AuthorizedOrgsDesc', you cannot change its 'name'."]
    pub name: PrimField<String>,
    #[doc = "Required. Resource name for the access policy which owns this 'AuthorizedOrgsDesc'."]
    pub parent: PrimField<String>,
}
impl BuildAccessContextManagerAuthorizedOrgsDesc {
    pub fn build(self, stack: &mut Stack) -> AccessContextManagerAuthorizedOrgsDesc {
        let out = AccessContextManagerAuthorizedOrgsDesc(Rc::new(
            AccessContextManagerAuthorizedOrgsDesc_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(AccessContextManagerAuthorizedOrgsDescData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    lifecycle: core::default::Default::default(),
                    for_each: None,
                    asset_type: core::default::Default::default(),
                    authorization_direction: core::default::Default::default(),
                    authorization_type: core::default::Default::default(),
                    deletion_policy: core::default::Default::default(),
                    id: core::default::Default::default(),
                    name: self.name,
                    orgs: core::default::Default::default(),
                    parent: self.parent,
                    timeouts: core::default::Default::default(),
                }),
            },
        ));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct AccessContextManagerAuthorizedOrgsDescRef {
    shared: StackShared,
    base: String,
}
impl Ref for AccessContextManagerAuthorizedOrgsDescRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl AccessContextManagerAuthorizedOrgsDescRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `asset_type` after provisioning.\nThe type of entities that need to use the authorization relationship during\nevaluation, such as a device. Valid values are \"ASSET_TYPE_DEVICE\" and\n\"ASSET_TYPE_CREDENTIAL_STRENGTH\". Possible values: [\"ASSET_TYPE_DEVICE\", \"ASSET_TYPE_CREDENTIAL_STRENGTH\"]"]
    pub fn asset_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.asset_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `authorization_direction` after provisioning.\nThe direction of the authorization relationship between this organization\nand the organizations listed in the \"orgs\" field. The valid values for this\nfield include the following:\n\nAUTHORIZATION_DIRECTION_FROM: Allows this organization to evaluate traffic\nin the organizations listed in the 'orgs' field.\n\nAUTHORIZATION_DIRECTION_TO: Allows the organizations listed in the 'orgs'\nfield to evaluate the traffic in this organization.\n\nFor the authorization relationship to take effect, all of the organizations\nmust authorize and specify the appropriate relationship direction. For\nexample, if organization A authorized organization B and C to evaluate its\ntraffic, by specifying \"AUTHORIZATION_DIRECTION_TO\" as the authorization\ndirection, organizations B and C must specify\n\"AUTHORIZATION_DIRECTION_FROM\" as the authorization direction in their\n\"AuthorizedOrgsDesc\" resource. Possible values: [\"AUTHORIZATION_DIRECTION_TO\", \"AUTHORIZATION_DIRECTION_FROM\"]"]
    pub fn authorization_direction(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.authorization_direction", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `authorization_type` after provisioning.\nA granular control type for authorization levels. Valid value is \"AUTHORIZATION_TYPE_TRUST\". Possible values: [\"AUTHORIZATION_TYPE_TRUST\"]"]
    pub fn authorization_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.authorization_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTime the AuthorizedOrgsDesc was created in UTC."]
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
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nResource name for the 'AuthorizedOrgsDesc'. Format:\n'accessPolicies/{access_policy}/authorizedOrgsDescs/{authorized_orgs_desc}'.\nThe 'authorized_orgs_desc' component must begin with a letter, followed by\nalphanumeric characters or '_'.\nAfter you create an 'AuthorizedOrgsDesc', you cannot change its 'name'."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `orgs` after provisioning.\nThe list of organization ids in this AuthorizedOrgsDesc.\nFormat: 'organizations/<org_number>'\nExample: 'organizations/123456'"]
    pub fn orgs(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.orgs", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\nRequired. Resource name for the access policy which owns this 'AuthorizedOrgsDesc'."]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nTime the AuthorizedOrgsDesc was updated in UTC."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AccessContextManagerAuthorizedOrgsDescTimeoutsElRef {
        AccessContextManagerAuthorizedOrgsDescTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct AccessContextManagerAuthorizedOrgsDescTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl AccessContextManagerAuthorizedOrgsDescTimeoutsEl {
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
impl ToListMappable for AccessContextManagerAuthorizedOrgsDescTimeoutsEl {
    type O = BlockAssignable<AccessContextManagerAuthorizedOrgsDescTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAccessContextManagerAuthorizedOrgsDescTimeoutsEl {}
impl BuildAccessContextManagerAuthorizedOrgsDescTimeoutsEl {
    pub fn build(self) -> AccessContextManagerAuthorizedOrgsDescTimeoutsEl {
        AccessContextManagerAuthorizedOrgsDescTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct AccessContextManagerAuthorizedOrgsDescTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AccessContextManagerAuthorizedOrgsDescTimeoutsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AccessContextManagerAuthorizedOrgsDescTimeoutsElRef {
        AccessContextManagerAuthorizedOrgsDescTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AccessContextManagerAuthorizedOrgsDescTimeoutsElRef {
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
