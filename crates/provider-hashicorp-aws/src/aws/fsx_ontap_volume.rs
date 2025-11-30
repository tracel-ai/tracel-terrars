use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct FsxOntapVolumeData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bypass_snaplock_enterprise_retention: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    copy_tags_to_backups: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    final_backup_tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    junction_path: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ontap_volume_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_style: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    size_in_bytes: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    size_in_megabytes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    skip_final_backup: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    snapshot_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_efficiency_enabled: Option<PrimField<bool>>,
    storage_virtual_machine_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volume_style: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volume_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    aggregate_configuration: Option<Vec<FsxOntapVolumeAggregateConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    snaplock_configuration: Option<Vec<FsxOntapVolumeSnaplockConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tiering_policy: Option<Vec<FsxOntapVolumeTieringPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<FsxOntapVolumeTimeoutsEl>,
    dynamic: FsxOntapVolumeDynamic,
}

struct FsxOntapVolume_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<FsxOntapVolumeData>,
}

#[derive(Clone)]
pub struct FsxOntapVolume(Rc<FsxOntapVolume_>);

impl FsxOntapVolume {
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

    #[doc = "Set the field `bypass_snaplock_enterprise_retention`.\n"]
    pub fn set_bypass_snaplock_enterprise_retention(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0
            .data
            .borrow_mut()
            .bypass_snaplock_enterprise_retention = Some(v.into());
        self
    }

    #[doc = "Set the field `copy_tags_to_backups`.\n"]
    pub fn set_copy_tags_to_backups(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().copy_tags_to_backups = Some(v.into());
        self
    }

    #[doc = "Set the field `final_backup_tags`.\n"]
    pub fn set_final_backup_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().final_backup_tags = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `junction_path`.\n"]
    pub fn set_junction_path(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().junction_path = Some(v.into());
        self
    }

    #[doc = "Set the field `ontap_volume_type`.\n"]
    pub fn set_ontap_volume_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().ontap_volume_type = Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `security_style`.\n"]
    pub fn set_security_style(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().security_style = Some(v.into());
        self
    }

    #[doc = "Set the field `size_in_bytes`.\n"]
    pub fn set_size_in_bytes(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().size_in_bytes = Some(v.into());
        self
    }

    #[doc = "Set the field `size_in_megabytes`.\n"]
    pub fn set_size_in_megabytes(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().size_in_megabytes = Some(v.into());
        self
    }

    #[doc = "Set the field `skip_final_backup`.\n"]
    pub fn set_skip_final_backup(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().skip_final_backup = Some(v.into());
        self
    }

