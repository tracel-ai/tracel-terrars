use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct DatasyncLocationFsxOpenzfsFileSystemData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    fsx_filesystem_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    security_group_arns: SetField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subdirectory: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protocol: Option<Vec<DatasyncLocationFsxOpenzfsFileSystemProtocolEl>>,
    dynamic: DatasyncLocationFsxOpenzfsFileSystemDynamic,
}

struct DatasyncLocationFsxOpenzfsFileSystem_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DatasyncLocationFsxOpenzfsFileSystemData>,
}

#[derive(Clone)]
pub struct DatasyncLocationFsxOpenzfsFileSystem(Rc<DatasyncLocationFsxOpenzfsFileSystem_>);

impl DatasyncLocationFsxOpenzfsFileSystem {
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

    #[doc = "Set the field `subdirectory`.\n"]
    pub fn set_subdirectory(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().subdirectory = Some(v.into());
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

    #[doc = "Set the field `protocol`.\n"]
    pub fn set_protocol(
        self,
        v: impl Into<BlockAssignable<DatasyncLocationFsxOpenzfsFileSystemProtocolEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().protocol = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.protocol = Some(d);
            }
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `creation_time` after provisioning.\n"]
    pub fn creation_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creation_time", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `fsx_filesystem_arn` after provisioning.\n"]
    pub fn fsx_filesystem_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.fsx_filesystem_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `security_group_arns` after provisioning.\n"]
    pub fn security_group_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.security_group_arns", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `subdirectory` after provisioning.\n"]
    pub fn subdirectory(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subdirectory", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> ListRef<DatasyncLocationFsxOpenzfsFileSystemProtocolElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.protocol", self.extract_ref()),
        )
    }
}

