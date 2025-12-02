use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataEfsAccessPointData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    access_point_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
}
struct DataEfsAccessPoint_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataEfsAccessPointData>,
}
#[derive(Clone)]
pub struct DataEfsAccessPoint(Rc<DataEfsAccessPoint_>);
impl DataEfsAccessPoint {
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
    #[doc = "Get a reference to the value of field `access_point_id` after provisioning.\n"]
    pub fn access_point_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.access_point_id", self.extract_ref()),
        )
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
    #[doc = "Get a reference to the value of field `posix_user` after provisioning.\n"]
    pub fn posix_user(&self) -> ListRef<DataEfsAccessPointPosixUserElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.posix_user", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `root_directory` after provisioning.\n"]
    pub fn root_directory(&self) -> ListRef<DataEfsAccessPointRootDirectoryElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.root_directory", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
}
impl Referable for DataEfsAccessPoint {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataEfsAccessPoint {}
impl ToListMappable for DataEfsAccessPoint {
    type O = ListRef<DataEfsAccessPointRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataEfsAccessPoint_ {
    fn extract_datasource_type(&self) -> String {
        "aws_efs_access_point".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataEfsAccessPoint {
    pub tf_id: String,
    #[doc = ""]
    pub access_point_id: PrimField<String>,
}
impl BuildDataEfsAccessPoint {
    pub fn build(self, stack: &mut Stack) -> DataEfsAccessPoint {
        let out = DataEfsAccessPoint(Rc::new(DataEfsAccessPoint_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataEfsAccessPointData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                access_point_id: self.access_point_id,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataEfsAccessPointRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataEfsAccessPointRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataEfsAccessPointRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `access_point_id` after provisioning.\n"]
    pub fn access_point_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.access_point_id", self.extract_ref()),
        )
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
    #[doc = "Get a reference to the value of field `posix_user` after provisioning.\n"]
    pub fn posix_user(&self) -> ListRef<DataEfsAccessPointPosixUserElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.posix_user", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `root_directory` after provisioning.\n"]
    pub fn root_directory(&self) -> ListRef<DataEfsAccessPointRootDirectoryElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.root_directory", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataEfsAccessPointPosixUserEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    gid: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secondary_gids: Option<SetField<PrimField<f64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    uid: Option<PrimField<f64>>,
}
impl DataEfsAccessPointPosixUserEl {
    #[doc = "Set the field `gid`.\n"]
    pub fn set_gid(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.gid = Some(v.into());
        self
    }
    #[doc = "Set the field `secondary_gids`.\n"]
    pub fn set_secondary_gids(mut self, v: impl Into<SetField<PrimField<f64>>>) -> Self {
        self.secondary_gids = Some(v.into());
        self
    }
    #[doc = "Set the field `uid`.\n"]
    pub fn set_uid(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.uid = Some(v.into());
        self
    }
}
impl ToListMappable for DataEfsAccessPointPosixUserEl {
    type O = BlockAssignable<DataEfsAccessPointPosixUserEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataEfsAccessPointPosixUserEl {}
impl BuildDataEfsAccessPointPosixUserEl {
    pub fn build(self) -> DataEfsAccessPointPosixUserEl {
        DataEfsAccessPointPosixUserEl {
            gid: core::default::Default::default(),
            secondary_gids: core::default::Default::default(),
            uid: core::default::Default::default(),
        }
    }
}
pub struct DataEfsAccessPointPosixUserElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataEfsAccessPointPosixUserElRef {
    fn new(shared: StackShared, base: String) -> DataEfsAccessPointPosixUserElRef {
        DataEfsAccessPointPosixUserElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataEfsAccessPointPosixUserElRef {
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
pub struct DataEfsAccessPointRootDirectoryElCreationInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    owner_gid: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    owner_uid: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    permissions: Option<PrimField<String>>,
}
impl DataEfsAccessPointRootDirectoryElCreationInfoEl {
    #[doc = "Set the field `owner_gid`.\n"]
    pub fn set_owner_gid(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.owner_gid = Some(v.into());
        self
    }
    #[doc = "Set the field `owner_uid`.\n"]
    pub fn set_owner_uid(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.owner_uid = Some(v.into());
        self
    }
    #[doc = "Set the field `permissions`.\n"]
    pub fn set_permissions(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.permissions = Some(v.into());
        self
    }
}
impl ToListMappable for DataEfsAccessPointRootDirectoryElCreationInfoEl {
    type O = BlockAssignable<DataEfsAccessPointRootDirectoryElCreationInfoEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataEfsAccessPointRootDirectoryElCreationInfoEl {}
impl BuildDataEfsAccessPointRootDirectoryElCreationInfoEl {
    pub fn build(self) -> DataEfsAccessPointRootDirectoryElCreationInfoEl {
        DataEfsAccessPointRootDirectoryElCreationInfoEl {
            owner_gid: core::default::Default::default(),
            owner_uid: core::default::Default::default(),
            permissions: core::default::Default::default(),
        }
    }
}
pub struct DataEfsAccessPointRootDirectoryElCreationInfoElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataEfsAccessPointRootDirectoryElCreationInfoElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataEfsAccessPointRootDirectoryElCreationInfoElRef {
        DataEfsAccessPointRootDirectoryElCreationInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataEfsAccessPointRootDirectoryElCreationInfoElRef {
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
#[derive(Serialize)]
pub struct DataEfsAccessPointRootDirectoryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    creation_info: Option<ListField<DataEfsAccessPointRootDirectoryElCreationInfoEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
}
impl DataEfsAccessPointRootDirectoryEl {
    #[doc = "Set the field `creation_info`.\n"]
    pub fn set_creation_info(
        mut self,
        v: impl Into<ListField<DataEfsAccessPointRootDirectoryElCreationInfoEl>>,
    ) -> Self {
        self.creation_info = Some(v.into());
        self
    }
    #[doc = "Set the field `path`.\n"]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }
}
impl ToListMappable for DataEfsAccessPointRootDirectoryEl {
    type O = BlockAssignable<DataEfsAccessPointRootDirectoryEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataEfsAccessPointRootDirectoryEl {}
impl BuildDataEfsAccessPointRootDirectoryEl {
    pub fn build(self) -> DataEfsAccessPointRootDirectoryEl {
        DataEfsAccessPointRootDirectoryEl {
            creation_info: core::default::Default::default(),
            path: core::default::Default::default(),
        }
    }
}
pub struct DataEfsAccessPointRootDirectoryElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataEfsAccessPointRootDirectoryElRef {
    fn new(shared: StackShared, base: String) -> DataEfsAccessPointRootDirectoryElRef {
        DataEfsAccessPointRootDirectoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataEfsAccessPointRootDirectoryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `creation_info` after provisioning.\n"]
    pub fn creation_info(&self) -> ListRef<DataEfsAccessPointRootDirectoryElCreationInfoElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.creation_info", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
}
