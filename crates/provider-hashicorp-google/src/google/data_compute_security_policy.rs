use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataComputeSecurityPolicyData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    self_link: Option<PrimField<String>>,
}
struct DataComputeSecurityPolicy_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataComputeSecurityPolicyData>,
}
#[derive(Clone)]
pub struct DataComputeSecurityPolicy(Rc<DataComputeSecurityPolicy_>);
impl DataComputeSecurityPolicy {
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
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\nThe name of the security policy."]
    pub fn set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\nThe project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `self_link`.\nThe URI of the created resource."]
    pub fn set_self_link(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().self_link = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `adaptive_protection_config` after provisioning.\nAdaptive Protection Config of this security policy."]
    pub fn adaptive_protection_config(
        &self,
    ) -> ListRef<DataComputeSecurityPolicyAdaptiveProtectionConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.adaptive_protection_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `advanced_options_config` after provisioning.\nAdvanced Options Config of this security policy."]
    pub fn advanced_options_config(
        &self,
    ) -> ListRef<DataComputeSecurityPolicyAdvancedOptionsConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.advanced_options_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nAn optional description of this security policy. Max size is 2048."]
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
    #[doc = "Get a reference to the value of field `fingerprint` after provisioning.\nFingerprint of this resource."]
    pub fn fingerprint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.fingerprint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `label_fingerprint` after provisioning.\nThe unique fingerprint of the labels."]
    pub fn label_fingerprint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.label_fingerprint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nLabels to apply to this address.  A list of key->value pairs.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the security policy."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `recaptcha_options_config` after provisioning.\nreCAPTCHA configuration options to be applied for the security policy."]
    pub fn recaptcha_options_config(
        &self,
    ) -> ListRef<DataComputeSecurityPolicyRecaptchaOptionsConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.recaptcha_options_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rule` after provisioning.\nThe set of rules that belong to this policy. There must always be a default rule (rule with priority 2147483647 and match \"*\"). If no rules are provided when creating a security policy, a default rule with action \"allow\" will be added."]
    pub fn rule(&self) -> SetRef<DataComputeSecurityPolicyRuleElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.rule", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\nThe URI of the created resource."]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe type indicates the intended use of the security policy. CLOUD_ARMOR - Cloud Armor backend security policies can be configured to filter incoming HTTP requests targeting backend services. They filter requests before they hit the origin servers. CLOUD_ARMOR_EDGE - Cloud Armor edge security policies can be configured to filter incoming HTTP requests targeting backend services (including Cloud CDN-enabled) as well as backend buckets (Cloud Storage). They filter requests before the request is served from Google's cache."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
}
impl Referable for DataComputeSecurityPolicy {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataComputeSecurityPolicy {}
impl ToListMappable for DataComputeSecurityPolicy {
    type O = ListRef<DataComputeSecurityPolicyRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataComputeSecurityPolicy_ {
    fn extract_datasource_type(&self) -> String {
        "google_compute_security_policy".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataComputeSecurityPolicy {
    pub tf_id: String,
}
impl BuildDataComputeSecurityPolicy {
    pub fn build(self, stack: &mut Stack) -> DataComputeSecurityPolicy {
        let out = DataComputeSecurityPolicy(Rc::new(DataComputeSecurityPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataComputeSecurityPolicyData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                name: core::default::Default::default(),
                project: core::default::Default::default(),
                self_link: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataComputeSecurityPolicyRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeSecurityPolicyRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataComputeSecurityPolicyRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `adaptive_protection_config` after provisioning.\nAdaptive Protection Config of this security policy."]
    pub fn adaptive_protection_config(
        &self,
    ) -> ListRef<DataComputeSecurityPolicyAdaptiveProtectionConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.adaptive_protection_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `advanced_options_config` after provisioning.\nAdvanced Options Config of this security policy."]
    pub fn advanced_options_config(
        &self,
    ) -> ListRef<DataComputeSecurityPolicyAdvancedOptionsConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.advanced_options_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nAn optional description of this security policy. Max size is 2048."]
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
    #[doc = "Get a reference to the value of field `fingerprint` after provisioning.\nFingerprint of this resource."]
    pub fn fingerprint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.fingerprint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `label_fingerprint` after provisioning.\nThe unique fingerprint of the labels."]
    pub fn label_fingerprint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.label_fingerprint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nLabels to apply to this address.  A list of key->value pairs.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the security policy."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `recaptcha_options_config` after provisioning.\nreCAPTCHA configuration options to be applied for the security policy."]
    pub fn recaptcha_options_config(
        &self,
    ) -> ListRef<DataComputeSecurityPolicyRecaptchaOptionsConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.recaptcha_options_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rule` after provisioning.\nThe set of rules that belong to this policy. There must always be a default rule (rule with priority 2147483647 and match \"*\"). If no rules are provided when creating a security policy, a default rule with action \"allow\" will be added."]
    pub fn rule(&self) -> SetRef<DataComputeSecurityPolicyRuleElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.rule", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\nThe URI of the created resource."]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe type indicates the intended use of the security policy. CLOUD_ARMOR - Cloud Armor backend security policies can be configured to filter incoming HTTP requests targeting backend services. They filter requests before they hit the origin servers. CLOUD_ARMOR_EDGE - Cloud Armor edge security policies can be configured to filter incoming HTTP requests targeting backend services (including Cloud CDN-enabled) as well as backend buckets (Cloud Storage). They filter requests before the request is served from Google's cache."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeSecurityPolicyAdaptiveProtectionConfigElLayer7DdosDefenseConfigElThresholdConfigsElTrafficGranularityConfigsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_each_unique_value: Option<PrimField<bool>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl DataComputeSecurityPolicyAdaptiveProtectionConfigElLayer7DdosDefenseConfigElThresholdConfigsElTrafficGranularityConfigsEl { # [doc = "Set the field `enable_each_unique_value`.\n"] pub fn set_enable_each_unique_value (mut self , v : impl Into < PrimField < bool > >) -> Self { self . enable_each_unique_value = Some (v . into ()) ; self } # [doc = "Set the field `type_`.\n"] pub fn set_type (mut self , v : impl Into < PrimField < String > >) -> Self { self . type_ = Some (v . into ()) ; self } # [doc = "Set the field `value`.\n"] pub fn set_value (mut self , v : impl Into < PrimField < String > >) -> Self { self . value = Some (v . into ()) ; self } }
