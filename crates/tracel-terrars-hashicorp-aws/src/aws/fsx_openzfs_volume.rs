use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct FsxOpenzfsVolumeData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    copy_tags_to_snapshots: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_compression_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete_volume_options: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    parent_volume_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    read_only: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    record_size_kib: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_capacity_quota_gib: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_capacity_reservation_gib: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volume_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nfs_exports: Option<Vec<FsxOpenzfsVolumeNfsExportsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    origin_snapshot: Option<Vec<FsxOpenzfsVolumeOriginSnapshotEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<FsxOpenzfsVolumeTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_and_group_quotas: Option<Vec<FsxOpenzfsVolumeUserAndGroupQuotasEl>>,
    dynamic: FsxOpenzfsVolumeDynamic,
}

struct FsxOpenzfsVolume_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<FsxOpenzfsVolumeData>,
}

#[derive(Clone)]
pub struct FsxOpenzfsVolume(Rc<FsxOpenzfsVolume_>);

impl FsxOpenzfsVolume {
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
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
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc = "Set the field `copy_tags_to_snapshots`.\n"]
    pub fn set_copy_tags_to_snapshots(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().copy_tags_to_snapshots = Some(v.into());
        self
    }

    #[doc = "Set the field `data_compression_type`.\n"]
    pub fn set_data_compression_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().data_compression_type = Some(v.into());
        self
    }

    #[doc = "Set the field `delete_volume_options`.\n"]
    pub fn set_delete_volume_options(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().delete_volume_options = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `read_only`.\n"]
    pub fn set_read_only(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().read_only = Some(v.into());
        self
    }

    #[doc = "Set the field `record_size_kib`.\n"]
    pub fn set_record_size_kib(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().record_size_kib = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `storage_capacity_quota_gib`.\n"]
    pub fn set_storage_capacity_quota_gib(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().storage_capacity_quota_gib = Some(v.into());
        self
    }

    #[doc = "Set the field `storage_capacity_reservation_gib`.\n"]
    pub fn set_storage_capacity_reservation_gib(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().storage_capacity_reservation_gib = Some(v.into());
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

    #[doc = "Set the field `volume_type`.\n"]
    pub fn set_volume_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().volume_type = Some(v.into());
        self
    }

    #[doc = "Set the field `nfs_exports`.\n"]
    pub fn set_nfs_exports(self, v: impl Into<BlockAssignable<FsxOpenzfsVolumeNfsExportsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().nfs_exports = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.nfs_exports = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `origin_snapshot`.\n"]
    pub fn set_origin_snapshot(self, v: impl Into<BlockAssignable<FsxOpenzfsVolumeOriginSnapshotEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().origin_snapshot = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.origin_snapshot = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<FsxOpenzfsVolumeTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Set the field `user_and_group_quotas`.\n"]
    pub fn set_user_and_group_quotas(self, v: impl Into<BlockAssignable<FsxOpenzfsVolumeUserAndGroupQuotasEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().user_and_group_quotas = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.user_and_group_quotas = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `copy_tags_to_snapshots` after provisioning.\n"]
    pub fn copy_tags_to_snapshots(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.copy_tags_to_snapshots", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_compression_type` after provisioning.\n"]
    pub fn data_compression_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.data_compression_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `delete_volume_options` after provisioning.\n"]
    pub fn delete_volume_options(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.delete_volume_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parent_volume_id` after provisioning.\n"]
    pub fn parent_volume_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.parent_volume_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `read_only` after provisioning.\n"]
    pub fn read_only(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.read_only", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `record_size_kib` after provisioning.\n"]
    pub fn record_size_kib(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.record_size_kib", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_capacity_quota_gib` after provisioning.\n"]
    pub fn storage_capacity_quota_gib(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.storage_capacity_quota_gib", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_capacity_reservation_gib` after provisioning.\n"]
    pub fn storage_capacity_reservation_gib(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.storage_capacity_reservation_gib", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `volume_type` after provisioning.\n"]
    pub fn volume_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.volume_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `nfs_exports` after provisioning.\n"]
    pub fn nfs_exports(&self) -> ListRef<FsxOpenzfsVolumeNfsExportsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.nfs_exports", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `origin_snapshot` after provisioning.\n"]
    pub fn origin_snapshot(&self) -> ListRef<FsxOpenzfsVolumeOriginSnapshotElRef> {
        ListRef::new(self.shared().clone(), format!("{}.origin_snapshot", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FsxOpenzfsVolumeTimeoutsElRef {
        FsxOpenzfsVolumeTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for FsxOpenzfsVolume {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for FsxOpenzfsVolume { }

impl ToListMappable for FsxOpenzfsVolume {
    type O = ListRef<FsxOpenzfsVolumeRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for FsxOpenzfsVolume_ {
    fn extract_resource_type(&self) -> String {
        "aws_fsx_openzfs_volume".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildFsxOpenzfsVolume {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub parent_volume_id: PrimField<String>,
}

impl BuildFsxOpenzfsVolume {
    pub fn build(self, stack: &mut Stack) -> FsxOpenzfsVolume {
        let out = FsxOpenzfsVolume(Rc::new(FsxOpenzfsVolume_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(FsxOpenzfsVolumeData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                copy_tags_to_snapshots: core::default::Default::default(),
                data_compression_type: core::default::Default::default(),
                delete_volume_options: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                parent_volume_id: self.parent_volume_id,
                read_only: core::default::Default::default(),
                record_size_kib: core::default::Default::default(),
                region: core::default::Default::default(),
                storage_capacity_quota_gib: core::default::Default::default(),
                storage_capacity_reservation_gib: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                volume_type: core::default::Default::default(),
                nfs_exports: core::default::Default::default(),
                origin_snapshot: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                user_and_group_quotas: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct FsxOpenzfsVolumeRef {
    shared: StackShared,
    base: String,
}

impl Ref for FsxOpenzfsVolumeRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl FsxOpenzfsVolumeRef {
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

    #[doc = "Get a reference to the value of field `copy_tags_to_snapshots` after provisioning.\n"]
    pub fn copy_tags_to_snapshots(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.copy_tags_to_snapshots", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_compression_type` after provisioning.\n"]
    pub fn data_compression_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.data_compression_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `delete_volume_options` after provisioning.\n"]
    pub fn delete_volume_options(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.delete_volume_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parent_volume_id` after provisioning.\n"]
    pub fn parent_volume_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.parent_volume_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `read_only` after provisioning.\n"]
    pub fn read_only(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.read_only", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `record_size_kib` after provisioning.\n"]
    pub fn record_size_kib(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.record_size_kib", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_capacity_quota_gib` after provisioning.\n"]
    pub fn storage_capacity_quota_gib(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.storage_capacity_quota_gib", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_capacity_reservation_gib` after provisioning.\n"]
    pub fn storage_capacity_reservation_gib(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.storage_capacity_reservation_gib", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `volume_type` after provisioning.\n"]
    pub fn volume_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.volume_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `nfs_exports` after provisioning.\n"]
    pub fn nfs_exports(&self) -> ListRef<FsxOpenzfsVolumeNfsExportsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.nfs_exports", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `origin_snapshot` after provisioning.\n"]
    pub fn origin_snapshot(&self) -> ListRef<FsxOpenzfsVolumeOriginSnapshotElRef> {
        ListRef::new(self.shared().clone(), format!("{}.origin_snapshot", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FsxOpenzfsVolumeTimeoutsElRef {
        FsxOpenzfsVolumeTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct FsxOpenzfsVolumeNfsExportsElClientConfigurationsEl {
    clients: PrimField<String>,
    options: ListField<PrimField<String>>,
}

impl FsxOpenzfsVolumeNfsExportsElClientConfigurationsEl { }

impl ToListMappable for FsxOpenzfsVolumeNfsExportsElClientConfigurationsEl {
    type O = BlockAssignable<FsxOpenzfsVolumeNfsExportsElClientConfigurationsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxOpenzfsVolumeNfsExportsElClientConfigurationsEl {
    #[doc = ""]
    pub clients: PrimField<String>,
    #[doc = ""]
    pub options: ListField<PrimField<String>>,
}

impl BuildFsxOpenzfsVolumeNfsExportsElClientConfigurationsEl {
    pub fn build(self) -> FsxOpenzfsVolumeNfsExportsElClientConfigurationsEl {
        FsxOpenzfsVolumeNfsExportsElClientConfigurationsEl {
            clients: self.clients,
            options: self.options,
        }
    }
}

pub struct FsxOpenzfsVolumeNfsExportsElClientConfigurationsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for FsxOpenzfsVolumeNfsExportsElClientConfigurationsElRef {
    fn new(shared: StackShared, base: String) -> FsxOpenzfsVolumeNfsExportsElClientConfigurationsElRef {
        FsxOpenzfsVolumeNfsExportsElClientConfigurationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxOpenzfsVolumeNfsExportsElClientConfigurationsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `clients` after provisioning.\n"]
    pub fn clients(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.clients", self.base))
    }

    #[doc = "Get a reference to the value of field `options` after provisioning.\n"]
    pub fn options(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.options", self.base))
    }
}

#[derive(Serialize, Default)]
struct FsxOpenzfsVolumeNfsExportsElDynamic {
    client_configurations: Option<DynamicBlock<FsxOpenzfsVolumeNfsExportsElClientConfigurationsEl>>,
}

#[derive(Serialize)]
pub struct FsxOpenzfsVolumeNfsExportsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    client_configurations: Option<Vec<FsxOpenzfsVolumeNfsExportsElClientConfigurationsEl>>,
    dynamic: FsxOpenzfsVolumeNfsExportsElDynamic,
}

impl FsxOpenzfsVolumeNfsExportsEl {
    #[doc = "Set the field `client_configurations`.\n"]
    pub fn set_client_configurations(
        mut self,
        v: impl Into<BlockAssignable<FsxOpenzfsVolumeNfsExportsElClientConfigurationsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.client_configurations = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.client_configurations = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for FsxOpenzfsVolumeNfsExportsEl {
    type O = BlockAssignable<FsxOpenzfsVolumeNfsExportsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxOpenzfsVolumeNfsExportsEl {}

impl BuildFsxOpenzfsVolumeNfsExportsEl {
    pub fn build(self) -> FsxOpenzfsVolumeNfsExportsEl {
        FsxOpenzfsVolumeNfsExportsEl {
            client_configurations: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct FsxOpenzfsVolumeNfsExportsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for FsxOpenzfsVolumeNfsExportsElRef {
    fn new(shared: StackShared, base: String) -> FsxOpenzfsVolumeNfsExportsElRef {
        FsxOpenzfsVolumeNfsExportsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxOpenzfsVolumeNfsExportsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct FsxOpenzfsVolumeOriginSnapshotEl {
    copy_strategy: PrimField<String>,
    snapshot_arn: PrimField<String>,
}

impl FsxOpenzfsVolumeOriginSnapshotEl { }

impl ToListMappable for FsxOpenzfsVolumeOriginSnapshotEl {
    type O = BlockAssignable<FsxOpenzfsVolumeOriginSnapshotEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxOpenzfsVolumeOriginSnapshotEl {
    #[doc = ""]
    pub copy_strategy: PrimField<String>,
    #[doc = ""]
    pub snapshot_arn: PrimField<String>,
}

impl BuildFsxOpenzfsVolumeOriginSnapshotEl {
    pub fn build(self) -> FsxOpenzfsVolumeOriginSnapshotEl {
        FsxOpenzfsVolumeOriginSnapshotEl {
            copy_strategy: self.copy_strategy,
            snapshot_arn: self.snapshot_arn,
        }
    }
}

pub struct FsxOpenzfsVolumeOriginSnapshotElRef {
    shared: StackShared,
    base: String,
}

impl Ref for FsxOpenzfsVolumeOriginSnapshotElRef {
    fn new(shared: StackShared, base: String) -> FsxOpenzfsVolumeOriginSnapshotElRef {
        FsxOpenzfsVolumeOriginSnapshotElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxOpenzfsVolumeOriginSnapshotElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `copy_strategy` after provisioning.\n"]
    pub fn copy_strategy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.copy_strategy", self.base))
    }

    #[doc = "Get a reference to the value of field `snapshot_arn` after provisioning.\n"]
    pub fn snapshot_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.snapshot_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct FsxOpenzfsVolumeTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl FsxOpenzfsVolumeTimeoutsEl {
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

impl ToListMappable for FsxOpenzfsVolumeTimeoutsEl {
    type O = BlockAssignable<FsxOpenzfsVolumeTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxOpenzfsVolumeTimeoutsEl {}

impl BuildFsxOpenzfsVolumeTimeoutsEl {
    pub fn build(self) -> FsxOpenzfsVolumeTimeoutsEl {
        FsxOpenzfsVolumeTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct FsxOpenzfsVolumeTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for FsxOpenzfsVolumeTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> FsxOpenzfsVolumeTimeoutsElRef {
        FsxOpenzfsVolumeTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxOpenzfsVolumeTimeoutsElRef {
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

#[derive(Serialize)]
pub struct FsxOpenzfsVolumeUserAndGroupQuotasEl {
    id: PrimField<f64>,
    storage_capacity_quota_gib: PrimField<f64>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}

impl FsxOpenzfsVolumeUserAndGroupQuotasEl { }

impl ToListMappable for FsxOpenzfsVolumeUserAndGroupQuotasEl {
    type O = BlockAssignable<FsxOpenzfsVolumeUserAndGroupQuotasEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxOpenzfsVolumeUserAndGroupQuotasEl {
    #[doc = ""]
    pub id: PrimField<f64>,
    #[doc = ""]
    pub storage_capacity_quota_gib: PrimField<f64>,
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl BuildFsxOpenzfsVolumeUserAndGroupQuotasEl {
    pub fn build(self) -> FsxOpenzfsVolumeUserAndGroupQuotasEl {
        FsxOpenzfsVolumeUserAndGroupQuotasEl {
            id: self.id,
            storage_capacity_quota_gib: self.storage_capacity_quota_gib,
            type_: self.type_,
        }
    }
}

pub struct FsxOpenzfsVolumeUserAndGroupQuotasElRef {
    shared: StackShared,
    base: String,
}

impl Ref for FsxOpenzfsVolumeUserAndGroupQuotasElRef {
    fn new(shared: StackShared, base: String) -> FsxOpenzfsVolumeUserAndGroupQuotasElRef {
        FsxOpenzfsVolumeUserAndGroupQuotasElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxOpenzfsVolumeUserAndGroupQuotasElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `storage_capacity_quota_gib` after provisioning.\n"]
    pub fn storage_capacity_quota_gib(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.storage_capacity_quota_gib", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct FsxOpenzfsVolumeDynamic {
    nfs_exports: Option<DynamicBlock<FsxOpenzfsVolumeNfsExportsEl>>,
    origin_snapshot: Option<DynamicBlock<FsxOpenzfsVolumeOriginSnapshotEl>>,
    user_and_group_quotas: Option<DynamicBlock<FsxOpenzfsVolumeUserAndGroupQuotasEl>>,
}
