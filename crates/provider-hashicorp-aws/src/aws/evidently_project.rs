use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct EvidentlyProjectData {
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
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_delivery: Option<Vec<EvidentlyProjectDataDeliveryEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<EvidentlyProjectTimeoutsEl>,
    dynamic: EvidentlyProjectDynamic,
}

struct EvidentlyProject_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<EvidentlyProjectData>,
}

#[derive(Clone)]
pub struct EvidentlyProject(Rc<EvidentlyProject_>);

impl EvidentlyProject {
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

    #[doc = "Set the field `data_delivery`.\n"]
    pub fn set_data_delivery(self, v: impl Into<BlockAssignable<EvidentlyProjectDataDeliveryEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().data_delivery = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.data_delivery = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<EvidentlyProjectTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `active_experiment_count` after provisioning.\n"]
    pub fn active_experiment_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.active_experiment_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `active_launch_count` after provisioning.\n"]
    pub fn active_launch_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.active_launch_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_time` after provisioning.\n"]
    pub fn created_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `experiment_count` after provisioning.\n"]
    pub fn experiment_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.experiment_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `feature_count` after provisioning.\n"]
    pub fn feature_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.feature_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_updated_time` after provisioning.\n"]
    pub fn last_updated_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.last_updated_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `launch_count` after provisioning.\n"]
    pub fn launch_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.launch_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `data_delivery` after provisioning.\n"]
    pub fn data_delivery(&self) -> ListRef<EvidentlyProjectDataDeliveryElRef> {
        ListRef::new(self.shared().clone(), format!("{}.data_delivery", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> EvidentlyProjectTimeoutsElRef {
        EvidentlyProjectTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for EvidentlyProject {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for EvidentlyProject { }

impl ToListMappable for EvidentlyProject {
    type O = ListRef<EvidentlyProjectRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for EvidentlyProject_ {
    fn extract_resource_type(&self) -> String {
        "aws_evidently_project".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildEvidentlyProject {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildEvidentlyProject {
    pub fn build(self, stack: &mut Stack) -> EvidentlyProject {
        let out = EvidentlyProject(Rc::new(EvidentlyProject_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(EvidentlyProjectData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                data_delivery: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct EvidentlyProjectRef {
    shared: StackShared,
    base: String,
}

impl Ref for EvidentlyProjectRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl EvidentlyProjectRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `active_experiment_count` after provisioning.\n"]
    pub fn active_experiment_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.active_experiment_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `active_launch_count` after provisioning.\n"]
    pub fn active_launch_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.active_launch_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_time` after provisioning.\n"]
    pub fn created_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `experiment_count` after provisioning.\n"]
    pub fn experiment_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.experiment_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `feature_count` after provisioning.\n"]
    pub fn feature_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.feature_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_updated_time` after provisioning.\n"]
    pub fn last_updated_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.last_updated_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `launch_count` after provisioning.\n"]
    pub fn launch_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.launch_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `data_delivery` after provisioning.\n"]
    pub fn data_delivery(&self) -> ListRef<EvidentlyProjectDataDeliveryElRef> {
        ListRef::new(self.shared().clone(), format!("{}.data_delivery", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> EvidentlyProjectTimeoutsElRef {
        EvidentlyProjectTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct EvidentlyProjectDataDeliveryElCloudwatchLogsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    log_group: Option<PrimField<String>>,
}

impl EvidentlyProjectDataDeliveryElCloudwatchLogsEl {
    #[doc = "Set the field `log_group`.\n"]
    pub fn set_log_group(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.log_group = Some(v.into());
        self
    }
}

impl ToListMappable for EvidentlyProjectDataDeliveryElCloudwatchLogsEl {
    type O = BlockAssignable<EvidentlyProjectDataDeliveryElCloudwatchLogsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEvidentlyProjectDataDeliveryElCloudwatchLogsEl {}

impl BuildEvidentlyProjectDataDeliveryElCloudwatchLogsEl {
    pub fn build(self) -> EvidentlyProjectDataDeliveryElCloudwatchLogsEl {
        EvidentlyProjectDataDeliveryElCloudwatchLogsEl { log_group: core::default::Default::default() }
    }
}

pub struct EvidentlyProjectDataDeliveryElCloudwatchLogsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EvidentlyProjectDataDeliveryElCloudwatchLogsElRef {
    fn new(shared: StackShared, base: String) -> EvidentlyProjectDataDeliveryElCloudwatchLogsElRef {
        EvidentlyProjectDataDeliveryElCloudwatchLogsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EvidentlyProjectDataDeliveryElCloudwatchLogsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `log_group` after provisioning.\n"]
    pub fn log_group(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_group", self.base))
    }
}

#[derive(Serialize)]
pub struct EvidentlyProjectDataDeliveryElS3DestinationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<PrimField<String>>,
}

impl EvidentlyProjectDataDeliveryElS3DestinationEl {
    #[doc = "Set the field `bucket`.\n"]
    pub fn set_bucket(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket = Some(v.into());
        self
    }

    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix = Some(v.into());
        self
    }
}

impl ToListMappable for EvidentlyProjectDataDeliveryElS3DestinationEl {
    type O = BlockAssignable<EvidentlyProjectDataDeliveryElS3DestinationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEvidentlyProjectDataDeliveryElS3DestinationEl {}

impl BuildEvidentlyProjectDataDeliveryElS3DestinationEl {
    pub fn build(self) -> EvidentlyProjectDataDeliveryElS3DestinationEl {
        EvidentlyProjectDataDeliveryElS3DestinationEl {
            bucket: core::default::Default::default(),
            prefix: core::default::Default::default(),
        }
    }
}

pub struct EvidentlyProjectDataDeliveryElS3DestinationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EvidentlyProjectDataDeliveryElS3DestinationElRef {
    fn new(shared: StackShared, base: String) -> EvidentlyProjectDataDeliveryElS3DestinationElRef {
        EvidentlyProjectDataDeliveryElS3DestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EvidentlyProjectDataDeliveryElS3DestinationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }
}

#[derive(Serialize, Default)]
struct EvidentlyProjectDataDeliveryElDynamic {
    cloudwatch_logs: Option<DynamicBlock<EvidentlyProjectDataDeliveryElCloudwatchLogsEl>>,
    s3_destination: Option<DynamicBlock<EvidentlyProjectDataDeliveryElS3DestinationEl>>,
}

#[derive(Serialize)]
pub struct EvidentlyProjectDataDeliveryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cloudwatch_logs: Option<Vec<EvidentlyProjectDataDeliveryElCloudwatchLogsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_destination: Option<Vec<EvidentlyProjectDataDeliveryElS3DestinationEl>>,
    dynamic: EvidentlyProjectDataDeliveryElDynamic,
}

impl EvidentlyProjectDataDeliveryEl {
    #[doc = "Set the field `cloudwatch_logs`.\n"]
    pub fn set_cloudwatch_logs(
        mut self,
        v: impl Into<BlockAssignable<EvidentlyProjectDataDeliveryElCloudwatchLogsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cloudwatch_logs = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cloudwatch_logs = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `s3_destination`.\n"]
    pub fn set_s3_destination(
        mut self,
        v: impl Into<BlockAssignable<EvidentlyProjectDataDeliveryElS3DestinationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.s3_destination = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_destination = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for EvidentlyProjectDataDeliveryEl {
    type O = BlockAssignable<EvidentlyProjectDataDeliveryEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEvidentlyProjectDataDeliveryEl {}

impl BuildEvidentlyProjectDataDeliveryEl {
    pub fn build(self) -> EvidentlyProjectDataDeliveryEl {
        EvidentlyProjectDataDeliveryEl {
            cloudwatch_logs: core::default::Default::default(),
            s3_destination: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct EvidentlyProjectDataDeliveryElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EvidentlyProjectDataDeliveryElRef {
    fn new(shared: StackShared, base: String) -> EvidentlyProjectDataDeliveryElRef {
        EvidentlyProjectDataDeliveryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EvidentlyProjectDataDeliveryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloudwatch_logs` after provisioning.\n"]
    pub fn cloudwatch_logs(&self) -> ListRef<EvidentlyProjectDataDeliveryElCloudwatchLogsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cloudwatch_logs", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_destination` after provisioning.\n"]
    pub fn s3_destination(&self) -> ListRef<EvidentlyProjectDataDeliveryElS3DestinationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.s3_destination", self.base))
    }
}

#[derive(Serialize)]
pub struct EvidentlyProjectTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl EvidentlyProjectTimeoutsEl {
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

impl ToListMappable for EvidentlyProjectTimeoutsEl {
    type O = BlockAssignable<EvidentlyProjectTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEvidentlyProjectTimeoutsEl {}

impl BuildEvidentlyProjectTimeoutsEl {
    pub fn build(self) -> EvidentlyProjectTimeoutsEl {
        EvidentlyProjectTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct EvidentlyProjectTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EvidentlyProjectTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> EvidentlyProjectTimeoutsElRef {
        EvidentlyProjectTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EvidentlyProjectTimeoutsElRef {
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
struct EvidentlyProjectDynamic {
    data_delivery: Option<DynamicBlock<EvidentlyProjectDataDeliveryEl>>,
}
