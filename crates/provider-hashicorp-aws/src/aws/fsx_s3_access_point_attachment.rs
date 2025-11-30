use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct FsxS3AccessPointAttachmentData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    openzfs_configuration: Option<Vec<FsxS3AccessPointAttachmentOpenzfsConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_access_point: Option<Vec<FsxS3AccessPointAttachmentS3AccessPointEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<FsxS3AccessPointAttachmentTimeoutsEl>,
    dynamic: FsxS3AccessPointAttachmentDynamic,
}

struct FsxS3AccessPointAttachment_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<FsxS3AccessPointAttachmentData>,
}

#[derive(Clone)]
pub struct FsxS3AccessPointAttachment(Rc<FsxS3AccessPointAttachment_>);

impl FsxS3AccessPointAttachment {
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

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `openzfs_configuration`.\n"]
    pub fn set_openzfs_configuration(
        self,
        v: impl Into<BlockAssignable<FsxS3AccessPointAttachmentOpenzfsConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().openzfs_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.openzfs_configuration = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `s3_access_point`.\n"]
    pub fn set_s3_access_point(
        self,
        v: impl Into<BlockAssignable<FsxS3AccessPointAttachmentS3AccessPointEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().s3_access_point = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.s3_access_point = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<FsxS3AccessPointAttachmentTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `s3_access_point_alias` after provisioning.\n"]
    pub fn s3_access_point_alias(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.s3_access_point_alias", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `s3_access_point_arn` after provisioning.\n"]
    pub fn s3_access_point_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.s3_access_point_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `openzfs_configuration` after provisioning.\n"]
    pub fn openzfs_configuration(
        &self,
    ) -> ListRef<FsxS3AccessPointAttachmentOpenzfsConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.openzfs_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `s3_access_point` after provisioning.\n"]
    pub fn s3_access_point(&self) -> ListRef<FsxS3AccessPointAttachmentS3AccessPointElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.s3_access_point", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FsxS3AccessPointAttachmentTimeoutsElRef {
        FsxS3AccessPointAttachmentTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for FsxS3AccessPointAttachment {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for FsxS3AccessPointAttachment {}

impl ToListMappable for FsxS3AccessPointAttachment {
    type O = ListRef<FsxS3AccessPointAttachmentRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for FsxS3AccessPointAttachment_ {
    fn extract_resource_type(&self) -> String {
        "aws_fsx_s3_access_point_attachment".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildFsxS3AccessPointAttachment {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl BuildFsxS3AccessPointAttachment {
    pub fn build(self, stack: &mut Stack) -> FsxS3AccessPointAttachment {
        let out = FsxS3AccessPointAttachment(Rc::new(FsxS3AccessPointAttachment_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(FsxS3AccessPointAttachmentData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                name: self.name,
                region: core::default::Default::default(),
                type_: self.type_,
                openzfs_configuration: core::default::Default::default(),
                s3_access_point: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct FsxS3AccessPointAttachmentRef {
    shared: StackShared,
    base: String,
}

impl Ref for FsxS3AccessPointAttachmentRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl FsxS3AccessPointAttachmentRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `s3_access_point_alias` after provisioning.\n"]
    pub fn s3_access_point_alias(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.s3_access_point_alias", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `s3_access_point_arn` after provisioning.\n"]
    pub fn s3_access_point_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.s3_access_point_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `openzfs_configuration` after provisioning.\n"]
    pub fn openzfs_configuration(
        &self,
    ) -> ListRef<FsxS3AccessPointAttachmentOpenzfsConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.openzfs_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `s3_access_point` after provisioning.\n"]
    pub fn s3_access_point(&self) -> ListRef<FsxS3AccessPointAttachmentS3AccessPointElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.s3_access_point", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FsxS3AccessPointAttachmentTimeoutsElRef {
        FsxS3AccessPointAttachmentTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct FsxS3AccessPointAttachmentOpenzfsConfigurationElFileSystemIdentityElPosixUserEl {
    gid: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secondary_gids: Option<ListField<PrimField<f64>>>,
    uid: PrimField<f64>,
}

impl FsxS3AccessPointAttachmentOpenzfsConfigurationElFileSystemIdentityElPosixUserEl {
    #[doc = "Set the field `secondary_gids`.\n"]
    pub fn set_secondary_gids(mut self, v: impl Into<ListField<PrimField<f64>>>) -> Self {
        self.secondary_gids = Some(v.into());
        self
    }
}

impl ToListMappable
    for FsxS3AccessPointAttachmentOpenzfsConfigurationElFileSystemIdentityElPosixUserEl
{
    type O = BlockAssignable<
        FsxS3AccessPointAttachmentOpenzfsConfigurationElFileSystemIdentityElPosixUserEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxS3AccessPointAttachmentOpenzfsConfigurationElFileSystemIdentityElPosixUserEl {
    #[doc = ""]
    pub gid: PrimField<f64>,
    #[doc = ""]
    pub uid: PrimField<f64>,
}

impl BuildFsxS3AccessPointAttachmentOpenzfsConfigurationElFileSystemIdentityElPosixUserEl {
    pub fn build(
        self,
    ) -> FsxS3AccessPointAttachmentOpenzfsConfigurationElFileSystemIdentityElPosixUserEl {
        FsxS3AccessPointAttachmentOpenzfsConfigurationElFileSystemIdentityElPosixUserEl {
            gid: self.gid,
            secondary_gids: core::default::Default::default(),
            uid: self.uid,
        }
    }
}

pub struct FsxS3AccessPointAttachmentOpenzfsConfigurationElFileSystemIdentityElPosixUserElRef {
    shared: StackShared,
    base: String,
}

impl Ref for FsxS3AccessPointAttachmentOpenzfsConfigurationElFileSystemIdentityElPosixUserElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> FsxS3AccessPointAttachmentOpenzfsConfigurationElFileSystemIdentityElPosixUserElRef {
        FsxS3AccessPointAttachmentOpenzfsConfigurationElFileSystemIdentityElPosixUserElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxS3AccessPointAttachmentOpenzfsConfigurationElFileSystemIdentityElPosixUserElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `gid` after provisioning.\n"]
    pub fn gid(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.gid", self.base))
    }

    #[doc = "Get a reference to the value of field `secondary_gids` after provisioning.\n"]
    pub fn secondary_gids(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(
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
struct FsxS3AccessPointAttachmentOpenzfsConfigurationElFileSystemIdentityElDynamic {
    posix_user: Option<
        DynamicBlock<
            FsxS3AccessPointAttachmentOpenzfsConfigurationElFileSystemIdentityElPosixUserEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct FsxS3AccessPointAttachmentOpenzfsConfigurationElFileSystemIdentityEl {
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    posix_user: Option<
        Vec<FsxS3AccessPointAttachmentOpenzfsConfigurationElFileSystemIdentityElPosixUserEl>,
    >,
    dynamic: FsxS3AccessPointAttachmentOpenzfsConfigurationElFileSystemIdentityElDynamic,
}

impl FsxS3AccessPointAttachmentOpenzfsConfigurationElFileSystemIdentityEl {
    #[doc = "Set the field `posix_user`.\n"]
    pub fn set_posix_user(
        mut self,
        v: impl Into<
            BlockAssignable<
                FsxS3AccessPointAttachmentOpenzfsConfigurationElFileSystemIdentityElPosixUserEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.posix_user = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.posix_user = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for FsxS3AccessPointAttachmentOpenzfsConfigurationElFileSystemIdentityEl {
    type O = BlockAssignable<FsxS3AccessPointAttachmentOpenzfsConfigurationElFileSystemIdentityEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxS3AccessPointAttachmentOpenzfsConfigurationElFileSystemIdentityEl {
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl BuildFsxS3AccessPointAttachmentOpenzfsConfigurationElFileSystemIdentityEl {
    pub fn build(self) -> FsxS3AccessPointAttachmentOpenzfsConfigurationElFileSystemIdentityEl {
        FsxS3AccessPointAttachmentOpenzfsConfigurationElFileSystemIdentityEl {
            type_: self.type_,
            posix_user: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct FsxS3AccessPointAttachmentOpenzfsConfigurationElFileSystemIdentityElRef {
    shared: StackShared,
    base: String,
}

impl Ref for FsxS3AccessPointAttachmentOpenzfsConfigurationElFileSystemIdentityElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> FsxS3AccessPointAttachmentOpenzfsConfigurationElFileSystemIdentityElRef {
        FsxS3AccessPointAttachmentOpenzfsConfigurationElFileSystemIdentityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxS3AccessPointAttachmentOpenzfsConfigurationElFileSystemIdentityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `posix_user` after provisioning.\n"]
    pub fn posix_user(
        &self,
    ) -> ListRef<FsxS3AccessPointAttachmentOpenzfsConfigurationElFileSystemIdentityElPosixUserElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.posix_user", self.base))
    }
}

#[derive(Serialize, Default)]
struct FsxS3AccessPointAttachmentOpenzfsConfigurationElDynamic {
    file_system_identity:
        Option<DynamicBlock<FsxS3AccessPointAttachmentOpenzfsConfigurationElFileSystemIdentityEl>>,
}

#[derive(Serialize)]
pub struct FsxS3AccessPointAttachmentOpenzfsConfigurationEl {
    volume_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file_system_identity:
        Option<Vec<FsxS3AccessPointAttachmentOpenzfsConfigurationElFileSystemIdentityEl>>,
    dynamic: FsxS3AccessPointAttachmentOpenzfsConfigurationElDynamic,
}

impl FsxS3AccessPointAttachmentOpenzfsConfigurationEl {
    #[doc = "Set the field `file_system_identity`.\n"]
    pub fn set_file_system_identity(
        mut self,
        v: impl Into<
            BlockAssignable<FsxS3AccessPointAttachmentOpenzfsConfigurationElFileSystemIdentityEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.file_system_identity = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.file_system_identity = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for FsxS3AccessPointAttachmentOpenzfsConfigurationEl {
    type O = BlockAssignable<FsxS3AccessPointAttachmentOpenzfsConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxS3AccessPointAttachmentOpenzfsConfigurationEl {
    #[doc = ""]
    pub volume_id: PrimField<String>,
}

impl BuildFsxS3AccessPointAttachmentOpenzfsConfigurationEl {
    pub fn build(self) -> FsxS3AccessPointAttachmentOpenzfsConfigurationEl {
        FsxS3AccessPointAttachmentOpenzfsConfigurationEl {
            volume_id: self.volume_id,
            file_system_identity: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct FsxS3AccessPointAttachmentOpenzfsConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for FsxS3AccessPointAttachmentOpenzfsConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> FsxS3AccessPointAttachmentOpenzfsConfigurationElRef {
        FsxS3AccessPointAttachmentOpenzfsConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxS3AccessPointAttachmentOpenzfsConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `volume_id` after provisioning.\n"]
    pub fn volume_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.volume_id", self.base))
    }

    #[doc = "Get a reference to the value of field `file_system_identity` after provisioning.\n"]
    pub fn file_system_identity(
        &self,
    ) -> ListRef<FsxS3AccessPointAttachmentOpenzfsConfigurationElFileSystemIdentityElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.file_system_identity", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct FsxS3AccessPointAttachmentS3AccessPointElVpcConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_id: Option<PrimField<String>>,
}

impl FsxS3AccessPointAttachmentS3AccessPointElVpcConfigurationEl {
    #[doc = "Set the field `vpc_id`.\n"]
    pub fn set_vpc_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.vpc_id = Some(v.into());
        self
    }
}

impl ToListMappable for FsxS3AccessPointAttachmentS3AccessPointElVpcConfigurationEl {
    type O = BlockAssignable<FsxS3AccessPointAttachmentS3AccessPointElVpcConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxS3AccessPointAttachmentS3AccessPointElVpcConfigurationEl {}

impl BuildFsxS3AccessPointAttachmentS3AccessPointElVpcConfigurationEl {
    pub fn build(self) -> FsxS3AccessPointAttachmentS3AccessPointElVpcConfigurationEl {
        FsxS3AccessPointAttachmentS3AccessPointElVpcConfigurationEl {
            vpc_id: core::default::Default::default(),
        }
    }
}

pub struct FsxS3AccessPointAttachmentS3AccessPointElVpcConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for FsxS3AccessPointAttachmentS3AccessPointElVpcConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> FsxS3AccessPointAttachmentS3AccessPointElVpcConfigurationElRef {
        FsxS3AccessPointAttachmentS3AccessPointElVpcConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxS3AccessPointAttachmentS3AccessPointElVpcConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct FsxS3AccessPointAttachmentS3AccessPointElDynamic {
    vpc_configuration:
        Option<DynamicBlock<FsxS3AccessPointAttachmentS3AccessPointElVpcConfigurationEl>>,
}

#[derive(Serialize)]
pub struct FsxS3AccessPointAttachmentS3AccessPointEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_configuration: Option<Vec<FsxS3AccessPointAttachmentS3AccessPointElVpcConfigurationEl>>,
    dynamic: FsxS3AccessPointAttachmentS3AccessPointElDynamic,
}

impl FsxS3AccessPointAttachmentS3AccessPointEl {
    #[doc = "Set the field `policy`.\n"]
    pub fn set_policy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.policy = Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_configuration`.\n"]
    pub fn set_vpc_configuration(
        mut self,
        v: impl Into<BlockAssignable<FsxS3AccessPointAttachmentS3AccessPointElVpcConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.vpc_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.vpc_configuration = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for FsxS3AccessPointAttachmentS3AccessPointEl {
    type O = BlockAssignable<FsxS3AccessPointAttachmentS3AccessPointEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxS3AccessPointAttachmentS3AccessPointEl {}

impl BuildFsxS3AccessPointAttachmentS3AccessPointEl {
    pub fn build(self) -> FsxS3AccessPointAttachmentS3AccessPointEl {
        FsxS3AccessPointAttachmentS3AccessPointEl {
            policy: core::default::Default::default(),
            vpc_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct FsxS3AccessPointAttachmentS3AccessPointElRef {
    shared: StackShared,
    base: String,
}

impl Ref for FsxS3AccessPointAttachmentS3AccessPointElRef {
    fn new(shared: StackShared, base: String) -> FsxS3AccessPointAttachmentS3AccessPointElRef {
        FsxS3AccessPointAttachmentS3AccessPointElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxS3AccessPointAttachmentS3AccessPointElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `policy` after provisioning.\n"]
    pub fn policy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.policy", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_configuration` after provisioning.\n"]
    pub fn vpc_configuration(
        &self,
    ) -> ListRef<FsxS3AccessPointAttachmentS3AccessPointElVpcConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vpc_configuration", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct FsxS3AccessPointAttachmentTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}

impl FsxS3AccessPointAttachmentTimeoutsEl {
    #[doc = "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }

    #[doc = "Set the field `delete`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }
}

impl ToListMappable for FsxS3AccessPointAttachmentTimeoutsEl {
    type O = BlockAssignable<FsxS3AccessPointAttachmentTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxS3AccessPointAttachmentTimeoutsEl {}

impl BuildFsxS3AccessPointAttachmentTimeoutsEl {
    pub fn build(self) -> FsxS3AccessPointAttachmentTimeoutsEl {
        FsxS3AccessPointAttachmentTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

pub struct FsxS3AccessPointAttachmentTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for FsxS3AccessPointAttachmentTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> FsxS3AccessPointAttachmentTimeoutsElRef {
        FsxS3AccessPointAttachmentTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxS3AccessPointAttachmentTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc = "Get a reference to the value of field `delete` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }
}

#[derive(Serialize, Default)]
struct FsxS3AccessPointAttachmentDynamic {
    openzfs_configuration: Option<DynamicBlock<FsxS3AccessPointAttachmentOpenzfsConfigurationEl>>,
    s3_access_point: Option<DynamicBlock<FsxS3AccessPointAttachmentS3AccessPointEl>>,
}