impl ToListMappable for DataComputeSecurityPolicyAdaptiveProtectionConfigElLayer7DdosDefenseConfigElThresholdConfigsElTrafficGranularityConfigsEl { type O = BlockAssignable < DataComputeSecurityPolicyAdaptiveProtectionConfigElLayer7DdosDefenseConfigElThresholdConfigsElTrafficGranularityConfigsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataComputeSecurityPolicyAdaptiveProtectionConfigElLayer7DdosDefenseConfigElThresholdConfigsElTrafficGranularityConfigsEl
{}
impl BuildDataComputeSecurityPolicyAdaptiveProtectionConfigElLayer7DdosDefenseConfigElThresholdConfigsElTrafficGranularityConfigsEl { pub fn build (self) -> DataComputeSecurityPolicyAdaptiveProtectionConfigElLayer7DdosDefenseConfigElThresholdConfigsElTrafficGranularityConfigsEl { DataComputeSecurityPolicyAdaptiveProtectionConfigElLayer7DdosDefenseConfigElThresholdConfigsElTrafficGranularityConfigsEl { enable_each_unique_value : core :: default :: Default :: default () , type_ : core :: default :: Default :: default () , value : core :: default :: Default :: default () , } } }
pub struct DataComputeSecurityPolicyAdaptiveProtectionConfigElLayer7DdosDefenseConfigElThresholdConfigsElTrafficGranularityConfigsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeSecurityPolicyAdaptiveProtectionConfigElLayer7DdosDefenseConfigElThresholdConfigsElTrafficGranularityConfigsElRef { fn new (shared : StackShared , base : String) -> DataComputeSecurityPolicyAdaptiveProtectionConfigElLayer7DdosDefenseConfigElThresholdConfigsElTrafficGranularityConfigsElRef { DataComputeSecurityPolicyAdaptiveProtectionConfigElLayer7DdosDefenseConfigElThresholdConfigsElTrafficGranularityConfigsElRef { shared : shared , base : base . to_string () , } } }
impl DataComputeSecurityPolicyAdaptiveProtectionConfigElLayer7DdosDefenseConfigElThresholdConfigsElTrafficGranularityConfigsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `enable_each_unique_value` after provisioning.\n"] pub fn enable_each_unique_value (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.enable_each_unique_value" , self . base)) } # [doc = "Get a reference to the value of field `type_` after provisioning.\n"] pub fn type_ (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.type" , self . base)) } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize)]
pub struct DataComputeSecurityPolicyAdaptiveProtectionConfigElLayer7DdosDefenseConfigElThresholdConfigsEl { # [serde (skip_serializing_if = "Option::is_none")] auto_deploy_confidence_threshold : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] auto_deploy_expiration_sec : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] auto_deploy_impacted_baseline_threshold : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] auto_deploy_load_threshold : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] detection_absolute_qps : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] detection_load_threshold : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] detection_relative_to_baseline_qps : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] name : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] traffic_granularity_configs : Option < ListField < DataComputeSecurityPolicyAdaptiveProtectionConfigElLayer7DdosDefenseConfigElThresholdConfigsElTrafficGranularityConfigsEl > > , }
impl
    DataComputeSecurityPolicyAdaptiveProtectionConfigElLayer7DdosDefenseConfigElThresholdConfigsEl
{
    #[doc = "Set the field `auto_deploy_confidence_threshold`.\n"]
    pub fn set_auto_deploy_confidence_threshold(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.auto_deploy_confidence_threshold = Some(v.into());
        self
    }
    #[doc = "Set the field `auto_deploy_expiration_sec`.\n"]
    pub fn set_auto_deploy_expiration_sec(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.auto_deploy_expiration_sec = Some(v.into());
        self
    }
    #[doc = "Set the field `auto_deploy_impacted_baseline_threshold`.\n"]
    pub fn set_auto_deploy_impacted_baseline_threshold(
        mut self,
        v: impl Into<PrimField<f64>>,
    ) -> Self {
        self.auto_deploy_impacted_baseline_threshold = Some(v.into());
        self
    }
    #[doc = "Set the field `auto_deploy_load_threshold`.\n"]
    pub fn set_auto_deploy_load_threshold(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.auto_deploy_load_threshold = Some(v.into());
        self
    }
    #[doc = "Set the field `detection_absolute_qps`.\n"]
    pub fn set_detection_absolute_qps(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.detection_absolute_qps = Some(v.into());
        self
    }
    #[doc = "Set the field `detection_load_threshold`.\n"]
    pub fn set_detection_load_threshold(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.detection_load_threshold = Some(v.into());
        self
    }
    #[doc = "Set the field `detection_relative_to_baseline_qps`.\n"]
    pub fn set_detection_relative_to_baseline_qps(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.detection_relative_to_baseline_qps = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `traffic_granularity_configs`.\n"]
    pub fn set_traffic_granularity_configs(
        mut self,
        v : impl Into < ListField < DataComputeSecurityPolicyAdaptiveProtectionConfigElLayer7DdosDefenseConfigElThresholdConfigsElTrafficGranularityConfigsEl > >,
    ) -> Self {
        self.traffic_granularity_configs = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeSecurityPolicyAdaptiveProtectionConfigElLayer7DdosDefenseConfigElThresholdConfigsEl { type O = BlockAssignable < DataComputeSecurityPolicyAdaptiveProtectionConfigElLayer7DdosDefenseConfigElThresholdConfigsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataComputeSecurityPolicyAdaptiveProtectionConfigElLayer7DdosDefenseConfigElThresholdConfigsEl
{}
impl BuildDataComputeSecurityPolicyAdaptiveProtectionConfigElLayer7DdosDefenseConfigElThresholdConfigsEl { pub fn build (self) -> DataComputeSecurityPolicyAdaptiveProtectionConfigElLayer7DdosDefenseConfigElThresholdConfigsEl { DataComputeSecurityPolicyAdaptiveProtectionConfigElLayer7DdosDefenseConfigElThresholdConfigsEl { auto_deploy_confidence_threshold : core :: default :: Default :: default () , auto_deploy_expiration_sec : core :: default :: Default :: default () , auto_deploy_impacted_baseline_threshold : core :: default :: Default :: default () , auto_deploy_load_threshold : core :: default :: Default :: default () , detection_absolute_qps : core :: default :: Default :: default () , detection_load_threshold : core :: default :: Default :: default () , detection_relative_to_baseline_qps : core :: default :: Default :: default () , name : core :: default :: Default :: default () , traffic_granularity_configs : core :: default :: Default :: default () , } } }
pub struct DataComputeSecurityPolicyAdaptiveProtectionConfigElLayer7DdosDefenseConfigElThresholdConfigsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeSecurityPolicyAdaptiveProtectionConfigElLayer7DdosDefenseConfigElThresholdConfigsElRef { fn new (shared : StackShared , base : String) -> DataComputeSecurityPolicyAdaptiveProtectionConfigElLayer7DdosDefenseConfigElThresholdConfigsElRef { DataComputeSecurityPolicyAdaptiveProtectionConfigElLayer7DdosDefenseConfigElThresholdConfigsElRef { shared : shared , base : base . to_string () , } } }
impl DataComputeSecurityPolicyAdaptiveProtectionConfigElLayer7DdosDefenseConfigElThresholdConfigsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `auto_deploy_confidence_threshold` after provisioning.\n"] pub fn auto_deploy_confidence_threshold (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.auto_deploy_confidence_threshold" , self . base)) } # [doc = "Get a reference to the value of field `auto_deploy_expiration_sec` after provisioning.\n"] pub fn auto_deploy_expiration_sec (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.auto_deploy_expiration_sec" , self . base)) } # [doc = "Get a reference to the value of field `auto_deploy_impacted_baseline_threshold` after provisioning.\n"] pub fn auto_deploy_impacted_baseline_threshold (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.auto_deploy_impacted_baseline_threshold" , self . base)) } # [doc = "Get a reference to the value of field `auto_deploy_load_threshold` after provisioning.\n"] pub fn auto_deploy_load_threshold (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.auto_deploy_load_threshold" , self . base)) } # [doc = "Get a reference to the value of field `detection_absolute_qps` after provisioning.\n"] pub fn detection_absolute_qps (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.detection_absolute_qps" , self . base)) } # [doc = "Get a reference to the value of field `detection_load_threshold` after provisioning.\n"] pub fn detection_load_threshold (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.detection_load_threshold" , self . base)) } # [doc = "Get a reference to the value of field `detection_relative_to_baseline_qps` after provisioning.\n"] pub fn detection_relative_to_baseline_qps (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.detection_relative_to_baseline_qps" , self . base)) } # [doc = "Get a reference to the value of field `name` after provisioning.\n"] pub fn name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.name" , self . base)) } # [doc = "Get a reference to the value of field `traffic_granularity_configs` after provisioning.\n"] pub fn traffic_granularity_configs (& self) -> ListRef < DataComputeSecurityPolicyAdaptiveProtectionConfigElLayer7DdosDefenseConfigElThresholdConfigsElTrafficGranularityConfigsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.traffic_granularity_configs" , self . base)) } }
#[derive(Serialize)]
pub struct DataComputeSecurityPolicyAdaptiveProtectionConfigElLayer7DdosDefenseConfigEl { # [serde (skip_serializing_if = "Option::is_none")] enable : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] rule_visibility : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] threshold_configs : Option < ListField < DataComputeSecurityPolicyAdaptiveProtectionConfigElLayer7DdosDefenseConfigElThresholdConfigsEl > > , }
impl DataComputeSecurityPolicyAdaptiveProtectionConfigElLayer7DdosDefenseConfigEl {
    #[doc = "Set the field `enable`.\n"]
    pub fn set_enable(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable = Some(v.into());
        self
    }
    #[doc = "Set the field `rule_visibility`.\n"]
    pub fn set_rule_visibility(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.rule_visibility = Some(v.into());
        self
    }
    #[doc = "Set the field `threshold_configs`.\n"]
    pub fn set_threshold_configs(
        mut self,
        v : impl Into < ListField < DataComputeSecurityPolicyAdaptiveProtectionConfigElLayer7DdosDefenseConfigElThresholdConfigsEl > >,
    ) -> Self {
        self.threshold_configs = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataComputeSecurityPolicyAdaptiveProtectionConfigElLayer7DdosDefenseConfigEl
{
    type O = BlockAssignable<
        DataComputeSecurityPolicyAdaptiveProtectionConfigElLayer7DdosDefenseConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeSecurityPolicyAdaptiveProtectionConfigElLayer7DdosDefenseConfigEl {}
impl BuildDataComputeSecurityPolicyAdaptiveProtectionConfigElLayer7DdosDefenseConfigEl {
    pub fn build(
        self,
    ) -> DataComputeSecurityPolicyAdaptiveProtectionConfigElLayer7DdosDefenseConfigEl {
        DataComputeSecurityPolicyAdaptiveProtectionConfigElLayer7DdosDefenseConfigEl {
            enable: core::default::Default::default(),
            rule_visibility: core::default::Default::default(),
            threshold_configs: core::default::Default::default(),
        }
    }
}
pub struct DataComputeSecurityPolicyAdaptiveProtectionConfigElLayer7DdosDefenseConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeSecurityPolicyAdaptiveProtectionConfigElLayer7DdosDefenseConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeSecurityPolicyAdaptiveProtectionConfigElLayer7DdosDefenseConfigElRef {
        DataComputeSecurityPolicyAdaptiveProtectionConfigElLayer7DdosDefenseConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeSecurityPolicyAdaptiveProtectionConfigElLayer7DdosDefenseConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enable` after provisioning.\n"]
    pub fn enable(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable", self.base))
    }
    #[doc = "Get a reference to the value of field `rule_visibility` after provisioning.\n"]
    pub fn rule_visibility(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rule_visibility", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `threshold_configs` after provisioning.\n"]    pub fn threshold_configs (& self) -> ListRef < DataComputeSecurityPolicyAdaptiveProtectionConfigElLayer7DdosDefenseConfigElThresholdConfigsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.threshold_configs", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeSecurityPolicyAdaptiveProtectionConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    layer_7_ddos_defense_config: Option<
        ListField<DataComputeSecurityPolicyAdaptiveProtectionConfigElLayer7DdosDefenseConfigEl>,
    >,
}
impl DataComputeSecurityPolicyAdaptiveProtectionConfigEl {
    #[doc = "Set the field `layer_7_ddos_defense_config`.\n"]
    pub fn set_layer_7_ddos_defense_config(
        mut self,
        v: impl Into<
            ListField<DataComputeSecurityPolicyAdaptiveProtectionConfigElLayer7DdosDefenseConfigEl>,
        >,
    ) -> Self {
        self.layer_7_ddos_defense_config = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeSecurityPolicyAdaptiveProtectionConfigEl {
    type O = BlockAssignable<DataComputeSecurityPolicyAdaptiveProtectionConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeSecurityPolicyAdaptiveProtectionConfigEl {}
impl BuildDataComputeSecurityPolicyAdaptiveProtectionConfigEl {
    pub fn build(self) -> DataComputeSecurityPolicyAdaptiveProtectionConfigEl {
        DataComputeSecurityPolicyAdaptiveProtectionConfigEl {
            layer_7_ddos_defense_config: core::default::Default::default(),
        }
    }
}
pub struct DataComputeSecurityPolicyAdaptiveProtectionConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeSecurityPolicyAdaptiveProtectionConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeSecurityPolicyAdaptiveProtectionConfigElRef {
        DataComputeSecurityPolicyAdaptiveProtectionConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeSecurityPolicyAdaptiveProtectionConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `layer_7_ddos_defense_config` after provisioning.\n"]
    pub fn layer_7_ddos_defense_config(
        &self,
    ) -> ListRef<DataComputeSecurityPolicyAdaptiveProtectionConfigElLayer7DdosDefenseConfigElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.layer_7_ddos_defense_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    content_types: Option<SetField<PrimField<String>>>,
}
impl DataComputeSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigEl {
    #[doc = "Set the field `content_types`.\n"]
    pub fn set_content_types(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.content_types = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigEl {
    type O = BlockAssignable<DataComputeSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigEl {}
impl BuildDataComputeSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigEl {
    pub fn build(self) -> DataComputeSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigEl {
        DataComputeSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigEl {
            content_types: core::default::Default::default(),
        }
    }
}
pub struct DataComputeSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigElRef {
        DataComputeSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `content_types` after provisioning.\n"]
    pub fn content_types(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.content_types", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeSecurityPolicyAdvancedOptionsConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    json_custom_config:
        Option<ListField<DataComputeSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    json_parsing: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_body_inspection_size: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_ip_request_headers: Option<SetField<PrimField<String>>>,
}
impl DataComputeSecurityPolicyAdvancedOptionsConfigEl {
    #[doc = "Set the field `json_custom_config`.\n"]
    pub fn set_json_custom_config(
        mut self,
        v: impl Into<ListField<DataComputeSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigEl>>,
    ) -> Self {
        self.json_custom_config = Some(v.into());
        self
    }
    #[doc = "Set the field `json_parsing`.\n"]
    pub fn set_json_parsing(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.json_parsing = Some(v.into());
        self
    }
    #[doc = "Set the field `log_level`.\n"]
    pub fn set_log_level(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.log_level = Some(v.into());
        self
    }
    #[doc = "Set the field `request_body_inspection_size`.\n"]
    pub fn set_request_body_inspection_size(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.request_body_inspection_size = Some(v.into());
        self
    }
    #[doc = "Set the field `user_ip_request_headers`.\n"]
    pub fn set_user_ip_request_headers(
        mut self,
        v: impl Into<SetField<PrimField<String>>>,
    ) -> Self {
        self.user_ip_request_headers = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeSecurityPolicyAdvancedOptionsConfigEl {
    type O = BlockAssignable<DataComputeSecurityPolicyAdvancedOptionsConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeSecurityPolicyAdvancedOptionsConfigEl {}
impl BuildDataComputeSecurityPolicyAdvancedOptionsConfigEl {
    pub fn build(self) -> DataComputeSecurityPolicyAdvancedOptionsConfigEl {
        DataComputeSecurityPolicyAdvancedOptionsConfigEl {
            json_custom_config: core::default::Default::default(),
            json_parsing: core::default::Default::default(),
            log_level: core::default::Default::default(),
            request_body_inspection_size: core::default::Default::default(),
            user_ip_request_headers: core::default::Default::default(),
        }
    }
}
pub struct DataComputeSecurityPolicyAdvancedOptionsConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeSecurityPolicyAdvancedOptionsConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeSecurityPolicyAdvancedOptionsConfigElRef {
        DataComputeSecurityPolicyAdvancedOptionsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeSecurityPolicyAdvancedOptionsConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `json_custom_config` after provisioning.\n"]
    pub fn json_custom_config(
        &self,
    ) -> ListRef<DataComputeSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.json_custom_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `json_parsing` after provisioning.\n"]
    pub fn json_parsing(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.json_parsing", self.base))
    }
    #[doc = "Get a reference to the value of field `log_level` after provisioning.\n"]
    pub fn log_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_level", self.base))
    }
    #[doc = "Get a reference to the value of field `request_body_inspection_size` after provisioning.\n"]
    pub fn request_body_inspection_size(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.request_body_inspection_size", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `user_ip_request_headers` after provisioning.\n"]
    pub fn user_ip_request_headers(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.user_ip_request_headers", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeSecurityPolicyRecaptchaOptionsConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    redirect_site_key: Option<PrimField<String>>,
}
impl DataComputeSecurityPolicyRecaptchaOptionsConfigEl {
    #[doc = "Set the field `redirect_site_key`.\n"]
    pub fn set_redirect_site_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.redirect_site_key = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeSecurityPolicyRecaptchaOptionsConfigEl {
    type O = BlockAssignable<DataComputeSecurityPolicyRecaptchaOptionsConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeSecurityPolicyRecaptchaOptionsConfigEl {}
impl BuildDataComputeSecurityPolicyRecaptchaOptionsConfigEl {
    pub fn build(self) -> DataComputeSecurityPolicyRecaptchaOptionsConfigEl {
        DataComputeSecurityPolicyRecaptchaOptionsConfigEl {
            redirect_site_key: core::default::Default::default(),
        }
    }
}
pub struct DataComputeSecurityPolicyRecaptchaOptionsConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeSecurityPolicyRecaptchaOptionsConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeSecurityPolicyRecaptchaOptionsConfigElRef {
        DataComputeSecurityPolicyRecaptchaOptionsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeSecurityPolicyRecaptchaOptionsConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `redirect_site_key` after provisioning.\n"]
    pub fn redirect_site_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.redirect_site_key", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeSecurityPolicyRuleElHeaderActionElRequestHeadersToAddsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    header_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    header_value: Option<PrimField<String>>,
}
impl DataComputeSecurityPolicyRuleElHeaderActionElRequestHeadersToAddsEl {
    #[doc = "Set the field `header_name`.\n"]
    pub fn set_header_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.header_name = Some(v.into());
        self
    }
    #[doc = "Set the field `header_value`.\n"]
    pub fn set_header_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.header_value = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeSecurityPolicyRuleElHeaderActionElRequestHeadersToAddsEl {
    type O = BlockAssignable<DataComputeSecurityPolicyRuleElHeaderActionElRequestHeadersToAddsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeSecurityPolicyRuleElHeaderActionElRequestHeadersToAddsEl {}
impl BuildDataComputeSecurityPolicyRuleElHeaderActionElRequestHeadersToAddsEl {
    pub fn build(self) -> DataComputeSecurityPolicyRuleElHeaderActionElRequestHeadersToAddsEl {
        DataComputeSecurityPolicyRuleElHeaderActionElRequestHeadersToAddsEl {
            header_name: core::default::Default::default(),
            header_value: core::default::Default::default(),
        }
    }
}
pub struct DataComputeSecurityPolicyRuleElHeaderActionElRequestHeadersToAddsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeSecurityPolicyRuleElHeaderActionElRequestHeadersToAddsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeSecurityPolicyRuleElHeaderActionElRequestHeadersToAddsElRef {
        DataComputeSecurityPolicyRuleElHeaderActionElRequestHeadersToAddsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeSecurityPolicyRuleElHeaderActionElRequestHeadersToAddsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `header_name` after provisioning.\n"]
    pub fn header_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.header_name", self.base))
    }
    #[doc = "Get a reference to the value of field `header_value` after provisioning.\n"]
    pub fn header_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.header_value", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeSecurityPolicyRuleElHeaderActionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    request_headers_to_adds:
        Option<ListField<DataComputeSecurityPolicyRuleElHeaderActionElRequestHeadersToAddsEl>>,
}
impl DataComputeSecurityPolicyRuleElHeaderActionEl {
    #[doc = "Set the field `request_headers_to_adds`.\n"]
    pub fn set_request_headers_to_adds(
        mut self,
        v: impl Into<ListField<DataComputeSecurityPolicyRuleElHeaderActionElRequestHeadersToAddsEl>>,
    ) -> Self {
        self.request_headers_to_adds = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeSecurityPolicyRuleElHeaderActionEl {
    type O = BlockAssignable<DataComputeSecurityPolicyRuleElHeaderActionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeSecurityPolicyRuleElHeaderActionEl {}
impl BuildDataComputeSecurityPolicyRuleElHeaderActionEl {
    pub fn build(self) -> DataComputeSecurityPolicyRuleElHeaderActionEl {
        DataComputeSecurityPolicyRuleElHeaderActionEl {
            request_headers_to_adds: core::default::Default::default(),
        }
    }
}
pub struct DataComputeSecurityPolicyRuleElHeaderActionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeSecurityPolicyRuleElHeaderActionElRef {
    fn new(shared: StackShared, base: String) -> DataComputeSecurityPolicyRuleElHeaderActionElRef {
        DataComputeSecurityPolicyRuleElHeaderActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeSecurityPolicyRuleElHeaderActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `request_headers_to_adds` after provisioning.\n"]
    pub fn request_headers_to_adds(
        &self,
    ) -> ListRef<DataComputeSecurityPolicyRuleElHeaderActionElRequestHeadersToAddsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.request_headers_to_adds", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeSecurityPolicyRuleElMatchElConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    src_ip_ranges: Option<SetField<PrimField<String>>>,
}
impl DataComputeSecurityPolicyRuleElMatchElConfigEl {
    #[doc = "Set the field `src_ip_ranges`.\n"]
    pub fn set_src_ip_ranges(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.src_ip_ranges = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeSecurityPolicyRuleElMatchElConfigEl {
    type O = BlockAssignable<DataComputeSecurityPolicyRuleElMatchElConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeSecurityPolicyRuleElMatchElConfigEl {}
impl BuildDataComputeSecurityPolicyRuleElMatchElConfigEl {
    pub fn build(self) -> DataComputeSecurityPolicyRuleElMatchElConfigEl {
        DataComputeSecurityPolicyRuleElMatchElConfigEl {
            src_ip_ranges: core::default::Default::default(),
        }
    }
}
pub struct DataComputeSecurityPolicyRuleElMatchElConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeSecurityPolicyRuleElMatchElConfigElRef {
    fn new(shared: StackShared, base: String) -> DataComputeSecurityPolicyRuleElMatchElConfigElRef {
        DataComputeSecurityPolicyRuleElMatchElConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeSecurityPolicyRuleElMatchElConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `src_ip_ranges` after provisioning.\n"]
    pub fn src_ip_ranges(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.src_ip_ranges", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeSecurityPolicyRuleElMatchElExprEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    expression: Option<PrimField<String>>,
}
impl DataComputeSecurityPolicyRuleElMatchElExprEl {
    #[doc = "Set the field `expression`.\n"]
    pub fn set_expression(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.expression = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeSecurityPolicyRuleElMatchElExprEl {
    type O = BlockAssignable<DataComputeSecurityPolicyRuleElMatchElExprEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeSecurityPolicyRuleElMatchElExprEl {}
impl BuildDataComputeSecurityPolicyRuleElMatchElExprEl {
    pub fn build(self) -> DataComputeSecurityPolicyRuleElMatchElExprEl {
        DataComputeSecurityPolicyRuleElMatchElExprEl {
            expression: core::default::Default::default(),
        }
    }
}
pub struct DataComputeSecurityPolicyRuleElMatchElExprElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeSecurityPolicyRuleElMatchElExprElRef {
    fn new(shared: StackShared, base: String) -> DataComputeSecurityPolicyRuleElMatchElExprElRef {
        DataComputeSecurityPolicyRuleElMatchElExprElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeSecurityPolicyRuleElMatchElExprElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `expression` after provisioning.\n"]
    pub fn expression(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeSecurityPolicyRuleElMatchElExprOptionsElRecaptchaOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    action_token_site_keys: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    session_token_site_keys: Option<ListField<PrimField<String>>>,
}
impl DataComputeSecurityPolicyRuleElMatchElExprOptionsElRecaptchaOptionsEl {
    #[doc = "Set the field `action_token_site_keys`.\n"]
    pub fn set_action_token_site_keys(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.action_token_site_keys = Some(v.into());
        self
    }
    #[doc = "Set the field `session_token_site_keys`.\n"]
    pub fn set_session_token_site_keys(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.session_token_site_keys = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeSecurityPolicyRuleElMatchElExprOptionsElRecaptchaOptionsEl {
    type O = BlockAssignable<DataComputeSecurityPolicyRuleElMatchElExprOptionsElRecaptchaOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeSecurityPolicyRuleElMatchElExprOptionsElRecaptchaOptionsEl {}
impl BuildDataComputeSecurityPolicyRuleElMatchElExprOptionsElRecaptchaOptionsEl {
    pub fn build(self) -> DataComputeSecurityPolicyRuleElMatchElExprOptionsElRecaptchaOptionsEl {
        DataComputeSecurityPolicyRuleElMatchElExprOptionsElRecaptchaOptionsEl {
            action_token_site_keys: core::default::Default::default(),
            session_token_site_keys: core::default::Default::default(),
        }
    }
}
pub struct DataComputeSecurityPolicyRuleElMatchElExprOptionsElRecaptchaOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeSecurityPolicyRuleElMatchElExprOptionsElRecaptchaOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeSecurityPolicyRuleElMatchElExprOptionsElRecaptchaOptionsElRef {
        DataComputeSecurityPolicyRuleElMatchElExprOptionsElRecaptchaOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeSecurityPolicyRuleElMatchElExprOptionsElRecaptchaOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `action_token_site_keys` after provisioning.\n"]
    pub fn action_token_site_keys(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.action_token_site_keys", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `session_token_site_keys` after provisioning.\n"]
    pub fn session_token_site_keys(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.session_token_site_keys", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeSecurityPolicyRuleElMatchElExprOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    recaptcha_options:
        Option<ListField<DataComputeSecurityPolicyRuleElMatchElExprOptionsElRecaptchaOptionsEl>>,
}
impl DataComputeSecurityPolicyRuleElMatchElExprOptionsEl {
    #[doc = "Set the field `recaptcha_options`.\n"]
    pub fn set_recaptcha_options(
        mut self,
        v: impl Into<ListField<DataComputeSecurityPolicyRuleElMatchElExprOptionsElRecaptchaOptionsEl>>,
    ) -> Self {
        self.recaptcha_options = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeSecurityPolicyRuleElMatchElExprOptionsEl {
    type O = BlockAssignable<DataComputeSecurityPolicyRuleElMatchElExprOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeSecurityPolicyRuleElMatchElExprOptionsEl {}
impl BuildDataComputeSecurityPolicyRuleElMatchElExprOptionsEl {
    pub fn build(self) -> DataComputeSecurityPolicyRuleElMatchElExprOptionsEl {
        DataComputeSecurityPolicyRuleElMatchElExprOptionsEl {
            recaptcha_options: core::default::Default::default(),
        }
    }
}
pub struct DataComputeSecurityPolicyRuleElMatchElExprOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeSecurityPolicyRuleElMatchElExprOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeSecurityPolicyRuleElMatchElExprOptionsElRef {
        DataComputeSecurityPolicyRuleElMatchElExprOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeSecurityPolicyRuleElMatchElExprOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `recaptcha_options` after provisioning.\n"]
    pub fn recaptcha_options(
        &self,
    ) -> ListRef<DataComputeSecurityPolicyRuleElMatchElExprOptionsElRecaptchaOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.recaptcha_options", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeSecurityPolicyRuleElMatchEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    config: Option<ListField<DataComputeSecurityPolicyRuleElMatchElConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expr: Option<ListField<DataComputeSecurityPolicyRuleElMatchElExprEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expr_options: Option<ListField<DataComputeSecurityPolicyRuleElMatchElExprOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    versioned_expr: Option<PrimField<String>>,
}
impl DataComputeSecurityPolicyRuleElMatchEl {
    #[doc = "Set the field `config`.\n"]
    pub fn set_config(
        mut self,
        v: impl Into<ListField<DataComputeSecurityPolicyRuleElMatchElConfigEl>>,
    ) -> Self {
        self.config = Some(v.into());
        self
    }
    #[doc = "Set the field `expr`.\n"]
    pub fn set_expr(
        mut self,
        v: impl Into<ListField<DataComputeSecurityPolicyRuleElMatchElExprEl>>,
    ) -> Self {
        self.expr = Some(v.into());
        self
    }
    #[doc = "Set the field `expr_options`.\n"]
    pub fn set_expr_options(
        mut self,
        v: impl Into<ListField<DataComputeSecurityPolicyRuleElMatchElExprOptionsEl>>,
    ) -> Self {
        self.expr_options = Some(v.into());
        self
    }
    #[doc = "Set the field `versioned_expr`.\n"]
    pub fn set_versioned_expr(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.versioned_expr = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeSecurityPolicyRuleElMatchEl {
    type O = BlockAssignable<DataComputeSecurityPolicyRuleElMatchEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeSecurityPolicyRuleElMatchEl {}
impl BuildDataComputeSecurityPolicyRuleElMatchEl {
    pub fn build(self) -> DataComputeSecurityPolicyRuleElMatchEl {
        DataComputeSecurityPolicyRuleElMatchEl {
            config: core::default::Default::default(),
            expr: core::default::Default::default(),
            expr_options: core::default::Default::default(),
            versioned_expr: core::default::Default::default(),
        }
    }
}
pub struct DataComputeSecurityPolicyRuleElMatchElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeSecurityPolicyRuleElMatchElRef {
    fn new(shared: StackShared, base: String) -> DataComputeSecurityPolicyRuleElMatchElRef {
        DataComputeSecurityPolicyRuleElMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeSecurityPolicyRuleElMatchElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(&self) -> ListRef<DataComputeSecurityPolicyRuleElMatchElConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.config", self.base))
    }
    #[doc = "Get a reference to the value of field `expr` after provisioning.\n"]
    pub fn expr(&self) -> ListRef<DataComputeSecurityPolicyRuleElMatchElExprElRef> {
        ListRef::new(self.shared().clone(), format!("{}.expr", self.base))
    }
    #[doc = "Get a reference to the value of field `expr_options` after provisioning.\n"]
    pub fn expr_options(&self) -> ListRef<DataComputeSecurityPolicyRuleElMatchElExprOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.expr_options", self.base))
    }
    #[doc = "Get a reference to the value of field `versioned_expr` after provisioning.\n"]
    pub fn versioned_expr(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.versioned_expr", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeSecurityPolicyRuleElPreconfiguredWafConfigElExclusionElRequestCookieEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    operator: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl DataComputeSecurityPolicyRuleElPreconfiguredWafConfigElExclusionElRequestCookieEl {
    #[doc = "Set the field `operator`.\n"]
    pub fn set_operator(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.operator = Some(v.into());
        self
    }
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataComputeSecurityPolicyRuleElPreconfiguredWafConfigElExclusionElRequestCookieEl
{
    type O = BlockAssignable<
        DataComputeSecurityPolicyRuleElPreconfiguredWafConfigElExclusionElRequestCookieEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeSecurityPolicyRuleElPreconfiguredWafConfigElExclusionElRequestCookieEl {
}
impl BuildDataComputeSecurityPolicyRuleElPreconfiguredWafConfigElExclusionElRequestCookieEl {
    pub fn build(
        self,
    ) -> DataComputeSecurityPolicyRuleElPreconfiguredWafConfigElExclusionElRequestCookieEl {
        DataComputeSecurityPolicyRuleElPreconfiguredWafConfigElExclusionElRequestCookieEl {
            operator: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}
pub struct DataComputeSecurityPolicyRuleElPreconfiguredWafConfigElExclusionElRequestCookieElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeSecurityPolicyRuleElPreconfiguredWafConfigElExclusionElRequestCookieElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeSecurityPolicyRuleElPreconfiguredWafConfigElExclusionElRequestCookieElRef {
        DataComputeSecurityPolicyRuleElPreconfiguredWafConfigElExclusionElRequestCookieElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeSecurityPolicyRuleElPreconfiguredWafConfigElExclusionElRequestCookieElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `operator` after provisioning.\n"]
    pub fn operator(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeSecurityPolicyRuleElPreconfiguredWafConfigElExclusionElRequestHeaderEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    operator: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl DataComputeSecurityPolicyRuleElPreconfiguredWafConfigElExclusionElRequestHeaderEl {
    #[doc = "Set the field `operator`.\n"]
    pub fn set_operator(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.operator = Some(v.into());
        self
    }
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataComputeSecurityPolicyRuleElPreconfiguredWafConfigElExclusionElRequestHeaderEl
{
    type O = BlockAssignable<
        DataComputeSecurityPolicyRuleElPreconfiguredWafConfigElExclusionElRequestHeaderEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeSecurityPolicyRuleElPreconfiguredWafConfigElExclusionElRequestHeaderEl {
}
impl BuildDataComputeSecurityPolicyRuleElPreconfiguredWafConfigElExclusionElRequestHeaderEl {
    pub fn build(
        self,
    ) -> DataComputeSecurityPolicyRuleElPreconfiguredWafConfigElExclusionElRequestHeaderEl {
        DataComputeSecurityPolicyRuleElPreconfiguredWafConfigElExclusionElRequestHeaderEl {
            operator: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}
pub struct DataComputeSecurityPolicyRuleElPreconfiguredWafConfigElExclusionElRequestHeaderElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeSecurityPolicyRuleElPreconfiguredWafConfigElExclusionElRequestHeaderElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeSecurityPolicyRuleElPreconfiguredWafConfigElExclusionElRequestHeaderElRef {
        DataComputeSecurityPolicyRuleElPreconfiguredWafConfigElExclusionElRequestHeaderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeSecurityPolicyRuleElPreconfiguredWafConfigElExclusionElRequestHeaderElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `operator` after provisioning.\n"]
    pub fn operator(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeSecurityPolicyRuleElPreconfiguredWafConfigElExclusionElRequestQueryParamEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    operator: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl DataComputeSecurityPolicyRuleElPreconfiguredWafConfigElExclusionElRequestQueryParamEl {
    #[doc = "Set the field `operator`.\n"]
    pub fn set_operator(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.operator = Some(v.into());
        self
    }
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataComputeSecurityPolicyRuleElPreconfiguredWafConfigElExclusionElRequestQueryParamEl
{
    type O = BlockAssignable<
        DataComputeSecurityPolicyRuleElPreconfiguredWafConfigElExclusionElRequestQueryParamEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeSecurityPolicyRuleElPreconfiguredWafConfigElExclusionElRequestQueryParamEl
{}
impl BuildDataComputeSecurityPolicyRuleElPreconfiguredWafConfigElExclusionElRequestQueryParamEl {
    pub fn build(
        self,
    ) -> DataComputeSecurityPolicyRuleElPreconfiguredWafConfigElExclusionElRequestQueryParamEl {
        DataComputeSecurityPolicyRuleElPreconfiguredWafConfigElExclusionElRequestQueryParamEl {
            operator: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}
pub struct DataComputeSecurityPolicyRuleElPreconfiguredWafConfigElExclusionElRequestQueryParamElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for DataComputeSecurityPolicyRuleElPreconfiguredWafConfigElExclusionElRequestQueryParamElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeSecurityPolicyRuleElPreconfiguredWafConfigElExclusionElRequestQueryParamElRef
    {
        DataComputeSecurityPolicyRuleElPreconfiguredWafConfigElExclusionElRequestQueryParamElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeSecurityPolicyRuleElPreconfiguredWafConfigElExclusionElRequestQueryParamElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `operator` after provisioning.\n"]
    pub fn operator(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeSecurityPolicyRuleElPreconfiguredWafConfigElExclusionElRequestUriEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    operator: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl DataComputeSecurityPolicyRuleElPreconfiguredWafConfigElExclusionElRequestUriEl {
    #[doc = "Set the field `operator`.\n"]
    pub fn set_operator(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.operator = Some(v.into());
        self
    }
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataComputeSecurityPolicyRuleElPreconfiguredWafConfigElExclusionElRequestUriEl
{
    type O = BlockAssignable<
        DataComputeSecurityPolicyRuleElPreconfiguredWafConfigElExclusionElRequestUriEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeSecurityPolicyRuleElPreconfiguredWafConfigElExclusionElRequestUriEl {}
impl BuildDataComputeSecurityPolicyRuleElPreconfiguredWafConfigElExclusionElRequestUriEl {
    pub fn build(
        self,
    ) -> DataComputeSecurityPolicyRuleElPreconfiguredWafConfigElExclusionElRequestUriEl {
        DataComputeSecurityPolicyRuleElPreconfiguredWafConfigElExclusionElRequestUriEl {
            operator: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}
pub struct DataComputeSecurityPolicyRuleElPreconfiguredWafConfigElExclusionElRequestUriElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeSecurityPolicyRuleElPreconfiguredWafConfigElExclusionElRequestUriElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeSecurityPolicyRuleElPreconfiguredWafConfigElExclusionElRequestUriElRef {
        DataComputeSecurityPolicyRuleElPreconfiguredWafConfigElExclusionElRequestUriElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeSecurityPolicyRuleElPreconfiguredWafConfigElExclusionElRequestUriElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `operator` after provisioning.\n"]
    pub fn operator(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeSecurityPolicyRuleElPreconfiguredWafConfigElExclusionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    request_cookie: Option<
        ListField<
            DataComputeSecurityPolicyRuleElPreconfiguredWafConfigElExclusionElRequestCookieEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_header: Option<
        ListField<
            DataComputeSecurityPolicyRuleElPreconfiguredWafConfigElExclusionElRequestHeaderEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_query_param: Option<
        ListField<
            DataComputeSecurityPolicyRuleElPreconfiguredWafConfigElExclusionElRequestQueryParamEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_uri: Option<
        ListField<DataComputeSecurityPolicyRuleElPreconfiguredWafConfigElExclusionElRequestUriEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_rule_ids: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_rule_set: Option<PrimField<String>>,
}
impl DataComputeSecurityPolicyRuleElPreconfiguredWafConfigElExclusionEl {
    #[doc = "Set the field `request_cookie`.\n"]
    pub fn set_request_cookie(
        mut self,
        v: impl Into<
            ListField<
                DataComputeSecurityPolicyRuleElPreconfiguredWafConfigElExclusionElRequestCookieEl,
            >,
        >,
    ) -> Self {
        self.request_cookie = Some(v.into());
        self
    }
    #[doc = "Set the field `request_header`.\n"]
    pub fn set_request_header(
        mut self,
        v: impl Into<
            ListField<
                DataComputeSecurityPolicyRuleElPreconfiguredWafConfigElExclusionElRequestHeaderEl,
            >,
        >,
    ) -> Self {
        self.request_header = Some(v.into());
        self
    }
    #[doc = "Set the field `request_query_param`.\n"]
    pub fn set_request_query_param(
        mut self,
        v : impl Into < ListField < DataComputeSecurityPolicyRuleElPreconfiguredWafConfigElExclusionElRequestQueryParamEl > >,
    ) -> Self {
        self.request_query_param = Some(v.into());
        self
    }
    #[doc = "Set the field `request_uri`.\n"]
    pub fn set_request_uri(
        mut self,
        v: impl Into<
            ListField<
                DataComputeSecurityPolicyRuleElPreconfiguredWafConfigElExclusionElRequestUriEl,
            >,
        >,
    ) -> Self {
        self.request_uri = Some(v.into());
        self
    }
    #[doc = "Set the field `target_rule_ids`.\n"]
    pub fn set_target_rule_ids(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.target_rule_ids = Some(v.into());
        self
    }
    #[doc = "Set the field `target_rule_set`.\n"]
    pub fn set_target_rule_set(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.target_rule_set = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeSecurityPolicyRuleElPreconfiguredWafConfigElExclusionEl {
    type O = BlockAssignable<DataComputeSecurityPolicyRuleElPreconfiguredWafConfigElExclusionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeSecurityPolicyRuleElPreconfiguredWafConfigElExclusionEl {}
impl BuildDataComputeSecurityPolicyRuleElPreconfiguredWafConfigElExclusionEl {
    pub fn build(self) -> DataComputeSecurityPolicyRuleElPreconfiguredWafConfigElExclusionEl {
        DataComputeSecurityPolicyRuleElPreconfiguredWafConfigElExclusionEl {
            request_cookie: core::default::Default::default(),
            request_header: core::default::Default::default(),
            request_query_param: core::default::Default::default(),
            request_uri: core::default::Default::default(),
            target_rule_ids: core::default::Default::default(),
            target_rule_set: core::default::Default::default(),
        }
    }
}
pub struct DataComputeSecurityPolicyRuleElPreconfiguredWafConfigElExclusionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeSecurityPolicyRuleElPreconfiguredWafConfigElExclusionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeSecurityPolicyRuleElPreconfiguredWafConfigElExclusionElRef {
        DataComputeSecurityPolicyRuleElPreconfiguredWafConfigElExclusionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeSecurityPolicyRuleElPreconfiguredWafConfigElExclusionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `request_cookie` after provisioning.\n"]
    pub fn request_cookie(
        &self,
    ) -> ListRef<DataComputeSecurityPolicyRuleElPreconfiguredWafConfigElExclusionElRequestCookieElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.request_cookie", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `request_header` after provisioning.\n"]
    pub fn request_header(
        &self,
    ) -> ListRef<DataComputeSecurityPolicyRuleElPreconfiguredWafConfigElExclusionElRequestHeaderElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.request_header", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `request_query_param` after provisioning.\n"]
    pub fn request_query_param(
        &self,
    ) -> ListRef<
        DataComputeSecurityPolicyRuleElPreconfiguredWafConfigElExclusionElRequestQueryParamElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.request_query_param", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `request_uri` after provisioning.\n"]
    pub fn request_uri(
        &self,
    ) -> ListRef<DataComputeSecurityPolicyRuleElPreconfiguredWafConfigElExclusionElRequestUriElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.request_uri", self.base))
    }
    #[doc = "Get a reference to the value of field `target_rule_ids` after provisioning.\n"]
    pub fn target_rule_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.target_rule_ids", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `target_rule_set` after provisioning.\n"]
    pub fn target_rule_set(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_rule_set", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeSecurityPolicyRuleElPreconfiguredWafConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    exclusion:
        Option<ListField<DataComputeSecurityPolicyRuleElPreconfiguredWafConfigElExclusionEl>>,
}
impl DataComputeSecurityPolicyRuleElPreconfiguredWafConfigEl {
    #[doc = "Set the field `exclusion`.\n"]
    pub fn set_exclusion(
        mut self,
        v: impl Into<ListField<DataComputeSecurityPolicyRuleElPreconfiguredWafConfigElExclusionEl>>,
    ) -> Self {
        self.exclusion = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeSecurityPolicyRuleElPreconfiguredWafConfigEl {
    type O = BlockAssignable<DataComputeSecurityPolicyRuleElPreconfiguredWafConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeSecurityPolicyRuleElPreconfiguredWafConfigEl {}
impl BuildDataComputeSecurityPolicyRuleElPreconfiguredWafConfigEl {
    pub fn build(self) -> DataComputeSecurityPolicyRuleElPreconfiguredWafConfigEl {
        DataComputeSecurityPolicyRuleElPreconfiguredWafConfigEl {
            exclusion: core::default::Default::default(),
        }
    }
}
pub struct DataComputeSecurityPolicyRuleElPreconfiguredWafConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeSecurityPolicyRuleElPreconfiguredWafConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeSecurityPolicyRuleElPreconfiguredWafConfigElRef {
        DataComputeSecurityPolicyRuleElPreconfiguredWafConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeSecurityPolicyRuleElPreconfiguredWafConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `exclusion` after provisioning.\n"]
    pub fn exclusion(
        &self,
    ) -> ListRef<DataComputeSecurityPolicyRuleElPreconfiguredWafConfigElExclusionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.exclusion", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeSecurityPolicyRuleElRateLimitOptionsElBanThresholdEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    interval_sec: Option<PrimField<f64>>,
}
impl DataComputeSecurityPolicyRuleElRateLimitOptionsElBanThresholdEl {
    #[doc = "Set the field `count`.\n"]
    pub fn set_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.count = Some(v.into());
        self
    }
    #[doc = "Set the field `interval_sec`.\n"]
    pub fn set_interval_sec(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.interval_sec = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeSecurityPolicyRuleElRateLimitOptionsElBanThresholdEl {
    type O = BlockAssignable<DataComputeSecurityPolicyRuleElRateLimitOptionsElBanThresholdEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeSecurityPolicyRuleElRateLimitOptionsElBanThresholdEl {}
impl BuildDataComputeSecurityPolicyRuleElRateLimitOptionsElBanThresholdEl {
    pub fn build(self) -> DataComputeSecurityPolicyRuleElRateLimitOptionsElBanThresholdEl {
        DataComputeSecurityPolicyRuleElRateLimitOptionsElBanThresholdEl {
            count: core::default::Default::default(),
            interval_sec: core::default::Default::default(),
        }
    }
}
pub struct DataComputeSecurityPolicyRuleElRateLimitOptionsElBanThresholdElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeSecurityPolicyRuleElRateLimitOptionsElBanThresholdElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeSecurityPolicyRuleElRateLimitOptionsElBanThresholdElRef {
        DataComputeSecurityPolicyRuleElRateLimitOptionsElBanThresholdElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeSecurityPolicyRuleElRateLimitOptionsElBanThresholdElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `count` after provisioning.\n"]
    pub fn count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }
    #[doc = "Get a reference to the value of field `interval_sec` after provisioning.\n"]
    pub fn interval_sec(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.interval_sec", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeSecurityPolicyRuleElRateLimitOptionsElEnforceOnKeyConfigsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enforce_on_key_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enforce_on_key_type: Option<PrimField<String>>,
}
impl DataComputeSecurityPolicyRuleElRateLimitOptionsElEnforceOnKeyConfigsEl {
    #[doc = "Set the field `enforce_on_key_name`.\n"]
    pub fn set_enforce_on_key_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.enforce_on_key_name = Some(v.into());
        self
    }
    #[doc = "Set the field `enforce_on_key_type`.\n"]
    pub fn set_enforce_on_key_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.enforce_on_key_type = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeSecurityPolicyRuleElRateLimitOptionsElEnforceOnKeyConfigsEl {
    type O =
        BlockAssignable<DataComputeSecurityPolicyRuleElRateLimitOptionsElEnforceOnKeyConfigsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeSecurityPolicyRuleElRateLimitOptionsElEnforceOnKeyConfigsEl {}
impl BuildDataComputeSecurityPolicyRuleElRateLimitOptionsElEnforceOnKeyConfigsEl {
    pub fn build(self) -> DataComputeSecurityPolicyRuleElRateLimitOptionsElEnforceOnKeyConfigsEl {
        DataComputeSecurityPolicyRuleElRateLimitOptionsElEnforceOnKeyConfigsEl {
            enforce_on_key_name: core::default::Default::default(),
            enforce_on_key_type: core::default::Default::default(),
        }
    }
}
pub struct DataComputeSecurityPolicyRuleElRateLimitOptionsElEnforceOnKeyConfigsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeSecurityPolicyRuleElRateLimitOptionsElEnforceOnKeyConfigsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeSecurityPolicyRuleElRateLimitOptionsElEnforceOnKeyConfigsElRef {
        DataComputeSecurityPolicyRuleElRateLimitOptionsElEnforceOnKeyConfigsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeSecurityPolicyRuleElRateLimitOptionsElEnforceOnKeyConfigsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enforce_on_key_name` after provisioning.\n"]
    pub fn enforce_on_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enforce_on_key_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enforce_on_key_type` after provisioning.\n"]
    pub fn enforce_on_key_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enforce_on_key_type", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeSecurityPolicyRuleElRateLimitOptionsElExceedRedirectOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    target: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl DataComputeSecurityPolicyRuleElRateLimitOptionsElExceedRedirectOptionsEl {
    #[doc = "Set the field `target`.\n"]
    pub fn set_target(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.target = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeSecurityPolicyRuleElRateLimitOptionsElExceedRedirectOptionsEl {
    type O =
        BlockAssignable<DataComputeSecurityPolicyRuleElRateLimitOptionsElExceedRedirectOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeSecurityPolicyRuleElRateLimitOptionsElExceedRedirectOptionsEl {}
impl BuildDataComputeSecurityPolicyRuleElRateLimitOptionsElExceedRedirectOptionsEl {
    pub fn build(self) -> DataComputeSecurityPolicyRuleElRateLimitOptionsElExceedRedirectOptionsEl {
        DataComputeSecurityPolicyRuleElRateLimitOptionsElExceedRedirectOptionsEl {
            target: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}
pub struct DataComputeSecurityPolicyRuleElRateLimitOptionsElExceedRedirectOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeSecurityPolicyRuleElRateLimitOptionsElExceedRedirectOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeSecurityPolicyRuleElRateLimitOptionsElExceedRedirectOptionsElRef {
        DataComputeSecurityPolicyRuleElRateLimitOptionsElExceedRedirectOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeSecurityPolicyRuleElRateLimitOptionsElExceedRedirectOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeSecurityPolicyRuleElRateLimitOptionsElRateLimitThresholdEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    interval_sec: Option<PrimField<f64>>,
}
impl DataComputeSecurityPolicyRuleElRateLimitOptionsElRateLimitThresholdEl {
    #[doc = "Set the field `count`.\n"]
    pub fn set_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.count = Some(v.into());
        self
    }
    #[doc = "Set the field `interval_sec`.\n"]
    pub fn set_interval_sec(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.interval_sec = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeSecurityPolicyRuleElRateLimitOptionsElRateLimitThresholdEl {
    type O = BlockAssignable<DataComputeSecurityPolicyRuleElRateLimitOptionsElRateLimitThresholdEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeSecurityPolicyRuleElRateLimitOptionsElRateLimitThresholdEl {}
impl BuildDataComputeSecurityPolicyRuleElRateLimitOptionsElRateLimitThresholdEl {
    pub fn build(self) -> DataComputeSecurityPolicyRuleElRateLimitOptionsElRateLimitThresholdEl {
        DataComputeSecurityPolicyRuleElRateLimitOptionsElRateLimitThresholdEl {
            count: core::default::Default::default(),
            interval_sec: core::default::Default::default(),
        }
    }
}
pub struct DataComputeSecurityPolicyRuleElRateLimitOptionsElRateLimitThresholdElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeSecurityPolicyRuleElRateLimitOptionsElRateLimitThresholdElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeSecurityPolicyRuleElRateLimitOptionsElRateLimitThresholdElRef {
        DataComputeSecurityPolicyRuleElRateLimitOptionsElRateLimitThresholdElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeSecurityPolicyRuleElRateLimitOptionsElRateLimitThresholdElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `count` after provisioning.\n"]
    pub fn count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }
    #[doc = "Get a reference to the value of field `interval_sec` after provisioning.\n"]
    pub fn interval_sec(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.interval_sec", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeSecurityPolicyRuleElRateLimitOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ban_duration_sec: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ban_threshold:
        Option<ListField<DataComputeSecurityPolicyRuleElRateLimitOptionsElBanThresholdEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    conform_action: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enforce_on_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enforce_on_key_configs:
        Option<ListField<DataComputeSecurityPolicyRuleElRateLimitOptionsElEnforceOnKeyConfigsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enforce_on_key_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exceed_action: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exceed_redirect_options:
        Option<ListField<DataComputeSecurityPolicyRuleElRateLimitOptionsElExceedRedirectOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rate_limit_threshold:
        Option<ListField<DataComputeSecurityPolicyRuleElRateLimitOptionsElRateLimitThresholdEl>>,
}
impl DataComputeSecurityPolicyRuleElRateLimitOptionsEl {
    #[doc = "Set the field `ban_duration_sec`.\n"]
    pub fn set_ban_duration_sec(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.ban_duration_sec = Some(v.into());
        self
    }
    #[doc = "Set the field `ban_threshold`.\n"]
    pub fn set_ban_threshold(
        mut self,
        v: impl Into<ListField<DataComputeSecurityPolicyRuleElRateLimitOptionsElBanThresholdEl>>,
    ) -> Self {
        self.ban_threshold = Some(v.into());
        self
    }
    #[doc = "Set the field `conform_action`.\n"]
    pub fn set_conform_action(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.conform_action = Some(v.into());
        self
    }
    #[doc = "Set the field `enforce_on_key`.\n"]
    pub fn set_enforce_on_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.enforce_on_key = Some(v.into());
        self
    }
    #[doc = "Set the field `enforce_on_key_configs`.\n"]
    pub fn set_enforce_on_key_configs(
        mut self,
        v: impl Into<ListField<DataComputeSecurityPolicyRuleElRateLimitOptionsElEnforceOnKeyConfigsEl>>,
    ) -> Self {
        self.enforce_on_key_configs = Some(v.into());
        self
    }
    #[doc = "Set the field `enforce_on_key_name`.\n"]
    pub fn set_enforce_on_key_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.enforce_on_key_name = Some(v.into());
        self
    }
    #[doc = "Set the field `exceed_action`.\n"]
    pub fn set_exceed_action(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.exceed_action = Some(v.into());
        self
    }
    #[doc = "Set the field `exceed_redirect_options`.\n"]
    pub fn set_exceed_redirect_options(
        mut self,
        v: impl Into<
            ListField<DataComputeSecurityPolicyRuleElRateLimitOptionsElExceedRedirectOptionsEl>,
        >,
    ) -> Self {
        self.exceed_redirect_options = Some(v.into());
        self
    }
    #[doc = "Set the field `rate_limit_threshold`.\n"]
    pub fn set_rate_limit_threshold(
        mut self,
        v: impl Into<ListField<DataComputeSecurityPolicyRuleElRateLimitOptionsElRateLimitThresholdEl>>,
    ) -> Self {
        self.rate_limit_threshold = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeSecurityPolicyRuleElRateLimitOptionsEl {
    type O = BlockAssignable<DataComputeSecurityPolicyRuleElRateLimitOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeSecurityPolicyRuleElRateLimitOptionsEl {}
impl BuildDataComputeSecurityPolicyRuleElRateLimitOptionsEl {
    pub fn build(self) -> DataComputeSecurityPolicyRuleElRateLimitOptionsEl {
        DataComputeSecurityPolicyRuleElRateLimitOptionsEl {
            ban_duration_sec: core::default::Default::default(),
            ban_threshold: core::default::Default::default(),
            conform_action: core::default::Default::default(),
            enforce_on_key: core::default::Default::default(),
            enforce_on_key_configs: core::default::Default::default(),
            enforce_on_key_name: core::default::Default::default(),
            exceed_action: core::default::Default::default(),
            exceed_redirect_options: core::default::Default::default(),
            rate_limit_threshold: core::default::Default::default(),
        }
    }
}
pub struct DataComputeSecurityPolicyRuleElRateLimitOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeSecurityPolicyRuleElRateLimitOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeSecurityPolicyRuleElRateLimitOptionsElRef {
        DataComputeSecurityPolicyRuleElRateLimitOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeSecurityPolicyRuleElRateLimitOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ban_duration_sec` after provisioning.\n"]
    pub fn ban_duration_sec(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ban_duration_sec", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ban_threshold` after provisioning.\n"]
    pub fn ban_threshold(
        &self,
    ) -> ListRef<DataComputeSecurityPolicyRuleElRateLimitOptionsElBanThresholdElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ban_threshold", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `conform_action` after provisioning.\n"]
    pub fn conform_action(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.conform_action", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enforce_on_key` after provisioning.\n"]
    pub fn enforce_on_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enforce_on_key", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enforce_on_key_configs` after provisioning.\n"]
    pub fn enforce_on_key_configs(
        &self,
    ) -> ListRef<DataComputeSecurityPolicyRuleElRateLimitOptionsElEnforceOnKeyConfigsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.enforce_on_key_configs", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enforce_on_key_name` after provisioning.\n"]
    pub fn enforce_on_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enforce_on_key_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `exceed_action` after provisioning.\n"]
    pub fn exceed_action(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.exceed_action", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `exceed_redirect_options` after provisioning.\n"]
    pub fn exceed_redirect_options(
        &self,
    ) -> ListRef<DataComputeSecurityPolicyRuleElRateLimitOptionsElExceedRedirectOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.exceed_redirect_options", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `rate_limit_threshold` after provisioning.\n"]
    pub fn rate_limit_threshold(
        &self,
    ) -> ListRef<DataComputeSecurityPolicyRuleElRateLimitOptionsElRateLimitThresholdElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rate_limit_threshold", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeSecurityPolicyRuleElRedirectOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    target: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl DataComputeSecurityPolicyRuleElRedirectOptionsEl {
    #[doc = "Set the field `target`.\n"]
    pub fn set_target(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.target = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeSecurityPolicyRuleElRedirectOptionsEl {
    type O = BlockAssignable<DataComputeSecurityPolicyRuleElRedirectOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeSecurityPolicyRuleElRedirectOptionsEl {}
impl BuildDataComputeSecurityPolicyRuleElRedirectOptionsEl {
    pub fn build(self) -> DataComputeSecurityPolicyRuleElRedirectOptionsEl {
        DataComputeSecurityPolicyRuleElRedirectOptionsEl {
            target: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}
pub struct DataComputeSecurityPolicyRuleElRedirectOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeSecurityPolicyRuleElRedirectOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeSecurityPolicyRuleElRedirectOptionsElRef {
        DataComputeSecurityPolicyRuleElRedirectOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeSecurityPolicyRuleElRedirectOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeSecurityPolicyRuleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    header_action: Option<ListField<DataComputeSecurityPolicyRuleElHeaderActionEl>>,
    #[serde(rename = "match", skip_serializing_if = "Option::is_none")]
    match_: Option<ListField<DataComputeSecurityPolicyRuleElMatchEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preconfigured_waf_config:
        Option<ListField<DataComputeSecurityPolicyRuleElPreconfiguredWafConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preview: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rate_limit_options: Option<ListField<DataComputeSecurityPolicyRuleElRateLimitOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    redirect_options: Option<ListField<DataComputeSecurityPolicyRuleElRedirectOptionsEl>>,
}
impl DataComputeSecurityPolicyRuleEl {
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.action = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `header_action`.\n"]
    pub fn set_header_action(
        mut self,
        v: impl Into<ListField<DataComputeSecurityPolicyRuleElHeaderActionEl>>,
    ) -> Self {
        self.header_action = Some(v.into());
        self
    }
    #[doc = "Set the field `match_`.\n"]
    pub fn set_match(
        mut self,
        v: impl Into<ListField<DataComputeSecurityPolicyRuleElMatchEl>>,
    ) -> Self {
        self.match_ = Some(v.into());
        self
    }
    #[doc = "Set the field `preconfigured_waf_config`.\n"]
    pub fn set_preconfigured_waf_config(
        mut self,
        v: impl Into<ListField<DataComputeSecurityPolicyRuleElPreconfiguredWafConfigEl>>,
    ) -> Self {
        self.preconfigured_waf_config = Some(v.into());
        self
    }
    #[doc = "Set the field `preview`.\n"]
    pub fn set_preview(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.preview = Some(v.into());
        self
    }
    #[doc = "Set the field `priority`.\n"]
    pub fn set_priority(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.priority = Some(v.into());
        self
    }
    #[doc = "Set the field `rate_limit_options`.\n"]
    pub fn set_rate_limit_options(
        mut self,
        v: impl Into<ListField<DataComputeSecurityPolicyRuleElRateLimitOptionsEl>>,
    ) -> Self {
        self.rate_limit_options = Some(v.into());
        self
    }
    #[doc = "Set the field `redirect_options`.\n"]
    pub fn set_redirect_options(
        mut self,
        v: impl Into<ListField<DataComputeSecurityPolicyRuleElRedirectOptionsEl>>,
    ) -> Self {
        self.redirect_options = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeSecurityPolicyRuleEl {
    type O = BlockAssignable<DataComputeSecurityPolicyRuleEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeSecurityPolicyRuleEl {}
impl BuildDataComputeSecurityPolicyRuleEl {
    pub fn build(self) -> DataComputeSecurityPolicyRuleEl {
        DataComputeSecurityPolicyRuleEl {
            action: core::default::Default::default(),
            description: core::default::Default::default(),
            header_action: core::default::Default::default(),
            match_: core::default::Default::default(),
            preconfigured_waf_config: core::default::Default::default(),
            preview: core::default::Default::default(),
            priority: core::default::Default::default(),
            rate_limit_options: core::default::Default::default(),
            redirect_options: core::default::Default::default(),
        }
    }
}
pub struct DataComputeSecurityPolicyRuleElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeSecurityPolicyRuleElRef {
    fn new(shared: StackShared, base: String) -> DataComputeSecurityPolicyRuleElRef {
        DataComputeSecurityPolicyRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeSecurityPolicyRuleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `header_action` after provisioning.\n"]
    pub fn header_action(&self) -> ListRef<DataComputeSecurityPolicyRuleElHeaderActionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.header_action", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> ListRef<DataComputeSecurityPolicyRuleElMatchElRef> {
        ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }
    #[doc = "Get a reference to the value of field `preconfigured_waf_config` after provisioning.\n"]
    pub fn preconfigured_waf_config(
        &self,
    ) -> ListRef<DataComputeSecurityPolicyRuleElPreconfiguredWafConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.preconfigured_waf_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `preview` after provisioning.\n"]
    pub fn preview(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.preview", self.base))
    }
    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }
    #[doc = "Get a reference to the value of field `rate_limit_options` after provisioning.\n"]
    pub fn rate_limit_options(
        &self,
    ) -> ListRef<DataComputeSecurityPolicyRuleElRateLimitOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rate_limit_options", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `redirect_options` after provisioning.\n"]
    pub fn redirect_options(&self) -> ListRef<DataComputeSecurityPolicyRuleElRedirectOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.redirect_options", self.base),
        )
    }
}