impl Referable for DatasyncLocationFsxOpenzfsFileSystem {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for DatasyncLocationFsxOpenzfsFileSystem {}

impl ToListMappable for DatasyncLocationFsxOpenzfsFileSystem {
    type O = ListRef<DatasyncLocationFsxOpenzfsFileSystemRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for DatasyncLocationFsxOpenzfsFileSystem_ {
    fn extract_resource_type(&self) -> String {
        "aws_datasync_location_fsx_openzfs_file_system".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDatasyncLocationFsxOpenzfsFileSystem {
    pub tf_id: String,
    #[doc = ""]
    pub fsx_filesystem_arn: PrimField<String>,
    #[doc = ""]
    pub security_group_arns: SetField<PrimField<String>>,
}

impl BuildDatasyncLocationFsxOpenzfsFileSystem {
    pub fn build(self, stack: &mut Stack) -> DatasyncLocationFsxOpenzfsFileSystem {
        let out =
            DatasyncLocationFsxOpenzfsFileSystem(Rc::new(DatasyncLocationFsxOpenzfsFileSystem_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(DatasyncLocationFsxOpenzfsFileSystemData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    lifecycle: core::default::Default::default(),
                    for_each: None,
                    fsx_filesystem_arn: self.fsx_filesystem_arn,
                    id: core::default::Default::default(),
                    region: core::default::Default::default(),
                    security_group_arns: self.security_group_arns,
                    subdirectory: core::default::Default::default(),
                    tags: core::default::Default::default(),
                    tags_all: core::default::Default::default(),
                    protocol: core::default::Default::default(),
                    dynamic: Default::default(),
                }),
            }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DatasyncLocationFsxOpenzfsFileSystemRef {
    shared: StackShared,
    base: String,
}

impl Ref for DatasyncLocationFsxOpenzfsFileSystemRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl DatasyncLocationFsxOpenzfsFileSystemRef {
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

    #[doc = "Get a reference to the value of field `creation_time` after provisioning.\n"]
    pub fn creation_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creation_time", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `fsx_filesystem_arn` after provisioning.\n"]
    pub fn fsx_filesystem_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.fsx_filesystem_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `security_group_arns` after provisioning.\n"]
    pub fn security_group_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.security_group_arns", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `subdirectory` after provisioning.\n"]
    pub fn subdirectory(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subdirectory", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> ListRef<DatasyncLocationFsxOpenzfsFileSystemProtocolElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.protocol", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DatasyncLocationFsxOpenzfsFileSystemProtocolElNfsElMountOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
}

impl DatasyncLocationFsxOpenzfsFileSystemProtocolElNfsElMountOptionsEl {
    #[doc = "Set the field `version`.\n"]
    pub fn set_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.version = Some(v.into());
        self
    }
}

impl ToListMappable for DatasyncLocationFsxOpenzfsFileSystemProtocolElNfsElMountOptionsEl {
    type O = BlockAssignable<DatasyncLocationFsxOpenzfsFileSystemProtocolElNfsElMountOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatasyncLocationFsxOpenzfsFileSystemProtocolElNfsElMountOptionsEl {}

impl BuildDatasyncLocationFsxOpenzfsFileSystemProtocolElNfsElMountOptionsEl {
    pub fn build(self) -> DatasyncLocationFsxOpenzfsFileSystemProtocolElNfsElMountOptionsEl {
        DatasyncLocationFsxOpenzfsFileSystemProtocolElNfsElMountOptionsEl {
            version: core::default::Default::default(),
        }
    }
}

pub struct DatasyncLocationFsxOpenzfsFileSystemProtocolElNfsElMountOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DatasyncLocationFsxOpenzfsFileSystemProtocolElNfsElMountOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatasyncLocationFsxOpenzfsFileSystemProtocolElNfsElMountOptionsElRef {
        DatasyncLocationFsxOpenzfsFileSystemProtocolElNfsElMountOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatasyncLocationFsxOpenzfsFileSystemProtocolElNfsElMountOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatasyncLocationFsxOpenzfsFileSystemProtocolElNfsElDynamic {
    mount_options:
        Option<DynamicBlock<DatasyncLocationFsxOpenzfsFileSystemProtocolElNfsElMountOptionsEl>>,
}

#[derive(Serialize)]
pub struct DatasyncLocationFsxOpenzfsFileSystemProtocolElNfsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    mount_options: Option<Vec<DatasyncLocationFsxOpenzfsFileSystemProtocolElNfsElMountOptionsEl>>,
    dynamic: DatasyncLocationFsxOpenzfsFileSystemProtocolElNfsElDynamic,
}

impl DatasyncLocationFsxOpenzfsFileSystemProtocolElNfsEl {
    #[doc = "Set the field `mount_options`.\n"]
    pub fn set_mount_options(
        mut self,
        v: impl Into<BlockAssignable<DatasyncLocationFsxOpenzfsFileSystemProtocolElNfsElMountOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.mount_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.mount_options = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for DatasyncLocationFsxOpenzfsFileSystemProtocolElNfsEl {
    type O = BlockAssignable<DatasyncLocationFsxOpenzfsFileSystemProtocolElNfsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatasyncLocationFsxOpenzfsFileSystemProtocolElNfsEl {}

impl BuildDatasyncLocationFsxOpenzfsFileSystemProtocolElNfsEl {
    pub fn build(self) -> DatasyncLocationFsxOpenzfsFileSystemProtocolElNfsEl {
        DatasyncLocationFsxOpenzfsFileSystemProtocolElNfsEl {
            mount_options: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct DatasyncLocationFsxOpenzfsFileSystemProtocolElNfsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DatasyncLocationFsxOpenzfsFileSystemProtocolElNfsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatasyncLocationFsxOpenzfsFileSystemProtocolElNfsElRef {
        DatasyncLocationFsxOpenzfsFileSystemProtocolElNfsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatasyncLocationFsxOpenzfsFileSystemProtocolElNfsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `mount_options` after provisioning.\n"]
    pub fn mount_options(
        &self,
    ) -> ListRef<DatasyncLocationFsxOpenzfsFileSystemProtocolElNfsElMountOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.mount_options", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct DatasyncLocationFsxOpenzfsFileSystemProtocolElDynamic {
    nfs: Option<DynamicBlock<DatasyncLocationFsxOpenzfsFileSystemProtocolElNfsEl>>,
}

#[derive(Serialize)]
pub struct DatasyncLocationFsxOpenzfsFileSystemProtocolEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    nfs: Option<Vec<DatasyncLocationFsxOpenzfsFileSystemProtocolElNfsEl>>,
    dynamic: DatasyncLocationFsxOpenzfsFileSystemProtocolElDynamic,
}

impl DatasyncLocationFsxOpenzfsFileSystemProtocolEl {
    #[doc = "Set the field `nfs`.\n"]
    pub fn set_nfs(
        mut self,
        v: impl Into<BlockAssignable<DatasyncLocationFsxOpenzfsFileSystemProtocolElNfsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.nfs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.nfs = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for DatasyncLocationFsxOpenzfsFileSystemProtocolEl {
    type O = BlockAssignable<DatasyncLocationFsxOpenzfsFileSystemProtocolEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatasyncLocationFsxOpenzfsFileSystemProtocolEl {}

impl BuildDatasyncLocationFsxOpenzfsFileSystemProtocolEl {
    pub fn build(self) -> DatasyncLocationFsxOpenzfsFileSystemProtocolEl {
        DatasyncLocationFsxOpenzfsFileSystemProtocolEl {
            nfs: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct DatasyncLocationFsxOpenzfsFileSystemProtocolElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DatasyncLocationFsxOpenzfsFileSystemProtocolElRef {
    fn new(shared: StackShared, base: String) -> DatasyncLocationFsxOpenzfsFileSystemProtocolElRef {
        DatasyncLocationFsxOpenzfsFileSystemProtocolElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatasyncLocationFsxOpenzfsFileSystemProtocolElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `nfs` after provisioning.\n"]
    pub fn nfs(&self) -> ListRef<DatasyncLocationFsxOpenzfsFileSystemProtocolElNfsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.nfs", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatasyncLocationFsxOpenzfsFileSystemDynamic {
    protocol: Option<DynamicBlock<DatasyncLocationFsxOpenzfsFileSystemProtocolEl>>,
}
