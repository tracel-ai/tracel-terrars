use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct Kinesisanalyticsv2ApplicationSnapshotData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    application_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    snapshot_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<Kinesisanalyticsv2ApplicationSnapshotTimeoutsEl>,
}

struct Kinesisanalyticsv2ApplicationSnapshot_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<Kinesisanalyticsv2ApplicationSnapshotData>,
}

#[derive(Clone)]
pub struct Kinesisanalyticsv2ApplicationSnapshot(Rc<Kinesisanalyticsv2ApplicationSnapshot_>);

impl Kinesisanalyticsv2ApplicationSnapshot {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<Kinesisanalyticsv2ApplicationSnapshotTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `application_name` after provisioning.\n"]
    pub fn application_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.application_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `application_version_id` after provisioning.\n"]
    pub fn application_version_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.application_version_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `snapshot_creation_timestamp` after provisioning.\n"]
    pub fn snapshot_creation_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.snapshot_creation_timestamp", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `snapshot_name` after provisioning.\n"]
    pub fn snapshot_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.snapshot_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Kinesisanalyticsv2ApplicationSnapshotTimeoutsElRef {
        Kinesisanalyticsv2ApplicationSnapshotTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for Kinesisanalyticsv2ApplicationSnapshot {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for Kinesisanalyticsv2ApplicationSnapshot { }

impl ToListMappable for Kinesisanalyticsv2ApplicationSnapshot {
    type O = ListRef<Kinesisanalyticsv2ApplicationSnapshotRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for Kinesisanalyticsv2ApplicationSnapshot_ {
    fn extract_resource_type(&self) -> String {
        "aws_kinesisanalyticsv2_application_snapshot".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildKinesisanalyticsv2ApplicationSnapshot {
    pub tf_id: String,
    #[doc = ""]
    pub application_name: PrimField<String>,
    #[doc = ""]
    pub snapshot_name: PrimField<String>,
}

impl BuildKinesisanalyticsv2ApplicationSnapshot {
    pub fn build(self, stack: &mut Stack) -> Kinesisanalyticsv2ApplicationSnapshot {
        let out = Kinesisanalyticsv2ApplicationSnapshot(Rc::new(Kinesisanalyticsv2ApplicationSnapshot_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(Kinesisanalyticsv2ApplicationSnapshotData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                application_name: self.application_name,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                snapshot_name: self.snapshot_name,
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct Kinesisanalyticsv2ApplicationSnapshotRef {
    shared: StackShared,
    base: String,
}

impl Ref for Kinesisanalyticsv2ApplicationSnapshotRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl Kinesisanalyticsv2ApplicationSnapshotRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `application_name` after provisioning.\n"]
    pub fn application_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.application_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `application_version_id` after provisioning.\n"]
    pub fn application_version_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.application_version_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `snapshot_creation_timestamp` after provisioning.\n"]
    pub fn snapshot_creation_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.snapshot_creation_timestamp", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `snapshot_name` after provisioning.\n"]
    pub fn snapshot_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.snapshot_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Kinesisanalyticsv2ApplicationSnapshotTimeoutsElRef {
        Kinesisanalyticsv2ApplicationSnapshotTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2ApplicationSnapshotTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}

impl Kinesisanalyticsv2ApplicationSnapshotTimeoutsEl {
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

impl ToListMappable for Kinesisanalyticsv2ApplicationSnapshotTimeoutsEl {
    type O = BlockAssignable<Kinesisanalyticsv2ApplicationSnapshotTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2ApplicationSnapshotTimeoutsEl {}

impl BuildKinesisanalyticsv2ApplicationSnapshotTimeoutsEl {
    pub fn build(self) -> Kinesisanalyticsv2ApplicationSnapshotTimeoutsEl {
        Kinesisanalyticsv2ApplicationSnapshotTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

pub struct Kinesisanalyticsv2ApplicationSnapshotTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Kinesisanalyticsv2ApplicationSnapshotTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> Kinesisanalyticsv2ApplicationSnapshotTimeoutsElRef {
        Kinesisanalyticsv2ApplicationSnapshotTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2ApplicationSnapshotTimeoutsElRef {
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
