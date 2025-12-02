use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DatasyncLocationFsxOntapFileSystemData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    security_group_arns: SetField<PrimField<String>>,
    storage_virtual_machine_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subdirectory: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protocol: Option<Vec<DatasyncLocationFsxOntapFileSystemProtocolEl>>,
    dynamic: DatasyncLocationFsxOntapFileSystemDynamic,
}
struct DatasyncLocationFsxOntapFileSystem_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DatasyncLocationFsxOntapFileSystemData>,
}
#[derive(Clone)]
pub struct DatasyncLocationFsxOntapFileSystem(Rc<DatasyncLocationFsxOntapFileSystem_>);
impl DatasyncLocationFsxOntapFileSystem {
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
        v: impl Into<BlockAssignable<DatasyncLocationFsxOntapFileSystemProtocolEl>>,
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
    #[doc = "Get a reference to the value of field `storage_virtual_machine_arn` after provisioning.\n"]
    pub fn storage_virtual_machine_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.storage_virtual_machine_arn", self.extract_ref()),
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
    pub fn protocol(&self) -> ListRef<DatasyncLocationFsxOntapFileSystemProtocolElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.protocol", self.extract_ref()),
        )
    }
}
impl Referable for DatasyncLocationFsxOntapFileSystem {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for DatasyncLocationFsxOntapFileSystem {}
impl ToListMappable for DatasyncLocationFsxOntapFileSystem {
    type O = ListRef<DatasyncLocationFsxOntapFileSystemRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for DatasyncLocationFsxOntapFileSystem_ {
    fn extract_resource_type(&self) -> String {
        "aws_datasync_location_fsx_ontap_file_system".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDatasyncLocationFsxOntapFileSystem {
    pub tf_id: String,
    #[doc = ""]
    pub security_group_arns: SetField<PrimField<String>>,
    #[doc = ""]
    pub storage_virtual_machine_arn: PrimField<String>,
}
impl BuildDatasyncLocationFsxOntapFileSystem {
    pub fn build(self, stack: &mut Stack) -> DatasyncLocationFsxOntapFileSystem {
        let out =
            DatasyncLocationFsxOntapFileSystem(Rc::new(DatasyncLocationFsxOntapFileSystem_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(DatasyncLocationFsxOntapFileSystemData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    lifecycle: core::default::Default::default(),
                    for_each: None,
                    id: core::default::Default::default(),
                    region: core::default::Default::default(),
                    security_group_arns: self.security_group_arns,
                    storage_virtual_machine_arn: self.storage_virtual_machine_arn,
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
pub struct DatasyncLocationFsxOntapFileSystemRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatasyncLocationFsxOntapFileSystemRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DatasyncLocationFsxOntapFileSystemRef {
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
    #[doc = "Get a reference to the value of field `storage_virtual_machine_arn` after provisioning.\n"]
    pub fn storage_virtual_machine_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.storage_virtual_machine_arn", self.extract_ref()),
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
    pub fn protocol(&self) -> ListRef<DatasyncLocationFsxOntapFileSystemProtocolElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.protocol", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DatasyncLocationFsxOntapFileSystemProtocolElNfsElMountOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
}
impl DatasyncLocationFsxOntapFileSystemProtocolElNfsElMountOptionsEl {
    #[doc = "Set the field `version`.\n"]
    pub fn set_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.version = Some(v.into());
        self
    }
}
impl ToListMappable for DatasyncLocationFsxOntapFileSystemProtocolElNfsElMountOptionsEl {
    type O = BlockAssignable<DatasyncLocationFsxOntapFileSystemProtocolElNfsElMountOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatasyncLocationFsxOntapFileSystemProtocolElNfsElMountOptionsEl {}
impl BuildDatasyncLocationFsxOntapFileSystemProtocolElNfsElMountOptionsEl {
    pub fn build(self) -> DatasyncLocationFsxOntapFileSystemProtocolElNfsElMountOptionsEl {
        DatasyncLocationFsxOntapFileSystemProtocolElNfsElMountOptionsEl {
            version: core::default::Default::default(),
        }
    }
}
pub struct DatasyncLocationFsxOntapFileSystemProtocolElNfsElMountOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatasyncLocationFsxOntapFileSystemProtocolElNfsElMountOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatasyncLocationFsxOntapFileSystemProtocolElNfsElMountOptionsElRef {
        DatasyncLocationFsxOntapFileSystemProtocolElNfsElMountOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatasyncLocationFsxOntapFileSystemProtocolElNfsElMountOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}
#[derive(Serialize, Default)]
struct DatasyncLocationFsxOntapFileSystemProtocolElNfsElDynamic {
    mount_options:
        Option<DynamicBlock<DatasyncLocationFsxOntapFileSystemProtocolElNfsElMountOptionsEl>>,
}
#[derive(Serialize)]
pub struct DatasyncLocationFsxOntapFileSystemProtocolElNfsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    mount_options: Option<Vec<DatasyncLocationFsxOntapFileSystemProtocolElNfsElMountOptionsEl>>,
    dynamic: DatasyncLocationFsxOntapFileSystemProtocolElNfsElDynamic,
}
impl DatasyncLocationFsxOntapFileSystemProtocolElNfsEl {
    #[doc = "Set the field `mount_options`.\n"]
    pub fn set_mount_options(
        mut self,
        v: impl Into<BlockAssignable<DatasyncLocationFsxOntapFileSystemProtocolElNfsElMountOptionsEl>>,
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
impl ToListMappable for DatasyncLocationFsxOntapFileSystemProtocolElNfsEl {
    type O = BlockAssignable<DatasyncLocationFsxOntapFileSystemProtocolElNfsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatasyncLocationFsxOntapFileSystemProtocolElNfsEl {}
impl BuildDatasyncLocationFsxOntapFileSystemProtocolElNfsEl {
    pub fn build(self) -> DatasyncLocationFsxOntapFileSystemProtocolElNfsEl {
        DatasyncLocationFsxOntapFileSystemProtocolElNfsEl {
            mount_options: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DatasyncLocationFsxOntapFileSystemProtocolElNfsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatasyncLocationFsxOntapFileSystemProtocolElNfsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatasyncLocationFsxOntapFileSystemProtocolElNfsElRef {
        DatasyncLocationFsxOntapFileSystemProtocolElNfsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatasyncLocationFsxOntapFileSystemProtocolElNfsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `mount_options` after provisioning.\n"]
    pub fn mount_options(
        &self,
    ) -> ListRef<DatasyncLocationFsxOntapFileSystemProtocolElNfsElMountOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.mount_options", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DatasyncLocationFsxOntapFileSystemProtocolElSmbElMountOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
}
impl DatasyncLocationFsxOntapFileSystemProtocolElSmbElMountOptionsEl {
    #[doc = "Set the field `version`.\n"]
    pub fn set_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.version = Some(v.into());
        self
    }
}
impl ToListMappable for DatasyncLocationFsxOntapFileSystemProtocolElSmbElMountOptionsEl {
    type O = BlockAssignable<DatasyncLocationFsxOntapFileSystemProtocolElSmbElMountOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatasyncLocationFsxOntapFileSystemProtocolElSmbElMountOptionsEl {}
impl BuildDatasyncLocationFsxOntapFileSystemProtocolElSmbElMountOptionsEl {
    pub fn build(self) -> DatasyncLocationFsxOntapFileSystemProtocolElSmbElMountOptionsEl {
        DatasyncLocationFsxOntapFileSystemProtocolElSmbElMountOptionsEl {
            version: core::default::Default::default(),
        }
    }
}
pub struct DatasyncLocationFsxOntapFileSystemProtocolElSmbElMountOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatasyncLocationFsxOntapFileSystemProtocolElSmbElMountOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatasyncLocationFsxOntapFileSystemProtocolElSmbElMountOptionsElRef {
        DatasyncLocationFsxOntapFileSystemProtocolElSmbElMountOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatasyncLocationFsxOntapFileSystemProtocolElSmbElMountOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}
#[derive(Serialize, Default)]
struct DatasyncLocationFsxOntapFileSystemProtocolElSmbElDynamic {
    mount_options:
        Option<DynamicBlock<DatasyncLocationFsxOntapFileSystemProtocolElSmbElMountOptionsEl>>,
}
#[derive(Serialize)]
pub struct DatasyncLocationFsxOntapFileSystemProtocolElSmbEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    domain: Option<PrimField<String>>,
    password: PrimField<String>,
    user: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mount_options: Option<Vec<DatasyncLocationFsxOntapFileSystemProtocolElSmbElMountOptionsEl>>,
    dynamic: DatasyncLocationFsxOntapFileSystemProtocolElSmbElDynamic,
}
impl DatasyncLocationFsxOntapFileSystemProtocolElSmbEl {
    #[doc = "Set the field `domain`.\n"]
    pub fn set_domain(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.domain = Some(v.into());
        self
    }
    #[doc = "Set the field `mount_options`.\n"]
    pub fn set_mount_options(
        mut self,
        v: impl Into<BlockAssignable<DatasyncLocationFsxOntapFileSystemProtocolElSmbElMountOptionsEl>>,
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
impl ToListMappable for DatasyncLocationFsxOntapFileSystemProtocolElSmbEl {
    type O = BlockAssignable<DatasyncLocationFsxOntapFileSystemProtocolElSmbEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatasyncLocationFsxOntapFileSystemProtocolElSmbEl {
    #[doc = ""]
    pub password: PrimField<String>,
    #[doc = ""]
    pub user: PrimField<String>,
}
impl BuildDatasyncLocationFsxOntapFileSystemProtocolElSmbEl {
    pub fn build(self) -> DatasyncLocationFsxOntapFileSystemProtocolElSmbEl {
        DatasyncLocationFsxOntapFileSystemProtocolElSmbEl {
            domain: core::default::Default::default(),
            password: self.password,
            user: self.user,
            mount_options: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DatasyncLocationFsxOntapFileSystemProtocolElSmbElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatasyncLocationFsxOntapFileSystemProtocolElSmbElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatasyncLocationFsxOntapFileSystemProtocolElSmbElRef {
        DatasyncLocationFsxOntapFileSystemProtocolElSmbElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatasyncLocationFsxOntapFileSystemProtocolElSmbElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `domain` after provisioning.\n"]
    pub fn domain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain", self.base))
    }
    #[doc = "Get a reference to the value of field `password` after provisioning.\n"]
    pub fn password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }
    #[doc = "Get a reference to the value of field `user` after provisioning.\n"]
    pub fn user(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user", self.base))
    }
    #[doc = "Get a reference to the value of field `mount_options` after provisioning.\n"]
    pub fn mount_options(
        &self,
    ) -> ListRef<DatasyncLocationFsxOntapFileSystemProtocolElSmbElMountOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.mount_options", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DatasyncLocationFsxOntapFileSystemProtocolElDynamic {
    nfs: Option<DynamicBlock<DatasyncLocationFsxOntapFileSystemProtocolElNfsEl>>,
    smb: Option<DynamicBlock<DatasyncLocationFsxOntapFileSystemProtocolElSmbEl>>,
}
#[derive(Serialize)]
pub struct DatasyncLocationFsxOntapFileSystemProtocolEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    nfs: Option<Vec<DatasyncLocationFsxOntapFileSystemProtocolElNfsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    smb: Option<Vec<DatasyncLocationFsxOntapFileSystemProtocolElSmbEl>>,
    dynamic: DatasyncLocationFsxOntapFileSystemProtocolElDynamic,
}
impl DatasyncLocationFsxOntapFileSystemProtocolEl {
    #[doc = "Set the field `nfs`.\n"]
    pub fn set_nfs(
        mut self,
        v: impl Into<BlockAssignable<DatasyncLocationFsxOntapFileSystemProtocolElNfsEl>>,
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
    #[doc = "Set the field `smb`.\n"]
    pub fn set_smb(
        mut self,
        v: impl Into<BlockAssignable<DatasyncLocationFsxOntapFileSystemProtocolElSmbEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.smb = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.smb = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DatasyncLocationFsxOntapFileSystemProtocolEl {
    type O = BlockAssignable<DatasyncLocationFsxOntapFileSystemProtocolEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatasyncLocationFsxOntapFileSystemProtocolEl {}
impl BuildDatasyncLocationFsxOntapFileSystemProtocolEl {
    pub fn build(self) -> DatasyncLocationFsxOntapFileSystemProtocolEl {
        DatasyncLocationFsxOntapFileSystemProtocolEl {
            nfs: core::default::Default::default(),
            smb: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DatasyncLocationFsxOntapFileSystemProtocolElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatasyncLocationFsxOntapFileSystemProtocolElRef {
    fn new(shared: StackShared, base: String) -> DatasyncLocationFsxOntapFileSystemProtocolElRef {
        DatasyncLocationFsxOntapFileSystemProtocolElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatasyncLocationFsxOntapFileSystemProtocolElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `nfs` after provisioning.\n"]
    pub fn nfs(&self) -> ListRef<DatasyncLocationFsxOntapFileSystemProtocolElNfsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.nfs", self.base))
    }
    #[doc = "Get a reference to the value of field `smb` after provisioning.\n"]
    pub fn smb(&self) -> ListRef<DatasyncLocationFsxOntapFileSystemProtocolElSmbElRef> {
        ListRef::new(self.shared().clone(), format!("{}.smb", self.base))
    }
}
#[derive(Serialize, Default)]
struct DatasyncLocationFsxOntapFileSystemDynamic {
    protocol: Option<DynamicBlock<DatasyncLocationFsxOntapFileSystemProtocolEl>>,
}
