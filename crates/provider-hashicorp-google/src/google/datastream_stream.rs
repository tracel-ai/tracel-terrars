use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DatastreamStreamData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    create_without_validation: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_managed_encryption_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    desired_state: Option<PrimField<String>>,
    display_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    stream_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    backfill_all: Option<Vec<DatastreamStreamBackfillAllEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    backfill_none: Option<Vec<DatastreamStreamBackfillNoneEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_config: Option<Vec<DatastreamStreamDestinationConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rule_sets: Option<Vec<DatastreamStreamRuleSetsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_config: Option<Vec<DatastreamStreamSourceConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DatastreamStreamTimeoutsEl>,
    dynamic: DatastreamStreamDynamic,
}
struct DatastreamStream_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DatastreamStreamData>,
}
#[derive(Clone)]
pub struct DatastreamStream(Rc<DatastreamStream_>);
impl DatastreamStream {
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
    #[doc = "Set the field `create_without_validation`.\nCreate the stream without validating it."]
    pub fn set_create_without_validation(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().create_without_validation = Some(v.into());
        self
    }
    #[doc = "Set the field `customer_managed_encryption_key`.\nA reference to a KMS encryption key. If provided, it will be used to encrypt the data. If left blank, data\nwill be encrypted using an internal Stream-specific encryption key provisioned through KMS."]
    pub fn set_customer_managed_encryption_key(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().customer_managed_encryption_key = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `desired_state`.\nDesired state of the Stream. Set this field to 'RUNNING' to start the stream,\n'NOT_STARTED' to create the stream without starting and 'PAUSED' to pause\nthe stream from a 'RUNNING' state.\nPossible values: NOT_STARTED, RUNNING, PAUSED. Default: NOT_STARTED"]
    pub fn set_desired_state(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().desired_state = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nLabels.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `backfill_all`.\n"]
    pub fn set_backfill_all(
        self,
        v: impl Into<BlockAssignable<DatastreamStreamBackfillAllEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().backfill_all = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.backfill_all = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `backfill_none`.\n"]
    pub fn set_backfill_none(
        self,
        v: impl Into<BlockAssignable<DatastreamStreamBackfillNoneEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().backfill_none = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.backfill_none = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `destination_config`.\n"]
    pub fn set_destination_config(
        self,
        v: impl Into<BlockAssignable<DatastreamStreamDestinationConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().destination_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.destination_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `rule_sets`.\n"]
    pub fn set_rule_sets(self, v: impl Into<BlockAssignable<DatastreamStreamRuleSetsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rule_sets = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rule_sets = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `source_config`.\n"]
    pub fn set_source_config(
        self,
        v: impl Into<BlockAssignable<DatastreamStreamSourceConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().source_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.source_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DatastreamStreamTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `create_without_validation` after provisioning.\nCreate the stream without validating it."]
    pub fn create_without_validation(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_without_validation", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `customer_managed_encryption_key` after provisioning.\nA reference to a KMS encryption key. If provided, it will be used to encrypt the data. If left blank, data\nwill be encrypted using an internal Stream-specific encryption key provisioned through KMS."]
    pub fn customer_managed_encryption_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.customer_managed_encryption_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `desired_state` after provisioning.\nDesired state of the Stream. Set this field to 'RUNNING' to start the stream,\n'NOT_STARTED' to create the stream without starting and 'PAUSED' to pause\nthe stream from a 'RUNNING' state.\nPossible values: NOT_STARTED, RUNNING, PAUSED. Default: NOT_STARTED"]
    pub fn desired_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.desired_state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nDisplay name."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nLabels.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe name of the location this stream is located in."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe stream's name."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe state of the stream."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `stream_id` after provisioning.\nThe stream identifier."]
    pub fn stream_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.stream_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `backfill_all` after provisioning.\n"]
    pub fn backfill_all(&self) -> ListRef<DatastreamStreamBackfillAllElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.backfill_all", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `backfill_none` after provisioning.\n"]
    pub fn backfill_none(&self) -> ListRef<DatastreamStreamBackfillNoneElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.backfill_none", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `destination_config` after provisioning.\n"]
    pub fn destination_config(&self) -> ListRef<DatastreamStreamDestinationConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.destination_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rule_sets` after provisioning.\n"]
    pub fn rule_sets(&self) -> ListRef<DatastreamStreamRuleSetsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rule_sets", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_config` after provisioning.\n"]
    pub fn source_config(&self) -> ListRef<DatastreamStreamSourceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DatastreamStreamTimeoutsElRef {
        DatastreamStreamTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for DatastreamStream {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for DatastreamStream {}
impl ToListMappable for DatastreamStream {
    type O = ListRef<DatastreamStreamRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for DatastreamStream_ {
    fn extract_resource_type(&self) -> String {
        "google_datastream_stream".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDatastreamStream {
    pub tf_id: String,
    #[doc = "Display name."]
    pub display_name: PrimField<String>,
    #[doc = "The name of the location this stream is located in."]
    pub location: PrimField<String>,
    #[doc = "The stream identifier."]
    pub stream_id: PrimField<String>,
}
impl BuildDatastreamStream {
    pub fn build(self, stack: &mut Stack) -> DatastreamStream {
        let out = DatastreamStream(Rc::new(DatastreamStream_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DatastreamStreamData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                create_without_validation: core::default::Default::default(),
                customer_managed_encryption_key: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                desired_state: core::default::Default::default(),
                display_name: self.display_name,
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                stream_id: self.stream_id,
                backfill_all: core::default::Default::default(),
                backfill_none: core::default::Default::default(),
                destination_config: core::default::Default::default(),
                rule_sets: core::default::Default::default(),
                source_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct DatastreamStreamRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DatastreamStreamRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_without_validation` after provisioning.\nCreate the stream without validating it."]
    pub fn create_without_validation(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_without_validation", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `customer_managed_encryption_key` after provisioning.\nA reference to a KMS encryption key. If provided, it will be used to encrypt the data. If left blank, data\nwill be encrypted using an internal Stream-specific encryption key provisioned through KMS."]
    pub fn customer_managed_encryption_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.customer_managed_encryption_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `desired_state` after provisioning.\nDesired state of the Stream. Set this field to 'RUNNING' to start the stream,\n'NOT_STARTED' to create the stream without starting and 'PAUSED' to pause\nthe stream from a 'RUNNING' state.\nPossible values: NOT_STARTED, RUNNING, PAUSED. Default: NOT_STARTED"]
    pub fn desired_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.desired_state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nDisplay name."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nLabels.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe name of the location this stream is located in."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe stream's name."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe state of the stream."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `stream_id` after provisioning.\nThe stream identifier."]
    pub fn stream_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.stream_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `backfill_all` after provisioning.\n"]
    pub fn backfill_all(&self) -> ListRef<DatastreamStreamBackfillAllElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.backfill_all", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `backfill_none` after provisioning.\n"]
    pub fn backfill_none(&self) -> ListRef<DatastreamStreamBackfillNoneElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.backfill_none", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `destination_config` after provisioning.\n"]
    pub fn destination_config(&self) -> ListRef<DatastreamStreamDestinationConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.destination_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rule_sets` after provisioning.\n"]
    pub fn rule_sets(&self) -> ListRef<DatastreamStreamRuleSetsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rule_sets", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_config` after provisioning.\n"]
    pub fn source_config(&self) -> ListRef<DatastreamStreamSourceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DatastreamStreamTimeoutsElRef {
        DatastreamStreamTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DatastreamStreamBackfillAllElMongodbExcludedObjectsElDatabasesElCollectionsElFieldsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    field: Option<PrimField<String>>,
}
impl DatastreamStreamBackfillAllElMongodbExcludedObjectsElDatabasesElCollectionsElFieldsEl {
    #[doc = "Set the field `field`.\nField name."]
    pub fn set_field(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.field = Some(v.into());
        self
    }
}
impl ToListMappable
    for DatastreamStreamBackfillAllElMongodbExcludedObjectsElDatabasesElCollectionsElFieldsEl
{
    type O = BlockAssignable<
        DatastreamStreamBackfillAllElMongodbExcludedObjectsElDatabasesElCollectionsElFieldsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamBackfillAllElMongodbExcludedObjectsElDatabasesElCollectionsElFieldsEl
{}
impl BuildDatastreamStreamBackfillAllElMongodbExcludedObjectsElDatabasesElCollectionsElFieldsEl {
    pub fn build(
        self,
    ) -> DatastreamStreamBackfillAllElMongodbExcludedObjectsElDatabasesElCollectionsElFieldsEl {
        DatastreamStreamBackfillAllElMongodbExcludedObjectsElDatabasesElCollectionsElFieldsEl {
            field: core::default::Default::default(),
        }
    }
}
pub struct DatastreamStreamBackfillAllElMongodbExcludedObjectsElDatabasesElCollectionsElFieldsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for DatastreamStreamBackfillAllElMongodbExcludedObjectsElDatabasesElCollectionsElFieldsElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamStreamBackfillAllElMongodbExcludedObjectsElDatabasesElCollectionsElFieldsElRef
    {
        DatastreamStreamBackfillAllElMongodbExcludedObjectsElDatabasesElCollectionsElFieldsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamBackfillAllElMongodbExcludedObjectsElDatabasesElCollectionsElFieldsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `field` after provisioning.\nField name."]
    pub fn field(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.field", self.base))
    }
}
#[derive(Serialize, Default)]
struct DatastreamStreamBackfillAllElMongodbExcludedObjectsElDatabasesElCollectionsElDynamic {
    fields: Option<
        DynamicBlock<
            DatastreamStreamBackfillAllElMongodbExcludedObjectsElDatabasesElCollectionsElFieldsEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct DatastreamStreamBackfillAllElMongodbExcludedObjectsElDatabasesElCollectionsEl {
    collection: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fields: Option<
        Vec<DatastreamStreamBackfillAllElMongodbExcludedObjectsElDatabasesElCollectionsElFieldsEl>,
    >,
    dynamic: DatastreamStreamBackfillAllElMongodbExcludedObjectsElDatabasesElCollectionsElDynamic,
}
impl DatastreamStreamBackfillAllElMongodbExcludedObjectsElDatabasesElCollectionsEl {
    #[doc = "Set the field `fields`.\n"]
    pub fn set_fields(
        mut self,
        v : impl Into < BlockAssignable < DatastreamStreamBackfillAllElMongodbExcludedObjectsElDatabasesElCollectionsElFieldsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.fields = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.fields = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for DatastreamStreamBackfillAllElMongodbExcludedObjectsElDatabasesElCollectionsEl
{
    type O = BlockAssignable<
        DatastreamStreamBackfillAllElMongodbExcludedObjectsElDatabasesElCollectionsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamBackfillAllElMongodbExcludedObjectsElDatabasesElCollectionsEl {
    #[doc = "Collection name."]
    pub collection: PrimField<String>,
}
impl BuildDatastreamStreamBackfillAllElMongodbExcludedObjectsElDatabasesElCollectionsEl {
    pub fn build(
        self,
    ) -> DatastreamStreamBackfillAllElMongodbExcludedObjectsElDatabasesElCollectionsEl {
        DatastreamStreamBackfillAllElMongodbExcludedObjectsElDatabasesElCollectionsEl {
            collection: self.collection,
            fields: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DatastreamStreamBackfillAllElMongodbExcludedObjectsElDatabasesElCollectionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamBackfillAllElMongodbExcludedObjectsElDatabasesElCollectionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamStreamBackfillAllElMongodbExcludedObjectsElDatabasesElCollectionsElRef {
        DatastreamStreamBackfillAllElMongodbExcludedObjectsElDatabasesElCollectionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamBackfillAllElMongodbExcludedObjectsElDatabasesElCollectionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `collection` after provisioning.\nCollection name."]
    pub fn collection(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.collection", self.base))
    }
    #[doc = "Get a reference to the value of field `fields` after provisioning.\n"]
    pub fn fields(
        &self,
    ) -> ListRef<
        DatastreamStreamBackfillAllElMongodbExcludedObjectsElDatabasesElCollectionsElFieldsElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.fields", self.base))
    }
}
#[derive(Serialize, Default)]
struct DatastreamStreamBackfillAllElMongodbExcludedObjectsElDatabasesElDynamic {
    collections: Option<
        DynamicBlock<DatastreamStreamBackfillAllElMongodbExcludedObjectsElDatabasesElCollectionsEl>,
    >,
}
#[derive(Serialize)]
pub struct DatastreamStreamBackfillAllElMongodbExcludedObjectsElDatabasesEl {
    database: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    collections:
        Option<Vec<DatastreamStreamBackfillAllElMongodbExcludedObjectsElDatabasesElCollectionsEl>>,
    dynamic: DatastreamStreamBackfillAllElMongodbExcludedObjectsElDatabasesElDynamic,
}
impl DatastreamStreamBackfillAllElMongodbExcludedObjectsElDatabasesEl {
    #[doc = "Set the field `collections`.\n"]
    pub fn set_collections(
        mut self,
        v: impl Into<
            BlockAssignable<
                DatastreamStreamBackfillAllElMongodbExcludedObjectsElDatabasesElCollectionsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.collections = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.collections = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DatastreamStreamBackfillAllElMongodbExcludedObjectsElDatabasesEl {
    type O = BlockAssignable<DatastreamStreamBackfillAllElMongodbExcludedObjectsElDatabasesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamBackfillAllElMongodbExcludedObjectsElDatabasesEl {
    #[doc = "Database name."]
    pub database: PrimField<String>,
}
impl BuildDatastreamStreamBackfillAllElMongodbExcludedObjectsElDatabasesEl {
    pub fn build(self) -> DatastreamStreamBackfillAllElMongodbExcludedObjectsElDatabasesEl {
        DatastreamStreamBackfillAllElMongodbExcludedObjectsElDatabasesEl {
            database: self.database,
            collections: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DatastreamStreamBackfillAllElMongodbExcludedObjectsElDatabasesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamBackfillAllElMongodbExcludedObjectsElDatabasesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamStreamBackfillAllElMongodbExcludedObjectsElDatabasesElRef {
        DatastreamStreamBackfillAllElMongodbExcludedObjectsElDatabasesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamBackfillAllElMongodbExcludedObjectsElDatabasesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `database` after provisioning.\nDatabase name."]
    pub fn database(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.database", self.base))
    }
    #[doc = "Get a reference to the value of field `collections` after provisioning.\n"]
    pub fn collections(
        &self,
    ) -> ListRef<DatastreamStreamBackfillAllElMongodbExcludedObjectsElDatabasesElCollectionsElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.collections", self.base))
    }
}
#[derive(Serialize, Default)]
struct DatastreamStreamBackfillAllElMongodbExcludedObjectsElDynamic {
    databases:
        Option<DynamicBlock<DatastreamStreamBackfillAllElMongodbExcludedObjectsElDatabasesEl>>,
}
#[derive(Serialize)]
pub struct DatastreamStreamBackfillAllElMongodbExcludedObjectsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    databases: Option<Vec<DatastreamStreamBackfillAllElMongodbExcludedObjectsElDatabasesEl>>,
    dynamic: DatastreamStreamBackfillAllElMongodbExcludedObjectsElDynamic,
}
impl DatastreamStreamBackfillAllElMongodbExcludedObjectsEl {
    #[doc = "Set the field `databases`.\n"]
    pub fn set_databases(
        mut self,
        v: impl Into<BlockAssignable<DatastreamStreamBackfillAllElMongodbExcludedObjectsElDatabasesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.databases = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.databases = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DatastreamStreamBackfillAllElMongodbExcludedObjectsEl {
    type O = BlockAssignable<DatastreamStreamBackfillAllElMongodbExcludedObjectsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamBackfillAllElMongodbExcludedObjectsEl {}
impl BuildDatastreamStreamBackfillAllElMongodbExcludedObjectsEl {
    pub fn build(self) -> DatastreamStreamBackfillAllElMongodbExcludedObjectsEl {
        DatastreamStreamBackfillAllElMongodbExcludedObjectsEl {
            databases: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DatastreamStreamBackfillAllElMongodbExcludedObjectsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamBackfillAllElMongodbExcludedObjectsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamStreamBackfillAllElMongodbExcludedObjectsElRef {
        DatastreamStreamBackfillAllElMongodbExcludedObjectsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamBackfillAllElMongodbExcludedObjectsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `databases` after provisioning.\n"]
    pub fn databases(
        &self,
    ) -> ListRef<DatastreamStreamBackfillAllElMongodbExcludedObjectsElDatabasesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.databases", self.base))
    }
}
#[derive(Serialize)]
pub struct DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    collation: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    column: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nullable: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ordinal_position: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    primary_key: Option<PrimField<bool>>,
}
impl
    DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsEl
{
    #[doc = "Set the field `collation`.\nColumn collation."]
    pub fn set_collation(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.collation = Some(v.into());
        self
    }
    #[doc = "Set the field `column`.\nColumn name."]
    pub fn set_column(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.column = Some(v.into());
        self
    }
    #[doc = "Set the field `data_type`.\nThe MySQL data type. Full data types list can be found here:\nhttps://dev.mysql.com/doc/refman/8.0/en/data-types.html"]
    pub fn set_data_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.data_type = Some(v.into());
        self
    }
    #[doc = "Set the field `nullable`.\nWhether or not the column can accept a null value."]
    pub fn set_nullable(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.nullable = Some(v.into());
        self
    }
    #[doc = "Set the field `ordinal_position`.\nThe ordinal position of the column in the table."]
    pub fn set_ordinal_position(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.ordinal_position = Some(v.into());
        self
    }
    #[doc = "Set the field `primary_key`.\nWhether or not the column represents a primary key."]
    pub fn set_primary_key(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.primary_key = Some(v.into());
        self
    }
}
impl ToListMappable for DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsEl { type O = BlockAssignable < DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsEl
{}
impl BuildDatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsEl { pub fn build (self) -> DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsEl { DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsEl { collation : core :: default :: Default :: default () , column : core :: default :: Default :: default () , data_type : core :: default :: Default :: default () , nullable : core :: default :: Default :: default () , ordinal_position : core :: default :: Default :: default () , primary_key : core :: default :: Default :: default () , } } }
pub struct DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsElRef { fn new (shared : StackShared , base : String) -> DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsElRef { DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsElRef { shared : shared , base : base . to_string () , } } }
impl DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `collation` after provisioning.\nColumn collation."] pub fn collation (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.collation" , self . base)) } # [doc = "Get a reference to the value of field `column` after provisioning.\nColumn name."] pub fn column (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.column" , self . base)) } # [doc = "Get a reference to the value of field `data_type` after provisioning.\nThe MySQL data type. Full data types list can be found here:\nhttps://dev.mysql.com/doc/refman/8.0/en/data-types.html"] pub fn data_type (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.data_type" , self . base)) } # [doc = "Get a reference to the value of field `length` after provisioning.\nColumn length."] pub fn length (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.length" , self . base)) } # [doc = "Get a reference to the value of field `nullable` after provisioning.\nWhether or not the column can accept a null value."] pub fn nullable (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.nullable" , self . base)) } # [doc = "Get a reference to the value of field `ordinal_position` after provisioning.\nThe ordinal position of the column in the table."] pub fn ordinal_position (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.ordinal_position" , self . base)) } # [doc = "Get a reference to the value of field `primary_key` after provisioning.\nWhether or not the column represents a primary key."] pub fn primary_key (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.primary_key" , self . base)) } }
#[derive(Serialize, Default)]
struct DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesElMysqlTablesElDynamic { mysql_columns : Option < DynamicBlock < DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsEl >> , }
#[derive(Serialize)]
pub struct DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesElMysqlTablesEl { table : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] mysql_columns : Option < Vec < DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsEl > > , dynamic : DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesElMysqlTablesElDynamic , }
impl DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesElMysqlTablesEl {
    #[doc = "Set the field `mysql_columns`.\n"]
    pub fn set_mysql_columns(
        mut self,
        v : impl Into < BlockAssignable < DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.mysql_columns = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.mysql_columns = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesElMysqlTablesEl
{
    type O = BlockAssignable<
        DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesElMysqlTablesEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesElMysqlTablesEl {
    #[doc = "Table name."]
    pub table: PrimField<String>,
}
impl BuildDatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesElMysqlTablesEl {
    pub fn build(
        self,
    ) -> DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesElMysqlTablesEl {
        DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesElMysqlTablesEl {
            table: self.table,
            mysql_columns: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesElMysqlTablesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesElMysqlTablesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesElMysqlTablesElRef {
        DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesElMysqlTablesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesElMysqlTablesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `table` after provisioning.\nTable name."]
    pub fn table(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.table", self.base))
    }
    #[doc = "Get a reference to the value of field `mysql_columns` after provisioning.\n"]    pub fn mysql_columns (& self) -> ListRef < DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.mysql_columns", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesElDynamic {
    mysql_tables: Option<
        DynamicBlock<
            DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesElMysqlTablesEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesEl {
    database: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mysql_tables: Option<
        Vec<DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesElMysqlTablesEl>,
    >,
    dynamic: DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesElDynamic,
}
impl DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesEl {
    #[doc = "Set the field `mysql_tables`.\n"]
    pub fn set_mysql_tables(
        mut self,
        v: impl Into<
            BlockAssignable<
                DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesElMysqlTablesEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.mysql_tables = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.mysql_tables = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesEl {
    type O = BlockAssignable<DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesEl {
    #[doc = "Database name."]
    pub database: PrimField<String>,
}
impl BuildDatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesEl {
    pub fn build(self) -> DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesEl {
        DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesEl {
            database: self.database,
            mysql_tables: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesElRef {
        DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `database` after provisioning.\nDatabase name."]
    pub fn database(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.database", self.base))
    }
    #[doc = "Get a reference to the value of field `mysql_tables` after provisioning.\n"]
    pub fn mysql_tables(
        &self,
    ) -> ListRef<DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesElMysqlTablesElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.mysql_tables", self.base))
    }
}
#[derive(Serialize, Default)]
struct DatastreamStreamBackfillAllElMysqlExcludedObjectsElDynamic {
    mysql_databases:
        Option<DynamicBlock<DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesEl>>,
}
#[derive(Serialize)]
pub struct DatastreamStreamBackfillAllElMysqlExcludedObjectsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    mysql_databases:
        Option<Vec<DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesEl>>,
    dynamic: DatastreamStreamBackfillAllElMysqlExcludedObjectsElDynamic,
}
impl DatastreamStreamBackfillAllElMysqlExcludedObjectsEl {
    #[doc = "Set the field `mysql_databases`.\n"]
    pub fn set_mysql_databases(
        mut self,
        v: impl Into<
            BlockAssignable<DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.mysql_databases = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.mysql_databases = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DatastreamStreamBackfillAllElMysqlExcludedObjectsEl {
    type O = BlockAssignable<DatastreamStreamBackfillAllElMysqlExcludedObjectsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamBackfillAllElMysqlExcludedObjectsEl {}
impl BuildDatastreamStreamBackfillAllElMysqlExcludedObjectsEl {
    pub fn build(self) -> DatastreamStreamBackfillAllElMysqlExcludedObjectsEl {
        DatastreamStreamBackfillAllElMysqlExcludedObjectsEl {
            mysql_databases: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DatastreamStreamBackfillAllElMysqlExcludedObjectsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamBackfillAllElMysqlExcludedObjectsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamStreamBackfillAllElMysqlExcludedObjectsElRef {
        DatastreamStreamBackfillAllElMysqlExcludedObjectsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamBackfillAllElMysqlExcludedObjectsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `mysql_databases` after provisioning.\n"]
    pub fn mysql_databases(
        &self,
    ) -> ListRef<DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.mysql_databases", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasElOracleTablesElOracleColumnsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    column: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_type: Option<PrimField<String>>,
}
impl
    DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasElOracleTablesElOracleColumnsEl
{
    #[doc = "Set the field `column`.\nColumn name."]
    pub fn set_column(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.column = Some(v.into());
        self
    }
    #[doc = "Set the field `data_type`.\nThe Oracle data type. Full data types list can be found here:\nhttps://docs.oracle.com/en/database/oracle/oracle-database/21/sqlrf/Data-Types.html"]
    pub fn set_data_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.data_type = Some(v.into());
        self
    }
}
impl ToListMappable for DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasElOracleTablesElOracleColumnsEl { type O = BlockAssignable < DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasElOracleTablesElOracleColumnsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasElOracleTablesElOracleColumnsEl
{}
impl BuildDatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasElOracleTablesElOracleColumnsEl { pub fn build (self) -> DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasElOracleTablesElOracleColumnsEl { DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasElOracleTablesElOracleColumnsEl { column : core :: default :: Default :: default () , data_type : core :: default :: Default :: default () , } } }
pub struct DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasElOracleTablesElOracleColumnsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasElOracleTablesElOracleColumnsElRef { fn new (shared : StackShared , base : String) -> DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasElOracleTablesElOracleColumnsElRef { DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasElOracleTablesElOracleColumnsElRef { shared : shared , base : base . to_string () , } } }
impl DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasElOracleTablesElOracleColumnsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `column` after provisioning.\nColumn name."] pub fn column (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.column" , self . base)) } # [doc = "Get a reference to the value of field `data_type` after provisioning.\nThe Oracle data type. Full data types list can be found here:\nhttps://docs.oracle.com/en/database/oracle/oracle-database/21/sqlrf/Data-Types.html"] pub fn data_type (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.data_type" , self . base)) } # [doc = "Get a reference to the value of field `encoding` after provisioning.\nColumn encoding."] pub fn encoding (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.encoding" , self . base)) } # [doc = "Get a reference to the value of field `length` after provisioning.\nColumn length."] pub fn length (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.length" , self . base)) } # [doc = "Get a reference to the value of field `nullable` after provisioning.\nWhether or not the column can accept a null value."] pub fn nullable (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.nullable" , self . base)) } # [doc = "Get a reference to the value of field `ordinal_position` after provisioning.\nThe ordinal position of the column in the table."] pub fn ordinal_position (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.ordinal_position" , self . base)) } # [doc = "Get a reference to the value of field `precision` after provisioning.\nColumn precision."] pub fn precision (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.precision" , self . base)) } # [doc = "Get a reference to the value of field `primary_key` after provisioning.\nWhether or not the column represents a primary key."] pub fn primary_key (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.primary_key" , self . base)) } # [doc = "Get a reference to the value of field `scale` after provisioning.\nColumn scale."] pub fn scale (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.scale" , self . base)) } }
#[derive(Serialize, Default)]
struct DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasElOracleTablesElDynamic { oracle_columns : Option < DynamicBlock < DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasElOracleTablesElOracleColumnsEl >> , }
#[derive(Serialize)]
pub struct DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasElOracleTablesEl { table : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] oracle_columns : Option < Vec < DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasElOracleTablesElOracleColumnsEl > > , dynamic : DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasElOracleTablesElDynamic , }
impl DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasElOracleTablesEl {
    #[doc = "Set the field `oracle_columns`.\n"]
    pub fn set_oracle_columns(
        mut self,
        v : impl Into < BlockAssignable < DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasElOracleTablesElOracleColumnsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.oracle_columns = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.oracle_columns = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasElOracleTablesEl
{
    type O = BlockAssignable<
        DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasElOracleTablesEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasElOracleTablesEl {
    #[doc = "Table name."]
    pub table: PrimField<String>,
}
impl BuildDatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasElOracleTablesEl {
    pub fn build(
        self,
    ) -> DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasElOracleTablesEl {
        DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasElOracleTablesEl {
            table: self.table,
            oracle_columns: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasElOracleTablesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasElOracleTablesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasElOracleTablesElRef {
        DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasElOracleTablesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasElOracleTablesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `table` after provisioning.\nTable name."]
    pub fn table(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.table", self.base))
    }
    #[doc = "Get a reference to the value of field `oracle_columns` after provisioning.\n"]    pub fn oracle_columns (& self) -> ListRef < DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasElOracleTablesElOracleColumnsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.oracle_columns", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasElDynamic {
    oracle_tables: Option<
        DynamicBlock<
            DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasElOracleTablesEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasEl {
    schema: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oracle_tables: Option<
        Vec<DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasElOracleTablesEl>,
    >,
    dynamic: DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasElDynamic,
}
impl DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasEl {
    #[doc = "Set the field `oracle_tables`.\n"]
    pub fn set_oracle_tables(
        mut self,
        v: impl Into<
            BlockAssignable<
                DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasElOracleTablesEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.oracle_tables = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.oracle_tables = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasEl {
    type O = BlockAssignable<DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasEl {
    #[doc = "Schema name."]
    pub schema: PrimField<String>,
}
impl BuildDatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasEl {
    pub fn build(self) -> DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasEl {
        DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasEl {
            schema: self.schema,
            oracle_tables: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasElRef {
        DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `schema` after provisioning.\nSchema name."]
    pub fn schema(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.schema", self.base))
    }
    #[doc = "Get a reference to the value of field `oracle_tables` after provisioning.\n"]
    pub fn oracle_tables(
        &self,
    ) -> ListRef<DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasElOracleTablesElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.oracle_tables", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DatastreamStreamBackfillAllElOracleExcludedObjectsElDynamic {
    oracle_schemas:
        Option<DynamicBlock<DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasEl>>,
}
#[derive(Serialize)]
pub struct DatastreamStreamBackfillAllElOracleExcludedObjectsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    oracle_schemas:
        Option<Vec<DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasEl>>,
    dynamic: DatastreamStreamBackfillAllElOracleExcludedObjectsElDynamic,
}
impl DatastreamStreamBackfillAllElOracleExcludedObjectsEl {
    #[doc = "Set the field `oracle_schemas`.\n"]
    pub fn set_oracle_schemas(
        mut self,
        v: impl Into<
            BlockAssignable<DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.oracle_schemas = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.oracle_schemas = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DatastreamStreamBackfillAllElOracleExcludedObjectsEl {
    type O = BlockAssignable<DatastreamStreamBackfillAllElOracleExcludedObjectsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamBackfillAllElOracleExcludedObjectsEl {}
impl BuildDatastreamStreamBackfillAllElOracleExcludedObjectsEl {
    pub fn build(self) -> DatastreamStreamBackfillAllElOracleExcludedObjectsEl {
        DatastreamStreamBackfillAllElOracleExcludedObjectsEl {
            oracle_schemas: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DatastreamStreamBackfillAllElOracleExcludedObjectsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamBackfillAllElOracleExcludedObjectsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamStreamBackfillAllElOracleExcludedObjectsElRef {
        DatastreamStreamBackfillAllElOracleExcludedObjectsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamBackfillAllElOracleExcludedObjectsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `oracle_schemas` after provisioning.\n"]
    pub fn oracle_schemas(
        &self,
    ) -> ListRef<DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.oracle_schemas", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    column: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nullable: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ordinal_position: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    primary_key: Option<PrimField<bool>>,
}
impl DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsEl { # [doc = "Set the field `column`.\nColumn name."] pub fn set_column (mut self , v : impl Into < PrimField < String > >) -> Self { self . column = Some (v . into ()) ; self } # [doc = "Set the field `data_type`.\nThe PostgreSQL data type. Full data types list can be found here:\nhttps://www.postgresql.org/docs/current/datatype.html"] pub fn set_data_type (mut self , v : impl Into < PrimField < String > >) -> Self { self . data_type = Some (v . into ()) ; self } # [doc = "Set the field `nullable`.\nWhether or not the column can accept a null value."] pub fn set_nullable (mut self , v : impl Into < PrimField < bool > >) -> Self { self . nullable = Some (v . into ()) ; self } # [doc = "Set the field `ordinal_position`.\nThe ordinal position of the column in the table."] pub fn set_ordinal_position (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . ordinal_position = Some (v . into ()) ; self } # [doc = "Set the field `primary_key`.\nWhether or not the column represents a primary key."] pub fn set_primary_key (mut self , v : impl Into < PrimField < bool > >) -> Self { self . primary_key = Some (v . into ()) ; self } }
impl ToListMappable for DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsEl { type O = BlockAssignable < DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsEl
{}
impl BuildDatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsEl { pub fn build (self) -> DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsEl { DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsEl { column : core :: default :: Default :: default () , data_type : core :: default :: Default :: default () , nullable : core :: default :: Default :: default () , ordinal_position : core :: default :: Default :: default () , primary_key : core :: default :: Default :: default () , } } }
pub struct DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsElRef { fn new (shared : StackShared , base : String) -> DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsElRef { DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsElRef { shared : shared , base : base . to_string () , } } }
impl DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `column` after provisioning.\nColumn name."] pub fn column (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.column" , self . base)) } # [doc = "Get a reference to the value of field `data_type` after provisioning.\nThe PostgreSQL data type. Full data types list can be found here:\nhttps://www.postgresql.org/docs/current/datatype.html"] pub fn data_type (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.data_type" , self . base)) } # [doc = "Get a reference to the value of field `length` after provisioning.\nColumn length."] pub fn length (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.length" , self . base)) } # [doc = "Get a reference to the value of field `nullable` after provisioning.\nWhether or not the column can accept a null value."] pub fn nullable (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.nullable" , self . base)) } # [doc = "Get a reference to the value of field `ordinal_position` after provisioning.\nThe ordinal position of the column in the table."] pub fn ordinal_position (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.ordinal_position" , self . base)) } # [doc = "Get a reference to the value of field `precision` after provisioning.\nColumn precision."] pub fn precision (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.precision" , self . base)) } # [doc = "Get a reference to the value of field `primary_key` after provisioning.\nWhether or not the column represents a primary key."] pub fn primary_key (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.primary_key" , self . base)) } # [doc = "Get a reference to the value of field `scale` after provisioning.\nColumn scale."] pub fn scale (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.scale" , self . base)) } }
#[derive(Serialize, Default)]
struct DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasElPostgresqlTablesElDynamic { postgresql_columns : Option < DynamicBlock < DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsEl >> , }
#[derive(Serialize)]
pub struct DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasElPostgresqlTablesEl { table : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] postgresql_columns : Option < Vec < DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsEl > > , dynamic : DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasElPostgresqlTablesElDynamic , }
impl DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasElPostgresqlTablesEl {
    #[doc = "Set the field `postgresql_columns`.\n"]
    pub fn set_postgresql_columns(
        mut self,
        v : impl Into < BlockAssignable < DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.postgresql_columns = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.postgresql_columns = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasElPostgresqlTablesEl { type O = BlockAssignable < DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasElPostgresqlTablesEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasElPostgresqlTablesEl
{
    #[doc = "Table name."]
    pub table: PrimField<String>,
}
impl BuildDatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasElPostgresqlTablesEl { pub fn build (self) -> DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasElPostgresqlTablesEl { DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasElPostgresqlTablesEl { table : self . table , postgresql_columns : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasElPostgresqlTablesElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasElPostgresqlTablesElRef { fn new (shared : StackShared , base : String) -> DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasElPostgresqlTablesElRef { DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasElPostgresqlTablesElRef { shared : shared , base : base . to_string () , } } }
impl
    DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasElPostgresqlTablesElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `table` after provisioning.\nTable name."]
    pub fn table(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.table", self.base))
    }
    #[doc = "Get a reference to the value of field `postgresql_columns` after provisioning.\n"]    pub fn postgresql_columns (& self) -> ListRef < DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.postgresql_columns", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasElDynamic { postgresql_tables : Option < DynamicBlock < DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasElPostgresqlTablesEl >> , }
#[derive(Serialize)]
pub struct DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasEl { schema : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] postgresql_tables : Option < Vec < DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasElPostgresqlTablesEl > > , dynamic : DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasElDynamic , }
impl DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasEl {
    #[doc = "Set the field `postgresql_tables`.\n"]
    pub fn set_postgresql_tables(
        mut self,
        v : impl Into < BlockAssignable < DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasElPostgresqlTablesEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.postgresql_tables = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.postgresql_tables = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasEl
{
    type O = BlockAssignable<
        DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasEl {
    #[doc = "Database name."]
    pub schema: PrimField<String>,
}
impl BuildDatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasEl {
    pub fn build(
        self,
    ) -> DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasEl {
        DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasEl {
            schema: self.schema,
            postgresql_tables: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasElRef {
        DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `schema` after provisioning.\nDatabase name."]
    pub fn schema(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.schema", self.base))
    }
    #[doc = "Get a reference to the value of field `postgresql_tables` after provisioning.\n"]    pub fn postgresql_tables (& self) -> ListRef < DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasElPostgresqlTablesElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.postgresql_tables", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElDynamic {
    postgresql_schemas: Option<
        DynamicBlock<DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasEl>,
    >,
}
#[derive(Serialize)]
pub struct DatastreamStreamBackfillAllElPostgresqlExcludedObjectsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    postgresql_schemas:
        Option<Vec<DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasEl>>,
    dynamic: DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElDynamic,
}
impl DatastreamStreamBackfillAllElPostgresqlExcludedObjectsEl {
    #[doc = "Set the field `postgresql_schemas`.\n"]
    pub fn set_postgresql_schemas(
        mut self,
        v: impl Into<
            BlockAssignable<
                DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.postgresql_schemas = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.postgresql_schemas = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DatastreamStreamBackfillAllElPostgresqlExcludedObjectsEl {
    type O = BlockAssignable<DatastreamStreamBackfillAllElPostgresqlExcludedObjectsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamBackfillAllElPostgresqlExcludedObjectsEl {}
impl BuildDatastreamStreamBackfillAllElPostgresqlExcludedObjectsEl {
    pub fn build(self) -> DatastreamStreamBackfillAllElPostgresqlExcludedObjectsEl {
        DatastreamStreamBackfillAllElPostgresqlExcludedObjectsEl {
            postgresql_schemas: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElRef {
        DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `postgresql_schemas` after provisioning.\n"]
    pub fn postgresql_schemas(
        &self,
    ) -> ListRef<DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.postgresql_schemas", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DatastreamStreamBackfillAllElSalesforceExcludedObjectsElObjectsElFieldsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}
impl DatastreamStreamBackfillAllElSalesforceExcludedObjectsElObjectsElFieldsEl {
    #[doc = "Set the field `name`.\nField name."]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}
impl ToListMappable for DatastreamStreamBackfillAllElSalesforceExcludedObjectsElObjectsElFieldsEl {
    type O =
        BlockAssignable<DatastreamStreamBackfillAllElSalesforceExcludedObjectsElObjectsElFieldsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamBackfillAllElSalesforceExcludedObjectsElObjectsElFieldsEl {}
impl BuildDatastreamStreamBackfillAllElSalesforceExcludedObjectsElObjectsElFieldsEl {
    pub fn build(
        self,
    ) -> DatastreamStreamBackfillAllElSalesforceExcludedObjectsElObjectsElFieldsEl {
        DatastreamStreamBackfillAllElSalesforceExcludedObjectsElObjectsElFieldsEl {
            name: core::default::Default::default(),
        }
    }
}
pub struct DatastreamStreamBackfillAllElSalesforceExcludedObjectsElObjectsElFieldsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamBackfillAllElSalesforceExcludedObjectsElObjectsElFieldsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamStreamBackfillAllElSalesforceExcludedObjectsElObjectsElFieldsElRef {
        DatastreamStreamBackfillAllElSalesforceExcludedObjectsElObjectsElFieldsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamBackfillAllElSalesforceExcludedObjectsElObjectsElFieldsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nField name."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
#[derive(Serialize, Default)]
struct DatastreamStreamBackfillAllElSalesforceExcludedObjectsElObjectsElDynamic {
    fields: Option<
        DynamicBlock<DatastreamStreamBackfillAllElSalesforceExcludedObjectsElObjectsElFieldsEl>,
    >,
}
#[derive(Serialize)]
pub struct DatastreamStreamBackfillAllElSalesforceExcludedObjectsElObjectsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    object_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fields: Option<Vec<DatastreamStreamBackfillAllElSalesforceExcludedObjectsElObjectsElFieldsEl>>,
    dynamic: DatastreamStreamBackfillAllElSalesforceExcludedObjectsElObjectsElDynamic,
}
impl DatastreamStreamBackfillAllElSalesforceExcludedObjectsElObjectsEl {
    #[doc = "Set the field `object_name`.\nName of object in Salesforce Org."]
    pub fn set_object_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.object_name = Some(v.into());
        self
    }
    #[doc = "Set the field `fields`.\n"]
    pub fn set_fields(
        mut self,
        v: impl Into<
            BlockAssignable<
                DatastreamStreamBackfillAllElSalesforceExcludedObjectsElObjectsElFieldsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.fields = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.fields = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DatastreamStreamBackfillAllElSalesforceExcludedObjectsElObjectsEl {
    type O = BlockAssignable<DatastreamStreamBackfillAllElSalesforceExcludedObjectsElObjectsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamBackfillAllElSalesforceExcludedObjectsElObjectsEl {}
impl BuildDatastreamStreamBackfillAllElSalesforceExcludedObjectsElObjectsEl {
    pub fn build(self) -> DatastreamStreamBackfillAllElSalesforceExcludedObjectsElObjectsEl {
        DatastreamStreamBackfillAllElSalesforceExcludedObjectsElObjectsEl {
            object_name: core::default::Default::default(),
            fields: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DatastreamStreamBackfillAllElSalesforceExcludedObjectsElObjectsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamBackfillAllElSalesforceExcludedObjectsElObjectsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamStreamBackfillAllElSalesforceExcludedObjectsElObjectsElRef {
        DatastreamStreamBackfillAllElSalesforceExcludedObjectsElObjectsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamBackfillAllElSalesforceExcludedObjectsElObjectsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `object_name` after provisioning.\nName of object in Salesforce Org."]
    pub fn object_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.object_name", self.base))
    }
    #[doc = "Get a reference to the value of field `fields` after provisioning.\n"]
    pub fn fields(
        &self,
    ) -> ListRef<DatastreamStreamBackfillAllElSalesforceExcludedObjectsElObjectsElFieldsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.fields", self.base))
    }
}
#[derive(Serialize, Default)]
struct DatastreamStreamBackfillAllElSalesforceExcludedObjectsElDynamic {
    objects:
        Option<DynamicBlock<DatastreamStreamBackfillAllElSalesforceExcludedObjectsElObjectsEl>>,
}
#[derive(Serialize)]
pub struct DatastreamStreamBackfillAllElSalesforceExcludedObjectsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    objects: Option<Vec<DatastreamStreamBackfillAllElSalesforceExcludedObjectsElObjectsEl>>,
    dynamic: DatastreamStreamBackfillAllElSalesforceExcludedObjectsElDynamic,
}
impl DatastreamStreamBackfillAllElSalesforceExcludedObjectsEl {
    #[doc = "Set the field `objects`.\n"]
    pub fn set_objects(
        mut self,
        v: impl Into<BlockAssignable<DatastreamStreamBackfillAllElSalesforceExcludedObjectsElObjectsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.objects = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.objects = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DatastreamStreamBackfillAllElSalesforceExcludedObjectsEl {
    type O = BlockAssignable<DatastreamStreamBackfillAllElSalesforceExcludedObjectsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamBackfillAllElSalesforceExcludedObjectsEl {}
impl BuildDatastreamStreamBackfillAllElSalesforceExcludedObjectsEl {
    pub fn build(self) -> DatastreamStreamBackfillAllElSalesforceExcludedObjectsEl {
        DatastreamStreamBackfillAllElSalesforceExcludedObjectsEl {
            objects: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DatastreamStreamBackfillAllElSalesforceExcludedObjectsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamBackfillAllElSalesforceExcludedObjectsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamStreamBackfillAllElSalesforceExcludedObjectsElRef {
        DatastreamStreamBackfillAllElSalesforceExcludedObjectsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamBackfillAllElSalesforceExcludedObjectsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `objects` after provisioning.\n"]
    pub fn objects(
        &self,
    ) -> ListRef<DatastreamStreamBackfillAllElSalesforceExcludedObjectsElObjectsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.objects", self.base))
    }
}
#[derive(Serialize)]
pub struct DatastreamStreamBackfillAllElSpannerExcludedObjectsElSchemasElTablesElColumnsEl {
    column: PrimField<String>,
}
impl DatastreamStreamBackfillAllElSpannerExcludedObjectsElSchemasElTablesElColumnsEl {}
impl ToListMappable
    for DatastreamStreamBackfillAllElSpannerExcludedObjectsElSchemasElTablesElColumnsEl
{
    type O = BlockAssignable<
        DatastreamStreamBackfillAllElSpannerExcludedObjectsElSchemasElTablesElColumnsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamBackfillAllElSpannerExcludedObjectsElSchemasElTablesElColumnsEl {
    #[doc = "Column name."]
    pub column: PrimField<String>,
}
impl BuildDatastreamStreamBackfillAllElSpannerExcludedObjectsElSchemasElTablesElColumnsEl {
    pub fn build(
        self,
    ) -> DatastreamStreamBackfillAllElSpannerExcludedObjectsElSchemasElTablesElColumnsEl {
        DatastreamStreamBackfillAllElSpannerExcludedObjectsElSchemasElTablesElColumnsEl {
            column: self.column,
        }
    }
}
pub struct DatastreamStreamBackfillAllElSpannerExcludedObjectsElSchemasElTablesElColumnsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamBackfillAllElSpannerExcludedObjectsElSchemasElTablesElColumnsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamStreamBackfillAllElSpannerExcludedObjectsElSchemasElTablesElColumnsElRef {
        DatastreamStreamBackfillAllElSpannerExcludedObjectsElSchemasElTablesElColumnsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamBackfillAllElSpannerExcludedObjectsElSchemasElTablesElColumnsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `column` after provisioning.\nColumn name."]
    pub fn column(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.column", self.base))
    }
    #[doc = "Get a reference to the value of field `data_type` after provisioning.\nThe Spanner data type. Full data types list can be found here:\nhttps://docs.cloud.google.com/spanner/docs/reference/standard-sql/data-types"]
    pub fn data_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.data_type", self.base))
    }
    #[doc = "Get a reference to the value of field `is_primary_key` after provisioning.\nWhether the column is a primary key."]
    pub fn is_primary_key(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_primary_key", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ordinal_position` after provisioning.\nThe ordinal position of the column in the table."]
    pub fn ordinal_position(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ordinal_position", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DatastreamStreamBackfillAllElSpannerExcludedObjectsElSchemasElTablesElDynamic {
    columns: Option<
        DynamicBlock<
            DatastreamStreamBackfillAllElSpannerExcludedObjectsElSchemasElTablesElColumnsEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct DatastreamStreamBackfillAllElSpannerExcludedObjectsElSchemasElTablesEl {
    table: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    columns: Option<
        Vec<DatastreamStreamBackfillAllElSpannerExcludedObjectsElSchemasElTablesElColumnsEl>,
    >,
    dynamic: DatastreamStreamBackfillAllElSpannerExcludedObjectsElSchemasElTablesElDynamic,
}
impl DatastreamStreamBackfillAllElSpannerExcludedObjectsElSchemasElTablesEl {
    #[doc = "Set the field `columns`.\n"]
    pub fn set_columns(
        mut self,
        v: impl Into<
            BlockAssignable<
                DatastreamStreamBackfillAllElSpannerExcludedObjectsElSchemasElTablesElColumnsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.columns = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.columns = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DatastreamStreamBackfillAllElSpannerExcludedObjectsElSchemasElTablesEl {
    type O =
        BlockAssignable<DatastreamStreamBackfillAllElSpannerExcludedObjectsElSchemasElTablesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamBackfillAllElSpannerExcludedObjectsElSchemasElTablesEl {
    #[doc = "Table name."]
    pub table: PrimField<String>,
}
impl BuildDatastreamStreamBackfillAllElSpannerExcludedObjectsElSchemasElTablesEl {
    pub fn build(self) -> DatastreamStreamBackfillAllElSpannerExcludedObjectsElSchemasElTablesEl {
        DatastreamStreamBackfillAllElSpannerExcludedObjectsElSchemasElTablesEl {
            table: self.table,
            columns: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DatastreamStreamBackfillAllElSpannerExcludedObjectsElSchemasElTablesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamBackfillAllElSpannerExcludedObjectsElSchemasElTablesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamStreamBackfillAllElSpannerExcludedObjectsElSchemasElTablesElRef {
        DatastreamStreamBackfillAllElSpannerExcludedObjectsElSchemasElTablesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamBackfillAllElSpannerExcludedObjectsElSchemasElTablesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `table` after provisioning.\nTable name."]
    pub fn table(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.table", self.base))
    }
    #[doc = "Get a reference to the value of field `columns` after provisioning.\n"]
    pub fn columns(
        &self,
    ) -> ListRef<DatastreamStreamBackfillAllElSpannerExcludedObjectsElSchemasElTablesElColumnsElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.columns", self.base))
    }
}
#[derive(Serialize, Default)]
struct DatastreamStreamBackfillAllElSpannerExcludedObjectsElSchemasElDynamic {
    tables: Option<
        DynamicBlock<DatastreamStreamBackfillAllElSpannerExcludedObjectsElSchemasElTablesEl>,
    >,
}
#[derive(Serialize)]
pub struct DatastreamStreamBackfillAllElSpannerExcludedObjectsElSchemasEl {
    schema: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tables: Option<Vec<DatastreamStreamBackfillAllElSpannerExcludedObjectsElSchemasElTablesEl>>,
    dynamic: DatastreamStreamBackfillAllElSpannerExcludedObjectsElSchemasElDynamic,
}
impl DatastreamStreamBackfillAllElSpannerExcludedObjectsElSchemasEl {
    #[doc = "Set the field `tables`.\n"]
    pub fn set_tables(
        mut self,
        v: impl Into<
            BlockAssignable<DatastreamStreamBackfillAllElSpannerExcludedObjectsElSchemasElTablesEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.tables = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.tables = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DatastreamStreamBackfillAllElSpannerExcludedObjectsElSchemasEl {
    type O = BlockAssignable<DatastreamStreamBackfillAllElSpannerExcludedObjectsElSchemasEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamBackfillAllElSpannerExcludedObjectsElSchemasEl {
    #[doc = "Schema name."]
    pub schema: PrimField<String>,
}
impl BuildDatastreamStreamBackfillAllElSpannerExcludedObjectsElSchemasEl {
    pub fn build(self) -> DatastreamStreamBackfillAllElSpannerExcludedObjectsElSchemasEl {
        DatastreamStreamBackfillAllElSpannerExcludedObjectsElSchemasEl {
            schema: self.schema,
            tables: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DatastreamStreamBackfillAllElSpannerExcludedObjectsElSchemasElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamBackfillAllElSpannerExcludedObjectsElSchemasElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamStreamBackfillAllElSpannerExcludedObjectsElSchemasElRef {
        DatastreamStreamBackfillAllElSpannerExcludedObjectsElSchemasElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamBackfillAllElSpannerExcludedObjectsElSchemasElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `schema` after provisioning.\nSchema name."]
    pub fn schema(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.schema", self.base))
    }
    #[doc = "Get a reference to the value of field `tables` after provisioning.\n"]
    pub fn tables(
        &self,
    ) -> ListRef<DatastreamStreamBackfillAllElSpannerExcludedObjectsElSchemasElTablesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tables", self.base))
    }
}
#[derive(Serialize, Default)]
struct DatastreamStreamBackfillAllElSpannerExcludedObjectsElDynamic {
    schemas: Option<DynamicBlock<DatastreamStreamBackfillAllElSpannerExcludedObjectsElSchemasEl>>,
}
#[derive(Serialize)]
pub struct DatastreamStreamBackfillAllElSpannerExcludedObjectsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    schemas: Option<Vec<DatastreamStreamBackfillAllElSpannerExcludedObjectsElSchemasEl>>,
    dynamic: DatastreamStreamBackfillAllElSpannerExcludedObjectsElDynamic,
}
impl DatastreamStreamBackfillAllElSpannerExcludedObjectsEl {
    #[doc = "Set the field `schemas`.\n"]
    pub fn set_schemas(
        mut self,
        v: impl Into<BlockAssignable<DatastreamStreamBackfillAllElSpannerExcludedObjectsElSchemasEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.schemas = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.schemas = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DatastreamStreamBackfillAllElSpannerExcludedObjectsEl {
    type O = BlockAssignable<DatastreamStreamBackfillAllElSpannerExcludedObjectsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamBackfillAllElSpannerExcludedObjectsEl {}
impl BuildDatastreamStreamBackfillAllElSpannerExcludedObjectsEl {
    pub fn build(self) -> DatastreamStreamBackfillAllElSpannerExcludedObjectsEl {
        DatastreamStreamBackfillAllElSpannerExcludedObjectsEl {
            schemas: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DatastreamStreamBackfillAllElSpannerExcludedObjectsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamBackfillAllElSpannerExcludedObjectsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamStreamBackfillAllElSpannerExcludedObjectsElRef {
        DatastreamStreamBackfillAllElSpannerExcludedObjectsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamBackfillAllElSpannerExcludedObjectsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `schemas` after provisioning.\n"]
    pub fn schemas(
        &self,
    ) -> ListRef<DatastreamStreamBackfillAllElSpannerExcludedObjectsElSchemasElRef> {
        ListRef::new(self.shared().clone(), format!("{}.schemas", self.base))
    }
}
#[derive(Serialize)]
pub struct DatastreamStreamBackfillAllElSqlServerExcludedObjectsElSchemasElTablesElColumnsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    column: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_type: Option<PrimField<String>>,
}
impl DatastreamStreamBackfillAllElSqlServerExcludedObjectsElSchemasElTablesElColumnsEl {
    #[doc = "Set the field `column`.\nColumn name."]
    pub fn set_column(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.column = Some(v.into());
        self
    }
    #[doc = "Set the field `data_type`.\nThe SQL Server data type. Full data types list can be found here:\nhttps://learn.microsoft.com/en-us/sql/t-sql/data-types/data-types-transact-sql?view=sql-server-ver16"]
    pub fn set_data_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.data_type = Some(v.into());
        self
    }
}
impl ToListMappable
    for DatastreamStreamBackfillAllElSqlServerExcludedObjectsElSchemasElTablesElColumnsEl
{
    type O = BlockAssignable<
        DatastreamStreamBackfillAllElSqlServerExcludedObjectsElSchemasElTablesElColumnsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamBackfillAllElSqlServerExcludedObjectsElSchemasElTablesElColumnsEl {
}
impl BuildDatastreamStreamBackfillAllElSqlServerExcludedObjectsElSchemasElTablesElColumnsEl {
    pub fn build(
        self,
    ) -> DatastreamStreamBackfillAllElSqlServerExcludedObjectsElSchemasElTablesElColumnsEl {
        DatastreamStreamBackfillAllElSqlServerExcludedObjectsElSchemasElTablesElColumnsEl {
            column: core::default::Default::default(),
            data_type: core::default::Default::default(),
        }
    }
}
pub struct DatastreamStreamBackfillAllElSqlServerExcludedObjectsElSchemasElTablesElColumnsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamBackfillAllElSqlServerExcludedObjectsElSchemasElTablesElColumnsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamStreamBackfillAllElSqlServerExcludedObjectsElSchemasElTablesElColumnsElRef {
        DatastreamStreamBackfillAllElSqlServerExcludedObjectsElSchemasElTablesElColumnsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamBackfillAllElSqlServerExcludedObjectsElSchemasElTablesElColumnsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `column` after provisioning.\nColumn name."]
    pub fn column(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.column", self.base))
    }
    #[doc = "Get a reference to the value of field `data_type` after provisioning.\nThe SQL Server data type. Full data types list can be found here:\nhttps://learn.microsoft.com/en-us/sql/t-sql/data-types/data-types-transact-sql?view=sql-server-ver16"]
    pub fn data_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.data_type", self.base))
    }
    #[doc = "Get a reference to the value of field `length` after provisioning.\nColumn length."]
    pub fn length(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.length", self.base))
    }
    #[doc = "Get a reference to the value of field `nullable` after provisioning.\nWhether or not the column can accept a null value."]
    pub fn nullable(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.nullable", self.base))
    }
    #[doc = "Get a reference to the value of field `ordinal_position` after provisioning.\nThe ordinal position of the column in the table."]
    pub fn ordinal_position(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ordinal_position", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `precision` after provisioning.\nColumn precision."]
    pub fn precision(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.precision", self.base))
    }
    #[doc = "Get a reference to the value of field `primary_key` after provisioning.\nWhether or not the column represents a primary key."]
    pub fn primary_key(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.primary_key", self.base))
    }
    #[doc = "Get a reference to the value of field `scale` after provisioning.\nColumn scale."]
    pub fn scale(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.scale", self.base))
    }
}
#[derive(Serialize, Default)]
struct DatastreamStreamBackfillAllElSqlServerExcludedObjectsElSchemasElTablesElDynamic {
    columns: Option<
        DynamicBlock<
            DatastreamStreamBackfillAllElSqlServerExcludedObjectsElSchemasElTablesElColumnsEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct DatastreamStreamBackfillAllElSqlServerExcludedObjectsElSchemasElTablesEl {
    table: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    columns: Option<
        Vec<DatastreamStreamBackfillAllElSqlServerExcludedObjectsElSchemasElTablesElColumnsEl>,
    >,
    dynamic: DatastreamStreamBackfillAllElSqlServerExcludedObjectsElSchemasElTablesElDynamic,
}
impl DatastreamStreamBackfillAllElSqlServerExcludedObjectsElSchemasElTablesEl {
    #[doc = "Set the field `columns`.\n"]
    pub fn set_columns(
        mut self,
        v: impl Into<
            BlockAssignable<
                DatastreamStreamBackfillAllElSqlServerExcludedObjectsElSchemasElTablesElColumnsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.columns = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.columns = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DatastreamStreamBackfillAllElSqlServerExcludedObjectsElSchemasElTablesEl {
    type O =
        BlockAssignable<DatastreamStreamBackfillAllElSqlServerExcludedObjectsElSchemasElTablesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamBackfillAllElSqlServerExcludedObjectsElSchemasElTablesEl {
    #[doc = "Table name."]
    pub table: PrimField<String>,
}
impl BuildDatastreamStreamBackfillAllElSqlServerExcludedObjectsElSchemasElTablesEl {
    pub fn build(self) -> DatastreamStreamBackfillAllElSqlServerExcludedObjectsElSchemasElTablesEl {
        DatastreamStreamBackfillAllElSqlServerExcludedObjectsElSchemasElTablesEl {
            table: self.table,
            columns: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DatastreamStreamBackfillAllElSqlServerExcludedObjectsElSchemasElTablesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamBackfillAllElSqlServerExcludedObjectsElSchemasElTablesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamStreamBackfillAllElSqlServerExcludedObjectsElSchemasElTablesElRef {
        DatastreamStreamBackfillAllElSqlServerExcludedObjectsElSchemasElTablesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamBackfillAllElSqlServerExcludedObjectsElSchemasElTablesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `table` after provisioning.\nTable name."]
    pub fn table(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.table", self.base))
    }
    #[doc = "Get a reference to the value of field `columns` after provisioning.\n"]
    pub fn columns(
        &self,
    ) -> ListRef<DatastreamStreamBackfillAllElSqlServerExcludedObjectsElSchemasElTablesElColumnsElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.columns", self.base))
    }
}
#[derive(Serialize, Default)]
struct DatastreamStreamBackfillAllElSqlServerExcludedObjectsElSchemasElDynamic {
    tables: Option<
        DynamicBlock<DatastreamStreamBackfillAllElSqlServerExcludedObjectsElSchemasElTablesEl>,
    >,
}
#[derive(Serialize)]
pub struct DatastreamStreamBackfillAllElSqlServerExcludedObjectsElSchemasEl {
    schema: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tables: Option<Vec<DatastreamStreamBackfillAllElSqlServerExcludedObjectsElSchemasElTablesEl>>,
    dynamic: DatastreamStreamBackfillAllElSqlServerExcludedObjectsElSchemasElDynamic,
}
impl DatastreamStreamBackfillAllElSqlServerExcludedObjectsElSchemasEl {
    #[doc = "Set the field `tables`.\n"]
    pub fn set_tables(
        mut self,
        v: impl Into<
            BlockAssignable<
                DatastreamStreamBackfillAllElSqlServerExcludedObjectsElSchemasElTablesEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.tables = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.tables = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DatastreamStreamBackfillAllElSqlServerExcludedObjectsElSchemasEl {
    type O = BlockAssignable<DatastreamStreamBackfillAllElSqlServerExcludedObjectsElSchemasEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamBackfillAllElSqlServerExcludedObjectsElSchemasEl {
    #[doc = "Schema name."]
    pub schema: PrimField<String>,
}
impl BuildDatastreamStreamBackfillAllElSqlServerExcludedObjectsElSchemasEl {
    pub fn build(self) -> DatastreamStreamBackfillAllElSqlServerExcludedObjectsElSchemasEl {
        DatastreamStreamBackfillAllElSqlServerExcludedObjectsElSchemasEl {
            schema: self.schema,
            tables: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DatastreamStreamBackfillAllElSqlServerExcludedObjectsElSchemasElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamBackfillAllElSqlServerExcludedObjectsElSchemasElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamStreamBackfillAllElSqlServerExcludedObjectsElSchemasElRef {
        DatastreamStreamBackfillAllElSqlServerExcludedObjectsElSchemasElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamBackfillAllElSqlServerExcludedObjectsElSchemasElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `schema` after provisioning.\nSchema name."]
    pub fn schema(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.schema", self.base))
    }
    #[doc = "Get a reference to the value of field `tables` after provisioning.\n"]
    pub fn tables(
        &self,
    ) -> ListRef<DatastreamStreamBackfillAllElSqlServerExcludedObjectsElSchemasElTablesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tables", self.base))
    }
}
#[derive(Serialize, Default)]
struct DatastreamStreamBackfillAllElSqlServerExcludedObjectsElDynamic {
    schemas: Option<DynamicBlock<DatastreamStreamBackfillAllElSqlServerExcludedObjectsElSchemasEl>>,
}
#[derive(Serialize)]
pub struct DatastreamStreamBackfillAllElSqlServerExcludedObjectsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    schemas: Option<Vec<DatastreamStreamBackfillAllElSqlServerExcludedObjectsElSchemasEl>>,
    dynamic: DatastreamStreamBackfillAllElSqlServerExcludedObjectsElDynamic,
}
impl DatastreamStreamBackfillAllElSqlServerExcludedObjectsEl {
    #[doc = "Set the field `schemas`.\n"]
    pub fn set_schemas(
        mut self,
        v: impl Into<BlockAssignable<DatastreamStreamBackfillAllElSqlServerExcludedObjectsElSchemasEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.schemas = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.schemas = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DatastreamStreamBackfillAllElSqlServerExcludedObjectsEl {
    type O = BlockAssignable<DatastreamStreamBackfillAllElSqlServerExcludedObjectsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamBackfillAllElSqlServerExcludedObjectsEl {}
impl BuildDatastreamStreamBackfillAllElSqlServerExcludedObjectsEl {
    pub fn build(self) -> DatastreamStreamBackfillAllElSqlServerExcludedObjectsEl {
        DatastreamStreamBackfillAllElSqlServerExcludedObjectsEl {
            schemas: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DatastreamStreamBackfillAllElSqlServerExcludedObjectsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamBackfillAllElSqlServerExcludedObjectsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamStreamBackfillAllElSqlServerExcludedObjectsElRef {
        DatastreamStreamBackfillAllElSqlServerExcludedObjectsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamBackfillAllElSqlServerExcludedObjectsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `schemas` after provisioning.\n"]
    pub fn schemas(
        &self,
    ) -> ListRef<DatastreamStreamBackfillAllElSqlServerExcludedObjectsElSchemasElRef> {
        ListRef::new(self.shared().clone(), format!("{}.schemas", self.base))
    }
}
#[derive(Serialize, Default)]
struct DatastreamStreamBackfillAllElDynamic {
    mongodb_excluded_objects:
        Option<DynamicBlock<DatastreamStreamBackfillAllElMongodbExcludedObjectsEl>>,
    mysql_excluded_objects:
        Option<DynamicBlock<DatastreamStreamBackfillAllElMysqlExcludedObjectsEl>>,
    oracle_excluded_objects:
        Option<DynamicBlock<DatastreamStreamBackfillAllElOracleExcludedObjectsEl>>,
    postgresql_excluded_objects:
        Option<DynamicBlock<DatastreamStreamBackfillAllElPostgresqlExcludedObjectsEl>>,
    salesforce_excluded_objects:
        Option<DynamicBlock<DatastreamStreamBackfillAllElSalesforceExcludedObjectsEl>>,
    spanner_excluded_objects:
        Option<DynamicBlock<DatastreamStreamBackfillAllElSpannerExcludedObjectsEl>>,
    sql_server_excluded_objects:
        Option<DynamicBlock<DatastreamStreamBackfillAllElSqlServerExcludedObjectsEl>>,
}
#[derive(Serialize)]
pub struct DatastreamStreamBackfillAllEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    mongodb_excluded_objects: Option<Vec<DatastreamStreamBackfillAllElMongodbExcludedObjectsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mysql_excluded_objects: Option<Vec<DatastreamStreamBackfillAllElMysqlExcludedObjectsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oracle_excluded_objects: Option<Vec<DatastreamStreamBackfillAllElOracleExcludedObjectsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    postgresql_excluded_objects:
        Option<Vec<DatastreamStreamBackfillAllElPostgresqlExcludedObjectsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    salesforce_excluded_objects:
        Option<Vec<DatastreamStreamBackfillAllElSalesforceExcludedObjectsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spanner_excluded_objects: Option<Vec<DatastreamStreamBackfillAllElSpannerExcludedObjectsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sql_server_excluded_objects:
        Option<Vec<DatastreamStreamBackfillAllElSqlServerExcludedObjectsEl>>,
    dynamic: DatastreamStreamBackfillAllElDynamic,
}
impl DatastreamStreamBackfillAllEl {
    #[doc = "Set the field `mongodb_excluded_objects`.\n"]
    pub fn set_mongodb_excluded_objects(
        mut self,
        v: impl Into<BlockAssignable<DatastreamStreamBackfillAllElMongodbExcludedObjectsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.mongodb_excluded_objects = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.mongodb_excluded_objects = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `mysql_excluded_objects`.\n"]
    pub fn set_mysql_excluded_objects(
        mut self,
        v: impl Into<BlockAssignable<DatastreamStreamBackfillAllElMysqlExcludedObjectsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.mysql_excluded_objects = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.mysql_excluded_objects = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `oracle_excluded_objects`.\n"]
    pub fn set_oracle_excluded_objects(
        mut self,
        v: impl Into<BlockAssignable<DatastreamStreamBackfillAllElOracleExcludedObjectsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.oracle_excluded_objects = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.oracle_excluded_objects = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `postgresql_excluded_objects`.\n"]
    pub fn set_postgresql_excluded_objects(
        mut self,
        v: impl Into<BlockAssignable<DatastreamStreamBackfillAllElPostgresqlExcludedObjectsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.postgresql_excluded_objects = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.postgresql_excluded_objects = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `salesforce_excluded_objects`.\n"]
    pub fn set_salesforce_excluded_objects(
        mut self,
        v: impl Into<BlockAssignable<DatastreamStreamBackfillAllElSalesforceExcludedObjectsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.salesforce_excluded_objects = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.salesforce_excluded_objects = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `spanner_excluded_objects`.\n"]
    pub fn set_spanner_excluded_objects(
        mut self,
        v: impl Into<BlockAssignable<DatastreamStreamBackfillAllElSpannerExcludedObjectsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.spanner_excluded_objects = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.spanner_excluded_objects = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `sql_server_excluded_objects`.\n"]
    pub fn set_sql_server_excluded_objects(
        mut self,
        v: impl Into<BlockAssignable<DatastreamStreamBackfillAllElSqlServerExcludedObjectsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.sql_server_excluded_objects = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.sql_server_excluded_objects = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DatastreamStreamBackfillAllEl {
    type O = BlockAssignable<DatastreamStreamBackfillAllEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamBackfillAllEl {}
impl BuildDatastreamStreamBackfillAllEl {
    pub fn build(self) -> DatastreamStreamBackfillAllEl {
        DatastreamStreamBackfillAllEl {
            mongodb_excluded_objects: core::default::Default::default(),
            mysql_excluded_objects: core::default::Default::default(),
            oracle_excluded_objects: core::default::Default::default(),
            postgresql_excluded_objects: core::default::Default::default(),
            salesforce_excluded_objects: core::default::Default::default(),
            spanner_excluded_objects: core::default::Default::default(),
            sql_server_excluded_objects: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DatastreamStreamBackfillAllElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamBackfillAllElRef {
    fn new(shared: StackShared, base: String) -> DatastreamStreamBackfillAllElRef {
        DatastreamStreamBackfillAllElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamBackfillAllElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `mongodb_excluded_objects` after provisioning.\n"]
    pub fn mongodb_excluded_objects(
        &self,
    ) -> ListRef<DatastreamStreamBackfillAllElMongodbExcludedObjectsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.mongodb_excluded_objects", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `mysql_excluded_objects` after provisioning.\n"]
    pub fn mysql_excluded_objects(
        &self,
    ) -> ListRef<DatastreamStreamBackfillAllElMysqlExcludedObjectsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.mysql_excluded_objects", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `oracle_excluded_objects` after provisioning.\n"]
    pub fn oracle_excluded_objects(
        &self,
    ) -> ListRef<DatastreamStreamBackfillAllElOracleExcludedObjectsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.oracle_excluded_objects", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `postgresql_excluded_objects` after provisioning.\n"]
    pub fn postgresql_excluded_objects(
        &self,
    ) -> ListRef<DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.postgresql_excluded_objects", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `salesforce_excluded_objects` after provisioning.\n"]
    pub fn salesforce_excluded_objects(
        &self,
    ) -> ListRef<DatastreamStreamBackfillAllElSalesforceExcludedObjectsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.salesforce_excluded_objects", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `spanner_excluded_objects` after provisioning.\n"]
    pub fn spanner_excluded_objects(
        &self,
    ) -> ListRef<DatastreamStreamBackfillAllElSpannerExcludedObjectsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.spanner_excluded_objects", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `sql_server_excluded_objects` after provisioning.\n"]
    pub fn sql_server_excluded_objects(
        &self,
    ) -> ListRef<DatastreamStreamBackfillAllElSqlServerExcludedObjectsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.sql_server_excluded_objects", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DatastreamStreamBackfillNoneEl {}
impl DatastreamStreamBackfillNoneEl {}
impl ToListMappable for DatastreamStreamBackfillNoneEl {
    type O = BlockAssignable<DatastreamStreamBackfillNoneEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamBackfillNoneEl {}
impl BuildDatastreamStreamBackfillNoneEl {
    pub fn build(self) -> DatastreamStreamBackfillNoneEl {
        DatastreamStreamBackfillNoneEl {}
    }
}
pub struct DatastreamStreamBackfillNoneElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamBackfillNoneElRef {
    fn new(shared: StackShared, base: String) -> DatastreamStreamBackfillNoneElRef {
        DatastreamStreamBackfillNoneElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamBackfillNoneElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct DatastreamStreamDestinationConfigElBigqueryDestinationConfigElAppendOnlyEl {}
impl DatastreamStreamDestinationConfigElBigqueryDestinationConfigElAppendOnlyEl {}
impl ToListMappable for DatastreamStreamDestinationConfigElBigqueryDestinationConfigElAppendOnlyEl {
    type O =
        BlockAssignable<DatastreamStreamDestinationConfigElBigqueryDestinationConfigElAppendOnlyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamDestinationConfigElBigqueryDestinationConfigElAppendOnlyEl {}
impl BuildDatastreamStreamDestinationConfigElBigqueryDestinationConfigElAppendOnlyEl {
    pub fn build(
        self,
    ) -> DatastreamStreamDestinationConfigElBigqueryDestinationConfigElAppendOnlyEl {
        DatastreamStreamDestinationConfigElBigqueryDestinationConfigElAppendOnlyEl {}
    }
}
pub struct DatastreamStreamDestinationConfigElBigqueryDestinationConfigElAppendOnlyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamDestinationConfigElBigqueryDestinationConfigElAppendOnlyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamStreamDestinationConfigElBigqueryDestinationConfigElAppendOnlyElRef {
        DatastreamStreamDestinationConfigElBigqueryDestinationConfigElAppendOnlyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamDestinationConfigElBigqueryDestinationConfigElAppendOnlyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct DatastreamStreamDestinationConfigElBigqueryDestinationConfigElBlmtConfigEl {
    bucket: PrimField<String>,
    connection_name: PrimField<String>,
    file_format: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    root_path: Option<PrimField<String>>,
    table_format: PrimField<String>,
}
impl DatastreamStreamDestinationConfigElBigqueryDestinationConfigElBlmtConfigEl {
    #[doc = "Set the field `root_path`.\nThe root path inside the Cloud Storage bucket."]
    pub fn set_root_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.root_path = Some(v.into());
        self
    }
}
impl ToListMappable for DatastreamStreamDestinationConfigElBigqueryDestinationConfigElBlmtConfigEl {
    type O =
        BlockAssignable<DatastreamStreamDestinationConfigElBigqueryDestinationConfigElBlmtConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamDestinationConfigElBigqueryDestinationConfigElBlmtConfigEl {
    #[doc = "The Cloud Storage bucket name."]
    pub bucket: PrimField<String>,
    #[doc = "The bigquery connection. Format: '{project}.{location}.{name}'"]
    pub connection_name: PrimField<String>,
    #[doc = "The file format."]
    pub file_format: PrimField<String>,
    #[doc = "The table format."]
    pub table_format: PrimField<String>,
}
impl BuildDatastreamStreamDestinationConfigElBigqueryDestinationConfigElBlmtConfigEl {
    pub fn build(
        self,
    ) -> DatastreamStreamDestinationConfigElBigqueryDestinationConfigElBlmtConfigEl {
        DatastreamStreamDestinationConfigElBigqueryDestinationConfigElBlmtConfigEl {
            bucket: self.bucket,
            connection_name: self.connection_name,
            file_format: self.file_format,
            root_path: core::default::Default::default(),
            table_format: self.table_format,
        }
    }
}
pub struct DatastreamStreamDestinationConfigElBigqueryDestinationConfigElBlmtConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamDestinationConfigElBigqueryDestinationConfigElBlmtConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamStreamDestinationConfigElBigqueryDestinationConfigElBlmtConfigElRef {
        DatastreamStreamDestinationConfigElBigqueryDestinationConfigElBlmtConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamDestinationConfigElBigqueryDestinationConfigElBlmtConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bucket` after provisioning.\nThe Cloud Storage bucket name."]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }
    #[doc = "Get a reference to the value of field `connection_name` after provisioning.\nThe bigquery connection. Format: '{project}.{location}.{name}'"]
    pub fn connection_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.connection_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `file_format` after provisioning.\nThe file format."]
    pub fn file_format(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.file_format", self.base))
    }
    #[doc = "Get a reference to the value of field `root_path` after provisioning.\nThe root path inside the Cloud Storage bucket."]
    pub fn root_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.root_path", self.base))
    }
    #[doc = "Get a reference to the value of field `table_format` after provisioning.\nThe table format."]
    pub fn table_format(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.table_format", self.base))
    }
}
#[derive(Serialize)]
pub struct DatastreamStreamDestinationConfigElBigqueryDestinationConfigElMergeEl {}
impl DatastreamStreamDestinationConfigElBigqueryDestinationConfigElMergeEl {}
impl ToListMappable for DatastreamStreamDestinationConfigElBigqueryDestinationConfigElMergeEl {
    type O = BlockAssignable<DatastreamStreamDestinationConfigElBigqueryDestinationConfigElMergeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamDestinationConfigElBigqueryDestinationConfigElMergeEl {}
impl BuildDatastreamStreamDestinationConfigElBigqueryDestinationConfigElMergeEl {
    pub fn build(self) -> DatastreamStreamDestinationConfigElBigqueryDestinationConfigElMergeEl {
        DatastreamStreamDestinationConfigElBigqueryDestinationConfigElMergeEl {}
    }
}
pub struct DatastreamStreamDestinationConfigElBigqueryDestinationConfigElMergeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamDestinationConfigElBigqueryDestinationConfigElMergeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamStreamDestinationConfigElBigqueryDestinationConfigElMergeElRef {
        DatastreamStreamDestinationConfigElBigqueryDestinationConfigElMergeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamDestinationConfigElBigqueryDestinationConfigElMergeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSingleTargetDatasetEl {
    dataset_id: PrimField<String>,
}
impl DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSingleTargetDatasetEl {}
impl ToListMappable
    for DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSingleTargetDatasetEl
{
    type O = BlockAssignable<
        DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSingleTargetDatasetEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamDestinationConfigElBigqueryDestinationConfigElSingleTargetDatasetEl
{
    #[doc = "Dataset ID in the format projects/{project}/datasets/{dataset_id} or\n{project}:{dataset_id}"]
    pub dataset_id: PrimField<String>,
}
impl BuildDatastreamStreamDestinationConfigElBigqueryDestinationConfigElSingleTargetDatasetEl {
    pub fn build(
        self,
    ) -> DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSingleTargetDatasetEl {
        DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSingleTargetDatasetEl {
            dataset_id: self.dataset_id,
        }
    }
}
pub struct DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSingleTargetDatasetElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSingleTargetDatasetElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSingleTargetDatasetElRef
    {
        DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSingleTargetDatasetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSingleTargetDatasetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `dataset_id` after provisioning.\nDataset ID in the format projects/{project}/datasets/{dataset_id} or\n{project}:{dataset_id}"]
    pub fn dataset_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dataset_id", self.base))
    }
}
#[derive(Serialize)]
pub struct DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSourceHierarchyDatasetsElDatasetTemplateEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    dataset_id_prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_name: Option<PrimField<String>>,
    location: PrimField<String>,
}
impl DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSourceHierarchyDatasetsElDatasetTemplateEl { # [doc = "Set the field `dataset_id_prefix`.\nIf supplied, every created dataset will have its name prefixed by the provided value.\nThe prefix and name will be separated by an underscore. i.e. _."] pub fn set_dataset_id_prefix (mut self , v : impl Into < PrimField < String > >) -> Self { self . dataset_id_prefix = Some (v . into ()) ; self } # [doc = "Set the field `kms_key_name`.\nDescribes the Cloud KMS encryption key that will be used to protect destination BigQuery\ntable. The BigQuery Service Account associated with your project requires access to this\nencryption key. i.e. projects/{project}/locations/{location}/keyRings/{key_ring}/cryptoKeys/{cryptoKey}.\nSee https://cloud.google.com/bigquery/docs/customer-managed-encryption for more information."] pub fn set_kms_key_name (mut self , v : impl Into < PrimField < String > >) -> Self { self . kms_key_name = Some (v . into ()) ; self } }
impl ToListMappable for DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSourceHierarchyDatasetsElDatasetTemplateEl { type O = BlockAssignable < DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSourceHierarchyDatasetsElDatasetTemplateEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDatastreamStreamDestinationConfigElBigqueryDestinationConfigElSourceHierarchyDatasetsElDatasetTemplateEl
{
    #[doc = "The geographic location where the dataset should reside.\nSee https://cloud.google.com/bigquery/docs/locations for supported locations."]
    pub location: PrimField<String>,
}
impl BuildDatastreamStreamDestinationConfigElBigqueryDestinationConfigElSourceHierarchyDatasetsElDatasetTemplateEl { pub fn build (self) -> DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSourceHierarchyDatasetsElDatasetTemplateEl { DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSourceHierarchyDatasetsElDatasetTemplateEl { dataset_id_prefix : core :: default :: Default :: default () , kms_key_name : core :: default :: Default :: default () , location : self . location , } } }
pub struct DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSourceHierarchyDatasetsElDatasetTemplateElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSourceHierarchyDatasetsElDatasetTemplateElRef { fn new (shared : StackShared , base : String) -> DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSourceHierarchyDatasetsElDatasetTemplateElRef { DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSourceHierarchyDatasetsElDatasetTemplateElRef { shared : shared , base : base . to_string () , } } }
impl DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSourceHierarchyDatasetsElDatasetTemplateElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `dataset_id_prefix` after provisioning.\nIf supplied, every created dataset will have its name prefixed by the provided value.\nThe prefix and name will be separated by an underscore. i.e. _."] pub fn dataset_id_prefix (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.dataset_id_prefix" , self . base)) } # [doc = "Get a reference to the value of field `kms_key_name` after provisioning.\nDescribes the Cloud KMS encryption key that will be used to protect destination BigQuery\ntable. The BigQuery Service Account associated with your project requires access to this\nencryption key. i.e. projects/{project}/locations/{location}/keyRings/{key_ring}/cryptoKeys/{cryptoKey}.\nSee https://cloud.google.com/bigquery/docs/customer-managed-encryption for more information."] pub fn kms_key_name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.kms_key_name" , self . base)) } # [doc = "Get a reference to the value of field `location` after provisioning.\nThe geographic location where the dataset should reside.\nSee https://cloud.google.com/bigquery/docs/locations for supported locations."] pub fn location (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.location" , self . base)) } }
#[derive(Serialize, Default)]
struct DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSourceHierarchyDatasetsElDynamic { dataset_template : Option < DynamicBlock < DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSourceHierarchyDatasetsElDatasetTemplateEl >> , }
#[derive(Serialize)]
pub struct DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSourceHierarchyDatasetsEl { # [serde (skip_serializing_if = "Option::is_none")] project_id : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] dataset_template : Option < Vec < DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSourceHierarchyDatasetsElDatasetTemplateEl > > , dynamic : DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSourceHierarchyDatasetsElDynamic , }
impl DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSourceHierarchyDatasetsEl {
    #[doc = "Set the field `project_id`.\nOptional. The project id of the BigQuery dataset. If not specified, the project will be inferred from the stream resource."]
    pub fn set_project_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.project_id = Some(v.into());
        self
    }
    #[doc = "Set the field `dataset_template`.\n"]
    pub fn set_dataset_template(
        mut self,
        v : impl Into < BlockAssignable < DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSourceHierarchyDatasetsElDatasetTemplateEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.dataset_template = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.dataset_template = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSourceHierarchyDatasetsEl
{
    type O = BlockAssignable<
        DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSourceHierarchyDatasetsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamDestinationConfigElBigqueryDestinationConfigElSourceHierarchyDatasetsEl
{}
impl BuildDatastreamStreamDestinationConfigElBigqueryDestinationConfigElSourceHierarchyDatasetsEl {
    pub fn build(
        self,
    ) -> DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSourceHierarchyDatasetsEl
    {
        DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSourceHierarchyDatasetsEl {
            project_id: core::default::Default::default(),
            dataset_template: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSourceHierarchyDatasetsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSourceHierarchyDatasetsElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSourceHierarchyDatasetsElRef
    {
        DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSourceHierarchyDatasetsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSourceHierarchyDatasetsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `project_id` after provisioning.\nOptional. The project id of the BigQuery dataset. If not specified, the project will be inferred from the stream resource."]
    pub fn project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }
    #[doc = "Get a reference to the value of field `dataset_template` after provisioning.\n"]    pub fn dataset_template (& self) -> ListRef < DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSourceHierarchyDatasetsElDatasetTemplateElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.dataset_template", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DatastreamStreamDestinationConfigElBigqueryDestinationConfigElDynamic {
    append_only: Option<
        DynamicBlock<DatastreamStreamDestinationConfigElBigqueryDestinationConfigElAppendOnlyEl>,
    >,
    blmt_config: Option<
        DynamicBlock<DatastreamStreamDestinationConfigElBigqueryDestinationConfigElBlmtConfigEl>,
    >,
    merge:
        Option<DynamicBlock<DatastreamStreamDestinationConfigElBigqueryDestinationConfigElMergeEl>>,
    single_target_dataset: Option<
        DynamicBlock<
            DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSingleTargetDatasetEl,
        >,
    >,
    source_hierarchy_datasets: Option<
        DynamicBlock<
            DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSourceHierarchyDatasetsEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct DatastreamStreamDestinationConfigElBigqueryDestinationConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    data_freshness: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    append_only:
        Option<Vec<DatastreamStreamDestinationConfigElBigqueryDestinationConfigElAppendOnlyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    blmt_config:
        Option<Vec<DatastreamStreamDestinationConfigElBigqueryDestinationConfigElBlmtConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    merge: Option<Vec<DatastreamStreamDestinationConfigElBigqueryDestinationConfigElMergeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    single_target_dataset: Option<
        Vec<DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSingleTargetDatasetEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_hierarchy_datasets: Option<
        Vec<
            DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSourceHierarchyDatasetsEl,
        >,
    >,
    dynamic: DatastreamStreamDestinationConfigElBigqueryDestinationConfigElDynamic,
}
impl DatastreamStreamDestinationConfigElBigqueryDestinationConfigEl {
    #[doc = "Set the field `data_freshness`.\nThe guaranteed data freshness (in seconds) when querying tables created by the stream.\nEditing this field will only affect new tables created in the future, but existing tables\nwill not be impacted. Lower values mean that queries will return fresher data, but may result in higher cost.\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\". Defaults to 900s."]
    pub fn set_data_freshness(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.data_freshness = Some(v.into());
        self
    }
    #[doc = "Set the field `append_only`.\n"]
    pub fn set_append_only(
        mut self,
        v: impl Into<
            BlockAssignable<
                DatastreamStreamDestinationConfigElBigqueryDestinationConfigElAppendOnlyEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.append_only = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.append_only = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `blmt_config`.\n"]
    pub fn set_blmt_config(
        mut self,
        v: impl Into<
            BlockAssignable<
                DatastreamStreamDestinationConfigElBigqueryDestinationConfigElBlmtConfigEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.blmt_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.blmt_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `merge`.\n"]
    pub fn set_merge(
        mut self,
        v: impl Into<
            BlockAssignable<DatastreamStreamDestinationConfigElBigqueryDestinationConfigElMergeEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.merge = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.merge = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `single_target_dataset`.\n"]
    pub fn set_single_target_dataset(
        mut self,
        v: impl Into<
            BlockAssignable<
                DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSingleTargetDatasetEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.single_target_dataset = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.single_target_dataset = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `source_hierarchy_datasets`.\n"]
    pub fn set_source_hierarchy_datasets(
        mut self,
        v : impl Into < BlockAssignable < DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSourceHierarchyDatasetsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.source_hierarchy_datasets = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.source_hierarchy_datasets = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DatastreamStreamDestinationConfigElBigqueryDestinationConfigEl {
    type O = BlockAssignable<DatastreamStreamDestinationConfigElBigqueryDestinationConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamDestinationConfigElBigqueryDestinationConfigEl {}
impl BuildDatastreamStreamDestinationConfigElBigqueryDestinationConfigEl {
    pub fn build(self) -> DatastreamStreamDestinationConfigElBigqueryDestinationConfigEl {
        DatastreamStreamDestinationConfigElBigqueryDestinationConfigEl {
            data_freshness: core::default::Default::default(),
            append_only: core::default::Default::default(),
            blmt_config: core::default::Default::default(),
            merge: core::default::Default::default(),
            single_target_dataset: core::default::Default::default(),
            source_hierarchy_datasets: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DatastreamStreamDestinationConfigElBigqueryDestinationConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamDestinationConfigElBigqueryDestinationConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamStreamDestinationConfigElBigqueryDestinationConfigElRef {
        DatastreamStreamDestinationConfigElBigqueryDestinationConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamDestinationConfigElBigqueryDestinationConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `data_freshness` after provisioning.\nThe guaranteed data freshness (in seconds) when querying tables created by the stream.\nEditing this field will only affect new tables created in the future, but existing tables\nwill not be impacted. Lower values mean that queries will return fresher data, but may result in higher cost.\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\". Defaults to 900s."]
    pub fn data_freshness(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_freshness", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `append_only` after provisioning.\n"]
    pub fn append_only(
        &self,
    ) -> ListRef<DatastreamStreamDestinationConfigElBigqueryDestinationConfigElAppendOnlyElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.append_only", self.base))
    }
    #[doc = "Get a reference to the value of field `blmt_config` after provisioning.\n"]
    pub fn blmt_config(
        &self,
    ) -> ListRef<DatastreamStreamDestinationConfigElBigqueryDestinationConfigElBlmtConfigElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.blmt_config", self.base))
    }
    #[doc = "Get a reference to the value of field `merge` after provisioning.\n"]
    pub fn merge(
        &self,
    ) -> ListRef<DatastreamStreamDestinationConfigElBigqueryDestinationConfigElMergeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.merge", self.base))
    }
    #[doc = "Get a reference to the value of field `single_target_dataset` after provisioning.\n"]
    pub fn single_target_dataset(
        &self,
    ) -> ListRef<
        DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSingleTargetDatasetElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.single_target_dataset", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `source_hierarchy_datasets` after provisioning.\n"]
    pub fn source_hierarchy_datasets(
        &self,
    ) -> ListRef<
        DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSourceHierarchyDatasetsElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source_hierarchy_datasets", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DatastreamStreamDestinationConfigElGcsDestinationConfigElAvroFileFormatEl {}
impl DatastreamStreamDestinationConfigElGcsDestinationConfigElAvroFileFormatEl {}
impl ToListMappable for DatastreamStreamDestinationConfigElGcsDestinationConfigElAvroFileFormatEl {
    type O =
        BlockAssignable<DatastreamStreamDestinationConfigElGcsDestinationConfigElAvroFileFormatEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamDestinationConfigElGcsDestinationConfigElAvroFileFormatEl {}
impl BuildDatastreamStreamDestinationConfigElGcsDestinationConfigElAvroFileFormatEl {
    pub fn build(
        self,
    ) -> DatastreamStreamDestinationConfigElGcsDestinationConfigElAvroFileFormatEl {
        DatastreamStreamDestinationConfigElGcsDestinationConfigElAvroFileFormatEl {}
    }
}
pub struct DatastreamStreamDestinationConfigElGcsDestinationConfigElAvroFileFormatElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamDestinationConfigElGcsDestinationConfigElAvroFileFormatElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamStreamDestinationConfigElGcsDestinationConfigElAvroFileFormatElRef {
        DatastreamStreamDestinationConfigElGcsDestinationConfigElAvroFileFormatElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamDestinationConfigElGcsDestinationConfigElAvroFileFormatElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct DatastreamStreamDestinationConfigElGcsDestinationConfigElJsonFileFormatEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    compression: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schema_file_format: Option<PrimField<String>>,
}
impl DatastreamStreamDestinationConfigElGcsDestinationConfigElJsonFileFormatEl {
    #[doc = "Set the field `compression`.\nCompression of the loaded JSON file. Possible values: [\"NO_COMPRESSION\", \"GZIP\"]"]
    pub fn set_compression(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.compression = Some(v.into());
        self
    }
    #[doc = "Set the field `schema_file_format`.\nThe schema file format along JSON data files. Possible values: [\"NO_SCHEMA_FILE\", \"AVRO_SCHEMA_FILE\"]"]
    pub fn set_schema_file_format(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.schema_file_format = Some(v.into());
        self
    }
}
impl ToListMappable for DatastreamStreamDestinationConfigElGcsDestinationConfigElJsonFileFormatEl {
    type O =
        BlockAssignable<DatastreamStreamDestinationConfigElGcsDestinationConfigElJsonFileFormatEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamDestinationConfigElGcsDestinationConfigElJsonFileFormatEl {}
impl BuildDatastreamStreamDestinationConfigElGcsDestinationConfigElJsonFileFormatEl {
    pub fn build(
        self,
    ) -> DatastreamStreamDestinationConfigElGcsDestinationConfigElJsonFileFormatEl {
        DatastreamStreamDestinationConfigElGcsDestinationConfigElJsonFileFormatEl {
            compression: core::default::Default::default(),
            schema_file_format: core::default::Default::default(),
        }
    }
}
pub struct DatastreamStreamDestinationConfigElGcsDestinationConfigElJsonFileFormatElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamDestinationConfigElGcsDestinationConfigElJsonFileFormatElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamStreamDestinationConfigElGcsDestinationConfigElJsonFileFormatElRef {
        DatastreamStreamDestinationConfigElGcsDestinationConfigElJsonFileFormatElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamDestinationConfigElGcsDestinationConfigElJsonFileFormatElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `compression` after provisioning.\nCompression of the loaded JSON file. Possible values: [\"NO_COMPRESSION\", \"GZIP\"]"]
    pub fn compression(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.compression", self.base))
    }
    #[doc = "Get a reference to the value of field `schema_file_format` after provisioning.\nThe schema file format along JSON data files. Possible values: [\"NO_SCHEMA_FILE\", \"AVRO_SCHEMA_FILE\"]"]
    pub fn schema_file_format(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.schema_file_format", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DatastreamStreamDestinationConfigElGcsDestinationConfigElDynamic {
    avro_file_format: Option<
        DynamicBlock<DatastreamStreamDestinationConfigElGcsDestinationConfigElAvroFileFormatEl>,
    >,
    json_file_format: Option<
        DynamicBlock<DatastreamStreamDestinationConfigElGcsDestinationConfigElJsonFileFormatEl>,
    >,
}
#[derive(Serialize)]
pub struct DatastreamStreamDestinationConfigElGcsDestinationConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    file_rotation_interval: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file_rotation_mb: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    avro_file_format:
        Option<Vec<DatastreamStreamDestinationConfigElGcsDestinationConfigElAvroFileFormatEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    json_file_format:
        Option<Vec<DatastreamStreamDestinationConfigElGcsDestinationConfigElJsonFileFormatEl>>,
    dynamic: DatastreamStreamDestinationConfigElGcsDestinationConfigElDynamic,
}
impl DatastreamStreamDestinationConfigElGcsDestinationConfigEl {
    #[doc = "Set the field `file_rotation_interval`.\nThe maximum duration for which new events are added before a file is closed and a new file is created.\nValues within the range of 15-60 seconds are allowed.\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\"."]
    pub fn set_file_rotation_interval(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.file_rotation_interval = Some(v.into());
        self
    }
    #[doc = "Set the field `file_rotation_mb`.\nThe maximum file size to be saved in the bucket."]
    pub fn set_file_rotation_mb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.file_rotation_mb = Some(v.into());
        self
    }
    #[doc = "Set the field `path`.\nPath inside the Cloud Storage bucket to write data to."]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }
    #[doc = "Set the field `avro_file_format`.\n"]
    pub fn set_avro_file_format(
        mut self,
        v: impl Into<
            BlockAssignable<
                DatastreamStreamDestinationConfigElGcsDestinationConfigElAvroFileFormatEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.avro_file_format = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.avro_file_format = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `json_file_format`.\n"]
    pub fn set_json_file_format(
        mut self,
        v: impl Into<
            BlockAssignable<
                DatastreamStreamDestinationConfigElGcsDestinationConfigElJsonFileFormatEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.json_file_format = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.json_file_format = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DatastreamStreamDestinationConfigElGcsDestinationConfigEl {
    type O = BlockAssignable<DatastreamStreamDestinationConfigElGcsDestinationConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamDestinationConfigElGcsDestinationConfigEl {}
impl BuildDatastreamStreamDestinationConfigElGcsDestinationConfigEl {
    pub fn build(self) -> DatastreamStreamDestinationConfigElGcsDestinationConfigEl {
        DatastreamStreamDestinationConfigElGcsDestinationConfigEl {
            file_rotation_interval: core::default::Default::default(),
            file_rotation_mb: core::default::Default::default(),
            path: core::default::Default::default(),
            avro_file_format: core::default::Default::default(),
            json_file_format: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DatastreamStreamDestinationConfigElGcsDestinationConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamDestinationConfigElGcsDestinationConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamStreamDestinationConfigElGcsDestinationConfigElRef {
        DatastreamStreamDestinationConfigElGcsDestinationConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamDestinationConfigElGcsDestinationConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `file_rotation_interval` after provisioning.\nThe maximum duration for which new events are added before a file is closed and a new file is created.\nValues within the range of 15-60 seconds are allowed.\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\"."]
    pub fn file_rotation_interval(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.file_rotation_interval", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `file_rotation_mb` after provisioning.\nThe maximum file size to be saved in the bucket."]
    pub fn file_rotation_mb(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.file_rotation_mb", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `path` after provisioning.\nPath inside the Cloud Storage bucket to write data to."]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
    #[doc = "Get a reference to the value of field `avro_file_format` after provisioning.\n"]
    pub fn avro_file_format(
        &self,
    ) -> ListRef<DatastreamStreamDestinationConfigElGcsDestinationConfigElAvroFileFormatElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.avro_file_format", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `json_file_format` after provisioning.\n"]
    pub fn json_file_format(
        &self,
    ) -> ListRef<DatastreamStreamDestinationConfigElGcsDestinationConfigElJsonFileFormatElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.json_file_format", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DatastreamStreamDestinationConfigElDynamic {
    bigquery_destination_config:
        Option<DynamicBlock<DatastreamStreamDestinationConfigElBigqueryDestinationConfigEl>>,
    gcs_destination_config:
        Option<DynamicBlock<DatastreamStreamDestinationConfigElGcsDestinationConfigEl>>,
}
#[derive(Serialize)]
pub struct DatastreamStreamDestinationConfigEl {
    destination_connection_profile: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bigquery_destination_config:
        Option<Vec<DatastreamStreamDestinationConfigElBigqueryDestinationConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gcs_destination_config: Option<Vec<DatastreamStreamDestinationConfigElGcsDestinationConfigEl>>,
    dynamic: DatastreamStreamDestinationConfigElDynamic,
}
impl DatastreamStreamDestinationConfigEl {
    #[doc = "Set the field `bigquery_destination_config`.\n"]
    pub fn set_bigquery_destination_config(
        mut self,
        v: impl Into<BlockAssignable<DatastreamStreamDestinationConfigElBigqueryDestinationConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.bigquery_destination_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.bigquery_destination_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `gcs_destination_config`.\n"]
    pub fn set_gcs_destination_config(
        mut self,
        v: impl Into<BlockAssignable<DatastreamStreamDestinationConfigElGcsDestinationConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.gcs_destination_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.gcs_destination_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DatastreamStreamDestinationConfigEl {
    type O = BlockAssignable<DatastreamStreamDestinationConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamDestinationConfigEl {
    #[doc = "Destination connection profile resource. Format: projects/{project}/locations/{location}/connectionProfiles/{name}"]
    pub destination_connection_profile: PrimField<String>,
}
impl BuildDatastreamStreamDestinationConfigEl {
    pub fn build(self) -> DatastreamStreamDestinationConfigEl {
        DatastreamStreamDestinationConfigEl {
            destination_connection_profile: self.destination_connection_profile,
            bigquery_destination_config: core::default::Default::default(),
            gcs_destination_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DatastreamStreamDestinationConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamDestinationConfigElRef {
    fn new(shared: StackShared, base: String) -> DatastreamStreamDestinationConfigElRef {
        DatastreamStreamDestinationConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamDestinationConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `destination_connection_profile` after provisioning.\nDestination connection profile resource. Format: projects/{project}/locations/{location}/connectionProfiles/{name}"]
    pub fn destination_connection_profile(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.destination_connection_profile", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `bigquery_destination_config` after provisioning.\n"]
    pub fn bigquery_destination_config(
        &self,
    ) -> ListRef<DatastreamStreamDestinationConfigElBigqueryDestinationConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.bigquery_destination_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `gcs_destination_config` after provisioning.\n"]
    pub fn gcs_destination_config(
        &self,
    ) -> ListRef<DatastreamStreamDestinationConfigElGcsDestinationConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.gcs_destination_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DatastreamStreamRuleSetsElCustomizationRulesElBigqueryClusteringEl {
    columns: ListField<PrimField<String>>,
}
impl DatastreamStreamRuleSetsElCustomizationRulesElBigqueryClusteringEl {}
impl ToListMappable for DatastreamStreamRuleSetsElCustomizationRulesElBigqueryClusteringEl {
    type O = BlockAssignable<DatastreamStreamRuleSetsElCustomizationRulesElBigqueryClusteringEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamRuleSetsElCustomizationRulesElBigqueryClusteringEl {
    #[doc = "Column names to set as clustering columns."]
    pub columns: ListField<PrimField<String>>,
}
impl BuildDatastreamStreamRuleSetsElCustomizationRulesElBigqueryClusteringEl {
    pub fn build(self) -> DatastreamStreamRuleSetsElCustomizationRulesElBigqueryClusteringEl {
        DatastreamStreamRuleSetsElCustomizationRulesElBigqueryClusteringEl {
            columns: self.columns,
        }
    }
}
pub struct DatastreamStreamRuleSetsElCustomizationRulesElBigqueryClusteringElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamRuleSetsElCustomizationRulesElBigqueryClusteringElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamStreamRuleSetsElCustomizationRulesElBigqueryClusteringElRef {
        DatastreamStreamRuleSetsElCustomizationRulesElBigqueryClusteringElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamRuleSetsElCustomizationRulesElBigqueryClusteringElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `columns` after provisioning.\nColumn names to set as clustering columns."]
    pub fn columns(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.columns", self.base))
    }
}
#[derive(Serialize)]
pub struct DatastreamStreamRuleSetsElCustomizationRulesElBigqueryPartitioningElIngestionTimePartitionEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    partitioning_time_granularity: Option<PrimField<String>>,
}
impl DatastreamStreamRuleSetsElCustomizationRulesElBigqueryPartitioningElIngestionTimePartitionEl {
    #[doc = "Set the field `partitioning_time_granularity`.\nPartition granularity. Possible values: [\"PARTITIONING_TIME_GRANULARITY_UNSPECIFIED\", \"PARTITIONING_TIME_GRANULARITY_HOUR\", \"PARTITIONING_TIME_GRANULARITY_DAY\", \"PARTITIONING_TIME_GRANULARITY_MONTH\", \"PARTITIONING_TIME_GRANULARITY_YEAR\"]"]
    pub fn set_partitioning_time_granularity(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.partitioning_time_granularity = Some(v.into());
        self
    }
}
impl ToListMappable
    for DatastreamStreamRuleSetsElCustomizationRulesElBigqueryPartitioningElIngestionTimePartitionEl
{
    type O = BlockAssignable < DatastreamStreamRuleSetsElCustomizationRulesElBigqueryPartitioningElIngestionTimePartitionEl > ;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamRuleSetsElCustomizationRulesElBigqueryPartitioningElIngestionTimePartitionEl
{}
impl BuildDatastreamStreamRuleSetsElCustomizationRulesElBigqueryPartitioningElIngestionTimePartitionEl { pub fn build (self) -> DatastreamStreamRuleSetsElCustomizationRulesElBigqueryPartitioningElIngestionTimePartitionEl { DatastreamStreamRuleSetsElCustomizationRulesElBigqueryPartitioningElIngestionTimePartitionEl { partitioning_time_granularity : core :: default :: Default :: default () , } } }
pub struct DatastreamStreamRuleSetsElCustomizationRulesElBigqueryPartitioningElIngestionTimePartitionElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamRuleSetsElCustomizationRulesElBigqueryPartitioningElIngestionTimePartitionElRef { fn new (shared : StackShared , base : String) -> DatastreamStreamRuleSetsElCustomizationRulesElBigqueryPartitioningElIngestionTimePartitionElRef { DatastreamStreamRuleSetsElCustomizationRulesElBigqueryPartitioningElIngestionTimePartitionElRef { shared : shared , base : base . to_string () , } } }
impl
    DatastreamStreamRuleSetsElCustomizationRulesElBigqueryPartitioningElIngestionTimePartitionElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `partitioning_time_granularity` after provisioning.\nPartition granularity. Possible values: [\"PARTITIONING_TIME_GRANULARITY_UNSPECIFIED\", \"PARTITIONING_TIME_GRANULARITY_HOUR\", \"PARTITIONING_TIME_GRANULARITY_DAY\", \"PARTITIONING_TIME_GRANULARITY_MONTH\", \"PARTITIONING_TIME_GRANULARITY_YEAR\"]"]
    pub fn partitioning_time_granularity(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.partitioning_time_granularity", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DatastreamStreamRuleSetsElCustomizationRulesElBigqueryPartitioningElIntegerRangePartitionEl
{
    column: PrimField<String>,
    end: PrimField<f64>,
    interval: PrimField<f64>,
    start: PrimField<f64>,
}
impl DatastreamStreamRuleSetsElCustomizationRulesElBigqueryPartitioningElIntegerRangePartitionEl {}
impl ToListMappable
    for DatastreamStreamRuleSetsElCustomizationRulesElBigqueryPartitioningElIntegerRangePartitionEl
{
    type O = BlockAssignable<
        DatastreamStreamRuleSetsElCustomizationRulesElBigqueryPartitioningElIntegerRangePartitionEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamRuleSetsElCustomizationRulesElBigqueryPartitioningElIntegerRangePartitionEl
{
    #[doc = "The partitioning column."]
    pub column: PrimField<String>,
    #[doc = "The ending value for range partitioning (exclusive)."]
    pub end: PrimField<f64>,
    #[doc = "The interval of each range within the partition."]
    pub interval: PrimField<f64>,
    #[doc = "The starting value for range partitioning (inclusive)."]
    pub start: PrimField<f64>,
}
impl
    BuildDatastreamStreamRuleSetsElCustomizationRulesElBigqueryPartitioningElIntegerRangePartitionEl
{
    pub fn build(
        self,
    ) -> DatastreamStreamRuleSetsElCustomizationRulesElBigqueryPartitioningElIntegerRangePartitionEl
    {
        DatastreamStreamRuleSetsElCustomizationRulesElBigqueryPartitioningElIntegerRangePartitionEl { column : self . column , end : self . end , interval : self . interval , start : self . start , }
    }
}
pub struct DatastreamStreamRuleSetsElCustomizationRulesElBigqueryPartitioningElIntegerRangePartitionElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamRuleSetsElCustomizationRulesElBigqueryPartitioningElIntegerRangePartitionElRef { fn new (shared : StackShared , base : String) -> DatastreamStreamRuleSetsElCustomizationRulesElBigqueryPartitioningElIntegerRangePartitionElRef { DatastreamStreamRuleSetsElCustomizationRulesElBigqueryPartitioningElIntegerRangePartitionElRef { shared : shared , base : base . to_string () , } } }
impl
    DatastreamStreamRuleSetsElCustomizationRulesElBigqueryPartitioningElIntegerRangePartitionElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `column` after provisioning.\nThe partitioning column."]
    pub fn column(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.column", self.base))
    }
    #[doc = "Get a reference to the value of field `end` after provisioning.\nThe ending value for range partitioning (exclusive)."]
    pub fn end(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.end", self.base))
    }
    #[doc = "Get a reference to the value of field `interval` after provisioning.\nThe interval of each range within the partition."]
    pub fn interval(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.interval", self.base))
    }
    #[doc = "Get a reference to the value of field `start` after provisioning.\nThe starting value for range partitioning (inclusive)."]
    pub fn start(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.start", self.base))
    }
}
#[derive(Serialize)]
pub struct DatastreamStreamRuleSetsElCustomizationRulesElBigqueryPartitioningElTimeUnitPartitionEl {
    column: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    partitioning_time_granularity: Option<PrimField<String>>,
}
impl DatastreamStreamRuleSetsElCustomizationRulesElBigqueryPartitioningElTimeUnitPartitionEl {
    #[doc = "Set the field `partitioning_time_granularity`.\nPartition granularity. Possible values: [\"PARTITIONING_TIME_GRANULARITY_UNSPECIFIED\", \"PARTITIONING_TIME_GRANULARITY_HOUR\", \"PARTITIONING_TIME_GRANULARITY_DAY\", \"PARTITIONING_TIME_GRANULARITY_MONTH\", \"PARTITIONING_TIME_GRANULARITY_YEAR\"]"]
    pub fn set_partitioning_time_granularity(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.partitioning_time_granularity = Some(v.into());
        self
    }
}
impl ToListMappable
    for DatastreamStreamRuleSetsElCustomizationRulesElBigqueryPartitioningElTimeUnitPartitionEl
{
    type O = BlockAssignable<
        DatastreamStreamRuleSetsElCustomizationRulesElBigqueryPartitioningElTimeUnitPartitionEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamRuleSetsElCustomizationRulesElBigqueryPartitioningElTimeUnitPartitionEl
{
    #[doc = "The partitioning column."]
    pub column: PrimField<String>,
}
impl BuildDatastreamStreamRuleSetsElCustomizationRulesElBigqueryPartitioningElTimeUnitPartitionEl {
    pub fn build(
        self,
    ) -> DatastreamStreamRuleSetsElCustomizationRulesElBigqueryPartitioningElTimeUnitPartitionEl
    {
        DatastreamStreamRuleSetsElCustomizationRulesElBigqueryPartitioningElTimeUnitPartitionEl {
            column: self.column,
            partitioning_time_granularity: core::default::Default::default(),
        }
    }
}
pub struct DatastreamStreamRuleSetsElCustomizationRulesElBigqueryPartitioningElTimeUnitPartitionElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for DatastreamStreamRuleSetsElCustomizationRulesElBigqueryPartitioningElTimeUnitPartitionElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamStreamRuleSetsElCustomizationRulesElBigqueryPartitioningElTimeUnitPartitionElRef
    {
        DatastreamStreamRuleSetsElCustomizationRulesElBigqueryPartitioningElTimeUnitPartitionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamRuleSetsElCustomizationRulesElBigqueryPartitioningElTimeUnitPartitionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `column` after provisioning.\nThe partitioning column."]
    pub fn column(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.column", self.base))
    }
    #[doc = "Get a reference to the value of field `partitioning_time_granularity` after provisioning.\nPartition granularity. Possible values: [\"PARTITIONING_TIME_GRANULARITY_UNSPECIFIED\", \"PARTITIONING_TIME_GRANULARITY_HOUR\", \"PARTITIONING_TIME_GRANULARITY_DAY\", \"PARTITIONING_TIME_GRANULARITY_MONTH\", \"PARTITIONING_TIME_GRANULARITY_YEAR\"]"]
    pub fn partitioning_time_granularity(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.partitioning_time_granularity", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DatastreamStreamRuleSetsElCustomizationRulesElBigqueryPartitioningElDynamic { ingestion_time_partition : Option < DynamicBlock < DatastreamStreamRuleSetsElCustomizationRulesElBigqueryPartitioningElIngestionTimePartitionEl >> , integer_range_partition : Option < DynamicBlock < DatastreamStreamRuleSetsElCustomizationRulesElBigqueryPartitioningElIntegerRangePartitionEl >> , time_unit_partition : Option < DynamicBlock < DatastreamStreamRuleSetsElCustomizationRulesElBigqueryPartitioningElTimeUnitPartitionEl >> , }
#[derive(Serialize)]
pub struct DatastreamStreamRuleSetsElCustomizationRulesElBigqueryPartitioningEl { # [serde (skip_serializing_if = "Option::is_none")] require_partition_filter : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] ingestion_time_partition : Option < Vec < DatastreamStreamRuleSetsElCustomizationRulesElBigqueryPartitioningElIngestionTimePartitionEl > > , # [serde (skip_serializing_if = "Option::is_none")] integer_range_partition : Option < Vec < DatastreamStreamRuleSetsElCustomizationRulesElBigqueryPartitioningElIntegerRangePartitionEl > > , # [serde (skip_serializing_if = "Option::is_none")] time_unit_partition : Option < Vec < DatastreamStreamRuleSetsElCustomizationRulesElBigqueryPartitioningElTimeUnitPartitionEl > > , dynamic : DatastreamStreamRuleSetsElCustomizationRulesElBigqueryPartitioningElDynamic , }
impl DatastreamStreamRuleSetsElCustomizationRulesElBigqueryPartitioningEl {
    #[doc = "Set the field `require_partition_filter`.\nIf true, queries over the table require a partition filter."]
    pub fn set_require_partition_filter(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.require_partition_filter = Some(v.into());
        self
    }
    #[doc = "Set the field `ingestion_time_partition`.\n"]
    pub fn set_ingestion_time_partition(
        mut self,
        v : impl Into < BlockAssignable < DatastreamStreamRuleSetsElCustomizationRulesElBigqueryPartitioningElIngestionTimePartitionEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ingestion_time_partition = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ingestion_time_partition = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `integer_range_partition`.\n"]
    pub fn set_integer_range_partition(
        mut self,
        v : impl Into < BlockAssignable < DatastreamStreamRuleSetsElCustomizationRulesElBigqueryPartitioningElIntegerRangePartitionEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.integer_range_partition = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.integer_range_partition = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `time_unit_partition`.\n"]
    pub fn set_time_unit_partition(
        mut self,
        v : impl Into < BlockAssignable < DatastreamStreamRuleSetsElCustomizationRulesElBigqueryPartitioningElTimeUnitPartitionEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.time_unit_partition = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.time_unit_partition = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DatastreamStreamRuleSetsElCustomizationRulesElBigqueryPartitioningEl {
    type O = BlockAssignable<DatastreamStreamRuleSetsElCustomizationRulesElBigqueryPartitioningEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamRuleSetsElCustomizationRulesElBigqueryPartitioningEl {}
impl BuildDatastreamStreamRuleSetsElCustomizationRulesElBigqueryPartitioningEl {
    pub fn build(self) -> DatastreamStreamRuleSetsElCustomizationRulesElBigqueryPartitioningEl {
        DatastreamStreamRuleSetsElCustomizationRulesElBigqueryPartitioningEl {
            require_partition_filter: core::default::Default::default(),
            ingestion_time_partition: core::default::Default::default(),
            integer_range_partition: core::default::Default::default(),
            time_unit_partition: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DatastreamStreamRuleSetsElCustomizationRulesElBigqueryPartitioningElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamRuleSetsElCustomizationRulesElBigqueryPartitioningElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamStreamRuleSetsElCustomizationRulesElBigqueryPartitioningElRef {
        DatastreamStreamRuleSetsElCustomizationRulesElBigqueryPartitioningElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamRuleSetsElCustomizationRulesElBigqueryPartitioningElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `require_partition_filter` after provisioning.\nIf true, queries over the table require a partition filter."]
    pub fn require_partition_filter(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.require_partition_filter", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ingestion_time_partition` after provisioning.\n"]    pub fn ingestion_time_partition (& self) -> ListRef < DatastreamStreamRuleSetsElCustomizationRulesElBigqueryPartitioningElIngestionTimePartitionElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.ingestion_time_partition", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `integer_range_partition` after provisioning.\n"]    pub fn integer_range_partition (& self) -> ListRef < DatastreamStreamRuleSetsElCustomizationRulesElBigqueryPartitioningElIntegerRangePartitionElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.integer_range_partition", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `time_unit_partition` after provisioning.\n"]
    pub fn time_unit_partition(
        &self,
    ) -> ListRef<
        DatastreamStreamRuleSetsElCustomizationRulesElBigqueryPartitioningElTimeUnitPartitionElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.time_unit_partition", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DatastreamStreamRuleSetsElCustomizationRulesElDynamic {
    bigquery_clustering:
        Option<DynamicBlock<DatastreamStreamRuleSetsElCustomizationRulesElBigqueryClusteringEl>>,
    bigquery_partitioning:
        Option<DynamicBlock<DatastreamStreamRuleSetsElCustomizationRulesElBigqueryPartitioningEl>>,
}
#[derive(Serialize)]
pub struct DatastreamStreamRuleSetsElCustomizationRulesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bigquery_clustering:
        Option<Vec<DatastreamStreamRuleSetsElCustomizationRulesElBigqueryClusteringEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bigquery_partitioning:
        Option<Vec<DatastreamStreamRuleSetsElCustomizationRulesElBigqueryPartitioningEl>>,
    dynamic: DatastreamStreamRuleSetsElCustomizationRulesElDynamic,
}
impl DatastreamStreamRuleSetsElCustomizationRulesEl {
    #[doc = "Set the field `bigquery_clustering`.\n"]
    pub fn set_bigquery_clustering(
        mut self,
        v: impl Into<
            BlockAssignable<DatastreamStreamRuleSetsElCustomizationRulesElBigqueryClusteringEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.bigquery_clustering = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.bigquery_clustering = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `bigquery_partitioning`.\n"]
    pub fn set_bigquery_partitioning(
        mut self,
        v: impl Into<
            BlockAssignable<DatastreamStreamRuleSetsElCustomizationRulesElBigqueryPartitioningEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.bigquery_partitioning = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.bigquery_partitioning = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DatastreamStreamRuleSetsElCustomizationRulesEl {
    type O = BlockAssignable<DatastreamStreamRuleSetsElCustomizationRulesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamRuleSetsElCustomizationRulesEl {}
impl BuildDatastreamStreamRuleSetsElCustomizationRulesEl {
    pub fn build(self) -> DatastreamStreamRuleSetsElCustomizationRulesEl {
        DatastreamStreamRuleSetsElCustomizationRulesEl {
            bigquery_clustering: core::default::Default::default(),
            bigquery_partitioning: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DatastreamStreamRuleSetsElCustomizationRulesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamRuleSetsElCustomizationRulesElRef {
    fn new(shared: StackShared, base: String) -> DatastreamStreamRuleSetsElCustomizationRulesElRef {
        DatastreamStreamRuleSetsElCustomizationRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamRuleSetsElCustomizationRulesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bigquery_clustering` after provisioning.\n"]
    pub fn bigquery_clustering(
        &self,
    ) -> ListRef<DatastreamStreamRuleSetsElCustomizationRulesElBigqueryClusteringElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.bigquery_clustering", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `bigquery_partitioning` after provisioning.\n"]
    pub fn bigquery_partitioning(
        &self,
    ) -> ListRef<DatastreamStreamRuleSetsElCustomizationRulesElBigqueryPartitioningElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.bigquery_partitioning", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElMongodbIdentifierEl {
    collection: PrimField<String>,
    database: PrimField<String>,
}
impl DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElMongodbIdentifierEl {}
impl ToListMappable
    for DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElMongodbIdentifierEl
{
    type O = BlockAssignable<
        DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElMongodbIdentifierEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElMongodbIdentifierEl
{
    #[doc = "The MongoDB collection name."]
    pub collection: PrimField<String>,
    #[doc = "The MongoDB database name."]
    pub database: PrimField<String>,
}
impl BuildDatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElMongodbIdentifierEl {
    pub fn build(
        self,
    ) -> DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElMongodbIdentifierEl {
        DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElMongodbIdentifierEl {
            collection: self.collection,
            database: self.database,
        }
    }
}
pub struct DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElMongodbIdentifierElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElMongodbIdentifierElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElMongodbIdentifierElRef
    {
        DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElMongodbIdentifierElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElMongodbIdentifierElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `collection` after provisioning.\nThe MongoDB collection name."]
    pub fn collection(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.collection", self.base))
    }
    #[doc = "Get a reference to the value of field `database` after provisioning.\nThe MongoDB database name."]
    pub fn database(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.database", self.base))
    }
}
#[derive(Serialize)]
pub struct DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElMysqlIdentifierEl {
    database: PrimField<String>,
    table: PrimField<String>,
}
impl DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElMysqlIdentifierEl {}
impl ToListMappable
    for DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElMysqlIdentifierEl
{
    type O = BlockAssignable<
        DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElMysqlIdentifierEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElMysqlIdentifierEl {
    #[doc = "The database name."]
    pub database: PrimField<String>,
    #[doc = "The table name."]
    pub table: PrimField<String>,
}
impl BuildDatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElMysqlIdentifierEl {
    pub fn build(
        self,
    ) -> DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElMysqlIdentifierEl {
        DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElMysqlIdentifierEl {
            database: self.database,
            table: self.table,
        }
    }
}
pub struct DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElMysqlIdentifierElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElMysqlIdentifierElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElMysqlIdentifierElRef {
        DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElMysqlIdentifierElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElMysqlIdentifierElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `database` after provisioning.\nThe database name."]
    pub fn database(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.database", self.base))
    }
    #[doc = "Get a reference to the value of field `table` after provisioning.\nThe table name."]
    pub fn table(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.table", self.base))
    }
}
#[derive(Serialize)]
pub struct DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElOracleIdentifierEl {
    schema: PrimField<String>,
    table: PrimField<String>,
}
impl DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElOracleIdentifierEl {}
impl ToListMappable
    for DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElOracleIdentifierEl
{
    type O = BlockAssignable<
        DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElOracleIdentifierEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElOracleIdentifierEl {
    #[doc = "The schema name."]
    pub schema: PrimField<String>,
    #[doc = "The table name."]
    pub table: PrimField<String>,
}
impl BuildDatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElOracleIdentifierEl {
    pub fn build(
        self,
    ) -> DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElOracleIdentifierEl {
        DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElOracleIdentifierEl {
            schema: self.schema,
            table: self.table,
        }
    }
}
pub struct DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElOracleIdentifierElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElOracleIdentifierElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElOracleIdentifierElRef {
        DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElOracleIdentifierElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElOracleIdentifierElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `schema` after provisioning.\nThe schema name."]
    pub fn schema(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.schema", self.base))
    }
    #[doc = "Get a reference to the value of field `table` after provisioning.\nThe table name."]
    pub fn table(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.table", self.base))
    }
}
#[derive(Serialize)]
pub struct DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElPostgresqlIdentifierEl {
    schema: PrimField<String>,
    table: PrimField<String>,
}
impl DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElPostgresqlIdentifierEl {}
impl ToListMappable
    for DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElPostgresqlIdentifierEl
{
    type O = BlockAssignable<
        DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElPostgresqlIdentifierEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElPostgresqlIdentifierEl
{
    #[doc = "The schema name."]
    pub schema: PrimField<String>,
    #[doc = "The table name."]
    pub table: PrimField<String>,
}
impl BuildDatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElPostgresqlIdentifierEl {
    pub fn build(
        self,
    ) -> DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElPostgresqlIdentifierEl
    {
        DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElPostgresqlIdentifierEl {
            schema: self.schema,
            table: self.table,
        }
    }
}
pub struct DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElPostgresqlIdentifierElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElPostgresqlIdentifierElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElPostgresqlIdentifierElRef
    {
        DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElPostgresqlIdentifierElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElPostgresqlIdentifierElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `schema` after provisioning.\nThe schema name."]
    pub fn schema(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.schema", self.base))
    }
    #[doc = "Get a reference to the value of field `table` after provisioning.\nThe table name."]
    pub fn table(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.table", self.base))
    }
}
#[derive(Serialize)]
pub struct DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElSalesforceIdentifierEl {
    object_name: PrimField<String>,
}
impl DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElSalesforceIdentifierEl {}
impl ToListMappable
    for DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElSalesforceIdentifierEl
{
    type O = BlockAssignable<
        DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElSalesforceIdentifierEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElSalesforceIdentifierEl
{
    #[doc = "The Salesforce object name."]
    pub object_name: PrimField<String>,
}
impl BuildDatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElSalesforceIdentifierEl {
    pub fn build(
        self,
    ) -> DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElSalesforceIdentifierEl
    {
        DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElSalesforceIdentifierEl {
            object_name: self.object_name,
        }
    }
}
pub struct DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElSalesforceIdentifierElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElSalesforceIdentifierElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElSalesforceIdentifierElRef
    {
        DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElSalesforceIdentifierElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElSalesforceIdentifierElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `object_name` after provisioning.\nThe Salesforce object name."]
    pub fn object_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.object_name", self.base))
    }
}
#[derive(Serialize)]
pub struct DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElSpannerIdentifierEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    schema: Option<PrimField<String>>,
    table: PrimField<String>,
}
impl DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElSpannerIdentifierEl {
    #[doc = "Set the field `schema`.\nThe schema name."]
    pub fn set_schema(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.schema = Some(v.into());
        self
    }
}
impl ToListMappable
    for DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElSpannerIdentifierEl
{
    type O = BlockAssignable<
        DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElSpannerIdentifierEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElSpannerIdentifierEl
{
    #[doc = "The table name."]
    pub table: PrimField<String>,
}
impl BuildDatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElSpannerIdentifierEl {
    pub fn build(
        self,
    ) -> DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElSpannerIdentifierEl {
        DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElSpannerIdentifierEl {
            schema: core::default::Default::default(),
            table: self.table,
        }
    }
}
pub struct DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElSpannerIdentifierElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElSpannerIdentifierElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElSpannerIdentifierElRef
    {
        DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElSpannerIdentifierElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElSpannerIdentifierElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `schema` after provisioning.\nThe schema name."]
    pub fn schema(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.schema", self.base))
    }
    #[doc = "Get a reference to the value of field `table` after provisioning.\nThe table name."]
    pub fn table(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.table", self.base))
    }
}
#[derive(Serialize)]
pub struct DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElSqlServerIdentifierEl {
    schema: PrimField<String>,
    table: PrimField<String>,
}
impl DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElSqlServerIdentifierEl {}
impl ToListMappable
    for DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElSqlServerIdentifierEl
{
    type O = BlockAssignable<
        DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElSqlServerIdentifierEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElSqlServerIdentifierEl
{
    #[doc = "The schema name."]
    pub schema: PrimField<String>,
    #[doc = "The table name."]
    pub table: PrimField<String>,
}
impl BuildDatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElSqlServerIdentifierEl {
    pub fn build(
        self,
    ) -> DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElSqlServerIdentifierEl {
        DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElSqlServerIdentifierEl {
            schema: self.schema,
            table: self.table,
        }
    }
}
pub struct DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElSqlServerIdentifierElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElSqlServerIdentifierElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElSqlServerIdentifierElRef
    {
        DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElSqlServerIdentifierElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElSqlServerIdentifierElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `schema` after provisioning.\nThe schema name."]
    pub fn schema(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.schema", self.base))
    }
    #[doc = "Get a reference to the value of field `table` after provisioning.\nThe table name."]
    pub fn table(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.table", self.base))
    }
}
#[derive(Serialize, Default)]
struct DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElDynamic {
    mongodb_identifier: Option<
        DynamicBlock<
            DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElMongodbIdentifierEl,
        >,
    >,
    mysql_identifier: Option<
        DynamicBlock<
            DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElMysqlIdentifierEl,
        >,
    >,
    oracle_identifier: Option<
        DynamicBlock<
            DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElOracleIdentifierEl,
        >,
    >,
    postgresql_identifier: Option<
        DynamicBlock<
            DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElPostgresqlIdentifierEl,
        >,
    >,
    salesforce_identifier: Option<
        DynamicBlock<
            DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElSalesforceIdentifierEl,
        >,
    >,
    spanner_identifier: Option<
        DynamicBlock<
            DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElSpannerIdentifierEl,
        >,
    >,
    sql_server_identifier: Option<
        DynamicBlock<
            DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElSqlServerIdentifierEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    mongodb_identifier: Option<
        Vec<DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElMongodbIdentifierEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    mysql_identifier: Option<
        Vec<DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElMysqlIdentifierEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    oracle_identifier: Option<
        Vec<DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElOracleIdentifierEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    postgresql_identifier: Option<
        Vec<DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElPostgresqlIdentifierEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    salesforce_identifier: Option<
        Vec<DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElSalesforceIdentifierEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    spanner_identifier: Option<
        Vec<DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElSpannerIdentifierEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    sql_server_identifier: Option<
        Vec<DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElSqlServerIdentifierEl>,
    >,
    dynamic: DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElDynamic,
}
impl DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierEl {
    #[doc = "Set the field `mongodb_identifier`.\n"]
    pub fn set_mongodb_identifier(
        mut self,
        v: impl Into<
            BlockAssignable<
                DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElMongodbIdentifierEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.mongodb_identifier = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.mongodb_identifier = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `mysql_identifier`.\n"]
    pub fn set_mysql_identifier(
        mut self,
        v: impl Into<
            BlockAssignable<
                DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElMysqlIdentifierEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.mysql_identifier = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.mysql_identifier = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `oracle_identifier`.\n"]
    pub fn set_oracle_identifier(
        mut self,
        v: impl Into<
            BlockAssignable<
                DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElOracleIdentifierEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.oracle_identifier = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.oracle_identifier = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `postgresql_identifier`.\n"]
    pub fn set_postgresql_identifier(
        mut self,
        v : impl Into < BlockAssignable < DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElPostgresqlIdentifierEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.postgresql_identifier = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.postgresql_identifier = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `salesforce_identifier`.\n"]
    pub fn set_salesforce_identifier(
        mut self,
        v : impl Into < BlockAssignable < DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElSalesforceIdentifierEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.salesforce_identifier = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.salesforce_identifier = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `spanner_identifier`.\n"]
    pub fn set_spanner_identifier(
        mut self,
        v: impl Into<
            BlockAssignable<
                DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElSpannerIdentifierEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.spanner_identifier = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.spanner_identifier = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `sql_server_identifier`.\n"]
    pub fn set_sql_server_identifier(
        mut self,
        v : impl Into < BlockAssignable < DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElSqlServerIdentifierEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.sql_server_identifier = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.sql_server_identifier = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierEl {
    type O = BlockAssignable<DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierEl {}
impl BuildDatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierEl {
    pub fn build(self) -> DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierEl {
        DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierEl {
            mongodb_identifier: core::default::Default::default(),
            mysql_identifier: core::default::Default::default(),
            oracle_identifier: core::default::Default::default(),
            postgresql_identifier: core::default::Default::default(),
            salesforce_identifier: core::default::Default::default(),
            spanner_identifier: core::default::Default::default(),
            sql_server_identifier: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElRef {
        DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `mongodb_identifier` after provisioning.\n"]
    pub fn mongodb_identifier(
        &self,
    ) -> ListRef<
        DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElMongodbIdentifierElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.mongodb_identifier", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `mysql_identifier` after provisioning.\n"]
    pub fn mysql_identifier(
        &self,
    ) -> ListRef<DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElMysqlIdentifierElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.mysql_identifier", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `oracle_identifier` after provisioning.\n"]
    pub fn oracle_identifier(
        &self,
    ) -> ListRef<
        DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElOracleIdentifierElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.oracle_identifier", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `postgresql_identifier` after provisioning.\n"]
    pub fn postgresql_identifier(
        &self,
    ) -> ListRef<
        DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElPostgresqlIdentifierElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.postgresql_identifier", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `salesforce_identifier` after provisioning.\n"]
    pub fn salesforce_identifier(
        &self,
    ) -> ListRef<
        DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElSalesforceIdentifierElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.salesforce_identifier", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `spanner_identifier` after provisioning.\n"]
    pub fn spanner_identifier(
        &self,
    ) -> ListRef<
        DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElSpannerIdentifierElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.spanner_identifier", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `sql_server_identifier` after provisioning.\n"]
    pub fn sql_server_identifier(
        &self,
    ) -> ListRef<
        DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElSqlServerIdentifierElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.sql_server_identifier", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DatastreamStreamRuleSetsElObjectFilterElDynamic {
    source_object_identifier:
        Option<DynamicBlock<DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierEl>>,
}
#[derive(Serialize)]
pub struct DatastreamStreamRuleSetsElObjectFilterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    source_object_identifier:
        Option<Vec<DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierEl>>,
    dynamic: DatastreamStreamRuleSetsElObjectFilterElDynamic,
}
impl DatastreamStreamRuleSetsElObjectFilterEl {
    #[doc = "Set the field `source_object_identifier`.\n"]
    pub fn set_source_object_identifier(
        mut self,
        v: impl Into<BlockAssignable<DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.source_object_identifier = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.source_object_identifier = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DatastreamStreamRuleSetsElObjectFilterEl {
    type O = BlockAssignable<DatastreamStreamRuleSetsElObjectFilterEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamRuleSetsElObjectFilterEl {}
impl BuildDatastreamStreamRuleSetsElObjectFilterEl {
    pub fn build(self) -> DatastreamStreamRuleSetsElObjectFilterEl {
        DatastreamStreamRuleSetsElObjectFilterEl {
            source_object_identifier: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DatastreamStreamRuleSetsElObjectFilterElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamRuleSetsElObjectFilterElRef {
    fn new(shared: StackShared, base: String) -> DatastreamStreamRuleSetsElObjectFilterElRef {
        DatastreamStreamRuleSetsElObjectFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamRuleSetsElObjectFilterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `source_object_identifier` after provisioning.\n"]
    pub fn source_object_identifier(
        &self,
    ) -> ListRef<DatastreamStreamRuleSetsElObjectFilterElSourceObjectIdentifierElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source_object_identifier", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DatastreamStreamRuleSetsElDynamic {
    customization_rules: Option<DynamicBlock<DatastreamStreamRuleSetsElCustomizationRulesEl>>,
    object_filter: Option<DynamicBlock<DatastreamStreamRuleSetsElObjectFilterEl>>,
}
#[derive(Serialize)]
pub struct DatastreamStreamRuleSetsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    customization_rules: Option<Vec<DatastreamStreamRuleSetsElCustomizationRulesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    object_filter: Option<Vec<DatastreamStreamRuleSetsElObjectFilterEl>>,
    dynamic: DatastreamStreamRuleSetsElDynamic,
}
impl DatastreamStreamRuleSetsEl {
    #[doc = "Set the field `customization_rules`.\n"]
    pub fn set_customization_rules(
        mut self,
        v: impl Into<BlockAssignable<DatastreamStreamRuleSetsElCustomizationRulesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.customization_rules = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.customization_rules = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `object_filter`.\n"]
    pub fn set_object_filter(
        mut self,
        v: impl Into<BlockAssignable<DatastreamStreamRuleSetsElObjectFilterEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.object_filter = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.object_filter = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DatastreamStreamRuleSetsEl {
    type O = BlockAssignable<DatastreamStreamRuleSetsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamRuleSetsEl {}
impl BuildDatastreamStreamRuleSetsEl {
    pub fn build(self) -> DatastreamStreamRuleSetsEl {
        DatastreamStreamRuleSetsEl {
            customization_rules: core::default::Default::default(),
            object_filter: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DatastreamStreamRuleSetsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamRuleSetsElRef {
    fn new(shared: StackShared, base: String) -> DatastreamStreamRuleSetsElRef {
        DatastreamStreamRuleSetsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamRuleSetsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `customization_rules` after provisioning.\n"]
    pub fn customization_rules(
        &self,
    ) -> ListRef<DatastreamStreamRuleSetsElCustomizationRulesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.customization_rules", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `object_filter` after provisioning.\n"]
    pub fn object_filter(&self) -> ListRef<DatastreamStreamRuleSetsElObjectFilterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.object_filter", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElMongodbSourceConfigElExcludeObjectsElDatabasesElCollectionsElFieldsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    field: Option<PrimField<String>>,
}
impl DatastreamStreamSourceConfigElMongodbSourceConfigElExcludeObjectsElDatabasesElCollectionsElFieldsEl { # [doc = "Set the field `field`.\nField name."] pub fn set_field (mut self , v : impl Into < PrimField < String > >) -> Self { self . field = Some (v . into ()) ; self } }
impl ToListMappable for DatastreamStreamSourceConfigElMongodbSourceConfigElExcludeObjectsElDatabasesElCollectionsElFieldsEl { type O = BlockAssignable < DatastreamStreamSourceConfigElMongodbSourceConfigElExcludeObjectsElDatabasesElCollectionsElFieldsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDatastreamStreamSourceConfigElMongodbSourceConfigElExcludeObjectsElDatabasesElCollectionsElFieldsEl
{}
impl BuildDatastreamStreamSourceConfigElMongodbSourceConfigElExcludeObjectsElDatabasesElCollectionsElFieldsEl { pub fn build (self) -> DatastreamStreamSourceConfigElMongodbSourceConfigElExcludeObjectsElDatabasesElCollectionsElFieldsEl { DatastreamStreamSourceConfigElMongodbSourceConfigElExcludeObjectsElDatabasesElCollectionsElFieldsEl { field : core :: default :: Default :: default () , } } }
pub struct DatastreamStreamSourceConfigElMongodbSourceConfigElExcludeObjectsElDatabasesElCollectionsElFieldsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamSourceConfigElMongodbSourceConfigElExcludeObjectsElDatabasesElCollectionsElFieldsElRef { fn new (shared : StackShared , base : String) -> DatastreamStreamSourceConfigElMongodbSourceConfigElExcludeObjectsElDatabasesElCollectionsElFieldsElRef { DatastreamStreamSourceConfigElMongodbSourceConfigElExcludeObjectsElDatabasesElCollectionsElFieldsElRef { shared : shared , base : base . to_string () , } } }
impl DatastreamStreamSourceConfigElMongodbSourceConfigElExcludeObjectsElDatabasesElCollectionsElFieldsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `field` after provisioning.\nField name."] pub fn field (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.field" , self . base)) } }
#[derive(Serialize, Default)]
struct DatastreamStreamSourceConfigElMongodbSourceConfigElExcludeObjectsElDatabasesElCollectionsElDynamic { fields : Option < DynamicBlock < DatastreamStreamSourceConfigElMongodbSourceConfigElExcludeObjectsElDatabasesElCollectionsElFieldsEl >> , }
#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElMongodbSourceConfigElExcludeObjectsElDatabasesElCollectionsEl { # [serde (skip_serializing_if = "Option::is_none")] collection : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] fields : Option < Vec < DatastreamStreamSourceConfigElMongodbSourceConfigElExcludeObjectsElDatabasesElCollectionsElFieldsEl > > , dynamic : DatastreamStreamSourceConfigElMongodbSourceConfigElExcludeObjectsElDatabasesElCollectionsElDynamic , }
impl DatastreamStreamSourceConfigElMongodbSourceConfigElExcludeObjectsElDatabasesElCollectionsEl {
    #[doc = "Set the field `collection`.\nCollection name."]
    pub fn set_collection(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.collection = Some(v.into());
        self
    }
    #[doc = "Set the field `fields`.\n"]
    pub fn set_fields(
        mut self,
        v : impl Into < BlockAssignable < DatastreamStreamSourceConfigElMongodbSourceConfigElExcludeObjectsElDatabasesElCollectionsElFieldsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.fields = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.fields = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for DatastreamStreamSourceConfigElMongodbSourceConfigElExcludeObjectsElDatabasesElCollectionsEl
{
    type O = BlockAssignable<
        DatastreamStreamSourceConfigElMongodbSourceConfigElExcludeObjectsElDatabasesElCollectionsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamSourceConfigElMongodbSourceConfigElExcludeObjectsElDatabasesElCollectionsEl
{}
impl
    BuildDatastreamStreamSourceConfigElMongodbSourceConfigElExcludeObjectsElDatabasesElCollectionsEl
{
    pub fn build(
        self,
    ) -> DatastreamStreamSourceConfigElMongodbSourceConfigElExcludeObjectsElDatabasesElCollectionsEl
    {
        DatastreamStreamSourceConfigElMongodbSourceConfigElExcludeObjectsElDatabasesElCollectionsEl { collection : core :: default :: Default :: default () , fields : core :: default :: Default :: default () , dynamic : Default :: default () , }
    }
}
pub struct DatastreamStreamSourceConfigElMongodbSourceConfigElExcludeObjectsElDatabasesElCollectionsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamSourceConfigElMongodbSourceConfigElExcludeObjectsElDatabasesElCollectionsElRef { fn new (shared : StackShared , base : String) -> DatastreamStreamSourceConfigElMongodbSourceConfigElExcludeObjectsElDatabasesElCollectionsElRef { DatastreamStreamSourceConfigElMongodbSourceConfigElExcludeObjectsElDatabasesElCollectionsElRef { shared : shared , base : base . to_string () , } } }
impl
    DatastreamStreamSourceConfigElMongodbSourceConfigElExcludeObjectsElDatabasesElCollectionsElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `collection` after provisioning.\nCollection name."]
    pub fn collection(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.collection", self.base))
    }
    #[doc = "Get a reference to the value of field `fields` after provisioning.\n"]    pub fn fields (& self) -> ListRef < DatastreamStreamSourceConfigElMongodbSourceConfigElExcludeObjectsElDatabasesElCollectionsElFieldsElRef >{
        ListRef::new(self.shared().clone(), format!("{}.fields", self.base))
    }
}
#[derive(Serialize, Default)]
struct DatastreamStreamSourceConfigElMongodbSourceConfigElExcludeObjectsElDatabasesElDynamic { collections : Option < DynamicBlock < DatastreamStreamSourceConfigElMongodbSourceConfigElExcludeObjectsElDatabasesElCollectionsEl >> , }
#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElMongodbSourceConfigElExcludeObjectsElDatabasesEl { # [serde (skip_serializing_if = "Option::is_none")] database : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] collections : Option < Vec < DatastreamStreamSourceConfigElMongodbSourceConfigElExcludeObjectsElDatabasesElCollectionsEl > > , dynamic : DatastreamStreamSourceConfigElMongodbSourceConfigElExcludeObjectsElDatabasesElDynamic , }
impl DatastreamStreamSourceConfigElMongodbSourceConfigElExcludeObjectsElDatabasesEl {
    #[doc = "Set the field `database`.\nDatabase name."]
    pub fn set_database(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.database = Some(v.into());
        self
    }
    #[doc = "Set the field `collections`.\n"]
    pub fn set_collections(
        mut self,
        v : impl Into < BlockAssignable < DatastreamStreamSourceConfigElMongodbSourceConfigElExcludeObjectsElDatabasesElCollectionsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.collections = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.collections = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for DatastreamStreamSourceConfigElMongodbSourceConfigElExcludeObjectsElDatabasesEl
{
    type O = BlockAssignable<
        DatastreamStreamSourceConfigElMongodbSourceConfigElExcludeObjectsElDatabasesEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamSourceConfigElMongodbSourceConfigElExcludeObjectsElDatabasesEl {}
impl BuildDatastreamStreamSourceConfigElMongodbSourceConfigElExcludeObjectsElDatabasesEl {
    pub fn build(
        self,
    ) -> DatastreamStreamSourceConfigElMongodbSourceConfigElExcludeObjectsElDatabasesEl {
        DatastreamStreamSourceConfigElMongodbSourceConfigElExcludeObjectsElDatabasesEl {
            database: core::default::Default::default(),
            collections: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DatastreamStreamSourceConfigElMongodbSourceConfigElExcludeObjectsElDatabasesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamSourceConfigElMongodbSourceConfigElExcludeObjectsElDatabasesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamStreamSourceConfigElMongodbSourceConfigElExcludeObjectsElDatabasesElRef {
        DatastreamStreamSourceConfigElMongodbSourceConfigElExcludeObjectsElDatabasesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamSourceConfigElMongodbSourceConfigElExcludeObjectsElDatabasesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `database` after provisioning.\nDatabase name."]
    pub fn database(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.database", self.base))
    }
    #[doc = "Get a reference to the value of field `collections` after provisioning.\n"]    pub fn collections (& self) -> ListRef < DatastreamStreamSourceConfigElMongodbSourceConfigElExcludeObjectsElDatabasesElCollectionsElRef >{
        ListRef::new(self.shared().clone(), format!("{}.collections", self.base))
    }
}
#[derive(Serialize, Default)]
struct DatastreamStreamSourceConfigElMongodbSourceConfigElExcludeObjectsElDynamic {
    databases: Option<
        DynamicBlock<
            DatastreamStreamSourceConfigElMongodbSourceConfigElExcludeObjectsElDatabasesEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElMongodbSourceConfigElExcludeObjectsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    databases:
        Option<Vec<DatastreamStreamSourceConfigElMongodbSourceConfigElExcludeObjectsElDatabasesEl>>,
    dynamic: DatastreamStreamSourceConfigElMongodbSourceConfigElExcludeObjectsElDynamic,
}
impl DatastreamStreamSourceConfigElMongodbSourceConfigElExcludeObjectsEl {
    #[doc = "Set the field `databases`.\n"]
    pub fn set_databases(
        mut self,
        v: impl Into<
            BlockAssignable<
                DatastreamStreamSourceConfigElMongodbSourceConfigElExcludeObjectsElDatabasesEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.databases = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.databases = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DatastreamStreamSourceConfigElMongodbSourceConfigElExcludeObjectsEl {
    type O = BlockAssignable<DatastreamStreamSourceConfigElMongodbSourceConfigElExcludeObjectsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamSourceConfigElMongodbSourceConfigElExcludeObjectsEl {}
impl BuildDatastreamStreamSourceConfigElMongodbSourceConfigElExcludeObjectsEl {
    pub fn build(self) -> DatastreamStreamSourceConfigElMongodbSourceConfigElExcludeObjectsEl {
        DatastreamStreamSourceConfigElMongodbSourceConfigElExcludeObjectsEl {
            databases: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DatastreamStreamSourceConfigElMongodbSourceConfigElExcludeObjectsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamSourceConfigElMongodbSourceConfigElExcludeObjectsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamStreamSourceConfigElMongodbSourceConfigElExcludeObjectsElRef {
        DatastreamStreamSourceConfigElMongodbSourceConfigElExcludeObjectsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamSourceConfigElMongodbSourceConfigElExcludeObjectsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `databases` after provisioning.\n"]
    pub fn databases(
        &self,
    ) -> ListRef<DatastreamStreamSourceConfigElMongodbSourceConfigElExcludeObjectsElDatabasesElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.databases", self.base))
    }
}
#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElMongodbSourceConfigElIncludeObjectsElDatabasesElCollectionsElFieldsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    field: Option<PrimField<String>>,
}
impl DatastreamStreamSourceConfigElMongodbSourceConfigElIncludeObjectsElDatabasesElCollectionsElFieldsEl { # [doc = "Set the field `field`.\nField name."] pub fn set_field (mut self , v : impl Into < PrimField < String > >) -> Self { self . field = Some (v . into ()) ; self } }
impl ToListMappable for DatastreamStreamSourceConfigElMongodbSourceConfigElIncludeObjectsElDatabasesElCollectionsElFieldsEl { type O = BlockAssignable < DatastreamStreamSourceConfigElMongodbSourceConfigElIncludeObjectsElDatabasesElCollectionsElFieldsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDatastreamStreamSourceConfigElMongodbSourceConfigElIncludeObjectsElDatabasesElCollectionsElFieldsEl
{}
impl BuildDatastreamStreamSourceConfigElMongodbSourceConfigElIncludeObjectsElDatabasesElCollectionsElFieldsEl { pub fn build (self) -> DatastreamStreamSourceConfigElMongodbSourceConfigElIncludeObjectsElDatabasesElCollectionsElFieldsEl { DatastreamStreamSourceConfigElMongodbSourceConfigElIncludeObjectsElDatabasesElCollectionsElFieldsEl { field : core :: default :: Default :: default () , } } }
pub struct DatastreamStreamSourceConfigElMongodbSourceConfigElIncludeObjectsElDatabasesElCollectionsElFieldsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamSourceConfigElMongodbSourceConfigElIncludeObjectsElDatabasesElCollectionsElFieldsElRef { fn new (shared : StackShared , base : String) -> DatastreamStreamSourceConfigElMongodbSourceConfigElIncludeObjectsElDatabasesElCollectionsElFieldsElRef { DatastreamStreamSourceConfigElMongodbSourceConfigElIncludeObjectsElDatabasesElCollectionsElFieldsElRef { shared : shared , base : base . to_string () , } } }
impl DatastreamStreamSourceConfigElMongodbSourceConfigElIncludeObjectsElDatabasesElCollectionsElFieldsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `field` after provisioning.\nField name."] pub fn field (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.field" , self . base)) } }
#[derive(Serialize, Default)]
struct DatastreamStreamSourceConfigElMongodbSourceConfigElIncludeObjectsElDatabasesElCollectionsElDynamic { fields : Option < DynamicBlock < DatastreamStreamSourceConfigElMongodbSourceConfigElIncludeObjectsElDatabasesElCollectionsElFieldsEl >> , }
#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElMongodbSourceConfigElIncludeObjectsElDatabasesElCollectionsEl { # [serde (skip_serializing_if = "Option::is_none")] collection : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] fields : Option < Vec < DatastreamStreamSourceConfigElMongodbSourceConfigElIncludeObjectsElDatabasesElCollectionsElFieldsEl > > , dynamic : DatastreamStreamSourceConfigElMongodbSourceConfigElIncludeObjectsElDatabasesElCollectionsElDynamic , }
impl DatastreamStreamSourceConfigElMongodbSourceConfigElIncludeObjectsElDatabasesElCollectionsEl {
    #[doc = "Set the field `collection`.\nCollection name."]
    pub fn set_collection(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.collection = Some(v.into());
        self
    }
    #[doc = "Set the field `fields`.\n"]
    pub fn set_fields(
        mut self,
        v : impl Into < BlockAssignable < DatastreamStreamSourceConfigElMongodbSourceConfigElIncludeObjectsElDatabasesElCollectionsElFieldsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.fields = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.fields = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for DatastreamStreamSourceConfigElMongodbSourceConfigElIncludeObjectsElDatabasesElCollectionsEl
{
    type O = BlockAssignable<
        DatastreamStreamSourceConfigElMongodbSourceConfigElIncludeObjectsElDatabasesElCollectionsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamSourceConfigElMongodbSourceConfigElIncludeObjectsElDatabasesElCollectionsEl
{}
impl
    BuildDatastreamStreamSourceConfigElMongodbSourceConfigElIncludeObjectsElDatabasesElCollectionsEl
{
    pub fn build(
        self,
    ) -> DatastreamStreamSourceConfigElMongodbSourceConfigElIncludeObjectsElDatabasesElCollectionsEl
    {
        DatastreamStreamSourceConfigElMongodbSourceConfigElIncludeObjectsElDatabasesElCollectionsEl { collection : core :: default :: Default :: default () , fields : core :: default :: Default :: default () , dynamic : Default :: default () , }
    }
}
pub struct DatastreamStreamSourceConfigElMongodbSourceConfigElIncludeObjectsElDatabasesElCollectionsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamSourceConfigElMongodbSourceConfigElIncludeObjectsElDatabasesElCollectionsElRef { fn new (shared : StackShared , base : String) -> DatastreamStreamSourceConfigElMongodbSourceConfigElIncludeObjectsElDatabasesElCollectionsElRef { DatastreamStreamSourceConfigElMongodbSourceConfigElIncludeObjectsElDatabasesElCollectionsElRef { shared : shared , base : base . to_string () , } } }
impl
    DatastreamStreamSourceConfigElMongodbSourceConfigElIncludeObjectsElDatabasesElCollectionsElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `collection` after provisioning.\nCollection name."]
    pub fn collection(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.collection", self.base))
    }
    #[doc = "Get a reference to the value of field `fields` after provisioning.\n"]    pub fn fields (& self) -> ListRef < DatastreamStreamSourceConfigElMongodbSourceConfigElIncludeObjectsElDatabasesElCollectionsElFieldsElRef >{
        ListRef::new(self.shared().clone(), format!("{}.fields", self.base))
    }
}
#[derive(Serialize, Default)]
struct DatastreamStreamSourceConfigElMongodbSourceConfigElIncludeObjectsElDatabasesElDynamic { collections : Option < DynamicBlock < DatastreamStreamSourceConfigElMongodbSourceConfigElIncludeObjectsElDatabasesElCollectionsEl >> , }
#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElMongodbSourceConfigElIncludeObjectsElDatabasesEl { # [serde (skip_serializing_if = "Option::is_none")] database : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] collections : Option < Vec < DatastreamStreamSourceConfigElMongodbSourceConfigElIncludeObjectsElDatabasesElCollectionsEl > > , dynamic : DatastreamStreamSourceConfigElMongodbSourceConfigElIncludeObjectsElDatabasesElDynamic , }
impl DatastreamStreamSourceConfigElMongodbSourceConfigElIncludeObjectsElDatabasesEl {
    #[doc = "Set the field `database`.\nDatabase name."]
    pub fn set_database(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.database = Some(v.into());
        self
    }
    #[doc = "Set the field `collections`.\n"]
    pub fn set_collections(
        mut self,
        v : impl Into < BlockAssignable < DatastreamStreamSourceConfigElMongodbSourceConfigElIncludeObjectsElDatabasesElCollectionsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.collections = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.collections = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for DatastreamStreamSourceConfigElMongodbSourceConfigElIncludeObjectsElDatabasesEl
{
    type O = BlockAssignable<
        DatastreamStreamSourceConfigElMongodbSourceConfigElIncludeObjectsElDatabasesEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamSourceConfigElMongodbSourceConfigElIncludeObjectsElDatabasesEl {}
impl BuildDatastreamStreamSourceConfigElMongodbSourceConfigElIncludeObjectsElDatabasesEl {
    pub fn build(
        self,
    ) -> DatastreamStreamSourceConfigElMongodbSourceConfigElIncludeObjectsElDatabasesEl {
        DatastreamStreamSourceConfigElMongodbSourceConfigElIncludeObjectsElDatabasesEl {
            database: core::default::Default::default(),
            collections: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DatastreamStreamSourceConfigElMongodbSourceConfigElIncludeObjectsElDatabasesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamSourceConfigElMongodbSourceConfigElIncludeObjectsElDatabasesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamStreamSourceConfigElMongodbSourceConfigElIncludeObjectsElDatabasesElRef {
        DatastreamStreamSourceConfigElMongodbSourceConfigElIncludeObjectsElDatabasesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamSourceConfigElMongodbSourceConfigElIncludeObjectsElDatabasesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `database` after provisioning.\nDatabase name."]
    pub fn database(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.database", self.base))
    }
    #[doc = "Get a reference to the value of field `collections` after provisioning.\n"]    pub fn collections (& self) -> ListRef < DatastreamStreamSourceConfigElMongodbSourceConfigElIncludeObjectsElDatabasesElCollectionsElRef >{
        ListRef::new(self.shared().clone(), format!("{}.collections", self.base))
    }
}
#[derive(Serialize, Default)]
struct DatastreamStreamSourceConfigElMongodbSourceConfigElIncludeObjectsElDynamic {
    databases: Option<
        DynamicBlock<
            DatastreamStreamSourceConfigElMongodbSourceConfigElIncludeObjectsElDatabasesEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElMongodbSourceConfigElIncludeObjectsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    databases:
        Option<Vec<DatastreamStreamSourceConfigElMongodbSourceConfigElIncludeObjectsElDatabasesEl>>,
    dynamic: DatastreamStreamSourceConfigElMongodbSourceConfigElIncludeObjectsElDynamic,
}
impl DatastreamStreamSourceConfigElMongodbSourceConfigElIncludeObjectsEl {
    #[doc = "Set the field `databases`.\n"]
    pub fn set_databases(
        mut self,
        v: impl Into<
            BlockAssignable<
                DatastreamStreamSourceConfigElMongodbSourceConfigElIncludeObjectsElDatabasesEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.databases = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.databases = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DatastreamStreamSourceConfigElMongodbSourceConfigElIncludeObjectsEl {
    type O = BlockAssignable<DatastreamStreamSourceConfigElMongodbSourceConfigElIncludeObjectsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamSourceConfigElMongodbSourceConfigElIncludeObjectsEl {}
impl BuildDatastreamStreamSourceConfigElMongodbSourceConfigElIncludeObjectsEl {
    pub fn build(self) -> DatastreamStreamSourceConfigElMongodbSourceConfigElIncludeObjectsEl {
        DatastreamStreamSourceConfigElMongodbSourceConfigElIncludeObjectsEl {
            databases: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DatastreamStreamSourceConfigElMongodbSourceConfigElIncludeObjectsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamSourceConfigElMongodbSourceConfigElIncludeObjectsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamStreamSourceConfigElMongodbSourceConfigElIncludeObjectsElRef {
        DatastreamStreamSourceConfigElMongodbSourceConfigElIncludeObjectsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamSourceConfigElMongodbSourceConfigElIncludeObjectsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `databases` after provisioning.\n"]
    pub fn databases(
        &self,
    ) -> ListRef<DatastreamStreamSourceConfigElMongodbSourceConfigElIncludeObjectsElDatabasesElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.databases", self.base))
    }
}
#[derive(Serialize, Default)]
struct DatastreamStreamSourceConfigElMongodbSourceConfigElDynamic {
    exclude_objects:
        Option<DynamicBlock<DatastreamStreamSourceConfigElMongodbSourceConfigElExcludeObjectsEl>>,
    include_objects:
        Option<DynamicBlock<DatastreamStreamSourceConfigElMongodbSourceConfigElIncludeObjectsEl>>,
}
#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElMongodbSourceConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_concurrent_backfill_tasks: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_objects:
        Option<Vec<DatastreamStreamSourceConfigElMongodbSourceConfigElExcludeObjectsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_objects:
        Option<Vec<DatastreamStreamSourceConfigElMongodbSourceConfigElIncludeObjectsEl>>,
    dynamic: DatastreamStreamSourceConfigElMongodbSourceConfigElDynamic,
}
impl DatastreamStreamSourceConfigElMongodbSourceConfigEl {
    #[doc = "Set the field `max_concurrent_backfill_tasks`.\nOptional. Maximum number of concurrent backfill tasks. The number\nshould be non-negative and less than or equal to 50. If not set\n(or set to 0), the system''s default value is used"]
    pub fn set_max_concurrent_backfill_tasks(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_concurrent_backfill_tasks = Some(v.into());
        self
    }
    #[doc = "Set the field `exclude_objects`.\n"]
    pub fn set_exclude_objects(
        mut self,
        v: impl Into<
            BlockAssignable<DatastreamStreamSourceConfigElMongodbSourceConfigElExcludeObjectsEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.exclude_objects = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.exclude_objects = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `include_objects`.\n"]
    pub fn set_include_objects(
        mut self,
        v: impl Into<
            BlockAssignable<DatastreamStreamSourceConfigElMongodbSourceConfigElIncludeObjectsEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.include_objects = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.include_objects = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DatastreamStreamSourceConfigElMongodbSourceConfigEl {
    type O = BlockAssignable<DatastreamStreamSourceConfigElMongodbSourceConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamSourceConfigElMongodbSourceConfigEl {}
impl BuildDatastreamStreamSourceConfigElMongodbSourceConfigEl {
    pub fn build(self) -> DatastreamStreamSourceConfigElMongodbSourceConfigEl {
        DatastreamStreamSourceConfigElMongodbSourceConfigEl {
            max_concurrent_backfill_tasks: core::default::Default::default(),
            exclude_objects: core::default::Default::default(),
            include_objects: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DatastreamStreamSourceConfigElMongodbSourceConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamSourceConfigElMongodbSourceConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamStreamSourceConfigElMongodbSourceConfigElRef {
        DatastreamStreamSourceConfigElMongodbSourceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamSourceConfigElMongodbSourceConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `max_concurrent_backfill_tasks` after provisioning.\nOptional. Maximum number of concurrent backfill tasks. The number\nshould be non-negative and less than or equal to 50. If not set\n(or set to 0), the system''s default value is used"]
    pub fn max_concurrent_backfill_tasks(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_concurrent_backfill_tasks", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `exclude_objects` after provisioning.\n"]
    pub fn exclude_objects(
        &self,
    ) -> ListRef<DatastreamStreamSourceConfigElMongodbSourceConfigElExcludeObjectsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.exclude_objects", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `include_objects` after provisioning.\n"]
    pub fn include_objects(
        &self,
    ) -> ListRef<DatastreamStreamSourceConfigElMongodbSourceConfigElIncludeObjectsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.include_objects", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElMysqlSourceConfigElBinaryLogPositionEl {}
impl DatastreamStreamSourceConfigElMysqlSourceConfigElBinaryLogPositionEl {}
impl ToListMappable for DatastreamStreamSourceConfigElMysqlSourceConfigElBinaryLogPositionEl {
    type O = BlockAssignable<DatastreamStreamSourceConfigElMysqlSourceConfigElBinaryLogPositionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamSourceConfigElMysqlSourceConfigElBinaryLogPositionEl {}
impl BuildDatastreamStreamSourceConfigElMysqlSourceConfigElBinaryLogPositionEl {
    pub fn build(self) -> DatastreamStreamSourceConfigElMysqlSourceConfigElBinaryLogPositionEl {
        DatastreamStreamSourceConfigElMysqlSourceConfigElBinaryLogPositionEl {}
    }
}
pub struct DatastreamStreamSourceConfigElMysqlSourceConfigElBinaryLogPositionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamSourceConfigElMysqlSourceConfigElBinaryLogPositionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamStreamSourceConfigElMysqlSourceConfigElBinaryLogPositionElRef {
        DatastreamStreamSourceConfigElMysqlSourceConfigElBinaryLogPositionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamSourceConfigElMysqlSourceConfigElBinaryLogPositionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    collation: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    column: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nullable: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ordinal_position: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    primary_key: Option<PrimField<bool>>,
}
impl DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsEl { # [doc = "Set the field `collation`.\nColumn collation."] pub fn set_collation (mut self , v : impl Into < PrimField < String > >) -> Self { self . collation = Some (v . into ()) ; self } # [doc = "Set the field `column`.\nColumn name."] pub fn set_column (mut self , v : impl Into < PrimField < String > >) -> Self { self . column = Some (v . into ()) ; self } # [doc = "Set the field `data_type`.\nThe MySQL data type. Full data types list can be found here:\nhttps://dev.mysql.com/doc/refman/8.0/en/data-types.html"] pub fn set_data_type (mut self , v : impl Into < PrimField < String > >) -> Self { self . data_type = Some (v . into ()) ; self } # [doc = "Set the field `nullable`.\nWhether or not the column can accept a null value."] pub fn set_nullable (mut self , v : impl Into < PrimField < bool > >) -> Self { self . nullable = Some (v . into ()) ; self } # [doc = "Set the field `ordinal_position`.\nThe ordinal position of the column in the table."] pub fn set_ordinal_position (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . ordinal_position = Some (v . into ()) ; self } # [doc = "Set the field `primary_key`.\nWhether or not the column represents a primary key."] pub fn set_primary_key (mut self , v : impl Into < PrimField < bool > >) -> Self { self . primary_key = Some (v . into ()) ; self } }
impl ToListMappable for DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsEl { type O = BlockAssignable < DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsEl
{}
impl BuildDatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsEl { pub fn build (self) -> DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsEl { DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsEl { collation : core :: default :: Default :: default () , column : core :: default :: Default :: default () , data_type : core :: default :: Default :: default () , nullable : core :: default :: Default :: default () , ordinal_position : core :: default :: Default :: default () , primary_key : core :: default :: Default :: default () , } } }
pub struct DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsElRef { fn new (shared : StackShared , base : String) -> DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsElRef { DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsElRef { shared : shared , base : base . to_string () , } } }
impl DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `collation` after provisioning.\nColumn collation."] pub fn collation (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.collation" , self . base)) } # [doc = "Get a reference to the value of field `column` after provisioning.\nColumn name."] pub fn column (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.column" , self . base)) } # [doc = "Get a reference to the value of field `data_type` after provisioning.\nThe MySQL data type. Full data types list can be found here:\nhttps://dev.mysql.com/doc/refman/8.0/en/data-types.html"] pub fn data_type (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.data_type" , self . base)) } # [doc = "Get a reference to the value of field `length` after provisioning.\nColumn length."] pub fn length (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.length" , self . base)) } # [doc = "Get a reference to the value of field `nullable` after provisioning.\nWhether or not the column can accept a null value."] pub fn nullable (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.nullable" , self . base)) } # [doc = "Get a reference to the value of field `ordinal_position` after provisioning.\nThe ordinal position of the column in the table."] pub fn ordinal_position (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.ordinal_position" , self . base)) } # [doc = "Get a reference to the value of field `primary_key` after provisioning.\nWhether or not the column represents a primary key."] pub fn primary_key (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.primary_key" , self . base)) } }
#[derive(Serialize, Default)]
struct DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesElMysqlTablesElDynamic { mysql_columns : Option < DynamicBlock < DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsEl >> , }
#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesElMysqlTablesEl { table : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] mysql_columns : Option < Vec < DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsEl > > , dynamic : DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesElMysqlTablesElDynamic , }
impl
    DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesElMysqlTablesEl
{
    #[doc = "Set the field `mysql_columns`.\n"]
    pub fn set_mysql_columns(
        mut self,
        v : impl Into < BlockAssignable < DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.mysql_columns = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.mysql_columns = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesElMysqlTablesEl { type O = BlockAssignable < DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesElMysqlTablesEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesElMysqlTablesEl
{
    #[doc = "Table name."]
    pub table: PrimField<String>,
}
impl BuildDatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesElMysqlTablesEl { pub fn build (self) -> DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesElMysqlTablesEl { DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesElMysqlTablesEl { table : self . table , mysql_columns : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesElMysqlTablesElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesElMysqlTablesElRef { fn new (shared : StackShared , base : String) -> DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesElMysqlTablesElRef { DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesElMysqlTablesElRef { shared : shared , base : base . to_string () , } } }
impl DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesElMysqlTablesElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `table` after provisioning.\nTable name."] pub fn table (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.table" , self . base)) } # [doc = "Get a reference to the value of field `mysql_columns` after provisioning.\n"] pub fn mysql_columns (& self) -> ListRef < DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.mysql_columns" , self . base)) } }
#[derive(Serialize, Default)]
struct DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesElDynamic { mysql_tables : Option < DynamicBlock < DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesElMysqlTablesEl >> , }
#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesEl { database : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] mysql_tables : Option < Vec < DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesElMysqlTablesEl > > , dynamic : DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesElDynamic , }
impl DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesEl {
    #[doc = "Set the field `mysql_tables`.\n"]
    pub fn set_mysql_tables(
        mut self,
        v : impl Into < BlockAssignable < DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesElMysqlTablesEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.mysql_tables = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.mysql_tables = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesEl
{
    type O = BlockAssignable<
        DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesEl {
    #[doc = "Database name."]
    pub database: PrimField<String>,
}
impl BuildDatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesEl {
    pub fn build(
        self,
    ) -> DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesEl {
        DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesEl {
            database: self.database,
            mysql_tables: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesElRef {
        DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `database` after provisioning.\nDatabase name."]
    pub fn database(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.database", self.base))
    }
    #[doc = "Get a reference to the value of field `mysql_tables` after provisioning.\n"]    pub fn mysql_tables (& self) -> ListRef < DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesElMysqlTablesElRef >{
        ListRef::new(self.shared().clone(), format!("{}.mysql_tables", self.base))
    }
}
#[derive(Serialize, Default)]
struct DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElDynamic {
    mysql_databases: Option<
        DynamicBlock<
            DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    mysql_databases: Option<
        Vec<DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesEl>,
    >,
    dynamic: DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElDynamic,
}
impl DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsEl {
    #[doc = "Set the field `mysql_databases`.\n"]
    pub fn set_mysql_databases(
        mut self,
        v: impl Into<
            BlockAssignable<
                DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.mysql_databases = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.mysql_databases = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsEl {
    type O = BlockAssignable<DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsEl {}
impl BuildDatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsEl {
    pub fn build(self) -> DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsEl {
        DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsEl {
            mysql_databases: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElRef {
        DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `mysql_databases` after provisioning.\n"]
    pub fn mysql_databases(
        &self,
    ) -> ListRef<DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.mysql_databases", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElMysqlSourceConfigElGtidEl {}
impl DatastreamStreamSourceConfigElMysqlSourceConfigElGtidEl {}
impl ToListMappable for DatastreamStreamSourceConfigElMysqlSourceConfigElGtidEl {
    type O = BlockAssignable<DatastreamStreamSourceConfigElMysqlSourceConfigElGtidEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamSourceConfigElMysqlSourceConfigElGtidEl {}
impl BuildDatastreamStreamSourceConfigElMysqlSourceConfigElGtidEl {
    pub fn build(self) -> DatastreamStreamSourceConfigElMysqlSourceConfigElGtidEl {
        DatastreamStreamSourceConfigElMysqlSourceConfigElGtidEl {}
    }
}
pub struct DatastreamStreamSourceConfigElMysqlSourceConfigElGtidElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamSourceConfigElMysqlSourceConfigElGtidElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamStreamSourceConfigElMysqlSourceConfigElGtidElRef {
        DatastreamStreamSourceConfigElMysqlSourceConfigElGtidElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamSourceConfigElMysqlSourceConfigElGtidElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    collation: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    column: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nullable: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ordinal_position: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    primary_key: Option<PrimField<bool>>,
}
impl DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsEl { # [doc = "Set the field `collation`.\nColumn collation."] pub fn set_collation (mut self , v : impl Into < PrimField < String > >) -> Self { self . collation = Some (v . into ()) ; self } # [doc = "Set the field `column`.\nColumn name."] pub fn set_column (mut self , v : impl Into < PrimField < String > >) -> Self { self . column = Some (v . into ()) ; self } # [doc = "Set the field `data_type`.\nThe MySQL data type. Full data types list can be found here:\nhttps://dev.mysql.com/doc/refman/8.0/en/data-types.html"] pub fn set_data_type (mut self , v : impl Into < PrimField < String > >) -> Self { self . data_type = Some (v . into ()) ; self } # [doc = "Set the field `nullable`.\nWhether or not the column can accept a null value."] pub fn set_nullable (mut self , v : impl Into < PrimField < bool > >) -> Self { self . nullable = Some (v . into ()) ; self } # [doc = "Set the field `ordinal_position`.\nThe ordinal position of the column in the table."] pub fn set_ordinal_position (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . ordinal_position = Some (v . into ()) ; self } # [doc = "Set the field `primary_key`.\nWhether or not the column represents a primary key."] pub fn set_primary_key (mut self , v : impl Into < PrimField < bool > >) -> Self { self . primary_key = Some (v . into ()) ; self } }
impl ToListMappable for DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsEl { type O = BlockAssignable < DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsEl
{}
impl BuildDatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsEl { pub fn build (self) -> DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsEl { DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsEl { collation : core :: default :: Default :: default () , column : core :: default :: Default :: default () , data_type : core :: default :: Default :: default () , nullable : core :: default :: Default :: default () , ordinal_position : core :: default :: Default :: default () , primary_key : core :: default :: Default :: default () , } } }
pub struct DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsElRef { fn new (shared : StackShared , base : String) -> DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsElRef { DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsElRef { shared : shared , base : base . to_string () , } } }
impl DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `collation` after provisioning.\nColumn collation."] pub fn collation (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.collation" , self . base)) } # [doc = "Get a reference to the value of field `column` after provisioning.\nColumn name."] pub fn column (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.column" , self . base)) } # [doc = "Get a reference to the value of field `data_type` after provisioning.\nThe MySQL data type. Full data types list can be found here:\nhttps://dev.mysql.com/doc/refman/8.0/en/data-types.html"] pub fn data_type (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.data_type" , self . base)) } # [doc = "Get a reference to the value of field `length` after provisioning.\nColumn length."] pub fn length (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.length" , self . base)) } # [doc = "Get a reference to the value of field `nullable` after provisioning.\nWhether or not the column can accept a null value."] pub fn nullable (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.nullable" , self . base)) } # [doc = "Get a reference to the value of field `ordinal_position` after provisioning.\nThe ordinal position of the column in the table."] pub fn ordinal_position (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.ordinal_position" , self . base)) } # [doc = "Get a reference to the value of field `primary_key` after provisioning.\nWhether or not the column represents a primary key."] pub fn primary_key (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.primary_key" , self . base)) } }
#[derive(Serialize, Default)]
struct DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesElMysqlTablesElDynamic { mysql_columns : Option < DynamicBlock < DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsEl >> , }
#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesElMysqlTablesEl { table : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] mysql_columns : Option < Vec < DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsEl > > , dynamic : DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesElMysqlTablesElDynamic , }
impl
    DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesElMysqlTablesEl
{
    #[doc = "Set the field `mysql_columns`.\n"]
    pub fn set_mysql_columns(
        mut self,
        v : impl Into < BlockAssignable < DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.mysql_columns = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.mysql_columns = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesElMysqlTablesEl { type O = BlockAssignable < DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesElMysqlTablesEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesElMysqlTablesEl
{
    #[doc = "Table name."]
    pub table: PrimField<String>,
}
impl BuildDatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesElMysqlTablesEl { pub fn build (self) -> DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesElMysqlTablesEl { DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesElMysqlTablesEl { table : self . table , mysql_columns : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesElMysqlTablesElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesElMysqlTablesElRef { fn new (shared : StackShared , base : String) -> DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesElMysqlTablesElRef { DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesElMysqlTablesElRef { shared : shared , base : base . to_string () , } } }
impl DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesElMysqlTablesElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `table` after provisioning.\nTable name."] pub fn table (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.table" , self . base)) } # [doc = "Get a reference to the value of field `mysql_columns` after provisioning.\n"] pub fn mysql_columns (& self) -> ListRef < DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.mysql_columns" , self . base)) } }
#[derive(Serialize, Default)]
struct DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesElDynamic { mysql_tables : Option < DynamicBlock < DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesElMysqlTablesEl >> , }
#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesEl { database : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] mysql_tables : Option < Vec < DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesElMysqlTablesEl > > , dynamic : DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesElDynamic , }
impl DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesEl {
    #[doc = "Set the field `mysql_tables`.\n"]
    pub fn set_mysql_tables(
        mut self,
        v : impl Into < BlockAssignable < DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesElMysqlTablesEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.mysql_tables = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.mysql_tables = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesEl
{
    type O = BlockAssignable<
        DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesEl {
    #[doc = "Database name."]
    pub database: PrimField<String>,
}
impl BuildDatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesEl {
    pub fn build(
        self,
    ) -> DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesEl {
        DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesEl {
            database: self.database,
            mysql_tables: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesElRef {
        DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `database` after provisioning.\nDatabase name."]
    pub fn database(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.database", self.base))
    }
    #[doc = "Get a reference to the value of field `mysql_tables` after provisioning.\n"]    pub fn mysql_tables (& self) -> ListRef < DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesElMysqlTablesElRef >{
        ListRef::new(self.shared().clone(), format!("{}.mysql_tables", self.base))
    }
}
#[derive(Serialize, Default)]
struct DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElDynamic {
    mysql_databases: Option<
        DynamicBlock<
            DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    mysql_databases: Option<
        Vec<DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesEl>,
    >,
    dynamic: DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElDynamic,
}
impl DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsEl {
    #[doc = "Set the field `mysql_databases`.\n"]
    pub fn set_mysql_databases(
        mut self,
        v: impl Into<
            BlockAssignable<
                DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.mysql_databases = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.mysql_databases = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsEl {
    type O = BlockAssignable<DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsEl {}
impl BuildDatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsEl {
    pub fn build(self) -> DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsEl {
        DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsEl {
            mysql_databases: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElRef {
        DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `mysql_databases` after provisioning.\n"]
    pub fn mysql_databases(
        &self,
    ) -> ListRef<DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.mysql_databases", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DatastreamStreamSourceConfigElMysqlSourceConfigElDynamic {
    binary_log_position:
        Option<DynamicBlock<DatastreamStreamSourceConfigElMysqlSourceConfigElBinaryLogPositionEl>>,
    exclude_objects:
        Option<DynamicBlock<DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsEl>>,
    gtid: Option<DynamicBlock<DatastreamStreamSourceConfigElMysqlSourceConfigElGtidEl>>,
    include_objects:
        Option<DynamicBlock<DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsEl>>,
}
#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElMysqlSourceConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_concurrent_backfill_tasks: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_concurrent_cdc_tasks: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    binary_log_position:
        Option<Vec<DatastreamStreamSourceConfigElMysqlSourceConfigElBinaryLogPositionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_objects: Option<Vec<DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gtid: Option<Vec<DatastreamStreamSourceConfigElMysqlSourceConfigElGtidEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_objects: Option<Vec<DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsEl>>,
    dynamic: DatastreamStreamSourceConfigElMysqlSourceConfigElDynamic,
}
impl DatastreamStreamSourceConfigElMysqlSourceConfigEl {
    #[doc = "Set the field `max_concurrent_backfill_tasks`.\nMaximum number of concurrent backfill tasks. The number should be non negative.\nIf not set (or set to 0), the system's default value will be used."]
    pub fn set_max_concurrent_backfill_tasks(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_concurrent_backfill_tasks = Some(v.into());
        self
    }
    #[doc = "Set the field `max_concurrent_cdc_tasks`.\nMaximum number of concurrent CDC tasks. The number should be non negative.\nIf not set (or set to 0), the system's default value will be used."]
    pub fn set_max_concurrent_cdc_tasks(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_concurrent_cdc_tasks = Some(v.into());
        self
    }
    #[doc = "Set the field `binary_log_position`.\n"]
    pub fn set_binary_log_position(
        mut self,
        v: impl Into<
            BlockAssignable<DatastreamStreamSourceConfigElMysqlSourceConfigElBinaryLogPositionEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.binary_log_position = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.binary_log_position = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `exclude_objects`.\n"]
    pub fn set_exclude_objects(
        mut self,
        v: impl Into<BlockAssignable<DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.exclude_objects = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.exclude_objects = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `gtid`.\n"]
    pub fn set_gtid(
        mut self,
        v: impl Into<BlockAssignable<DatastreamStreamSourceConfigElMysqlSourceConfigElGtidEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.gtid = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.gtid = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `include_objects`.\n"]
    pub fn set_include_objects(
        mut self,
        v: impl Into<BlockAssignable<DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.include_objects = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.include_objects = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DatastreamStreamSourceConfigElMysqlSourceConfigEl {
    type O = BlockAssignable<DatastreamStreamSourceConfigElMysqlSourceConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamSourceConfigElMysqlSourceConfigEl {}
impl BuildDatastreamStreamSourceConfigElMysqlSourceConfigEl {
    pub fn build(self) -> DatastreamStreamSourceConfigElMysqlSourceConfigEl {
        DatastreamStreamSourceConfigElMysqlSourceConfigEl {
            max_concurrent_backfill_tasks: core::default::Default::default(),
            max_concurrent_cdc_tasks: core::default::Default::default(),
            binary_log_position: core::default::Default::default(),
            exclude_objects: core::default::Default::default(),
            gtid: core::default::Default::default(),
            include_objects: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DatastreamStreamSourceConfigElMysqlSourceConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamSourceConfigElMysqlSourceConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamStreamSourceConfigElMysqlSourceConfigElRef {
        DatastreamStreamSourceConfigElMysqlSourceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamSourceConfigElMysqlSourceConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `max_concurrent_backfill_tasks` after provisioning.\nMaximum number of concurrent backfill tasks. The number should be non negative.\nIf not set (or set to 0), the system's default value will be used."]
    pub fn max_concurrent_backfill_tasks(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_concurrent_backfill_tasks", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_concurrent_cdc_tasks` after provisioning.\nMaximum number of concurrent CDC tasks. The number should be non negative.\nIf not set (or set to 0), the system's default value will be used."]
    pub fn max_concurrent_cdc_tasks(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_concurrent_cdc_tasks", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `binary_log_position` after provisioning.\n"]
    pub fn binary_log_position(
        &self,
    ) -> ListRef<DatastreamStreamSourceConfigElMysqlSourceConfigElBinaryLogPositionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.binary_log_position", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `exclude_objects` after provisioning.\n"]
    pub fn exclude_objects(
        &self,
    ) -> ListRef<DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.exclude_objects", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `gtid` after provisioning.\n"]
    pub fn gtid(&self) -> ListRef<DatastreamStreamSourceConfigElMysqlSourceConfigElGtidElRef> {
        ListRef::new(self.shared().clone(), format!("{}.gtid", self.base))
    }
    #[doc = "Get a reference to the value of field `include_objects` after provisioning.\n"]
    pub fn include_objects(
        &self,
    ) -> ListRef<DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.include_objects", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElOracleSourceConfigElDropLargeObjectsEl {}
impl DatastreamStreamSourceConfigElOracleSourceConfigElDropLargeObjectsEl {}
impl ToListMappable for DatastreamStreamSourceConfigElOracleSourceConfigElDropLargeObjectsEl {
    type O = BlockAssignable<DatastreamStreamSourceConfigElOracleSourceConfigElDropLargeObjectsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamSourceConfigElOracleSourceConfigElDropLargeObjectsEl {}
impl BuildDatastreamStreamSourceConfigElOracleSourceConfigElDropLargeObjectsEl {
    pub fn build(self) -> DatastreamStreamSourceConfigElOracleSourceConfigElDropLargeObjectsEl {
        DatastreamStreamSourceConfigElOracleSourceConfigElDropLargeObjectsEl {}
    }
}
pub struct DatastreamStreamSourceConfigElOracleSourceConfigElDropLargeObjectsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamSourceConfigElOracleSourceConfigElDropLargeObjectsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamStreamSourceConfigElOracleSourceConfigElDropLargeObjectsElRef {
        DatastreamStreamSourceConfigElOracleSourceConfigElDropLargeObjectsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamSourceConfigElOracleSourceConfigElDropLargeObjectsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasElOracleTablesElOracleColumnsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    column: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_type: Option<PrimField<String>>,
}
impl DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasElOracleTablesElOracleColumnsEl { # [doc = "Set the field `column`.\nColumn name."] pub fn set_column (mut self , v : impl Into < PrimField < String > >) -> Self { self . column = Some (v . into ()) ; self } # [doc = "Set the field `data_type`.\nThe Oracle data type. Full data types list can be found here:\nhttps://docs.oracle.com/en/database/oracle/oracle-database/21/sqlrf/Data-Types.html"] pub fn set_data_type (mut self , v : impl Into < PrimField < String > >) -> Self { self . data_type = Some (v . into ()) ; self } }
impl ToListMappable for DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasElOracleTablesElOracleColumnsEl { type O = BlockAssignable < DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasElOracleTablesElOracleColumnsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasElOracleTablesElOracleColumnsEl
{}
impl BuildDatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasElOracleTablesElOracleColumnsEl { pub fn build (self) -> DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasElOracleTablesElOracleColumnsEl { DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasElOracleTablesElOracleColumnsEl { column : core :: default :: Default :: default () , data_type : core :: default :: Default :: default () , } } }
pub struct DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasElOracleTablesElOracleColumnsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasElOracleTablesElOracleColumnsElRef { fn new (shared : StackShared , base : String) -> DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasElOracleTablesElOracleColumnsElRef { DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasElOracleTablesElOracleColumnsElRef { shared : shared , base : base . to_string () , } } }
impl DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasElOracleTablesElOracleColumnsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `column` after provisioning.\nColumn name."] pub fn column (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.column" , self . base)) } # [doc = "Get a reference to the value of field `data_type` after provisioning.\nThe Oracle data type. Full data types list can be found here:\nhttps://docs.oracle.com/en/database/oracle/oracle-database/21/sqlrf/Data-Types.html"] pub fn data_type (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.data_type" , self . base)) } # [doc = "Get a reference to the value of field `encoding` after provisioning.\nColumn encoding."] pub fn encoding (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.encoding" , self . base)) } # [doc = "Get a reference to the value of field `length` after provisioning.\nColumn length."] pub fn length (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.length" , self . base)) } # [doc = "Get a reference to the value of field `nullable` after provisioning.\nWhether or not the column can accept a null value."] pub fn nullable (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.nullable" , self . base)) } # [doc = "Get a reference to the value of field `ordinal_position` after provisioning.\nThe ordinal position of the column in the table."] pub fn ordinal_position (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.ordinal_position" , self . base)) } # [doc = "Get a reference to the value of field `precision` after provisioning.\nColumn precision."] pub fn precision (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.precision" , self . base)) } # [doc = "Get a reference to the value of field `primary_key` after provisioning.\nWhether or not the column represents a primary key."] pub fn primary_key (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.primary_key" , self . base)) } # [doc = "Get a reference to the value of field `scale` after provisioning.\nColumn scale."] pub fn scale (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.scale" , self . base)) } }
#[derive(Serialize, Default)]
struct DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasElOracleTablesElDynamic { oracle_columns : Option < DynamicBlock < DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasElOracleTablesElOracleColumnsEl >> , }
#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasElOracleTablesEl { table : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] oracle_columns : Option < Vec < DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasElOracleTablesElOracleColumnsEl > > , dynamic : DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasElOracleTablesElDynamic , }
impl
    DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasElOracleTablesEl
{
    #[doc = "Set the field `oracle_columns`.\n"]
    pub fn set_oracle_columns(
        mut self,
        v : impl Into < BlockAssignable < DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasElOracleTablesElOracleColumnsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.oracle_columns = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.oracle_columns = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasElOracleTablesEl { type O = BlockAssignable < DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasElOracleTablesEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasElOracleTablesEl
{
    #[doc = "Table name."]
    pub table: PrimField<String>,
}
impl BuildDatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasElOracleTablesEl { pub fn build (self) -> DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasElOracleTablesEl { DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasElOracleTablesEl { table : self . table , oracle_columns : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasElOracleTablesElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasElOracleTablesElRef { fn new (shared : StackShared , base : String) -> DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasElOracleTablesElRef { DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasElOracleTablesElRef { shared : shared , base : base . to_string () , } } }
impl DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasElOracleTablesElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `table` after provisioning.\nTable name."] pub fn table (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.table" , self . base)) } # [doc = "Get a reference to the value of field `oracle_columns` after provisioning.\n"] pub fn oracle_columns (& self) -> ListRef < DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasElOracleTablesElOracleColumnsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.oracle_columns" , self . base)) } }
#[derive(Serialize, Default)]
struct DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasElDynamic { oracle_tables : Option < DynamicBlock < DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasElOracleTablesEl >> , }
#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasEl { schema : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] oracle_tables : Option < Vec < DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasElOracleTablesEl > > , dynamic : DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasElDynamic , }
impl DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasEl {
    #[doc = "Set the field `oracle_tables`.\n"]
    pub fn set_oracle_tables(
        mut self,
        v : impl Into < BlockAssignable < DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasElOracleTablesEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.oracle_tables = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.oracle_tables = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasEl
{
    type O = BlockAssignable<
        DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasEl {
    #[doc = "Schema name."]
    pub schema: PrimField<String>,
}
impl BuildDatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasEl {
    pub fn build(
        self,
    ) -> DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasEl {
        DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasEl {
            schema: self.schema,
            oracle_tables: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasElRef {
        DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `schema` after provisioning.\nSchema name."]
    pub fn schema(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.schema", self.base))
    }
    #[doc = "Get a reference to the value of field `oracle_tables` after provisioning.\n"]    pub fn oracle_tables (& self) -> ListRef < DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasElOracleTablesElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.oracle_tables", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElDynamic {
    oracle_schemas: Option<
        DynamicBlock<
            DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    oracle_schemas: Option<
        Vec<DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasEl>,
    >,
    dynamic: DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElDynamic,
}
impl DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsEl {
    #[doc = "Set the field `oracle_schemas`.\n"]
    pub fn set_oracle_schemas(
        mut self,
        v: impl Into<
            BlockAssignable<
                DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.oracle_schemas = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.oracle_schemas = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsEl {
    type O = BlockAssignable<DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsEl {}
impl BuildDatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsEl {
    pub fn build(self) -> DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsEl {
        DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsEl {
            oracle_schemas: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElRef {
        DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `oracle_schemas` after provisioning.\n"]
    pub fn oracle_schemas(
        &self,
    ) -> ListRef<DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.oracle_schemas", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasElOracleTablesElOracleColumnsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    column: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_type: Option<PrimField<String>>,
}
impl DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasElOracleTablesElOracleColumnsEl { # [doc = "Set the field `column`.\nColumn name."] pub fn set_column (mut self , v : impl Into < PrimField < String > >) -> Self { self . column = Some (v . into ()) ; self } # [doc = "Set the field `data_type`.\nThe Oracle data type. Full data types list can be found here:\nhttps://docs.oracle.com/en/database/oracle/oracle-database/21/sqlrf/Data-Types.html"] pub fn set_data_type (mut self , v : impl Into < PrimField < String > >) -> Self { self . data_type = Some (v . into ()) ; self } }
impl ToListMappable for DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasElOracleTablesElOracleColumnsEl { type O = BlockAssignable < DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasElOracleTablesElOracleColumnsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasElOracleTablesElOracleColumnsEl
{}
impl BuildDatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasElOracleTablesElOracleColumnsEl { pub fn build (self) -> DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasElOracleTablesElOracleColumnsEl { DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasElOracleTablesElOracleColumnsEl { column : core :: default :: Default :: default () , data_type : core :: default :: Default :: default () , } } }
pub struct DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasElOracleTablesElOracleColumnsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasElOracleTablesElOracleColumnsElRef { fn new (shared : StackShared , base : String) -> DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasElOracleTablesElOracleColumnsElRef { DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasElOracleTablesElOracleColumnsElRef { shared : shared , base : base . to_string () , } } }
impl DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasElOracleTablesElOracleColumnsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `column` after provisioning.\nColumn name."] pub fn column (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.column" , self . base)) } # [doc = "Get a reference to the value of field `data_type` after provisioning.\nThe Oracle data type. Full data types list can be found here:\nhttps://docs.oracle.com/en/database/oracle/oracle-database/21/sqlrf/Data-Types.html"] pub fn data_type (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.data_type" , self . base)) } # [doc = "Get a reference to the value of field `encoding` after provisioning.\nColumn encoding."] pub fn encoding (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.encoding" , self . base)) } # [doc = "Get a reference to the value of field `length` after provisioning.\nColumn length."] pub fn length (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.length" , self . base)) } # [doc = "Get a reference to the value of field `nullable` after provisioning.\nWhether or not the column can accept a null value."] pub fn nullable (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.nullable" , self . base)) } # [doc = "Get a reference to the value of field `ordinal_position` after provisioning.\nThe ordinal position of the column in the table."] pub fn ordinal_position (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.ordinal_position" , self . base)) } # [doc = "Get a reference to the value of field `precision` after provisioning.\nColumn precision."] pub fn precision (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.precision" , self . base)) } # [doc = "Get a reference to the value of field `primary_key` after provisioning.\nWhether or not the column represents a primary key."] pub fn primary_key (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.primary_key" , self . base)) } # [doc = "Get a reference to the value of field `scale` after provisioning.\nColumn scale."] pub fn scale (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.scale" , self . base)) } }
#[derive(Serialize, Default)]
struct DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasElOracleTablesElDynamic { oracle_columns : Option < DynamicBlock < DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasElOracleTablesElOracleColumnsEl >> , }
#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasElOracleTablesEl { table : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] oracle_columns : Option < Vec < DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasElOracleTablesElOracleColumnsEl > > , dynamic : DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasElOracleTablesElDynamic , }
impl
    DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasElOracleTablesEl
{
    #[doc = "Set the field `oracle_columns`.\n"]
    pub fn set_oracle_columns(
        mut self,
        v : impl Into < BlockAssignable < DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasElOracleTablesElOracleColumnsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.oracle_columns = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.oracle_columns = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasElOracleTablesEl { type O = BlockAssignable < DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasElOracleTablesEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasElOracleTablesEl
{
    #[doc = "Table name."]
    pub table: PrimField<String>,
}
impl BuildDatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasElOracleTablesEl { pub fn build (self) -> DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasElOracleTablesEl { DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasElOracleTablesEl { table : self . table , oracle_columns : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasElOracleTablesElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasElOracleTablesElRef { fn new (shared : StackShared , base : String) -> DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasElOracleTablesElRef { DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasElOracleTablesElRef { shared : shared , base : base . to_string () , } } }
impl DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasElOracleTablesElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `table` after provisioning.\nTable name."] pub fn table (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.table" , self . base)) } # [doc = "Get a reference to the value of field `oracle_columns` after provisioning.\n"] pub fn oracle_columns (& self) -> ListRef < DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasElOracleTablesElOracleColumnsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.oracle_columns" , self . base)) } }
#[derive(Serialize, Default)]
struct DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasElDynamic { oracle_tables : Option < DynamicBlock < DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasElOracleTablesEl >> , }
#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasEl { schema : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] oracle_tables : Option < Vec < DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasElOracleTablesEl > > , dynamic : DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasElDynamic , }
impl DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasEl {
    #[doc = "Set the field `oracle_tables`.\n"]
    pub fn set_oracle_tables(
        mut self,
        v : impl Into < BlockAssignable < DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasElOracleTablesEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.oracle_tables = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.oracle_tables = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasEl
{
    type O = BlockAssignable<
        DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasEl {
    #[doc = "Schema name."]
    pub schema: PrimField<String>,
}
impl BuildDatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasEl {
    pub fn build(
        self,
    ) -> DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasEl {
        DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasEl {
            schema: self.schema,
            oracle_tables: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasElRef {
        DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `schema` after provisioning.\nSchema name."]
    pub fn schema(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.schema", self.base))
    }
    #[doc = "Get a reference to the value of field `oracle_tables` after provisioning.\n"]    pub fn oracle_tables (& self) -> ListRef < DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasElOracleTablesElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.oracle_tables", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElDynamic {
    oracle_schemas: Option<
        DynamicBlock<
            DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    oracle_schemas: Option<
        Vec<DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasEl>,
    >,
    dynamic: DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElDynamic,
}
impl DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsEl {
    #[doc = "Set the field `oracle_schemas`.\n"]
    pub fn set_oracle_schemas(
        mut self,
        v: impl Into<
            BlockAssignable<
                DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.oracle_schemas = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.oracle_schemas = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsEl {
    type O = BlockAssignable<DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsEl {}
impl BuildDatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsEl {
    pub fn build(self) -> DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsEl {
        DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsEl {
            oracle_schemas: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElRef {
        DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `oracle_schemas` after provisioning.\n"]
    pub fn oracle_schemas(
        &self,
    ) -> ListRef<DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.oracle_schemas", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElOracleSourceConfigElStreamLargeObjectsEl {}
impl DatastreamStreamSourceConfigElOracleSourceConfigElStreamLargeObjectsEl {}
impl ToListMappable for DatastreamStreamSourceConfigElOracleSourceConfigElStreamLargeObjectsEl {
    type O =
        BlockAssignable<DatastreamStreamSourceConfigElOracleSourceConfigElStreamLargeObjectsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamSourceConfigElOracleSourceConfigElStreamLargeObjectsEl {}
impl BuildDatastreamStreamSourceConfigElOracleSourceConfigElStreamLargeObjectsEl {
    pub fn build(self) -> DatastreamStreamSourceConfigElOracleSourceConfigElStreamLargeObjectsEl {
        DatastreamStreamSourceConfigElOracleSourceConfigElStreamLargeObjectsEl {}
    }
}
pub struct DatastreamStreamSourceConfigElOracleSourceConfigElStreamLargeObjectsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamSourceConfigElOracleSourceConfigElStreamLargeObjectsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamStreamSourceConfigElOracleSourceConfigElStreamLargeObjectsElRef {
        DatastreamStreamSourceConfigElOracleSourceConfigElStreamLargeObjectsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamSourceConfigElOracleSourceConfigElStreamLargeObjectsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize, Default)]
struct DatastreamStreamSourceConfigElOracleSourceConfigElDynamic {
    drop_large_objects:
        Option<DynamicBlock<DatastreamStreamSourceConfigElOracleSourceConfigElDropLargeObjectsEl>>,
    exclude_objects:
        Option<DynamicBlock<DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsEl>>,
    include_objects:
        Option<DynamicBlock<DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsEl>>,
    stream_large_objects: Option<
        DynamicBlock<DatastreamStreamSourceConfigElOracleSourceConfigElStreamLargeObjectsEl>,
    >,
}
#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElOracleSourceConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_concurrent_backfill_tasks: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_concurrent_cdc_tasks: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    drop_large_objects:
        Option<Vec<DatastreamStreamSourceConfigElOracleSourceConfigElDropLargeObjectsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_objects:
        Option<Vec<DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_objects:
        Option<Vec<DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stream_large_objects:
        Option<Vec<DatastreamStreamSourceConfigElOracleSourceConfigElStreamLargeObjectsEl>>,
    dynamic: DatastreamStreamSourceConfigElOracleSourceConfigElDynamic,
}
impl DatastreamStreamSourceConfigElOracleSourceConfigEl {
    #[doc = "Set the field `max_concurrent_backfill_tasks`.\nMaximum number of concurrent backfill tasks. The number should be non negative.\nIf not set (or set to 0), the system's default value will be used."]
    pub fn set_max_concurrent_backfill_tasks(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_concurrent_backfill_tasks = Some(v.into());
        self
    }
    #[doc = "Set the field `max_concurrent_cdc_tasks`.\nMaximum number of concurrent CDC tasks. The number should be non negative.\nIf not set (or set to 0), the system's default value will be used."]
    pub fn set_max_concurrent_cdc_tasks(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_concurrent_cdc_tasks = Some(v.into());
        self
    }
    #[doc = "Set the field `drop_large_objects`.\n"]
    pub fn set_drop_large_objects(
        mut self,
        v: impl Into<
            BlockAssignable<DatastreamStreamSourceConfigElOracleSourceConfigElDropLargeObjectsEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.drop_large_objects = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.drop_large_objects = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `exclude_objects`.\n"]
    pub fn set_exclude_objects(
        mut self,
        v: impl Into<
            BlockAssignable<DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.exclude_objects = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.exclude_objects = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `include_objects`.\n"]
    pub fn set_include_objects(
        mut self,
        v: impl Into<
            BlockAssignable<DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.include_objects = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.include_objects = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `stream_large_objects`.\n"]
    pub fn set_stream_large_objects(
        mut self,
        v: impl Into<
            BlockAssignable<DatastreamStreamSourceConfigElOracleSourceConfigElStreamLargeObjectsEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.stream_large_objects = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.stream_large_objects = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DatastreamStreamSourceConfigElOracleSourceConfigEl {
    type O = BlockAssignable<DatastreamStreamSourceConfigElOracleSourceConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamSourceConfigElOracleSourceConfigEl {}
impl BuildDatastreamStreamSourceConfigElOracleSourceConfigEl {
    pub fn build(self) -> DatastreamStreamSourceConfigElOracleSourceConfigEl {
        DatastreamStreamSourceConfigElOracleSourceConfigEl {
            max_concurrent_backfill_tasks: core::default::Default::default(),
            max_concurrent_cdc_tasks: core::default::Default::default(),
            drop_large_objects: core::default::Default::default(),
            exclude_objects: core::default::Default::default(),
            include_objects: core::default::Default::default(),
            stream_large_objects: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DatastreamStreamSourceConfigElOracleSourceConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamSourceConfigElOracleSourceConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamStreamSourceConfigElOracleSourceConfigElRef {
        DatastreamStreamSourceConfigElOracleSourceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamSourceConfigElOracleSourceConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `max_concurrent_backfill_tasks` after provisioning.\nMaximum number of concurrent backfill tasks. The number should be non negative.\nIf not set (or set to 0), the system's default value will be used."]
    pub fn max_concurrent_backfill_tasks(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_concurrent_backfill_tasks", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_concurrent_cdc_tasks` after provisioning.\nMaximum number of concurrent CDC tasks. The number should be non negative.\nIf not set (or set to 0), the system's default value will be used."]
    pub fn max_concurrent_cdc_tasks(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_concurrent_cdc_tasks", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `drop_large_objects` after provisioning.\n"]
    pub fn drop_large_objects(
        &self,
    ) -> ListRef<DatastreamStreamSourceConfigElOracleSourceConfigElDropLargeObjectsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.drop_large_objects", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `exclude_objects` after provisioning.\n"]
    pub fn exclude_objects(
        &self,
    ) -> ListRef<DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.exclude_objects", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `include_objects` after provisioning.\n"]
    pub fn include_objects(
        &self,
    ) -> ListRef<DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.include_objects", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `stream_large_objects` after provisioning.\n"]
    pub fn stream_large_objects(
        &self,
    ) -> ListRef<DatastreamStreamSourceConfigElOracleSourceConfigElStreamLargeObjectsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.stream_large_objects", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    column: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nullable: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ordinal_position: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    primary_key: Option<PrimField<bool>>,
}
impl DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsEl { # [doc = "Set the field `column`.\nColumn name."] pub fn set_column (mut self , v : impl Into < PrimField < String > >) -> Self { self . column = Some (v . into ()) ; self } # [doc = "Set the field `data_type`.\nThe PostgreSQL data type. Full data types list can be found here:\nhttps://www.postgresql.org/docs/current/datatype.html"] pub fn set_data_type (mut self , v : impl Into < PrimField < String > >) -> Self { self . data_type = Some (v . into ()) ; self } # [doc = "Set the field `nullable`.\nWhether or not the column can accept a null value."] pub fn set_nullable (mut self , v : impl Into < PrimField < bool > >) -> Self { self . nullable = Some (v . into ()) ; self } # [doc = "Set the field `ordinal_position`.\nThe ordinal position of the column in the table."] pub fn set_ordinal_position (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . ordinal_position = Some (v . into ()) ; self } # [doc = "Set the field `primary_key`.\nWhether or not the column represents a primary key."] pub fn set_primary_key (mut self , v : impl Into < PrimField < bool > >) -> Self { self . primary_key = Some (v . into ()) ; self } }
impl ToListMappable for DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsEl { type O = BlockAssignable < DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsEl
{}
impl BuildDatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsEl { pub fn build (self) -> DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsEl { DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsEl { column : core :: default :: Default :: default () , data_type : core :: default :: Default :: default () , nullable : core :: default :: Default :: default () , ordinal_position : core :: default :: Default :: default () , primary_key : core :: default :: Default :: default () , } } }
pub struct DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsElRef { fn new (shared : StackShared , base : String) -> DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsElRef { DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsElRef { shared : shared , base : base . to_string () , } } }
impl DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `column` after provisioning.\nColumn name."] pub fn column (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.column" , self . base)) } # [doc = "Get a reference to the value of field `data_type` after provisioning.\nThe PostgreSQL data type. Full data types list can be found here:\nhttps://www.postgresql.org/docs/current/datatype.html"] pub fn data_type (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.data_type" , self . base)) } # [doc = "Get a reference to the value of field `length` after provisioning.\nColumn length."] pub fn length (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.length" , self . base)) } # [doc = "Get a reference to the value of field `nullable` after provisioning.\nWhether or not the column can accept a null value."] pub fn nullable (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.nullable" , self . base)) } # [doc = "Get a reference to the value of field `ordinal_position` after provisioning.\nThe ordinal position of the column in the table."] pub fn ordinal_position (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.ordinal_position" , self . base)) } # [doc = "Get a reference to the value of field `precision` after provisioning.\nColumn precision."] pub fn precision (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.precision" , self . base)) } # [doc = "Get a reference to the value of field `primary_key` after provisioning.\nWhether or not the column represents a primary key."] pub fn primary_key (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.primary_key" , self . base)) } # [doc = "Get a reference to the value of field `scale` after provisioning.\nColumn scale."] pub fn scale (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.scale" , self . base)) } }
#[derive(Serialize, Default)]
struct DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasElPostgresqlTablesElDynamic { postgresql_columns : Option < DynamicBlock < DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsEl >> , }
#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasElPostgresqlTablesEl { table : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] postgresql_columns : Option < Vec < DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsEl > > , dynamic : DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasElPostgresqlTablesElDynamic , }
impl DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasElPostgresqlTablesEl { # [doc = "Set the field `postgresql_columns`.\n"] pub fn set_postgresql_columns (mut self , v : impl Into < BlockAssignable < DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . postgresql_columns = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . postgresql_columns = Some (d) ; } } self } }
impl ToListMappable for DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasElPostgresqlTablesEl { type O = BlockAssignable < DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasElPostgresqlTablesEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasElPostgresqlTablesEl
{
    #[doc = "Table name."]
    pub table: PrimField<String>,
}
impl BuildDatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasElPostgresqlTablesEl { pub fn build (self) -> DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasElPostgresqlTablesEl { DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasElPostgresqlTablesEl { table : self . table , postgresql_columns : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasElPostgresqlTablesElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasElPostgresqlTablesElRef { fn new (shared : StackShared , base : String) -> DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasElPostgresqlTablesElRef { DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasElPostgresqlTablesElRef { shared : shared , base : base . to_string () , } } }
impl DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasElPostgresqlTablesElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `table` after provisioning.\nTable name."] pub fn table (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.table" , self . base)) } # [doc = "Get a reference to the value of field `postgresql_columns` after provisioning.\n"] pub fn postgresql_columns (& self) -> ListRef < DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.postgresql_columns" , self . base)) } }
#[derive(Serialize, Default)]
struct DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasElDynamic { postgresql_tables : Option < DynamicBlock < DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasElPostgresqlTablesEl >> , }
#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasEl { schema : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] postgresql_tables : Option < Vec < DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasElPostgresqlTablesEl > > , dynamic : DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasElDynamic , }
impl DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasEl {
    #[doc = "Set the field `postgresql_tables`.\n"]
    pub fn set_postgresql_tables(
        mut self,
        v : impl Into < BlockAssignable < DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasElPostgresqlTablesEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.postgresql_tables = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.postgresql_tables = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasEl
{
    type O = BlockAssignable<
        DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasEl
{
    #[doc = "Database name."]
    pub schema: PrimField<String>,
}
impl
    BuildDatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasEl
{
    pub fn build(
        self,
    ) -> DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasEl
    {
        DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasEl {
            schema: self.schema,
            postgresql_tables: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasElRef
    {
        DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasElRef { shared : shared , base : base . to_string () , }
    }
}
impl DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `schema` after provisioning.\nDatabase name."]
    pub fn schema(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.schema", self.base))
    }
    #[doc = "Get a reference to the value of field `postgresql_tables` after provisioning.\n"]    pub fn postgresql_tables (& self) -> ListRef < DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasElPostgresqlTablesElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.postgresql_tables", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElDynamic { postgresql_schemas : Option < DynamicBlock < DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasEl >> , }
#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsEl { # [serde (skip_serializing_if = "Option::is_none")] postgresql_schemas : Option < Vec < DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasEl > > , dynamic : DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElDynamic , }
impl DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsEl {
    #[doc = "Set the field `postgresql_schemas`.\n"]
    pub fn set_postgresql_schemas(
        mut self,
        v : impl Into < BlockAssignable < DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.postgresql_schemas = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.postgresql_schemas = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsEl {
    type O =
        BlockAssignable<DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsEl {}
impl BuildDatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsEl {
    pub fn build(self) -> DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsEl {
        DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsEl {
            postgresql_schemas: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElRef {
        DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `postgresql_schemas` after provisioning.\n"]    pub fn postgresql_schemas (& self) -> ListRef < DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.postgresql_schemas", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    column: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nullable: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ordinal_position: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    primary_key: Option<PrimField<bool>>,
}
impl DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsEl { # [doc = "Set the field `column`.\nColumn name."] pub fn set_column (mut self , v : impl Into < PrimField < String > >) -> Self { self . column = Some (v . into ()) ; self } # [doc = "Set the field `data_type`.\nThe PostgreSQL data type. Full data types list can be found here:\nhttps://www.postgresql.org/docs/current/datatype.html"] pub fn set_data_type (mut self , v : impl Into < PrimField < String > >) -> Self { self . data_type = Some (v . into ()) ; self } # [doc = "Set the field `nullable`.\nWhether or not the column can accept a null value."] pub fn set_nullable (mut self , v : impl Into < PrimField < bool > >) -> Self { self . nullable = Some (v . into ()) ; self } # [doc = "Set the field `ordinal_position`.\nThe ordinal position of the column in the table."] pub fn set_ordinal_position (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . ordinal_position = Some (v . into ()) ; self } # [doc = "Set the field `primary_key`.\nWhether or not the column represents a primary key."] pub fn set_primary_key (mut self , v : impl Into < PrimField < bool > >) -> Self { self . primary_key = Some (v . into ()) ; self } }
impl ToListMappable for DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsEl { type O = BlockAssignable < DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsEl
{}
impl BuildDatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsEl { pub fn build (self) -> DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsEl { DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsEl { column : core :: default :: Default :: default () , data_type : core :: default :: Default :: default () , nullable : core :: default :: Default :: default () , ordinal_position : core :: default :: Default :: default () , primary_key : core :: default :: Default :: default () , } } }
pub struct DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsElRef { fn new (shared : StackShared , base : String) -> DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsElRef { DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsElRef { shared : shared , base : base . to_string () , } } }
impl DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `column` after provisioning.\nColumn name."] pub fn column (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.column" , self . base)) } # [doc = "Get a reference to the value of field `data_type` after provisioning.\nThe PostgreSQL data type. Full data types list can be found here:\nhttps://www.postgresql.org/docs/current/datatype.html"] pub fn data_type (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.data_type" , self . base)) } # [doc = "Get a reference to the value of field `length` after provisioning.\nColumn length."] pub fn length (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.length" , self . base)) } # [doc = "Get a reference to the value of field `nullable` after provisioning.\nWhether or not the column can accept a null value."] pub fn nullable (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.nullable" , self . base)) } # [doc = "Get a reference to the value of field `ordinal_position` after provisioning.\nThe ordinal position of the column in the table."] pub fn ordinal_position (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.ordinal_position" , self . base)) } # [doc = "Get a reference to the value of field `precision` after provisioning.\nColumn precision."] pub fn precision (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.precision" , self . base)) } # [doc = "Get a reference to the value of field `primary_key` after provisioning.\nWhether or not the column represents a primary key."] pub fn primary_key (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.primary_key" , self . base)) } # [doc = "Get a reference to the value of field `scale` after provisioning.\nColumn scale."] pub fn scale (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.scale" , self . base)) } }
#[derive(Serialize, Default)]
struct DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasElPostgresqlTablesElDynamic { postgresql_columns : Option < DynamicBlock < DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsEl >> , }
#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasElPostgresqlTablesEl { table : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] postgresql_columns : Option < Vec < DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsEl > > , dynamic : DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasElPostgresqlTablesElDynamic , }
impl DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasElPostgresqlTablesEl { # [doc = "Set the field `postgresql_columns`.\n"] pub fn set_postgresql_columns (mut self , v : impl Into < BlockAssignable < DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . postgresql_columns = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . postgresql_columns = Some (d) ; } } self } }
impl ToListMappable for DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasElPostgresqlTablesEl { type O = BlockAssignable < DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasElPostgresqlTablesEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasElPostgresqlTablesEl
{
    #[doc = "Table name."]
    pub table: PrimField<String>,
}
impl BuildDatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasElPostgresqlTablesEl { pub fn build (self) -> DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasElPostgresqlTablesEl { DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasElPostgresqlTablesEl { table : self . table , postgresql_columns : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasElPostgresqlTablesElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasElPostgresqlTablesElRef { fn new (shared : StackShared , base : String) -> DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasElPostgresqlTablesElRef { DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasElPostgresqlTablesElRef { shared : shared , base : base . to_string () , } } }
impl DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasElPostgresqlTablesElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `table` after provisioning.\nTable name."] pub fn table (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.table" , self . base)) } # [doc = "Get a reference to the value of field `postgresql_columns` after provisioning.\n"] pub fn postgresql_columns (& self) -> ListRef < DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.postgresql_columns" , self . base)) } }
#[derive(Serialize, Default)]
struct DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasElDynamic { postgresql_tables : Option < DynamicBlock < DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasElPostgresqlTablesEl >> , }
#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasEl { schema : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] postgresql_tables : Option < Vec < DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasElPostgresqlTablesEl > > , dynamic : DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasElDynamic , }
impl DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasEl {
    #[doc = "Set the field `postgresql_tables`.\n"]
    pub fn set_postgresql_tables(
        mut self,
        v : impl Into < BlockAssignable < DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasElPostgresqlTablesEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.postgresql_tables = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.postgresql_tables = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasEl
{
    type O = BlockAssignable<
        DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasEl
{
    #[doc = "Database name."]
    pub schema: PrimField<String>,
}
impl
    BuildDatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasEl
{
    pub fn build(
        self,
    ) -> DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasEl
    {
        DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasEl {
            schema: self.schema,
            postgresql_tables: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasElRef
    {
        DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasElRef { shared : shared , base : base . to_string () , }
    }
}
impl DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `schema` after provisioning.\nDatabase name."]
    pub fn schema(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.schema", self.base))
    }
    #[doc = "Get a reference to the value of field `postgresql_tables` after provisioning.\n"]    pub fn postgresql_tables (& self) -> ListRef < DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasElPostgresqlTablesElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.postgresql_tables", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElDynamic { postgresql_schemas : Option < DynamicBlock < DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasEl >> , }
#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsEl { # [serde (skip_serializing_if = "Option::is_none")] postgresql_schemas : Option < Vec < DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasEl > > , dynamic : DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElDynamic , }
impl DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsEl {
    #[doc = "Set the field `postgresql_schemas`.\n"]
    pub fn set_postgresql_schemas(
        mut self,
        v : impl Into < BlockAssignable < DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.postgresql_schemas = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.postgresql_schemas = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsEl {
    type O =
        BlockAssignable<DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsEl {}
impl BuildDatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsEl {
    pub fn build(self) -> DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsEl {
        DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsEl {
            postgresql_schemas: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElRef {
        DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `postgresql_schemas` after provisioning.\n"]    pub fn postgresql_schemas (& self) -> ListRef < DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.postgresql_schemas", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DatastreamStreamSourceConfigElPostgresqlSourceConfigElDynamic {
    exclude_objects: Option<
        DynamicBlock<DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsEl>,
    >,
    include_objects: Option<
        DynamicBlock<DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsEl>,
    >,
}
#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElPostgresqlSourceConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_concurrent_backfill_tasks: Option<PrimField<f64>>,
    publication: PrimField<String>,
    replication_slot: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_objects:
        Option<Vec<DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_objects:
        Option<Vec<DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsEl>>,
    dynamic: DatastreamStreamSourceConfigElPostgresqlSourceConfigElDynamic,
}
impl DatastreamStreamSourceConfigElPostgresqlSourceConfigEl {
    #[doc = "Set the field `max_concurrent_backfill_tasks`.\nMaximum number of concurrent backfill tasks. The number should be non\nnegative. If not set (or set to 0), the system's default value will be used."]
    pub fn set_max_concurrent_backfill_tasks(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_concurrent_backfill_tasks = Some(v.into());
        self
    }
    #[doc = "Set the field `exclude_objects`.\n"]
    pub fn set_exclude_objects(
        mut self,
        v: impl Into<
            BlockAssignable<DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.exclude_objects = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.exclude_objects = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `include_objects`.\n"]
    pub fn set_include_objects(
        mut self,
        v: impl Into<
            BlockAssignable<DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.include_objects = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.include_objects = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DatastreamStreamSourceConfigElPostgresqlSourceConfigEl {
    type O = BlockAssignable<DatastreamStreamSourceConfigElPostgresqlSourceConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamSourceConfigElPostgresqlSourceConfigEl {
    #[doc = "The name of the publication that includes the set of all tables\nthat are defined in the stream's include_objects."]
    pub publication: PrimField<String>,
    #[doc = "The name of the logical replication slot that's configured with\nthe pgoutput plugin."]
    pub replication_slot: PrimField<String>,
}
impl BuildDatastreamStreamSourceConfigElPostgresqlSourceConfigEl {
    pub fn build(self) -> DatastreamStreamSourceConfigElPostgresqlSourceConfigEl {
        DatastreamStreamSourceConfigElPostgresqlSourceConfigEl {
            max_concurrent_backfill_tasks: core::default::Default::default(),
            publication: self.publication,
            replication_slot: self.replication_slot,
            exclude_objects: core::default::Default::default(),
            include_objects: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DatastreamStreamSourceConfigElPostgresqlSourceConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamSourceConfigElPostgresqlSourceConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamStreamSourceConfigElPostgresqlSourceConfigElRef {
        DatastreamStreamSourceConfigElPostgresqlSourceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamSourceConfigElPostgresqlSourceConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `max_concurrent_backfill_tasks` after provisioning.\nMaximum number of concurrent backfill tasks. The number should be non\nnegative. If not set (or set to 0), the system's default value will be used."]
    pub fn max_concurrent_backfill_tasks(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_concurrent_backfill_tasks", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `publication` after provisioning.\nThe name of the publication that includes the set of all tables\nthat are defined in the stream's include_objects."]
    pub fn publication(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.publication", self.base))
    }
    #[doc = "Get a reference to the value of field `replication_slot` after provisioning.\nThe name of the logical replication slot that's configured with\nthe pgoutput plugin."]
    pub fn replication_slot(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.replication_slot", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `exclude_objects` after provisioning.\n"]
    pub fn exclude_objects(
        &self,
    ) -> ListRef<DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.exclude_objects", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `include_objects` after provisioning.\n"]
    pub fn include_objects(
        &self,
    ) -> ListRef<DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.include_objects", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElSalesforceSourceConfigElExcludeObjectsElObjectsElFieldsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}
impl DatastreamStreamSourceConfigElSalesforceSourceConfigElExcludeObjectsElObjectsElFieldsEl {
    #[doc = "Set the field `name`.\nField name."]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}
impl ToListMappable
    for DatastreamStreamSourceConfigElSalesforceSourceConfigElExcludeObjectsElObjectsElFieldsEl
{
    type O = BlockAssignable<
        DatastreamStreamSourceConfigElSalesforceSourceConfigElExcludeObjectsElObjectsElFieldsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamSourceConfigElSalesforceSourceConfigElExcludeObjectsElObjectsElFieldsEl
{}
impl BuildDatastreamStreamSourceConfigElSalesforceSourceConfigElExcludeObjectsElObjectsElFieldsEl {
    pub fn build(
        self,
    ) -> DatastreamStreamSourceConfigElSalesforceSourceConfigElExcludeObjectsElObjectsElFieldsEl
    {
        DatastreamStreamSourceConfigElSalesforceSourceConfigElExcludeObjectsElObjectsElFieldsEl {
            name: core::default::Default::default(),
        }
    }
}
pub struct DatastreamStreamSourceConfigElSalesforceSourceConfigElExcludeObjectsElObjectsElFieldsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for DatastreamStreamSourceConfigElSalesforceSourceConfigElExcludeObjectsElObjectsElFieldsElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamStreamSourceConfigElSalesforceSourceConfigElExcludeObjectsElObjectsElFieldsElRef
    {
        DatastreamStreamSourceConfigElSalesforceSourceConfigElExcludeObjectsElObjectsElFieldsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamSourceConfigElSalesforceSourceConfigElExcludeObjectsElObjectsElFieldsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nField name."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
#[derive(Serialize, Default)]
struct DatastreamStreamSourceConfigElSalesforceSourceConfigElExcludeObjectsElObjectsElDynamic {
    fields: Option<
        DynamicBlock<
            DatastreamStreamSourceConfigElSalesforceSourceConfigElExcludeObjectsElObjectsElFieldsEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElSalesforceSourceConfigElExcludeObjectsElObjectsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    object_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fields: Option<
        Vec<
            DatastreamStreamSourceConfigElSalesforceSourceConfigElExcludeObjectsElObjectsElFieldsEl,
        >,
    >,
    dynamic: DatastreamStreamSourceConfigElSalesforceSourceConfigElExcludeObjectsElObjectsElDynamic,
}
impl DatastreamStreamSourceConfigElSalesforceSourceConfigElExcludeObjectsElObjectsEl {
    #[doc = "Set the field `object_name`.\nName of object in Salesforce Org."]
    pub fn set_object_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.object_name = Some(v.into());
        self
    }
    #[doc = "Set the field `fields`.\n"]
    pub fn set_fields(
        mut self,
        v : impl Into < BlockAssignable < DatastreamStreamSourceConfigElSalesforceSourceConfigElExcludeObjectsElObjectsElFieldsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.fields = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.fields = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for DatastreamStreamSourceConfigElSalesforceSourceConfigElExcludeObjectsElObjectsEl
{
    type O = BlockAssignable<
        DatastreamStreamSourceConfigElSalesforceSourceConfigElExcludeObjectsElObjectsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamSourceConfigElSalesforceSourceConfigElExcludeObjectsElObjectsEl {}
impl BuildDatastreamStreamSourceConfigElSalesforceSourceConfigElExcludeObjectsElObjectsEl {
    pub fn build(
        self,
    ) -> DatastreamStreamSourceConfigElSalesforceSourceConfigElExcludeObjectsElObjectsEl {
        DatastreamStreamSourceConfigElSalesforceSourceConfigElExcludeObjectsElObjectsEl {
            object_name: core::default::Default::default(),
            fields: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DatastreamStreamSourceConfigElSalesforceSourceConfigElExcludeObjectsElObjectsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamSourceConfigElSalesforceSourceConfigElExcludeObjectsElObjectsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamStreamSourceConfigElSalesforceSourceConfigElExcludeObjectsElObjectsElRef {
        DatastreamStreamSourceConfigElSalesforceSourceConfigElExcludeObjectsElObjectsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamSourceConfigElSalesforceSourceConfigElExcludeObjectsElObjectsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `object_name` after provisioning.\nName of object in Salesforce Org."]
    pub fn object_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.object_name", self.base))
    }
    #[doc = "Get a reference to the value of field `fields` after provisioning.\n"]
    pub fn fields(
        &self,
    ) -> ListRef<
        DatastreamStreamSourceConfigElSalesforceSourceConfigElExcludeObjectsElObjectsElFieldsElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.fields", self.base))
    }
}
#[derive(Serialize, Default)]
struct DatastreamStreamSourceConfigElSalesforceSourceConfigElExcludeObjectsElDynamic {
    objects: Option<
        DynamicBlock<
            DatastreamStreamSourceConfigElSalesforceSourceConfigElExcludeObjectsElObjectsEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElSalesforceSourceConfigElExcludeObjectsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    objects: Option<
        Vec<DatastreamStreamSourceConfigElSalesforceSourceConfigElExcludeObjectsElObjectsEl>,
    >,
    dynamic: DatastreamStreamSourceConfigElSalesforceSourceConfigElExcludeObjectsElDynamic,
}
impl DatastreamStreamSourceConfigElSalesforceSourceConfigElExcludeObjectsEl {
    #[doc = "Set the field `objects`.\n"]
    pub fn set_objects(
        mut self,
        v: impl Into<
            BlockAssignable<
                DatastreamStreamSourceConfigElSalesforceSourceConfigElExcludeObjectsElObjectsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.objects = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.objects = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DatastreamStreamSourceConfigElSalesforceSourceConfigElExcludeObjectsEl {
    type O =
        BlockAssignable<DatastreamStreamSourceConfigElSalesforceSourceConfigElExcludeObjectsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamSourceConfigElSalesforceSourceConfigElExcludeObjectsEl {}
impl BuildDatastreamStreamSourceConfigElSalesforceSourceConfigElExcludeObjectsEl {
    pub fn build(self) -> DatastreamStreamSourceConfigElSalesforceSourceConfigElExcludeObjectsEl {
        DatastreamStreamSourceConfigElSalesforceSourceConfigElExcludeObjectsEl {
            objects: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DatastreamStreamSourceConfigElSalesforceSourceConfigElExcludeObjectsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamSourceConfigElSalesforceSourceConfigElExcludeObjectsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamStreamSourceConfigElSalesforceSourceConfigElExcludeObjectsElRef {
        DatastreamStreamSourceConfigElSalesforceSourceConfigElExcludeObjectsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamSourceConfigElSalesforceSourceConfigElExcludeObjectsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `objects` after provisioning.\n"]
    pub fn objects(
        &self,
    ) -> ListRef<DatastreamStreamSourceConfigElSalesforceSourceConfigElExcludeObjectsElObjectsElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.objects", self.base))
    }
}
#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElSalesforceSourceConfigElIncludeObjectsElObjectsElFieldsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}
impl DatastreamStreamSourceConfigElSalesforceSourceConfigElIncludeObjectsElObjectsElFieldsEl {
    #[doc = "Set the field `name`.\nField name."]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}
impl ToListMappable
    for DatastreamStreamSourceConfigElSalesforceSourceConfigElIncludeObjectsElObjectsElFieldsEl
{
    type O = BlockAssignable<
        DatastreamStreamSourceConfigElSalesforceSourceConfigElIncludeObjectsElObjectsElFieldsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamSourceConfigElSalesforceSourceConfigElIncludeObjectsElObjectsElFieldsEl
{}
impl BuildDatastreamStreamSourceConfigElSalesforceSourceConfigElIncludeObjectsElObjectsElFieldsEl {
    pub fn build(
        self,
    ) -> DatastreamStreamSourceConfigElSalesforceSourceConfigElIncludeObjectsElObjectsElFieldsEl
    {
        DatastreamStreamSourceConfigElSalesforceSourceConfigElIncludeObjectsElObjectsElFieldsEl {
            name: core::default::Default::default(),
        }
    }
}
pub struct DatastreamStreamSourceConfigElSalesforceSourceConfigElIncludeObjectsElObjectsElFieldsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for DatastreamStreamSourceConfigElSalesforceSourceConfigElIncludeObjectsElObjectsElFieldsElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamStreamSourceConfigElSalesforceSourceConfigElIncludeObjectsElObjectsElFieldsElRef
    {
        DatastreamStreamSourceConfigElSalesforceSourceConfigElIncludeObjectsElObjectsElFieldsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamSourceConfigElSalesforceSourceConfigElIncludeObjectsElObjectsElFieldsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nField name."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
#[derive(Serialize, Default)]
struct DatastreamStreamSourceConfigElSalesforceSourceConfigElIncludeObjectsElObjectsElDynamic {
    fields: Option<
        DynamicBlock<
            DatastreamStreamSourceConfigElSalesforceSourceConfigElIncludeObjectsElObjectsElFieldsEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElSalesforceSourceConfigElIncludeObjectsElObjectsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    object_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fields: Option<
        Vec<
            DatastreamStreamSourceConfigElSalesforceSourceConfigElIncludeObjectsElObjectsElFieldsEl,
        >,
    >,
    dynamic: DatastreamStreamSourceConfigElSalesforceSourceConfigElIncludeObjectsElObjectsElDynamic,
}
impl DatastreamStreamSourceConfigElSalesforceSourceConfigElIncludeObjectsElObjectsEl {
    #[doc = "Set the field `object_name`.\nName of object in Salesforce Org."]
    pub fn set_object_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.object_name = Some(v.into());
        self
    }
    #[doc = "Set the field `fields`.\n"]
    pub fn set_fields(
        mut self,
        v : impl Into < BlockAssignable < DatastreamStreamSourceConfigElSalesforceSourceConfigElIncludeObjectsElObjectsElFieldsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.fields = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.fields = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for DatastreamStreamSourceConfigElSalesforceSourceConfigElIncludeObjectsElObjectsEl
{
    type O = BlockAssignable<
        DatastreamStreamSourceConfigElSalesforceSourceConfigElIncludeObjectsElObjectsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamSourceConfigElSalesforceSourceConfigElIncludeObjectsElObjectsEl {}
impl BuildDatastreamStreamSourceConfigElSalesforceSourceConfigElIncludeObjectsElObjectsEl {
    pub fn build(
        self,
    ) -> DatastreamStreamSourceConfigElSalesforceSourceConfigElIncludeObjectsElObjectsEl {
        DatastreamStreamSourceConfigElSalesforceSourceConfigElIncludeObjectsElObjectsEl {
            object_name: core::default::Default::default(),
            fields: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DatastreamStreamSourceConfigElSalesforceSourceConfigElIncludeObjectsElObjectsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamSourceConfigElSalesforceSourceConfigElIncludeObjectsElObjectsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamStreamSourceConfigElSalesforceSourceConfigElIncludeObjectsElObjectsElRef {
        DatastreamStreamSourceConfigElSalesforceSourceConfigElIncludeObjectsElObjectsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamSourceConfigElSalesforceSourceConfigElIncludeObjectsElObjectsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `object_name` after provisioning.\nName of object in Salesforce Org."]
    pub fn object_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.object_name", self.base))
    }
    #[doc = "Get a reference to the value of field `fields` after provisioning.\n"]
    pub fn fields(
        &self,
    ) -> ListRef<
        DatastreamStreamSourceConfigElSalesforceSourceConfigElIncludeObjectsElObjectsElFieldsElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.fields", self.base))
    }
}
#[derive(Serialize, Default)]
struct DatastreamStreamSourceConfigElSalesforceSourceConfigElIncludeObjectsElDynamic {
    objects: Option<
        DynamicBlock<
            DatastreamStreamSourceConfigElSalesforceSourceConfigElIncludeObjectsElObjectsEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElSalesforceSourceConfigElIncludeObjectsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    objects: Option<
        Vec<DatastreamStreamSourceConfigElSalesforceSourceConfigElIncludeObjectsElObjectsEl>,
    >,
    dynamic: DatastreamStreamSourceConfigElSalesforceSourceConfigElIncludeObjectsElDynamic,
}
impl DatastreamStreamSourceConfigElSalesforceSourceConfigElIncludeObjectsEl {
    #[doc = "Set the field `objects`.\n"]
    pub fn set_objects(
        mut self,
        v: impl Into<
            BlockAssignable<
                DatastreamStreamSourceConfigElSalesforceSourceConfigElIncludeObjectsElObjectsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.objects = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.objects = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DatastreamStreamSourceConfigElSalesforceSourceConfigElIncludeObjectsEl {
    type O =
        BlockAssignable<DatastreamStreamSourceConfigElSalesforceSourceConfigElIncludeObjectsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamSourceConfigElSalesforceSourceConfigElIncludeObjectsEl {}
impl BuildDatastreamStreamSourceConfigElSalesforceSourceConfigElIncludeObjectsEl {
    pub fn build(self) -> DatastreamStreamSourceConfigElSalesforceSourceConfigElIncludeObjectsEl {
        DatastreamStreamSourceConfigElSalesforceSourceConfigElIncludeObjectsEl {
            objects: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DatastreamStreamSourceConfigElSalesforceSourceConfigElIncludeObjectsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamSourceConfigElSalesforceSourceConfigElIncludeObjectsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamStreamSourceConfigElSalesforceSourceConfigElIncludeObjectsElRef {
        DatastreamStreamSourceConfigElSalesforceSourceConfigElIncludeObjectsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamSourceConfigElSalesforceSourceConfigElIncludeObjectsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `objects` after provisioning.\n"]
    pub fn objects(
        &self,
    ) -> ListRef<DatastreamStreamSourceConfigElSalesforceSourceConfigElIncludeObjectsElObjectsElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.objects", self.base))
    }
}
#[derive(Serialize, Default)]
struct DatastreamStreamSourceConfigElSalesforceSourceConfigElDynamic {
    exclude_objects: Option<
        DynamicBlock<DatastreamStreamSourceConfigElSalesforceSourceConfigElExcludeObjectsEl>,
    >,
    include_objects: Option<
        DynamicBlock<DatastreamStreamSourceConfigElSalesforceSourceConfigElIncludeObjectsEl>,
    >,
}
#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElSalesforceSourceConfigEl {
    polling_interval: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_objects:
        Option<Vec<DatastreamStreamSourceConfigElSalesforceSourceConfigElExcludeObjectsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_objects:
        Option<Vec<DatastreamStreamSourceConfigElSalesforceSourceConfigElIncludeObjectsEl>>,
    dynamic: DatastreamStreamSourceConfigElSalesforceSourceConfigElDynamic,
}
impl DatastreamStreamSourceConfigElSalesforceSourceConfigEl {
    #[doc = "Set the field `exclude_objects`.\n"]
    pub fn set_exclude_objects(
        mut self,
        v: impl Into<
            BlockAssignable<DatastreamStreamSourceConfigElSalesforceSourceConfigElExcludeObjectsEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.exclude_objects = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.exclude_objects = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `include_objects`.\n"]
    pub fn set_include_objects(
        mut self,
        v: impl Into<
            BlockAssignable<DatastreamStreamSourceConfigElSalesforceSourceConfigElIncludeObjectsEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.include_objects = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.include_objects = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DatastreamStreamSourceConfigElSalesforceSourceConfigEl {
    type O = BlockAssignable<DatastreamStreamSourceConfigElSalesforceSourceConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamSourceConfigElSalesforceSourceConfigEl {
    #[doc = "Salesforce objects polling interval. The interval at which new changes will be polled for each object. The duration must be between 5 minutes and 24 hours."]
    pub polling_interval: PrimField<String>,
}
impl BuildDatastreamStreamSourceConfigElSalesforceSourceConfigEl {
    pub fn build(self) -> DatastreamStreamSourceConfigElSalesforceSourceConfigEl {
        DatastreamStreamSourceConfigElSalesforceSourceConfigEl {
            polling_interval: self.polling_interval,
            exclude_objects: core::default::Default::default(),
            include_objects: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DatastreamStreamSourceConfigElSalesforceSourceConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamSourceConfigElSalesforceSourceConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamStreamSourceConfigElSalesforceSourceConfigElRef {
        DatastreamStreamSourceConfigElSalesforceSourceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamSourceConfigElSalesforceSourceConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `polling_interval` after provisioning.\nSalesforce objects polling interval. The interval at which new changes will be polled for each object. The duration must be between 5 minutes and 24 hours."]
    pub fn polling_interval(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.polling_interval", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `exclude_objects` after provisioning.\n"]
    pub fn exclude_objects(
        &self,
    ) -> ListRef<DatastreamStreamSourceConfigElSalesforceSourceConfigElExcludeObjectsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.exclude_objects", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `include_objects` after provisioning.\n"]
    pub fn include_objects(
        &self,
    ) -> ListRef<DatastreamStreamSourceConfigElSalesforceSourceConfigElIncludeObjectsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.include_objects", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElSpannerSourceConfigElExcludeObjectsElSchemasElTablesElColumnsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    column: Option<PrimField<String>>,
}
impl DatastreamStreamSourceConfigElSpannerSourceConfigElExcludeObjectsElSchemasElTablesElColumnsEl {
    #[doc = "Set the field `column`.\nColumn name."]
    pub fn set_column(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.column = Some(v.into());
        self
    }
}
impl ToListMappable for DatastreamStreamSourceConfigElSpannerSourceConfigElExcludeObjectsElSchemasElTablesElColumnsEl { type O = BlockAssignable < DatastreamStreamSourceConfigElSpannerSourceConfigElExcludeObjectsElSchemasElTablesElColumnsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDatastreamStreamSourceConfigElSpannerSourceConfigElExcludeObjectsElSchemasElTablesElColumnsEl
{}
impl BuildDatastreamStreamSourceConfigElSpannerSourceConfigElExcludeObjectsElSchemasElTablesElColumnsEl { pub fn build (self) -> DatastreamStreamSourceConfigElSpannerSourceConfigElExcludeObjectsElSchemasElTablesElColumnsEl { DatastreamStreamSourceConfigElSpannerSourceConfigElExcludeObjectsElSchemasElTablesElColumnsEl { column : core :: default :: Default :: default () , } } }
pub struct DatastreamStreamSourceConfigElSpannerSourceConfigElExcludeObjectsElSchemasElTablesElColumnsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamSourceConfigElSpannerSourceConfigElExcludeObjectsElSchemasElTablesElColumnsElRef { fn new (shared : StackShared , base : String) -> DatastreamStreamSourceConfigElSpannerSourceConfigElExcludeObjectsElSchemasElTablesElColumnsElRef { DatastreamStreamSourceConfigElSpannerSourceConfigElExcludeObjectsElSchemasElTablesElColumnsElRef { shared : shared , base : base . to_string () , } } }
impl
    DatastreamStreamSourceConfigElSpannerSourceConfigElExcludeObjectsElSchemasElTablesElColumnsElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `column` after provisioning.\nColumn name."]
    pub fn column(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.column", self.base))
    }
    #[doc = "Get a reference to the value of field `data_type` after provisioning.\nThe Spanner data type. Full data types list can be found here:\nhttps://docs.cloud.google.com/spanner/docs/reference/standard-sql/data-types"]
    pub fn data_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.data_type", self.base))
    }
    #[doc = "Get a reference to the value of field `is_primary_key` after provisioning.\nWhether the column is a primary key."]
    pub fn is_primary_key(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_primary_key", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ordinal_position` after provisioning.\nThe ordinal position of the column in the table."]
    pub fn ordinal_position(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ordinal_position", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DatastreamStreamSourceConfigElSpannerSourceConfigElExcludeObjectsElSchemasElTablesElDynamic { columns : Option < DynamicBlock < DatastreamStreamSourceConfigElSpannerSourceConfigElExcludeObjectsElSchemasElTablesElColumnsEl >> , }
#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElSpannerSourceConfigElExcludeObjectsElSchemasElTablesEl { table : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] columns : Option < Vec < DatastreamStreamSourceConfigElSpannerSourceConfigElExcludeObjectsElSchemasElTablesElColumnsEl > > , dynamic : DatastreamStreamSourceConfigElSpannerSourceConfigElExcludeObjectsElSchemasElTablesElDynamic , }
impl DatastreamStreamSourceConfigElSpannerSourceConfigElExcludeObjectsElSchemasElTablesEl {
    #[doc = "Set the field `columns`.\n"]
    pub fn set_columns(
        mut self,
        v : impl Into < BlockAssignable < DatastreamStreamSourceConfigElSpannerSourceConfigElExcludeObjectsElSchemasElTablesElColumnsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.columns = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.columns = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for DatastreamStreamSourceConfigElSpannerSourceConfigElExcludeObjectsElSchemasElTablesEl
{
    type O = BlockAssignable<
        DatastreamStreamSourceConfigElSpannerSourceConfigElExcludeObjectsElSchemasElTablesEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamSourceConfigElSpannerSourceConfigElExcludeObjectsElSchemasElTablesEl
{
    #[doc = "Table name."]
    pub table: PrimField<String>,
}
impl BuildDatastreamStreamSourceConfigElSpannerSourceConfigElExcludeObjectsElSchemasElTablesEl {
    pub fn build(
        self,
    ) -> DatastreamStreamSourceConfigElSpannerSourceConfigElExcludeObjectsElSchemasElTablesEl {
        DatastreamStreamSourceConfigElSpannerSourceConfigElExcludeObjectsElSchemasElTablesEl {
            table: self.table,
            columns: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DatastreamStreamSourceConfigElSpannerSourceConfigElExcludeObjectsElSchemasElTablesElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for DatastreamStreamSourceConfigElSpannerSourceConfigElExcludeObjectsElSchemasElTablesElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamStreamSourceConfigElSpannerSourceConfigElExcludeObjectsElSchemasElTablesElRef
    {
        DatastreamStreamSourceConfigElSpannerSourceConfigElExcludeObjectsElSchemasElTablesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamSourceConfigElSpannerSourceConfigElExcludeObjectsElSchemasElTablesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `table` after provisioning.\nTable name."]
    pub fn table(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.table", self.base))
    }
    #[doc = "Get a reference to the value of field `columns` after provisioning.\n"]    pub fn columns (& self) -> ListRef < DatastreamStreamSourceConfigElSpannerSourceConfigElExcludeObjectsElSchemasElTablesElColumnsElRef >{
        ListRef::new(self.shared().clone(), format!("{}.columns", self.base))
    }
}
#[derive(Serialize, Default)]
struct DatastreamStreamSourceConfigElSpannerSourceConfigElExcludeObjectsElSchemasElDynamic {
    tables: Option<
        DynamicBlock<
            DatastreamStreamSourceConfigElSpannerSourceConfigElExcludeObjectsElSchemasElTablesEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElSpannerSourceConfigElExcludeObjectsElSchemasEl {
    schema: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tables: Option<
        Vec<DatastreamStreamSourceConfigElSpannerSourceConfigElExcludeObjectsElSchemasElTablesEl>,
    >,
    dynamic: DatastreamStreamSourceConfigElSpannerSourceConfigElExcludeObjectsElSchemasElDynamic,
}
impl DatastreamStreamSourceConfigElSpannerSourceConfigElExcludeObjectsElSchemasEl {
    #[doc = "Set the field `tables`.\n"]
    pub fn set_tables(
        mut self,
        v : impl Into < BlockAssignable < DatastreamStreamSourceConfigElSpannerSourceConfigElExcludeObjectsElSchemasElTablesEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.tables = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.tables = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for DatastreamStreamSourceConfigElSpannerSourceConfigElExcludeObjectsElSchemasEl
{
    type O = BlockAssignable<
        DatastreamStreamSourceConfigElSpannerSourceConfigElExcludeObjectsElSchemasEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamSourceConfigElSpannerSourceConfigElExcludeObjectsElSchemasEl {
    #[doc = "Schema name."]
    pub schema: PrimField<String>,
}
impl BuildDatastreamStreamSourceConfigElSpannerSourceConfigElExcludeObjectsElSchemasEl {
    pub fn build(
        self,
    ) -> DatastreamStreamSourceConfigElSpannerSourceConfigElExcludeObjectsElSchemasEl {
        DatastreamStreamSourceConfigElSpannerSourceConfigElExcludeObjectsElSchemasEl {
            schema: self.schema,
            tables: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DatastreamStreamSourceConfigElSpannerSourceConfigElExcludeObjectsElSchemasElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamSourceConfigElSpannerSourceConfigElExcludeObjectsElSchemasElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamStreamSourceConfigElSpannerSourceConfigElExcludeObjectsElSchemasElRef {
        DatastreamStreamSourceConfigElSpannerSourceConfigElExcludeObjectsElSchemasElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamSourceConfigElSpannerSourceConfigElExcludeObjectsElSchemasElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `schema` after provisioning.\nSchema name."]
    pub fn schema(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.schema", self.base))
    }
    #[doc = "Get a reference to the value of field `tables` after provisioning.\n"]
    pub fn tables(
        &self,
    ) -> ListRef<
        DatastreamStreamSourceConfigElSpannerSourceConfigElExcludeObjectsElSchemasElTablesElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.tables", self.base))
    }
}
#[derive(Serialize, Default)]
struct DatastreamStreamSourceConfigElSpannerSourceConfigElExcludeObjectsElDynamic {
    schemas: Option<
        DynamicBlock<DatastreamStreamSourceConfigElSpannerSourceConfigElExcludeObjectsElSchemasEl>,
    >,
}
#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElSpannerSourceConfigElExcludeObjectsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    schemas:
        Option<Vec<DatastreamStreamSourceConfigElSpannerSourceConfigElExcludeObjectsElSchemasEl>>,
    dynamic: DatastreamStreamSourceConfigElSpannerSourceConfigElExcludeObjectsElDynamic,
}
impl DatastreamStreamSourceConfigElSpannerSourceConfigElExcludeObjectsEl {
    #[doc = "Set the field `schemas`.\n"]
    pub fn set_schemas(
        mut self,
        v: impl Into<
            BlockAssignable<
                DatastreamStreamSourceConfigElSpannerSourceConfigElExcludeObjectsElSchemasEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.schemas = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.schemas = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DatastreamStreamSourceConfigElSpannerSourceConfigElExcludeObjectsEl {
    type O = BlockAssignable<DatastreamStreamSourceConfigElSpannerSourceConfigElExcludeObjectsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamSourceConfigElSpannerSourceConfigElExcludeObjectsEl {}
impl BuildDatastreamStreamSourceConfigElSpannerSourceConfigElExcludeObjectsEl {
    pub fn build(self) -> DatastreamStreamSourceConfigElSpannerSourceConfigElExcludeObjectsEl {
        DatastreamStreamSourceConfigElSpannerSourceConfigElExcludeObjectsEl {
            schemas: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DatastreamStreamSourceConfigElSpannerSourceConfigElExcludeObjectsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamSourceConfigElSpannerSourceConfigElExcludeObjectsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamStreamSourceConfigElSpannerSourceConfigElExcludeObjectsElRef {
        DatastreamStreamSourceConfigElSpannerSourceConfigElExcludeObjectsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamSourceConfigElSpannerSourceConfigElExcludeObjectsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `schemas` after provisioning.\n"]
    pub fn schemas(
        &self,
    ) -> ListRef<DatastreamStreamSourceConfigElSpannerSourceConfigElExcludeObjectsElSchemasElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.schemas", self.base))
    }
}
#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElSpannerSourceConfigElIncludeObjectsElSchemasElTablesElColumnsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    column: Option<PrimField<String>>,
}
impl DatastreamStreamSourceConfigElSpannerSourceConfigElIncludeObjectsElSchemasElTablesElColumnsEl {
    #[doc = "Set the field `column`.\nColumn name."]
    pub fn set_column(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.column = Some(v.into());
        self
    }
}
impl ToListMappable for DatastreamStreamSourceConfigElSpannerSourceConfigElIncludeObjectsElSchemasElTablesElColumnsEl { type O = BlockAssignable < DatastreamStreamSourceConfigElSpannerSourceConfigElIncludeObjectsElSchemasElTablesElColumnsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDatastreamStreamSourceConfigElSpannerSourceConfigElIncludeObjectsElSchemasElTablesElColumnsEl
{}
impl BuildDatastreamStreamSourceConfigElSpannerSourceConfigElIncludeObjectsElSchemasElTablesElColumnsEl { pub fn build (self) -> DatastreamStreamSourceConfigElSpannerSourceConfigElIncludeObjectsElSchemasElTablesElColumnsEl { DatastreamStreamSourceConfigElSpannerSourceConfigElIncludeObjectsElSchemasElTablesElColumnsEl { column : core :: default :: Default :: default () , } } }
pub struct DatastreamStreamSourceConfigElSpannerSourceConfigElIncludeObjectsElSchemasElTablesElColumnsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamSourceConfigElSpannerSourceConfigElIncludeObjectsElSchemasElTablesElColumnsElRef { fn new (shared : StackShared , base : String) -> DatastreamStreamSourceConfigElSpannerSourceConfigElIncludeObjectsElSchemasElTablesElColumnsElRef { DatastreamStreamSourceConfigElSpannerSourceConfigElIncludeObjectsElSchemasElTablesElColumnsElRef { shared : shared , base : base . to_string () , } } }
impl
    DatastreamStreamSourceConfigElSpannerSourceConfigElIncludeObjectsElSchemasElTablesElColumnsElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `column` after provisioning.\nColumn name."]
    pub fn column(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.column", self.base))
    }
    #[doc = "Get a reference to the value of field `data_type` after provisioning.\nThe Spanner data type. Full data types list can be found here:\nhttps://docs.cloud.google.com/spanner/docs/reference/standard-sql/data-types"]
    pub fn data_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.data_type", self.base))
    }
    #[doc = "Get a reference to the value of field `is_primary_key` after provisioning.\nWhether or not the column is a primary key."]
    pub fn is_primary_key(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_primary_key", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ordinal_position` after provisioning.\nThe ordinal position of the column in the table."]
    pub fn ordinal_position(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ordinal_position", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DatastreamStreamSourceConfigElSpannerSourceConfigElIncludeObjectsElSchemasElTablesElDynamic { columns : Option < DynamicBlock < DatastreamStreamSourceConfigElSpannerSourceConfigElIncludeObjectsElSchemasElTablesElColumnsEl >> , }
#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElSpannerSourceConfigElIncludeObjectsElSchemasElTablesEl { table : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] columns : Option < Vec < DatastreamStreamSourceConfigElSpannerSourceConfigElIncludeObjectsElSchemasElTablesElColumnsEl > > , dynamic : DatastreamStreamSourceConfigElSpannerSourceConfigElIncludeObjectsElSchemasElTablesElDynamic , }
impl DatastreamStreamSourceConfigElSpannerSourceConfigElIncludeObjectsElSchemasElTablesEl {
    #[doc = "Set the field `columns`.\n"]
    pub fn set_columns(
        mut self,
        v : impl Into < BlockAssignable < DatastreamStreamSourceConfigElSpannerSourceConfigElIncludeObjectsElSchemasElTablesElColumnsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.columns = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.columns = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for DatastreamStreamSourceConfigElSpannerSourceConfigElIncludeObjectsElSchemasElTablesEl
{
    type O = BlockAssignable<
        DatastreamStreamSourceConfigElSpannerSourceConfigElIncludeObjectsElSchemasElTablesEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamSourceConfigElSpannerSourceConfigElIncludeObjectsElSchemasElTablesEl
{
    #[doc = "Table name."]
    pub table: PrimField<String>,
}
impl BuildDatastreamStreamSourceConfigElSpannerSourceConfigElIncludeObjectsElSchemasElTablesEl {
    pub fn build(
        self,
    ) -> DatastreamStreamSourceConfigElSpannerSourceConfigElIncludeObjectsElSchemasElTablesEl {
        DatastreamStreamSourceConfigElSpannerSourceConfigElIncludeObjectsElSchemasElTablesEl {
            table: self.table,
            columns: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DatastreamStreamSourceConfigElSpannerSourceConfigElIncludeObjectsElSchemasElTablesElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for DatastreamStreamSourceConfigElSpannerSourceConfigElIncludeObjectsElSchemasElTablesElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamStreamSourceConfigElSpannerSourceConfigElIncludeObjectsElSchemasElTablesElRef
    {
        DatastreamStreamSourceConfigElSpannerSourceConfigElIncludeObjectsElSchemasElTablesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamSourceConfigElSpannerSourceConfigElIncludeObjectsElSchemasElTablesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `table` after provisioning.\nTable name."]
    pub fn table(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.table", self.base))
    }
    #[doc = "Get a reference to the value of field `columns` after provisioning.\n"]    pub fn columns (& self) -> ListRef < DatastreamStreamSourceConfigElSpannerSourceConfigElIncludeObjectsElSchemasElTablesElColumnsElRef >{
        ListRef::new(self.shared().clone(), format!("{}.columns", self.base))
    }
}
#[derive(Serialize, Default)]
struct DatastreamStreamSourceConfigElSpannerSourceConfigElIncludeObjectsElSchemasElDynamic {
    tables: Option<
        DynamicBlock<
            DatastreamStreamSourceConfigElSpannerSourceConfigElIncludeObjectsElSchemasElTablesEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElSpannerSourceConfigElIncludeObjectsElSchemasEl {
    schema: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tables: Option<
        Vec<DatastreamStreamSourceConfigElSpannerSourceConfigElIncludeObjectsElSchemasElTablesEl>,
    >,
    dynamic: DatastreamStreamSourceConfigElSpannerSourceConfigElIncludeObjectsElSchemasElDynamic,
}
impl DatastreamStreamSourceConfigElSpannerSourceConfigElIncludeObjectsElSchemasEl {
    #[doc = "Set the field `tables`.\n"]
    pub fn set_tables(
        mut self,
        v : impl Into < BlockAssignable < DatastreamStreamSourceConfigElSpannerSourceConfigElIncludeObjectsElSchemasElTablesEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.tables = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.tables = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for DatastreamStreamSourceConfigElSpannerSourceConfigElIncludeObjectsElSchemasEl
{
    type O = BlockAssignable<
        DatastreamStreamSourceConfigElSpannerSourceConfigElIncludeObjectsElSchemasEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamSourceConfigElSpannerSourceConfigElIncludeObjectsElSchemasEl {
    #[doc = "Schema name."]
    pub schema: PrimField<String>,
}
impl BuildDatastreamStreamSourceConfigElSpannerSourceConfigElIncludeObjectsElSchemasEl {
    pub fn build(
        self,
    ) -> DatastreamStreamSourceConfigElSpannerSourceConfigElIncludeObjectsElSchemasEl {
        DatastreamStreamSourceConfigElSpannerSourceConfigElIncludeObjectsElSchemasEl {
            schema: self.schema,
            tables: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DatastreamStreamSourceConfigElSpannerSourceConfigElIncludeObjectsElSchemasElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamSourceConfigElSpannerSourceConfigElIncludeObjectsElSchemasElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamStreamSourceConfigElSpannerSourceConfigElIncludeObjectsElSchemasElRef {
        DatastreamStreamSourceConfigElSpannerSourceConfigElIncludeObjectsElSchemasElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamSourceConfigElSpannerSourceConfigElIncludeObjectsElSchemasElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `schema` after provisioning.\nSchema name."]
    pub fn schema(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.schema", self.base))
    }
    #[doc = "Get a reference to the value of field `tables` after provisioning.\n"]
    pub fn tables(
        &self,
    ) -> ListRef<
        DatastreamStreamSourceConfigElSpannerSourceConfigElIncludeObjectsElSchemasElTablesElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.tables", self.base))
    }
}
#[derive(Serialize, Default)]
struct DatastreamStreamSourceConfigElSpannerSourceConfigElIncludeObjectsElDynamic {
    schemas: Option<
        DynamicBlock<DatastreamStreamSourceConfigElSpannerSourceConfigElIncludeObjectsElSchemasEl>,
    >,
}
#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElSpannerSourceConfigElIncludeObjectsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    schemas:
        Option<Vec<DatastreamStreamSourceConfigElSpannerSourceConfigElIncludeObjectsElSchemasEl>>,
    dynamic: DatastreamStreamSourceConfigElSpannerSourceConfigElIncludeObjectsElDynamic,
}
impl DatastreamStreamSourceConfigElSpannerSourceConfigElIncludeObjectsEl {
    #[doc = "Set the field `schemas`.\n"]
    pub fn set_schemas(
        mut self,
        v: impl Into<
            BlockAssignable<
                DatastreamStreamSourceConfigElSpannerSourceConfigElIncludeObjectsElSchemasEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.schemas = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.schemas = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DatastreamStreamSourceConfigElSpannerSourceConfigElIncludeObjectsEl {
    type O = BlockAssignable<DatastreamStreamSourceConfigElSpannerSourceConfigElIncludeObjectsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamSourceConfigElSpannerSourceConfigElIncludeObjectsEl {}
impl BuildDatastreamStreamSourceConfigElSpannerSourceConfigElIncludeObjectsEl {
    pub fn build(self) -> DatastreamStreamSourceConfigElSpannerSourceConfigElIncludeObjectsEl {
        DatastreamStreamSourceConfigElSpannerSourceConfigElIncludeObjectsEl {
            schemas: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DatastreamStreamSourceConfigElSpannerSourceConfigElIncludeObjectsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamSourceConfigElSpannerSourceConfigElIncludeObjectsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamStreamSourceConfigElSpannerSourceConfigElIncludeObjectsElRef {
        DatastreamStreamSourceConfigElSpannerSourceConfigElIncludeObjectsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamSourceConfigElSpannerSourceConfigElIncludeObjectsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `schemas` after provisioning.\n"]
    pub fn schemas(
        &self,
    ) -> ListRef<DatastreamStreamSourceConfigElSpannerSourceConfigElIncludeObjectsElSchemasElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.schemas", self.base))
    }
}
#[derive(Serialize, Default)]
struct DatastreamStreamSourceConfigElSpannerSourceConfigElDynamic {
    exclude_objects:
        Option<DynamicBlock<DatastreamStreamSourceConfigElSpannerSourceConfigElExcludeObjectsEl>>,
    include_objects:
        Option<DynamicBlock<DatastreamStreamSourceConfigElSpannerSourceConfigElIncludeObjectsEl>>,
}
#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElSpannerSourceConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    backfill_data_boost_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    change_stream_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fgac_role: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_concurrent_backfill_tasks: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_concurrent_cdc_tasks: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spanner_rpc_priority: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_objects:
        Option<Vec<DatastreamStreamSourceConfigElSpannerSourceConfigElExcludeObjectsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_objects:
        Option<Vec<DatastreamStreamSourceConfigElSpannerSourceConfigElIncludeObjectsEl>>,
    dynamic: DatastreamStreamSourceConfigElSpannerSourceConfigElDynamic,
}
impl DatastreamStreamSourceConfigElSpannerSourceConfigEl {
    #[doc = "Set the field `backfill_data_boost_enabled`.\nWhether to use DataBoost for backfill queries."]
    pub fn set_backfill_data_boost_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.backfill_data_boost_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `change_stream_name`.\nThe Spanner change stream name to use."]
    pub fn set_change_stream_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.change_stream_name = Some(v.into());
        self
    }
    #[doc = "Set the field `fgac_role`.\nThe FGAC role to use for Spanner queries."]
    pub fn set_fgac_role(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.fgac_role = Some(v.into());
        self
    }
    #[doc = "Set the field `max_concurrent_backfill_tasks`.\nMax concurrent backfill tasks."]
    pub fn set_max_concurrent_backfill_tasks(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_concurrent_backfill_tasks = Some(v.into());
        self
    }
    #[doc = "Set the field `max_concurrent_cdc_tasks`.\nMax concurrent CDC tasks."]
    pub fn set_max_concurrent_cdc_tasks(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_concurrent_cdc_tasks = Some(v.into());
        self
    }
    #[doc = "Set the field `spanner_rpc_priority`.\nThe RPC priority to use for Spanner queries. Possible values: [\"LOW\", \"MEDIUM\", \"HIGH\"]"]
    pub fn set_spanner_rpc_priority(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.spanner_rpc_priority = Some(v.into());
        self
    }
    #[doc = "Set the field `exclude_objects`.\n"]
    pub fn set_exclude_objects(
        mut self,
        v: impl Into<
            BlockAssignable<DatastreamStreamSourceConfigElSpannerSourceConfigElExcludeObjectsEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.exclude_objects = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.exclude_objects = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `include_objects`.\n"]
    pub fn set_include_objects(
        mut self,
        v: impl Into<
            BlockAssignable<DatastreamStreamSourceConfigElSpannerSourceConfigElIncludeObjectsEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.include_objects = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.include_objects = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DatastreamStreamSourceConfigElSpannerSourceConfigEl {
    type O = BlockAssignable<DatastreamStreamSourceConfigElSpannerSourceConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamSourceConfigElSpannerSourceConfigEl {}
impl BuildDatastreamStreamSourceConfigElSpannerSourceConfigEl {
    pub fn build(self) -> DatastreamStreamSourceConfigElSpannerSourceConfigEl {
        DatastreamStreamSourceConfigElSpannerSourceConfigEl {
            backfill_data_boost_enabled: core::default::Default::default(),
            change_stream_name: core::default::Default::default(),
            fgac_role: core::default::Default::default(),
            max_concurrent_backfill_tasks: core::default::Default::default(),
            max_concurrent_cdc_tasks: core::default::Default::default(),
            spanner_rpc_priority: core::default::Default::default(),
            exclude_objects: core::default::Default::default(),
            include_objects: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DatastreamStreamSourceConfigElSpannerSourceConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamSourceConfigElSpannerSourceConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamStreamSourceConfigElSpannerSourceConfigElRef {
        DatastreamStreamSourceConfigElSpannerSourceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamSourceConfigElSpannerSourceConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `backfill_data_boost_enabled` after provisioning.\nWhether to use DataBoost for backfill queries."]
    pub fn backfill_data_boost_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backfill_data_boost_enabled", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `change_stream_name` after provisioning.\nThe Spanner change stream name to use."]
    pub fn change_stream_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.change_stream_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `fgac_role` after provisioning.\nThe FGAC role to use for Spanner queries."]
    pub fn fgac_role(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.fgac_role", self.base))
    }
    #[doc = "Get a reference to the value of field `max_concurrent_backfill_tasks` after provisioning.\nMax concurrent backfill tasks."]
    pub fn max_concurrent_backfill_tasks(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_concurrent_backfill_tasks", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_concurrent_cdc_tasks` after provisioning.\nMax concurrent CDC tasks."]
    pub fn max_concurrent_cdc_tasks(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_concurrent_cdc_tasks", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `spanner_rpc_priority` after provisioning.\nThe RPC priority to use for Spanner queries. Possible values: [\"LOW\", \"MEDIUM\", \"HIGH\"]"]
    pub fn spanner_rpc_priority(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.spanner_rpc_priority", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `exclude_objects` after provisioning.\n"]
    pub fn exclude_objects(
        &self,
    ) -> ListRef<DatastreamStreamSourceConfigElSpannerSourceConfigElExcludeObjectsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.exclude_objects", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `include_objects` after provisioning.\n"]
    pub fn include_objects(
        &self,
    ) -> ListRef<DatastreamStreamSourceConfigElSpannerSourceConfigElIncludeObjectsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.include_objects", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElSqlServerSourceConfigElChangeTablesEl {}
impl DatastreamStreamSourceConfigElSqlServerSourceConfigElChangeTablesEl {}
impl ToListMappable for DatastreamStreamSourceConfigElSqlServerSourceConfigElChangeTablesEl {
    type O = BlockAssignable<DatastreamStreamSourceConfigElSqlServerSourceConfigElChangeTablesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamSourceConfigElSqlServerSourceConfigElChangeTablesEl {}
impl BuildDatastreamStreamSourceConfigElSqlServerSourceConfigElChangeTablesEl {
    pub fn build(self) -> DatastreamStreamSourceConfigElSqlServerSourceConfigElChangeTablesEl {
        DatastreamStreamSourceConfigElSqlServerSourceConfigElChangeTablesEl {}
    }
}
pub struct DatastreamStreamSourceConfigElSqlServerSourceConfigElChangeTablesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamSourceConfigElSqlServerSourceConfigElChangeTablesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamStreamSourceConfigElSqlServerSourceConfigElChangeTablesElRef {
        DatastreamStreamSourceConfigElSqlServerSourceConfigElChangeTablesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamSourceConfigElSqlServerSourceConfigElChangeTablesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElSqlServerSourceConfigElExcludeObjectsElSchemasElTablesElColumnsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    column: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_type: Option<PrimField<String>>,
}
impl
    DatastreamStreamSourceConfigElSqlServerSourceConfigElExcludeObjectsElSchemasElTablesElColumnsEl
{
    #[doc = "Set the field `column`.\nColumn name."]
    pub fn set_column(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.column = Some(v.into());
        self
    }
    #[doc = "Set the field `data_type`.\nThe SQL Server data type. Full data types list can be found here:\nhttps://learn.microsoft.com/en-us/sql/t-sql/data-types/data-types-transact-sql?view=sql-server-ver16"]
    pub fn set_data_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.data_type = Some(v.into());
        self
    }
}
impl ToListMappable for DatastreamStreamSourceConfigElSqlServerSourceConfigElExcludeObjectsElSchemasElTablesElColumnsEl { type O = BlockAssignable < DatastreamStreamSourceConfigElSqlServerSourceConfigElExcludeObjectsElSchemasElTablesElColumnsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDatastreamStreamSourceConfigElSqlServerSourceConfigElExcludeObjectsElSchemasElTablesElColumnsEl
{}
impl BuildDatastreamStreamSourceConfigElSqlServerSourceConfigElExcludeObjectsElSchemasElTablesElColumnsEl { pub fn build (self) -> DatastreamStreamSourceConfigElSqlServerSourceConfigElExcludeObjectsElSchemasElTablesElColumnsEl { DatastreamStreamSourceConfigElSqlServerSourceConfigElExcludeObjectsElSchemasElTablesElColumnsEl { column : core :: default :: Default :: default () , data_type : core :: default :: Default :: default () , } } }
pub struct DatastreamStreamSourceConfigElSqlServerSourceConfigElExcludeObjectsElSchemasElTablesElColumnsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamSourceConfigElSqlServerSourceConfigElExcludeObjectsElSchemasElTablesElColumnsElRef { fn new (shared : StackShared , base : String) -> DatastreamStreamSourceConfigElSqlServerSourceConfigElExcludeObjectsElSchemasElTablesElColumnsElRef { DatastreamStreamSourceConfigElSqlServerSourceConfigElExcludeObjectsElSchemasElTablesElColumnsElRef { shared : shared , base : base . to_string () , } } }
impl DatastreamStreamSourceConfigElSqlServerSourceConfigElExcludeObjectsElSchemasElTablesElColumnsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `column` after provisioning.\nColumn name."] pub fn column (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.column" , self . base)) } # [doc = "Get a reference to the value of field `data_type` after provisioning.\nThe SQL Server data type. Full data types list can be found here:\nhttps://learn.microsoft.com/en-us/sql/t-sql/data-types/data-types-transact-sql?view=sql-server-ver16"] pub fn data_type (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.data_type" , self . base)) } # [doc = "Get a reference to the value of field `length` after provisioning.\nColumn length."] pub fn length (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.length" , self . base)) } # [doc = "Get a reference to the value of field `nullable` after provisioning.\nWhether or not the column can accept a null value."] pub fn nullable (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.nullable" , self . base)) } # [doc = "Get a reference to the value of field `ordinal_position` after provisioning.\nThe ordinal position of the column in the table."] pub fn ordinal_position (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.ordinal_position" , self . base)) } # [doc = "Get a reference to the value of field `precision` after provisioning.\nColumn precision."] pub fn precision (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.precision" , self . base)) } # [doc = "Get a reference to the value of field `primary_key` after provisioning.\nWhether or not the column represents a primary key."] pub fn primary_key (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.primary_key" , self . base)) } # [doc = "Get a reference to the value of field `scale` after provisioning.\nColumn scale."] pub fn scale (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.scale" , self . base)) } }
#[derive(Serialize, Default)]
struct DatastreamStreamSourceConfigElSqlServerSourceConfigElExcludeObjectsElSchemasElTablesElDynamic { columns : Option < DynamicBlock < DatastreamStreamSourceConfigElSqlServerSourceConfigElExcludeObjectsElSchemasElTablesElColumnsEl >> , }
#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElSqlServerSourceConfigElExcludeObjectsElSchemasElTablesEl { table : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] columns : Option < Vec < DatastreamStreamSourceConfigElSqlServerSourceConfigElExcludeObjectsElSchemasElTablesElColumnsEl > > , dynamic : DatastreamStreamSourceConfigElSqlServerSourceConfigElExcludeObjectsElSchemasElTablesElDynamic , }
impl DatastreamStreamSourceConfigElSqlServerSourceConfigElExcludeObjectsElSchemasElTablesEl {
    #[doc = "Set the field `columns`.\n"]
    pub fn set_columns(
        mut self,
        v : impl Into < BlockAssignable < DatastreamStreamSourceConfigElSqlServerSourceConfigElExcludeObjectsElSchemasElTablesElColumnsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.columns = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.columns = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for DatastreamStreamSourceConfigElSqlServerSourceConfigElExcludeObjectsElSchemasElTablesEl
{
    type O = BlockAssignable<
        DatastreamStreamSourceConfigElSqlServerSourceConfigElExcludeObjectsElSchemasElTablesEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamSourceConfigElSqlServerSourceConfigElExcludeObjectsElSchemasElTablesEl
{
    #[doc = "Table name."]
    pub table: PrimField<String>,
}
impl BuildDatastreamStreamSourceConfigElSqlServerSourceConfigElExcludeObjectsElSchemasElTablesEl {
    pub fn build(
        self,
    ) -> DatastreamStreamSourceConfigElSqlServerSourceConfigElExcludeObjectsElSchemasElTablesEl
    {
        DatastreamStreamSourceConfigElSqlServerSourceConfigElExcludeObjectsElSchemasElTablesEl {
            table: self.table,
            columns: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DatastreamStreamSourceConfigElSqlServerSourceConfigElExcludeObjectsElSchemasElTablesElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for DatastreamStreamSourceConfigElSqlServerSourceConfigElExcludeObjectsElSchemasElTablesElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamStreamSourceConfigElSqlServerSourceConfigElExcludeObjectsElSchemasElTablesElRef
    {
        DatastreamStreamSourceConfigElSqlServerSourceConfigElExcludeObjectsElSchemasElTablesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamSourceConfigElSqlServerSourceConfigElExcludeObjectsElSchemasElTablesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `table` after provisioning.\nTable name."]
    pub fn table(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.table", self.base))
    }
    #[doc = "Get a reference to the value of field `columns` after provisioning.\n"]    pub fn columns (& self) -> ListRef < DatastreamStreamSourceConfigElSqlServerSourceConfigElExcludeObjectsElSchemasElTablesElColumnsElRef >{
        ListRef::new(self.shared().clone(), format!("{}.columns", self.base))
    }
}
#[derive(Serialize, Default)]
struct DatastreamStreamSourceConfigElSqlServerSourceConfigElExcludeObjectsElSchemasElDynamic {
    tables: Option<
        DynamicBlock<
            DatastreamStreamSourceConfigElSqlServerSourceConfigElExcludeObjectsElSchemasElTablesEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElSqlServerSourceConfigElExcludeObjectsElSchemasEl {
    schema: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tables: Option<
        Vec<DatastreamStreamSourceConfigElSqlServerSourceConfigElExcludeObjectsElSchemasElTablesEl>,
    >,
    dynamic: DatastreamStreamSourceConfigElSqlServerSourceConfigElExcludeObjectsElSchemasElDynamic,
}
impl DatastreamStreamSourceConfigElSqlServerSourceConfigElExcludeObjectsElSchemasEl {
    #[doc = "Set the field `tables`.\n"]
    pub fn set_tables(
        mut self,
        v : impl Into < BlockAssignable < DatastreamStreamSourceConfigElSqlServerSourceConfigElExcludeObjectsElSchemasElTablesEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.tables = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.tables = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for DatastreamStreamSourceConfigElSqlServerSourceConfigElExcludeObjectsElSchemasEl
{
    type O = BlockAssignable<
        DatastreamStreamSourceConfigElSqlServerSourceConfigElExcludeObjectsElSchemasEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamSourceConfigElSqlServerSourceConfigElExcludeObjectsElSchemasEl {
    #[doc = "Schema name."]
    pub schema: PrimField<String>,
}
impl BuildDatastreamStreamSourceConfigElSqlServerSourceConfigElExcludeObjectsElSchemasEl {
    pub fn build(
        self,
    ) -> DatastreamStreamSourceConfigElSqlServerSourceConfigElExcludeObjectsElSchemasEl {
        DatastreamStreamSourceConfigElSqlServerSourceConfigElExcludeObjectsElSchemasEl {
            schema: self.schema,
            tables: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DatastreamStreamSourceConfigElSqlServerSourceConfigElExcludeObjectsElSchemasElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamSourceConfigElSqlServerSourceConfigElExcludeObjectsElSchemasElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamStreamSourceConfigElSqlServerSourceConfigElExcludeObjectsElSchemasElRef {
        DatastreamStreamSourceConfigElSqlServerSourceConfigElExcludeObjectsElSchemasElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamSourceConfigElSqlServerSourceConfigElExcludeObjectsElSchemasElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `schema` after provisioning.\nSchema name."]
    pub fn schema(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.schema", self.base))
    }
    #[doc = "Get a reference to the value of field `tables` after provisioning.\n"]
    pub fn tables(
        &self,
    ) -> ListRef<
        DatastreamStreamSourceConfigElSqlServerSourceConfigElExcludeObjectsElSchemasElTablesElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.tables", self.base))
    }
}
#[derive(Serialize, Default)]
struct DatastreamStreamSourceConfigElSqlServerSourceConfigElExcludeObjectsElDynamic {
    schemas: Option<
        DynamicBlock<
            DatastreamStreamSourceConfigElSqlServerSourceConfigElExcludeObjectsElSchemasEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElSqlServerSourceConfigElExcludeObjectsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    schemas:
        Option<Vec<DatastreamStreamSourceConfigElSqlServerSourceConfigElExcludeObjectsElSchemasEl>>,
    dynamic: DatastreamStreamSourceConfigElSqlServerSourceConfigElExcludeObjectsElDynamic,
}
impl DatastreamStreamSourceConfigElSqlServerSourceConfigElExcludeObjectsEl {
    #[doc = "Set the field `schemas`.\n"]
    pub fn set_schemas(
        mut self,
        v: impl Into<
            BlockAssignable<
                DatastreamStreamSourceConfigElSqlServerSourceConfigElExcludeObjectsElSchemasEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.schemas = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.schemas = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DatastreamStreamSourceConfigElSqlServerSourceConfigElExcludeObjectsEl {
    type O = BlockAssignable<DatastreamStreamSourceConfigElSqlServerSourceConfigElExcludeObjectsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamSourceConfigElSqlServerSourceConfigElExcludeObjectsEl {}
impl BuildDatastreamStreamSourceConfigElSqlServerSourceConfigElExcludeObjectsEl {
    pub fn build(self) -> DatastreamStreamSourceConfigElSqlServerSourceConfigElExcludeObjectsEl {
        DatastreamStreamSourceConfigElSqlServerSourceConfigElExcludeObjectsEl {
            schemas: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DatastreamStreamSourceConfigElSqlServerSourceConfigElExcludeObjectsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamSourceConfigElSqlServerSourceConfigElExcludeObjectsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamStreamSourceConfigElSqlServerSourceConfigElExcludeObjectsElRef {
        DatastreamStreamSourceConfigElSqlServerSourceConfigElExcludeObjectsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamSourceConfigElSqlServerSourceConfigElExcludeObjectsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `schemas` after provisioning.\n"]
    pub fn schemas(
        &self,
    ) -> ListRef<DatastreamStreamSourceConfigElSqlServerSourceConfigElExcludeObjectsElSchemasElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.schemas", self.base))
    }
}
#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElSqlServerSourceConfigElIncludeObjectsElSchemasElTablesElColumnsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    column: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_type: Option<PrimField<String>>,
}
impl
    DatastreamStreamSourceConfigElSqlServerSourceConfigElIncludeObjectsElSchemasElTablesElColumnsEl
{
    #[doc = "Set the field `column`.\nColumn name."]
    pub fn set_column(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.column = Some(v.into());
        self
    }
    #[doc = "Set the field `data_type`.\nThe SQL Server data type. Full data types list can be found here:\nhttps://learn.microsoft.com/en-us/sql/t-sql/data-types/data-types-transact-sql?view=sql-server-ver16"]
    pub fn set_data_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.data_type = Some(v.into());
        self
    }
}
impl ToListMappable for DatastreamStreamSourceConfigElSqlServerSourceConfigElIncludeObjectsElSchemasElTablesElColumnsEl { type O = BlockAssignable < DatastreamStreamSourceConfigElSqlServerSourceConfigElIncludeObjectsElSchemasElTablesElColumnsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDatastreamStreamSourceConfigElSqlServerSourceConfigElIncludeObjectsElSchemasElTablesElColumnsEl
{}
impl BuildDatastreamStreamSourceConfigElSqlServerSourceConfigElIncludeObjectsElSchemasElTablesElColumnsEl { pub fn build (self) -> DatastreamStreamSourceConfigElSqlServerSourceConfigElIncludeObjectsElSchemasElTablesElColumnsEl { DatastreamStreamSourceConfigElSqlServerSourceConfigElIncludeObjectsElSchemasElTablesElColumnsEl { column : core :: default :: Default :: default () , data_type : core :: default :: Default :: default () , } } }
pub struct DatastreamStreamSourceConfigElSqlServerSourceConfigElIncludeObjectsElSchemasElTablesElColumnsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamSourceConfigElSqlServerSourceConfigElIncludeObjectsElSchemasElTablesElColumnsElRef { fn new (shared : StackShared , base : String) -> DatastreamStreamSourceConfigElSqlServerSourceConfigElIncludeObjectsElSchemasElTablesElColumnsElRef { DatastreamStreamSourceConfigElSqlServerSourceConfigElIncludeObjectsElSchemasElTablesElColumnsElRef { shared : shared , base : base . to_string () , } } }
impl DatastreamStreamSourceConfigElSqlServerSourceConfigElIncludeObjectsElSchemasElTablesElColumnsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `column` after provisioning.\nColumn name."] pub fn column (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.column" , self . base)) } # [doc = "Get a reference to the value of field `data_type` after provisioning.\nThe SQL Server data type. Full data types list can be found here:\nhttps://learn.microsoft.com/en-us/sql/t-sql/data-types/data-types-transact-sql?view=sql-server-ver16"] pub fn data_type (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.data_type" , self . base)) } # [doc = "Get a reference to the value of field `length` after provisioning.\nColumn length."] pub fn length (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.length" , self . base)) } # [doc = "Get a reference to the value of field `nullable` after provisioning.\nWhether or not the column can accept a null value."] pub fn nullable (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.nullable" , self . base)) } # [doc = "Get a reference to the value of field `ordinal_position` after provisioning.\nThe ordinal position of the column in the table."] pub fn ordinal_position (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.ordinal_position" , self . base)) } # [doc = "Get a reference to the value of field `precision` after provisioning.\nColumn precision."] pub fn precision (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.precision" , self . base)) } # [doc = "Get a reference to the value of field `primary_key` after provisioning.\nWhether or not the column represents a primary key."] pub fn primary_key (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.primary_key" , self . base)) } # [doc = "Get a reference to the value of field `scale` after provisioning.\nColumn scale."] pub fn scale (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.scale" , self . base)) } }
#[derive(Serialize, Default)]
struct DatastreamStreamSourceConfigElSqlServerSourceConfigElIncludeObjectsElSchemasElTablesElDynamic { columns : Option < DynamicBlock < DatastreamStreamSourceConfigElSqlServerSourceConfigElIncludeObjectsElSchemasElTablesElColumnsEl >> , }
#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElSqlServerSourceConfigElIncludeObjectsElSchemasElTablesEl { table : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] columns : Option < Vec < DatastreamStreamSourceConfigElSqlServerSourceConfigElIncludeObjectsElSchemasElTablesElColumnsEl > > , dynamic : DatastreamStreamSourceConfigElSqlServerSourceConfigElIncludeObjectsElSchemasElTablesElDynamic , }
impl DatastreamStreamSourceConfigElSqlServerSourceConfigElIncludeObjectsElSchemasElTablesEl {
    #[doc = "Set the field `columns`.\n"]
    pub fn set_columns(
        mut self,
        v : impl Into < BlockAssignable < DatastreamStreamSourceConfigElSqlServerSourceConfigElIncludeObjectsElSchemasElTablesElColumnsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.columns = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.columns = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for DatastreamStreamSourceConfigElSqlServerSourceConfigElIncludeObjectsElSchemasElTablesEl
{
    type O = BlockAssignable<
        DatastreamStreamSourceConfigElSqlServerSourceConfigElIncludeObjectsElSchemasElTablesEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamSourceConfigElSqlServerSourceConfigElIncludeObjectsElSchemasElTablesEl
{
    #[doc = "Table name."]
    pub table: PrimField<String>,
}
impl BuildDatastreamStreamSourceConfigElSqlServerSourceConfigElIncludeObjectsElSchemasElTablesEl {
    pub fn build(
        self,
    ) -> DatastreamStreamSourceConfigElSqlServerSourceConfigElIncludeObjectsElSchemasElTablesEl
    {
        DatastreamStreamSourceConfigElSqlServerSourceConfigElIncludeObjectsElSchemasElTablesEl {
            table: self.table,
            columns: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DatastreamStreamSourceConfigElSqlServerSourceConfigElIncludeObjectsElSchemasElTablesElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for DatastreamStreamSourceConfigElSqlServerSourceConfigElIncludeObjectsElSchemasElTablesElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamStreamSourceConfigElSqlServerSourceConfigElIncludeObjectsElSchemasElTablesElRef
    {
        DatastreamStreamSourceConfigElSqlServerSourceConfigElIncludeObjectsElSchemasElTablesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamSourceConfigElSqlServerSourceConfigElIncludeObjectsElSchemasElTablesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `table` after provisioning.\nTable name."]
    pub fn table(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.table", self.base))
    }
    #[doc = "Get a reference to the value of field `columns` after provisioning.\n"]    pub fn columns (& self) -> ListRef < DatastreamStreamSourceConfigElSqlServerSourceConfigElIncludeObjectsElSchemasElTablesElColumnsElRef >{
        ListRef::new(self.shared().clone(), format!("{}.columns", self.base))
    }
}
#[derive(Serialize, Default)]
struct DatastreamStreamSourceConfigElSqlServerSourceConfigElIncludeObjectsElSchemasElDynamic {
    tables: Option<
        DynamicBlock<
            DatastreamStreamSourceConfigElSqlServerSourceConfigElIncludeObjectsElSchemasElTablesEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElSqlServerSourceConfigElIncludeObjectsElSchemasEl {
    schema: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tables: Option<
        Vec<DatastreamStreamSourceConfigElSqlServerSourceConfigElIncludeObjectsElSchemasElTablesEl>,
    >,
    dynamic: DatastreamStreamSourceConfigElSqlServerSourceConfigElIncludeObjectsElSchemasElDynamic,
}
impl DatastreamStreamSourceConfigElSqlServerSourceConfigElIncludeObjectsElSchemasEl {
    #[doc = "Set the field `tables`.\n"]
    pub fn set_tables(
        mut self,
        v : impl Into < BlockAssignable < DatastreamStreamSourceConfigElSqlServerSourceConfigElIncludeObjectsElSchemasElTablesEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.tables = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.tables = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for DatastreamStreamSourceConfigElSqlServerSourceConfigElIncludeObjectsElSchemasEl
{
    type O = BlockAssignable<
        DatastreamStreamSourceConfigElSqlServerSourceConfigElIncludeObjectsElSchemasEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamSourceConfigElSqlServerSourceConfigElIncludeObjectsElSchemasEl {
    #[doc = "Schema name."]
    pub schema: PrimField<String>,
}
impl BuildDatastreamStreamSourceConfigElSqlServerSourceConfigElIncludeObjectsElSchemasEl {
    pub fn build(
        self,
    ) -> DatastreamStreamSourceConfigElSqlServerSourceConfigElIncludeObjectsElSchemasEl {
        DatastreamStreamSourceConfigElSqlServerSourceConfigElIncludeObjectsElSchemasEl {
            schema: self.schema,
            tables: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DatastreamStreamSourceConfigElSqlServerSourceConfigElIncludeObjectsElSchemasElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamSourceConfigElSqlServerSourceConfigElIncludeObjectsElSchemasElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamStreamSourceConfigElSqlServerSourceConfigElIncludeObjectsElSchemasElRef {
        DatastreamStreamSourceConfigElSqlServerSourceConfigElIncludeObjectsElSchemasElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamSourceConfigElSqlServerSourceConfigElIncludeObjectsElSchemasElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `schema` after provisioning.\nSchema name."]
    pub fn schema(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.schema", self.base))
    }
    #[doc = "Get a reference to the value of field `tables` after provisioning.\n"]
    pub fn tables(
        &self,
    ) -> ListRef<
        DatastreamStreamSourceConfigElSqlServerSourceConfigElIncludeObjectsElSchemasElTablesElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.tables", self.base))
    }
}
#[derive(Serialize, Default)]
struct DatastreamStreamSourceConfigElSqlServerSourceConfigElIncludeObjectsElDynamic {
    schemas: Option<
        DynamicBlock<
            DatastreamStreamSourceConfigElSqlServerSourceConfigElIncludeObjectsElSchemasEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElSqlServerSourceConfigElIncludeObjectsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    schemas:
        Option<Vec<DatastreamStreamSourceConfigElSqlServerSourceConfigElIncludeObjectsElSchemasEl>>,
    dynamic: DatastreamStreamSourceConfigElSqlServerSourceConfigElIncludeObjectsElDynamic,
}
impl DatastreamStreamSourceConfigElSqlServerSourceConfigElIncludeObjectsEl {
    #[doc = "Set the field `schemas`.\n"]
    pub fn set_schemas(
        mut self,
        v: impl Into<
            BlockAssignable<
                DatastreamStreamSourceConfigElSqlServerSourceConfigElIncludeObjectsElSchemasEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.schemas = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.schemas = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DatastreamStreamSourceConfigElSqlServerSourceConfigElIncludeObjectsEl {
    type O = BlockAssignable<DatastreamStreamSourceConfigElSqlServerSourceConfigElIncludeObjectsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamSourceConfigElSqlServerSourceConfigElIncludeObjectsEl {}
impl BuildDatastreamStreamSourceConfigElSqlServerSourceConfigElIncludeObjectsEl {
    pub fn build(self) -> DatastreamStreamSourceConfigElSqlServerSourceConfigElIncludeObjectsEl {
        DatastreamStreamSourceConfigElSqlServerSourceConfigElIncludeObjectsEl {
            schemas: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DatastreamStreamSourceConfigElSqlServerSourceConfigElIncludeObjectsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamSourceConfigElSqlServerSourceConfigElIncludeObjectsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamStreamSourceConfigElSqlServerSourceConfigElIncludeObjectsElRef {
        DatastreamStreamSourceConfigElSqlServerSourceConfigElIncludeObjectsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamSourceConfigElSqlServerSourceConfigElIncludeObjectsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `schemas` after provisioning.\n"]
    pub fn schemas(
        &self,
    ) -> ListRef<DatastreamStreamSourceConfigElSqlServerSourceConfigElIncludeObjectsElSchemasElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.schemas", self.base))
    }
}
#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElSqlServerSourceConfigElTransactionLogsEl {}
impl DatastreamStreamSourceConfigElSqlServerSourceConfigElTransactionLogsEl {}
impl ToListMappable for DatastreamStreamSourceConfigElSqlServerSourceConfigElTransactionLogsEl {
    type O =
        BlockAssignable<DatastreamStreamSourceConfigElSqlServerSourceConfigElTransactionLogsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamSourceConfigElSqlServerSourceConfigElTransactionLogsEl {}
impl BuildDatastreamStreamSourceConfigElSqlServerSourceConfigElTransactionLogsEl {
    pub fn build(self) -> DatastreamStreamSourceConfigElSqlServerSourceConfigElTransactionLogsEl {
        DatastreamStreamSourceConfigElSqlServerSourceConfigElTransactionLogsEl {}
    }
}
pub struct DatastreamStreamSourceConfigElSqlServerSourceConfigElTransactionLogsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamSourceConfigElSqlServerSourceConfigElTransactionLogsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamStreamSourceConfigElSqlServerSourceConfigElTransactionLogsElRef {
        DatastreamStreamSourceConfigElSqlServerSourceConfigElTransactionLogsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamSourceConfigElSqlServerSourceConfigElTransactionLogsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize, Default)]
struct DatastreamStreamSourceConfigElSqlServerSourceConfigElDynamic {
    change_tables:
        Option<DynamicBlock<DatastreamStreamSourceConfigElSqlServerSourceConfigElChangeTablesEl>>,
    exclude_objects:
        Option<DynamicBlock<DatastreamStreamSourceConfigElSqlServerSourceConfigElExcludeObjectsEl>>,
    include_objects:
        Option<DynamicBlock<DatastreamStreamSourceConfigElSqlServerSourceConfigElIncludeObjectsEl>>,
    transaction_logs: Option<
        DynamicBlock<DatastreamStreamSourceConfigElSqlServerSourceConfigElTransactionLogsEl>,
    >,
}
#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElSqlServerSourceConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_concurrent_backfill_tasks: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_concurrent_cdc_tasks: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    change_tables: Option<Vec<DatastreamStreamSourceConfigElSqlServerSourceConfigElChangeTablesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_objects:
        Option<Vec<DatastreamStreamSourceConfigElSqlServerSourceConfigElExcludeObjectsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_objects:
        Option<Vec<DatastreamStreamSourceConfigElSqlServerSourceConfigElIncludeObjectsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transaction_logs:
        Option<Vec<DatastreamStreamSourceConfigElSqlServerSourceConfigElTransactionLogsEl>>,
    dynamic: DatastreamStreamSourceConfigElSqlServerSourceConfigElDynamic,
}
impl DatastreamStreamSourceConfigElSqlServerSourceConfigEl {
    #[doc = "Set the field `max_concurrent_backfill_tasks`.\nMax concurrent backfill tasks."]
    pub fn set_max_concurrent_backfill_tasks(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_concurrent_backfill_tasks = Some(v.into());
        self
    }
    #[doc = "Set the field `max_concurrent_cdc_tasks`.\nMax concurrent CDC tasks."]
    pub fn set_max_concurrent_cdc_tasks(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_concurrent_cdc_tasks = Some(v.into());
        self
    }
    #[doc = "Set the field `change_tables`.\n"]
    pub fn set_change_tables(
        mut self,
        v: impl Into<
            BlockAssignable<DatastreamStreamSourceConfigElSqlServerSourceConfigElChangeTablesEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.change_tables = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.change_tables = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `exclude_objects`.\n"]
    pub fn set_exclude_objects(
        mut self,
        v: impl Into<
            BlockAssignable<DatastreamStreamSourceConfigElSqlServerSourceConfigElExcludeObjectsEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.exclude_objects = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.exclude_objects = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `include_objects`.\n"]
    pub fn set_include_objects(
        mut self,
        v: impl Into<
            BlockAssignable<DatastreamStreamSourceConfigElSqlServerSourceConfigElIncludeObjectsEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.include_objects = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.include_objects = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `transaction_logs`.\n"]
    pub fn set_transaction_logs(
        mut self,
        v: impl Into<
            BlockAssignable<DatastreamStreamSourceConfigElSqlServerSourceConfigElTransactionLogsEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.transaction_logs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.transaction_logs = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DatastreamStreamSourceConfigElSqlServerSourceConfigEl {
    type O = BlockAssignable<DatastreamStreamSourceConfigElSqlServerSourceConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamSourceConfigElSqlServerSourceConfigEl {}
impl BuildDatastreamStreamSourceConfigElSqlServerSourceConfigEl {
    pub fn build(self) -> DatastreamStreamSourceConfigElSqlServerSourceConfigEl {
        DatastreamStreamSourceConfigElSqlServerSourceConfigEl {
            max_concurrent_backfill_tasks: core::default::Default::default(),
            max_concurrent_cdc_tasks: core::default::Default::default(),
            change_tables: core::default::Default::default(),
            exclude_objects: core::default::Default::default(),
            include_objects: core::default::Default::default(),
            transaction_logs: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DatastreamStreamSourceConfigElSqlServerSourceConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamSourceConfigElSqlServerSourceConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamStreamSourceConfigElSqlServerSourceConfigElRef {
        DatastreamStreamSourceConfigElSqlServerSourceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamSourceConfigElSqlServerSourceConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `max_concurrent_backfill_tasks` after provisioning.\nMax concurrent backfill tasks."]
    pub fn max_concurrent_backfill_tasks(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_concurrent_backfill_tasks", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_concurrent_cdc_tasks` after provisioning.\nMax concurrent CDC tasks."]
    pub fn max_concurrent_cdc_tasks(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_concurrent_cdc_tasks", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `change_tables` after provisioning.\n"]
    pub fn change_tables(
        &self,
    ) -> ListRef<DatastreamStreamSourceConfigElSqlServerSourceConfigElChangeTablesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.change_tables", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `exclude_objects` after provisioning.\n"]
    pub fn exclude_objects(
        &self,
    ) -> ListRef<DatastreamStreamSourceConfigElSqlServerSourceConfigElExcludeObjectsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.exclude_objects", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `include_objects` after provisioning.\n"]
    pub fn include_objects(
        &self,
    ) -> ListRef<DatastreamStreamSourceConfigElSqlServerSourceConfigElIncludeObjectsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.include_objects", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `transaction_logs` after provisioning.\n"]
    pub fn transaction_logs(
        &self,
    ) -> ListRef<DatastreamStreamSourceConfigElSqlServerSourceConfigElTransactionLogsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.transaction_logs", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DatastreamStreamSourceConfigElDynamic {
    mongodb_source_config:
        Option<DynamicBlock<DatastreamStreamSourceConfigElMongodbSourceConfigEl>>,
    mysql_source_config: Option<DynamicBlock<DatastreamStreamSourceConfigElMysqlSourceConfigEl>>,
    oracle_source_config: Option<DynamicBlock<DatastreamStreamSourceConfigElOracleSourceConfigEl>>,
    postgresql_source_config:
        Option<DynamicBlock<DatastreamStreamSourceConfigElPostgresqlSourceConfigEl>>,
    salesforce_source_config:
        Option<DynamicBlock<DatastreamStreamSourceConfigElSalesforceSourceConfigEl>>,
    spanner_source_config:
        Option<DynamicBlock<DatastreamStreamSourceConfigElSpannerSourceConfigEl>>,
    sql_server_source_config:
        Option<DynamicBlock<DatastreamStreamSourceConfigElSqlServerSourceConfigEl>>,
}
#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigEl {
    source_connection_profile: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mongodb_source_config: Option<Vec<DatastreamStreamSourceConfigElMongodbSourceConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mysql_source_config: Option<Vec<DatastreamStreamSourceConfigElMysqlSourceConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oracle_source_config: Option<Vec<DatastreamStreamSourceConfigElOracleSourceConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    postgresql_source_config: Option<Vec<DatastreamStreamSourceConfigElPostgresqlSourceConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    salesforce_source_config: Option<Vec<DatastreamStreamSourceConfigElSalesforceSourceConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spanner_source_config: Option<Vec<DatastreamStreamSourceConfigElSpannerSourceConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sql_server_source_config: Option<Vec<DatastreamStreamSourceConfigElSqlServerSourceConfigEl>>,
    dynamic: DatastreamStreamSourceConfigElDynamic,
}
impl DatastreamStreamSourceConfigEl {
    #[doc = "Set the field `mongodb_source_config`.\n"]
    pub fn set_mongodb_source_config(
        mut self,
        v: impl Into<BlockAssignable<DatastreamStreamSourceConfigElMongodbSourceConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.mongodb_source_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.mongodb_source_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `mysql_source_config`.\n"]
    pub fn set_mysql_source_config(
        mut self,
        v: impl Into<BlockAssignable<DatastreamStreamSourceConfigElMysqlSourceConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.mysql_source_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.mysql_source_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `oracle_source_config`.\n"]
    pub fn set_oracle_source_config(
        mut self,
        v: impl Into<BlockAssignable<DatastreamStreamSourceConfigElOracleSourceConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.oracle_source_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.oracle_source_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `postgresql_source_config`.\n"]
    pub fn set_postgresql_source_config(
        mut self,
        v: impl Into<BlockAssignable<DatastreamStreamSourceConfigElPostgresqlSourceConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.postgresql_source_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.postgresql_source_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `salesforce_source_config`.\n"]
    pub fn set_salesforce_source_config(
        mut self,
        v: impl Into<BlockAssignable<DatastreamStreamSourceConfigElSalesforceSourceConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.salesforce_source_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.salesforce_source_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `spanner_source_config`.\n"]
    pub fn set_spanner_source_config(
        mut self,
        v: impl Into<BlockAssignable<DatastreamStreamSourceConfigElSpannerSourceConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.spanner_source_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.spanner_source_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `sql_server_source_config`.\n"]
    pub fn set_sql_server_source_config(
        mut self,
        v: impl Into<BlockAssignable<DatastreamStreamSourceConfigElSqlServerSourceConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.sql_server_source_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.sql_server_source_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DatastreamStreamSourceConfigEl {
    type O = BlockAssignable<DatastreamStreamSourceConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamSourceConfigEl {
    #[doc = "Source connection profile resource. Format: projects/{project}/locations/{location}/connectionProfiles/{name}"]
    pub source_connection_profile: PrimField<String>,
}
impl BuildDatastreamStreamSourceConfigEl {
    pub fn build(self) -> DatastreamStreamSourceConfigEl {
        DatastreamStreamSourceConfigEl {
            source_connection_profile: self.source_connection_profile,
            mongodb_source_config: core::default::Default::default(),
            mysql_source_config: core::default::Default::default(),
            oracle_source_config: core::default::Default::default(),
            postgresql_source_config: core::default::Default::default(),
            salesforce_source_config: core::default::Default::default(),
            spanner_source_config: core::default::Default::default(),
            sql_server_source_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DatastreamStreamSourceConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamSourceConfigElRef {
    fn new(shared: StackShared, base: String) -> DatastreamStreamSourceConfigElRef {
        DatastreamStreamSourceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamSourceConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `source_connection_profile` after provisioning.\nSource connection profile resource. Format: projects/{project}/locations/{location}/connectionProfiles/{name}"]
    pub fn source_connection_profile(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_connection_profile", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `mongodb_source_config` after provisioning.\n"]
    pub fn mongodb_source_config(
        &self,
    ) -> ListRef<DatastreamStreamSourceConfigElMongodbSourceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.mongodb_source_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `mysql_source_config` after provisioning.\n"]
    pub fn mysql_source_config(
        &self,
    ) -> ListRef<DatastreamStreamSourceConfigElMysqlSourceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.mysql_source_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `oracle_source_config` after provisioning.\n"]
    pub fn oracle_source_config(
        &self,
    ) -> ListRef<DatastreamStreamSourceConfigElOracleSourceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.oracle_source_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `postgresql_source_config` after provisioning.\n"]
    pub fn postgresql_source_config(
        &self,
    ) -> ListRef<DatastreamStreamSourceConfigElPostgresqlSourceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.postgresql_source_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `salesforce_source_config` after provisioning.\n"]
    pub fn salesforce_source_config(
        &self,
    ) -> ListRef<DatastreamStreamSourceConfigElSalesforceSourceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.salesforce_source_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `spanner_source_config` after provisioning.\n"]
    pub fn spanner_source_config(
        &self,
    ) -> ListRef<DatastreamStreamSourceConfigElSpannerSourceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.spanner_source_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `sql_server_source_config` after provisioning.\n"]
    pub fn sql_server_source_config(
        &self,
    ) -> ListRef<DatastreamStreamSourceConfigElSqlServerSourceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.sql_server_source_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DatastreamStreamTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl DatastreamStreamTimeoutsEl {
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
impl ToListMappable for DatastreamStreamTimeoutsEl {
    type O = BlockAssignable<DatastreamStreamTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamStreamTimeoutsEl {}
impl BuildDatastreamStreamTimeoutsEl {
    pub fn build(self) -> DatastreamStreamTimeoutsEl {
        DatastreamStreamTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct DatastreamStreamTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamStreamTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DatastreamStreamTimeoutsElRef {
        DatastreamStreamTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamStreamTimeoutsElRef {
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
struct DatastreamStreamDynamic {
    backfill_all: Option<DynamicBlock<DatastreamStreamBackfillAllEl>>,
    backfill_none: Option<DynamicBlock<DatastreamStreamBackfillNoneEl>>,
    destination_config: Option<DynamicBlock<DatastreamStreamDestinationConfigEl>>,
    rule_sets: Option<DynamicBlock<DatastreamStreamRuleSetsEl>>,
    source_config: Option<DynamicBlock<DatastreamStreamSourceConfigEl>>,
}
