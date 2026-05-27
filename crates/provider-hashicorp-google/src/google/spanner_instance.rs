use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct SpannerInstanceData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    config: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_backup_schedule_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    display_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    edition: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    force_destroy: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    num_nodes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    processing_units: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    autoscaling_config: Option<Vec<SpannerInstanceAutoscalingConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<SpannerInstanceTimeoutsEl>,
    dynamic: SpannerInstanceDynamic,
}
struct SpannerInstance_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SpannerInstanceData>,
}
#[derive(Clone)]
pub struct SpannerInstance(Rc<SpannerInstance_>);
impl SpannerInstance {
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
    #[doc = "Set the field `default_backup_schedule_type`.\nControls the default backup behavior for new databases within the instance.\nNote that 'AUTOMATIC' is not permitted for free instances, as backups and backup schedules are not allowed for free instances.\nif unset or NONE, no default backup schedule will be created for new databases within the instance. Possible values: [\"NONE\", \"AUTOMATIC\"]"]
    pub fn set_default_backup_schedule_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().default_backup_schedule_type = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `edition`.\nThe edition selected for this instance. Different editions provide different capabilities at different price points. Possible values: [\"EDITION_UNSPECIFIED\", \"STANDARD\", \"ENTERPRISE\", \"ENTERPRISE_PLUS\"]"]
    pub fn set_edition(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().edition = Some(v.into());
        self
    }
    #[doc = "Set the field `force_destroy`.\nWhen deleting a spanner instance, this boolean option will delete all backups of this instance.\nThis must be set to true if you created a backup manually in the console."]
    pub fn set_force_destroy(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().force_destroy = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `instance_type`.\nThe type of this instance. The type can be used to distinguish product variants, that can affect aspects like:\nusage restrictions, quotas and billing. Currently this is used to distinguish FREE_INSTANCE vs PROVISIONED instances.\nWhen configured as FREE_INSTANCE, the field 'edition' should not be configured. Possible values: [\"PROVISIONED\", \"FREE_INSTANCE\"]"]
    pub fn set_instance_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().instance_type = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nAn object containing a list of \"key\": value pairs.\nExample: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\nA unique identifier for the instance, which cannot be changed after\nthe instance is created. The name must be between 6 and 30 characters\nin length.\nIf not provided, a random string starting with 'tf-' will be selected."]
    pub fn set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name = Some(v.into());
        self
    }
    #[doc = "Set the field `num_nodes`.\nThe number of nodes allocated to this instance. Exactly one of either num_nodes, processing_units or\nautoscaling_config must be present in terraform except when instance_type = FREE_INSTANCE."]
    pub fn set_num_nodes(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().num_nodes = Some(v.into());
        self
    }
    #[doc = "Set the field `processing_units`.\nThe number of processing units allocated to this instance. Exactly one of either num_nodes,\nprocessing_units or autoscaling_config must be present in terraform except when instance_type = FREE_INSTANCE."]
    pub fn set_processing_units(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().processing_units = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `autoscaling_config`.\n"]
    pub fn set_autoscaling_config(
        self,
        v: impl Into<BlockAssignable<SpannerInstanceAutoscalingConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().autoscaling_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.autoscaling_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<SpannerInstanceTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `config` after provisioning.\nThe name of the instance's configuration (similar but not\nquite the same as a region) which defines the geographic placement and\nreplication of your databases in this instance. It determines where your data\nis stored. Values are typically of the form 'regional-europe-west1' , 'us-central' etc.\nIn order to obtain a valid list please consult the\n[Configuration section of the docs](https://cloud.google.com/spanner/docs/instances)."]
    pub fn config(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `default_backup_schedule_type` after provisioning.\nControls the default backup behavior for new databases within the instance.\nNote that 'AUTOMATIC' is not permitted for free instances, as backups and backup schedules are not allowed for free instances.\nif unset or NONE, no default backup schedule will be created for new databases within the instance. Possible values: [\"NONE\", \"AUTOMATIC\"]"]
    pub fn default_backup_schedule_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_backup_schedule_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe descriptive name for this instance as it appears in UIs. Must be\nunique per project and between 4 and 30 characters in length."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `edition` after provisioning.\nThe edition selected for this instance. Different editions provide different capabilities at different price points. Possible values: [\"EDITION_UNSPECIFIED\", \"STANDARD\", \"ENTERPRISE\", \"ENTERPRISE_PLUS\"]"]
    pub fn edition(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.edition", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `force_destroy` after provisioning.\nWhen deleting a spanner instance, this boolean option will delete all backups of this instance.\nThis must be set to true if you created a backup manually in the console."]
    pub fn force_destroy(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.force_destroy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\nThe type of this instance. The type can be used to distinguish product variants, that can affect aspects like:\nusage restrictions, quotas and billing. Currently this is used to distinguish FREE_INSTANCE vs PROVISIONED instances.\nWhen configured as FREE_INSTANCE, the field 'edition' should not be configured. Possible values: [\"PROVISIONED\", \"FREE_INSTANCE\"]"]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nAn object containing a list of \"key\": value pairs.\nExample: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nA unique identifier for the instance, which cannot be changed after\nthe instance is created. The name must be between 6 and 30 characters\nin length.\nIf not provided, a random string starting with 'tf-' will be selected."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `num_nodes` after provisioning.\nThe number of nodes allocated to this instance. Exactly one of either num_nodes, processing_units or\nautoscaling_config must be present in terraform except when instance_type = FREE_INSTANCE."]
    pub fn num_nodes(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.num_nodes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `processing_units` after provisioning.\nThe number of processing units allocated to this instance. Exactly one of either num_nodes,\nprocessing_units or autoscaling_config must be present in terraform except when instance_type = FREE_INSTANCE."]
    pub fn processing_units(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.processing_units", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nInstance status: 'CREATING' or 'READY'."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `autoscaling_config` after provisioning.\n"]
    pub fn autoscaling_config(&self) -> ListRef<SpannerInstanceAutoscalingConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.autoscaling_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SpannerInstanceTimeoutsElRef {
        SpannerInstanceTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for SpannerInstance {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for SpannerInstance {}
impl ToListMappable for SpannerInstance {
    type O = ListRef<SpannerInstanceRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for SpannerInstance_ {
    fn extract_resource_type(&self) -> String {
        "google_spanner_instance".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildSpannerInstance {
    pub tf_id: String,
    #[doc = "The name of the instance's configuration (similar but not\nquite the same as a region) which defines the geographic placement and\nreplication of your databases in this instance. It determines where your data\nis stored. Values are typically of the form 'regional-europe-west1' , 'us-central' etc.\nIn order to obtain a valid list please consult the\n[Configuration section of the docs](https://cloud.google.com/spanner/docs/instances)."]
    pub config: PrimField<String>,
    #[doc = "The descriptive name for this instance as it appears in UIs. Must be\nunique per project and between 4 and 30 characters in length."]
    pub display_name: PrimField<String>,
}
impl BuildSpannerInstance {
    pub fn build(self, stack: &mut Stack) -> SpannerInstance {
        let out = SpannerInstance(Rc::new(SpannerInstance_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(SpannerInstanceData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                config: self.config,
                default_backup_schedule_type: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                display_name: self.display_name,
                edition: core::default::Default::default(),
                force_destroy: core::default::Default::default(),
                id: core::default::Default::default(),
                instance_type: core::default::Default::default(),
                labels: core::default::Default::default(),
                name: core::default::Default::default(),
                num_nodes: core::default::Default::default(),
                processing_units: core::default::Default::default(),
                project: core::default::Default::default(),
                autoscaling_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct SpannerInstanceRef {
    shared: StackShared,
    base: String,
}
impl Ref for SpannerInstanceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl SpannerInstanceRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `config` after provisioning.\nThe name of the instance's configuration (similar but not\nquite the same as a region) which defines the geographic placement and\nreplication of your databases in this instance. It determines where your data\nis stored. Values are typically of the form 'regional-europe-west1' , 'us-central' etc.\nIn order to obtain a valid list please consult the\n[Configuration section of the docs](https://cloud.google.com/spanner/docs/instances)."]
    pub fn config(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `default_backup_schedule_type` after provisioning.\nControls the default backup behavior for new databases within the instance.\nNote that 'AUTOMATIC' is not permitted for free instances, as backups and backup schedules are not allowed for free instances.\nif unset or NONE, no default backup schedule will be created for new databases within the instance. Possible values: [\"NONE\", \"AUTOMATIC\"]"]
    pub fn default_backup_schedule_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_backup_schedule_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe descriptive name for this instance as it appears in UIs. Must be\nunique per project and between 4 and 30 characters in length."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `edition` after provisioning.\nThe edition selected for this instance. Different editions provide different capabilities at different price points. Possible values: [\"EDITION_UNSPECIFIED\", \"STANDARD\", \"ENTERPRISE\", \"ENTERPRISE_PLUS\"]"]
    pub fn edition(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.edition", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `force_destroy` after provisioning.\nWhen deleting a spanner instance, this boolean option will delete all backups of this instance.\nThis must be set to true if you created a backup manually in the console."]
    pub fn force_destroy(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.force_destroy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\nThe type of this instance. The type can be used to distinguish product variants, that can affect aspects like:\nusage restrictions, quotas and billing. Currently this is used to distinguish FREE_INSTANCE vs PROVISIONED instances.\nWhen configured as FREE_INSTANCE, the field 'edition' should not be configured. Possible values: [\"PROVISIONED\", \"FREE_INSTANCE\"]"]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nAn object containing a list of \"key\": value pairs.\nExample: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nA unique identifier for the instance, which cannot be changed after\nthe instance is created. The name must be between 6 and 30 characters\nin length.\nIf not provided, a random string starting with 'tf-' will be selected."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `num_nodes` after provisioning.\nThe number of nodes allocated to this instance. Exactly one of either num_nodes, processing_units or\nautoscaling_config must be present in terraform except when instance_type = FREE_INSTANCE."]
    pub fn num_nodes(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.num_nodes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `processing_units` after provisioning.\nThe number of processing units allocated to this instance. Exactly one of either num_nodes,\nprocessing_units or autoscaling_config must be present in terraform except when instance_type = FREE_INSTANCE."]
    pub fn processing_units(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.processing_units", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nInstance status: 'CREATING' or 'READY'."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `autoscaling_config` after provisioning.\n"]
    pub fn autoscaling_config(&self) -> ListRef<SpannerInstanceAutoscalingConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.autoscaling_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SpannerInstanceTimeoutsElRef {
        SpannerInstanceTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct SpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElOverridesElAutoscalingLimitsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    max_nodes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_processing_units: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_nodes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_processing_units: Option<PrimField<f64>>,
}
impl
    SpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElOverridesElAutoscalingLimitsEl
{
    #[doc = "Set the field `max_nodes`.\nThe maximum number of nodes for this specific replica."]
    pub fn set_max_nodes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_nodes = Some(v.into());
        self
    }
    #[doc = "Set the field `max_processing_units`.\nThe maximum number of processing units for this specific replica.\nIf set, this number should be multiples of 1000 and be greater than or equal to\nmin_processing_units."]
    pub fn set_max_processing_units(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_processing_units = Some(v.into());
        self
    }
    #[doc = "Set the field `min_nodes`.\nThe minimum number of nodes for this specific replica."]
    pub fn set_min_nodes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min_nodes = Some(v.into());
        self
    }
    #[doc = "Set the field `min_processing_units`.\nThe minimum number of processing units for this specific replica.\nIf set, this number should be multiples of 1000."]
    pub fn set_min_processing_units(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min_processing_units = Some(v.into());
        self
    }
}
impl ToListMappable for SpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElOverridesElAutoscalingLimitsEl { type O = BlockAssignable < SpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElOverridesElAutoscalingLimitsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildSpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElOverridesElAutoscalingLimitsEl
{}
impl BuildSpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElOverridesElAutoscalingLimitsEl { pub fn build (self) -> SpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElOverridesElAutoscalingLimitsEl { SpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElOverridesElAutoscalingLimitsEl { max_nodes : core :: default :: Default :: default () , max_processing_units : core :: default :: Default :: default () , min_nodes : core :: default :: Default :: default () , min_processing_units : core :: default :: Default :: default () , } } }
pub struct SpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElOverridesElAutoscalingLimitsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for SpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElOverridesElAutoscalingLimitsElRef { fn new (shared : StackShared , base : String) -> SpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElOverridesElAutoscalingLimitsElRef { SpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElOverridesElAutoscalingLimitsElRef { shared : shared , base : base . to_string () , } } }
impl SpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElOverridesElAutoscalingLimitsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `max_nodes` after provisioning.\nThe maximum number of nodes for this specific replica."] pub fn max_nodes (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.max_nodes" , self . base)) } # [doc = "Get a reference to the value of field `max_processing_units` after provisioning.\nThe maximum number of processing units for this specific replica.\nIf set, this number should be multiples of 1000 and be greater than or equal to\nmin_processing_units."] pub fn max_processing_units (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.max_processing_units" , self . base)) } # [doc = "Get a reference to the value of field `min_nodes` after provisioning.\nThe minimum number of nodes for this specific replica."] pub fn min_nodes (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.min_nodes" , self . base)) } # [doc = "Get a reference to the value of field `min_processing_units` after provisioning.\nThe minimum number of processing units for this specific replica.\nIf set, this number should be multiples of 1000."] pub fn min_processing_units (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.min_processing_units" , self . base)) } }
#[derive(Serialize, Default)]
struct SpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElOverridesElDynamic { autoscaling_limits : Option < DynamicBlock < SpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElOverridesElAutoscalingLimitsEl >> , }
#[derive(Serialize)]
pub struct SpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElOverridesEl { # [serde (skip_serializing_if = "Option::is_none")] autoscaling_target_high_priority_cpu_utilization_percent : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] autoscaling_target_total_cpu_utilization_percent : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] disable_high_priority_cpu_autoscaling : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] disable_total_cpu_autoscaling : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] autoscaling_limits : Option < Vec < SpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElOverridesElAutoscalingLimitsEl > > , dynamic : SpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElOverridesElDynamic , }
impl SpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElOverridesEl {
    #[doc = "Set the field `autoscaling_target_high_priority_cpu_utilization_percent`.\nThe target high priority cpu utilization percentage that the autoscaler\nshould be trying to achieve for this replica.\nThis number is on a scale from 0 (no utilization) to 100 (full utilization)."]
    pub fn set_autoscaling_target_high_priority_cpu_utilization_percent(
        mut self,
        v: impl Into<PrimField<f64>>,
    ) -> Self {
        self.autoscaling_target_high_priority_cpu_utilization_percent = Some(v.into());
        self
    }
    #[doc = "Set the field `autoscaling_target_total_cpu_utilization_percent`.\nThe target total cpu utilization percentage that the autoscaler\nshould be trying to achieve for this replica.\nThis number is on a scale from 0 (no utilization) to 100 (full utilization)."]
    pub fn set_autoscaling_target_total_cpu_utilization_percent(
        mut self,
        v: impl Into<PrimField<f64>>,
    ) -> Self {
        self.autoscaling_target_total_cpu_utilization_percent = Some(v.into());
        self
    }
    #[doc = "Set the field `disable_high_priority_cpu_autoscaling`.\nIf true, disables high priority CPU autoscaling for this replica and ignores\nhigh_priority_cpu_utilization_percent in the top-level autoscaling configuration."]
    pub fn set_disable_high_priority_cpu_autoscaling(
        mut self,
        v: impl Into<PrimField<bool>>,
    ) -> Self {
        self.disable_high_priority_cpu_autoscaling = Some(v.into());
        self
    }
    #[doc = "Set the field `disable_total_cpu_autoscaling`.\nIf true, disables total CPU autoscaling for this replica and ignores\ntotal_cpu_utilization_percent in the top-level autoscaling configuration."]
    pub fn set_disable_total_cpu_autoscaling(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disable_total_cpu_autoscaling = Some(v.into());
        self
    }
    #[doc = "Set the field `autoscaling_limits`.\n"]
    pub fn set_autoscaling_limits(
        mut self,
        v : impl Into < BlockAssignable < SpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElOverridesElAutoscalingLimitsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.autoscaling_limits = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.autoscaling_limits = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for SpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElOverridesEl
{
    type O = BlockAssignable<
        SpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElOverridesEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElOverridesEl {}
impl BuildSpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElOverridesEl {
    pub fn build(
        self,
    ) -> SpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElOverridesEl {
        SpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElOverridesEl {
            autoscaling_target_high_priority_cpu_utilization_percent:
                core::default::Default::default(),
            autoscaling_target_total_cpu_utilization_percent: core::default::Default::default(),
            disable_high_priority_cpu_autoscaling: core::default::Default::default(),
            disable_total_cpu_autoscaling: core::default::Default::default(),
            autoscaling_limits: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct SpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElOverridesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElOverridesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElOverridesElRef {
        SpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElOverridesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElOverridesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `autoscaling_target_high_priority_cpu_utilization_percent` after provisioning.\nThe target high priority cpu utilization percentage that the autoscaler\nshould be trying to achieve for this replica.\nThis number is on a scale from 0 (no utilization) to 100 (full utilization)."]
    pub fn autoscaling_target_high_priority_cpu_utilization_percent(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.autoscaling_target_high_priority_cpu_utilization_percent",
                self.base
            ),
        )
    }
    #[doc = "Get a reference to the value of field `autoscaling_target_total_cpu_utilization_percent` after provisioning.\nThe target total cpu utilization percentage that the autoscaler\nshould be trying to achieve for this replica.\nThis number is on a scale from 0 (no utilization) to 100 (full utilization)."]
    pub fn autoscaling_target_total_cpu_utilization_percent(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.autoscaling_target_total_cpu_utilization_percent",
                self.base
            ),
        )
    }
    #[doc = "Get a reference to the value of field `disable_high_priority_cpu_autoscaling` after provisioning.\nIf true, disables high priority CPU autoscaling for this replica and ignores\nhigh_priority_cpu_utilization_percent in the top-level autoscaling configuration."]
    pub fn disable_high_priority_cpu_autoscaling(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_high_priority_cpu_autoscaling", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `disable_total_cpu_autoscaling` after provisioning.\nIf true, disables total CPU autoscaling for this replica and ignores\ntotal_cpu_utilization_percent in the top-level autoscaling configuration."]
    pub fn disable_total_cpu_autoscaling(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_total_cpu_autoscaling", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `autoscaling_limits` after provisioning.\n"]    pub fn autoscaling_limits (& self) -> ListRef < SpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElOverridesElAutoscalingLimitsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.autoscaling_limits", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct SpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElReplicaSelectionEl {
    location: PrimField<String>,
}
impl SpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElReplicaSelectionEl {}
impl ToListMappable
    for SpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElReplicaSelectionEl
{
    type O = BlockAssignable<
        SpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElReplicaSelectionEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElReplicaSelectionEl {
    #[doc = "The location of the replica to apply asymmetric autoscaling options."]
    pub location: PrimField<String>,
}
impl BuildSpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElReplicaSelectionEl {
    pub fn build(
        self,
    ) -> SpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElReplicaSelectionEl {
        SpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElReplicaSelectionEl {
            location: self.location,
        }
    }
}
pub struct SpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElReplicaSelectionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElReplicaSelectionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElReplicaSelectionElRef {
        SpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElReplicaSelectionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElReplicaSelectionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the replica to apply asymmetric autoscaling options."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }
}
#[derive(Serialize, Default)]
struct SpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElDynamic {
    overrides: Option<
        DynamicBlock<SpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElOverridesEl>,
    >,
    replica_selection: Option<
        DynamicBlock<
            SpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElReplicaSelectionEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct SpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    overrides:
        Option<Vec<SpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElOverridesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    replica_selection: Option<
        Vec<SpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElReplicaSelectionEl>,
    >,
    dynamic: SpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElDynamic,
}
impl SpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsEl {
    #[doc = "Set the field `overrides`.\n"]
    pub fn set_overrides(
        mut self,
        v: impl Into<
            BlockAssignable<
                SpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElOverridesEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.overrides = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.overrides = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `replica_selection`.\n"]
    pub fn set_replica_selection(
        mut self,
        v: impl Into<
            BlockAssignable<
                SpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElReplicaSelectionEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.replica_selection = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.replica_selection = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for SpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsEl {
    type O = BlockAssignable<SpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsEl {}
impl BuildSpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsEl {
    pub fn build(self) -> SpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsEl {
        SpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsEl {
            overrides: core::default::Default::default(),
            replica_selection: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct SpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElRef {
        SpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `overrides` after provisioning.\n"]
    pub fn overrides(
        &self,
    ) -> ListRef<SpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElOverridesElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.overrides", self.base))
    }
    #[doc = "Get a reference to the value of field `replica_selection` after provisioning.\n"]
    pub fn replica_selection(
        &self,
    ) -> ListRef<
        SpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElReplicaSelectionElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.replica_selection", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct SpannerInstanceAutoscalingConfigElAutoscalingLimitsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_nodes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_processing_units: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_nodes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_processing_units: Option<PrimField<f64>>,
}
impl SpannerInstanceAutoscalingConfigElAutoscalingLimitsEl {
    #[doc = "Set the field `max_nodes`.\nSpecifies maximum number of nodes allocated to the instance. If set, this number\nshould be greater than or equal to min_nodes."]
    pub fn set_max_nodes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_nodes = Some(v.into());
        self
    }
    #[doc = "Set the field `max_processing_units`.\nSpecifies maximum number of processing units allocated to the instance.\nIf set, this number should be multiples of 1000 and be greater than or equal to\nmin_processing_units."]
    pub fn set_max_processing_units(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_processing_units = Some(v.into());
        self
    }
    #[doc = "Set the field `min_nodes`.\nSpecifies number of nodes allocated to the instance. If set, this number\nshould be greater than or equal to 1."]
    pub fn set_min_nodes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min_nodes = Some(v.into());
        self
    }
    #[doc = "Set the field `min_processing_units`.\nSpecifies minimum number of processing units allocated to the instance.\nIf set, this number should be multiples of 1000."]
    pub fn set_min_processing_units(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min_processing_units = Some(v.into());
        self
    }
}
impl ToListMappable for SpannerInstanceAutoscalingConfigElAutoscalingLimitsEl {
    type O = BlockAssignable<SpannerInstanceAutoscalingConfigElAutoscalingLimitsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSpannerInstanceAutoscalingConfigElAutoscalingLimitsEl {}
impl BuildSpannerInstanceAutoscalingConfigElAutoscalingLimitsEl {
    pub fn build(self) -> SpannerInstanceAutoscalingConfigElAutoscalingLimitsEl {
        SpannerInstanceAutoscalingConfigElAutoscalingLimitsEl {
            max_nodes: core::default::Default::default(),
            max_processing_units: core::default::Default::default(),
            min_nodes: core::default::Default::default(),
            min_processing_units: core::default::Default::default(),
        }
    }
}
pub struct SpannerInstanceAutoscalingConfigElAutoscalingLimitsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SpannerInstanceAutoscalingConfigElAutoscalingLimitsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SpannerInstanceAutoscalingConfigElAutoscalingLimitsElRef {
        SpannerInstanceAutoscalingConfigElAutoscalingLimitsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SpannerInstanceAutoscalingConfigElAutoscalingLimitsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `max_nodes` after provisioning.\nSpecifies maximum number of nodes allocated to the instance. If set, this number\nshould be greater than or equal to min_nodes."]
    pub fn max_nodes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_nodes", self.base))
    }
    #[doc = "Get a reference to the value of field `max_processing_units` after provisioning.\nSpecifies maximum number of processing units allocated to the instance.\nIf set, this number should be multiples of 1000 and be greater than or equal to\nmin_processing_units."]
    pub fn max_processing_units(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_processing_units", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `min_nodes` after provisioning.\nSpecifies number of nodes allocated to the instance. If set, this number\nshould be greater than or equal to 1."]
    pub fn min_nodes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_nodes", self.base))
    }
    #[doc = "Get a reference to the value of field `min_processing_units` after provisioning.\nSpecifies minimum number of processing units allocated to the instance.\nIf set, this number should be multiples of 1000."]
    pub fn min_processing_units(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_processing_units", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct SpannerInstanceAutoscalingConfigElAutoscalingTargetsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    high_priority_cpu_utilization_percent: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_utilization_percent: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    total_cpu_utilization_percent: Option<PrimField<f64>>,
}
impl SpannerInstanceAutoscalingConfigElAutoscalingTargetsEl {
    #[doc = "Set the field `high_priority_cpu_utilization_percent`.\nSpecifies the target high priority cpu utilization percentage that the autoscaler\nshould be trying to achieve for the instance.\nThis number is on a scale from 0 (no utilization) to 100 (full utilization).."]
    pub fn set_high_priority_cpu_utilization_percent(
        mut self,
        v: impl Into<PrimField<f64>>,
    ) -> Self {
        self.high_priority_cpu_utilization_percent = Some(v.into());
        self
    }
    #[doc = "Set the field `storage_utilization_percent`.\nSpecifies the target storage utilization percentage that the autoscaler\nshould be trying to achieve for the instance.\nThis number is on a scale from 0 (no utilization) to 100 (full utilization)."]
    pub fn set_storage_utilization_percent(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.storage_utilization_percent = Some(v.into());
        self
    }
    #[doc = "Set the field `total_cpu_utilization_percent`.\nThe target total cpu utilization percentage that the autoscaler should be trying to achieve for the instance.\nThis number is on a scale from 0 (no utilization) to 100 (full utilization). The valid range is [10, 90] inclusive.\nIf not specified or set to 0, the autoscaler will skip scaling based on total cpu utilization.\nThe value should be higher than high_priority_cpu_utilization_percent if present."]
    pub fn set_total_cpu_utilization_percent(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.total_cpu_utilization_percent = Some(v.into());
        self
    }
}
impl ToListMappable for SpannerInstanceAutoscalingConfigElAutoscalingTargetsEl {
    type O = BlockAssignable<SpannerInstanceAutoscalingConfigElAutoscalingTargetsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSpannerInstanceAutoscalingConfigElAutoscalingTargetsEl {}
impl BuildSpannerInstanceAutoscalingConfigElAutoscalingTargetsEl {
    pub fn build(self) -> SpannerInstanceAutoscalingConfigElAutoscalingTargetsEl {
        SpannerInstanceAutoscalingConfigElAutoscalingTargetsEl {
            high_priority_cpu_utilization_percent: core::default::Default::default(),
            storage_utilization_percent: core::default::Default::default(),
            total_cpu_utilization_percent: core::default::Default::default(),
        }
    }
}
pub struct SpannerInstanceAutoscalingConfigElAutoscalingTargetsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SpannerInstanceAutoscalingConfigElAutoscalingTargetsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SpannerInstanceAutoscalingConfigElAutoscalingTargetsElRef {
        SpannerInstanceAutoscalingConfigElAutoscalingTargetsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SpannerInstanceAutoscalingConfigElAutoscalingTargetsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `high_priority_cpu_utilization_percent` after provisioning.\nSpecifies the target high priority cpu utilization percentage that the autoscaler\nshould be trying to achieve for the instance.\nThis number is on a scale from 0 (no utilization) to 100 (full utilization).."]
    pub fn high_priority_cpu_utilization_percent(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.high_priority_cpu_utilization_percent", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `storage_utilization_percent` after provisioning.\nSpecifies the target storage utilization percentage that the autoscaler\nshould be trying to achieve for the instance.\nThis number is on a scale from 0 (no utilization) to 100 (full utilization)."]
    pub fn storage_utilization_percent(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.storage_utilization_percent", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `total_cpu_utilization_percent` after provisioning.\nThe target total cpu utilization percentage that the autoscaler should be trying to achieve for the instance.\nThis number is on a scale from 0 (no utilization) to 100 (full utilization). The valid range is [10, 90] inclusive.\nIf not specified or set to 0, the autoscaler will skip scaling based on total cpu utilization.\nThe value should be higher than high_priority_cpu_utilization_percent if present."]
    pub fn total_cpu_utilization_percent(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.total_cpu_utilization_percent", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct SpannerInstanceAutoscalingConfigElDynamic {
    asymmetric_autoscaling_options:
        Option<DynamicBlock<SpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsEl>>,
    autoscaling_limits: Option<DynamicBlock<SpannerInstanceAutoscalingConfigElAutoscalingLimitsEl>>,
    autoscaling_targets:
        Option<DynamicBlock<SpannerInstanceAutoscalingConfigElAutoscalingTargetsEl>>,
}
#[derive(Serialize)]
pub struct SpannerInstanceAutoscalingConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    asymmetric_autoscaling_options:
        Option<Vec<SpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    autoscaling_limits: Option<Vec<SpannerInstanceAutoscalingConfigElAutoscalingLimitsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    autoscaling_targets: Option<Vec<SpannerInstanceAutoscalingConfigElAutoscalingTargetsEl>>,
    dynamic: SpannerInstanceAutoscalingConfigElDynamic,
}
impl SpannerInstanceAutoscalingConfigEl {
    #[doc = "Set the field `asymmetric_autoscaling_options`.\n"]
    pub fn set_asymmetric_autoscaling_options(
        mut self,
        v: impl Into<BlockAssignable<SpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.asymmetric_autoscaling_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.asymmetric_autoscaling_options = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `autoscaling_limits`.\n"]
    pub fn set_autoscaling_limits(
        mut self,
        v: impl Into<BlockAssignable<SpannerInstanceAutoscalingConfigElAutoscalingLimitsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.autoscaling_limits = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.autoscaling_limits = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `autoscaling_targets`.\n"]
    pub fn set_autoscaling_targets(
        mut self,
        v: impl Into<BlockAssignable<SpannerInstanceAutoscalingConfigElAutoscalingTargetsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.autoscaling_targets = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.autoscaling_targets = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for SpannerInstanceAutoscalingConfigEl {
    type O = BlockAssignable<SpannerInstanceAutoscalingConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSpannerInstanceAutoscalingConfigEl {}
impl BuildSpannerInstanceAutoscalingConfigEl {
    pub fn build(self) -> SpannerInstanceAutoscalingConfigEl {
        SpannerInstanceAutoscalingConfigEl {
            asymmetric_autoscaling_options: core::default::Default::default(),
            autoscaling_limits: core::default::Default::default(),
            autoscaling_targets: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct SpannerInstanceAutoscalingConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SpannerInstanceAutoscalingConfigElRef {
    fn new(shared: StackShared, base: String) -> SpannerInstanceAutoscalingConfigElRef {
        SpannerInstanceAutoscalingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SpannerInstanceAutoscalingConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `asymmetric_autoscaling_options` after provisioning.\n"]
    pub fn asymmetric_autoscaling_options(
        &self,
    ) -> ListRef<SpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.asymmetric_autoscaling_options", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `autoscaling_limits` after provisioning.\n"]
    pub fn autoscaling_limits(
        &self,
    ) -> ListRef<SpannerInstanceAutoscalingConfigElAutoscalingLimitsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.autoscaling_limits", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `autoscaling_targets` after provisioning.\n"]
    pub fn autoscaling_targets(
        &self,
    ) -> ListRef<SpannerInstanceAutoscalingConfigElAutoscalingTargetsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.autoscaling_targets", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct SpannerInstanceTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl SpannerInstanceTimeoutsEl {
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
impl ToListMappable for SpannerInstanceTimeoutsEl {
    type O = BlockAssignable<SpannerInstanceTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSpannerInstanceTimeoutsEl {}
impl BuildSpannerInstanceTimeoutsEl {
    pub fn build(self) -> SpannerInstanceTimeoutsEl {
        SpannerInstanceTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct SpannerInstanceTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SpannerInstanceTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> SpannerInstanceTimeoutsElRef {
        SpannerInstanceTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SpannerInstanceTimeoutsElRef {
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
struct SpannerInstanceDynamic {
    autoscaling_config: Option<DynamicBlock<SpannerInstanceAutoscalingConfigEl>>,
}
