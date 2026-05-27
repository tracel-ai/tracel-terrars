use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct VertexAiFeaturestoreEntitytypeData {
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
    description: Option<PrimField<String>>,
    featurestore: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    monitoring_config: Option<Vec<VertexAiFeaturestoreEntitytypeMonitoringConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<VertexAiFeaturestoreEntitytypeTimeoutsEl>,
    dynamic: VertexAiFeaturestoreEntitytypeDynamic,
}
struct VertexAiFeaturestoreEntitytype_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<VertexAiFeaturestoreEntitytypeData>,
}
#[derive(Clone)]
pub struct VertexAiFeaturestoreEntitytype(Rc<VertexAiFeaturestoreEntitytype_>);
impl VertexAiFeaturestoreEntitytype {
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
    #[doc = "Set the field `description`.\nOptional. Description of the EntityType."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nA set of key/value label pairs to assign to this EntityType.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\nThe name of the EntityType. This value may be up to 60 characters, and valid characters are [a-z0-9_]. The first character cannot be a number."]
    pub fn set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name = Some(v.into());
        self
    }
    #[doc = "Set the field `monitoring_config`.\n"]
    pub fn set_monitoring_config(
        self,
        v: impl Into<BlockAssignable<VertexAiFeaturestoreEntitytypeMonitoringConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().monitoring_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.monitoring_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<VertexAiFeaturestoreEntitytypeTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp of when the featurestore was created in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nOptional. Description of the EntityType."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nUsed to perform consistent read-modify-write updates."]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.etag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `featurestore` after provisioning.\nThe name of the Featurestore to use, in the format projects/{project}/locations/{location}/featurestores/{featurestore}."]
    pub fn featurestore(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.featurestore", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nA set of key/value label pairs to assign to this EntityType.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the EntityType. This value may be up to 60 characters, and valid characters are [a-z0-9_]. The first character cannot be a number."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nThe region of the EntityType."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe timestamp of when the featurestore was last updated in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `monitoring_config` after provisioning.\n"]
    pub fn monitoring_config(
        &self,
    ) -> ListRef<VertexAiFeaturestoreEntitytypeMonitoringConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.monitoring_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VertexAiFeaturestoreEntitytypeTimeoutsElRef {
        VertexAiFeaturestoreEntitytypeTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for VertexAiFeaturestoreEntitytype {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for VertexAiFeaturestoreEntitytype {}
impl ToListMappable for VertexAiFeaturestoreEntitytype {
    type O = ListRef<VertexAiFeaturestoreEntitytypeRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for VertexAiFeaturestoreEntitytype_ {
    fn extract_resource_type(&self) -> String {
        "google_vertex_ai_featurestore_entitytype".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildVertexAiFeaturestoreEntitytype {
    pub tf_id: String,
    #[doc = "The name of the Featurestore to use, in the format projects/{project}/locations/{location}/featurestores/{featurestore}."]
    pub featurestore: PrimField<String>,
}
impl BuildVertexAiFeaturestoreEntitytype {
    pub fn build(self, stack: &mut Stack) -> VertexAiFeaturestoreEntitytype {
        let out = VertexAiFeaturestoreEntitytype(Rc::new(VertexAiFeaturestoreEntitytype_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(VertexAiFeaturestoreEntitytypeData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                featurestore: self.featurestore,
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                name: core::default::Default::default(),
                monitoring_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct VertexAiFeaturestoreEntitytypeRef {
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiFeaturestoreEntitytypeRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl VertexAiFeaturestoreEntitytypeRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp of when the featurestore was created in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nOptional. Description of the EntityType."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nUsed to perform consistent read-modify-write updates."]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.etag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `featurestore` after provisioning.\nThe name of the Featurestore to use, in the format projects/{project}/locations/{location}/featurestores/{featurestore}."]
    pub fn featurestore(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.featurestore", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nA set of key/value label pairs to assign to this EntityType.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the EntityType. This value may be up to 60 characters, and valid characters are [a-z0-9_]. The first character cannot be a number."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nThe region of the EntityType."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe timestamp of when the featurestore was last updated in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `monitoring_config` after provisioning.\n"]
    pub fn monitoring_config(
        &self,
    ) -> ListRef<VertexAiFeaturestoreEntitytypeMonitoringConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.monitoring_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VertexAiFeaturestoreEntitytypeTimeoutsElRef {
        VertexAiFeaturestoreEntitytypeTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct VertexAiFeaturestoreEntitytypeMonitoringConfigElCategoricalThresholdConfigEl {
    value: PrimField<f64>,
}
impl VertexAiFeaturestoreEntitytypeMonitoringConfigElCategoricalThresholdConfigEl {}
impl ToListMappable
    for VertexAiFeaturestoreEntitytypeMonitoringConfigElCategoricalThresholdConfigEl
{
    type O = BlockAssignable<
        VertexAiFeaturestoreEntitytypeMonitoringConfigElCategoricalThresholdConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVertexAiFeaturestoreEntitytypeMonitoringConfigElCategoricalThresholdConfigEl {
    #[doc = "Specify a threshold value that can trigger the alert. For categorical feature, the distribution distance is calculated by L-inifinity norm. Each feature must have a non-zero threshold if they need to be monitored. Otherwise no alert will be triggered for that feature. The default value is 0.3."]
    pub value: PrimField<f64>,
}
impl BuildVertexAiFeaturestoreEntitytypeMonitoringConfigElCategoricalThresholdConfigEl {
    pub fn build(
        self,
    ) -> VertexAiFeaturestoreEntitytypeMonitoringConfigElCategoricalThresholdConfigEl {
        VertexAiFeaturestoreEntitytypeMonitoringConfigElCategoricalThresholdConfigEl {
            value: self.value,
        }
    }
}
pub struct VertexAiFeaturestoreEntitytypeMonitoringConfigElCategoricalThresholdConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiFeaturestoreEntitytypeMonitoringConfigElCategoricalThresholdConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> VertexAiFeaturestoreEntitytypeMonitoringConfigElCategoricalThresholdConfigElRef {
        VertexAiFeaturestoreEntitytypeMonitoringConfigElCategoricalThresholdConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VertexAiFeaturestoreEntitytypeMonitoringConfigElCategoricalThresholdConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\nSpecify a threshold value that can trigger the alert. For categorical feature, the distribution distance is calculated by L-inifinity norm. Each feature must have a non-zero threshold if they need to be monitored. Otherwise no alert will be triggered for that feature. The default value is 0.3."]
    pub fn value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct VertexAiFeaturestoreEntitytypeMonitoringConfigElImportFeaturesAnalysisEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    anomaly_detection_baseline: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
}
impl VertexAiFeaturestoreEntitytypeMonitoringConfigElImportFeaturesAnalysisEl {
    #[doc = "Set the field `anomaly_detection_baseline`.\nDefines the baseline to do anomaly detection for feature values imported by each [entityTypes.importFeatureValues][] operation. The value must be one of the values below:\n* LATEST_STATS: Choose the later one statistics generated by either most recent snapshot analysis or previous import features analysis. If non of them exists, skip anomaly detection and only generate a statistics.\n* MOST_RECENT_SNAPSHOT_STATS: Use the statistics generated by the most recent snapshot analysis if exists.\n* PREVIOUS_IMPORT_FEATURES_STATS: Use the statistics generated by the previous import features analysis if exists."]
    pub fn set_anomaly_detection_baseline(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.anomaly_detection_baseline = Some(v.into());
        self
    }
    #[doc = "Set the field `state`.\nWhether to enable / disable / inherite default hebavior for import features analysis. The value must be one of the values below:\n* DEFAULT: The default behavior of whether to enable the monitoring. EntityType-level config: disabled.\n* ENABLED: Explicitly enables import features analysis. EntityType-level config: by default enables import features analysis for all Features under it.\n* DISABLED: Explicitly disables import features analysis. EntityType-level config: by default disables import features analysis for all Features under it."]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }
}
impl ToListMappable for VertexAiFeaturestoreEntitytypeMonitoringConfigElImportFeaturesAnalysisEl {
    type O =
        BlockAssignable<VertexAiFeaturestoreEntitytypeMonitoringConfigElImportFeaturesAnalysisEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVertexAiFeaturestoreEntitytypeMonitoringConfigElImportFeaturesAnalysisEl {}
impl BuildVertexAiFeaturestoreEntitytypeMonitoringConfigElImportFeaturesAnalysisEl {
    pub fn build(self) -> VertexAiFeaturestoreEntitytypeMonitoringConfigElImportFeaturesAnalysisEl {
        VertexAiFeaturestoreEntitytypeMonitoringConfigElImportFeaturesAnalysisEl {
            anomaly_detection_baseline: core::default::Default::default(),
            state: core::default::Default::default(),
        }
    }
}
pub struct VertexAiFeaturestoreEntitytypeMonitoringConfigElImportFeaturesAnalysisElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiFeaturestoreEntitytypeMonitoringConfigElImportFeaturesAnalysisElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> VertexAiFeaturestoreEntitytypeMonitoringConfigElImportFeaturesAnalysisElRef {
        VertexAiFeaturestoreEntitytypeMonitoringConfigElImportFeaturesAnalysisElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VertexAiFeaturestoreEntitytypeMonitoringConfigElImportFeaturesAnalysisElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `anomaly_detection_baseline` after provisioning.\nDefines the baseline to do anomaly detection for feature values imported by each [entityTypes.importFeatureValues][] operation. The value must be one of the values below:\n* LATEST_STATS: Choose the later one statistics generated by either most recent snapshot analysis or previous import features analysis. If non of them exists, skip anomaly detection and only generate a statistics.\n* MOST_RECENT_SNAPSHOT_STATS: Use the statistics generated by the most recent snapshot analysis if exists.\n* PREVIOUS_IMPORT_FEATURES_STATS: Use the statistics generated by the previous import features analysis if exists."]
    pub fn anomaly_detection_baseline(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.anomaly_detection_baseline", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nWhether to enable / disable / inherite default hebavior for import features analysis. The value must be one of the values below:\n* DEFAULT: The default behavior of whether to enable the monitoring. EntityType-level config: disabled.\n* ENABLED: Explicitly enables import features analysis. EntityType-level config: by default enables import features analysis for all Features under it.\n* DISABLED: Explicitly disables import features analysis. EntityType-level config: by default disables import features analysis for all Features under it."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}
#[derive(Serialize)]
pub struct VertexAiFeaturestoreEntitytypeMonitoringConfigElNumericalThresholdConfigEl {
    value: PrimField<f64>,
}
impl VertexAiFeaturestoreEntitytypeMonitoringConfigElNumericalThresholdConfigEl {}
impl ToListMappable for VertexAiFeaturestoreEntitytypeMonitoringConfigElNumericalThresholdConfigEl {
    type O =
        BlockAssignable<VertexAiFeaturestoreEntitytypeMonitoringConfigElNumericalThresholdConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVertexAiFeaturestoreEntitytypeMonitoringConfigElNumericalThresholdConfigEl {
    #[doc = "Specify a threshold value that can trigger the alert. For numerical feature, the distribution distance is calculated by Jensen–Shannon divergence. Each feature must have a non-zero threshold if they need to be monitored. Otherwise no alert will be triggered for that feature. The default value is 0.3."]
    pub value: PrimField<f64>,
}
impl BuildVertexAiFeaturestoreEntitytypeMonitoringConfigElNumericalThresholdConfigEl {
    pub fn build(
        self,
    ) -> VertexAiFeaturestoreEntitytypeMonitoringConfigElNumericalThresholdConfigEl {
        VertexAiFeaturestoreEntitytypeMonitoringConfigElNumericalThresholdConfigEl {
            value: self.value,
        }
    }
}
pub struct VertexAiFeaturestoreEntitytypeMonitoringConfigElNumericalThresholdConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiFeaturestoreEntitytypeMonitoringConfigElNumericalThresholdConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> VertexAiFeaturestoreEntitytypeMonitoringConfigElNumericalThresholdConfigElRef {
        VertexAiFeaturestoreEntitytypeMonitoringConfigElNumericalThresholdConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VertexAiFeaturestoreEntitytypeMonitoringConfigElNumericalThresholdConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\nSpecify a threshold value that can trigger the alert. For numerical feature, the distribution distance is calculated by Jensen–Shannon divergence. Each feature must have a non-zero threshold if they need to be monitored. Otherwise no alert will be triggered for that feature. The default value is 0.3."]
    pub fn value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct VertexAiFeaturestoreEntitytypeMonitoringConfigElSnapshotAnalysisEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    monitoring_interval_days: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    staleness_days: Option<PrimField<f64>>,
}
impl VertexAiFeaturestoreEntitytypeMonitoringConfigElSnapshotAnalysisEl {
    #[doc = "Set the field `disabled`.\nThe monitoring schedule for snapshot analysis. For EntityType-level config: unset / disabled = true indicates disabled by default for Features under it; otherwise by default enable snapshot analysis monitoring with monitoringInterval for Features under it."]
    pub fn set_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disabled = Some(v.into());
        self
    }
    #[doc = "Set the field `monitoring_interval_days`.\nConfiguration of the snapshot analysis based monitoring pipeline running interval. The value indicates number of days. The default value is 1.\nIf both FeaturestoreMonitoringConfig.SnapshotAnalysis.monitoring_interval_days and [FeaturestoreMonitoringConfig.SnapshotAnalysis.monitoring_interval][] are set when creating/updating EntityTypes/Features, FeaturestoreMonitoringConfig.SnapshotAnalysis.monitoring_interval_days will be used."]
    pub fn set_monitoring_interval_days(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.monitoring_interval_days = Some(v.into());
        self
    }
    #[doc = "Set the field `staleness_days`.\nCustomized export features time window for snapshot analysis. Unit is one day. The default value is 21 days. Minimum value is 1 day. Maximum value is 4000 days."]
    pub fn set_staleness_days(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.staleness_days = Some(v.into());
        self
    }
}
impl ToListMappable for VertexAiFeaturestoreEntitytypeMonitoringConfigElSnapshotAnalysisEl {
    type O = BlockAssignable<VertexAiFeaturestoreEntitytypeMonitoringConfigElSnapshotAnalysisEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVertexAiFeaturestoreEntitytypeMonitoringConfigElSnapshotAnalysisEl {}
impl BuildVertexAiFeaturestoreEntitytypeMonitoringConfigElSnapshotAnalysisEl {
    pub fn build(self) -> VertexAiFeaturestoreEntitytypeMonitoringConfigElSnapshotAnalysisEl {
        VertexAiFeaturestoreEntitytypeMonitoringConfigElSnapshotAnalysisEl {
            disabled: core::default::Default::default(),
            monitoring_interval_days: core::default::Default::default(),
            staleness_days: core::default::Default::default(),
        }
    }
}
pub struct VertexAiFeaturestoreEntitytypeMonitoringConfigElSnapshotAnalysisElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiFeaturestoreEntitytypeMonitoringConfigElSnapshotAnalysisElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> VertexAiFeaturestoreEntitytypeMonitoringConfigElSnapshotAnalysisElRef {
        VertexAiFeaturestoreEntitytypeMonitoringConfigElSnapshotAnalysisElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VertexAiFeaturestoreEntitytypeMonitoringConfigElSnapshotAnalysisElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\nThe monitoring schedule for snapshot analysis. For EntityType-level config: unset / disabled = true indicates disabled by default for Features under it; otherwise by default enable snapshot analysis monitoring with monitoringInterval for Features under it."]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
    #[doc = "Get a reference to the value of field `monitoring_interval_days` after provisioning.\nConfiguration of the snapshot analysis based monitoring pipeline running interval. The value indicates number of days. The default value is 1.\nIf both FeaturestoreMonitoringConfig.SnapshotAnalysis.monitoring_interval_days and [FeaturestoreMonitoringConfig.SnapshotAnalysis.monitoring_interval][] are set when creating/updating EntityTypes/Features, FeaturestoreMonitoringConfig.SnapshotAnalysis.monitoring_interval_days will be used."]
    pub fn monitoring_interval_days(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.monitoring_interval_days", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `staleness_days` after provisioning.\nCustomized export features time window for snapshot analysis. Unit is one day. The default value is 21 days. Minimum value is 1 day. Maximum value is 4000 days."]
    pub fn staleness_days(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.staleness_days", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct VertexAiFeaturestoreEntitytypeMonitoringConfigElDynamic {
    categorical_threshold_config: Option<
        DynamicBlock<VertexAiFeaturestoreEntitytypeMonitoringConfigElCategoricalThresholdConfigEl>,
    >,
    import_features_analysis: Option<
        DynamicBlock<VertexAiFeaturestoreEntitytypeMonitoringConfigElImportFeaturesAnalysisEl>,
    >,
    numerical_threshold_config: Option<
        DynamicBlock<VertexAiFeaturestoreEntitytypeMonitoringConfigElNumericalThresholdConfigEl>,
    >,
    snapshot_analysis:
        Option<DynamicBlock<VertexAiFeaturestoreEntitytypeMonitoringConfigElSnapshotAnalysisEl>>,
}
#[derive(Serialize)]
pub struct VertexAiFeaturestoreEntitytypeMonitoringConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    categorical_threshold_config:
        Option<Vec<VertexAiFeaturestoreEntitytypeMonitoringConfigElCategoricalThresholdConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    import_features_analysis:
        Option<Vec<VertexAiFeaturestoreEntitytypeMonitoringConfigElImportFeaturesAnalysisEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    numerical_threshold_config:
        Option<Vec<VertexAiFeaturestoreEntitytypeMonitoringConfigElNumericalThresholdConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    snapshot_analysis:
        Option<Vec<VertexAiFeaturestoreEntitytypeMonitoringConfigElSnapshotAnalysisEl>>,
    dynamic: VertexAiFeaturestoreEntitytypeMonitoringConfigElDynamic,
}
impl VertexAiFeaturestoreEntitytypeMonitoringConfigEl {
    #[doc = "Set the field `categorical_threshold_config`.\n"]
    pub fn set_categorical_threshold_config(
        mut self,
        v: impl Into<
            BlockAssignable<
                VertexAiFeaturestoreEntitytypeMonitoringConfigElCategoricalThresholdConfigEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.categorical_threshold_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.categorical_threshold_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `import_features_analysis`.\n"]
    pub fn set_import_features_analysis(
        mut self,
        v: impl Into<
            BlockAssignable<
                VertexAiFeaturestoreEntitytypeMonitoringConfigElImportFeaturesAnalysisEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.import_features_analysis = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.import_features_analysis = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `numerical_threshold_config`.\n"]
    pub fn set_numerical_threshold_config(
        mut self,
        v: impl Into<
            BlockAssignable<
                VertexAiFeaturestoreEntitytypeMonitoringConfigElNumericalThresholdConfigEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.numerical_threshold_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.numerical_threshold_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `snapshot_analysis`.\n"]
    pub fn set_snapshot_analysis(
        mut self,
        v: impl Into<
            BlockAssignable<VertexAiFeaturestoreEntitytypeMonitoringConfigElSnapshotAnalysisEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.snapshot_analysis = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.snapshot_analysis = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for VertexAiFeaturestoreEntitytypeMonitoringConfigEl {
    type O = BlockAssignable<VertexAiFeaturestoreEntitytypeMonitoringConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVertexAiFeaturestoreEntitytypeMonitoringConfigEl {}
impl BuildVertexAiFeaturestoreEntitytypeMonitoringConfigEl {
    pub fn build(self) -> VertexAiFeaturestoreEntitytypeMonitoringConfigEl {
        VertexAiFeaturestoreEntitytypeMonitoringConfigEl {
            categorical_threshold_config: core::default::Default::default(),
            import_features_analysis: core::default::Default::default(),
            numerical_threshold_config: core::default::Default::default(),
            snapshot_analysis: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct VertexAiFeaturestoreEntitytypeMonitoringConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiFeaturestoreEntitytypeMonitoringConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> VertexAiFeaturestoreEntitytypeMonitoringConfigElRef {
        VertexAiFeaturestoreEntitytypeMonitoringConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VertexAiFeaturestoreEntitytypeMonitoringConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `categorical_threshold_config` after provisioning.\n"]
    pub fn categorical_threshold_config(
        &self,
    ) -> ListRef<VertexAiFeaturestoreEntitytypeMonitoringConfigElCategoricalThresholdConfigElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.categorical_threshold_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `import_features_analysis` after provisioning.\n"]
    pub fn import_features_analysis(
        &self,
    ) -> ListRef<VertexAiFeaturestoreEntitytypeMonitoringConfigElImportFeaturesAnalysisElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.import_features_analysis", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `numerical_threshold_config` after provisioning.\n"]
    pub fn numerical_threshold_config(
        &self,
    ) -> ListRef<VertexAiFeaturestoreEntitytypeMonitoringConfigElNumericalThresholdConfigElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.numerical_threshold_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `snapshot_analysis` after provisioning.\n"]
    pub fn snapshot_analysis(
        &self,
    ) -> ListRef<VertexAiFeaturestoreEntitytypeMonitoringConfigElSnapshotAnalysisElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.snapshot_analysis", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct VertexAiFeaturestoreEntitytypeTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl VertexAiFeaturestoreEntitytypeTimeoutsEl {
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
impl ToListMappable for VertexAiFeaturestoreEntitytypeTimeoutsEl {
    type O = BlockAssignable<VertexAiFeaturestoreEntitytypeTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVertexAiFeaturestoreEntitytypeTimeoutsEl {}
impl BuildVertexAiFeaturestoreEntitytypeTimeoutsEl {
    pub fn build(self) -> VertexAiFeaturestoreEntitytypeTimeoutsEl {
        VertexAiFeaturestoreEntitytypeTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct VertexAiFeaturestoreEntitytypeTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiFeaturestoreEntitytypeTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> VertexAiFeaturestoreEntitytypeTimeoutsElRef {
        VertexAiFeaturestoreEntitytypeTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VertexAiFeaturestoreEntitytypeTimeoutsElRef {
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
struct VertexAiFeaturestoreEntitytypeDynamic {
    monitoring_config: Option<DynamicBlock<VertexAiFeaturestoreEntitytypeMonitoringConfigEl>>,
}
