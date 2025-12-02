use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct FsxFileCacheData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    copy_tags_to_data_repository_associations: Option<PrimField<bool>>,
    file_cache_type: PrimField<String>,
    file_cache_type_version: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_group_ids: Option<SetField<PrimField<String>>>,
    storage_capacity: PrimField<f64>,
    subnet_ids: ListField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_repository_association: Option<Vec<FsxFileCacheDataRepositoryAssociationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lustre_configuration: Option<Vec<FsxFileCacheLustreConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<FsxFileCacheTimeoutsEl>,
    dynamic: FsxFileCacheDynamic,
}
struct FsxFileCache_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<FsxFileCacheData>,
}
#[derive(Clone)]
pub struct FsxFileCache(Rc<FsxFileCache_>);
impl FsxFileCache {
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
    #[doc = "Set the field `copy_tags_to_data_repository_associations`.\n"]
    pub fn set_copy_tags_to_data_repository_associations(
        self,
        v: impl Into<PrimField<bool>>,
    ) -> Self {
        self.0
            .data
            .borrow_mut()
            .copy_tags_to_data_repository_associations = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().kms_key_id = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `security_group_ids`.\n"]
    pub fn set_security_group_ids(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().security_group_ids = Some(v.into());
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
    #[doc = "Set the field `data_repository_association`.\n"]
    pub fn set_data_repository_association(
        self,
        v: impl Into<BlockAssignable<FsxFileCacheDataRepositoryAssociationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().data_repository_association = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.data_repository_association = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `lustre_configuration`.\n"]
    pub fn set_lustre_configuration(
        self,
        v: impl Into<BlockAssignable<FsxFileCacheLustreConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().lustre_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.lustre_configuration = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<FsxFileCacheTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `copy_tags_to_data_repository_associations` after provisioning.\n"]
    pub fn copy_tags_to_data_repository_associations(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.copy_tags_to_data_repository_associations",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `data_repository_association_ids` after provisioning.\n"]
    pub fn data_repository_association_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.data_repository_association_ids", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dns_name` after provisioning.\n"]
    pub fn dns_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dns_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `file_cache_id` after provisioning.\n"]
    pub fn file_cache_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.file_cache_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `file_cache_type` after provisioning.\n"]
    pub fn file_cache_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.file_cache_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `file_cache_type_version` after provisioning.\n"]
    pub fn file_cache_type_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.file_cache_type_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network_interface_ids` after provisioning.\n"]
    pub fn network_interface_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.network_interface_ids", self.extract_ref()),
        )
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
    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.security_group_ids", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `storage_capacity` after provisioning.\n"]
    pub fn storage_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.storage_capacity", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.subnet_ids", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vpc_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FsxFileCacheTimeoutsElRef {
        FsxFileCacheTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for FsxFileCache {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for FsxFileCache {}
impl ToListMappable for FsxFileCache {
    type O = ListRef<FsxFileCacheRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for FsxFileCache_ {
    fn extract_resource_type(&self) -> String {
        "aws_fsx_file_cache".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildFsxFileCache {
    pub tf_id: String,
    #[doc = ""]
    pub file_cache_type: PrimField<String>,
    #[doc = ""]
    pub file_cache_type_version: PrimField<String>,
    #[doc = ""]
    pub storage_capacity: PrimField<f64>,
    #[doc = ""]
    pub subnet_ids: ListField<PrimField<String>>,
}
impl BuildFsxFileCache {
    pub fn build(self, stack: &mut Stack) -> FsxFileCache {
        let out = FsxFileCache(Rc::new(FsxFileCache_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(FsxFileCacheData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                copy_tags_to_data_repository_associations: core::default::Default::default(),
                file_cache_type: self.file_cache_type,
                file_cache_type_version: self.file_cache_type_version,
                id: core::default::Default::default(),
                kms_key_id: core::default::Default::default(),
                region: core::default::Default::default(),
                security_group_ids: core::default::Default::default(),
                storage_capacity: self.storage_capacity,
                subnet_ids: self.subnet_ids,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                data_repository_association: core::default::Default::default(),
                lustre_configuration: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct FsxFileCacheRef {
    shared: StackShared,
    base: String,
}
impl Ref for FsxFileCacheRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl FsxFileCacheRef {
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
    #[doc = "Get a reference to the value of field `copy_tags_to_data_repository_associations` after provisioning.\n"]
    pub fn copy_tags_to_data_repository_associations(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.copy_tags_to_data_repository_associations",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `data_repository_association_ids` after provisioning.\n"]
    pub fn data_repository_association_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.data_repository_association_ids", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dns_name` after provisioning.\n"]
    pub fn dns_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dns_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `file_cache_id` after provisioning.\n"]
    pub fn file_cache_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.file_cache_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `file_cache_type` after provisioning.\n"]
    pub fn file_cache_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.file_cache_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `file_cache_type_version` after provisioning.\n"]
    pub fn file_cache_type_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.file_cache_type_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network_interface_ids` after provisioning.\n"]
    pub fn network_interface_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.network_interface_ids", self.extract_ref()),
        )
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
    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.security_group_ids", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `storage_capacity` after provisioning.\n"]
    pub fn storage_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.storage_capacity", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.subnet_ids", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vpc_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FsxFileCacheTimeoutsElRef {
        FsxFileCacheTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct FsxFileCacheDataRepositoryAssociationElNfsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dns_ips: Option<SetField<PrimField<String>>>,
    version: PrimField<String>,
}
impl FsxFileCacheDataRepositoryAssociationElNfsEl {
    #[doc = "Set the field `dns_ips`.\n"]
    pub fn set_dns_ips(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.dns_ips = Some(v.into());
        self
    }
}
impl ToListMappable for FsxFileCacheDataRepositoryAssociationElNfsEl {
    type O = BlockAssignable<FsxFileCacheDataRepositoryAssociationElNfsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFsxFileCacheDataRepositoryAssociationElNfsEl {
    #[doc = ""]
    pub version: PrimField<String>,
}
impl BuildFsxFileCacheDataRepositoryAssociationElNfsEl {
    pub fn build(self) -> FsxFileCacheDataRepositoryAssociationElNfsEl {
        FsxFileCacheDataRepositoryAssociationElNfsEl {
            dns_ips: core::default::Default::default(),
            version: self.version,
        }
    }
}
pub struct FsxFileCacheDataRepositoryAssociationElNfsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FsxFileCacheDataRepositoryAssociationElNfsElRef {
    fn new(shared: StackShared, base: String) -> FsxFileCacheDataRepositoryAssociationElNfsElRef {
        FsxFileCacheDataRepositoryAssociationElNfsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FsxFileCacheDataRepositoryAssociationElNfsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `dns_ips` after provisioning.\n"]
    pub fn dns_ips(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.dns_ips", self.base))
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}
#[derive(Serialize, Default)]
struct FsxFileCacheDataRepositoryAssociationElDynamic {
    nfs: Option<DynamicBlock<FsxFileCacheDataRepositoryAssociationElNfsEl>>,
}
#[derive(Serialize)]
pub struct FsxFileCacheDataRepositoryAssociationEl {
    data_repository_path: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_repository_subdirectories: Option<SetField<PrimField<String>>>,
    file_cache_path: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nfs: Option<Vec<FsxFileCacheDataRepositoryAssociationElNfsEl>>,
    dynamic: FsxFileCacheDataRepositoryAssociationElDynamic,
}
impl FsxFileCacheDataRepositoryAssociationEl {
    #[doc = "Set the field `data_repository_subdirectories`.\n"]
    pub fn set_data_repository_subdirectories(
        mut self,
        v: impl Into<SetField<PrimField<String>>>,
    ) -> Self {
        self.data_repository_subdirectories = Some(v.into());
        self
    }
    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.tags = Some(v.into());
        self
    }
    #[doc = "Set the field `nfs`.\n"]
    pub fn set_nfs(
        mut self,
        v: impl Into<BlockAssignable<FsxFileCacheDataRepositoryAssociationElNfsEl>>,
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
impl ToListMappable for FsxFileCacheDataRepositoryAssociationEl {
    type O = BlockAssignable<FsxFileCacheDataRepositoryAssociationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFsxFileCacheDataRepositoryAssociationEl {
    #[doc = ""]
    pub data_repository_path: PrimField<String>,
    #[doc = ""]
    pub file_cache_path: PrimField<String>,
}
impl BuildFsxFileCacheDataRepositoryAssociationEl {
    pub fn build(self) -> FsxFileCacheDataRepositoryAssociationEl {
        FsxFileCacheDataRepositoryAssociationEl {
            data_repository_path: self.data_repository_path,
            data_repository_subdirectories: core::default::Default::default(),
            file_cache_path: self.file_cache_path,
            tags: core::default::Default::default(),
            nfs: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct FsxFileCacheDataRepositoryAssociationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FsxFileCacheDataRepositoryAssociationElRef {
    fn new(shared: StackShared, base: String) -> FsxFileCacheDataRepositoryAssociationElRef {
        FsxFileCacheDataRepositoryAssociationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FsxFileCacheDataRepositoryAssociationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `association_id` after provisioning.\n"]
    pub fn association_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.association_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `data_repository_path` after provisioning.\n"]
    pub fn data_repository_path(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_repository_path", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `data_repository_subdirectories` after provisioning.\n"]
    pub fn data_repository_subdirectories(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.data_repository_subdirectories", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `file_cache_id` after provisioning.\n"]
    pub fn file_cache_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.file_cache_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `file_cache_path` after provisioning.\n"]
    pub fn file_cache_path(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.file_cache_path", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `file_system_id` after provisioning.\n"]
    pub fn file_system_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.file_system_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `file_system_path` after provisioning.\n"]
    pub fn file_system_path(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.file_system_path", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `imported_file_chunk_size` after provisioning.\n"]
    pub fn imported_file_chunk_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.imported_file_chunk_size", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `resource_arn` after provisioning.\n"]
    pub fn resource_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_arn", self.base))
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}
#[derive(Serialize)]
pub struct FsxFileCacheLustreConfigurationElLogConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    destination: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    level: Option<PrimField<String>>,
}
impl FsxFileCacheLustreConfigurationElLogConfigurationEl {
    #[doc = "Set the field `destination`.\n"]
    pub fn set_destination(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.destination = Some(v.into());
        self
    }
    #[doc = "Set the field `level`.\n"]
    pub fn set_level(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.level = Some(v.into());
        self
    }
}
impl ToListMappable for FsxFileCacheLustreConfigurationElLogConfigurationEl {
    type O = BlockAssignable<FsxFileCacheLustreConfigurationElLogConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFsxFileCacheLustreConfigurationElLogConfigurationEl {}
impl BuildFsxFileCacheLustreConfigurationElLogConfigurationEl {
    pub fn build(self) -> FsxFileCacheLustreConfigurationElLogConfigurationEl {
        FsxFileCacheLustreConfigurationElLogConfigurationEl {
            destination: core::default::Default::default(),
            level: core::default::Default::default(),
        }
    }
}
pub struct FsxFileCacheLustreConfigurationElLogConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FsxFileCacheLustreConfigurationElLogConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> FsxFileCacheLustreConfigurationElLogConfigurationElRef {
        FsxFileCacheLustreConfigurationElLogConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FsxFileCacheLustreConfigurationElLogConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.destination", self.base))
    }
    #[doc = "Get a reference to the value of field `level` after provisioning.\n"]
    pub fn level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.level", self.base))
    }
}
#[derive(Serialize)]
pub struct FsxFileCacheLustreConfigurationElMetadataConfigurationEl {
    storage_capacity: PrimField<f64>,
}
impl FsxFileCacheLustreConfigurationElMetadataConfigurationEl {}
impl ToListMappable for FsxFileCacheLustreConfigurationElMetadataConfigurationEl {
    type O = BlockAssignable<FsxFileCacheLustreConfigurationElMetadataConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFsxFileCacheLustreConfigurationElMetadataConfigurationEl {
    #[doc = ""]
    pub storage_capacity: PrimField<f64>,
}
impl BuildFsxFileCacheLustreConfigurationElMetadataConfigurationEl {
    pub fn build(self) -> FsxFileCacheLustreConfigurationElMetadataConfigurationEl {
        FsxFileCacheLustreConfigurationElMetadataConfigurationEl {
            storage_capacity: self.storage_capacity,
        }
    }
}
pub struct FsxFileCacheLustreConfigurationElMetadataConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FsxFileCacheLustreConfigurationElMetadataConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> FsxFileCacheLustreConfigurationElMetadataConfigurationElRef {
        FsxFileCacheLustreConfigurationElMetadataConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FsxFileCacheLustreConfigurationElMetadataConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `storage_capacity` after provisioning.\n"]
    pub fn storage_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.storage_capacity", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct FsxFileCacheLustreConfigurationElDynamic {
    metadata_configuration:
        Option<DynamicBlock<FsxFileCacheLustreConfigurationElMetadataConfigurationEl>>,
}
#[derive(Serialize)]
pub struct FsxFileCacheLustreConfigurationEl {
    deployment_type: PrimField<String>,
    per_unit_storage_throughput: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weekly_maintenance_start_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata_configuration: Option<Vec<FsxFileCacheLustreConfigurationElMetadataConfigurationEl>>,
    dynamic: FsxFileCacheLustreConfigurationElDynamic,
}
impl FsxFileCacheLustreConfigurationEl {
    #[doc = "Set the field `weekly_maintenance_start_time`.\n"]
    pub fn set_weekly_maintenance_start_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.weekly_maintenance_start_time = Some(v.into());
        self
    }
    #[doc = "Set the field `metadata_configuration`.\n"]
    pub fn set_metadata_configuration(
        mut self,
        v: impl Into<BlockAssignable<FsxFileCacheLustreConfigurationElMetadataConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.metadata_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.metadata_configuration = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for FsxFileCacheLustreConfigurationEl {
    type O = BlockAssignable<FsxFileCacheLustreConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFsxFileCacheLustreConfigurationEl {
    #[doc = ""]
    pub deployment_type: PrimField<String>,
    #[doc = ""]
    pub per_unit_storage_throughput: PrimField<f64>,
}
impl BuildFsxFileCacheLustreConfigurationEl {
    pub fn build(self) -> FsxFileCacheLustreConfigurationEl {
        FsxFileCacheLustreConfigurationEl {
            deployment_type: self.deployment_type,
            per_unit_storage_throughput: self.per_unit_storage_throughput,
            weekly_maintenance_start_time: core::default::Default::default(),
            metadata_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct FsxFileCacheLustreConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FsxFileCacheLustreConfigurationElRef {
    fn new(shared: StackShared, base: String) -> FsxFileCacheLustreConfigurationElRef {
        FsxFileCacheLustreConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FsxFileCacheLustreConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `deployment_type` after provisioning.\n"]
    pub fn deployment_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deployment_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `log_configuration` after provisioning.\n"]
    pub fn log_configuration(
        &self,
    ) -> SetRef<FsxFileCacheLustreConfigurationElLogConfigurationElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.log_configuration", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `mount_name` after provisioning.\n"]
    pub fn mount_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mount_name", self.base))
    }
    #[doc = "Get a reference to the value of field `per_unit_storage_throughput` after provisioning.\n"]
    pub fn per_unit_storage_throughput(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.per_unit_storage_throughput", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `weekly_maintenance_start_time` after provisioning.\n"]
    pub fn weekly_maintenance_start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.weekly_maintenance_start_time", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct FsxFileCacheTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl FsxFileCacheTimeoutsEl {
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
impl ToListMappable for FsxFileCacheTimeoutsEl {
    type O = BlockAssignable<FsxFileCacheTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFsxFileCacheTimeoutsEl {}
impl BuildFsxFileCacheTimeoutsEl {
    pub fn build(self) -> FsxFileCacheTimeoutsEl {
        FsxFileCacheTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct FsxFileCacheTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FsxFileCacheTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> FsxFileCacheTimeoutsElRef {
        FsxFileCacheTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FsxFileCacheTimeoutsElRef {
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
struct FsxFileCacheDynamic {
    data_repository_association: Option<DynamicBlock<FsxFileCacheDataRepositoryAssociationEl>>,
    lustre_configuration: Option<DynamicBlock<FsxFileCacheLustreConfigurationEl>>,
}
