use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DmsReplicationTaskData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cdc_start_position: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cdc_start_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    migration_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    replication_instance_arn: PrimField<String>,
    replication_task_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    replication_task_settings: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_identifier: Option<PrimField<String>>,
    source_endpoint_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_replication_task: Option<PrimField<bool>>,
    table_mappings: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    target_endpoint_arn: PrimField<String>,
}
struct DmsReplicationTask_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DmsReplicationTaskData>,
}
#[derive(Clone)]
pub struct DmsReplicationTask(Rc<DmsReplicationTask_>);
impl DmsReplicationTask {
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
    #[doc = "Set the field `cdc_start_position`.\n"]
    pub fn set_cdc_start_position(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().cdc_start_position = Some(v.into());
        self
    }
    #[doc = "Set the field `cdc_start_time`.\n"]
    pub fn set_cdc_start_time(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().cdc_start_time = Some(v.into());
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
    #[doc = "Set the field `replication_task_settings`.\n"]
    pub fn set_replication_task_settings(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().replication_task_settings = Some(v.into());
        self
    }
    #[doc = "Set the field `resource_identifier`.\n"]
    pub fn set_resource_identifier(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().resource_identifier = Some(v.into());
        self
    }
    #[doc = "Set the field `start_replication_task`.\n"]
    pub fn set_start_replication_task(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().start_replication_task = Some(v.into());
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
    #[doc = "Get a reference to the value of field `cdc_start_position` after provisioning.\n"]
    pub fn cdc_start_position(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cdc_start_position", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cdc_start_time` after provisioning.\n"]
    pub fn cdc_start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cdc_start_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `migration_type` after provisioning.\n"]
    pub fn migration_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.migration_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `replication_instance_arn` after provisioning.\n"]
    pub fn replication_instance_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.replication_instance_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `replication_task_arn` after provisioning.\n"]
    pub fn replication_task_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.replication_task_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `replication_task_id` after provisioning.\n"]
    pub fn replication_task_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.replication_task_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `replication_task_settings` after provisioning.\n"]
    pub fn replication_task_settings(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.replication_task_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resource_identifier` after provisioning.\n"]
    pub fn resource_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_identifier", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_endpoint_arn` after provisioning.\n"]
    pub fn source_endpoint_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_endpoint_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `start_replication_task` after provisioning.\n"]
    pub fn start_replication_task(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.start_replication_task", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `table_mappings` after provisioning.\n"]
    pub fn table_mappings(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.table_mappings", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `target_endpoint_arn` after provisioning.\n"]
    pub fn target_endpoint_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_endpoint_arn", self.extract_ref()),
        )
    }
}
impl Referable for DmsReplicationTask {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for DmsReplicationTask {}
impl ToListMappable for DmsReplicationTask {
    type O = ListRef<DmsReplicationTaskRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for DmsReplicationTask_ {
    fn extract_resource_type(&self) -> String {
        "aws_dms_replication_task".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDmsReplicationTask {
    pub tf_id: String,
    #[doc = ""]
    pub migration_type: PrimField<String>,
    #[doc = ""]
    pub replication_instance_arn: PrimField<String>,
    #[doc = ""]
    pub replication_task_id: PrimField<String>,
    #[doc = ""]
    pub source_endpoint_arn: PrimField<String>,
    #[doc = ""]
    pub table_mappings: PrimField<String>,
    #[doc = ""]
    pub target_endpoint_arn: PrimField<String>,
}
impl BuildDmsReplicationTask {
    pub fn build(self, stack: &mut Stack) -> DmsReplicationTask {
        let out = DmsReplicationTask(Rc::new(DmsReplicationTask_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DmsReplicationTaskData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                cdc_start_position: core::default::Default::default(),
                cdc_start_time: core::default::Default::default(),
                id: core::default::Default::default(),
                migration_type: self.migration_type,
                region: core::default::Default::default(),
                replication_instance_arn: self.replication_instance_arn,
                replication_task_id: self.replication_task_id,
                replication_task_settings: core::default::Default::default(),
                resource_identifier: core::default::Default::default(),
                source_endpoint_arn: self.source_endpoint_arn,
                start_replication_task: core::default::Default::default(),
                table_mappings: self.table_mappings,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                target_endpoint_arn: self.target_endpoint_arn,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct DmsReplicationTaskRef {
    shared: StackShared,
    base: String,
}
impl Ref for DmsReplicationTaskRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DmsReplicationTaskRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cdc_start_position` after provisioning.\n"]
    pub fn cdc_start_position(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cdc_start_position", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cdc_start_time` after provisioning.\n"]
    pub fn cdc_start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cdc_start_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `migration_type` after provisioning.\n"]
    pub fn migration_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.migration_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `replication_instance_arn` after provisioning.\n"]
    pub fn replication_instance_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.replication_instance_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `replication_task_arn` after provisioning.\n"]
    pub fn replication_task_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.replication_task_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `replication_task_id` after provisioning.\n"]
    pub fn replication_task_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.replication_task_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `replication_task_settings` after provisioning.\n"]
    pub fn replication_task_settings(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.replication_task_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resource_identifier` after provisioning.\n"]
    pub fn resource_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_identifier", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_endpoint_arn` after provisioning.\n"]
    pub fn source_endpoint_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_endpoint_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `start_replication_task` after provisioning.\n"]
    pub fn start_replication_task(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.start_replication_task", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `table_mappings` after provisioning.\n"]
    pub fn table_mappings(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.table_mappings", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `target_endpoint_arn` after provisioning.\n"]
    pub fn target_endpoint_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_endpoint_arn", self.extract_ref()),
        )
    }
}
