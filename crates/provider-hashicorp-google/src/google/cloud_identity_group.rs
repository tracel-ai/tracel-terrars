use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct CloudIdentityGroupData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    initial_group_config: Option<PrimField<String>>,
    labels: RecField<PrimField<String>>,
    parent: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group_key: Option<Vec<CloudIdentityGroupGroupKeyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<CloudIdentityGroupTimeoutsEl>,
    dynamic: CloudIdentityGroupDynamic,
}
struct CloudIdentityGroup_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CloudIdentityGroupData>,
}
#[derive(Clone)]
pub struct CloudIdentityGroup(Rc<CloudIdentityGroup_>);
impl CloudIdentityGroup {
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
    #[doc = "Set the field `description`.\nAn extended description to help users determine the purpose of a Group.\nMust not be longer than 4,096 characters."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\nThe display name of the Group."]
    pub fn set_display_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `initial_group_config`.\nThe initial configuration options for creating a Group.\n\nSee the\n[API reference](https://cloud.google.com/identity/docs/reference/rest/v1beta1/groups/create#initialgroupconfig)\nfor possible values. Default value: \"EMPTY\" Possible values: [\"INITIAL_GROUP_CONFIG_UNSPECIFIED\", \"WITH_INITIAL_OWNER\", \"EMPTY\"]"]
    pub fn set_initial_group_config(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().initial_group_config = Some(v.into());
        self
    }
    #[doc = "Set the field `group_key`.\n"]
    pub fn set_group_key(
        self,
        v: impl Into<BlockAssignable<CloudIdentityGroupGroupKeyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().group_key = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.group_key = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<CloudIdentityGroupTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `additional_group_keys` after provisioning.\nAdditional group keys associated with the Group"]
    pub fn additional_group_keys(&self) -> ListRef<CloudIdentityGroupAdditionalGroupKeysElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.additional_group_keys", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe time when the Group was created."]
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nAn extended description to help users determine the purpose of a Group.\nMust not be longer than 4,096 characters."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the Group."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `initial_group_config` after provisioning.\nThe initial configuration options for creating a Group.\n\nSee the\n[API reference](https://cloud.google.com/identity/docs/reference/rest/v1beta1/groups/create#initialgroupconfig)\nfor possible values. Default value: \"EMPTY\" Possible values: [\"INITIAL_GROUP_CONFIG_UNSPECIFIED\", \"WITH_INITIAL_OWNER\", \"EMPTY\"]"]
    pub fn initial_group_config(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.initial_group_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nOne or more label entries that apply to the Group. Currently supported labels contain a key with an empty value.\n\nGoogle Groups are the default type of group and have a label with a key of cloudidentity.googleapis.com/groups.discussion_forum and an empty value.\n\nExisting Google Groups can have an additional label with a key of cloudidentity.googleapis.com/groups.security and an empty value added to them. This is an immutable change and the security label cannot be removed once added.\n\nDynamic groups have a label with a key of cloudidentity.googleapis.com/groups.dynamic.\n\nIdentity-mapped groups for Cloud Search have a label with a key of system/groups/external and an empty value."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nResource name of the Group in the format: groups/{group_id}, where group_id\nis the unique ID assigned to the Group."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\nThe resource name of the entity under which this Group resides in the\nCloud Identity resource hierarchy.\n\nMust be of the form identitysources/{identity_source_id} for external-identity-mapped\ngroups or customers/{customer_id} for Google Groups."]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe time when the Group was last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `group_key` after provisioning.\n"]
    pub fn group_key(&self) -> ListRef<CloudIdentityGroupGroupKeyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.group_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudIdentityGroupTimeoutsElRef {
        CloudIdentityGroupTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for CloudIdentityGroup {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for CloudIdentityGroup {}
impl ToListMappable for CloudIdentityGroup {
    type O = ListRef<CloudIdentityGroupRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for CloudIdentityGroup_ {
    fn extract_resource_type(&self) -> String {
        "google_cloud_identity_group".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildCloudIdentityGroup {
    pub tf_id: String,
    #[doc = "One or more label entries that apply to the Group. Currently supported labels contain a key with an empty value.\n\nGoogle Groups are the default type of group and have a label with a key of cloudidentity.googleapis.com/groups.discussion_forum and an empty value.\n\nExisting Google Groups can have an additional label with a key of cloudidentity.googleapis.com/groups.security and an empty value added to them. This is an immutable change and the security label cannot be removed once added.\n\nDynamic groups have a label with a key of cloudidentity.googleapis.com/groups.dynamic.\n\nIdentity-mapped groups for Cloud Search have a label with a key of system/groups/external and an empty value."]
    pub labels: RecField<PrimField<String>>,
    #[doc = "The resource name of the entity under which this Group resides in the\nCloud Identity resource hierarchy.\n\nMust be of the form identitysources/{identity_source_id} for external-identity-mapped\ngroups or customers/{customer_id} for Google Groups."]
    pub parent: PrimField<String>,
}
impl BuildCloudIdentityGroup {
    pub fn build(self, stack: &mut Stack) -> CloudIdentityGroup {
        let out = CloudIdentityGroup(Rc::new(CloudIdentityGroup_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(CloudIdentityGroupData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                initial_group_config: core::default::Default::default(),
                labels: self.labels,
                parent: self.parent,
                group_key: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct CloudIdentityGroupRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudIdentityGroupRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl CloudIdentityGroupRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `additional_group_keys` after provisioning.\nAdditional group keys associated with the Group"]
    pub fn additional_group_keys(&self) -> ListRef<CloudIdentityGroupAdditionalGroupKeysElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.additional_group_keys", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe time when the Group was created."]
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nAn extended description to help users determine the purpose of a Group.\nMust not be longer than 4,096 characters."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the Group."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `initial_group_config` after provisioning.\nThe initial configuration options for creating a Group.\n\nSee the\n[API reference](https://cloud.google.com/identity/docs/reference/rest/v1beta1/groups/create#initialgroupconfig)\nfor possible values. Default value: \"EMPTY\" Possible values: [\"INITIAL_GROUP_CONFIG_UNSPECIFIED\", \"WITH_INITIAL_OWNER\", \"EMPTY\"]"]
    pub fn initial_group_config(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.initial_group_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nOne or more label entries that apply to the Group. Currently supported labels contain a key with an empty value.\n\nGoogle Groups are the default type of group and have a label with a key of cloudidentity.googleapis.com/groups.discussion_forum and an empty value.\n\nExisting Google Groups can have an additional label with a key of cloudidentity.googleapis.com/groups.security and an empty value added to them. This is an immutable change and the security label cannot be removed once added.\n\nDynamic groups have a label with a key of cloudidentity.googleapis.com/groups.dynamic.\n\nIdentity-mapped groups for Cloud Search have a label with a key of system/groups/external and an empty value."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nResource name of the Group in the format: groups/{group_id}, where group_id\nis the unique ID assigned to the Group."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\nThe resource name of the entity under which this Group resides in the\nCloud Identity resource hierarchy.\n\nMust be of the form identitysources/{identity_source_id} for external-identity-mapped\ngroups or customers/{customer_id} for Google Groups."]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe time when the Group was last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `group_key` after provisioning.\n"]
    pub fn group_key(&self) -> ListRef<CloudIdentityGroupGroupKeyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.group_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudIdentityGroupTimeoutsElRef {
        CloudIdentityGroupTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct CloudIdentityGroupAdditionalGroupKeysEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    namespace: Option<PrimField<String>>,
}
impl CloudIdentityGroupAdditionalGroupKeysEl {
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
impl ToListMappable for CloudIdentityGroupAdditionalGroupKeysEl {
    type O = BlockAssignable<CloudIdentityGroupAdditionalGroupKeysEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudIdentityGroupAdditionalGroupKeysEl {}
impl BuildCloudIdentityGroupAdditionalGroupKeysEl {
    pub fn build(self) -> CloudIdentityGroupAdditionalGroupKeysEl {
        CloudIdentityGroupAdditionalGroupKeysEl {
            id: core::default::Default::default(),
            namespace: core::default::Default::default(),
        }
    }
}
pub struct CloudIdentityGroupAdditionalGroupKeysElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudIdentityGroupAdditionalGroupKeysElRef {
    fn new(shared: StackShared, base: String) -> CloudIdentityGroupAdditionalGroupKeysElRef {
        CloudIdentityGroupAdditionalGroupKeysElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudIdentityGroupAdditionalGroupKeysElRef {
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
pub struct CloudIdentityGroupGroupKeyEl {
    id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    namespace: Option<PrimField<String>>,
}
impl CloudIdentityGroupGroupKeyEl {
    #[doc = "Set the field `namespace`.\nThe namespace in which the entity exists.\n\nIf not specified, the EntityKey represents a Google-managed entity\nsuch as a Google user or a Google Group.\n\nIf specified, the EntityKey represents an external-identity-mapped group.\nThe namespace must correspond to an identity source created in Admin Console\nand must be in the form of 'identitysources/{identity_source_id}'."]
    pub fn set_namespace(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.namespace = Some(v.into());
        self
    }
}
impl ToListMappable for CloudIdentityGroupGroupKeyEl {
    type O = BlockAssignable<CloudIdentityGroupGroupKeyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudIdentityGroupGroupKeyEl {
    #[doc = "The ID of the entity.\n\nFor Google-managed entities, the id must be the email address of an existing\ngroup or user.\n\nFor external-identity-mapped entities, the id must be a string conforming\nto the Identity Source's requirements.\n\nMust be unique within a namespace."]
    pub id: PrimField<String>,
}
impl BuildCloudIdentityGroupGroupKeyEl {
    pub fn build(self) -> CloudIdentityGroupGroupKeyEl {
        CloudIdentityGroupGroupKeyEl {
            id: self.id,
            namespace: core::default::Default::default(),
        }
    }
}
pub struct CloudIdentityGroupGroupKeyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudIdentityGroupGroupKeyElRef {
    fn new(shared: StackShared, base: String) -> CloudIdentityGroupGroupKeyElRef {
        CloudIdentityGroupGroupKeyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudIdentityGroupGroupKeyElRef {
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
pub struct CloudIdentityGroupTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl CloudIdentityGroupTimeoutsEl {
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
impl ToListMappable for CloudIdentityGroupTimeoutsEl {
    type O = BlockAssignable<CloudIdentityGroupTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudIdentityGroupTimeoutsEl {}
impl BuildCloudIdentityGroupTimeoutsEl {
    pub fn build(self) -> CloudIdentityGroupTimeoutsEl {
        CloudIdentityGroupTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct CloudIdentityGroupTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudIdentityGroupTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> CloudIdentityGroupTimeoutsElRef {
        CloudIdentityGroupTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudIdentityGroupTimeoutsElRef {
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
struct CloudIdentityGroupDynamic {
    group_key: Option<DynamicBlock<CloudIdentityGroupGroupKeyEl>>,
}
