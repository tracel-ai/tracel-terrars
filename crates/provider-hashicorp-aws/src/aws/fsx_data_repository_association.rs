use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct FsxDataRepositoryAssociationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    batch_import_meta_data_on_create: Option<PrimField<bool>>,
    data_repository_path: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete_data_in_filesystem: Option<PrimField<bool>>,
    file_system_id: PrimField<String>,
    file_system_path: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    imported_file_chunk_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3: Option<Vec<FsxDataRepositoryAssociationS3El>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<FsxDataRepositoryAssociationTimeoutsEl>,
    dynamic: FsxDataRepositoryAssociationDynamic,
}
struct FsxDataRepositoryAssociation_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<FsxDataRepositoryAssociationData>,
}
#[derive(Clone)]
pub struct FsxDataRepositoryAssociation(Rc<FsxDataRepositoryAssociation_>);
impl FsxDataRepositoryAssociation {
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
    #[doc = "Set the field `batch_import_meta_data_on_create`.\n"]
    pub fn set_batch_import_meta_data_on_create(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().batch_import_meta_data_on_create = Some(v.into());
        self
    }
    #[doc = "Set the field `delete_data_in_filesystem`.\n"]
    pub fn set_delete_data_in_filesystem(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().delete_data_in_filesystem = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `imported_file_chunk_size`.\n"]
    pub fn set_imported_file_chunk_size(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().imported_file_chunk_size = Some(v.into());
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
    #[doc = "Set the field `s3`.\n"]
    pub fn set_s3(self, v: impl Into<BlockAssignable<FsxDataRepositoryAssociationS3El>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().s3 = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.s3 = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<FsxDataRepositoryAssociationTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `association_id` after provisioning.\n"]
    pub fn association_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.association_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `batch_import_meta_data_on_create` after provisioning.\n"]
    pub fn batch_import_meta_data_on_create(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.batch_import_meta_data_on_create", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_repository_path` after provisioning.\n"]
    pub fn data_repository_path(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_repository_path", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `delete_data_in_filesystem` after provisioning.\n"]
    pub fn delete_data_in_filesystem(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delete_data_in_filesystem", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `file_system_id` after provisioning.\n"]
    pub fn file_system_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.file_system_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `file_system_path` after provisioning.\n"]
    pub fn file_system_path(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.file_system_path", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `imported_file_chunk_size` after provisioning.\n"]
    pub fn imported_file_chunk_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.imported_file_chunk_size", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `s3` after provisioning.\n"]
    pub fn s3(&self) -> ListRef<FsxDataRepositoryAssociationS3ElRef> {
        ListRef::new(self.shared().clone(), format!("{}.s3", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FsxDataRepositoryAssociationTimeoutsElRef {
        FsxDataRepositoryAssociationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for FsxDataRepositoryAssociation {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for FsxDataRepositoryAssociation {}
impl ToListMappable for FsxDataRepositoryAssociation {
    type O = ListRef<FsxDataRepositoryAssociationRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for FsxDataRepositoryAssociation_ {
    fn extract_resource_type(&self) -> String {
        "aws_fsx_data_repository_association".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildFsxDataRepositoryAssociation {
    pub tf_id: String,
    #[doc = ""]
    pub data_repository_path: PrimField<String>,
    #[doc = ""]
    pub file_system_id: PrimField<String>,
    #[doc = ""]
    pub file_system_path: PrimField<String>,
}
impl BuildFsxDataRepositoryAssociation {
    pub fn build(self, stack: &mut Stack) -> FsxDataRepositoryAssociation {
        let out = FsxDataRepositoryAssociation(Rc::new(FsxDataRepositoryAssociation_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(FsxDataRepositoryAssociationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                batch_import_meta_data_on_create: core::default::Default::default(),
                data_repository_path: self.data_repository_path,
                delete_data_in_filesystem: core::default::Default::default(),
                file_system_id: self.file_system_id,
                file_system_path: self.file_system_path,
                id: core::default::Default::default(),
                imported_file_chunk_size: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                s3: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct FsxDataRepositoryAssociationRef {
    shared: StackShared,
    base: String,
}
impl Ref for FsxDataRepositoryAssociationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl FsxDataRepositoryAssociationRef {
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
    #[doc = "Get a reference to the value of field `association_id` after provisioning.\n"]
    pub fn association_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.association_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `batch_import_meta_data_on_create` after provisioning.\n"]
    pub fn batch_import_meta_data_on_create(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.batch_import_meta_data_on_create", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_repository_path` after provisioning.\n"]
    pub fn data_repository_path(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_repository_path", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `delete_data_in_filesystem` after provisioning.\n"]
    pub fn delete_data_in_filesystem(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delete_data_in_filesystem", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `file_system_id` after provisioning.\n"]
    pub fn file_system_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.file_system_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `file_system_path` after provisioning.\n"]
    pub fn file_system_path(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.file_system_path", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `imported_file_chunk_size` after provisioning.\n"]
    pub fn imported_file_chunk_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.imported_file_chunk_size", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `s3` after provisioning.\n"]
    pub fn s3(&self) -> ListRef<FsxDataRepositoryAssociationS3ElRef> {
        ListRef::new(self.shared().clone(), format!("{}.s3", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FsxDataRepositoryAssociationTimeoutsElRef {
        FsxDataRepositoryAssociationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct FsxDataRepositoryAssociationS3ElAutoExportPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    events: Option<ListField<PrimField<String>>>,
}
impl FsxDataRepositoryAssociationS3ElAutoExportPolicyEl {
    #[doc = "Set the field `events`.\n"]
    pub fn set_events(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.events = Some(v.into());
        self
    }
}
impl ToListMappable for FsxDataRepositoryAssociationS3ElAutoExportPolicyEl {
    type O = BlockAssignable<FsxDataRepositoryAssociationS3ElAutoExportPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFsxDataRepositoryAssociationS3ElAutoExportPolicyEl {}
impl BuildFsxDataRepositoryAssociationS3ElAutoExportPolicyEl {
    pub fn build(self) -> FsxDataRepositoryAssociationS3ElAutoExportPolicyEl {
        FsxDataRepositoryAssociationS3ElAutoExportPolicyEl {
            events: core::default::Default::default(),
        }
    }
}
pub struct FsxDataRepositoryAssociationS3ElAutoExportPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FsxDataRepositoryAssociationS3ElAutoExportPolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> FsxDataRepositoryAssociationS3ElAutoExportPolicyElRef {
        FsxDataRepositoryAssociationS3ElAutoExportPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FsxDataRepositoryAssociationS3ElAutoExportPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `events` after provisioning.\n"]
    pub fn events(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.events", self.base))
    }
}
#[derive(Serialize)]
pub struct FsxDataRepositoryAssociationS3ElAutoImportPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    events: Option<ListField<PrimField<String>>>,
}
impl FsxDataRepositoryAssociationS3ElAutoImportPolicyEl {
    #[doc = "Set the field `events`.\n"]
    pub fn set_events(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.events = Some(v.into());
        self
    }
}
impl ToListMappable for FsxDataRepositoryAssociationS3ElAutoImportPolicyEl {
    type O = BlockAssignable<FsxDataRepositoryAssociationS3ElAutoImportPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFsxDataRepositoryAssociationS3ElAutoImportPolicyEl {}
impl BuildFsxDataRepositoryAssociationS3ElAutoImportPolicyEl {
    pub fn build(self) -> FsxDataRepositoryAssociationS3ElAutoImportPolicyEl {
        FsxDataRepositoryAssociationS3ElAutoImportPolicyEl {
            events: core::default::Default::default(),
        }
    }
}
pub struct FsxDataRepositoryAssociationS3ElAutoImportPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FsxDataRepositoryAssociationS3ElAutoImportPolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> FsxDataRepositoryAssociationS3ElAutoImportPolicyElRef {
        FsxDataRepositoryAssociationS3ElAutoImportPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FsxDataRepositoryAssociationS3ElAutoImportPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `events` after provisioning.\n"]
    pub fn events(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.events", self.base))
    }
}
#[derive(Serialize, Default)]
struct FsxDataRepositoryAssociationS3ElDynamic {
    auto_export_policy: Option<DynamicBlock<FsxDataRepositoryAssociationS3ElAutoExportPolicyEl>>,
    auto_import_policy: Option<DynamicBlock<FsxDataRepositoryAssociationS3ElAutoImportPolicyEl>>,
}
#[derive(Serialize)]
pub struct FsxDataRepositoryAssociationS3El {
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_export_policy: Option<Vec<FsxDataRepositoryAssociationS3ElAutoExportPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_import_policy: Option<Vec<FsxDataRepositoryAssociationS3ElAutoImportPolicyEl>>,
    dynamic: FsxDataRepositoryAssociationS3ElDynamic,
}
impl FsxDataRepositoryAssociationS3El {
    #[doc = "Set the field `auto_export_policy`.\n"]
    pub fn set_auto_export_policy(
        mut self,
        v: impl Into<BlockAssignable<FsxDataRepositoryAssociationS3ElAutoExportPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.auto_export_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.auto_export_policy = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `auto_import_policy`.\n"]
    pub fn set_auto_import_policy(
        mut self,
        v: impl Into<BlockAssignable<FsxDataRepositoryAssociationS3ElAutoImportPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.auto_import_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.auto_import_policy = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for FsxDataRepositoryAssociationS3El {
    type O = BlockAssignable<FsxDataRepositoryAssociationS3El>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFsxDataRepositoryAssociationS3El {}
impl BuildFsxDataRepositoryAssociationS3El {
    pub fn build(self) -> FsxDataRepositoryAssociationS3El {
        FsxDataRepositoryAssociationS3El {
            auto_export_policy: core::default::Default::default(),
            auto_import_policy: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct FsxDataRepositoryAssociationS3ElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FsxDataRepositoryAssociationS3ElRef {
    fn new(shared: StackShared, base: String) -> FsxDataRepositoryAssociationS3ElRef {
        FsxDataRepositoryAssociationS3ElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FsxDataRepositoryAssociationS3ElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `auto_export_policy` after provisioning.\n"]
    pub fn auto_export_policy(
        &self,
    ) -> ListRef<FsxDataRepositoryAssociationS3ElAutoExportPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.auto_export_policy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `auto_import_policy` after provisioning.\n"]
    pub fn auto_import_policy(
        &self,
    ) -> ListRef<FsxDataRepositoryAssociationS3ElAutoImportPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.auto_import_policy", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct FsxDataRepositoryAssociationTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl FsxDataRepositoryAssociationTimeoutsEl {
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
impl ToListMappable for FsxDataRepositoryAssociationTimeoutsEl {
    type O = BlockAssignable<FsxDataRepositoryAssociationTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFsxDataRepositoryAssociationTimeoutsEl {}
impl BuildFsxDataRepositoryAssociationTimeoutsEl {
    pub fn build(self) -> FsxDataRepositoryAssociationTimeoutsEl {
        FsxDataRepositoryAssociationTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct FsxDataRepositoryAssociationTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FsxDataRepositoryAssociationTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> FsxDataRepositoryAssociationTimeoutsElRef {
        FsxDataRepositoryAssociationTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FsxDataRepositoryAssociationTimeoutsElRef {
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
struct FsxDataRepositoryAssociationDynamic {
    s3: Option<DynamicBlock<FsxDataRepositoryAssociationS3El>>,
}
