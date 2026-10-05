use super::provider::ProviderGrafana;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct FleetManagementPipelineData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    config_type: Option<PrimField<String>>,
    contents: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    matchers: Option<ListField<PrimField<String>>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    terraform_source_namespace: Option<PrimField<String>>,
}
struct FleetManagementPipeline_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<FleetManagementPipelineData>,
}
#[derive(Clone)]
pub struct FleetManagementPipeline(Rc<FleetManagementPipeline_>);
impl FleetManagementPipeline {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(self, provider: &ProviderGrafana) -> Self {
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
    #[doc = "Set the field `config_type`.\nType of the config. Must be one of: ALLOY, OTEL. Defaults to ALLOY if not specified."]
    pub fn set_config_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().config_type = Some(v.into());
        self
    }
    #[doc = "Set the field `enabled`.\nWhether the pipeline is enabled for collectors"]
    pub fn set_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `matchers`.\nUsed to match against collectors and assign pipelines to them; follows the syntax of Prometheus Alertmanager matchers"]
    pub fn set_matchers(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().matchers = Some(v.into());
        self
    }
    #[doc = "Set the field `terraform_source_namespace`.\nNamespace sent with the pipeline source (always `SOURCE_TYPE_TERRAFORM` in the Fleet Management API). Use a stable value per Terraform root or workspace so the UI shows Terraform as the source and API sync semantics stay consistent. If omitted, the namespace `default` is used."]
    pub fn set_terraform_source_namespace(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().terraform_source_namespace = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `config_type` after provisioning.\nType of the config. Must be one of: ALLOY, OTEL. Defaults to ALLOY if not specified."]
    pub fn config_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.config_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `contents` after provisioning.\nConfiguration contents of the pipeline to be used by collectors (can be Alloy config syntax or OTel YAML)"]
    pub fn contents(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.contents", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nWhether the pipeline is enabled for collectors"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\nServer-assigned ID of the pipeline"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `matchers` after provisioning.\nUsed to match against collectors and assign pipelines to them; follows the syntax of Prometheus Alertmanager matchers"]
    pub fn matchers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.matchers", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the pipeline which is the unique identifier for the pipeline"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_source_namespace` after provisioning.\nNamespace sent with the pipeline source (always `SOURCE_TYPE_TERRAFORM` in the Fleet Management API). Use a stable value per Terraform root or workspace so the UI shows Terraform as the source and API sync semantics stay consistent. If omitted, the namespace `default` is used."]
    pub fn terraform_source_namespace(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.terraform_source_namespace", self.extract_ref()),
        )
    }
}
impl Referable for FleetManagementPipeline {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for FleetManagementPipeline {}
impl ToListMappable for FleetManagementPipeline {
    type O = ListRef<FleetManagementPipelineRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for FleetManagementPipeline_ {
    fn extract_resource_type(&self) -> String {
        "grafana_fleet_management_pipeline".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildFleetManagementPipeline {
    pub tf_id: String,
    #[doc = "Configuration contents of the pipeline to be used by collectors (can be Alloy config syntax or OTel YAML)"]
    pub contents: PrimField<String>,
    #[doc = "Name of the pipeline which is the unique identifier for the pipeline"]
    pub name: PrimField<String>,
}
impl BuildFleetManagementPipeline {
    pub fn build(self, stack: &mut Stack) -> FleetManagementPipeline {
        let out = FleetManagementPipeline(Rc::new(FleetManagementPipeline_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(FleetManagementPipelineData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                config_type: core::default::Default::default(),
                contents: self.contents,
                enabled: core::default::Default::default(),
                matchers: core::default::Default::default(),
                name: self.name,
                terraform_source_namespace: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct FleetManagementPipelineRef {
    shared: StackShared,
    base: String,
}
impl Ref for FleetManagementPipelineRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl FleetManagementPipelineRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `config_type` after provisioning.\nType of the config. Must be one of: ALLOY, OTEL. Defaults to ALLOY if not specified."]
    pub fn config_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.config_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `contents` after provisioning.\nConfiguration contents of the pipeline to be used by collectors (can be Alloy config syntax or OTel YAML)"]
    pub fn contents(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.contents", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nWhether the pipeline is enabled for collectors"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\nServer-assigned ID of the pipeline"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `matchers` after provisioning.\nUsed to match against collectors and assign pipelines to them; follows the syntax of Prometheus Alertmanager matchers"]
    pub fn matchers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.matchers", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the pipeline which is the unique identifier for the pipeline"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_source_namespace` after provisioning.\nNamespace sent with the pipeline source (always `SOURCE_TYPE_TERRAFORM` in the Fleet Management API). Use a stable value per Terraform root or workspace so the UI shows Terraform as the source and API sync semantics stay consistent. If omitted, the namespace `default` is used."]
    pub fn terraform_source_namespace(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.terraform_source_namespace", self.extract_ref()),
        )
    }
}
