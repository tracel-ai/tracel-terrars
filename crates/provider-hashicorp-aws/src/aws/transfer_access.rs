use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct TransferAccessData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    external_id: PrimField<String>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    role: Option<PrimField<String>>,
    server_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    home_directory_mappings: Option<Vec<TransferAccessHomeDirectoryMappingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    posix_profile: Option<Vec<TransferAccessPosixProfileEl>>,
    dynamic: TransferAccessDynamic,
}
struct TransferAccess_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<TransferAccessData>,
}
#[derive(Clone)]
pub struct TransferAccess(Rc<TransferAccess_>);
impl TransferAccess {
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
    #[doc = "Set the field `role`.\n"]
    pub fn set_role(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().role = Some(v.into());
        self
    }
    #[doc = "Set the field `home_directory_mappings`.\n"]
    pub fn set_home_directory_mappings(
        self,
        v: impl Into<BlockAssignable<TransferAccessHomeDirectoryMappingsEl>>,
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
        v: impl Into<BlockAssignable<TransferAccessPosixProfileEl>>,
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
    #[doc = "Get a reference to the value of field `external_id` after provisioning.\n"]
    pub fn external_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.external_id", self.extract_ref()),
        )
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
    #[doc = "Get a reference to the value of field `home_directory_mappings` after provisioning.\n"]
    pub fn home_directory_mappings(&self) -> ListRef<TransferAccessHomeDirectoryMappingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.home_directory_mappings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `posix_profile` after provisioning.\n"]
    pub fn posix_profile(&self) -> ListRef<TransferAccessPosixProfileElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.posix_profile", self.extract_ref()),
        )
    }
}
impl Referable for TransferAccess {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for TransferAccess {}
impl ToListMappable for TransferAccess {
    type O = ListRef<TransferAccessRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for TransferAccess_ {
    fn extract_resource_type(&self) -> String {
        "aws_transfer_access".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildTransferAccess {
    pub tf_id: String,
    #[doc = ""]
    pub external_id: PrimField<String>,
    #[doc = ""]
    pub server_id: PrimField<String>,
}
impl BuildTransferAccess {
    pub fn build(self, stack: &mut Stack) -> TransferAccess {
        let out = TransferAccess(Rc::new(TransferAccess_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(TransferAccessData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                external_id: self.external_id,
                home_directory: core::default::Default::default(),
                home_directory_type: core::default::Default::default(),
                id: core::default::Default::default(),
                policy: core::default::Default::default(),
                region: core::default::Default::default(),
                role: core::default::Default::default(),
                server_id: self.server_id,
                home_directory_mappings: core::default::Default::default(),
                posix_profile: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct TransferAccessRef {
    shared: StackShared,
    base: String,
}
impl Ref for TransferAccessRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl TransferAccessRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `external_id` after provisioning.\n"]
    pub fn external_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.external_id", self.extract_ref()),
        )
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
    #[doc = "Get a reference to the value of field `home_directory_mappings` after provisioning.\n"]
    pub fn home_directory_mappings(&self) -> ListRef<TransferAccessHomeDirectoryMappingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.home_directory_mappings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `posix_profile` after provisioning.\n"]
    pub fn posix_profile(&self) -> ListRef<TransferAccessPosixProfileElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.posix_profile", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct TransferAccessHomeDirectoryMappingsEl {
    entry: PrimField<String>,
    target: PrimField<String>,
}
impl TransferAccessHomeDirectoryMappingsEl {}
impl ToListMappable for TransferAccessHomeDirectoryMappingsEl {
    type O = BlockAssignable<TransferAccessHomeDirectoryMappingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildTransferAccessHomeDirectoryMappingsEl {
    #[doc = ""]
    pub entry: PrimField<String>,
    #[doc = ""]
    pub target: PrimField<String>,
}
impl BuildTransferAccessHomeDirectoryMappingsEl {
    pub fn build(self) -> TransferAccessHomeDirectoryMappingsEl {
        TransferAccessHomeDirectoryMappingsEl {
            entry: self.entry,
            target: self.target,
        }
    }
}
pub struct TransferAccessHomeDirectoryMappingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for TransferAccessHomeDirectoryMappingsElRef {
    fn new(shared: StackShared, base: String) -> TransferAccessHomeDirectoryMappingsElRef {
        TransferAccessHomeDirectoryMappingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl TransferAccessHomeDirectoryMappingsElRef {
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
pub struct TransferAccessPosixProfileEl {
    gid: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secondary_gids: Option<SetField<PrimField<f64>>>,
    uid: PrimField<f64>,
}
impl TransferAccessPosixProfileEl {
    #[doc = "Set the field `secondary_gids`.\n"]
    pub fn set_secondary_gids(mut self, v: impl Into<SetField<PrimField<f64>>>) -> Self {
        self.secondary_gids = Some(v.into());
        self
    }
}
impl ToListMappable for TransferAccessPosixProfileEl {
    type O = BlockAssignable<TransferAccessPosixProfileEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildTransferAccessPosixProfileEl {
    #[doc = ""]
    pub gid: PrimField<f64>,
    #[doc = ""]
    pub uid: PrimField<f64>,
}
impl BuildTransferAccessPosixProfileEl {
    pub fn build(self) -> TransferAccessPosixProfileEl {
        TransferAccessPosixProfileEl {
            gid: self.gid,
            secondary_gids: core::default::Default::default(),
            uid: self.uid,
        }
    }
}
pub struct TransferAccessPosixProfileElRef {
    shared: StackShared,
    base: String,
}
impl Ref for TransferAccessPosixProfileElRef {
    fn new(shared: StackShared, base: String) -> TransferAccessPosixProfileElRef {
        TransferAccessPosixProfileElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl TransferAccessPosixProfileElRef {
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
#[derive(Serialize, Default)]
struct TransferAccessDynamic {
    home_directory_mappings: Option<DynamicBlock<TransferAccessHomeDirectoryMappingsEl>>,
    posix_profile: Option<DynamicBlock<TransferAccessPosixProfileEl>>,
}
