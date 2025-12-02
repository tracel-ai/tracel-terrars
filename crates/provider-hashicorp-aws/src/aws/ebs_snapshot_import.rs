use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct EbsSnapshotImportData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encrypted: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    permanent_restore: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    role_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_tier: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temporary_restore_days: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_data: Option<Vec<EbsSnapshotImportClientDataEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_container: Option<Vec<EbsSnapshotImportDiskContainerEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<EbsSnapshotImportTimeoutsEl>,
    dynamic: EbsSnapshotImportDynamic,
}
struct EbsSnapshotImport_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<EbsSnapshotImportData>,
}
#[derive(Clone)]
pub struct EbsSnapshotImport(Rc<EbsSnapshotImport_>);
impl EbsSnapshotImport {
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
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `encrypted`.\n"]
    pub fn set_encrypted(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().encrypted = Some(v.into());
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
    #[doc = "Set the field `permanent_restore`.\n"]
    pub fn set_permanent_restore(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().permanent_restore = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `role_name`.\n"]
    pub fn set_role_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().role_name = Some(v.into());
        self
    }
    #[doc = "Set the field `storage_tier`.\n"]
    pub fn set_storage_tier(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().storage_tier = Some(v.into());
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
    #[doc = "Set the field `temporary_restore_days`.\n"]
    pub fn set_temporary_restore_days(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().temporary_restore_days = Some(v.into());
        self
    }
    #[doc = "Set the field `client_data`.\n"]
    pub fn set_client_data(
        self,
        v: impl Into<BlockAssignable<EbsSnapshotImportClientDataEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().client_data = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.client_data = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `disk_container`.\n"]
    pub fn set_disk_container(
        self,
        v: impl Into<BlockAssignable<EbsSnapshotImportDiskContainerEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().disk_container = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.disk_container = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<EbsSnapshotImportTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `data_encryption_key_id` after provisioning.\n"]
    pub fn data_encryption_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_encryption_key_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `encrypted` after provisioning.\n"]
    pub fn encrypted(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.encrypted", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `outpost_arn` after provisioning.\n"]
    pub fn outpost_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.outpost_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `owner_alias` after provisioning.\n"]
    pub fn owner_alias(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.owner_alias", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.owner_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `permanent_restore` after provisioning.\n"]
    pub fn permanent_restore(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.permanent_restore", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `role_name` after provisioning.\n"]
    pub fn role_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.role_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `storage_tier` after provisioning.\n"]
    pub fn storage_tier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.storage_tier", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `temporary_restore_days` after provisioning.\n"]
    pub fn temporary_restore_days(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.temporary_restore_days", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `volume_id` after provisioning.\n"]
    pub fn volume_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.volume_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `volume_size` after provisioning.\n"]
    pub fn volume_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.volume_size", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `client_data` after provisioning.\n"]
    pub fn client_data(&self) -> ListRef<EbsSnapshotImportClientDataElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.client_data", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `disk_container` after provisioning.\n"]
    pub fn disk_container(&self) -> ListRef<EbsSnapshotImportDiskContainerElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.disk_container", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> EbsSnapshotImportTimeoutsElRef {
        EbsSnapshotImportTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for EbsSnapshotImport {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for EbsSnapshotImport {}
impl ToListMappable for EbsSnapshotImport {
    type O = ListRef<EbsSnapshotImportRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for EbsSnapshotImport_ {
    fn extract_resource_type(&self) -> String {
        "aws_ebs_snapshot_import".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildEbsSnapshotImport {
    pub tf_id: String,
}
impl BuildEbsSnapshotImport {
    pub fn build(self, stack: &mut Stack) -> EbsSnapshotImport {
        let out = EbsSnapshotImport(Rc::new(EbsSnapshotImport_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(EbsSnapshotImportData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                description: core::default::Default::default(),
                encrypted: core::default::Default::default(),
                id: core::default::Default::default(),
                kms_key_id: core::default::Default::default(),
                permanent_restore: core::default::Default::default(),
                region: core::default::Default::default(),
                role_name: core::default::Default::default(),
                storage_tier: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                temporary_restore_days: core::default::Default::default(),
                client_data: core::default::Default::default(),
                disk_container: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct EbsSnapshotImportRef {
    shared: StackShared,
    base: String,
}
impl Ref for EbsSnapshotImportRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl EbsSnapshotImportRef {
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
    #[doc = "Get a reference to the value of field `data_encryption_key_id` after provisioning.\n"]
    pub fn data_encryption_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_encryption_key_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `encrypted` after provisioning.\n"]
    pub fn encrypted(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.encrypted", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `outpost_arn` after provisioning.\n"]
    pub fn outpost_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.outpost_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `owner_alias` after provisioning.\n"]
    pub fn owner_alias(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.owner_alias", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.owner_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `permanent_restore` after provisioning.\n"]
    pub fn permanent_restore(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.permanent_restore", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `role_name` after provisioning.\n"]
    pub fn role_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.role_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `storage_tier` after provisioning.\n"]
    pub fn storage_tier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.storage_tier", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `temporary_restore_days` after provisioning.\n"]
    pub fn temporary_restore_days(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.temporary_restore_days", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `volume_id` after provisioning.\n"]
    pub fn volume_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.volume_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `volume_size` after provisioning.\n"]
    pub fn volume_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.volume_size", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `client_data` after provisioning.\n"]
    pub fn client_data(&self) -> ListRef<EbsSnapshotImportClientDataElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.client_data", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `disk_container` after provisioning.\n"]
    pub fn disk_container(&self) -> ListRef<EbsSnapshotImportDiskContainerElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.disk_container", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> EbsSnapshotImportTimeoutsElRef {
        EbsSnapshotImportTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct EbsSnapshotImportClientDataEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    comment: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    upload_end: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    upload_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    upload_start: Option<PrimField<String>>,
}
impl EbsSnapshotImportClientDataEl {
    #[doc = "Set the field `comment`.\n"]
    pub fn set_comment(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.comment = Some(v.into());
        self
    }
    #[doc = "Set the field `upload_end`.\n"]
    pub fn set_upload_end(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.upload_end = Some(v.into());
        self
    }
    #[doc = "Set the field `upload_size`.\n"]
    pub fn set_upload_size(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.upload_size = Some(v.into());
        self
    }
    #[doc = "Set the field `upload_start`.\n"]
    pub fn set_upload_start(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.upload_start = Some(v.into());
        self
    }
}
impl ToListMappable for EbsSnapshotImportClientDataEl {
    type O = BlockAssignable<EbsSnapshotImportClientDataEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEbsSnapshotImportClientDataEl {}
impl BuildEbsSnapshotImportClientDataEl {
    pub fn build(self) -> EbsSnapshotImportClientDataEl {
        EbsSnapshotImportClientDataEl {
            comment: core::default::Default::default(),
            upload_end: core::default::Default::default(),
            upload_size: core::default::Default::default(),
            upload_start: core::default::Default::default(),
        }
    }
}
pub struct EbsSnapshotImportClientDataElRef {
    shared: StackShared,
    base: String,
}
impl Ref for EbsSnapshotImportClientDataElRef {
    fn new(shared: StackShared, base: String) -> EbsSnapshotImportClientDataElRef {
        EbsSnapshotImportClientDataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl EbsSnapshotImportClientDataElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `comment` after provisioning.\n"]
    pub fn comment(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comment", self.base))
    }
    #[doc = "Get a reference to the value of field `upload_end` after provisioning.\n"]
    pub fn upload_end(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.upload_end", self.base))
    }
    #[doc = "Get a reference to the value of field `upload_size` after provisioning.\n"]
    pub fn upload_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.upload_size", self.base))
    }
    #[doc = "Get a reference to the value of field `upload_start` after provisioning.\n"]
    pub fn upload_start(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.upload_start", self.base))
    }
}
#[derive(Serialize)]
pub struct EbsSnapshotImportDiskContainerElUserBucketEl {
    s3_bucket: PrimField<String>,
    s3_key: PrimField<String>,
}
impl EbsSnapshotImportDiskContainerElUserBucketEl {}
impl ToListMappable for EbsSnapshotImportDiskContainerElUserBucketEl {
    type O = BlockAssignable<EbsSnapshotImportDiskContainerElUserBucketEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEbsSnapshotImportDiskContainerElUserBucketEl {
    #[doc = ""]
    pub s3_bucket: PrimField<String>,
    #[doc = ""]
    pub s3_key: PrimField<String>,
}
impl BuildEbsSnapshotImportDiskContainerElUserBucketEl {
    pub fn build(self) -> EbsSnapshotImportDiskContainerElUserBucketEl {
        EbsSnapshotImportDiskContainerElUserBucketEl {
            s3_bucket: self.s3_bucket,
            s3_key: self.s3_key,
        }
    }
}
pub struct EbsSnapshotImportDiskContainerElUserBucketElRef {
    shared: StackShared,
    base: String,
}
impl Ref for EbsSnapshotImportDiskContainerElUserBucketElRef {
    fn new(shared: StackShared, base: String) -> EbsSnapshotImportDiskContainerElUserBucketElRef {
        EbsSnapshotImportDiskContainerElUserBucketElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl EbsSnapshotImportDiskContainerElUserBucketElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `s3_bucket` after provisioning.\n"]
    pub fn s3_bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_bucket", self.base))
    }
    #[doc = "Get a reference to the value of field `s3_key` after provisioning.\n"]
    pub fn s3_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_key", self.base))
    }
}
#[derive(Serialize, Default)]
struct EbsSnapshotImportDiskContainerElDynamic {
    user_bucket: Option<DynamicBlock<EbsSnapshotImportDiskContainerElUserBucketEl>>,
}
#[derive(Serialize)]
pub struct EbsSnapshotImportDiskContainerEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    format: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_bucket: Option<Vec<EbsSnapshotImportDiskContainerElUserBucketEl>>,
    dynamic: EbsSnapshotImportDiskContainerElDynamic,
}
impl EbsSnapshotImportDiskContainerEl {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `url`.\n"]
    pub fn set_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.url = Some(v.into());
        self
    }
    #[doc = "Set the field `user_bucket`.\n"]
    pub fn set_user_bucket(
        mut self,
        v: impl Into<BlockAssignable<EbsSnapshotImportDiskContainerElUserBucketEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.user_bucket = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.user_bucket = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for EbsSnapshotImportDiskContainerEl {
    type O = BlockAssignable<EbsSnapshotImportDiskContainerEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEbsSnapshotImportDiskContainerEl {
    #[doc = ""]
    pub format: PrimField<String>,
}
impl BuildEbsSnapshotImportDiskContainerEl {
    pub fn build(self) -> EbsSnapshotImportDiskContainerEl {
        EbsSnapshotImportDiskContainerEl {
            description: core::default::Default::default(),
            format: self.format,
            url: core::default::Default::default(),
            user_bucket: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct EbsSnapshotImportDiskContainerElRef {
    shared: StackShared,
    base: String,
}
impl Ref for EbsSnapshotImportDiskContainerElRef {
    fn new(shared: StackShared, base: String) -> EbsSnapshotImportDiskContainerElRef {
        EbsSnapshotImportDiskContainerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl EbsSnapshotImportDiskContainerElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `format` after provisioning.\n"]
    pub fn format(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.format", self.base))
    }
    #[doc = "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
    #[doc = "Get a reference to the value of field `user_bucket` after provisioning.\n"]
    pub fn user_bucket(&self) -> ListRef<EbsSnapshotImportDiskContainerElUserBucketElRef> {
        ListRef::new(self.shared().clone(), format!("{}.user_bucket", self.base))
    }
}
#[derive(Serialize)]
pub struct EbsSnapshotImportTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}
impl EbsSnapshotImportTimeoutsEl {
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
}
impl ToListMappable for EbsSnapshotImportTimeoutsEl {
    type O = BlockAssignable<EbsSnapshotImportTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEbsSnapshotImportTimeoutsEl {}
impl BuildEbsSnapshotImportTimeoutsEl {
    pub fn build(self) -> EbsSnapshotImportTimeoutsEl {
        EbsSnapshotImportTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}
pub struct EbsSnapshotImportTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for EbsSnapshotImportTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> EbsSnapshotImportTimeoutsElRef {
        EbsSnapshotImportTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl EbsSnapshotImportTimeoutsElRef {
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
}
#[derive(Serialize, Default)]
struct EbsSnapshotImportDynamic {
    client_data: Option<DynamicBlock<EbsSnapshotImportClientDataEl>>,
    disk_container: Option<DynamicBlock<EbsSnapshotImportDiskContainerEl>>,
}
