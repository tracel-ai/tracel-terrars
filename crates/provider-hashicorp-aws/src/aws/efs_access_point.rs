use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct EfsAccessPointData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    file_system_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    posix_user: Option<Vec<EfsAccessPointPosixUserEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    root_directory: Option<Vec<EfsAccessPointRootDirectoryEl>>,
    dynamic: EfsAccessPointDynamic,
}
struct EfsAccessPoint_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<EfsAccessPointData>,
}
#[derive(Clone)]
pub struct EfsAccessPoint(Rc<EfsAccessPoint_>);
impl EfsAccessPoint {
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
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
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
    #[doc = "Set the field `posix_user`.\n"]
    pub fn set_posix_user(self, v: impl Into<BlockAssignable<EfsAccessPointPosixUserEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().posix_user = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.posix_user = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `root_directory`.\n"]
    pub fn set_root_directory(
        self,
        v: impl Into<BlockAssignable<EfsAccessPointRootDirectoryEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().root_directory = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.root_directory = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `file_system_arn` after provisioning.\n"]
    pub fn file_system_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.file_system_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `file_system_id` after provisioning.\n"]
    pub fn file_system_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.file_system_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.owner_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `posix_user` after provisioning.\n"]
    pub fn posix_user(&self) -> ListRef<EfsAccessPointPosixUserElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.posix_user", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `root_directory` after provisioning.\n"]
    pub fn root_directory(&self) -> ListRef<EfsAccessPointRootDirectoryElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.root_directory", self.extract_ref()),
        )
    }
}
impl Referable for EfsAccessPoint {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for EfsAccessPoint {}
impl ToListMappable for EfsAccessPoint {
    type O = ListRef<EfsAccessPointRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for EfsAccessPoint_ {
    fn extract_resource_type(&self) -> String {
        "aws_efs_access_point".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildEfsAccessPoint {
    pub tf_id: String,
    #[doc = ""]
    pub file_system_id: PrimField<String>,
}
impl BuildEfsAccessPoint {
    pub fn build(self, stack: &mut Stack) -> EfsAccessPoint {
        let out = EfsAccessPoint(Rc::new(EfsAccessPoint_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(EfsAccessPointData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                file_system_id: self.file_system_id,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                posix_user: core::default::Default::default(),
                root_directory: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct EfsAccessPointRef {
    shared: StackShared,
    base: String,
}
impl Ref for EfsAccessPointRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl EfsAccessPointRef {
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
    #[doc = "Get a reference to the value of field `file_system_arn` after provisioning.\n"]
    pub fn file_system_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.file_system_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `file_system_id` after provisioning.\n"]
    pub fn file_system_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.file_system_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.owner_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `posix_user` after provisioning.\n"]
    pub fn posix_user(&self) -> ListRef<EfsAccessPointPosixUserElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.posix_user", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `root_directory` after provisioning.\n"]
    pub fn root_directory(&self) -> ListRef<EfsAccessPointRootDirectoryElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.root_directory", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct EfsAccessPointPosixUserEl {
    gid: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secondary_gids: Option<SetField<PrimField<f64>>>,
    uid: PrimField<f64>,
}
impl EfsAccessPointPosixUserEl {
    #[doc = "Set the field `secondary_gids`.\n"]
    pub fn set_secondary_gids(mut self, v: impl Into<SetField<PrimField<f64>>>) -> Self {
        self.secondary_gids = Some(v.into());
        self
    }
}
impl ToListMappable for EfsAccessPointPosixUserEl {
    type O = BlockAssignable<EfsAccessPointPosixUserEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEfsAccessPointPosixUserEl {
    #[doc = ""]
    pub gid: PrimField<f64>,
    #[doc = ""]
    pub uid: PrimField<f64>,
}
impl BuildEfsAccessPointPosixUserEl {
    pub fn build(self) -> EfsAccessPointPosixUserEl {
        EfsAccessPointPosixUserEl {
            gid: self.gid,
            secondary_gids: core::default::Default::default(),
            uid: self.uid,
        }
    }
}
pub struct EfsAccessPointPosixUserElRef {
    shared: StackShared,
    base: String,
}
impl Ref for EfsAccessPointPosixUserElRef {
    fn new(shared: StackShared, base: String) -> EfsAccessPointPosixUserElRef {
        EfsAccessPointPosixUserElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl EfsAccessPointPosixUserElRef {
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
pub struct EfsAccessPointRootDirectoryElCreationInfoEl {
    owner_gid: PrimField<f64>,
    owner_uid: PrimField<f64>,
    permissions: PrimField<String>,
}
impl EfsAccessPointRootDirectoryElCreationInfoEl {}
impl ToListMappable for EfsAccessPointRootDirectoryElCreationInfoEl {
    type O = BlockAssignable<EfsAccessPointRootDirectoryElCreationInfoEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEfsAccessPointRootDirectoryElCreationInfoEl {
    #[doc = ""]
    pub owner_gid: PrimField<f64>,
    #[doc = ""]
    pub owner_uid: PrimField<f64>,
    #[doc = ""]
    pub permissions: PrimField<String>,
}
impl BuildEfsAccessPointRootDirectoryElCreationInfoEl {
    pub fn build(self) -> EfsAccessPointRootDirectoryElCreationInfoEl {
        EfsAccessPointRootDirectoryElCreationInfoEl {
            owner_gid: self.owner_gid,
            owner_uid: self.owner_uid,
            permissions: self.permissions,
        }
    }
}
pub struct EfsAccessPointRootDirectoryElCreationInfoElRef {
    shared: StackShared,
    base: String,
}
impl Ref for EfsAccessPointRootDirectoryElCreationInfoElRef {
    fn new(shared: StackShared, base: String) -> EfsAccessPointRootDirectoryElCreationInfoElRef {
        EfsAccessPointRootDirectoryElCreationInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl EfsAccessPointRootDirectoryElCreationInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `owner_gid` after provisioning.\n"]
    pub fn owner_gid(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.owner_gid", self.base))
    }
    #[doc = "Get a reference to the value of field `owner_uid` after provisioning.\n"]
    pub fn owner_uid(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.owner_uid", self.base))
    }
    #[doc = "Get a reference to the value of field `permissions` after provisioning.\n"]
    pub fn permissions(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.permissions", self.base))
    }
}
#[derive(Serialize, Default)]
struct EfsAccessPointRootDirectoryElDynamic {
    creation_info: Option<DynamicBlock<EfsAccessPointRootDirectoryElCreationInfoEl>>,
}
#[derive(Serialize)]
pub struct EfsAccessPointRootDirectoryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    creation_info: Option<Vec<EfsAccessPointRootDirectoryElCreationInfoEl>>,
    dynamic: EfsAccessPointRootDirectoryElDynamic,
}
impl EfsAccessPointRootDirectoryEl {
    #[doc = "Set the field `path`.\n"]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }
    #[doc = "Set the field `creation_info`.\n"]
    pub fn set_creation_info(
        mut self,
        v: impl Into<BlockAssignable<EfsAccessPointRootDirectoryElCreationInfoEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.creation_info = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.creation_info = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for EfsAccessPointRootDirectoryEl {
    type O = BlockAssignable<EfsAccessPointRootDirectoryEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEfsAccessPointRootDirectoryEl {}
impl BuildEfsAccessPointRootDirectoryEl {
    pub fn build(self) -> EfsAccessPointRootDirectoryEl {
        EfsAccessPointRootDirectoryEl {
            path: core::default::Default::default(),
            creation_info: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct EfsAccessPointRootDirectoryElRef {
    shared: StackShared,
    base: String,
}
impl Ref for EfsAccessPointRootDirectoryElRef {
    fn new(shared: StackShared, base: String) -> EfsAccessPointRootDirectoryElRef {
        EfsAccessPointRootDirectoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl EfsAccessPointRootDirectoryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
    #[doc = "Get a reference to the value of field `creation_info` after provisioning.\n"]
    pub fn creation_info(&self) -> ListRef<EfsAccessPointRootDirectoryElCreationInfoElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.creation_info", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct EfsAccessPointDynamic {
    posix_user: Option<DynamicBlock<EfsAccessPointPosixUserEl>>,
    root_directory: Option<DynamicBlock<EfsAccessPointRootDirectoryEl>>,
}
