use super::provider::ProviderGrafana;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DashboardData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    config_json: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    folder: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    org_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    overwrite: Option<PrimField<bool>>,
}
struct Dashboard_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DashboardData>,
}
#[derive(Clone)]
pub struct Dashboard(Rc<Dashboard_>);
impl Dashboard {
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
    #[doc = "Set the field `folder`.\nThe id or UID of the folder to save the dashboard in."]
    pub fn set_folder(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().folder = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `message`.\nSet a commit message for the version history."]
    pub fn set_message(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().message = Some(v.into());
        self
    }
    #[doc = "Set the field `org_id`.\nThe Organization ID. If not set, the Org ID defined in the provider block will be used."]
    pub fn set_org_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().org_id = Some(v.into());
        self
    }
    #[doc = "Set the field `overwrite`.\nSet to true if you want to overwrite existing dashboard with newer version, same dashboard title in folder or same dashboard uid."]
    pub fn set_overwrite(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().overwrite = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `config_json` after provisioning.\nThe complete dashboard model JSON.\n\nStarting with Grafana v13, use the resource corresponding to your dashboard's API version for Kubernetes-style dashboards.\n\nIf you decide to use this legacy resource with a Kubernetes-style dashboard definition:\n- In Grafana v12, provide the \"spec\" field of the dashboard definition.\n- In Grafana v13 and later, provide the full Kubernetes-style dashboard JSON (including \"apiVersion\", \"kind\", \"metadata\", and \"spec\")."]
    pub fn config_json(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.config_json", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dashboard_id` after provisioning.\nThe numeric ID of the dashboard computed by Grafana."]
    pub fn dashboard_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dashboard_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `folder` after provisioning.\nThe id or UID of the folder to save the dashboard in."]
    pub fn folder(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.folder", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `message` after provisioning.\nSet a commit message for the version history."]
    pub fn message(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.message", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `org_id` after provisioning.\nThe Organization ID. If not set, the Org ID defined in the provider block will be used."]
    pub fn org_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.org_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `overwrite` after provisioning.\nSet to true if you want to overwrite existing dashboard with newer version, same dashboard title in folder or same dashboard uid."]
    pub fn overwrite(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.overwrite", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nThe unique identifier of a dashboard. This is used to construct its URL. It's automatically generated if not provided when creating a dashboard. The uid allows having consistent URLs for accessing dashboards and when syncing dashboards between multiple Grafana installs."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `url` after provisioning.\nThe full URL of the dashboard."]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\nWhenever you save a version of your dashboard, a copy of that version is saved so that previous versions of your dashboard are not lost."]
    pub fn version(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version", self.extract_ref()),
        )
    }
}
impl Referable for Dashboard {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for Dashboard {}
impl ToListMappable for Dashboard {
    type O = ListRef<DashboardRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for Dashboard_ {
    fn extract_resource_type(&self) -> String {
        "grafana_dashboard".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDashboard {
    pub tf_id: String,
    #[doc = "The complete dashboard model JSON.\n\nStarting with Grafana v13, use the resource corresponding to your dashboard's API version for Kubernetes-style dashboards.\n\nIf you decide to use this legacy resource with a Kubernetes-style dashboard definition:\n- In Grafana v12, provide the \"spec\" field of the dashboard definition.\n- In Grafana v13 and later, provide the full Kubernetes-style dashboard JSON (including \"apiVersion\", \"kind\", \"metadata\", and \"spec\")."]
    pub config_json: PrimField<String>,
}
impl BuildDashboard {
    pub fn build(self, stack: &mut Stack) -> Dashboard {
        let out = Dashboard(Rc::new(Dashboard_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DashboardData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                config_json: self.config_json,
                folder: core::default::Default::default(),
                id: core::default::Default::default(),
                message: core::default::Default::default(),
                org_id: core::default::Default::default(),
                overwrite: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct DashboardRef {
    shared: StackShared,
    base: String,
}
impl Ref for DashboardRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DashboardRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `config_json` after provisioning.\nThe complete dashboard model JSON.\n\nStarting with Grafana v13, use the resource corresponding to your dashboard's API version for Kubernetes-style dashboards.\n\nIf you decide to use this legacy resource with a Kubernetes-style dashboard definition:\n- In Grafana v12, provide the \"spec\" field of the dashboard definition.\n- In Grafana v13 and later, provide the full Kubernetes-style dashboard JSON (including \"apiVersion\", \"kind\", \"metadata\", and \"spec\")."]
    pub fn config_json(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.config_json", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dashboard_id` after provisioning.\nThe numeric ID of the dashboard computed by Grafana."]
    pub fn dashboard_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dashboard_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `folder` after provisioning.\nThe id or UID of the folder to save the dashboard in."]
    pub fn folder(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.folder", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `message` after provisioning.\nSet a commit message for the version history."]
    pub fn message(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.message", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `org_id` after provisioning.\nThe Organization ID. If not set, the Org ID defined in the provider block will be used."]
    pub fn org_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.org_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `overwrite` after provisioning.\nSet to true if you want to overwrite existing dashboard with newer version, same dashboard title in folder or same dashboard uid."]
    pub fn overwrite(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.overwrite", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nThe unique identifier of a dashboard. This is used to construct its URL. It's automatically generated if not provided when creating a dashboard. The uid allows having consistent URLs for accessing dashboards and when syncing dashboards between multiple Grafana installs."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `url` after provisioning.\nThe full URL of the dashboard."]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\nWhenever you save a version of your dashboard, a copy of that version is saved so that previous versions of your dashboard are not lost."]
    pub fn version(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version", self.extract_ref()),
        )
    }
}
