use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ComputeDiskAsyncReplicationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    primary_disk: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secondary_disk: Option<Vec<ComputeDiskAsyncReplicationSecondaryDiskEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ComputeDiskAsyncReplicationTimeoutsEl>,
    dynamic: ComputeDiskAsyncReplicationDynamic,
}
struct ComputeDiskAsyncReplication_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ComputeDiskAsyncReplicationData>,
}
#[derive(Clone)]
pub struct ComputeDiskAsyncReplication(Rc<ComputeDiskAsyncReplication_>);
impl ComputeDiskAsyncReplication {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(self, provider: &ProviderGoogle) -> Self {
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
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `secondary_disk`.\n"]
    pub fn set_secondary_disk(
        self,
        v: impl Into<BlockAssignable<ComputeDiskAsyncReplicationSecondaryDiskEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().secondary_disk = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.secondary_disk = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ComputeDiskAsyncReplicationTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `primary_disk` after provisioning.\nPrimary disk for asynchronous replication."]
    pub fn primary_disk(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.primary_disk", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `secondary_disk` after provisioning.\n"]
    pub fn secondary_disk(&self) -> ListRef<ComputeDiskAsyncReplicationSecondaryDiskElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.secondary_disk", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeDiskAsyncReplicationTimeoutsElRef {
        ComputeDiskAsyncReplicationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ComputeDiskAsyncReplication {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ComputeDiskAsyncReplication {}
impl ToListMappable for ComputeDiskAsyncReplication {
    type O = ListRef<ComputeDiskAsyncReplicationRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ComputeDiskAsyncReplication_ {
    fn extract_resource_type(&self) -> String {
        "google_compute_disk_async_replication".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildComputeDiskAsyncReplication {
    pub tf_id: String,
    #[doc = "Primary disk for asynchronous replication."]
    pub primary_disk: PrimField<String>,
}
impl BuildComputeDiskAsyncReplication {
    pub fn build(self, stack: &mut Stack) -> ComputeDiskAsyncReplication {
        let out = ComputeDiskAsyncReplication(Rc::new(ComputeDiskAsyncReplication_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ComputeDiskAsyncReplicationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                primary_disk: self.primary_disk,
                secondary_disk: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ComputeDiskAsyncReplicationRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeDiskAsyncReplicationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ComputeDiskAsyncReplicationRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `primary_disk` after provisioning.\nPrimary disk for asynchronous replication."]
    pub fn primary_disk(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.primary_disk", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `secondary_disk` after provisioning.\n"]
    pub fn secondary_disk(&self) -> ListRef<ComputeDiskAsyncReplicationSecondaryDiskElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.secondary_disk", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeDiskAsyncReplicationTimeoutsElRef {
        ComputeDiskAsyncReplicationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeDiskAsyncReplicationSecondaryDiskEl {
    disk: PrimField<String>,
}
impl ComputeDiskAsyncReplicationSecondaryDiskEl {}
impl ToListMappable for ComputeDiskAsyncReplicationSecondaryDiskEl {
    type O = BlockAssignable<ComputeDiskAsyncReplicationSecondaryDiskEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeDiskAsyncReplicationSecondaryDiskEl {
    #[doc = "Secondary disk for asynchronous replication."]
    pub disk: PrimField<String>,
}
impl BuildComputeDiskAsyncReplicationSecondaryDiskEl {
    pub fn build(self) -> ComputeDiskAsyncReplicationSecondaryDiskEl {
        ComputeDiskAsyncReplicationSecondaryDiskEl { disk: self.disk }
    }
}
pub struct ComputeDiskAsyncReplicationSecondaryDiskElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeDiskAsyncReplicationSecondaryDiskElRef {
    fn new(shared: StackShared, base: String) -> ComputeDiskAsyncReplicationSecondaryDiskElRef {
        ComputeDiskAsyncReplicationSecondaryDiskElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeDiskAsyncReplicationSecondaryDiskElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `disk` after provisioning.\nSecondary disk for asynchronous replication."]
    pub fn disk(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk", self.base))
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nOutput-only. Status of replication on the secondary disk."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeDiskAsyncReplicationTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}
impl ComputeDiskAsyncReplicationTimeoutsEl {
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
impl ToListMappable for ComputeDiskAsyncReplicationTimeoutsEl {
    type O = BlockAssignable<ComputeDiskAsyncReplicationTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeDiskAsyncReplicationTimeoutsEl {}
impl BuildComputeDiskAsyncReplicationTimeoutsEl {
    pub fn build(self) -> ComputeDiskAsyncReplicationTimeoutsEl {
        ComputeDiskAsyncReplicationTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}
pub struct ComputeDiskAsyncReplicationTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeDiskAsyncReplicationTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ComputeDiskAsyncReplicationTimeoutsElRef {
        ComputeDiskAsyncReplicationTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeDiskAsyncReplicationTimeoutsElRef {
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
struct ComputeDiskAsyncReplicationDynamic {
    secondary_disk: Option<DynamicBlock<ComputeDiskAsyncReplicationSecondaryDiskEl>>,
}
