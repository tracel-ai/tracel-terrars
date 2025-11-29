use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct FinspaceKxDataviewData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    auto_update: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    availability_zone_id: Option<PrimField<String>>,
    az_mode: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    changeset_id: Option<PrimField<String>>,
    database_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    environment_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    read_write: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    segment_configurations: Option<Vec<FinspaceKxDataviewSegmentConfigurationsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<FinspaceKxDataviewTimeoutsEl>,
    dynamic: FinspaceKxDataviewDynamic,
}

struct FinspaceKxDataview_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<FinspaceKxDataviewData>,
}

#[derive(Clone)]
pub struct FinspaceKxDataview(Rc<FinspaceKxDataview_>);

impl FinspaceKxDataview {
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

    #[doc = "Set the field `availability_zone_id`.\n"]
    pub fn set_availability_zone_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().availability_zone_id = Some(v.into());
        self
    }

    #[doc = "Set the field `changeset_id`.\n"]
    pub fn set_changeset_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().changeset_id = Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `read_write`.\n"]
    pub fn set_read_write(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().read_write = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
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

    #[doc = "Set the field `segment_configurations`.\n"]
    pub fn set_segment_configurations(
        self,
        v: impl Into<BlockAssignable<FinspaceKxDataviewSegmentConfigurationsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().segment_configurations = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.segment_configurations = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<FinspaceKxDataviewTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_update` after provisioning.\n"]
    pub fn auto_update(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_update", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `availability_zone_id` after provisioning.\n"]
    pub fn availability_zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.availability_zone_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `az_mode` after provisioning.\n"]
    pub fn az_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.az_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `changeset_id` after provisioning.\n"]
    pub fn changeset_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.changeset_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_timestamp` after provisioning.\n"]
    pub fn created_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_timestamp", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `database_name` after provisioning.\n"]
    pub fn database_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.database_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `environment_id` after provisioning.\n"]
    pub fn environment_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.environment_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_modified_timestamp` after provisioning.\n"]
    pub fn last_modified_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.last_modified_timestamp", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `read_write` after provisioning.\n"]
    pub fn read_write(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.read_write", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `segment_configurations` after provisioning.\n"]
    pub fn segment_configurations(&self) -> ListRef<FinspaceKxDataviewSegmentConfigurationsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.segment_configurations", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FinspaceKxDataviewTimeoutsElRef {
        FinspaceKxDataviewTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for FinspaceKxDataview {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for FinspaceKxDataview { }

impl ToListMappable for FinspaceKxDataview {
    type O = ListRef<FinspaceKxDataviewRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for FinspaceKxDataview_ {
    fn extract_resource_type(&self) -> String {
        "aws_finspace_kx_dataview".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildFinspaceKxDataview {
    pub tf_id: String,
    #[doc = ""]
    pub auto_update: PrimField<bool>,
    #[doc = ""]
    pub az_mode: PrimField<String>,
    #[doc = ""]
    pub database_name: PrimField<String>,
    #[doc = ""]
    pub environment_id: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildFinspaceKxDataview {
    pub fn build(self, stack: &mut Stack) -> FinspaceKxDataview {
        let out = FinspaceKxDataview(Rc::new(FinspaceKxDataview_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(FinspaceKxDataviewData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                auto_update: self.auto_update,
                availability_zone_id: core::default::Default::default(),
                az_mode: self.az_mode,
                changeset_id: core::default::Default::default(),
                database_name: self.database_name,
                description: core::default::Default::default(),
                environment_id: self.environment_id,
                id: core::default::Default::default(),
                name: self.name,
                read_write: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                segment_configurations: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct FinspaceKxDataviewRef {
    shared: StackShared,
    base: String,
}

impl Ref for FinspaceKxDataviewRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl FinspaceKxDataviewRef {
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

    #[doc = "Get a reference to the value of field `auto_update` after provisioning.\n"]
    pub fn auto_update(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_update", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `availability_zone_id` after provisioning.\n"]
    pub fn availability_zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.availability_zone_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `az_mode` after provisioning.\n"]
    pub fn az_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.az_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `changeset_id` after provisioning.\n"]
    pub fn changeset_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.changeset_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_timestamp` after provisioning.\n"]
    pub fn created_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_timestamp", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `database_name` after provisioning.\n"]
    pub fn database_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.database_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `environment_id` after provisioning.\n"]
    pub fn environment_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.environment_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_modified_timestamp` after provisioning.\n"]
    pub fn last_modified_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.last_modified_timestamp", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `read_write` after provisioning.\n"]
    pub fn read_write(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.read_write", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `segment_configurations` after provisioning.\n"]
    pub fn segment_configurations(&self) -> ListRef<FinspaceKxDataviewSegmentConfigurationsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.segment_configurations", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FinspaceKxDataviewTimeoutsElRef {
        FinspaceKxDataviewTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct FinspaceKxDataviewSegmentConfigurationsEl {
    db_paths: ListField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    on_demand: Option<PrimField<bool>>,
    volume_name: PrimField<String>,
}

impl FinspaceKxDataviewSegmentConfigurationsEl {
    #[doc = "Set the field `on_demand`.\n"]
    pub fn set_on_demand(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.on_demand = Some(v.into());
        self
    }
}

impl ToListMappable for FinspaceKxDataviewSegmentConfigurationsEl {
    type O = BlockAssignable<FinspaceKxDataviewSegmentConfigurationsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFinspaceKxDataviewSegmentConfigurationsEl {
    #[doc = ""]
    pub db_paths: ListField<PrimField<String>>,
    #[doc = ""]
    pub volume_name: PrimField<String>,
}

impl BuildFinspaceKxDataviewSegmentConfigurationsEl {
    pub fn build(self) -> FinspaceKxDataviewSegmentConfigurationsEl {
        FinspaceKxDataviewSegmentConfigurationsEl {
            db_paths: self.db_paths,
            on_demand: core::default::Default::default(),
            volume_name: self.volume_name,
        }
    }
}

pub struct FinspaceKxDataviewSegmentConfigurationsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for FinspaceKxDataviewSegmentConfigurationsElRef {
    fn new(shared: StackShared, base: String) -> FinspaceKxDataviewSegmentConfigurationsElRef {
        FinspaceKxDataviewSegmentConfigurationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FinspaceKxDataviewSegmentConfigurationsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `db_paths` after provisioning.\n"]
    pub fn db_paths(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.db_paths", self.base))
    }

    #[doc = "Get a reference to the value of field `on_demand` after provisioning.\n"]
    pub fn on_demand(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.on_demand", self.base))
    }

    #[doc = "Get a reference to the value of field `volume_name` after provisioning.\n"]
    pub fn volume_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.volume_name", self.base))
    }
}

#[derive(Serialize)]
pub struct FinspaceKxDataviewTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl FinspaceKxDataviewTimeoutsEl {
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

impl ToListMappable for FinspaceKxDataviewTimeoutsEl {
    type O = BlockAssignable<FinspaceKxDataviewTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFinspaceKxDataviewTimeoutsEl {}

impl BuildFinspaceKxDataviewTimeoutsEl {
    pub fn build(self) -> FinspaceKxDataviewTimeoutsEl {
        FinspaceKxDataviewTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct FinspaceKxDataviewTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for FinspaceKxDataviewTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> FinspaceKxDataviewTimeoutsElRef {
        FinspaceKxDataviewTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FinspaceKxDataviewTimeoutsElRef {
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
struct FinspaceKxDataviewDynamic {
    segment_configurations: Option<DynamicBlock<FinspaceKxDataviewSegmentConfigurationsEl>>,
}