    #[doc = "Set the field `snapshot_policy`.\n"]
    pub fn set_snapshot_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().snapshot_policy = Some(v.into());
        self
    }

    #[doc = "Set the field `storage_efficiency_enabled`.\n"]
    pub fn set_storage_efficiency_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().storage_efficiency_enabled = Some(v.into());
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

    #[doc = "Set the field `volume_style`.\n"]
    pub fn set_volume_style(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().volume_style = Some(v.into());
        self
    }

    #[doc = "Set the field `volume_type`.\n"]
    pub fn set_volume_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().volume_type = Some(v.into());
        self
    }

    #[doc = "Set the field `aggregate_configuration`.\n"]
    pub fn set_aggregate_configuration(
        self,
        v: impl Into<BlockAssignable<FsxOntapVolumeAggregateConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().aggregate_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.aggregate_configuration = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `snaplock_configuration`.\n"]
    pub fn set_snaplock_configuration(
        self,
        v: impl Into<BlockAssignable<FsxOntapVolumeSnaplockConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().snaplock_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.snaplock_configuration = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `tiering_policy`.\n"]
    pub fn set_tiering_policy(
        self,
        v: impl Into<BlockAssignable<FsxOntapVolumeTieringPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().tiering_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.tiering_policy = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<FsxOntapVolumeTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bypass_snaplock_enterprise_retention` after provisioning.\n"]
    pub fn bypass_snaplock_enterprise_retention(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.bypass_snaplock_enterprise_retention",
                self.extract_ref()
            ),
        )
    }

    #[doc = "Get a reference to the value of field `copy_tags_to_backups` after provisioning.\n"]
    pub fn copy_tags_to_backups(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.copy_tags_to_backups", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `file_system_id` after provisioning.\n"]
    pub fn file_system_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.file_system_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `final_backup_tags` after provisioning.\n"]
    pub fn final_backup_tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.final_backup_tags", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `flexcache_endpoint_type` after provisioning.\n"]
    pub fn flexcache_endpoint_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.flexcache_endpoint_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `junction_path` after provisioning.\n"]
    pub fn junction_path(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.junction_path", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `ontap_volume_type` after provisioning.\n"]
    pub fn ontap_volume_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ontap_volume_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `security_style` after provisioning.\n"]
    pub fn security_style(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.security_style", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `size_in_bytes` after provisioning.\n"]
    pub fn size_in_bytes(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.size_in_bytes", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `size_in_megabytes` after provisioning.\n"]
    pub fn size_in_megabytes(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.size_in_megabytes", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `skip_final_backup` after provisioning.\n"]
    pub fn skip_final_backup(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.skip_final_backup", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `snapshot_policy` after provisioning.\n"]
    pub fn snapshot_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.snapshot_policy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `storage_efficiency_enabled` after provisioning.\n"]
    pub fn storage_efficiency_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.storage_efficiency_enabled", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `storage_virtual_machine_id` after provisioning.\n"]
    pub fn storage_virtual_machine_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.storage_virtual_machine_id", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `uuid` after provisioning.\n"]
    pub fn uuid(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.uuid", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `volume_style` after provisioning.\n"]
    pub fn volume_style(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.volume_style", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `volume_type` after provisioning.\n"]
    pub fn volume_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.volume_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `aggregate_configuration` after provisioning.\n"]
    pub fn aggregate_configuration(&self) -> ListRef<FsxOntapVolumeAggregateConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.aggregate_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `snaplock_configuration` after provisioning.\n"]
    pub fn snaplock_configuration(&self) -> ListRef<FsxOntapVolumeSnaplockConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.snaplock_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tiering_policy` after provisioning.\n"]
    pub fn tiering_policy(&self) -> ListRef<FsxOntapVolumeTieringPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tiering_policy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FsxOntapVolumeTimeoutsElRef {
        FsxOntapVolumeTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for FsxOntapVolume {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for FsxOntapVolume {}

impl ToListMappable for FsxOntapVolume {
    type O = ListRef<FsxOntapVolumeRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for FsxOntapVolume_ {
    fn extract_resource_type(&self) -> String {
        "aws_fsx_ontap_volume".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildFsxOntapVolume {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub storage_virtual_machine_id: PrimField<String>,
}

impl BuildFsxOntapVolume {
    pub fn build(self, stack: &mut Stack) -> FsxOntapVolume {
        let out = FsxOntapVolume(Rc::new(FsxOntapVolume_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(FsxOntapVolumeData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                bypass_snaplock_enterprise_retention: core::default::Default::default(),
                copy_tags_to_backups: core::default::Default::default(),
                final_backup_tags: core::default::Default::default(),
                id: core::default::Default::default(),
                junction_path: core::default::Default::default(),
                name: self.name,
                ontap_volume_type: core::default::Default::default(),
                region: core::default::Default::default(),
                security_style: core::default::Default::default(),
                size_in_bytes: core::default::Default::default(),
                size_in_megabytes: core::default::Default::default(),
                skip_final_backup: core::default::Default::default(),
                snapshot_policy: core::default::Default::default(),
                storage_efficiency_enabled: core::default::Default::default(),
                storage_virtual_machine_id: self.storage_virtual_machine_id,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                volume_style: core::default::Default::default(),
                volume_type: core::default::Default::default(),
                aggregate_configuration: core::default::Default::default(),
                snaplock_configuration: core::default::Default::default(),
                tiering_policy: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct FsxOntapVolumeRef {
    shared: StackShared,
    base: String,
}

impl Ref for FsxOntapVolumeRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl FsxOntapVolumeRef {
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

    #[doc = "Get a reference to the value of field `bypass_snaplock_enterprise_retention` after provisioning.\n"]
    pub fn bypass_snaplock_enterprise_retention(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.bypass_snaplock_enterprise_retention",
                self.extract_ref()
            ),
        )
    }

    #[doc = "Get a reference to the value of field `copy_tags_to_backups` after provisioning.\n"]
    pub fn copy_tags_to_backups(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.copy_tags_to_backups", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `file_system_id` after provisioning.\n"]
    pub fn file_system_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.file_system_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `final_backup_tags` after provisioning.\n"]
    pub fn final_backup_tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.final_backup_tags", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `flexcache_endpoint_type` after provisioning.\n"]
    pub fn flexcache_endpoint_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.flexcache_endpoint_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `junction_path` after provisioning.\n"]
    pub fn junction_path(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.junction_path", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `ontap_volume_type` after provisioning.\n"]
    pub fn ontap_volume_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ontap_volume_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `security_style` after provisioning.\n"]
    pub fn security_style(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.security_style", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `size_in_bytes` after provisioning.\n"]
    pub fn size_in_bytes(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.size_in_bytes", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `size_in_megabytes` after provisioning.\n"]
    pub fn size_in_megabytes(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.size_in_megabytes", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `skip_final_backup` after provisioning.\n"]
    pub fn skip_final_backup(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.skip_final_backup", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `snapshot_policy` after provisioning.\n"]
    pub fn snapshot_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.snapshot_policy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `storage_efficiency_enabled` after provisioning.\n"]
    pub fn storage_efficiency_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.storage_efficiency_enabled", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `storage_virtual_machine_id` after provisioning.\n"]
    pub fn storage_virtual_machine_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.storage_virtual_machine_id", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `uuid` after provisioning.\n"]
    pub fn uuid(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.uuid", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `volume_style` after provisioning.\n"]
    pub fn volume_style(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.volume_style", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `volume_type` after provisioning.\n"]
    pub fn volume_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.volume_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `aggregate_configuration` after provisioning.\n"]
    pub fn aggregate_configuration(&self) -> ListRef<FsxOntapVolumeAggregateConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.aggregate_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `snaplock_configuration` after provisioning.\n"]
    pub fn snaplock_configuration(&self) -> ListRef<FsxOntapVolumeSnaplockConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.snaplock_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tiering_policy` after provisioning.\n"]
    pub fn tiering_policy(&self) -> ListRef<FsxOntapVolumeTieringPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tiering_policy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FsxOntapVolumeTimeoutsElRef {
        FsxOntapVolumeTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct FsxOntapVolumeAggregateConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    aggregates: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    constituents_per_aggregate: Option<PrimField<f64>>,
}

impl FsxOntapVolumeAggregateConfigurationEl {
    #[doc = "Set the field `aggregates`.\n"]
    pub fn set_aggregates(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.aggregates = Some(v.into());
        self
    }

    #[doc = "Set the field `constituents_per_aggregate`.\n"]
    pub fn set_constituents_per_aggregate(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.constituents_per_aggregate = Some(v.into());
        self
    }
}

impl ToListMappable for FsxOntapVolumeAggregateConfigurationEl {
    type O = BlockAssignable<FsxOntapVolumeAggregateConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxOntapVolumeAggregateConfigurationEl {}

impl BuildFsxOntapVolumeAggregateConfigurationEl {
    pub fn build(self) -> FsxOntapVolumeAggregateConfigurationEl {
        FsxOntapVolumeAggregateConfigurationEl {
            aggregates: core::default::Default::default(),
            constituents_per_aggregate: core::default::Default::default(),
        }
    }
}

pub struct FsxOntapVolumeAggregateConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for FsxOntapVolumeAggregateConfigurationElRef {
    fn new(shared: StackShared, base: String) -> FsxOntapVolumeAggregateConfigurationElRef {
        FsxOntapVolumeAggregateConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxOntapVolumeAggregateConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `aggregates` after provisioning.\n"]
    pub fn aggregates(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.aggregates", self.base))
    }

    #[doc = "Get a reference to the value of field `constituents_per_aggregate` after provisioning.\n"]
    pub fn constituents_per_aggregate(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.constituents_per_aggregate", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `total_constituents` after provisioning.\n"]
    pub fn total_constituents(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.total_constituents", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct FsxOntapVolumeSnaplockConfigurationElAutocommitPeriodEl {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<f64>>,
}

impl FsxOntapVolumeSnaplockConfigurationElAutocommitPeriodEl {
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.value = Some(v.into());
        self
    }
}

impl ToListMappable for FsxOntapVolumeSnaplockConfigurationElAutocommitPeriodEl {
    type O = BlockAssignable<FsxOntapVolumeSnaplockConfigurationElAutocommitPeriodEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxOntapVolumeSnaplockConfigurationElAutocommitPeriodEl {}

impl BuildFsxOntapVolumeSnaplockConfigurationElAutocommitPeriodEl {
    pub fn build(self) -> FsxOntapVolumeSnaplockConfigurationElAutocommitPeriodEl {
        FsxOntapVolumeSnaplockConfigurationElAutocommitPeriodEl {
            type_: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct FsxOntapVolumeSnaplockConfigurationElAutocommitPeriodElRef {
    shared: StackShared,
    base: String,
}

impl Ref for FsxOntapVolumeSnaplockConfigurationElAutocommitPeriodElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> FsxOntapVolumeSnaplockConfigurationElAutocommitPeriodElRef {
        FsxOntapVolumeSnaplockConfigurationElAutocommitPeriodElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxOntapVolumeSnaplockConfigurationElAutocommitPeriodElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct FsxOntapVolumeSnaplockConfigurationElRetentionPeriodElDefaultRetentionEl {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<f64>>,
}

impl FsxOntapVolumeSnaplockConfigurationElRetentionPeriodElDefaultRetentionEl {
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.value = Some(v.into());
        self
    }
}

impl ToListMappable for FsxOntapVolumeSnaplockConfigurationElRetentionPeriodElDefaultRetentionEl {
    type O =
        BlockAssignable<FsxOntapVolumeSnaplockConfigurationElRetentionPeriodElDefaultRetentionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxOntapVolumeSnaplockConfigurationElRetentionPeriodElDefaultRetentionEl {}

impl BuildFsxOntapVolumeSnaplockConfigurationElRetentionPeriodElDefaultRetentionEl {
    pub fn build(self) -> FsxOntapVolumeSnaplockConfigurationElRetentionPeriodElDefaultRetentionEl {
        FsxOntapVolumeSnaplockConfigurationElRetentionPeriodElDefaultRetentionEl {
            type_: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct FsxOntapVolumeSnaplockConfigurationElRetentionPeriodElDefaultRetentionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for FsxOntapVolumeSnaplockConfigurationElRetentionPeriodElDefaultRetentionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> FsxOntapVolumeSnaplockConfigurationElRetentionPeriodElDefaultRetentionElRef {
        FsxOntapVolumeSnaplockConfigurationElRetentionPeriodElDefaultRetentionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxOntapVolumeSnaplockConfigurationElRetentionPeriodElDefaultRetentionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct FsxOntapVolumeSnaplockConfigurationElRetentionPeriodElMaximumRetentionEl {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<f64>>,
}

impl FsxOntapVolumeSnaplockConfigurationElRetentionPeriodElMaximumRetentionEl {
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.value = Some(v.into());
        self
    }
}

impl ToListMappable for FsxOntapVolumeSnaplockConfigurationElRetentionPeriodElMaximumRetentionEl {
    type O =
        BlockAssignable<FsxOntapVolumeSnaplockConfigurationElRetentionPeriodElMaximumRetentionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxOntapVolumeSnaplockConfigurationElRetentionPeriodElMaximumRetentionEl {}

impl BuildFsxOntapVolumeSnaplockConfigurationElRetentionPeriodElMaximumRetentionEl {
    pub fn build(self) -> FsxOntapVolumeSnaplockConfigurationElRetentionPeriodElMaximumRetentionEl {
        FsxOntapVolumeSnaplockConfigurationElRetentionPeriodElMaximumRetentionEl {
            type_: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct FsxOntapVolumeSnaplockConfigurationElRetentionPeriodElMaximumRetentionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for FsxOntapVolumeSnaplockConfigurationElRetentionPeriodElMaximumRetentionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> FsxOntapVolumeSnaplockConfigurationElRetentionPeriodElMaximumRetentionElRef {
        FsxOntapVolumeSnaplockConfigurationElRetentionPeriodElMaximumRetentionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxOntapVolumeSnaplockConfigurationElRetentionPeriodElMaximumRetentionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct FsxOntapVolumeSnaplockConfigurationElRetentionPeriodElMinimumRetentionEl {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<f64>>,
}

impl FsxOntapVolumeSnaplockConfigurationElRetentionPeriodElMinimumRetentionEl {
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.value = Some(v.into());
        self
    }
}

impl ToListMappable for FsxOntapVolumeSnaplockConfigurationElRetentionPeriodElMinimumRetentionEl {
    type O =
        BlockAssignable<FsxOntapVolumeSnaplockConfigurationElRetentionPeriodElMinimumRetentionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxOntapVolumeSnaplockConfigurationElRetentionPeriodElMinimumRetentionEl {}

impl BuildFsxOntapVolumeSnaplockConfigurationElRetentionPeriodElMinimumRetentionEl {
    pub fn build(self) -> FsxOntapVolumeSnaplockConfigurationElRetentionPeriodElMinimumRetentionEl {
        FsxOntapVolumeSnaplockConfigurationElRetentionPeriodElMinimumRetentionEl {
            type_: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct FsxOntapVolumeSnaplockConfigurationElRetentionPeriodElMinimumRetentionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for FsxOntapVolumeSnaplockConfigurationElRetentionPeriodElMinimumRetentionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> FsxOntapVolumeSnaplockConfigurationElRetentionPeriodElMinimumRetentionElRef {
        FsxOntapVolumeSnaplockConfigurationElRetentionPeriodElMinimumRetentionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxOntapVolumeSnaplockConfigurationElRetentionPeriodElMinimumRetentionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct FsxOntapVolumeSnaplockConfigurationElRetentionPeriodElDynamic {
    default_retention: Option<
        DynamicBlock<FsxOntapVolumeSnaplockConfigurationElRetentionPeriodElDefaultRetentionEl>,
    >,
    maximum_retention: Option<
        DynamicBlock<FsxOntapVolumeSnaplockConfigurationElRetentionPeriodElMaximumRetentionEl>,
    >,
    minimum_retention: Option<
        DynamicBlock<FsxOntapVolumeSnaplockConfigurationElRetentionPeriodElMinimumRetentionEl>,
    >,
}

#[derive(Serialize)]
pub struct FsxOntapVolumeSnaplockConfigurationElRetentionPeriodEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    default_retention:
        Option<Vec<FsxOntapVolumeSnaplockConfigurationElRetentionPeriodElDefaultRetentionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maximum_retention:
        Option<Vec<FsxOntapVolumeSnaplockConfigurationElRetentionPeriodElMaximumRetentionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minimum_retention:
        Option<Vec<FsxOntapVolumeSnaplockConfigurationElRetentionPeriodElMinimumRetentionEl>>,
    dynamic: FsxOntapVolumeSnaplockConfigurationElRetentionPeriodElDynamic,
}

impl FsxOntapVolumeSnaplockConfigurationElRetentionPeriodEl {
    #[doc = "Set the field `default_retention`.\n"]
    pub fn set_default_retention(
        mut self,
        v: impl Into<
            BlockAssignable<
                FsxOntapVolumeSnaplockConfigurationElRetentionPeriodElDefaultRetentionEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.default_retention = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.default_retention = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `maximum_retention`.\n"]
    pub fn set_maximum_retention(
        mut self,
        v: impl Into<
            BlockAssignable<
                FsxOntapVolumeSnaplockConfigurationElRetentionPeriodElMaximumRetentionEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.maximum_retention = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.maximum_retention = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `minimum_retention`.\n"]
    pub fn set_minimum_retention(
        mut self,
        v: impl Into<
            BlockAssignable<
                FsxOntapVolumeSnaplockConfigurationElRetentionPeriodElMinimumRetentionEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.minimum_retention = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.minimum_retention = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for FsxOntapVolumeSnaplockConfigurationElRetentionPeriodEl {
    type O = BlockAssignable<FsxOntapVolumeSnaplockConfigurationElRetentionPeriodEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxOntapVolumeSnaplockConfigurationElRetentionPeriodEl {}

impl BuildFsxOntapVolumeSnaplockConfigurationElRetentionPeriodEl {
    pub fn build(self) -> FsxOntapVolumeSnaplockConfigurationElRetentionPeriodEl {
        FsxOntapVolumeSnaplockConfigurationElRetentionPeriodEl {
            default_retention: core::default::Default::default(),
            maximum_retention: core::default::Default::default(),
            minimum_retention: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct FsxOntapVolumeSnaplockConfigurationElRetentionPeriodElRef {
    shared: StackShared,
    base: String,
}

impl Ref for FsxOntapVolumeSnaplockConfigurationElRetentionPeriodElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> FsxOntapVolumeSnaplockConfigurationElRetentionPeriodElRef {
        FsxOntapVolumeSnaplockConfigurationElRetentionPeriodElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxOntapVolumeSnaplockConfigurationElRetentionPeriodElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `default_retention` after provisioning.\n"]
    pub fn default_retention(
        &self,
    ) -> ListRef<FsxOntapVolumeSnaplockConfigurationElRetentionPeriodElDefaultRetentionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.default_retention", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `maximum_retention` after provisioning.\n"]
    pub fn maximum_retention(
        &self,
    ) -> ListRef<FsxOntapVolumeSnaplockConfigurationElRetentionPeriodElMaximumRetentionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.maximum_retention", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `minimum_retention` after provisioning.\n"]
    pub fn minimum_retention(
        &self,
    ) -> ListRef<FsxOntapVolumeSnaplockConfigurationElRetentionPeriodElMinimumRetentionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.minimum_retention", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct FsxOntapVolumeSnaplockConfigurationElDynamic {
    autocommit_period:
        Option<DynamicBlock<FsxOntapVolumeSnaplockConfigurationElAutocommitPeriodEl>>,
    retention_period: Option<DynamicBlock<FsxOntapVolumeSnaplockConfigurationElRetentionPeriodEl>>,
}

#[derive(Serialize)]
pub struct FsxOntapVolumeSnaplockConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    audit_log_volume: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    privileged_delete: Option<PrimField<String>>,
    snaplock_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volume_append_mode_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    autocommit_period: Option<Vec<FsxOntapVolumeSnaplockConfigurationElAutocommitPeriodEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retention_period: Option<Vec<FsxOntapVolumeSnaplockConfigurationElRetentionPeriodEl>>,
    dynamic: FsxOntapVolumeSnaplockConfigurationElDynamic,
}

impl FsxOntapVolumeSnaplockConfigurationEl {
    #[doc = "Set the field `audit_log_volume`.\n"]
    pub fn set_audit_log_volume(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.audit_log_volume = Some(v.into());
        self
    }

    #[doc = "Set the field `privileged_delete`.\n"]
    pub fn set_privileged_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.privileged_delete = Some(v.into());
        self
    }

    #[doc = "Set the field `volume_append_mode_enabled`.\n"]
    pub fn set_volume_append_mode_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.volume_append_mode_enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `autocommit_period`.\n"]
    pub fn set_autocommit_period(
        mut self,
        v: impl Into<BlockAssignable<FsxOntapVolumeSnaplockConfigurationElAutocommitPeriodEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.autocommit_period = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.autocommit_period = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `retention_period`.\n"]
    pub fn set_retention_period(
        mut self,
        v: impl Into<BlockAssignable<FsxOntapVolumeSnaplockConfigurationElRetentionPeriodEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.retention_period = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.retention_period = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for FsxOntapVolumeSnaplockConfigurationEl {
    type O = BlockAssignable<FsxOntapVolumeSnaplockConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxOntapVolumeSnaplockConfigurationEl {
    #[doc = ""]
    pub snaplock_type: PrimField<String>,
}

impl BuildFsxOntapVolumeSnaplockConfigurationEl {
    pub fn build(self) -> FsxOntapVolumeSnaplockConfigurationEl {
        FsxOntapVolumeSnaplockConfigurationEl {
            audit_log_volume: core::default::Default::default(),
            privileged_delete: core::default::Default::default(),
            snaplock_type: self.snaplock_type,
            volume_append_mode_enabled: core::default::Default::default(),
            autocommit_period: core::default::Default::default(),
            retention_period: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct FsxOntapVolumeSnaplockConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for FsxOntapVolumeSnaplockConfigurationElRef {
    fn new(shared: StackShared, base: String) -> FsxOntapVolumeSnaplockConfigurationElRef {
        FsxOntapVolumeSnaplockConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxOntapVolumeSnaplockConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `audit_log_volume` after provisioning.\n"]
    pub fn audit_log_volume(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.audit_log_volume", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `privileged_delete` after provisioning.\n"]
    pub fn privileged_delete(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.privileged_delete", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `snaplock_type` after provisioning.\n"]
    pub fn snaplock_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.snaplock_type", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `volume_append_mode_enabled` after provisioning.\n"]
    pub fn volume_append_mode_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.volume_append_mode_enabled", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `autocommit_period` after provisioning.\n"]
    pub fn autocommit_period(
        &self,
    ) -> ListRef<FsxOntapVolumeSnaplockConfigurationElAutocommitPeriodElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.autocommit_period", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `retention_period` after provisioning.\n"]
    pub fn retention_period(
        &self,
    ) -> ListRef<FsxOntapVolumeSnaplockConfigurationElRetentionPeriodElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.retention_period", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct FsxOntapVolumeTieringPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cooling_period: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl FsxOntapVolumeTieringPolicyEl {
    #[doc = "Set the field `cooling_period`.\n"]
    pub fn set_cooling_period(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.cooling_period = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}

impl ToListMappable for FsxOntapVolumeTieringPolicyEl {
    type O = BlockAssignable<FsxOntapVolumeTieringPolicyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxOntapVolumeTieringPolicyEl {}

impl BuildFsxOntapVolumeTieringPolicyEl {
    pub fn build(self) -> FsxOntapVolumeTieringPolicyEl {
        FsxOntapVolumeTieringPolicyEl {
            cooling_period: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct FsxOntapVolumeTieringPolicyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for FsxOntapVolumeTieringPolicyElRef {
    fn new(shared: StackShared, base: String) -> FsxOntapVolumeTieringPolicyElRef {
        FsxOntapVolumeTieringPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxOntapVolumeTieringPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cooling_period` after provisioning.\n"]
    pub fn cooling_period(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cooling_period", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct FsxOntapVolumeTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl FsxOntapVolumeTimeoutsEl {
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

impl ToListMappable for FsxOntapVolumeTimeoutsEl {
    type O = BlockAssignable<FsxOntapVolumeTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxOntapVolumeTimeoutsEl {}

impl BuildFsxOntapVolumeTimeoutsEl {
    pub fn build(self) -> FsxOntapVolumeTimeoutsEl {
        FsxOntapVolumeTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct FsxOntapVolumeTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for FsxOntapVolumeTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> FsxOntapVolumeTimeoutsElRef {
        FsxOntapVolumeTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxOntapVolumeTimeoutsElRef {
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
struct FsxOntapVolumeDynamic {
    aggregate_configuration: Option<DynamicBlock<FsxOntapVolumeAggregateConfigurationEl>>,
    snaplock_configuration: Option<DynamicBlock<FsxOntapVolumeSnaplockConfigurationEl>>,
    tiering_policy: Option<DynamicBlock<FsxOntapVolumeTieringPolicyEl>>,
}
