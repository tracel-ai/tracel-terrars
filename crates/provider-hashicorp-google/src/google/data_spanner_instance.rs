use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataSpannerInstanceData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    config: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
}
struct DataSpannerInstance_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataSpannerInstanceData>,
}
#[derive(Clone)]
pub struct DataSpannerInstance(Rc<DataSpannerInstance_>);
impl DataSpannerInstance {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(&self, provider: &ProviderGoogle) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }
    #[doc = "Set the field `config`.\nThe name of the instance's configuration (similar but not\nquite the same as a region) which defines the geographic placement and\nreplication of your databases in this instance. It determines where your data\nis stored. Values are typically of the form 'regional-europe-west1' , 'us-central' etc.\nIn order to obtain a valid list please consult the\n[Configuration section of the docs](https://cloud.google.com/spanner/docs/instances)."]
    pub fn set_config(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().config = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\nThe descriptive name for this instance as it appears in UIs. Must be\nunique per project and between 4 and 30 characters in length."]
    pub fn set_display_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `autoscaling_config` after provisioning.\nThe autoscaling configuration. Autoscaling is enabled if this field is set.\nExactly one of either num_nodes, processing_units or autoscaling_config must be\npresent in terraform except when instance_type = FREE_INSTANCE.\nWhen autoscaling is enabled, num_nodes and processing_units are treated as,\nOUTPUT_ONLY fields and reflect the current compute capacity allocated to\nthe instance."]
    pub fn autoscaling_config(&self) -> ListRef<DataSpannerInstanceAutoscalingConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.autoscaling_config", self.extract_ref()),
        )
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
}
impl Referable for DataSpannerInstance {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataSpannerInstance {}
impl ToListMappable for DataSpannerInstance {
    type O = ListRef<DataSpannerInstanceRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataSpannerInstance_ {
    fn extract_datasource_type(&self) -> String {
        "google_spanner_instance".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataSpannerInstance {
    pub tf_id: String,
    #[doc = "A unique identifier for the instance, which cannot be changed after\nthe instance is created. The name must be between 6 and 30 characters\nin length.\nIf not provided, a random string starting with 'tf-' will be selected."]
    pub name: PrimField<String>,
}
impl BuildDataSpannerInstance {
    pub fn build(self, stack: &mut Stack) -> DataSpannerInstance {
        let out = DataSpannerInstance(Rc::new(DataSpannerInstance_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataSpannerInstanceData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                config: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataSpannerInstanceRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataSpannerInstanceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataSpannerInstanceRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `autoscaling_config` after provisioning.\nThe autoscaling configuration. Autoscaling is enabled if this field is set.\nExactly one of either num_nodes, processing_units or autoscaling_config must be\npresent in terraform except when instance_type = FREE_INSTANCE.\nWhen autoscaling is enabled, num_nodes and processing_units are treated as,\nOUTPUT_ONLY fields and reflect the current compute capacity allocated to\nthe instance."]
    pub fn autoscaling_config(&self) -> ListRef<DataSpannerInstanceAutoscalingConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.autoscaling_config", self.extract_ref()),
        )
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
}
#[derive(Serialize)]
pub struct DataSpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElOverridesElAutoscalingLimitsEl
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
impl DataSpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElOverridesElAutoscalingLimitsEl { # [doc = "Set the field `max_nodes`.\n"] pub fn set_max_nodes (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . max_nodes = Some (v . into ()) ; self } # [doc = "Set the field `max_processing_units`.\n"] pub fn set_max_processing_units (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . max_processing_units = Some (v . into ()) ; self } # [doc = "Set the field `min_nodes`.\n"] pub fn set_min_nodes (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . min_nodes = Some (v . into ()) ; self } # [doc = "Set the field `min_processing_units`.\n"] pub fn set_min_processing_units (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . min_processing_units = Some (v . into ()) ; self } }
impl ToListMappable for DataSpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElOverridesElAutoscalingLimitsEl { type O = BlockAssignable < DataSpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElOverridesElAutoscalingLimitsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataSpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElOverridesElAutoscalingLimitsEl
{}
impl BuildDataSpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElOverridesElAutoscalingLimitsEl { pub fn build (self) -> DataSpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElOverridesElAutoscalingLimitsEl { DataSpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElOverridesElAutoscalingLimitsEl { max_nodes : core :: default :: Default :: default () , max_processing_units : core :: default :: Default :: default () , min_nodes : core :: default :: Default :: default () , min_processing_units : core :: default :: Default :: default () , } } }
pub struct DataSpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElOverridesElAutoscalingLimitsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataSpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElOverridesElAutoscalingLimitsElRef { fn new (shared : StackShared , base : String) -> DataSpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElOverridesElAutoscalingLimitsElRef { DataSpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElOverridesElAutoscalingLimitsElRef { shared : shared , base : base . to_string () , } } }
impl DataSpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElOverridesElAutoscalingLimitsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `max_nodes` after provisioning.\n"] pub fn max_nodes (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.max_nodes" , self . base)) } # [doc = "Get a reference to the value of field `max_processing_units` after provisioning.\n"] pub fn max_processing_units (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.max_processing_units" , self . base)) } # [doc = "Get a reference to the value of field `min_nodes` after provisioning.\n"] pub fn min_nodes (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.min_nodes" , self . base)) } # [doc = "Get a reference to the value of field `min_processing_units` after provisioning.\n"] pub fn min_processing_units (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.min_processing_units" , self . base)) } }
#[derive(Serialize)]
pub struct DataSpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElOverridesEl { # [serde (skip_serializing_if = "Option::is_none")] autoscaling_limits : Option < ListField < DataSpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElOverridesElAutoscalingLimitsEl > > , # [serde (skip_serializing_if = "Option::is_none")] autoscaling_target_high_priority_cpu_utilization_percent : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] autoscaling_target_total_cpu_utilization_percent : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] disable_high_priority_cpu_autoscaling : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] disable_total_cpu_autoscaling : Option < PrimField < bool > > , }
impl DataSpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElOverridesEl {
    #[doc = "Set the field `autoscaling_limits`.\n"]
    pub fn set_autoscaling_limits(
        mut self,
        v : impl Into < ListField < DataSpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElOverridesElAutoscalingLimitsEl > >,
    ) -> Self {
        self.autoscaling_limits = Some(v.into());
        self
    }
    #[doc = "Set the field `autoscaling_target_high_priority_cpu_utilization_percent`.\n"]
    pub fn set_autoscaling_target_high_priority_cpu_utilization_percent(
        mut self,
        v: impl Into<PrimField<f64>>,
    ) -> Self {
        self.autoscaling_target_high_priority_cpu_utilization_percent = Some(v.into());
        self
    }
    #[doc = "Set the field `autoscaling_target_total_cpu_utilization_percent`.\n"]
    pub fn set_autoscaling_target_total_cpu_utilization_percent(
        mut self,
        v: impl Into<PrimField<f64>>,
    ) -> Self {
        self.autoscaling_target_total_cpu_utilization_percent = Some(v.into());
        self
    }
    #[doc = "Set the field `disable_high_priority_cpu_autoscaling`.\n"]
    pub fn set_disable_high_priority_cpu_autoscaling(
        mut self,
        v: impl Into<PrimField<bool>>,
    ) -> Self {
        self.disable_high_priority_cpu_autoscaling = Some(v.into());
        self
    }
    #[doc = "Set the field `disable_total_cpu_autoscaling`.\n"]
    pub fn set_disable_total_cpu_autoscaling(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disable_total_cpu_autoscaling = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataSpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElOverridesEl
{
    type O = BlockAssignable<
        DataSpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElOverridesEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataSpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElOverridesEl {}
impl BuildDataSpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElOverridesEl {
    pub fn build(
        self,
    ) -> DataSpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElOverridesEl {
        DataSpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElOverridesEl {
            autoscaling_limits: core::default::Default::default(),
            autoscaling_target_high_priority_cpu_utilization_percent:
                core::default::Default::default(),
            autoscaling_target_total_cpu_utilization_percent: core::default::Default::default(),
            disable_high_priority_cpu_autoscaling: core::default::Default::default(),
            disable_total_cpu_autoscaling: core::default::Default::default(),
        }
    }
}
pub struct DataSpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElOverridesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataSpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElOverridesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataSpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElOverridesElRef {
        DataSpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElOverridesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataSpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElOverridesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `autoscaling_limits` after provisioning.\n"]    pub fn autoscaling_limits (& self) -> ListRef < DataSpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElOverridesElAutoscalingLimitsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.autoscaling_limits", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `autoscaling_target_high_priority_cpu_utilization_percent` after provisioning.\n"]
    pub fn autoscaling_target_high_priority_cpu_utilization_percent(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.autoscaling_target_high_priority_cpu_utilization_percent",
                self.base
            ),
        )
    }
    #[doc = "Get a reference to the value of field `autoscaling_target_total_cpu_utilization_percent` after provisioning.\n"]
    pub fn autoscaling_target_total_cpu_utilization_percent(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.autoscaling_target_total_cpu_utilization_percent",
                self.base
            ),
        )
    }
    #[doc = "Get a reference to the value of field `disable_high_priority_cpu_autoscaling` after provisioning.\n"]
    pub fn disable_high_priority_cpu_autoscaling(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_high_priority_cpu_autoscaling", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `disable_total_cpu_autoscaling` after provisioning.\n"]
    pub fn disable_total_cpu_autoscaling(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_total_cpu_autoscaling", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataSpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElReplicaSelectionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
}
impl DataSpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElReplicaSelectionEl {
    #[doc = "Set the field `location`.\n"]
    pub fn set_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.location = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataSpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElReplicaSelectionEl
{
    type O = BlockAssignable<
        DataSpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElReplicaSelectionEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataSpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElReplicaSelectionEl
{}
impl BuildDataSpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElReplicaSelectionEl {
    pub fn build(
        self,
    ) -> DataSpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElReplicaSelectionEl
    {
        DataSpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElReplicaSelectionEl {
            location: core::default::Default::default(),
        }
    }
}
pub struct DataSpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElReplicaSelectionElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for DataSpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElReplicaSelectionElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataSpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElReplicaSelectionElRef
    {
        DataSpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElReplicaSelectionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataSpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElReplicaSelectionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }
}
#[derive(Serialize)]
pub struct DataSpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    overrides: Option<
        ListField<DataSpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElOverridesEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    replica_selection: Option<
        ListField<
            DataSpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElReplicaSelectionEl,
        >,
    >,
}
impl DataSpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsEl {
    #[doc = "Set the field `overrides`.\n"]
    pub fn set_overrides(
        mut self,
        v: impl Into<
            ListField<
                DataSpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElOverridesEl,
            >,
        >,
    ) -> Self {
        self.overrides = Some(v.into());
        self
    }
    #[doc = "Set the field `replica_selection`.\n"]
    pub fn set_replica_selection(
        mut self,
        v : impl Into < ListField < DataSpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElReplicaSelectionEl > >,
    ) -> Self {
        self.replica_selection = Some(v.into());
        self
    }
}
impl ToListMappable for DataSpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsEl {
    type O = BlockAssignable<DataSpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataSpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsEl {}
impl BuildDataSpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsEl {
    pub fn build(self) -> DataSpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsEl {
        DataSpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsEl {
            overrides: core::default::Default::default(),
            replica_selection: core::default::Default::default(),
        }
    }
}
pub struct DataSpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataSpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataSpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElRef {
        DataSpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataSpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `overrides` after provisioning.\n"]
    pub fn overrides(
        &self,
    ) -> ListRef<DataSpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElOverridesElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.overrides", self.base))
    }
    #[doc = "Get a reference to the value of field `replica_selection` after provisioning.\n"]
    pub fn replica_selection(
        &self,
    ) -> ListRef<
        DataSpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElReplicaSelectionElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.replica_selection", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataSpannerInstanceAutoscalingConfigElAutoscalingLimitsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_nodes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_processing_units: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_nodes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_processing_units: Option<PrimField<f64>>,
}
impl DataSpannerInstanceAutoscalingConfigElAutoscalingLimitsEl {
    #[doc = "Set the field `max_nodes`.\n"]
    pub fn set_max_nodes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_nodes = Some(v.into());
        self
    }
    #[doc = "Set the field `max_processing_units`.\n"]
    pub fn set_max_processing_units(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_processing_units = Some(v.into());
        self
    }
    #[doc = "Set the field `min_nodes`.\n"]
    pub fn set_min_nodes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min_nodes = Some(v.into());
        self
    }
    #[doc = "Set the field `min_processing_units`.\n"]
    pub fn set_min_processing_units(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min_processing_units = Some(v.into());
        self
    }
}
impl ToListMappable for DataSpannerInstanceAutoscalingConfigElAutoscalingLimitsEl {
    type O = BlockAssignable<DataSpannerInstanceAutoscalingConfigElAutoscalingLimitsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataSpannerInstanceAutoscalingConfigElAutoscalingLimitsEl {}
impl BuildDataSpannerInstanceAutoscalingConfigElAutoscalingLimitsEl {
    pub fn build(self) -> DataSpannerInstanceAutoscalingConfigElAutoscalingLimitsEl {
        DataSpannerInstanceAutoscalingConfigElAutoscalingLimitsEl {
            max_nodes: core::default::Default::default(),
            max_processing_units: core::default::Default::default(),
            min_nodes: core::default::Default::default(),
            min_processing_units: core::default::Default::default(),
        }
    }
}
pub struct DataSpannerInstanceAutoscalingConfigElAutoscalingLimitsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataSpannerInstanceAutoscalingConfigElAutoscalingLimitsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataSpannerInstanceAutoscalingConfigElAutoscalingLimitsElRef {
        DataSpannerInstanceAutoscalingConfigElAutoscalingLimitsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataSpannerInstanceAutoscalingConfigElAutoscalingLimitsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `max_nodes` after provisioning.\n"]
    pub fn max_nodes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_nodes", self.base))
    }
    #[doc = "Get a reference to the value of field `max_processing_units` after provisioning.\n"]
    pub fn max_processing_units(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_processing_units", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `min_nodes` after provisioning.\n"]
    pub fn min_nodes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_nodes", self.base))
    }
    #[doc = "Get a reference to the value of field `min_processing_units` after provisioning.\n"]
    pub fn min_processing_units(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_processing_units", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataSpannerInstanceAutoscalingConfigElAutoscalingTargetsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    high_priority_cpu_utilization_percent: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_utilization_percent: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    total_cpu_utilization_percent: Option<PrimField<f64>>,
}
impl DataSpannerInstanceAutoscalingConfigElAutoscalingTargetsEl {
    #[doc = "Set the field `high_priority_cpu_utilization_percent`.\n"]
    pub fn set_high_priority_cpu_utilization_percent(
        mut self,
        v: impl Into<PrimField<f64>>,
    ) -> Self {
        self.high_priority_cpu_utilization_percent = Some(v.into());
        self
    }
    #[doc = "Set the field `storage_utilization_percent`.\n"]
    pub fn set_storage_utilization_percent(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.storage_utilization_percent = Some(v.into());
        self
    }
    #[doc = "Set the field `total_cpu_utilization_percent`.\n"]
    pub fn set_total_cpu_utilization_percent(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.total_cpu_utilization_percent = Some(v.into());
        self
    }
}
impl ToListMappable for DataSpannerInstanceAutoscalingConfigElAutoscalingTargetsEl {
    type O = BlockAssignable<DataSpannerInstanceAutoscalingConfigElAutoscalingTargetsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataSpannerInstanceAutoscalingConfigElAutoscalingTargetsEl {}
impl BuildDataSpannerInstanceAutoscalingConfigElAutoscalingTargetsEl {
    pub fn build(self) -> DataSpannerInstanceAutoscalingConfigElAutoscalingTargetsEl {
        DataSpannerInstanceAutoscalingConfigElAutoscalingTargetsEl {
            high_priority_cpu_utilization_percent: core::default::Default::default(),
            storage_utilization_percent: core::default::Default::default(),
            total_cpu_utilization_percent: core::default::Default::default(),
        }
    }
}
pub struct DataSpannerInstanceAutoscalingConfigElAutoscalingTargetsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataSpannerInstanceAutoscalingConfigElAutoscalingTargetsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataSpannerInstanceAutoscalingConfigElAutoscalingTargetsElRef {
        DataSpannerInstanceAutoscalingConfigElAutoscalingTargetsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataSpannerInstanceAutoscalingConfigElAutoscalingTargetsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `high_priority_cpu_utilization_percent` after provisioning.\n"]
    pub fn high_priority_cpu_utilization_percent(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.high_priority_cpu_utilization_percent", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `storage_utilization_percent` after provisioning.\n"]
    pub fn storage_utilization_percent(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.storage_utilization_percent", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `total_cpu_utilization_percent` after provisioning.\n"]
    pub fn total_cpu_utilization_percent(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.total_cpu_utilization_percent", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataSpannerInstanceAutoscalingConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    asymmetric_autoscaling_options:
        Option<ListField<DataSpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    autoscaling_limits:
        Option<ListField<DataSpannerInstanceAutoscalingConfigElAutoscalingLimitsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    autoscaling_targets:
        Option<ListField<DataSpannerInstanceAutoscalingConfigElAutoscalingTargetsEl>>,
}
impl DataSpannerInstanceAutoscalingConfigEl {
    #[doc = "Set the field `asymmetric_autoscaling_options`.\n"]
    pub fn set_asymmetric_autoscaling_options(
        mut self,
        v: impl Into<ListField<DataSpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsEl>>,
    ) -> Self {
        self.asymmetric_autoscaling_options = Some(v.into());
        self
    }
    #[doc = "Set the field `autoscaling_limits`.\n"]
    pub fn set_autoscaling_limits(
        mut self,
        v: impl Into<ListField<DataSpannerInstanceAutoscalingConfigElAutoscalingLimitsEl>>,
    ) -> Self {
        self.autoscaling_limits = Some(v.into());
        self
    }
    #[doc = "Set the field `autoscaling_targets`.\n"]
    pub fn set_autoscaling_targets(
        mut self,
        v: impl Into<ListField<DataSpannerInstanceAutoscalingConfigElAutoscalingTargetsEl>>,
    ) -> Self {
        self.autoscaling_targets = Some(v.into());
        self
    }
}
impl ToListMappable for DataSpannerInstanceAutoscalingConfigEl {
    type O = BlockAssignable<DataSpannerInstanceAutoscalingConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataSpannerInstanceAutoscalingConfigEl {}
impl BuildDataSpannerInstanceAutoscalingConfigEl {
    pub fn build(self) -> DataSpannerInstanceAutoscalingConfigEl {
        DataSpannerInstanceAutoscalingConfigEl {
            asymmetric_autoscaling_options: core::default::Default::default(),
            autoscaling_limits: core::default::Default::default(),
            autoscaling_targets: core::default::Default::default(),
        }
    }
}
pub struct DataSpannerInstanceAutoscalingConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataSpannerInstanceAutoscalingConfigElRef {
    fn new(shared: StackShared, base: String) -> DataSpannerInstanceAutoscalingConfigElRef {
        DataSpannerInstanceAutoscalingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataSpannerInstanceAutoscalingConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `asymmetric_autoscaling_options` after provisioning.\n"]
    pub fn asymmetric_autoscaling_options(
        &self,
    ) -> ListRef<DataSpannerInstanceAutoscalingConfigElAsymmetricAutoscalingOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.asymmetric_autoscaling_options", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `autoscaling_limits` after provisioning.\n"]
    pub fn autoscaling_limits(
        &self,
    ) -> ListRef<DataSpannerInstanceAutoscalingConfigElAutoscalingLimitsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.autoscaling_limits", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `autoscaling_targets` after provisioning.\n"]
    pub fn autoscaling_targets(
        &self,
    ) -> ListRef<DataSpannerInstanceAutoscalingConfigElAutoscalingTargetsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.autoscaling_targets", self.base),
        )
    }
}
