use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct TransferUserData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    home_directory: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    home_directory_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    role: PrimField<String>,
    server_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    user_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    home_directory_mappings: Option<Vec<TransferUserHomeDirectoryMappingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    posix_profile: Option<Vec<TransferUserPosixProfileEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<TransferUserTimeoutsEl>,
    dynamic: TransferUserDynamic,
}
struct TransferUser_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<TransferUserData>,
}
#[derive(Clone)]
pub struct TransferUser(Rc<TransferUser_>);
impl TransferUser {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(self, provider: &ProviderAws) -> Self {
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
    #[doc = "Set the field `home_directory`.\n"]
    pub fn set_home_directory(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().home_directory = Some(v.into());
        self
    }
    #[doc = "Set the field `home_directory_type`.\n"]
    pub fn set_home_directory_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().home_directory_type = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `policy`.\n"]
    pub fn set_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().policy = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }
    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags_all = Some(v.into());
        self
    }
    #[doc = "Set the field `home_directory_mappings`.\n"]
    pub fn set_home_directory_mappings(
        self,
        v: impl Into<BlockAssignable<TransferUserHomeDirectoryMappingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().home_directory_mappings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.home_directory_mappings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `posix_profile`.\n"]
    pub fn set_posix_profile(
        self,
        v: impl Into<BlockAssignable<TransferUserPosixProfileEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().posix_profile = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.posix_profile = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<TransferUserTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `home_directory` after provisioning.\n"]
    pub fn home_directory(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.home_directory", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `home_directory_type` after provisioning.\n"]
    pub fn home_directory_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.home_directory_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `policy` after provisioning.\n"]
    pub fn policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `role` after provisioning.\n"]
    pub fn role(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.role", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `server_id` after provisioning.\n"]
    pub fn server_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.server_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `user_name` after provisioning.\n"]
    pub fn user_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `home_directory_mappings` after provisioning.\n"]
    pub fn home_directory_mappings(&self) -> ListRef<TransferUserHomeDirectoryMappingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.home_directory_mappings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `posix_profile` after provisioning.\n"]
    pub fn posix_profile(&self) -> ListRef<TransferUserPosixProfileElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.posix_profile", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> TransferUserTimeoutsElRef {
        TransferUserTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for TransferUser {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for TransferUser {}
impl ToListMappable for TransferUser {
    type O = ListRef<TransferUserRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for TransferUser_ {
    fn extract_resource_type(&self) -> String {
        "aws_transfer_user".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildTransferUser {
    pub tf_id: String,
    #[doc = ""]
    pub role: PrimField<String>,
    #[doc = ""]
    pub server_id: PrimField<String>,
    #[doc = ""]
    pub user_name: PrimField<String>,
}
impl BuildTransferUser {
    pub fn build(self, stack: &mut Stack) -> TransferUser {
        let out = TransferUser(Rc::new(TransferUser_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(TransferUserData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                home_directory: core::default::Default::default(),
                home_directory_type: core::default::Default::default(),
                id: core::default::Default::default(),
                policy: core::default::Default::default(),
                region: core::default::Default::default(),
                role: self.role,
                server_id: self.server_id,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                user_name: self.user_name,
                home_directory_mappings: core::default::Default::default(),
                posix_profile: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct TransferUserRef {
    shared: StackShared,
    base: String,
}
impl Ref for TransferUserRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl TransferUserRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `home_directory` after provisioning.\n"]
    pub fn home_directory(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.home_directory", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `home_directory_type` after provisioning.\n"]
    pub fn home_directory_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.home_directory_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `policy` after provisioning.\n"]
    pub fn policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `role` after provisioning.\n"]
    pub fn role(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.role", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `server_id` after provisioning.\n"]
    pub fn server_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.server_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `user_name` after provisioning.\n"]
    pub fn user_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `home_directory_mappings` after provisioning.\n"]
    pub fn home_directory_mappings(&self) -> ListRef<TransferUserHomeDirectoryMappingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.home_directory_mappings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `posix_profile` after provisioning.\n"]
    pub fn posix_profile(&self) -> ListRef<TransferUserPosixProfileElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.posix_profile", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> TransferUserTimeoutsElRef {
        TransferUserTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct TransferUserHomeDirectoryMappingsEl {
    entry: PrimField<String>,
    target: PrimField<String>,
}
impl TransferUserHomeDirectoryMappingsEl {}
impl ToListMappable for TransferUserHomeDirectoryMappingsEl {
    type O = BlockAssignable<TransferUserHomeDirectoryMappingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildTransferUserHomeDirectoryMappingsEl {
    #[doc = ""]
    pub entry: PrimField<String>,
    #[doc = ""]
    pub target: PrimField<String>,
}
impl BuildTransferUserHomeDirectoryMappingsEl {
    pub fn build(self) -> TransferUserHomeDirectoryMappingsEl {
        TransferUserHomeDirectoryMappingsEl {
            entry: self.entry,
            target: self.target,
        }
    }
}
pub struct TransferUserHomeDirectoryMappingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for TransferUserHomeDirectoryMappingsElRef {
    fn new(shared: StackShared, base: String) -> TransferUserHomeDirectoryMappingsElRef {
        TransferUserHomeDirectoryMappingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl TransferUserHomeDirectoryMappingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `entry` after provisioning.\n"]
    pub fn entry(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.entry", self.base))
    }
    #[doc = "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target", self.base))
    }
}
#[derive(Serialize)]
pub struct TransferUserPosixProfileEl {
    gid: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secondary_gids: Option<SetField<PrimField<f64>>>,
    uid: PrimField<f64>,
}
impl TransferUserPosixProfileEl {
    #[doc = "Set the field `secondary_gids`.\n"]
    pub fn set_secondary_gids(mut self, v: impl Into<SetField<PrimField<f64>>>) -> Self {
        self.secondary_gids = Some(v.into());
        self
    }
}
impl ToListMappable for TransferUserPosixProfileEl {
    type O = BlockAssignable<TransferUserPosixProfileEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildTransferUserPosixProfileEl {
    #[doc = ""]
    pub gid: PrimField<f64>,
    #[doc = ""]
    pub uid: PrimField<f64>,
}
impl BuildTransferUserPosixProfileEl {
    pub fn build(self) -> TransferUserPosixProfileEl {
        TransferUserPosixProfileEl {
            gid: self.gid,
            secondary_gids: core::default::Default::default(),
            uid: self.uid,
        }
    }
}
pub struct TransferUserPosixProfileElRef {
    shared: StackShared,
    base: String,
}
impl Ref for TransferUserPosixProfileElRef {
    fn new(shared: StackShared, base: String) -> TransferUserPosixProfileElRef {
        TransferUserPosixProfileElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl TransferUserPosixProfileElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `gid` after provisioning.\n"]
    pub fn gid(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.gid", self.base))
    }
    #[doc = "Get a reference to the value of field `secondary_gids` after provisioning.\n"]
    pub fn secondary_gids(&self) -> SetRef<PrimExpr<f64>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.secondary_gids", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\n"]
    pub fn uid(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.base))
    }
}
#[derive(Serialize)]
pub struct TransferUserTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}
impl TransferUserTimeoutsEl {
    #[doc = "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }
}
impl ToListMappable for TransferUserTimeoutsEl {
    type O = BlockAssignable<TransferUserTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildTransferUserTimeoutsEl {}
impl BuildTransferUserTimeoutsEl {
    pub fn build(self) -> TransferUserTimeoutsEl {
        TransferUserTimeoutsEl {
            delete: core::default::Default::default(),
        }
    }
}
pub struct TransferUserTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for TransferUserTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> TransferUserTimeoutsElRef {
        TransferUserTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl TransferUserTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }
}
#[derive(Serialize, Default)]
struct TransferUserDynamic {
    home_directory_mappings: Option<DynamicBlock<TransferUserHomeDirectoryMappingsEl>>,
    posix_profile: Option<DynamicBlock<TransferUserPosixProfileEl>>,
}
