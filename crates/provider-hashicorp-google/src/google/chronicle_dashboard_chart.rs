use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ChronicleDashboardChartData {
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
    id: Option<PrimField<String>>,
    instance: PrimField<String>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    native_dashboard: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    chart_layout: Option<Vec<ChronicleDashboardChartChartLayoutEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dashboard_chart: Option<Vec<ChronicleDashboardChartDashboardChartEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dashboard_query: Option<Vec<ChronicleDashboardChartDashboardQueryEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ChronicleDashboardChartTimeoutsEl>,
    dynamic: ChronicleDashboardChartDynamic,
}
struct ChronicleDashboardChart_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ChronicleDashboardChartData>,
}
#[derive(Clone)]
pub struct ChronicleDashboardChart(Rc<ChronicleDashboardChart_>);
impl ChronicleDashboardChart {
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
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `native_dashboard`.\nThe parent NativeDashboard resource name, formatted as projects/{project}/locations/{location}/instances/{instance}/nativeDashboards/{dashboard_id}"]
    pub fn set_native_dashboard(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().native_dashboard = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `chart_layout`.\n"]
    pub fn set_chart_layout(
        self,
        v: impl Into<BlockAssignable<ChronicleDashboardChartChartLayoutEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().chart_layout = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.chart_layout = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `dashboard_chart`.\n"]
    pub fn set_dashboard_chart(
        self,
        v: impl Into<BlockAssignable<ChronicleDashboardChartDashboardChartEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().dashboard_chart = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.dashboard_chart = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `dashboard_query`.\n"]
    pub fn set_dashboard_query(
        self,
        v: impl Into<BlockAssignable<ChronicleDashboardChartDashboardQueryEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().dashboard_query = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.dashboard_query = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ChronicleDashboardChartTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `chart_id` after provisioning.\nThe unique identifier of the chart, automatically extracted from the full resource name."]
    pub fn chart_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.chart_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `instance` after provisioning.\nThe ID of the Chronicle instance."]
    pub fn instance(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the Chronicle instance."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe full resource name of the DashboardChart."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `native_dashboard` after provisioning.\nThe parent NativeDashboard resource name, formatted as projects/{project}/locations/{location}/instances/{instance}/nativeDashboards/{dashboard_id}"]
    pub fn native_dashboard(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.native_dashboard", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `chart_layout` after provisioning.\n"]
    pub fn chart_layout(&self) -> ListRef<ChronicleDashboardChartChartLayoutElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.chart_layout", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dashboard_chart` after provisioning.\n"]
    pub fn dashboard_chart(&self) -> ListRef<ChronicleDashboardChartDashboardChartElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dashboard_chart", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dashboard_query` after provisioning.\n"]
    pub fn dashboard_query(&self) -> ListRef<ChronicleDashboardChartDashboardQueryElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dashboard_query", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ChronicleDashboardChartTimeoutsElRef {
        ChronicleDashboardChartTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ChronicleDashboardChart {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ChronicleDashboardChart {}
impl ToListMappable for ChronicleDashboardChart {
    type O = ListRef<ChronicleDashboardChartRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ChronicleDashboardChart_ {
    fn extract_resource_type(&self) -> String {
        "google_chronicle_dashboard_chart".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildChronicleDashboardChart {
    pub tf_id: String,
    #[doc = "The ID of the Chronicle instance."]
    pub instance: PrimField<String>,
    #[doc = "The location of the Chronicle instance."]
    pub location: PrimField<String>,
}
impl BuildChronicleDashboardChart {
    pub fn build(self, stack: &mut Stack) -> ChronicleDashboardChart {
        let out = ChronicleDashboardChart(Rc::new(ChronicleDashboardChart_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ChronicleDashboardChartData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                instance: self.instance,
                location: self.location,
                native_dashboard: core::default::Default::default(),
                project: core::default::Default::default(),
                chart_layout: core::default::Default::default(),
                dashboard_chart: core::default::Default::default(),
                dashboard_query: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ChronicleDashboardChartRef {
    shared: StackShared,
    base: String,
}
impl Ref for ChronicleDashboardChartRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ChronicleDashboardChartRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `chart_id` after provisioning.\nThe unique identifier of the chart, automatically extracted from the full resource name."]
    pub fn chart_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.chart_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `instance` after provisioning.\nThe ID of the Chronicle instance."]
    pub fn instance(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the Chronicle instance."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe full resource name of the DashboardChart."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `native_dashboard` after provisioning.\nThe parent NativeDashboard resource name, formatted as projects/{project}/locations/{location}/instances/{instance}/nativeDashboards/{dashboard_id}"]
    pub fn native_dashboard(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.native_dashboard", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `chart_layout` after provisioning.\n"]
    pub fn chart_layout(&self) -> ListRef<ChronicleDashboardChartChartLayoutElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.chart_layout", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dashboard_chart` after provisioning.\n"]
    pub fn dashboard_chart(&self) -> ListRef<ChronicleDashboardChartDashboardChartElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dashboard_chart", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dashboard_query` after provisioning.\n"]
    pub fn dashboard_query(&self) -> ListRef<ChronicleDashboardChartDashboardQueryElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dashboard_query", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ChronicleDashboardChartTimeoutsElRef {
        ChronicleDashboardChartTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ChronicleDashboardChartChartLayoutEl {
    span_x: PrimField<f64>,
    span_y: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_x: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_y: Option<PrimField<f64>>,
}
impl ChronicleDashboardChartChartLayoutEl {
    #[doc = "Set the field `start_x`.\n"]
    pub fn set_start_x(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.start_x = Some(v.into());
        self
    }
    #[doc = "Set the field `start_y`.\n"]
    pub fn set_start_y(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.start_y = Some(v.into());
        self
    }
}
impl ToListMappable for ChronicleDashboardChartChartLayoutEl {
    type O = BlockAssignable<ChronicleDashboardChartChartLayoutEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildChronicleDashboardChartChartLayoutEl {
    #[doc = ""]
    pub span_x: PrimField<f64>,
    #[doc = ""]
    pub span_y: PrimField<f64>,
}
impl BuildChronicleDashboardChartChartLayoutEl {
    pub fn build(self) -> ChronicleDashboardChartChartLayoutEl {
        ChronicleDashboardChartChartLayoutEl {
            span_x: self.span_x,
            span_y: self.span_y,
            start_x: core::default::Default::default(),
            start_y: core::default::Default::default(),
        }
    }
}
pub struct ChronicleDashboardChartChartLayoutElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ChronicleDashboardChartChartLayoutElRef {
    fn new(shared: StackShared, base: String) -> ChronicleDashboardChartChartLayoutElRef {
        ChronicleDashboardChartChartLayoutElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ChronicleDashboardChartChartLayoutElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `span_x` after provisioning.\n"]
    pub fn span_x(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.span_x", self.base))
    }
    #[doc = "Get a reference to the value of field `span_y` after provisioning.\n"]
    pub fn span_y(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.span_y", self.base))
    }
    #[doc = "Get a reference to the value of field `start_x` after provisioning.\n"]
    pub fn start_x(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.start_x", self.base))
    }
    #[doc = "Get a reference to the value of field `start_y` after provisioning.\n"]
    pub fn start_y(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.start_y", self.base))
    }
}
#[derive(Serialize)]
pub struct ChronicleDashboardChartDashboardChartElChartDatasourceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    data_sources: Option<ListField<PrimField<String>>>,
}
impl ChronicleDashboardChartDashboardChartElChartDatasourceEl {
    #[doc = "Set the field `data_sources`.\nName(s) of the datasource used in the chart. Available values include:\n'UDM', 'ENTITY', 'INGESTION_METRICS', 'RULE_DETECTIONS', 'RULESETS',\n'GLOBAL', 'IOC_MATCHES', 'RULES', 'SOAR_CASES', 'SOAR_PLAYBOOKS',\n'SOAR_CASE_HISTORY', 'DATA_TABLE', 'INVESTIGATION', 'INVESTIGATION_FEEDBACK'."]
    pub fn set_data_sources(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.data_sources = Some(v.into());
        self
    }
}
impl ToListMappable for ChronicleDashboardChartDashboardChartElChartDatasourceEl {
    type O = BlockAssignable<ChronicleDashboardChartDashboardChartElChartDatasourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildChronicleDashboardChartDashboardChartElChartDatasourceEl {}
impl BuildChronicleDashboardChartDashboardChartElChartDatasourceEl {
    pub fn build(self) -> ChronicleDashboardChartDashboardChartElChartDatasourceEl {
        ChronicleDashboardChartDashboardChartElChartDatasourceEl {
            data_sources: core::default::Default::default(),
        }
    }
}
pub struct ChronicleDashboardChartDashboardChartElChartDatasourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ChronicleDashboardChartDashboardChartElChartDatasourceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ChronicleDashboardChartDashboardChartElChartDatasourceElRef {
        ChronicleDashboardChartDashboardChartElChartDatasourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ChronicleDashboardChartDashboardChartElChartDatasourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `dashboard_query` after provisioning.\nThe unique system ID of the query linked to this chart."]
    pub fn dashboard_query(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dashboard_query", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `data_sources` after provisioning.\nName(s) of the datasource used in the chart. Available values include:\n'UDM', 'ENTITY', 'INGESTION_METRICS', 'RULE_DETECTIONS', 'RULESETS',\n'GLOBAL', 'IOC_MATCHES', 'RULES', 'SOAR_CASES', 'SOAR_PLAYBOOKS',\n'SOAR_CASE_HISTORY', 'DATA_TABLE', 'INVESTIGATION', 'INVESTIGATION_FEEDBACK'."]
    pub fn data_sources(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.data_sources", self.base))
    }
}
#[derive(Serialize)]
pub struct ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElExternalLinkEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    link: PrimField<String>,
}
impl ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElExternalLinkEl { # [doc = "Set the field `description`.\n"] pub fn set_description (mut self , v : impl Into < PrimField < String > >) -> Self { self . description = Some (v . into ()) ; self } }
impl ToListMappable for ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElExternalLinkEl { type O = BlockAssignable < ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElExternalLinkEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElExternalLinkEl
{
    #[doc = ""]
    pub link: PrimField<String>,
}
impl BuildChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElExternalLinkEl { pub fn build (self) -> ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElExternalLinkEl { ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElExternalLinkEl { description : core :: default :: Default :: default () , link : self . link , } } }
pub struct ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElExternalLinkElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElExternalLinkElRef { fn new (shared : StackShared , base : String) -> ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElExternalLinkElRef { ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElExternalLinkElRef { shared : shared , base : base . to_string () , } } }
impl ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElExternalLinkElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `description` after provisioning.\n"] pub fn description (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.description" , self . base)) } # [doc = "Get a reference to the value of field `link` after provisioning.\n"] pub fn link (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.link" , self . base)) } }
#[derive(Serialize)]
pub struct ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElFilterElDashboardFiltersElFilterOperatorAndValuesEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    field_values: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter_operator: Option<PrimField<String>>,
}
impl ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElFilterElDashboardFiltersElFilterOperatorAndValuesEl { # [doc = "Set the field `field_values`.\n"] pub fn set_field_values (mut self , v : impl Into < ListField < PrimField < String > > >) -> Self { self . field_values = Some (v . into ()) ; self } # [doc = "Set the field `filter_operator`.\n Possible values: [\"EQUAL\", \"NOT_EQUAL\", \"IN\", \"GREATER_THAN\", \"GREATER_THAN_OR_EQUAL_TO\", \"LESS_THAN\", \"LESS_THAN_OR_EQUAL_TO\", \"BETWEEN\", \"PAST\", \"IS_NULL\", \"IS_NOT_NULL\", \"STARTS_WITH\", \"ENDS_WITH\", \"DOES_NOT_STARTS_WITH\", \"DOES_NOT_ENDS_WITH\", \"NOT_IN\", \"CONTAINS\", \"DOES_NOT_CONTAIN\"]"] pub fn set_filter_operator (mut self , v : impl Into < PrimField < String > >) -> Self { self . filter_operator = Some (v . into ()) ; self } }
impl ToListMappable for ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElFilterElDashboardFiltersElFilterOperatorAndValuesEl { type O = BlockAssignable < ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElFilterElDashboardFiltersElFilterOperatorAndValuesEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElFilterElDashboardFiltersElFilterOperatorAndValuesEl
{}
impl BuildChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElFilterElDashboardFiltersElFilterOperatorAndValuesEl { pub fn build (self) -> ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElFilterElDashboardFiltersElFilterOperatorAndValuesEl { ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElFilterElDashboardFiltersElFilterOperatorAndValuesEl { field_values : core :: default :: Default :: default () , filter_operator : core :: default :: Default :: default () , } } }
pub struct ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElFilterElDashboardFiltersElFilterOperatorAndValuesElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElFilterElDashboardFiltersElFilterOperatorAndValuesElRef { fn new (shared : StackShared , base : String) -> ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElFilterElDashboardFiltersElFilterOperatorAndValuesElRef { ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElFilterElDashboardFiltersElFilterOperatorAndValuesElRef { shared : shared , base : base . to_string () , } } }
impl ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElFilterElDashboardFiltersElFilterOperatorAndValuesElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `field_values` after provisioning.\n"] pub fn field_values (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.field_values" , self . base)) } # [doc = "Get a reference to the value of field `filter_operator` after provisioning.\n Possible values: [\"EQUAL\", \"NOT_EQUAL\", \"IN\", \"GREATER_THAN\", \"GREATER_THAN_OR_EQUAL_TO\", \"LESS_THAN\", \"LESS_THAN_OR_EQUAL_TO\", \"BETWEEN\", \"PAST\", \"IS_NULL\", \"IS_NOT_NULL\", \"STARTS_WITH\", \"ENDS_WITH\", \"DOES_NOT_STARTS_WITH\", \"DOES_NOT_ENDS_WITH\", \"NOT_IN\", \"CONTAINS\", \"DOES_NOT_CONTAIN\"]"] pub fn filter_operator (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.filter_operator" , self . base)) } }
#[derive(Serialize, Default)]
struct ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElFilterElDashboardFiltersElDynamic { filter_operator_and_values : Option < DynamicBlock < ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElFilterElDashboardFiltersElFilterOperatorAndValuesEl >> , }
#[derive(Serialize)]
pub struct ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElFilterElDashboardFiltersEl { dashboard_filter_id : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] filter_operator_and_values : Option < Vec < ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElFilterElDashboardFiltersElFilterOperatorAndValuesEl > > , dynamic : ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElFilterElDashboardFiltersElDynamic , }
impl ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElFilterElDashboardFiltersEl { # [doc = "Set the field `filter_operator_and_values`.\n"] pub fn set_filter_operator_and_values (mut self , v : impl Into < BlockAssignable < ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElFilterElDashboardFiltersElFilterOperatorAndValuesEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . filter_operator_and_values = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . filter_operator_and_values = Some (d) ; } } self } }
impl ToListMappable for ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElFilterElDashboardFiltersEl { type O = BlockAssignable < ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElFilterElDashboardFiltersEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElFilterElDashboardFiltersEl
{
    #[doc = ""]
    pub dashboard_filter_id: PrimField<String>,
}
impl BuildChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElFilterElDashboardFiltersEl { pub fn build (self) -> ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElFilterElDashboardFiltersEl { ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElFilterElDashboardFiltersEl { dashboard_filter_id : self . dashboard_filter_id , filter_operator_and_values : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElFilterElDashboardFiltersElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElFilterElDashboardFiltersElRef { fn new (shared : StackShared , base : String) -> ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElFilterElDashboardFiltersElRef { ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElFilterElDashboardFiltersElRef { shared : shared , base : base . to_string () , } } }
impl ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElFilterElDashboardFiltersElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `dashboard_filter_id` after provisioning.\n"] pub fn dashboard_filter_id (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.dashboard_filter_id" , self . base)) } # [doc = "Get a reference to the value of field `filter_operator_and_values` after provisioning.\n"] pub fn filter_operator_and_values (& self) -> ListRef < ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElFilterElDashboardFiltersElFilterOperatorAndValuesElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.filter_operator_and_values" , self . base)) } }
#[derive(Serialize, Default)]
struct ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElFilterElDynamic { dashboard_filters : Option < DynamicBlock < ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElFilterElDashboardFiltersEl >> , }
#[derive(Serialize)]
pub struct ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElFilterEl { # [serde (skip_serializing_if = "Option::is_none")] dashboard_filters : Option < Vec < ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElFilterElDashboardFiltersEl > > , dynamic : ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElFilterElDynamic , }
impl
    ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElFilterEl
{
    #[doc = "Set the field `dashboard_filters`.\n"]
    pub fn set_dashboard_filters(
        mut self,
        v : impl Into < BlockAssignable < ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElFilterElDashboardFiltersEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.dashboard_filters = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.dashboard_filters = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElFilterEl { type O = BlockAssignable < ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElFilterEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElFilterEl
{}
impl BuildChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElFilterEl { pub fn build (self) -> ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElFilterEl { ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElFilterEl { dashboard_filters : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElFilterElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElFilterElRef { fn new (shared : StackShared , base : String) -> ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElFilterElRef { ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElFilterElRef { shared : shared , base : base . to_string () , } } }
impl ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElFilterElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `dashboard_filters` after provisioning.\n"] pub fn dashboard_filters (& self) -> ListRef < ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElFilterElDashboardFiltersElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.dashboard_filters" , self . base)) } }
#[derive(Serialize)]
pub struct ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElQueryEl
{
    query: PrimField<String>,
}
impl
    ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElQueryEl
{
}
impl ToListMappable for ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElQueryEl { type O = BlockAssignable < ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElQueryEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElQueryEl
{
    #[doc = ""]
    pub query: PrimField<String>,
}
impl BuildChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElQueryEl { pub fn build (self) -> ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElQueryEl { ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElQueryEl { query : self . query , } } }
pub struct ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElQueryElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElQueryElRef { fn new (shared : StackShared , base : String) -> ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElQueryElRef { ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElQueryElRef { shared : shared , base : base . to_string () , } } }
impl ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElQueryElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `query` after provisioning.\n"] pub fn query (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.query" , self . base)) } }
#[derive(Serialize, Default)]
struct ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElDynamic { external_link : Option < DynamicBlock < ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElExternalLinkEl >> , filter : Option < DynamicBlock < ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElFilterEl >> , query : Option < DynamicBlock < ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElQueryEl >> , }
#[derive(Serialize)]
pub struct ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsEl { # [serde (skip_serializing_if = "Option::is_none")] left_click_column : Option < PrimField < String > > , new_tab : PrimField < bool > , # [serde (skip_serializing_if = "Option::is_none")] external_link : Option < Vec < ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElExternalLinkEl > > , # [serde (skip_serializing_if = "Option::is_none")] filter : Option < Vec < ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElFilterEl > > , # [serde (skip_serializing_if = "Option::is_none")] query : Option < Vec < ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElQueryEl > > , dynamic : ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElDynamic , }
impl ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsEl {
    #[doc = "Set the field `left_click_column`.\n"]
    pub fn set_left_click_column(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.left_click_column = Some(v.into());
        self
    }
    #[doc = "Set the field `external_link`.\n"]
    pub fn set_external_link(
        mut self,
        v : impl Into < BlockAssignable < ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElExternalLinkEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.external_link = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.external_link = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(
        mut self,
        v : impl Into < BlockAssignable < ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElFilterEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.filter = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.filter = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `query`.\n"]
    pub fn set_query(
        mut self,
        v : impl Into < BlockAssignable < ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElQueryEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.query = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.query = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsEl
{
    type O = BlockAssignable<
        ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsEl
{
    #[doc = ""]
    pub new_tab: PrimField<bool>,
}
impl BuildChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsEl {
    pub fn build(
        self,
    ) -> ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsEl
    {
        ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsEl {
            left_click_column: core::default::Default::default(),
            new_tab: self.new_tab,
            external_link: core::default::Default::default(),
            filter: core::default::Default::default(),
            query: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElRef
    {
        ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElRef { shared : shared , base : base . to_string () , }
    }
}
impl ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `left_click_column` after provisioning.\n"]
    pub fn left_click_column(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.left_click_column", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `new_tab` after provisioning.\n"]
    pub fn new_tab(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.new_tab", self.base))
    }
    #[doc = "Get a reference to the value of field `external_link` after provisioning.\n"]    pub fn external_link (& self) -> ListRef < ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElExternalLinkElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.external_link", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]    pub fn filter (& self) -> ListRef < ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElFilterElRef >{
        ListRef::new(self.shared().clone(), format!("{}.filter", self.base))
    }
    #[doc = "Get a reference to the value of field `query` after provisioning.\n"]    pub fn query (& self) -> ListRef < ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElQueryElRef >{
        ListRef::new(self.shared().clone(), format!("{}.query", self.base))
    }
}
#[derive(Serialize)]
pub struct ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElDefaultSettingsEl
{
    enabled: PrimField<bool>,
}
impl ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElDefaultSettingsEl {}
impl ToListMappable
    for ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElDefaultSettingsEl
{
    type O = BlockAssignable<
        ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElDefaultSettingsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElDefaultSettingsEl
{
    #[doc = ""]
    pub enabled: PrimField<bool>,
}
impl
    BuildChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElDefaultSettingsEl
{
    pub fn build(
        self,
    ) -> ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElDefaultSettingsEl
    {
        ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElDefaultSettingsEl {
            enabled: self.enabled,
        }
    }
}
pub struct ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElDefaultSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElDefaultSettingsElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElDefaultSettingsElRef
    {
        ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElDefaultSettingsElRef { shared : shared , base : base . to_string () , }
    }
}
impl ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElDefaultSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize, Default)]
struct ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElDynamic { custom_settings : Option < DynamicBlock < ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsEl >> , default_settings : Option < DynamicBlock < ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElDefaultSettingsEl >> , }
#[derive(Serialize)]
pub struct ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsEl { display_name : PrimField < String > , id : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] custom_settings : Option < Vec < ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsEl > > , # [serde (skip_serializing_if = "Option::is_none")] default_settings : Option < Vec < ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElDefaultSettingsEl > > , dynamic : ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElDynamic , }
impl ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsEl {
    #[doc = "Set the field `custom_settings`.\n"]
    pub fn set_custom_settings(
        mut self,
        v : impl Into < BlockAssignable < ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.custom_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `default_settings`.\n"]
    pub fn set_default_settings(
        mut self,
        v : impl Into < BlockAssignable < ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElDefaultSettingsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.default_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.default_settings = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsEl {
    type O =
        BlockAssignable<ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsEl {
    #[doc = ""]
    pub display_name: PrimField<String>,
    #[doc = ""]
    pub id: PrimField<String>,
}
impl BuildChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsEl {
    pub fn build(self) -> ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsEl {
        ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsEl {
            display_name: self.display_name,
            id: self.id,
            custom_settings: core::default::Default::default(),
            default_settings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElRef {
        ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
    #[doc = "Get a reference to the value of field `custom_settings` after provisioning.\n"]
    pub fn custom_settings(
        &self,
    ) -> ListRef<
        ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElCustomSettingsElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `default_settings` after provisioning.\n"]    pub fn default_settings (& self) -> ListRef < ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElDefaultSettingsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.default_settings", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElExternalLinkEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    link: PrimField<String>,
}
impl ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElExternalLinkEl { # [doc = "Set the field `description`.\n"] pub fn set_description (mut self , v : impl Into < PrimField < String > >) -> Self { self . description = Some (v . into ()) ; self } }
impl ToListMappable for ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElExternalLinkEl { type O = BlockAssignable < ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElExternalLinkEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElExternalLinkEl
{
    #[doc = ""]
    pub link: PrimField<String>,
}
impl BuildChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElExternalLinkEl { pub fn build (self) -> ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElExternalLinkEl { ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElExternalLinkEl { description : core :: default :: Default :: default () , link : self . link , } } }
pub struct ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElExternalLinkElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElExternalLinkElRef { fn new (shared : StackShared , base : String) -> ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElExternalLinkElRef { ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElExternalLinkElRef { shared : shared , base : base . to_string () , } } }
impl ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElExternalLinkElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `description` after provisioning.\n"] pub fn description (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.description" , self . base)) } # [doc = "Get a reference to the value of field `link` after provisioning.\n"] pub fn link (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.link" , self . base)) } }
#[derive(Serialize)]
pub struct ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElFilterElDashboardFiltersElFilterOperatorAndValuesEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    field_values: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter_operator: Option<PrimField<String>>,
}
impl ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElFilterElDashboardFiltersElFilterOperatorAndValuesEl { # [doc = "Set the field `field_values`.\n"] pub fn set_field_values (mut self , v : impl Into < ListField < PrimField < String > > >) -> Self { self . field_values = Some (v . into ()) ; self } # [doc = "Set the field `filter_operator`.\n Possible values: [\"EQUAL\", \"NOT_EQUAL\", \"IN\", \"GREATER_THAN\", \"GREATER_THAN_OR_EQUAL_TO\", \"LESS_THAN\", \"LESS_THAN_OR_EQUAL_TO\", \"BETWEEN\", \"PAST\", \"IS_NULL\", \"IS_NOT_NULL\", \"STARTS_WITH\", \"ENDS_WITH\", \"DOES_NOT_STARTS_WITH\", \"DOES_NOT_ENDS_WITH\", \"NOT_IN\", \"CONTAINS\", \"DOES_NOT_CONTAIN\"]"] pub fn set_filter_operator (mut self , v : impl Into < PrimField < String > >) -> Self { self . filter_operator = Some (v . into ()) ; self } }
impl ToListMappable for ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElFilterElDashboardFiltersElFilterOperatorAndValuesEl { type O = BlockAssignable < ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElFilterElDashboardFiltersElFilterOperatorAndValuesEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElFilterElDashboardFiltersElFilterOperatorAndValuesEl
{}
impl BuildChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElFilterElDashboardFiltersElFilterOperatorAndValuesEl { pub fn build (self) -> ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElFilterElDashboardFiltersElFilterOperatorAndValuesEl { ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElFilterElDashboardFiltersElFilterOperatorAndValuesEl { field_values : core :: default :: Default :: default () , filter_operator : core :: default :: Default :: default () , } } }
pub struct ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElFilterElDashboardFiltersElFilterOperatorAndValuesElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElFilterElDashboardFiltersElFilterOperatorAndValuesElRef { fn new (shared : StackShared , base : String) -> ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElFilterElDashboardFiltersElFilterOperatorAndValuesElRef { ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElFilterElDashboardFiltersElFilterOperatorAndValuesElRef { shared : shared , base : base . to_string () , } } }
impl ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElFilterElDashboardFiltersElFilterOperatorAndValuesElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `field_values` after provisioning.\n"] pub fn field_values (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.field_values" , self . base)) } # [doc = "Get a reference to the value of field `filter_operator` after provisioning.\n Possible values: [\"EQUAL\", \"NOT_EQUAL\", \"IN\", \"GREATER_THAN\", \"GREATER_THAN_OR_EQUAL_TO\", \"LESS_THAN\", \"LESS_THAN_OR_EQUAL_TO\", \"BETWEEN\", \"PAST\", \"IS_NULL\", \"IS_NOT_NULL\", \"STARTS_WITH\", \"ENDS_WITH\", \"DOES_NOT_STARTS_WITH\", \"DOES_NOT_ENDS_WITH\", \"NOT_IN\", \"CONTAINS\", \"DOES_NOT_CONTAIN\"]"] pub fn filter_operator (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.filter_operator" , self . base)) } }
#[derive(Serialize, Default)]
struct ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElFilterElDashboardFiltersElDynamic { filter_operator_and_values : Option < DynamicBlock < ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElFilterElDashboardFiltersElFilterOperatorAndValuesEl >> , }
#[derive(Serialize)]
pub struct ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElFilterElDashboardFiltersEl { dashboard_filter_id : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] filter_operator_and_values : Option < Vec < ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElFilterElDashboardFiltersElFilterOperatorAndValuesEl > > , dynamic : ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElFilterElDashboardFiltersElDynamic , }
impl ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElFilterElDashboardFiltersEl { # [doc = "Set the field `filter_operator_and_values`.\n"] pub fn set_filter_operator_and_values (mut self , v : impl Into < BlockAssignable < ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElFilterElDashboardFiltersElFilterOperatorAndValuesEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . filter_operator_and_values = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . filter_operator_and_values = Some (d) ; } } self } }
impl ToListMappable for ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElFilterElDashboardFiltersEl { type O = BlockAssignable < ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElFilterElDashboardFiltersEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElFilterElDashboardFiltersEl
{
    #[doc = ""]
    pub dashboard_filter_id: PrimField<String>,
}
impl BuildChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElFilterElDashboardFiltersEl { pub fn build (self) -> ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElFilterElDashboardFiltersEl { ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElFilterElDashboardFiltersEl { dashboard_filter_id : self . dashboard_filter_id , filter_operator_and_values : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElFilterElDashboardFiltersElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElFilterElDashboardFiltersElRef { fn new (shared : StackShared , base : String) -> ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElFilterElDashboardFiltersElRef { ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElFilterElDashboardFiltersElRef { shared : shared , base : base . to_string () , } } }
impl ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElFilterElDashboardFiltersElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `dashboard_filter_id` after provisioning.\n"] pub fn dashboard_filter_id (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.dashboard_filter_id" , self . base)) } # [doc = "Get a reference to the value of field `filter_operator_and_values` after provisioning.\n"] pub fn filter_operator_and_values (& self) -> ListRef < ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElFilterElDashboardFiltersElFilterOperatorAndValuesElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.filter_operator_and_values" , self . base)) } }
#[derive(Serialize, Default)]
struct ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElFilterElDynamic { dashboard_filters : Option < DynamicBlock < ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElFilterElDashboardFiltersEl >> , }
#[derive(Serialize)]
pub struct ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElFilterEl { # [serde (skip_serializing_if = "Option::is_none")] dashboard_filters : Option < Vec < ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElFilterElDashboardFiltersEl > > , dynamic : ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElFilterElDynamic , }
impl ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElFilterEl { # [doc = "Set the field `dashboard_filters`.\n"] pub fn set_dashboard_filters (mut self , v : impl Into < BlockAssignable < ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElFilterElDashboardFiltersEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . dashboard_filters = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . dashboard_filters = Some (d) ; } } self } }
impl ToListMappable for ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElFilterEl { type O = BlockAssignable < ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElFilterEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElFilterEl
{}
impl BuildChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElFilterEl { pub fn build (self) -> ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElFilterEl { ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElFilterEl { dashboard_filters : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElFilterElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElFilterElRef { fn new (shared : StackShared , base : String) -> ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElFilterElRef { ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElFilterElRef { shared : shared , base : base . to_string () , } } }
impl ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElFilterElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `dashboard_filters` after provisioning.\n"] pub fn dashboard_filters (& self) -> ListRef < ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElFilterElDashboardFiltersElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.dashboard_filters" , self . base)) } }
#[derive(Serialize)]
pub struct ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElQueryEl
{
    query: PrimField<String>,
}
impl
    ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElQueryEl
{
}
impl ToListMappable for ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElQueryEl { type O = BlockAssignable < ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElQueryEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElQueryEl
{
    #[doc = ""]
    pub query: PrimField<String>,
}
impl BuildChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElQueryEl { pub fn build (self) -> ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElQueryEl { ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElQueryEl { query : self . query , } } }
pub struct ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElQueryElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElQueryElRef { fn new (shared : StackShared , base : String) -> ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElQueryElRef { ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElQueryElRef { shared : shared , base : base . to_string () , } } }
impl ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElQueryElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `query` after provisioning.\n"] pub fn query (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.query" , self . base)) } }
#[derive(Serialize, Default)]
struct ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElDynamic { external_link : Option < DynamicBlock < ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElExternalLinkEl >> , filter : Option < DynamicBlock < ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElFilterEl >> , query : Option < DynamicBlock < ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElQueryEl >> , }
#[derive(Serialize)]
pub struct ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsEl { new_tab : PrimField < bool > , # [serde (skip_serializing_if = "Option::is_none")] external_link : Option < Vec < ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElExternalLinkEl > > , # [serde (skip_serializing_if = "Option::is_none")] filter : Option < Vec < ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElFilterEl > > , # [serde (skip_serializing_if = "Option::is_none")] query : Option < Vec < ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElQueryEl > > , dynamic : ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElDynamic , }
impl ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsEl {
    #[doc = "Set the field `external_link`.\n"]
    pub fn set_external_link(
        mut self,
        v : impl Into < BlockAssignable < ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElExternalLinkEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.external_link = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.external_link = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(
        mut self,
        v : impl Into < BlockAssignable < ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElFilterEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.filter = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.filter = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `query`.\n"]
    pub fn set_query(
        mut self,
        v : impl Into < BlockAssignable < ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElQueryEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.query = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.query = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsEl
{
    type O = BlockAssignable<
        ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsEl
{
    #[doc = ""]
    pub new_tab: PrimField<bool>,
}
impl
    BuildChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsEl
{
    pub fn build(
        self,
    ) -> ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsEl
    {
        ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsEl {
            new_tab: self.new_tab,
            external_link: core::default::Default::default(),
            filter: core::default::Default::default(),
            query: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElRef
    {
        ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElRef { shared : shared , base : base . to_string () , }
    }
}
impl ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `new_tab` after provisioning.\n"]
    pub fn new_tab(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.new_tab", self.base))
    }
    #[doc = "Get a reference to the value of field `external_link` after provisioning.\n"]    pub fn external_link (& self) -> ListRef < ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElExternalLinkElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.external_link", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]    pub fn filter (& self) -> ListRef < ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElFilterElRef >{
        ListRef::new(self.shared().clone(), format!("{}.filter", self.base))
    }
    #[doc = "Get a reference to the value of field `query` after provisioning.\n"]    pub fn query (& self) -> ListRef < ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElQueryElRef >{
        ListRef::new(self.shared().clone(), format!("{}.query", self.base))
    }
}
#[derive(Serialize)]
pub struct ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElDefaultSettingsEl
{
    enabled: PrimField<bool>,
}
impl ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElDefaultSettingsEl {}
impl ToListMappable
    for ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElDefaultSettingsEl
{
    type O = BlockAssignable<
        ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElDefaultSettingsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElDefaultSettingsEl
{
    #[doc = ""]
    pub enabled: PrimField<bool>,
}
impl
    BuildChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElDefaultSettingsEl
{
    pub fn build(
        self,
    ) -> ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElDefaultSettingsEl
    {
        ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElDefaultSettingsEl {
            enabled: self.enabled,
        }
    }
}
pub struct ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElDefaultSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElDefaultSettingsElRef { fn new (shared : StackShared , base : String) -> ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElDefaultSettingsElRef { ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElDefaultSettingsElRef { shared : shared , base : base . to_string () , } } }
impl ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElDefaultSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize, Default)]
struct ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElDynamic { custom_settings : Option < DynamicBlock < ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsEl >> , default_settings : Option < DynamicBlock < ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElDefaultSettingsEl >> , }
#[derive(Serialize)]
pub struct ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsEl { display_name : PrimField < String > , id : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] custom_settings : Option < Vec < ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsEl > > , # [serde (skip_serializing_if = "Option::is_none")] default_settings : Option < Vec < ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElDefaultSettingsEl > > , dynamic : ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElDynamic , }
impl ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsEl {
    #[doc = "Set the field `custom_settings`.\n"]
    pub fn set_custom_settings(
        mut self,
        v : impl Into < BlockAssignable < ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.custom_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `default_settings`.\n"]
    pub fn set_default_settings(
        mut self,
        v : impl Into < BlockAssignable < ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElDefaultSettingsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.default_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.default_settings = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsEl {
    type O =
        BlockAssignable<ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsEl {
    #[doc = ""]
    pub display_name: PrimField<String>,
    #[doc = ""]
    pub id: PrimField<String>,
}
impl BuildChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsEl {
    pub fn build(
        self,
    ) -> ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsEl {
        ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsEl {
            display_name: self.display_name,
            id: self.id,
            custom_settings: core::default::Default::default(),
            default_settings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElRef {
        ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
    #[doc = "Get a reference to the value of field `custom_settings` after provisioning.\n"]    pub fn custom_settings (& self) -> ListRef < ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElCustomSettingsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `default_settings` after provisioning.\n"]    pub fn default_settings (& self) -> ListRef < ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElDefaultSettingsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.default_settings", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ChronicleDashboardChartDashboardChartElDrillDownConfigElDynamic {
    left_drill_downs: Option<
        DynamicBlock<ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsEl>,
    >,
    right_drill_downs: Option<
        DynamicBlock<ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsEl>,
    >,
}
#[derive(Serialize)]
pub struct ChronicleDashboardChartDashboardChartElDrillDownConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    left_drill_downs:
        Option<Vec<ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    right_drill_downs:
        Option<Vec<ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsEl>>,
    dynamic: ChronicleDashboardChartDashboardChartElDrillDownConfigElDynamic,
}
impl ChronicleDashboardChartDashboardChartElDrillDownConfigEl {
    #[doc = "Set the field `left_drill_downs`.\n"]
    pub fn set_left_drill_downs(
        mut self,
        v: impl Into<
            BlockAssignable<
                ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.left_drill_downs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.left_drill_downs = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `right_drill_downs`.\n"]
    pub fn set_right_drill_downs(
        mut self,
        v: impl Into<
            BlockAssignable<
                ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.right_drill_downs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.right_drill_downs = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ChronicleDashboardChartDashboardChartElDrillDownConfigEl {
    type O = BlockAssignable<ChronicleDashboardChartDashboardChartElDrillDownConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildChronicleDashboardChartDashboardChartElDrillDownConfigEl {}
impl BuildChronicleDashboardChartDashboardChartElDrillDownConfigEl {
    pub fn build(self) -> ChronicleDashboardChartDashboardChartElDrillDownConfigEl {
        ChronicleDashboardChartDashboardChartElDrillDownConfigEl {
            left_drill_downs: core::default::Default::default(),
            right_drill_downs: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ChronicleDashboardChartDashboardChartElDrillDownConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ChronicleDashboardChartDashboardChartElDrillDownConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ChronicleDashboardChartDashboardChartElDrillDownConfigElRef {
        ChronicleDashboardChartDashboardChartElDrillDownConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ChronicleDashboardChartDashboardChartElDrillDownConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `left_drill_downs` after provisioning.\n"]
    pub fn left_drill_downs(
        &self,
    ) -> ListRef<ChronicleDashboardChartDashboardChartElDrillDownConfigElLeftDrillDownsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.left_drill_downs", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `right_drill_downs` after provisioning.\n"]
    pub fn right_drill_downs(
        &self,
    ) -> ListRef<ChronicleDashboardChartDashboardChartElDrillDownConfigElRightDrillDownsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.right_drill_downs", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ChronicleDashboardChartDashboardChartElVisualizationElButtonElPropertiesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    button_style: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<PrimField<String>>,
}
impl ChronicleDashboardChartDashboardChartElVisualizationElButtonElPropertiesEl {
    #[doc = "Set the field `button_style`.\n Possible values: [\"BUTTON_STYLE_UNSPECIFIED\", \"BUTTON_STYLE_FILLED\", \"BUTTON_STYLE_OUTLINED\", \"BUTTON_STYLE_TRANSPARENT\"]"]
    pub fn set_button_style(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.button_style = Some(v.into());
        self
    }
    #[doc = "Set the field `color`.\n"]
    pub fn set_color(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.color = Some(v.into());
        self
    }
}
impl ToListMappable for ChronicleDashboardChartDashboardChartElVisualizationElButtonElPropertiesEl {
    type O =
        BlockAssignable<ChronicleDashboardChartDashboardChartElVisualizationElButtonElPropertiesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildChronicleDashboardChartDashboardChartElVisualizationElButtonElPropertiesEl {}
impl BuildChronicleDashboardChartDashboardChartElVisualizationElButtonElPropertiesEl {
    pub fn build(
        self,
    ) -> ChronicleDashboardChartDashboardChartElVisualizationElButtonElPropertiesEl {
        ChronicleDashboardChartDashboardChartElVisualizationElButtonElPropertiesEl {
            button_style: core::default::Default::default(),
            color: core::default::Default::default(),
        }
    }
}
pub struct ChronicleDashboardChartDashboardChartElVisualizationElButtonElPropertiesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ChronicleDashboardChartDashboardChartElVisualizationElButtonElPropertiesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ChronicleDashboardChartDashboardChartElVisualizationElButtonElPropertiesElRef {
        ChronicleDashboardChartDashboardChartElVisualizationElButtonElPropertiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ChronicleDashboardChartDashboardChartElVisualizationElButtonElPropertiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `button_style` after provisioning.\n Possible values: [\"BUTTON_STYLE_UNSPECIFIED\", \"BUTTON_STYLE_FILLED\", \"BUTTON_STYLE_OUTLINED\", \"BUTTON_STYLE_TRANSPARENT\"]"]
    pub fn button_style(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.button_style", self.base))
    }
    #[doc = "Get a reference to the value of field `color` after provisioning.\n"]
    pub fn color(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.color", self.base))
    }
}
#[derive(Serialize, Default)]
struct ChronicleDashboardChartDashboardChartElVisualizationElButtonElDynamic {
    properties: Option<
        DynamicBlock<ChronicleDashboardChartDashboardChartElVisualizationElButtonElPropertiesEl>,
    >,
}
#[derive(Serialize)]
pub struct ChronicleDashboardChartDashboardChartElVisualizationElButtonEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    hyperlink: PrimField<String>,
    label: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    new_tab: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    properties:
        Option<Vec<ChronicleDashboardChartDashboardChartElVisualizationElButtonElPropertiesEl>>,
    dynamic: ChronicleDashboardChartDashboardChartElVisualizationElButtonElDynamic,
}
impl ChronicleDashboardChartDashboardChartElVisualizationElButtonEl {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `new_tab`.\n"]
    pub fn set_new_tab(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.new_tab = Some(v.into());
        self
    }
    #[doc = "Set the field `properties`.\n"]
    pub fn set_properties(
        mut self,
        v: impl Into<
            BlockAssignable<
                ChronicleDashboardChartDashboardChartElVisualizationElButtonElPropertiesEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.properties = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.properties = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ChronicleDashboardChartDashboardChartElVisualizationElButtonEl {
    type O = BlockAssignable<ChronicleDashboardChartDashboardChartElVisualizationElButtonEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildChronicleDashboardChartDashboardChartElVisualizationElButtonEl {
    #[doc = ""]
    pub hyperlink: PrimField<String>,
    #[doc = ""]
    pub label: PrimField<String>,
}
impl BuildChronicleDashboardChartDashboardChartElVisualizationElButtonEl {
    pub fn build(self) -> ChronicleDashboardChartDashboardChartElVisualizationElButtonEl {
        ChronicleDashboardChartDashboardChartElVisualizationElButtonEl {
            description: core::default::Default::default(),
            hyperlink: self.hyperlink,
            label: self.label,
            new_tab: core::default::Default::default(),
            properties: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ChronicleDashboardChartDashboardChartElVisualizationElButtonElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ChronicleDashboardChartDashboardChartElVisualizationElButtonElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ChronicleDashboardChartDashboardChartElVisualizationElButtonElRef {
        ChronicleDashboardChartDashboardChartElVisualizationElButtonElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ChronicleDashboardChartDashboardChartElVisualizationElButtonElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `hyperlink` after provisioning.\n"]
    pub fn hyperlink(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.hyperlink", self.base))
    }
    #[doc = "Get a reference to the value of field `label` after provisioning.\n"]
    pub fn label(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.label", self.base))
    }
    #[doc = "Get a reference to the value of field `new_tab` after provisioning.\n"]
    pub fn new_tab(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.new_tab", self.base))
    }
    #[doc = "Get a reference to the value of field `properties` after provisioning.\n"]
    pub fn properties(
        &self,
    ) -> ListRef<ChronicleDashboardChartDashboardChartElVisualizationElButtonElPropertiesElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.properties", self.base))
    }
}
#[derive(Serialize)]
pub struct ChronicleDashboardChartDashboardChartElVisualizationElColumnDefsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    field: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    header: Option<PrimField<String>>,
}
impl ChronicleDashboardChartDashboardChartElVisualizationElColumnDefsEl {
    #[doc = "Set the field `field`.\n"]
    pub fn set_field(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.field = Some(v.into());
        self
    }
    #[doc = "Set the field `header`.\n"]
    pub fn set_header(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.header = Some(v.into());
        self
    }
}
impl ToListMappable for ChronicleDashboardChartDashboardChartElVisualizationElColumnDefsEl {
    type O = BlockAssignable<ChronicleDashboardChartDashboardChartElVisualizationElColumnDefsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildChronicleDashboardChartDashboardChartElVisualizationElColumnDefsEl {}
impl BuildChronicleDashboardChartDashboardChartElVisualizationElColumnDefsEl {
    pub fn build(self) -> ChronicleDashboardChartDashboardChartElVisualizationElColumnDefsEl {
        ChronicleDashboardChartDashboardChartElVisualizationElColumnDefsEl {
            field: core::default::Default::default(),
            header: core::default::Default::default(),
        }
    }
}
pub struct ChronicleDashboardChartDashboardChartElVisualizationElColumnDefsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ChronicleDashboardChartDashboardChartElVisualizationElColumnDefsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ChronicleDashboardChartDashboardChartElVisualizationElColumnDefsElRef {
        ChronicleDashboardChartDashboardChartElVisualizationElColumnDefsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ChronicleDashboardChartDashboardChartElVisualizationElColumnDefsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `field` after provisioning.\n"]
    pub fn field(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.field", self.base))
    }
    #[doc = "Get a reference to the value of field `header` after provisioning.\n"]
    pub fn header(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.header", self.base))
    }
}
#[derive(Serialize)]
pub struct ChronicleDashboardChartDashboardChartElVisualizationElGoogleMapsConfigElDataSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    count_column: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    latitude_column: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    longitude_column: Option<PrimField<String>>,
}
impl ChronicleDashboardChartDashboardChartElVisualizationElGoogleMapsConfigElDataSettingsEl {
    #[doc = "Set the field `count_column`.\n"]
    pub fn set_count_column(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.count_column = Some(v.into());
        self
    }
    #[doc = "Set the field `latitude_column`.\n"]
    pub fn set_latitude_column(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.latitude_column = Some(v.into());
        self
    }
    #[doc = "Set the field `longitude_column`.\n"]
    pub fn set_longitude_column(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.longitude_column = Some(v.into());
        self
    }
}
impl ToListMappable
    for ChronicleDashboardChartDashboardChartElVisualizationElGoogleMapsConfigElDataSettingsEl
{
    type O = BlockAssignable<
        ChronicleDashboardChartDashboardChartElVisualizationElGoogleMapsConfigElDataSettingsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildChronicleDashboardChartDashboardChartElVisualizationElGoogleMapsConfigElDataSettingsEl
{}
impl BuildChronicleDashboardChartDashboardChartElVisualizationElGoogleMapsConfigElDataSettingsEl {
    pub fn build(
        self,
    ) -> ChronicleDashboardChartDashboardChartElVisualizationElGoogleMapsConfigElDataSettingsEl
    {
        ChronicleDashboardChartDashboardChartElVisualizationElGoogleMapsConfigElDataSettingsEl {
            count_column: core::default::Default::default(),
            latitude_column: core::default::Default::default(),
            longitude_column: core::default::Default::default(),
        }
    }
}
pub struct ChronicleDashboardChartDashboardChartElVisualizationElGoogleMapsConfigElDataSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for ChronicleDashboardChartDashboardChartElVisualizationElGoogleMapsConfigElDataSettingsElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> ChronicleDashboardChartDashboardChartElVisualizationElGoogleMapsConfigElDataSettingsElRef
    {
        ChronicleDashboardChartDashboardChartElVisualizationElGoogleMapsConfigElDataSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ChronicleDashboardChartDashboardChartElVisualizationElGoogleMapsConfigElDataSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `count_column` after provisioning.\n"]
    pub fn count_column(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.count_column", self.base))
    }
    #[doc = "Get a reference to the value of field `latitude_column` after provisioning.\n"]
    pub fn latitude_column(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.latitude_column", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `longitude_column` after provisioning.\n"]
    pub fn longitude_column(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.longitude_column", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ChronicleDashboardChartDashboardChartElVisualizationElGoogleMapsConfigElMapPositionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    fit_data: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    latitude_value: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    longitude_value: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zoom_scale_value: Option<PrimField<f64>>,
}
impl ChronicleDashboardChartDashboardChartElVisualizationElGoogleMapsConfigElMapPositionEl {
    #[doc = "Set the field `fit_data`.\n"]
    pub fn set_fit_data(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.fit_data = Some(v.into());
        self
    }
    #[doc = "Set the field `latitude_value`.\n"]
    pub fn set_latitude_value(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.latitude_value = Some(v.into());
        self
    }
    #[doc = "Set the field `longitude_value`.\n"]
    pub fn set_longitude_value(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.longitude_value = Some(v.into());
        self
    }
    #[doc = "Set the field `zoom_scale_value`.\n"]
    pub fn set_zoom_scale_value(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.zoom_scale_value = Some(v.into());
        self
    }
}
impl ToListMappable
    for ChronicleDashboardChartDashboardChartElVisualizationElGoogleMapsConfigElMapPositionEl
{
    type O = BlockAssignable<
        ChronicleDashboardChartDashboardChartElVisualizationElGoogleMapsConfigElMapPositionEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildChronicleDashboardChartDashboardChartElVisualizationElGoogleMapsConfigElMapPositionEl
{}
impl BuildChronicleDashboardChartDashboardChartElVisualizationElGoogleMapsConfigElMapPositionEl {
    pub fn build(
        self,
    ) -> ChronicleDashboardChartDashboardChartElVisualizationElGoogleMapsConfigElMapPositionEl {
        ChronicleDashboardChartDashboardChartElVisualizationElGoogleMapsConfigElMapPositionEl {
            fit_data: core::default::Default::default(),
            latitude_value: core::default::Default::default(),
            longitude_value: core::default::Default::default(),
            zoom_scale_value: core::default::Default::default(),
        }
    }
}
pub struct ChronicleDashboardChartDashboardChartElVisualizationElGoogleMapsConfigElMapPositionElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for ChronicleDashboardChartDashboardChartElVisualizationElGoogleMapsConfigElMapPositionElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> ChronicleDashboardChartDashboardChartElVisualizationElGoogleMapsConfigElMapPositionElRef
    {
        ChronicleDashboardChartDashboardChartElVisualizationElGoogleMapsConfigElMapPositionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ChronicleDashboardChartDashboardChartElVisualizationElGoogleMapsConfigElMapPositionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `fit_data` after provisioning.\n"]
    pub fn fit_data(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.fit_data", self.base))
    }
    #[doc = "Get a reference to the value of field `latitude_value` after provisioning.\n"]
    pub fn latitude_value(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.latitude_value", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `longitude_value` after provisioning.\n"]
    pub fn longitude_value(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.longitude_value", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `zoom_scale_value` after provisioning.\n"]
    pub fn zoom_scale_value(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.zoom_scale_value", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ChronicleDashboardChartDashboardChartElVisualizationElGoogleMapsConfigElPointSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    point_size_type: Option<PrimField<String>>,
}
impl ChronicleDashboardChartDashboardChartElVisualizationElGoogleMapsConfigElPointSettingsEl {
    #[doc = "Set the field `color`.\n"]
    pub fn set_color(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.color = Some(v.into());
        self
    }
    #[doc = "Set the field `point_size_type`.\n Possible values: [\"POINT_SIZE_TYPE_UNSPECIFIED\", \"POINT_SIZE_TYPE_FIXED\", \"POINT_SIZE_TYPE_PROPORTIONAL_TO_SIZE\"]"]
    pub fn set_point_size_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.point_size_type = Some(v.into());
        self
    }
}
impl ToListMappable
    for ChronicleDashboardChartDashboardChartElVisualizationElGoogleMapsConfigElPointSettingsEl
{
    type O = BlockAssignable<
        ChronicleDashboardChartDashboardChartElVisualizationElGoogleMapsConfigElPointSettingsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildChronicleDashboardChartDashboardChartElVisualizationElGoogleMapsConfigElPointSettingsEl
{}
impl BuildChronicleDashboardChartDashboardChartElVisualizationElGoogleMapsConfigElPointSettingsEl {
    pub fn build(
        self,
    ) -> ChronicleDashboardChartDashboardChartElVisualizationElGoogleMapsConfigElPointSettingsEl
    {
        ChronicleDashboardChartDashboardChartElVisualizationElGoogleMapsConfigElPointSettingsEl {
            color: core::default::Default::default(),
            point_size_type: core::default::Default::default(),
        }
    }
}
pub struct ChronicleDashboardChartDashboardChartElVisualizationElGoogleMapsConfigElPointSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for ChronicleDashboardChartDashboardChartElVisualizationElGoogleMapsConfigElPointSettingsElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> ChronicleDashboardChartDashboardChartElVisualizationElGoogleMapsConfigElPointSettingsElRef
    {
        ChronicleDashboardChartDashboardChartElVisualizationElGoogleMapsConfigElPointSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ChronicleDashboardChartDashboardChartElVisualizationElGoogleMapsConfigElPointSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `color` after provisioning.\n"]
    pub fn color(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.color", self.base))
    }
    #[doc = "Get a reference to the value of field `point_size_type` after provisioning.\n Possible values: [\"POINT_SIZE_TYPE_UNSPECIFIED\", \"POINT_SIZE_TYPE_FIXED\", \"POINT_SIZE_TYPE_PROPORTIONAL_TO_SIZE\"]"]
    pub fn point_size_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.point_size_type", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ChronicleDashboardChartDashboardChartElVisualizationElGoogleMapsConfigElDynamic {
    data_settings: Option<
        DynamicBlock<
            ChronicleDashboardChartDashboardChartElVisualizationElGoogleMapsConfigElDataSettingsEl,
        >,
    >,
    map_position: Option<
        DynamicBlock<
            ChronicleDashboardChartDashboardChartElVisualizationElGoogleMapsConfigElMapPositionEl,
        >,
    >,
    point_settings: Option<
        DynamicBlock<
            ChronicleDashboardChartDashboardChartElVisualizationElGoogleMapsConfigElPointSettingsEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct ChronicleDashboardChartDashboardChartElVisualizationElGoogleMapsConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    plot_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_settings: Option<
        Vec<ChronicleDashboardChartDashboardChartElVisualizationElGoogleMapsConfigElDataSettingsEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    map_position: Option<
        Vec<ChronicleDashboardChartDashboardChartElVisualizationElGoogleMapsConfigElMapPositionEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    point_settings: Option<
        Vec<
            ChronicleDashboardChartDashboardChartElVisualizationElGoogleMapsConfigElPointSettingsEl,
        >,
    >,
    dynamic: ChronicleDashboardChartDashboardChartElVisualizationElGoogleMapsConfigElDynamic,
}
impl ChronicleDashboardChartDashboardChartElVisualizationElGoogleMapsConfigEl {
    #[doc = "Set the field `plot_mode`.\n Possible values: [\"PLOT_MODE_UNSPECIFIED\", \"PLOT_MODE_POINTS\", \"PLOT_MODE_HEATMAP\", \"PLOT_MODE_BOTH\"]"]
    pub fn set_plot_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.plot_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `data_settings`.\n"]
    pub fn set_data_settings(
        mut self,
        v : impl Into < BlockAssignable < ChronicleDashboardChartDashboardChartElVisualizationElGoogleMapsConfigElDataSettingsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.data_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.data_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `map_position`.\n"]
    pub fn set_map_position(
        mut self,
        v : impl Into < BlockAssignable < ChronicleDashboardChartDashboardChartElVisualizationElGoogleMapsConfigElMapPositionEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.map_position = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.map_position = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `point_settings`.\n"]
    pub fn set_point_settings(
        mut self,
        v : impl Into < BlockAssignable < ChronicleDashboardChartDashboardChartElVisualizationElGoogleMapsConfigElPointSettingsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.point_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.point_settings = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ChronicleDashboardChartDashboardChartElVisualizationElGoogleMapsConfigEl {
    type O =
        BlockAssignable<ChronicleDashboardChartDashboardChartElVisualizationElGoogleMapsConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildChronicleDashboardChartDashboardChartElVisualizationElGoogleMapsConfigEl {}
impl BuildChronicleDashboardChartDashboardChartElVisualizationElGoogleMapsConfigEl {
    pub fn build(self) -> ChronicleDashboardChartDashboardChartElVisualizationElGoogleMapsConfigEl {
        ChronicleDashboardChartDashboardChartElVisualizationElGoogleMapsConfigEl {
            plot_mode: core::default::Default::default(),
            data_settings: core::default::Default::default(),
            map_position: core::default::Default::default(),
            point_settings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ChronicleDashboardChartDashboardChartElVisualizationElGoogleMapsConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ChronicleDashboardChartDashboardChartElVisualizationElGoogleMapsConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ChronicleDashboardChartDashboardChartElVisualizationElGoogleMapsConfigElRef {
        ChronicleDashboardChartDashboardChartElVisualizationElGoogleMapsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ChronicleDashboardChartDashboardChartElVisualizationElGoogleMapsConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `plot_mode` after provisioning.\n Possible values: [\"PLOT_MODE_UNSPECIFIED\", \"PLOT_MODE_POINTS\", \"PLOT_MODE_HEATMAP\", \"PLOT_MODE_BOTH\"]"]
    pub fn plot_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.plot_mode", self.base))
    }
    #[doc = "Get a reference to the value of field `data_settings` after provisioning.\n"]
    pub fn data_settings(
        &self,
    ) -> ListRef<
        ChronicleDashboardChartDashboardChartElVisualizationElGoogleMapsConfigElDataSettingsElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.data_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `map_position` after provisioning.\n"]
    pub fn map_position(
        &self,
    ) -> ListRef<
        ChronicleDashboardChartDashboardChartElVisualizationElGoogleMapsConfigElMapPositionElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.map_position", self.base))
    }
    #[doc = "Get a reference to the value of field `point_settings` after provisioning.\n"]
    pub fn point_settings(
        &self,
    ) -> ListRef<
        ChronicleDashboardChartDashboardChartElVisualizationElGoogleMapsConfigElPointSettingsElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.point_settings", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ChronicleDashboardChartDashboardChartElVisualizationElLegendsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bottom: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    left: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    legend_align: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    legend_orient: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    padding: Option<ListField<PrimField<f64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    right: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    show: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    z: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    z_level: Option<PrimField<f64>>,
}
impl ChronicleDashboardChartDashboardChartElVisualizationElLegendsEl {
    #[doc = "Set the field `bottom`.\n"]
    pub fn set_bottom(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.bottom = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }
    #[doc = "Set the field `left`.\n"]
    pub fn set_left(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.left = Some(v.into());
        self
    }
    #[doc = "Set the field `legend_align`.\n Possible values: [\"AUTO\", \"LEFT\", \"RIGHT\"]"]
    pub fn set_legend_align(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.legend_align = Some(v.into());
        self
    }
    #[doc = "Set the field `legend_orient`.\n Possible values: [\"VERTICAL\", \"HORIZONTAL\"]"]
    pub fn set_legend_orient(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.legend_orient = Some(v.into());
        self
    }
    #[doc = "Set the field `padding`.\n"]
    pub fn set_padding(mut self, v: impl Into<ListField<PrimField<f64>>>) -> Self {
        self.padding = Some(v.into());
        self
    }
    #[doc = "Set the field `right`.\n"]
    pub fn set_right(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.right = Some(v.into());
        self
    }
    #[doc = "Set the field `show`.\n"]
    pub fn set_show(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.show = Some(v.into());
        self
    }
    #[doc = "Set the field `top`.\n"]
    pub fn set_top(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.top = Some(v.into());
        self
    }
    #[doc = "Set the field `z`.\n"]
    pub fn set_z(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.z = Some(v.into());
        self
    }
    #[doc = "Set the field `z_level`.\n"]
    pub fn set_z_level(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.z_level = Some(v.into());
        self
    }
}
impl ToListMappable for ChronicleDashboardChartDashboardChartElVisualizationElLegendsEl {
    type O = BlockAssignable<ChronicleDashboardChartDashboardChartElVisualizationElLegendsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildChronicleDashboardChartDashboardChartElVisualizationElLegendsEl {}
impl BuildChronicleDashboardChartDashboardChartElVisualizationElLegendsEl {
    pub fn build(self) -> ChronicleDashboardChartDashboardChartElVisualizationElLegendsEl {
        ChronicleDashboardChartDashboardChartElVisualizationElLegendsEl {
            bottom: core::default::Default::default(),
            id: core::default::Default::default(),
            left: core::default::Default::default(),
            legend_align: core::default::Default::default(),
            legend_orient: core::default::Default::default(),
            padding: core::default::Default::default(),
            right: core::default::Default::default(),
            show: core::default::Default::default(),
            top: core::default::Default::default(),
            z: core::default::Default::default(),
            z_level: core::default::Default::default(),
        }
    }
}
pub struct ChronicleDashboardChartDashboardChartElVisualizationElLegendsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ChronicleDashboardChartDashboardChartElVisualizationElLegendsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ChronicleDashboardChartDashboardChartElVisualizationElLegendsElRef {
        ChronicleDashboardChartDashboardChartElVisualizationElLegendsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ChronicleDashboardChartDashboardChartElVisualizationElLegendsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bottom` after provisioning.\n"]
    pub fn bottom(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.bottom", self.base))
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
    #[doc = "Get a reference to the value of field `left` after provisioning.\n"]
    pub fn left(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.left", self.base))
    }
    #[doc = "Get a reference to the value of field `legend_align` after provisioning.\n Possible values: [\"AUTO\", \"LEFT\", \"RIGHT\"]"]
    pub fn legend_align(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.legend_align", self.base))
    }
    #[doc = "Get a reference to the value of field `legend_orient` after provisioning.\n Possible values: [\"VERTICAL\", \"HORIZONTAL\"]"]
    pub fn legend_orient(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.legend_orient", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `padding` after provisioning.\n"]
    pub fn padding(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.padding", self.base))
    }
    #[doc = "Get a reference to the value of field `right` after provisioning.\n"]
    pub fn right(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.right", self.base))
    }
    #[doc = "Get a reference to the value of field `show` after provisioning.\n"]
    pub fn show(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.show", self.base))
    }
    #[doc = "Get a reference to the value of field `top` after provisioning.\n"]
    pub fn top(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.top", self.base))
    }
    #[doc = "Get a reference to the value of field `z` after provisioning.\n"]
    pub fn z(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.z", self.base))
    }
    #[doc = "Get a reference to the value of field `z_level` after provisioning.\n"]
    pub fn z_level(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.z_level", self.base))
    }
}
#[derive(Serialize)]
pub struct ChronicleDashboardChartDashboardChartElVisualizationElMarkdownElPropertiesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    background_color: Option<PrimField<String>>,
}
impl ChronicleDashboardChartDashboardChartElVisualizationElMarkdownElPropertiesEl {
    #[doc = "Set the field `background_color`.\n"]
    pub fn set_background_color(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.background_color = Some(v.into());
        self
    }
}
impl ToListMappable
    for ChronicleDashboardChartDashboardChartElVisualizationElMarkdownElPropertiesEl
{
    type O = BlockAssignable<
        ChronicleDashboardChartDashboardChartElVisualizationElMarkdownElPropertiesEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildChronicleDashboardChartDashboardChartElVisualizationElMarkdownElPropertiesEl {}
impl BuildChronicleDashboardChartDashboardChartElVisualizationElMarkdownElPropertiesEl {
    pub fn build(
        self,
    ) -> ChronicleDashboardChartDashboardChartElVisualizationElMarkdownElPropertiesEl {
        ChronicleDashboardChartDashboardChartElVisualizationElMarkdownElPropertiesEl {
            background_color: core::default::Default::default(),
        }
    }
}
pub struct ChronicleDashboardChartDashboardChartElVisualizationElMarkdownElPropertiesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ChronicleDashboardChartDashboardChartElVisualizationElMarkdownElPropertiesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ChronicleDashboardChartDashboardChartElVisualizationElMarkdownElPropertiesElRef {
        ChronicleDashboardChartDashboardChartElVisualizationElMarkdownElPropertiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ChronicleDashboardChartDashboardChartElVisualizationElMarkdownElPropertiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `background_color` after provisioning.\n"]
    pub fn background_color(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.background_color", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ChronicleDashboardChartDashboardChartElVisualizationElMarkdownElDynamic {
    properties: Option<
        DynamicBlock<ChronicleDashboardChartDashboardChartElVisualizationElMarkdownElPropertiesEl>,
    >,
}
#[derive(Serialize)]
pub struct ChronicleDashboardChartDashboardChartElVisualizationElMarkdownEl {
    content: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    properties:
        Option<Vec<ChronicleDashboardChartDashboardChartElVisualizationElMarkdownElPropertiesEl>>,
    dynamic: ChronicleDashboardChartDashboardChartElVisualizationElMarkdownElDynamic,
}
impl ChronicleDashboardChartDashboardChartElVisualizationElMarkdownEl {
    #[doc = "Set the field `properties`.\n"]
    pub fn set_properties(
        mut self,
        v: impl Into<
            BlockAssignable<
                ChronicleDashboardChartDashboardChartElVisualizationElMarkdownElPropertiesEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.properties = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.properties = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ChronicleDashboardChartDashboardChartElVisualizationElMarkdownEl {
    type O = BlockAssignable<ChronicleDashboardChartDashboardChartElVisualizationElMarkdownEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildChronicleDashboardChartDashboardChartElVisualizationElMarkdownEl {
    #[doc = ""]
    pub content: PrimField<String>,
}
impl BuildChronicleDashboardChartDashboardChartElVisualizationElMarkdownEl {
    pub fn build(self) -> ChronicleDashboardChartDashboardChartElVisualizationElMarkdownEl {
        ChronicleDashboardChartDashboardChartElVisualizationElMarkdownEl {
            content: self.content,
            properties: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ChronicleDashboardChartDashboardChartElVisualizationElMarkdownElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ChronicleDashboardChartDashboardChartElVisualizationElMarkdownElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ChronicleDashboardChartDashboardChartElVisualizationElMarkdownElRef {
        ChronicleDashboardChartDashboardChartElVisualizationElMarkdownElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ChronicleDashboardChartDashboardChartElVisualizationElMarkdownElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `content` after provisioning.\n"]
    pub fn content(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content", self.base))
    }
    #[doc = "Get a reference to the value of field `properties` after provisioning.\n"]
    pub fn properties(
        &self,
    ) -> ListRef<ChronicleDashboardChartDashboardChartElVisualizationElMarkdownElPropertiesElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.properties", self.base))
    }
}
#[derive(Serialize)]
pub struct ChronicleDashboardChartDashboardChartElVisualizationElSeriesElAreaStyleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    opacity: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    origin: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shadow_blur: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shadow_color: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shadow_offset_x: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shadow_offset_y: Option<PrimField<f64>>,
}
impl ChronicleDashboardChartDashboardChartElVisualizationElSeriesElAreaStyleEl {
    #[doc = "Set the field `color`.\n"]
    pub fn set_color(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.color = Some(v.into());
        self
    }
    #[doc = "Set the field `opacity`.\n"]
    pub fn set_opacity(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.opacity = Some(v.into());
        self
    }
    #[doc = "Set the field `origin`.\n"]
    pub fn set_origin(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.origin = Some(v.into());
        self
    }
    #[doc = "Set the field `shadow_blur`.\n"]
    pub fn set_shadow_blur(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.shadow_blur = Some(v.into());
        self
    }
    #[doc = "Set the field `shadow_color`.\n"]
    pub fn set_shadow_color(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.shadow_color = Some(v.into());
        self
    }
    #[doc = "Set the field `shadow_offset_x`.\n"]
    pub fn set_shadow_offset_x(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.shadow_offset_x = Some(v.into());
        self
    }
    #[doc = "Set the field `shadow_offset_y`.\n"]
    pub fn set_shadow_offset_y(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.shadow_offset_y = Some(v.into());
        self
    }
}
impl ToListMappable for ChronicleDashboardChartDashboardChartElVisualizationElSeriesElAreaStyleEl {
    type O =
        BlockAssignable<ChronicleDashboardChartDashboardChartElVisualizationElSeriesElAreaStyleEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildChronicleDashboardChartDashboardChartElVisualizationElSeriesElAreaStyleEl {}
impl BuildChronicleDashboardChartDashboardChartElVisualizationElSeriesElAreaStyleEl {
    pub fn build(
        self,
    ) -> ChronicleDashboardChartDashboardChartElVisualizationElSeriesElAreaStyleEl {
        ChronicleDashboardChartDashboardChartElVisualizationElSeriesElAreaStyleEl {
            color: core::default::Default::default(),
            opacity: core::default::Default::default(),
            origin: core::default::Default::default(),
            shadow_blur: core::default::Default::default(),
            shadow_color: core::default::Default::default(),
            shadow_offset_x: core::default::Default::default(),
            shadow_offset_y: core::default::Default::default(),
        }
    }
}
pub struct ChronicleDashboardChartDashboardChartElVisualizationElSeriesElAreaStyleElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ChronicleDashboardChartDashboardChartElVisualizationElSeriesElAreaStyleElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ChronicleDashboardChartDashboardChartElVisualizationElSeriesElAreaStyleElRef {
        ChronicleDashboardChartDashboardChartElVisualizationElSeriesElAreaStyleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ChronicleDashboardChartDashboardChartElVisualizationElSeriesElAreaStyleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `color` after provisioning.\n"]
    pub fn color(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.color", self.base))
    }
    #[doc = "Get a reference to the value of field `opacity` after provisioning.\n"]
    pub fn opacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.opacity", self.base))
    }
    #[doc = "Get a reference to the value of field `origin` after provisioning.\n"]
    pub fn origin(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.origin", self.base))
    }
    #[doc = "Get a reference to the value of field `shadow_blur` after provisioning.\n"]
    pub fn shadow_blur(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.shadow_blur", self.base))
    }
    #[doc = "Get a reference to the value of field `shadow_color` after provisioning.\n"]
    pub fn shadow_color(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.shadow_color", self.base))
    }
    #[doc = "Get a reference to the value of field `shadow_offset_x` after provisioning.\n"]
    pub fn shadow_offset_x(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.shadow_offset_x", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `shadow_offset_y` after provisioning.\n"]
    pub fn shadow_offset_y(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.shadow_offset_y", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ChronicleDashboardChartDashboardChartElVisualizationElSeriesElDataLabelEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    show: Option<PrimField<bool>>,
}
impl ChronicleDashboardChartDashboardChartElVisualizationElSeriesElDataLabelEl {
    #[doc = "Set the field `show`.\nWhether to show data label."]
    pub fn set_show(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.show = Some(v.into());
        self
    }
}
impl ToListMappable for ChronicleDashboardChartDashboardChartElVisualizationElSeriesElDataLabelEl {
    type O =
        BlockAssignable<ChronicleDashboardChartDashboardChartElVisualizationElSeriesElDataLabelEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildChronicleDashboardChartDashboardChartElVisualizationElSeriesElDataLabelEl {}
impl BuildChronicleDashboardChartDashboardChartElVisualizationElSeriesElDataLabelEl {
    pub fn build(
        self,
    ) -> ChronicleDashboardChartDashboardChartElVisualizationElSeriesElDataLabelEl {
        ChronicleDashboardChartDashboardChartElVisualizationElSeriesElDataLabelEl {
            show: core::default::Default::default(),
        }
    }
}
pub struct ChronicleDashboardChartDashboardChartElVisualizationElSeriesElDataLabelElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ChronicleDashboardChartDashboardChartElVisualizationElSeriesElDataLabelElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ChronicleDashboardChartDashboardChartElVisualizationElSeriesElDataLabelElRef {
        ChronicleDashboardChartDashboardChartElVisualizationElSeriesElDataLabelElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ChronicleDashboardChartDashboardChartElVisualizationElSeriesElDataLabelElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `show` after provisioning.\nWhether to show data label."]
    pub fn show(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.show", self.base))
    }
}
#[derive(Serialize)]
pub struct ChronicleDashboardChartDashboardChartElVisualizationElSeriesElEncodeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    item_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    x: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    y: Option<PrimField<String>>,
}
impl ChronicleDashboardChartDashboardChartElVisualizationElSeriesElEncodeEl {
    #[doc = "Set the field `item_name`.\n"]
    pub fn set_item_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.item_name = Some(v.into());
        self
    }
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
    #[doc = "Set the field `x`.\n"]
    pub fn set_x(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.x = Some(v.into());
        self
    }
    #[doc = "Set the field `y`.\n"]
    pub fn set_y(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.y = Some(v.into());
        self
    }
}
impl ToListMappable for ChronicleDashboardChartDashboardChartElVisualizationElSeriesElEncodeEl {
    type O =
        BlockAssignable<ChronicleDashboardChartDashboardChartElVisualizationElSeriesElEncodeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildChronicleDashboardChartDashboardChartElVisualizationElSeriesElEncodeEl {}
impl BuildChronicleDashboardChartDashboardChartElVisualizationElSeriesElEncodeEl {
    pub fn build(self) -> ChronicleDashboardChartDashboardChartElVisualizationElSeriesElEncodeEl {
        ChronicleDashboardChartDashboardChartElVisualizationElSeriesElEncodeEl {
            item_name: core::default::Default::default(),
            value: core::default::Default::default(),
            x: core::default::Default::default(),
            y: core::default::Default::default(),
        }
    }
}
pub struct ChronicleDashboardChartDashboardChartElVisualizationElSeriesElEncodeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ChronicleDashboardChartDashboardChartElVisualizationElSeriesElEncodeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ChronicleDashboardChartDashboardChartElVisualizationElSeriesElEncodeElRef {
        ChronicleDashboardChartDashboardChartElVisualizationElSeriesElEncodeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ChronicleDashboardChartDashboardChartElVisualizationElSeriesElEncodeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `item_name` after provisioning.\n"]
    pub fn item_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.item_name", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
    #[doc = "Get a reference to the value of field `x` after provisioning.\n"]
    pub fn x(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.x", self.base))
    }
    #[doc = "Get a reference to the value of field `y` after provisioning.\n"]
    pub fn y(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.y", self.base))
    }
}
#[derive(Serialize)]
pub struct ChronicleDashboardChartDashboardChartElVisualizationElSeriesElGaugeConfigElBaseValueEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<f64>>,
}
impl ChronicleDashboardChartDashboardChartElVisualizationElSeriesElGaugeConfigElBaseValueEl {
    #[doc = "Set the field `color`.\n"]
    pub fn set_color(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.color = Some(v.into());
        self
    }
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable
    for ChronicleDashboardChartDashboardChartElVisualizationElSeriesElGaugeConfigElBaseValueEl
{
    type O = BlockAssignable<
        ChronicleDashboardChartDashboardChartElVisualizationElSeriesElGaugeConfigElBaseValueEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildChronicleDashboardChartDashboardChartElVisualizationElSeriesElGaugeConfigElBaseValueEl
{}
impl BuildChronicleDashboardChartDashboardChartElVisualizationElSeriesElGaugeConfigElBaseValueEl {
    pub fn build(
        self,
    ) -> ChronicleDashboardChartDashboardChartElVisualizationElSeriesElGaugeConfigElBaseValueEl
    {
        ChronicleDashboardChartDashboardChartElVisualizationElSeriesElGaugeConfigElBaseValueEl {
            color: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}
pub struct ChronicleDashboardChartDashboardChartElVisualizationElSeriesElGaugeConfigElBaseValueElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for ChronicleDashboardChartDashboardChartElVisualizationElSeriesElGaugeConfigElBaseValueElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> ChronicleDashboardChartDashboardChartElVisualizationElSeriesElGaugeConfigElBaseValueElRef
    {
        ChronicleDashboardChartDashboardChartElVisualizationElSeriesElGaugeConfigElBaseValueElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ChronicleDashboardChartDashboardChartElVisualizationElSeriesElGaugeConfigElBaseValueElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `color` after provisioning.\n"]
    pub fn color(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.color", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct ChronicleDashboardChartDashboardChartElVisualizationElSeriesElGaugeConfigElLimitValueEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<f64>>,
}
impl ChronicleDashboardChartDashboardChartElVisualizationElSeriesElGaugeConfigElLimitValueEl {
    #[doc = "Set the field `color`.\n"]
    pub fn set_color(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.color = Some(v.into());
        self
    }
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable
    for ChronicleDashboardChartDashboardChartElVisualizationElSeriesElGaugeConfigElLimitValueEl
{
    type O = BlockAssignable<
        ChronicleDashboardChartDashboardChartElVisualizationElSeriesElGaugeConfigElLimitValueEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildChronicleDashboardChartDashboardChartElVisualizationElSeriesElGaugeConfigElLimitValueEl
{}
impl BuildChronicleDashboardChartDashboardChartElVisualizationElSeriesElGaugeConfigElLimitValueEl {
    pub fn build(
        self,
    ) -> ChronicleDashboardChartDashboardChartElVisualizationElSeriesElGaugeConfigElLimitValueEl
    {
        ChronicleDashboardChartDashboardChartElVisualizationElSeriesElGaugeConfigElLimitValueEl {
            color: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}
pub struct ChronicleDashboardChartDashboardChartElVisualizationElSeriesElGaugeConfigElLimitValueElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for ChronicleDashboardChartDashboardChartElVisualizationElSeriesElGaugeConfigElLimitValueElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> ChronicleDashboardChartDashboardChartElVisualizationElSeriesElGaugeConfigElLimitValueElRef
    {
        ChronicleDashboardChartDashboardChartElVisualizationElSeriesElGaugeConfigElLimitValueElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ChronicleDashboardChartDashboardChartElVisualizationElSeriesElGaugeConfigElLimitValueElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `color` after provisioning.\n"]
    pub fn color(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.color", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct ChronicleDashboardChartDashboardChartElVisualizationElSeriesElGaugeConfigElThresholdValuesEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<f64>>,
}
impl ChronicleDashboardChartDashboardChartElVisualizationElSeriesElGaugeConfigElThresholdValuesEl {
    #[doc = "Set the field `color`.\n"]
    pub fn set_color(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.color = Some(v.into());
        self
    }
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable
    for ChronicleDashboardChartDashboardChartElVisualizationElSeriesElGaugeConfigElThresholdValuesEl
{
    type O = BlockAssignable < ChronicleDashboardChartDashboardChartElVisualizationElSeriesElGaugeConfigElThresholdValuesEl > ;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildChronicleDashboardChartDashboardChartElVisualizationElSeriesElGaugeConfigElThresholdValuesEl
{}
impl BuildChronicleDashboardChartDashboardChartElVisualizationElSeriesElGaugeConfigElThresholdValuesEl { pub fn build (self) -> ChronicleDashboardChartDashboardChartElVisualizationElSeriesElGaugeConfigElThresholdValuesEl { ChronicleDashboardChartDashboardChartElVisualizationElSeriesElGaugeConfigElThresholdValuesEl { color : core :: default :: Default :: default () , value : core :: default :: Default :: default () , } } }
pub struct ChronicleDashboardChartDashboardChartElVisualizationElSeriesElGaugeConfigElThresholdValuesElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for ChronicleDashboardChartDashboardChartElVisualizationElSeriesElGaugeConfigElThresholdValuesElRef { fn new (shared : StackShared , base : String) -> ChronicleDashboardChartDashboardChartElVisualizationElSeriesElGaugeConfigElThresholdValuesElRef { ChronicleDashboardChartDashboardChartElVisualizationElSeriesElGaugeConfigElThresholdValuesElRef { shared : shared , base : base . to_string () , } } }
impl
    ChronicleDashboardChartDashboardChartElVisualizationElSeriesElGaugeConfigElThresholdValuesElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `color` after provisioning.\n"]
    pub fn color(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.color", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize, Default)]
struct ChronicleDashboardChartDashboardChartElVisualizationElSeriesElGaugeConfigElDynamic { base_value : Option < DynamicBlock < ChronicleDashboardChartDashboardChartElVisualizationElSeriesElGaugeConfigElBaseValueEl >> , limit_value : Option < DynamicBlock < ChronicleDashboardChartDashboardChartElVisualizationElSeriesElGaugeConfigElLimitValueEl >> , threshold_values : Option < DynamicBlock < ChronicleDashboardChartDashboardChartElVisualizationElSeriesElGaugeConfigElThresholdValuesEl >> , }
#[derive(Serialize)]
pub struct ChronicleDashboardChartDashboardChartElVisualizationElSeriesElGaugeConfigEl { # [serde (skip_serializing_if = "Option::is_none")] base_value : Option < Vec < ChronicleDashboardChartDashboardChartElVisualizationElSeriesElGaugeConfigElBaseValueEl > > , # [serde (skip_serializing_if = "Option::is_none")] limit_value : Option < Vec < ChronicleDashboardChartDashboardChartElVisualizationElSeriesElGaugeConfigElLimitValueEl > > , # [serde (skip_serializing_if = "Option::is_none")] threshold_values : Option < Vec < ChronicleDashboardChartDashboardChartElVisualizationElSeriesElGaugeConfigElThresholdValuesEl > > , dynamic : ChronicleDashboardChartDashboardChartElVisualizationElSeriesElGaugeConfigElDynamic , }
impl ChronicleDashboardChartDashboardChartElVisualizationElSeriesElGaugeConfigEl {
    #[doc = "Set the field `base_value`.\n"]
    pub fn set_base_value(
        mut self,
        v : impl Into < BlockAssignable < ChronicleDashboardChartDashboardChartElVisualizationElSeriesElGaugeConfigElBaseValueEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.base_value = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.base_value = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `limit_value`.\n"]
    pub fn set_limit_value(
        mut self,
        v : impl Into < BlockAssignable < ChronicleDashboardChartDashboardChartElVisualizationElSeriesElGaugeConfigElLimitValueEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.limit_value = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.limit_value = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `threshold_values`.\n"]
    pub fn set_threshold_values(
        mut self,
        v : impl Into < BlockAssignable < ChronicleDashboardChartDashboardChartElVisualizationElSeriesElGaugeConfigElThresholdValuesEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.threshold_values = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.threshold_values = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for ChronicleDashboardChartDashboardChartElVisualizationElSeriesElGaugeConfigEl
{
    type O = BlockAssignable<
        ChronicleDashboardChartDashboardChartElVisualizationElSeriesElGaugeConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildChronicleDashboardChartDashboardChartElVisualizationElSeriesElGaugeConfigEl {}
impl BuildChronicleDashboardChartDashboardChartElVisualizationElSeriesElGaugeConfigEl {
    pub fn build(
        self,
    ) -> ChronicleDashboardChartDashboardChartElVisualizationElSeriesElGaugeConfigEl {
        ChronicleDashboardChartDashboardChartElVisualizationElSeriesElGaugeConfigEl {
            base_value: core::default::Default::default(),
            limit_value: core::default::Default::default(),
            threshold_values: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ChronicleDashboardChartDashboardChartElVisualizationElSeriesElGaugeConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ChronicleDashboardChartDashboardChartElVisualizationElSeriesElGaugeConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ChronicleDashboardChartDashboardChartElVisualizationElSeriesElGaugeConfigElRef {
        ChronicleDashboardChartDashboardChartElVisualizationElSeriesElGaugeConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ChronicleDashboardChartDashboardChartElVisualizationElSeriesElGaugeConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `base_value` after provisioning.\n"]
    pub fn base_value(
        &self,
    ) -> ListRef<
        ChronicleDashboardChartDashboardChartElVisualizationElSeriesElGaugeConfigElBaseValueElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.base_value", self.base))
    }
    #[doc = "Get a reference to the value of field `limit_value` after provisioning.\n"]
    pub fn limit_value(
        &self,
    ) -> ListRef<
        ChronicleDashboardChartDashboardChartElVisualizationElSeriesElGaugeConfigElLimitValueElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.limit_value", self.base))
    }
    #[doc = "Get a reference to the value of field `threshold_values` after provisioning.\n"]    pub fn threshold_values (& self) -> ListRef < ChronicleDashboardChartDashboardChartElVisualizationElSeriesElGaugeConfigElThresholdValuesElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.threshold_values", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ChronicleDashboardChartDashboardChartElVisualizationElSeriesElItemColorsElColorsElValueEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<PrimField<String>>,
}
impl ChronicleDashboardChartDashboardChartElVisualizationElSeriesElItemColorsElColorsElValueEl {
    #[doc = "Set the field `color`.\n"]
    pub fn set_color(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.color = Some(v.into());
        self
    }
    #[doc = "Set the field `label`.\n"]
    pub fn set_label(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.label = Some(v.into());
        self
    }
}
impl ToListMappable
    for ChronicleDashboardChartDashboardChartElVisualizationElSeriesElItemColorsElColorsElValueEl
{
    type O = BlockAssignable<
        ChronicleDashboardChartDashboardChartElVisualizationElSeriesElItemColorsElColorsElValueEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildChronicleDashboardChartDashboardChartElVisualizationElSeriesElItemColorsElColorsElValueEl
{}
impl
    BuildChronicleDashboardChartDashboardChartElVisualizationElSeriesElItemColorsElColorsElValueEl
{
    pub fn build(
        self,
    ) -> ChronicleDashboardChartDashboardChartElVisualizationElSeriesElItemColorsElColorsElValueEl
    {
        ChronicleDashboardChartDashboardChartElVisualizationElSeriesElItemColorsElColorsElValueEl {
            color: core::default::Default::default(),
            label: core::default::Default::default(),
        }
    }
}
pub struct ChronicleDashboardChartDashboardChartElVisualizationElSeriesElItemColorsElColorsElValueElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for ChronicleDashboardChartDashboardChartElVisualizationElSeriesElItemColorsElColorsElValueElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> ChronicleDashboardChartDashboardChartElVisualizationElSeriesElItemColorsElColorsElValueElRef
    {
        ChronicleDashboardChartDashboardChartElVisualizationElSeriesElItemColorsElColorsElValueElRef { shared : shared , base : base . to_string () , }
    }
}
impl ChronicleDashboardChartDashboardChartElVisualizationElSeriesElItemColorsElColorsElValueElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `color` after provisioning.\n"]
    pub fn color(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.color", self.base))
    }
    #[doc = "Get a reference to the value of field `label` after provisioning.\n"]
    pub fn label(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.label", self.base))
    }
}
#[derive(Serialize, Default)]
struct ChronicleDashboardChartDashboardChartElVisualizationElSeriesElItemColorsElColorsElDynamic { value : Option < DynamicBlock < ChronicleDashboardChartDashboardChartElVisualizationElSeriesElItemColorsElColorsElValueEl >> , }
#[derive(Serialize)]
pub struct ChronicleDashboardChartDashboardChartElVisualizationElSeriesElItemColorsElColorsEl { # [serde (skip_serializing_if = "Option::is_none")] key : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] value : Option < Vec < ChronicleDashboardChartDashboardChartElVisualizationElSeriesElItemColorsElColorsElValueEl > > , dynamic : ChronicleDashboardChartDashboardChartElVisualizationElSeriesElItemColorsElColorsElDynamic , }
impl ChronicleDashboardChartDashboardChartElVisualizationElSeriesElItemColorsElColorsEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(
        mut self,
        v : impl Into < BlockAssignable < ChronicleDashboardChartDashboardChartElVisualizationElSeriesElItemColorsElColorsElValueEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.value = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.value = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for ChronicleDashboardChartDashboardChartElVisualizationElSeriesElItemColorsElColorsEl
{
    type O = BlockAssignable<
        ChronicleDashboardChartDashboardChartElVisualizationElSeriesElItemColorsElColorsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildChronicleDashboardChartDashboardChartElVisualizationElSeriesElItemColorsElColorsEl
{}
impl BuildChronicleDashboardChartDashboardChartElVisualizationElSeriesElItemColorsElColorsEl {
    pub fn build(
        self,
    ) -> ChronicleDashboardChartDashboardChartElVisualizationElSeriesElItemColorsElColorsEl {
        ChronicleDashboardChartDashboardChartElVisualizationElSeriesElItemColorsElColorsEl {
            key: core::default::Default::default(),
            value: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ChronicleDashboardChartDashboardChartElVisualizationElSeriesElItemColorsElColorsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ChronicleDashboardChartDashboardChartElVisualizationElSeriesElItemColorsElColorsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ChronicleDashboardChartDashboardChartElVisualizationElSeriesElItemColorsElColorsElRef {
        ChronicleDashboardChartDashboardChartElVisualizationElSeriesElItemColorsElColorsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ChronicleDashboardChartDashboardChartElVisualizationElSeriesElItemColorsElColorsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]    pub fn value (& self) -> ListRef < ChronicleDashboardChartDashboardChartElVisualizationElSeriesElItemColorsElColorsElValueElRef >{
        ListRef::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize, Default)]
struct ChronicleDashboardChartDashboardChartElVisualizationElSeriesElItemColorsElDynamic {
    colors: Option<
        DynamicBlock<
            ChronicleDashboardChartDashboardChartElVisualizationElSeriesElItemColorsElColorsEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct ChronicleDashboardChartDashboardChartElVisualizationElSeriesElItemColorsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    colors: Option<
        Vec<ChronicleDashboardChartDashboardChartElVisualizationElSeriesElItemColorsElColorsEl>,
    >,
    dynamic: ChronicleDashboardChartDashboardChartElVisualizationElSeriesElItemColorsElDynamic,
}
impl ChronicleDashboardChartDashboardChartElVisualizationElSeriesElItemColorsEl {
    #[doc = "Set the field `colors`.\n"]
    pub fn set_colors(
        mut self,
        v: impl Into<
            BlockAssignable<
                ChronicleDashboardChartDashboardChartElVisualizationElSeriesElItemColorsElColorsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.colors = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.colors = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ChronicleDashboardChartDashboardChartElVisualizationElSeriesElItemColorsEl {
    type O =
        BlockAssignable<ChronicleDashboardChartDashboardChartElVisualizationElSeriesElItemColorsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildChronicleDashboardChartDashboardChartElVisualizationElSeriesElItemColorsEl {}
impl BuildChronicleDashboardChartDashboardChartElVisualizationElSeriesElItemColorsEl {
    pub fn build(
        self,
    ) -> ChronicleDashboardChartDashboardChartElVisualizationElSeriesElItemColorsEl {
        ChronicleDashboardChartDashboardChartElVisualizationElSeriesElItemColorsEl {
            colors: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ChronicleDashboardChartDashboardChartElVisualizationElSeriesElItemColorsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ChronicleDashboardChartDashboardChartElVisualizationElSeriesElItemColorsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ChronicleDashboardChartDashboardChartElVisualizationElSeriesElItemColorsElRef {
        ChronicleDashboardChartDashboardChartElVisualizationElSeriesElItemColorsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ChronicleDashboardChartDashboardChartElVisualizationElSeriesElItemColorsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `colors` after provisioning.\n"]
    pub fn colors(
        &self,
    ) -> ListRef<
        ChronicleDashboardChartDashboardChartElVisualizationElSeriesElItemColorsElColorsElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.colors", self.base))
    }
}
#[derive(Serialize)]
pub struct ChronicleDashboardChartDashboardChartElVisualizationElSeriesElItemStyleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    border_color: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    border_width: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<PrimField<String>>,
}
impl ChronicleDashboardChartDashboardChartElVisualizationElSeriesElItemStyleEl {
    #[doc = "Set the field `border_color`.\n"]
    pub fn set_border_color(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.border_color = Some(v.into());
        self
    }
    #[doc = "Set the field `border_width`.\n"]
    pub fn set_border_width(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.border_width = Some(v.into());
        self
    }
    #[doc = "Set the field `color`.\n"]
    pub fn set_color(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.color = Some(v.into());
        self
    }
}
impl ToListMappable for ChronicleDashboardChartDashboardChartElVisualizationElSeriesElItemStyleEl {
    type O =
        BlockAssignable<ChronicleDashboardChartDashboardChartElVisualizationElSeriesElItemStyleEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildChronicleDashboardChartDashboardChartElVisualizationElSeriesElItemStyleEl {}
impl BuildChronicleDashboardChartDashboardChartElVisualizationElSeriesElItemStyleEl {
    pub fn build(
        self,
    ) -> ChronicleDashboardChartDashboardChartElVisualizationElSeriesElItemStyleEl {
        ChronicleDashboardChartDashboardChartElVisualizationElSeriesElItemStyleEl {
            border_color: core::default::Default::default(),
            border_width: core::default::Default::default(),
            color: core::default::Default::default(),
        }
    }
}
pub struct ChronicleDashboardChartDashboardChartElVisualizationElSeriesElItemStyleElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ChronicleDashboardChartDashboardChartElVisualizationElSeriesElItemStyleElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ChronicleDashboardChartDashboardChartElVisualizationElSeriesElItemStyleElRef {
        ChronicleDashboardChartDashboardChartElVisualizationElSeriesElItemStyleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ChronicleDashboardChartDashboardChartElVisualizationElSeriesElItemStyleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `border_color` after provisioning.\n"]
    pub fn border_color(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.border_color", self.base))
    }
    #[doc = "Get a reference to the value of field `border_width` after provisioning.\n"]
    pub fn border_width(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.border_width", self.base))
    }
    #[doc = "Get a reference to the value of field `color` after provisioning.\n"]
    pub fn color(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.color", self.base))
    }
}
#[derive(Serialize)]
pub struct ChronicleDashboardChartDashboardChartElVisualizationElSeriesElMetricTrendConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    metric_display_trend: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metric_format: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metric_trend_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    show_metric_trend: Option<PrimField<bool>>,
}
impl ChronicleDashboardChartDashboardChartElVisualizationElSeriesElMetricTrendConfigEl {
    #[doc = "Set the field `metric_display_trend`.\n Possible values: [\"METRIC_DISPLAY_TREND_UNSPECIFIED\", \"METRIC_DISPLAY_TREND_ABSOLUTE_VALUE\", \"METRIC_DISPLAY_TREND_PERCENTAGE\", \"METRIC_DISPLAY_TREND_ABSOLUTE_VALUE_AND_PERCENTAGE\"]"]
    pub fn set_metric_display_trend(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.metric_display_trend = Some(v.into());
        self
    }
    #[doc = "Set the field `metric_format`.\n Possible values: [\"METRIC_FORMAT_UNSPECIFIED\", \"METRIC_FORMAT_NUMBER\", \"METRIC_FORMAT_PLAIN_TEXT\"]"]
    pub fn set_metric_format(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.metric_format = Some(v.into());
        self
    }
    #[doc = "Set the field `metric_trend_type`.\n Possible values: [\"METRIC_TREND_TYPE_UNSPECIFIED\", \"METRIC_TREND_TYPE_REGULAR\", \"METRIC_TREND_TYPE_INVERSE\"]"]
    pub fn set_metric_trend_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.metric_trend_type = Some(v.into());
        self
    }
    #[doc = "Set the field `show_metric_trend`.\n"]
    pub fn set_show_metric_trend(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.show_metric_trend = Some(v.into());
        self
    }
}
impl ToListMappable
    for ChronicleDashboardChartDashboardChartElVisualizationElSeriesElMetricTrendConfigEl
{
    type O = BlockAssignable<
        ChronicleDashboardChartDashboardChartElVisualizationElSeriesElMetricTrendConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildChronicleDashboardChartDashboardChartElVisualizationElSeriesElMetricTrendConfigEl {
}
impl BuildChronicleDashboardChartDashboardChartElVisualizationElSeriesElMetricTrendConfigEl {
    pub fn build(
        self,
    ) -> ChronicleDashboardChartDashboardChartElVisualizationElSeriesElMetricTrendConfigEl {
        ChronicleDashboardChartDashboardChartElVisualizationElSeriesElMetricTrendConfigEl {
            metric_display_trend: core::default::Default::default(),
            metric_format: core::default::Default::default(),
            metric_trend_type: core::default::Default::default(),
            show_metric_trend: core::default::Default::default(),
        }
    }
}
pub struct ChronicleDashboardChartDashboardChartElVisualizationElSeriesElMetricTrendConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ChronicleDashboardChartDashboardChartElVisualizationElSeriesElMetricTrendConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ChronicleDashboardChartDashboardChartElVisualizationElSeriesElMetricTrendConfigElRef {
        ChronicleDashboardChartDashboardChartElVisualizationElSeriesElMetricTrendConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ChronicleDashboardChartDashboardChartElVisualizationElSeriesElMetricTrendConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `metric_display_trend` after provisioning.\n Possible values: [\"METRIC_DISPLAY_TREND_UNSPECIFIED\", \"METRIC_DISPLAY_TREND_ABSOLUTE_VALUE\", \"METRIC_DISPLAY_TREND_PERCENTAGE\", \"METRIC_DISPLAY_TREND_ABSOLUTE_VALUE_AND_PERCENTAGE\"]"]
    pub fn metric_display_trend(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.metric_display_trend", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `metric_format` after provisioning.\n Possible values: [\"METRIC_FORMAT_UNSPECIFIED\", \"METRIC_FORMAT_NUMBER\", \"METRIC_FORMAT_PLAIN_TEXT\"]"]
    pub fn metric_format(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.metric_format", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `metric_trend_type` after provisioning.\n Possible values: [\"METRIC_TREND_TYPE_UNSPECIFIED\", \"METRIC_TREND_TYPE_REGULAR\", \"METRIC_TREND_TYPE_INVERSE\"]"]
    pub fn metric_trend_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.metric_trend_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `show_metric_trend` after provisioning.\n"]
    pub fn show_metric_trend(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.show_metric_trend", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ChronicleDashboardChartDashboardChartElVisualizationElSeriesElDynamic {
    area_style: Option<
        DynamicBlock<ChronicleDashboardChartDashboardChartElVisualizationElSeriesElAreaStyleEl>,
    >,
    data_label: Option<
        DynamicBlock<ChronicleDashboardChartDashboardChartElVisualizationElSeriesElDataLabelEl>,
    >,
    encode: Option<
        DynamicBlock<ChronicleDashboardChartDashboardChartElVisualizationElSeriesElEncodeEl>,
    >,
    gauge_config: Option<
        DynamicBlock<ChronicleDashboardChartDashboardChartElVisualizationElSeriesElGaugeConfigEl>,
    >,
    item_colors: Option<
        DynamicBlock<ChronicleDashboardChartDashboardChartElVisualizationElSeriesElItemColorsEl>,
    >,
    item_style: Option<
        DynamicBlock<ChronicleDashboardChartDashboardChartElVisualizationElSeriesElItemStyleEl>,
    >,
    metric_trend_config: Option<
        DynamicBlock<
            ChronicleDashboardChartDashboardChartElVisualizationElSeriesElMetricTrendConfigEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct ChronicleDashboardChartDashboardChartElVisualizationElSeriesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    field: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    radius: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    series_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    series_stack_strategy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    series_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    series_unique_value: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    show_background: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    show_symbol: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stack: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    area_style:
        Option<Vec<ChronicleDashboardChartDashboardChartElVisualizationElSeriesElAreaStyleEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_label:
        Option<Vec<ChronicleDashboardChartDashboardChartElVisualizationElSeriesElDataLabelEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encode: Option<Vec<ChronicleDashboardChartDashboardChartElVisualizationElSeriesElEncodeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gauge_config:
        Option<Vec<ChronicleDashboardChartDashboardChartElVisualizationElSeriesElGaugeConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    item_colors:
        Option<Vec<ChronicleDashboardChartDashboardChartElVisualizationElSeriesElItemColorsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    item_style:
        Option<Vec<ChronicleDashboardChartDashboardChartElVisualizationElSeriesElItemStyleEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metric_trend_config: Option<
        Vec<ChronicleDashboardChartDashboardChartElVisualizationElSeriesElMetricTrendConfigEl>,
    >,
    dynamic: ChronicleDashboardChartDashboardChartElVisualizationElSeriesElDynamic,
}
impl ChronicleDashboardChartDashboardChartElVisualizationElSeriesEl {
    #[doc = "Set the field `field`.\n"]
    pub fn set_field(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.field = Some(v.into());
        self
    }
    #[doc = "Set the field `label`.\n"]
    pub fn set_label(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.label = Some(v.into());
        self
    }
    #[doc = "Set the field `radius`.\n"]
    pub fn set_radius(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.radius = Some(v.into());
        self
    }
    #[doc = "Set the field `series_name`.\nUser specified series label."]
    pub fn set_series_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.series_name = Some(v.into());
        self
    }
    #[doc = "Set the field `series_stack_strategy`.\n Possible values: [\"SAMESIGN\", \"ALL\", \"POSITIVE\", \"NEGATIVE\"]"]
    pub fn set_series_stack_strategy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.series_stack_strategy = Some(v.into());
        self
    }
    #[doc = "Set the field `series_type`.\n Possible values: [\"LINE\", \"BAR\", \"PIE\", \"TEXT\", \"MAP\", \"GAUGE\", \"SCATTERPLOT\"]"]
    pub fn set_series_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.series_type = Some(v.into());
        self
    }
    #[doc = "Set the field `series_unique_value`.\n"]
    pub fn set_series_unique_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.series_unique_value = Some(v.into());
        self
    }
    #[doc = "Set the field `show_background`.\n"]
    pub fn set_show_background(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.show_background = Some(v.into());
        self
    }
    #[doc = "Set the field `show_symbol`.\n"]
    pub fn set_show_symbol(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.show_symbol = Some(v.into());
        self
    }
    #[doc = "Set the field `stack`.\n"]
    pub fn set_stack(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.stack = Some(v.into());
        self
    }
    #[doc = "Set the field `area_style`.\n"]
    pub fn set_area_style(
        mut self,
        v: impl Into<
            BlockAssignable<
                ChronicleDashboardChartDashboardChartElVisualizationElSeriesElAreaStyleEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.area_style = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.area_style = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `data_label`.\n"]
    pub fn set_data_label(
        mut self,
        v: impl Into<
            BlockAssignable<
                ChronicleDashboardChartDashboardChartElVisualizationElSeriesElDataLabelEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.data_label = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.data_label = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `encode`.\n"]
    pub fn set_encode(
        mut self,
        v: impl Into<
            BlockAssignable<ChronicleDashboardChartDashboardChartElVisualizationElSeriesElEncodeEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.encode = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.encode = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `gauge_config`.\n"]
    pub fn set_gauge_config(
        mut self,
        v: impl Into<
            BlockAssignable<
                ChronicleDashboardChartDashboardChartElVisualizationElSeriesElGaugeConfigEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.gauge_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.gauge_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `item_colors`.\n"]
    pub fn set_item_colors(
        mut self,
        v: impl Into<
            BlockAssignable<
                ChronicleDashboardChartDashboardChartElVisualizationElSeriesElItemColorsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.item_colors = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.item_colors = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `item_style`.\n"]
    pub fn set_item_style(
        mut self,
        v: impl Into<
            BlockAssignable<
                ChronicleDashboardChartDashboardChartElVisualizationElSeriesElItemStyleEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.item_style = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.item_style = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `metric_trend_config`.\n"]
    pub fn set_metric_trend_config(
        mut self,
        v: impl Into<
            BlockAssignable<
                ChronicleDashboardChartDashboardChartElVisualizationElSeriesElMetricTrendConfigEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.metric_trend_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.metric_trend_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ChronicleDashboardChartDashboardChartElVisualizationElSeriesEl {
    type O = BlockAssignable<ChronicleDashboardChartDashboardChartElVisualizationElSeriesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildChronicleDashboardChartDashboardChartElVisualizationElSeriesEl {}
impl BuildChronicleDashboardChartDashboardChartElVisualizationElSeriesEl {
    pub fn build(self) -> ChronicleDashboardChartDashboardChartElVisualizationElSeriesEl {
        ChronicleDashboardChartDashboardChartElVisualizationElSeriesEl {
            field: core::default::Default::default(),
            label: core::default::Default::default(),
            radius: core::default::Default::default(),
            series_name: core::default::Default::default(),
            series_stack_strategy: core::default::Default::default(),
            series_type: core::default::Default::default(),
            series_unique_value: core::default::Default::default(),
            show_background: core::default::Default::default(),
            show_symbol: core::default::Default::default(),
            stack: core::default::Default::default(),
            area_style: core::default::Default::default(),
            data_label: core::default::Default::default(),
            encode: core::default::Default::default(),
            gauge_config: core::default::Default::default(),
            item_colors: core::default::Default::default(),
            item_style: core::default::Default::default(),
            metric_trend_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ChronicleDashboardChartDashboardChartElVisualizationElSeriesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ChronicleDashboardChartDashboardChartElVisualizationElSeriesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ChronicleDashboardChartDashboardChartElVisualizationElSeriesElRef {
        ChronicleDashboardChartDashboardChartElVisualizationElSeriesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ChronicleDashboardChartDashboardChartElVisualizationElSeriesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `field` after provisioning.\n"]
    pub fn field(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.field", self.base))
    }
    #[doc = "Get a reference to the value of field `label` after provisioning.\n"]
    pub fn label(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.label", self.base))
    }
    #[doc = "Get a reference to the value of field `radius` after provisioning.\n"]
    pub fn radius(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.radius", self.base))
    }
    #[doc = "Get a reference to the value of field `series_name` after provisioning.\nUser specified series label."]
    pub fn series_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.series_name", self.base))
    }
    #[doc = "Get a reference to the value of field `series_stack_strategy` after provisioning.\n Possible values: [\"SAMESIGN\", \"ALL\", \"POSITIVE\", \"NEGATIVE\"]"]
    pub fn series_stack_strategy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.series_stack_strategy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `series_type` after provisioning.\n Possible values: [\"LINE\", \"BAR\", \"PIE\", \"TEXT\", \"MAP\", \"GAUGE\", \"SCATTERPLOT\"]"]
    pub fn series_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.series_type", self.base))
    }
    #[doc = "Get a reference to the value of field `series_unique_value` after provisioning.\n"]
    pub fn series_unique_value(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.series_unique_value", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `show_background` after provisioning.\n"]
    pub fn show_background(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.show_background", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `show_symbol` after provisioning.\n"]
    pub fn show_symbol(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.show_symbol", self.base))
    }
    #[doc = "Get a reference to the value of field `stack` after provisioning.\n"]
    pub fn stack(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.stack", self.base))
    }
    #[doc = "Get a reference to the value of field `area_style` after provisioning.\n"]
    pub fn area_style(
        &self,
    ) -> ListRef<ChronicleDashboardChartDashboardChartElVisualizationElSeriesElAreaStyleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.area_style", self.base))
    }
    #[doc = "Get a reference to the value of field `data_label` after provisioning.\n"]
    pub fn data_label(
        &self,
    ) -> ListRef<ChronicleDashboardChartDashboardChartElVisualizationElSeriesElDataLabelElRef> {
        ListRef::new(self.shared().clone(), format!("{}.data_label", self.base))
    }
    #[doc = "Get a reference to the value of field `encode` after provisioning.\n"]
    pub fn encode(
        &self,
    ) -> ListRef<ChronicleDashboardChartDashboardChartElVisualizationElSeriesElEncodeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.encode", self.base))
    }
    #[doc = "Get a reference to the value of field `gauge_config` after provisioning.\n"]
    pub fn gauge_config(
        &self,
    ) -> ListRef<ChronicleDashboardChartDashboardChartElVisualizationElSeriesElGaugeConfigElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.gauge_config", self.base))
    }
    #[doc = "Get a reference to the value of field `item_colors` after provisioning.\n"]
    pub fn item_colors(
        &self,
    ) -> ListRef<ChronicleDashboardChartDashboardChartElVisualizationElSeriesElItemColorsElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.item_colors", self.base))
    }
    #[doc = "Get a reference to the value of field `item_style` after provisioning.\n"]
    pub fn item_style(
        &self,
    ) -> ListRef<ChronicleDashboardChartDashboardChartElVisualizationElSeriesElItemStyleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.item_style", self.base))
    }
    #[doc = "Get a reference to the value of field `metric_trend_config` after provisioning.\n"]
    pub fn metric_trend_config(
        &self,
    ) -> ListRef<ChronicleDashboardChartDashboardChartElVisualizationElSeriesElMetricTrendConfigElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.metric_trend_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ChronicleDashboardChartDashboardChartElVisualizationElTableConfigElColumnRenderTypeSettingsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    column_render_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    field: Option<PrimField<String>>,
}
impl ChronicleDashboardChartDashboardChartElVisualizationElTableConfigElColumnRenderTypeSettingsEl {
    #[doc = "Set the field `column_render_type`.\n Possible values: [\"RENDER_TYPE_UNSPECIFIED\", \"RENDER_TYPE_TEXT\", \"RENDER_TYPE_ICON\", \"RENDER_TYPE_ICON_AND_TEXT\"]"]
    pub fn set_column_render_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.column_render_type = Some(v.into());
        self
    }
    #[doc = "Set the field `field`.\n"]
    pub fn set_field(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.field = Some(v.into());
        self
    }
}
impl ToListMappable for ChronicleDashboardChartDashboardChartElVisualizationElTableConfigElColumnRenderTypeSettingsEl { type O = BlockAssignable < ChronicleDashboardChartDashboardChartElVisualizationElTableConfigElColumnRenderTypeSettingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildChronicleDashboardChartDashboardChartElVisualizationElTableConfigElColumnRenderTypeSettingsEl
{}
impl BuildChronicleDashboardChartDashboardChartElVisualizationElTableConfigElColumnRenderTypeSettingsEl { pub fn build (self) -> ChronicleDashboardChartDashboardChartElVisualizationElTableConfigElColumnRenderTypeSettingsEl { ChronicleDashboardChartDashboardChartElVisualizationElTableConfigElColumnRenderTypeSettingsEl { column_render_type : core :: default :: Default :: default () , field : core :: default :: Default :: default () , } } }
pub struct ChronicleDashboardChartDashboardChartElVisualizationElTableConfigElColumnRenderTypeSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for ChronicleDashboardChartDashboardChartElVisualizationElTableConfigElColumnRenderTypeSettingsElRef { fn new (shared : StackShared , base : String) -> ChronicleDashboardChartDashboardChartElVisualizationElTableConfigElColumnRenderTypeSettingsElRef { ChronicleDashboardChartDashboardChartElVisualizationElTableConfigElColumnRenderTypeSettingsElRef { shared : shared , base : base . to_string () , } } }
impl
    ChronicleDashboardChartDashboardChartElVisualizationElTableConfigElColumnRenderTypeSettingsElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `column_render_type` after provisioning.\n Possible values: [\"RENDER_TYPE_UNSPECIFIED\", \"RENDER_TYPE_TEXT\", \"RENDER_TYPE_ICON\", \"RENDER_TYPE_ICON_AND_TEXT\"]"]
    pub fn column_render_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.column_render_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `field` after provisioning.\n"]
    pub fn field(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.field", self.base))
    }
}
#[derive(Serialize)]
pub struct ChronicleDashboardChartDashboardChartElVisualizationElTableConfigElColumnTooltipSettingsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    cell_tooltip_text: Option<PrimField<String>>,
    field: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    header_tooltip_text: Option<PrimField<String>>,
}
impl ChronicleDashboardChartDashboardChartElVisualizationElTableConfigElColumnTooltipSettingsEl {
    #[doc = "Set the field `cell_tooltip_text`.\n"]
    pub fn set_cell_tooltip_text(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cell_tooltip_text = Some(v.into());
        self
    }
    #[doc = "Set the field `header_tooltip_text`.\n"]
    pub fn set_header_tooltip_text(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.header_tooltip_text = Some(v.into());
        self
    }
}
impl ToListMappable
    for ChronicleDashboardChartDashboardChartElVisualizationElTableConfigElColumnTooltipSettingsEl
{
    type O = BlockAssignable<
        ChronicleDashboardChartDashboardChartElVisualizationElTableConfigElColumnTooltipSettingsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildChronicleDashboardChartDashboardChartElVisualizationElTableConfigElColumnTooltipSettingsEl
{
    #[doc = ""]
    pub field: PrimField<String>,
}
impl
    BuildChronicleDashboardChartDashboardChartElVisualizationElTableConfigElColumnTooltipSettingsEl
{
    pub fn build(
        self,
    ) -> ChronicleDashboardChartDashboardChartElVisualizationElTableConfigElColumnTooltipSettingsEl
    {
        ChronicleDashboardChartDashboardChartElVisualizationElTableConfigElColumnTooltipSettingsEl {
            cell_tooltip_text: core::default::Default::default(),
            field: self.field,
            header_tooltip_text: core::default::Default::default(),
        }
    }
}
pub struct ChronicleDashboardChartDashboardChartElVisualizationElTableConfigElColumnTooltipSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for ChronicleDashboardChartDashboardChartElVisualizationElTableConfigElColumnTooltipSettingsElRef { fn new (shared : StackShared , base : String) -> ChronicleDashboardChartDashboardChartElVisualizationElTableConfigElColumnTooltipSettingsElRef { ChronicleDashboardChartDashboardChartElVisualizationElTableConfigElColumnTooltipSettingsElRef { shared : shared , base : base . to_string () , } } }
impl ChronicleDashboardChartDashboardChartElVisualizationElTableConfigElColumnTooltipSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cell_tooltip_text` after provisioning.\n"]
    pub fn cell_tooltip_text(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cell_tooltip_text", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `field` after provisioning.\n"]
    pub fn field(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.field", self.base))
    }
    #[doc = "Get a reference to the value of field `header_tooltip_text` after provisioning.\n"]
    pub fn header_tooltip_text(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.header_tooltip_text", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ChronicleDashboardChartDashboardChartElVisualizationElTableConfigElDynamic { column_render_type_settings : Option < DynamicBlock < ChronicleDashboardChartDashboardChartElVisualizationElTableConfigElColumnRenderTypeSettingsEl >> , column_tooltip_settings : Option < DynamicBlock < ChronicleDashboardChartDashboardChartElVisualizationElTableConfigElColumnTooltipSettingsEl >> , }
#[derive(Serialize)]
pub struct ChronicleDashboardChartDashboardChartElVisualizationElTableConfigEl { # [serde (skip_serializing_if = "Option::is_none")] enable_text_wrap : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] column_render_type_settings : Option < Vec < ChronicleDashboardChartDashboardChartElVisualizationElTableConfigElColumnRenderTypeSettingsEl > > , # [serde (skip_serializing_if = "Option::is_none")] column_tooltip_settings : Option < Vec < ChronicleDashboardChartDashboardChartElVisualizationElTableConfigElColumnTooltipSettingsEl > > , dynamic : ChronicleDashboardChartDashboardChartElVisualizationElTableConfigElDynamic , }
impl ChronicleDashboardChartDashboardChartElVisualizationElTableConfigEl {
    #[doc = "Set the field `enable_text_wrap`.\n"]
    pub fn set_enable_text_wrap(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_text_wrap = Some(v.into());
        self
    }
    #[doc = "Set the field `column_render_type_settings`.\n"]
    pub fn set_column_render_type_settings(
        mut self,
        v : impl Into < BlockAssignable < ChronicleDashboardChartDashboardChartElVisualizationElTableConfigElColumnRenderTypeSettingsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.column_render_type_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.column_render_type_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `column_tooltip_settings`.\n"]
    pub fn set_column_tooltip_settings(
        mut self,
        v : impl Into < BlockAssignable < ChronicleDashboardChartDashboardChartElVisualizationElTableConfigElColumnTooltipSettingsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.column_tooltip_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.column_tooltip_settings = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ChronicleDashboardChartDashboardChartElVisualizationElTableConfigEl {
    type O = BlockAssignable<ChronicleDashboardChartDashboardChartElVisualizationElTableConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildChronicleDashboardChartDashboardChartElVisualizationElTableConfigEl {}
impl BuildChronicleDashboardChartDashboardChartElVisualizationElTableConfigEl {
    pub fn build(self) -> ChronicleDashboardChartDashboardChartElVisualizationElTableConfigEl {
        ChronicleDashboardChartDashboardChartElVisualizationElTableConfigEl {
            enable_text_wrap: core::default::Default::default(),
            column_render_type_settings: core::default::Default::default(),
            column_tooltip_settings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ChronicleDashboardChartDashboardChartElVisualizationElTableConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ChronicleDashboardChartDashboardChartElVisualizationElTableConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ChronicleDashboardChartDashboardChartElVisualizationElTableConfigElRef {
        ChronicleDashboardChartDashboardChartElVisualizationElTableConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ChronicleDashboardChartDashboardChartElVisualizationElTableConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enable_text_wrap` after provisioning.\n"]
    pub fn enable_text_wrap(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_text_wrap", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `column_render_type_settings` after provisioning.\n"]    pub fn column_render_type_settings (& self) -> ListRef < ChronicleDashboardChartDashboardChartElVisualizationElTableConfigElColumnRenderTypeSettingsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.column_render_type_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `column_tooltip_settings` after provisioning.\n"]    pub fn column_tooltip_settings (& self) -> ListRef < ChronicleDashboardChartDashboardChartElVisualizationElTableConfigElColumnTooltipSettingsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.column_tooltip_settings", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ChronicleDashboardChartDashboardChartElVisualizationElTooltipEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    show: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tooltip_trigger: Option<PrimField<String>>,
}
impl ChronicleDashboardChartDashboardChartElVisualizationElTooltipEl {
    #[doc = "Set the field `show`.\n"]
    pub fn set_show(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.show = Some(v.into());
        self
    }
    #[doc = "Set the field `tooltip_trigger`.\n Possible values: [\"TOOLTIP_TRIGGER_UNSPECIFIED\", \"TOOLTIP_TRIGGER_NONE\", \"TOOLTIP_TRIGGER_ITEM\", \"TOOLTIP_TRIGGER_AXIS\"]"]
    pub fn set_tooltip_trigger(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tooltip_trigger = Some(v.into());
        self
    }
}
impl ToListMappable for ChronicleDashboardChartDashboardChartElVisualizationElTooltipEl {
    type O = BlockAssignable<ChronicleDashboardChartDashboardChartElVisualizationElTooltipEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildChronicleDashboardChartDashboardChartElVisualizationElTooltipEl {}
impl BuildChronicleDashboardChartDashboardChartElVisualizationElTooltipEl {
    pub fn build(self) -> ChronicleDashboardChartDashboardChartElVisualizationElTooltipEl {
        ChronicleDashboardChartDashboardChartElVisualizationElTooltipEl {
            show: core::default::Default::default(),
            tooltip_trigger: core::default::Default::default(),
        }
    }
}
pub struct ChronicleDashboardChartDashboardChartElVisualizationElTooltipElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ChronicleDashboardChartDashboardChartElVisualizationElTooltipElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ChronicleDashboardChartDashboardChartElVisualizationElTooltipElRef {
        ChronicleDashboardChartDashboardChartElVisualizationElTooltipElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ChronicleDashboardChartDashboardChartElVisualizationElTooltipElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `show` after provisioning.\n"]
    pub fn show(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.show", self.base))
    }
    #[doc = "Get a reference to the value of field `tooltip_trigger` after provisioning.\n Possible values: [\"TOOLTIP_TRIGGER_UNSPECIFIED\", \"TOOLTIP_TRIGGER_NONE\", \"TOOLTIP_TRIGGER_ITEM\", \"TOOLTIP_TRIGGER_AXIS\"]"]
    pub fn tooltip_trigger(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.tooltip_trigger", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ChronicleDashboardChartDashboardChartElVisualizationElVisualMapsElPiecesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<PrimField<f64>>,
}
impl ChronicleDashboardChartDashboardChartElVisualizationElVisualMapsElPiecesEl {
    #[doc = "Set the field `color`.\n"]
    pub fn set_color(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.color = Some(v.into());
        self
    }
    #[doc = "Set the field `label`.\n"]
    pub fn set_label(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.label = Some(v.into());
        self
    }
    #[doc = "Set the field `max`.\n"]
    pub fn set_max(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max = Some(v.into());
        self
    }
    #[doc = "Set the field `min`.\n"]
    pub fn set_min(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min = Some(v.into());
        self
    }
}
impl ToListMappable for ChronicleDashboardChartDashboardChartElVisualizationElVisualMapsElPiecesEl {
    type O =
        BlockAssignable<ChronicleDashboardChartDashboardChartElVisualizationElVisualMapsElPiecesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildChronicleDashboardChartDashboardChartElVisualizationElVisualMapsElPiecesEl {}
impl BuildChronicleDashboardChartDashboardChartElVisualizationElVisualMapsElPiecesEl {
    pub fn build(
        self,
    ) -> ChronicleDashboardChartDashboardChartElVisualizationElVisualMapsElPiecesEl {
        ChronicleDashboardChartDashboardChartElVisualizationElVisualMapsElPiecesEl {
            color: core::default::Default::default(),
            label: core::default::Default::default(),
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}
pub struct ChronicleDashboardChartDashboardChartElVisualizationElVisualMapsElPiecesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ChronicleDashboardChartDashboardChartElVisualizationElVisualMapsElPiecesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ChronicleDashboardChartDashboardChartElVisualizationElVisualMapsElPiecesElRef {
        ChronicleDashboardChartDashboardChartElVisualizationElVisualMapsElPiecesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ChronicleDashboardChartDashboardChartElVisualizationElVisualMapsElPiecesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `color` after provisioning.\n"]
    pub fn color(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.color", self.base))
    }
    #[doc = "Get a reference to the value of field `label` after provisioning.\n"]
    pub fn label(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.label", self.base))
    }
    #[doc = "Get a reference to the value of field `max` after provisioning.\n"]
    pub fn max(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max", self.base))
    }
    #[doc = "Get a reference to the value of field `min` after provisioning.\n"]
    pub fn min(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min", self.base))
    }
}
#[derive(Serialize, Default)]
struct ChronicleDashboardChartDashboardChartElVisualizationElVisualMapsElDynamic {
    pieces: Option<
        DynamicBlock<ChronicleDashboardChartDashboardChartElVisualizationElVisualMapsElPiecesEl>,
    >,
}
#[derive(Serialize)]
pub struct ChronicleDashboardChartDashboardChartElVisualizationElVisualMapsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    visual_map_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pieces: Option<Vec<ChronicleDashboardChartDashboardChartElVisualizationElVisualMapsElPiecesEl>>,
    dynamic: ChronicleDashboardChartDashboardChartElVisualizationElVisualMapsElDynamic,
}
impl ChronicleDashboardChartDashboardChartElVisualizationElVisualMapsEl {
    #[doc = "Set the field `visual_map_type`.\n Possible values: [\"VISUAL_MAP_TYPE_UNSPECIFIED\", \"CONTINUOUS\", \"PIECEWISE\"]"]
    pub fn set_visual_map_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.visual_map_type = Some(v.into());
        self
    }
    #[doc = "Set the field `pieces`.\n"]
    pub fn set_pieces(
        mut self,
        v: impl Into<
            BlockAssignable<
                ChronicleDashboardChartDashboardChartElVisualizationElVisualMapsElPiecesEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.pieces = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.pieces = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ChronicleDashboardChartDashboardChartElVisualizationElVisualMapsEl {
    type O = BlockAssignable<ChronicleDashboardChartDashboardChartElVisualizationElVisualMapsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildChronicleDashboardChartDashboardChartElVisualizationElVisualMapsEl {}
impl BuildChronicleDashboardChartDashboardChartElVisualizationElVisualMapsEl {
    pub fn build(self) -> ChronicleDashboardChartDashboardChartElVisualizationElVisualMapsEl {
        ChronicleDashboardChartDashboardChartElVisualizationElVisualMapsEl {
            visual_map_type: core::default::Default::default(),
            pieces: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ChronicleDashboardChartDashboardChartElVisualizationElVisualMapsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ChronicleDashboardChartDashboardChartElVisualizationElVisualMapsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ChronicleDashboardChartDashboardChartElVisualizationElVisualMapsElRef {
        ChronicleDashboardChartDashboardChartElVisualizationElVisualMapsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ChronicleDashboardChartDashboardChartElVisualizationElVisualMapsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `visual_map_type` after provisioning.\n Possible values: [\"VISUAL_MAP_TYPE_UNSPECIFIED\", \"CONTINUOUS\", \"PIECEWISE\"]"]
    pub fn visual_map_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.visual_map_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `pieces` after provisioning.\n"]
    pub fn pieces(
        &self,
    ) -> ListRef<ChronicleDashboardChartDashboardChartElVisualizationElVisualMapsElPiecesElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.pieces", self.base))
    }
}
#[derive(Serialize)]
pub struct ChronicleDashboardChartDashboardChartElVisualizationElXAxesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    axis_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<PrimField<f64>>,
}
impl ChronicleDashboardChartDashboardChartElVisualizationElXAxesEl {
    #[doc = "Set the field `axis_type`.\n Possible values: [\"VALUE\", \"CATEGORY\", \"TIME\", \"LOG\"]"]
    pub fn set_axis_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.axis_type = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\n"]
    pub fn set_display_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `max`.\n"]
    pub fn set_max(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max = Some(v.into());
        self
    }
    #[doc = "Set the field `min`.\n"]
    pub fn set_min(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min = Some(v.into());
        self
    }
}
impl ToListMappable for ChronicleDashboardChartDashboardChartElVisualizationElXAxesEl {
    type O = BlockAssignable<ChronicleDashboardChartDashboardChartElVisualizationElXAxesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildChronicleDashboardChartDashboardChartElVisualizationElXAxesEl {}
impl BuildChronicleDashboardChartDashboardChartElVisualizationElXAxesEl {
    pub fn build(self) -> ChronicleDashboardChartDashboardChartElVisualizationElXAxesEl {
        ChronicleDashboardChartDashboardChartElVisualizationElXAxesEl {
            axis_type: core::default::Default::default(),
            display_name: core::default::Default::default(),
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}
pub struct ChronicleDashboardChartDashboardChartElVisualizationElXAxesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ChronicleDashboardChartDashboardChartElVisualizationElXAxesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ChronicleDashboardChartDashboardChartElVisualizationElXAxesElRef {
        ChronicleDashboardChartDashboardChartElVisualizationElXAxesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ChronicleDashboardChartDashboardChartElVisualizationElXAxesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `axis_type` after provisioning.\n Possible values: [\"VALUE\", \"CATEGORY\", \"TIME\", \"LOG\"]"]
    pub fn axis_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.axis_type", self.base))
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
    #[doc = "Get a reference to the value of field `max` after provisioning.\n"]
    pub fn max(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max", self.base))
    }
    #[doc = "Get a reference to the value of field `min` after provisioning.\n"]
    pub fn min(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min", self.base))
    }
}
#[derive(Serialize)]
pub struct ChronicleDashboardChartDashboardChartElVisualizationElYAxesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    axis_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<PrimField<f64>>,
}
impl ChronicleDashboardChartDashboardChartElVisualizationElYAxesEl {
    #[doc = "Set the field `axis_type`.\n Possible values: [\"VALUE\", \"CATEGORY\", \"TIME\", \"LOG\"]"]
    pub fn set_axis_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.axis_type = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\n"]
    pub fn set_display_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `max`.\n"]
    pub fn set_max(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max = Some(v.into());
        self
    }
    #[doc = "Set the field `min`.\n"]
    pub fn set_min(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min = Some(v.into());
        self
    }
}
impl ToListMappable for ChronicleDashboardChartDashboardChartElVisualizationElYAxesEl {
    type O = BlockAssignable<ChronicleDashboardChartDashboardChartElVisualizationElYAxesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildChronicleDashboardChartDashboardChartElVisualizationElYAxesEl {}
impl BuildChronicleDashboardChartDashboardChartElVisualizationElYAxesEl {
    pub fn build(self) -> ChronicleDashboardChartDashboardChartElVisualizationElYAxesEl {
        ChronicleDashboardChartDashboardChartElVisualizationElYAxesEl {
            axis_type: core::default::Default::default(),
            display_name: core::default::Default::default(),
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}
pub struct ChronicleDashboardChartDashboardChartElVisualizationElYAxesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ChronicleDashboardChartDashboardChartElVisualizationElYAxesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ChronicleDashboardChartDashboardChartElVisualizationElYAxesElRef {
        ChronicleDashboardChartDashboardChartElVisualizationElYAxesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ChronicleDashboardChartDashboardChartElVisualizationElYAxesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `axis_type` after provisioning.\n Possible values: [\"VALUE\", \"CATEGORY\", \"TIME\", \"LOG\"]"]
    pub fn axis_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.axis_type", self.base))
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
    #[doc = "Get a reference to the value of field `max` after provisioning.\n"]
    pub fn max(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max", self.base))
    }
    #[doc = "Get a reference to the value of field `min` after provisioning.\n"]
    pub fn min(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min", self.base))
    }
}
#[derive(Serialize, Default)]
struct ChronicleDashboardChartDashboardChartElVisualizationElDynamic {
    button: Option<DynamicBlock<ChronicleDashboardChartDashboardChartElVisualizationElButtonEl>>,
    column_defs:
        Option<DynamicBlock<ChronicleDashboardChartDashboardChartElVisualizationElColumnDefsEl>>,
    google_maps_config: Option<
        DynamicBlock<ChronicleDashboardChartDashboardChartElVisualizationElGoogleMapsConfigEl>,
    >,
    legends: Option<DynamicBlock<ChronicleDashboardChartDashboardChartElVisualizationElLegendsEl>>,
    markdown:
        Option<DynamicBlock<ChronicleDashboardChartDashboardChartElVisualizationElMarkdownEl>>,
    series: Option<DynamicBlock<ChronicleDashboardChartDashboardChartElVisualizationElSeriesEl>>,
    table_config:
        Option<DynamicBlock<ChronicleDashboardChartDashboardChartElVisualizationElTableConfigEl>>,
    tooltip: Option<DynamicBlock<ChronicleDashboardChartDashboardChartElVisualizationElTooltipEl>>,
    visual_maps:
        Option<DynamicBlock<ChronicleDashboardChartDashboardChartElVisualizationElVisualMapsEl>>,
    x_axes: Option<DynamicBlock<ChronicleDashboardChartDashboardChartElVisualizationElXAxesEl>>,
    y_axes: Option<DynamicBlock<ChronicleDashboardChartDashboardChartElVisualizationElYAxesEl>>,
}
#[derive(Serialize)]
pub struct ChronicleDashboardChartDashboardChartElVisualizationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    grouping_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    series_column: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    threshold_coloring_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    button: Option<Vec<ChronicleDashboardChartDashboardChartElVisualizationElButtonEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    column_defs: Option<Vec<ChronicleDashboardChartDashboardChartElVisualizationElColumnDefsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    google_maps_config:
        Option<Vec<ChronicleDashboardChartDashboardChartElVisualizationElGoogleMapsConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    legends: Option<Vec<ChronicleDashboardChartDashboardChartElVisualizationElLegendsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    markdown: Option<Vec<ChronicleDashboardChartDashboardChartElVisualizationElMarkdownEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    series: Option<Vec<ChronicleDashboardChartDashboardChartElVisualizationElSeriesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    table_config: Option<Vec<ChronicleDashboardChartDashboardChartElVisualizationElTableConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tooltip: Option<Vec<ChronicleDashboardChartDashboardChartElVisualizationElTooltipEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    visual_maps: Option<Vec<ChronicleDashboardChartDashboardChartElVisualizationElVisualMapsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    x_axes: Option<Vec<ChronicleDashboardChartDashboardChartElVisualizationElXAxesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    y_axes: Option<Vec<ChronicleDashboardChartDashboardChartElVisualizationElYAxesEl>>,
    dynamic: ChronicleDashboardChartDashboardChartElVisualizationElDynamic,
}
impl ChronicleDashboardChartDashboardChartElVisualizationEl {
    #[doc = "Set the field `grouping_type`.\n"]
    pub fn set_grouping_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.grouping_type = Some(v.into());
        self
    }
    #[doc = "Set the field `series_column`.\n"]
    pub fn set_series_column(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.series_column = Some(v.into());
        self
    }
    #[doc = "Set the field `threshold_coloring_enabled`.\n"]
    pub fn set_threshold_coloring_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.threshold_coloring_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `button`.\n"]
    pub fn set_button(
        mut self,
        v: impl Into<BlockAssignable<ChronicleDashboardChartDashboardChartElVisualizationElButtonEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.button = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.button = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `column_defs`.\n"]
    pub fn set_column_defs(
        mut self,
        v: impl Into<
            BlockAssignable<ChronicleDashboardChartDashboardChartElVisualizationElColumnDefsEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.column_defs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.column_defs = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `google_maps_config`.\n"]
    pub fn set_google_maps_config(
        mut self,
        v: impl Into<
            BlockAssignable<
                ChronicleDashboardChartDashboardChartElVisualizationElGoogleMapsConfigEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.google_maps_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.google_maps_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `legends`.\n"]
    pub fn set_legends(
        mut self,
        v: impl Into<BlockAssignable<ChronicleDashboardChartDashboardChartElVisualizationElLegendsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.legends = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.legends = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `markdown`.\n"]
    pub fn set_markdown(
        mut self,
        v: impl Into<BlockAssignable<ChronicleDashboardChartDashboardChartElVisualizationElMarkdownEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.markdown = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.markdown = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `series`.\n"]
    pub fn set_series(
        mut self,
        v: impl Into<BlockAssignable<ChronicleDashboardChartDashboardChartElVisualizationElSeriesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.series = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.series = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `table_config`.\n"]
    pub fn set_table_config(
        mut self,
        v: impl Into<
            BlockAssignable<ChronicleDashboardChartDashboardChartElVisualizationElTableConfigEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.table_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.table_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `tooltip`.\n"]
    pub fn set_tooltip(
        mut self,
        v: impl Into<BlockAssignable<ChronicleDashboardChartDashboardChartElVisualizationElTooltipEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.tooltip = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.tooltip = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `visual_maps`.\n"]
    pub fn set_visual_maps(
        mut self,
        v: impl Into<
            BlockAssignable<ChronicleDashboardChartDashboardChartElVisualizationElVisualMapsEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.visual_maps = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.visual_maps = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `x_axes`.\n"]
    pub fn set_x_axes(
        mut self,
        v: impl Into<BlockAssignable<ChronicleDashboardChartDashboardChartElVisualizationElXAxesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.x_axes = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.x_axes = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `y_axes`.\n"]
    pub fn set_y_axes(
        mut self,
        v: impl Into<BlockAssignable<ChronicleDashboardChartDashboardChartElVisualizationElYAxesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.y_axes = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.y_axes = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ChronicleDashboardChartDashboardChartElVisualizationEl {
    type O = BlockAssignable<ChronicleDashboardChartDashboardChartElVisualizationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildChronicleDashboardChartDashboardChartElVisualizationEl {}
impl BuildChronicleDashboardChartDashboardChartElVisualizationEl {
    pub fn build(self) -> ChronicleDashboardChartDashboardChartElVisualizationEl {
        ChronicleDashboardChartDashboardChartElVisualizationEl {
            grouping_type: core::default::Default::default(),
            series_column: core::default::Default::default(),
            threshold_coloring_enabled: core::default::Default::default(),
            button: core::default::Default::default(),
            column_defs: core::default::Default::default(),
            google_maps_config: core::default::Default::default(),
            legends: core::default::Default::default(),
            markdown: core::default::Default::default(),
            series: core::default::Default::default(),
            table_config: core::default::Default::default(),
            tooltip: core::default::Default::default(),
            visual_maps: core::default::Default::default(),
            x_axes: core::default::Default::default(),
            y_axes: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ChronicleDashboardChartDashboardChartElVisualizationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ChronicleDashboardChartDashboardChartElVisualizationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ChronicleDashboardChartDashboardChartElVisualizationElRef {
        ChronicleDashboardChartDashboardChartElVisualizationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ChronicleDashboardChartDashboardChartElVisualizationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `grouping_type` after provisioning.\n"]
    pub fn grouping_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.grouping_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `series_column` after provisioning.\n"]
    pub fn series_column(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.series_column", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `threshold_coloring_enabled` after provisioning.\n"]
    pub fn threshold_coloring_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.threshold_coloring_enabled", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `button` after provisioning.\n"]
    pub fn button(
        &self,
    ) -> ListRef<ChronicleDashboardChartDashboardChartElVisualizationElButtonElRef> {
        ListRef::new(self.shared().clone(), format!("{}.button", self.base))
    }
    #[doc = "Get a reference to the value of field `column_defs` after provisioning.\n"]
    pub fn column_defs(
        &self,
    ) -> ListRef<ChronicleDashboardChartDashboardChartElVisualizationElColumnDefsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.column_defs", self.base))
    }
    #[doc = "Get a reference to the value of field `google_maps_config` after provisioning.\n"]
    pub fn google_maps_config(
        &self,
    ) -> ListRef<ChronicleDashboardChartDashboardChartElVisualizationElGoogleMapsConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.google_maps_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `legends` after provisioning.\n"]
    pub fn legends(
        &self,
    ) -> ListRef<ChronicleDashboardChartDashboardChartElVisualizationElLegendsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.legends", self.base))
    }
    #[doc = "Get a reference to the value of field `markdown` after provisioning.\n"]
    pub fn markdown(
        &self,
    ) -> ListRef<ChronicleDashboardChartDashboardChartElVisualizationElMarkdownElRef> {
        ListRef::new(self.shared().clone(), format!("{}.markdown", self.base))
    }
    #[doc = "Get a reference to the value of field `series` after provisioning.\n"]
    pub fn series(
        &self,
    ) -> ListRef<ChronicleDashboardChartDashboardChartElVisualizationElSeriesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.series", self.base))
    }
    #[doc = "Get a reference to the value of field `table_config` after provisioning.\n"]
    pub fn table_config(
        &self,
    ) -> ListRef<ChronicleDashboardChartDashboardChartElVisualizationElTableConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.table_config", self.base))
    }
    #[doc = "Get a reference to the value of field `tooltip` after provisioning.\n"]
    pub fn tooltip(
        &self,
    ) -> ListRef<ChronicleDashboardChartDashboardChartElVisualizationElTooltipElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tooltip", self.base))
    }
    #[doc = "Get a reference to the value of field `visual_maps` after provisioning.\n"]
    pub fn visual_maps(
        &self,
    ) -> ListRef<ChronicleDashboardChartDashboardChartElVisualizationElVisualMapsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.visual_maps", self.base))
    }
    #[doc = "Get a reference to the value of field `x_axes` after provisioning.\n"]
    pub fn x_axes(
        &self,
    ) -> ListRef<ChronicleDashboardChartDashboardChartElVisualizationElXAxesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.x_axes", self.base))
    }
    #[doc = "Get a reference to the value of field `y_axes` after provisioning.\n"]
    pub fn y_axes(
        &self,
    ) -> ListRef<ChronicleDashboardChartDashboardChartElVisualizationElYAxesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.y_axes", self.base))
    }
}
#[derive(Serialize, Default)]
struct ChronicleDashboardChartDashboardChartElDynamic {
    chart_datasource:
        Option<DynamicBlock<ChronicleDashboardChartDashboardChartElChartDatasourceEl>>,
    drill_down_config:
        Option<DynamicBlock<ChronicleDashboardChartDashboardChartElDrillDownConfigEl>>,
    visualization: Option<DynamicBlock<ChronicleDashboardChartDashboardChartElVisualizationEl>>,
}
#[derive(Serialize)]
pub struct ChronicleDashboardChartDashboardChartEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    display_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tile_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    chart_datasource: Option<Vec<ChronicleDashboardChartDashboardChartElChartDatasourceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    drill_down_config: Option<Vec<ChronicleDashboardChartDashboardChartElDrillDownConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    visualization: Option<Vec<ChronicleDashboardChartDashboardChartElVisualizationEl>>,
    dynamic: ChronicleDashboardChartDashboardChartElDynamic,
}
impl ChronicleDashboardChartDashboardChartEl {
    #[doc = "Set the field `description`.\nDescription of the dashboardChart."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `tile_type`.\nType of tile (e.g., visualization, button, markdown). Possible values: [\"TILE_TYPE_UNSPECIFIED\", \"TILE_TYPE_VISUALIZATION\", \"TILE_TYPE_BUTTON\", \"TILE_TYPE_MARKDOWN\"]"]
    pub fn set_tile_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tile_type = Some(v.into());
        self
    }
    #[doc = "Set the field `chart_datasource`.\n"]
    pub fn set_chart_datasource(
        mut self,
        v: impl Into<BlockAssignable<ChronicleDashboardChartDashboardChartElChartDatasourceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.chart_datasource = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.chart_datasource = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `drill_down_config`.\n"]
    pub fn set_drill_down_config(
        mut self,
        v: impl Into<BlockAssignable<ChronicleDashboardChartDashboardChartElDrillDownConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.drill_down_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.drill_down_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `visualization`.\n"]
    pub fn set_visualization(
        mut self,
        v: impl Into<BlockAssignable<ChronicleDashboardChartDashboardChartElVisualizationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.visualization = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.visualization = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ChronicleDashboardChartDashboardChartEl {
    type O = BlockAssignable<ChronicleDashboardChartDashboardChartEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildChronicleDashboardChartDashboardChartEl {
    #[doc = "Display name/Title of the dashboardChart visible to users."]
    pub display_name: PrimField<String>,
}
impl BuildChronicleDashboardChartDashboardChartEl {
    pub fn build(self) -> ChronicleDashboardChartDashboardChartEl {
        ChronicleDashboardChartDashboardChartEl {
            description: core::default::Default::default(),
            display_name: self.display_name,
            tile_type: core::default::Default::default(),
            chart_datasource: core::default::Default::default(),
            drill_down_config: core::default::Default::default(),
            visualization: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ChronicleDashboardChartDashboardChartElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ChronicleDashboardChartDashboardChartElRef {
    fn new(shared: StackShared, base: String) -> ChronicleDashboardChartDashboardChartElRef {
        ChronicleDashboardChartDashboardChartElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ChronicleDashboardChartDashboardChartElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription of the dashboardChart."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nDisplay name/Title of the dashboardChart visible to users."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nThis checksum is computed by the server."]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.etag", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the DashboardChart."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `tile_type` after provisioning.\nType of tile (e.g., visualization, button, markdown). Possible values: [\"TILE_TYPE_UNSPECIFIED\", \"TILE_TYPE_VISUALIZATION\", \"TILE_TYPE_BUTTON\", \"TILE_TYPE_MARKDOWN\"]"]
    pub fn tile_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tile_type", self.base))
    }
    #[doc = "Get a reference to the value of field `chart_datasource` after provisioning.\n"]
    pub fn chart_datasource(
        &self,
    ) -> ListRef<ChronicleDashboardChartDashboardChartElChartDatasourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.chart_datasource", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `drill_down_config` after provisioning.\n"]
    pub fn drill_down_config(
        &self,
    ) -> ListRef<ChronicleDashboardChartDashboardChartElDrillDownConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.drill_down_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `visualization` after provisioning.\n"]
    pub fn visualization(
        &self,
    ) -> ListRef<ChronicleDashboardChartDashboardChartElVisualizationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.visualization", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ChronicleDashboardChartDashboardQueryElInputElRelativeTimeEl {
    start_time_val: PrimField<String>,
    time_unit: PrimField<String>,
}
impl ChronicleDashboardChartDashboardQueryElInputElRelativeTimeEl {}
impl ToListMappable for ChronicleDashboardChartDashboardQueryElInputElRelativeTimeEl {
    type O = BlockAssignable<ChronicleDashboardChartDashboardQueryElInputElRelativeTimeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildChronicleDashboardChartDashboardQueryElInputElRelativeTimeEl {
    #[doc = ""]
    pub start_time_val: PrimField<String>,
    #[doc = "The time unit for the relative range. Possible values: [\"SECOND\", \"MINUTE\", \"HOUR\", \"DAY\", \"WEEK\", \"MONTH\", \"YEAR\"]"]
    pub time_unit: PrimField<String>,
}
impl BuildChronicleDashboardChartDashboardQueryElInputElRelativeTimeEl {
    pub fn build(self) -> ChronicleDashboardChartDashboardQueryElInputElRelativeTimeEl {
        ChronicleDashboardChartDashboardQueryElInputElRelativeTimeEl {
            start_time_val: self.start_time_val,
            time_unit: self.time_unit,
        }
    }
}
pub struct ChronicleDashboardChartDashboardQueryElInputElRelativeTimeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ChronicleDashboardChartDashboardQueryElInputElRelativeTimeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ChronicleDashboardChartDashboardQueryElInputElRelativeTimeElRef {
        ChronicleDashboardChartDashboardQueryElInputElRelativeTimeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ChronicleDashboardChartDashboardQueryElInputElRelativeTimeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `start_time_val` after provisioning.\n"]
    pub fn start_time_val(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.start_time_val", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `time_unit` after provisioning.\nThe time unit for the relative range. Possible values: [\"SECOND\", \"MINUTE\", \"HOUR\", \"DAY\", \"WEEK\", \"MONTH\", \"YEAR\"]"]
    pub fn time_unit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.time_unit", self.base))
    }
}
#[derive(Serialize)]
pub struct ChronicleDashboardChartDashboardQueryElInputElTimeWindowEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    end_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time: Option<PrimField<String>>,
}
impl ChronicleDashboardChartDashboardQueryElInputElTimeWindowEl {
    #[doc = "Set the field `end_time`.\n"]
    pub fn set_end_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.end_time = Some(v.into());
        self
    }
    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.start_time = Some(v.into());
        self
    }
}
impl ToListMappable for ChronicleDashboardChartDashboardQueryElInputElTimeWindowEl {
    type O = BlockAssignable<ChronicleDashboardChartDashboardQueryElInputElTimeWindowEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildChronicleDashboardChartDashboardQueryElInputElTimeWindowEl {}
impl BuildChronicleDashboardChartDashboardQueryElInputElTimeWindowEl {
    pub fn build(self) -> ChronicleDashboardChartDashboardQueryElInputElTimeWindowEl {
        ChronicleDashboardChartDashboardQueryElInputElTimeWindowEl {
            end_time: core::default::Default::default(),
            start_time: core::default::Default::default(),
        }
    }
}
pub struct ChronicleDashboardChartDashboardQueryElInputElTimeWindowElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ChronicleDashboardChartDashboardQueryElInputElTimeWindowElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ChronicleDashboardChartDashboardQueryElInputElTimeWindowElRef {
        ChronicleDashboardChartDashboardQueryElInputElTimeWindowElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ChronicleDashboardChartDashboardQueryElInputElTimeWindowElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `end_time` after provisioning.\n"]
    pub fn end_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.end_time", self.base))
    }
    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}
#[derive(Serialize, Default)]
struct ChronicleDashboardChartDashboardQueryElInputElDynamic {
    relative_time:
        Option<DynamicBlock<ChronicleDashboardChartDashboardQueryElInputElRelativeTimeEl>>,
    time_window: Option<DynamicBlock<ChronicleDashboardChartDashboardQueryElInputElTimeWindowEl>>,
}
#[derive(Serialize)]
pub struct ChronicleDashboardChartDashboardQueryElInputEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    relative_time: Option<Vec<ChronicleDashboardChartDashboardQueryElInputElRelativeTimeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time_window: Option<Vec<ChronicleDashboardChartDashboardQueryElInputElTimeWindowEl>>,
    dynamic: ChronicleDashboardChartDashboardQueryElInputElDynamic,
}
impl ChronicleDashboardChartDashboardQueryElInputEl {
    #[doc = "Set the field `relative_time`.\n"]
    pub fn set_relative_time(
        mut self,
        v: impl Into<BlockAssignable<ChronicleDashboardChartDashboardQueryElInputElRelativeTimeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.relative_time = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.relative_time = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `time_window`.\n"]
    pub fn set_time_window(
        mut self,
        v: impl Into<BlockAssignable<ChronicleDashboardChartDashboardQueryElInputElTimeWindowEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.time_window = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.time_window = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ChronicleDashboardChartDashboardQueryElInputEl {
    type O = BlockAssignable<ChronicleDashboardChartDashboardQueryElInputEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildChronicleDashboardChartDashboardQueryElInputEl {}
impl BuildChronicleDashboardChartDashboardQueryElInputEl {
    pub fn build(self) -> ChronicleDashboardChartDashboardQueryElInputEl {
        ChronicleDashboardChartDashboardQueryElInputEl {
            relative_time: core::default::Default::default(),
            time_window: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ChronicleDashboardChartDashboardQueryElInputElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ChronicleDashboardChartDashboardQueryElInputElRef {
    fn new(shared: StackShared, base: String) -> ChronicleDashboardChartDashboardQueryElInputElRef {
        ChronicleDashboardChartDashboardQueryElInputElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ChronicleDashboardChartDashboardQueryElInputElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `relative_time` after provisioning.\n"]
    pub fn relative_time(
        &self,
    ) -> ListRef<ChronicleDashboardChartDashboardQueryElInputElRelativeTimeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.relative_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `time_window` after provisioning.\n"]
    pub fn time_window(
        &self,
    ) -> ListRef<ChronicleDashboardChartDashboardQueryElInputElTimeWindowElRef> {
        ListRef::new(self.shared().clone(), format!("{}.time_window", self.base))
    }
}
#[derive(Serialize, Default)]
struct ChronicleDashboardChartDashboardQueryElDynamic {
    input: Option<DynamicBlock<ChronicleDashboardChartDashboardQueryElInputEl>>,
}
#[derive(Serialize)]
pub struct ChronicleDashboardChartDashboardQueryEl {
    query: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input: Option<Vec<ChronicleDashboardChartDashboardQueryElInputEl>>,
    dynamic: ChronicleDashboardChartDashboardQueryElDynamic,
}
impl ChronicleDashboardChartDashboardQueryEl {
    #[doc = "Set the field `input`.\n"]
    pub fn set_input(
        mut self,
        v: impl Into<BlockAssignable<ChronicleDashboardChartDashboardQueryElInputEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.input = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.input = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ChronicleDashboardChartDashboardQueryEl {
    type O = BlockAssignable<ChronicleDashboardChartDashboardQueryEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildChronicleDashboardChartDashboardQueryEl {
    #[doc = "The raw query string."]
    pub query: PrimField<String>,
}
impl BuildChronicleDashboardChartDashboardQueryEl {
    pub fn build(self) -> ChronicleDashboardChartDashboardQueryEl {
        ChronicleDashboardChartDashboardQueryEl {
            query: self.query,
            input: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ChronicleDashboardChartDashboardQueryElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ChronicleDashboardChartDashboardQueryElRef {
    fn new(shared: StackShared, base: String) -> ChronicleDashboardChartDashboardQueryElRef {
        ChronicleDashboardChartDashboardQueryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ChronicleDashboardChartDashboardQueryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nThis checksum is computed by the server."]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.etag", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nname of the query."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `query` after provisioning.\nThe raw query string."]
    pub fn query(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.query", self.base))
    }
    #[doc = "Get a reference to the value of field `input` after provisioning.\n"]
    pub fn input(&self) -> ListRef<ChronicleDashboardChartDashboardQueryElInputElRef> {
        ListRef::new(self.shared().clone(), format!("{}.input", self.base))
    }
}
#[derive(Serialize)]
pub struct ChronicleDashboardChartTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ChronicleDashboardChartTimeoutsEl {
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
impl ToListMappable for ChronicleDashboardChartTimeoutsEl {
    type O = BlockAssignable<ChronicleDashboardChartTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildChronicleDashboardChartTimeoutsEl {}
impl BuildChronicleDashboardChartTimeoutsEl {
    pub fn build(self) -> ChronicleDashboardChartTimeoutsEl {
        ChronicleDashboardChartTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ChronicleDashboardChartTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ChronicleDashboardChartTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ChronicleDashboardChartTimeoutsElRef {
        ChronicleDashboardChartTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ChronicleDashboardChartTimeoutsElRef {
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
struct ChronicleDashboardChartDynamic {
    chart_layout: Option<DynamicBlock<ChronicleDashboardChartChartLayoutEl>>,
    dashboard_chart: Option<DynamicBlock<ChronicleDashboardChartDashboardChartEl>>,
    dashboard_query: Option<DynamicBlock<ChronicleDashboardChartDashboardQueryEl>>,
}
