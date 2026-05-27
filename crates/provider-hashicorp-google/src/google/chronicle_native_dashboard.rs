use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ChronicleNativeDashboardData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    access: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    display_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    instance: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_pinned: Option<PrimField<bool>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    charts: Option<Vec<ChronicleNativeDashboardChartsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filters: Option<Vec<ChronicleNativeDashboardFiltersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ChronicleNativeDashboardTimeoutsEl>,
    dynamic: ChronicleNativeDashboardDynamic,
}
struct ChronicleNativeDashboard_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ChronicleNativeDashboardData>,
}
#[derive(Clone)]
pub struct ChronicleNativeDashboard(Rc<ChronicleNativeDashboard_>);
impl ChronicleNativeDashboard {
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
    #[doc = "Set the field `access`.\nThe access level of the dashboard.\nPossible values:\nDASHBOARD_PRIVATE\nDASHBOARD_PUBLIC"]
    pub fn set_access(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().access = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nA description of the dashboard."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `is_pinned`.\nWhether the dashboard is pinned by the user."]
    pub fn set_is_pinned(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().is_pinned = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\nThe type of dashboard.\nPossible values:\nCURATED, PRIVATE, PUBLIC, CUSTOM, MARKETPLACE"]
    pub fn set_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().type_ = Some(v.into());
        self
    }
    #[doc = "Set the field `charts`.\n"]
    pub fn set_charts(
        self,
        v: impl Into<BlockAssignable<ChronicleNativeDashboardChartsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().charts = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.charts = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `filters`.\n"]
    pub fn set_filters(
        self,
        v: impl Into<BlockAssignable<ChronicleNativeDashboardFiltersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().filters = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.filters = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ChronicleNativeDashboardTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `access` after provisioning.\nThe access level of the dashboard.\nPossible values:\nDASHBOARD_PRIVATE\nDASHBOARD_PUBLIC"]
    pub fn access(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.access", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe creation time of the dashboard."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_user_id` after provisioning.\nThe ID of the user who created the dashboard."]
    pub fn create_user_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_user_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dashboard_id` after provisioning.\nThe unique ID of the Dashboard."]
    pub fn dashboard_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dashboard_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA description of the dashboard."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe display name/title of the dashboard visible to users."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nServer-computed checksum for optimistic concurrency control,\nsent on update and delete requests."]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.etag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `fingerprint` after provisioning.\nThe server-generated fingerprint of the dashboard definition."]
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
    #[doc = "Get a reference to the value of field `instance` after provisioning.\nThe ID of the Chronicle instance."]
    pub fn instance(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `is_pinned` after provisioning.\nWhether the dashboard is pinned by the user."]
    pub fn is_pinned(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_pinned", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `last_viewed_time` after provisioning.\nThe time when this dashboard was last viewed."]
    pub fn last_viewed_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_viewed_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the Chronicle instance."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe full resource name of the dashboard."]
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
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe type of dashboard.\nPossible values:\nCURATED, PRIVATE, PUBLIC, CUSTOM, MARKETPLACE"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe time when the dashboard was last edited."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_user_id` after provisioning.\nThe ID of the user who last edited the dashboard."]
    pub fn update_user_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_user_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `charts` after provisioning.\n"]
    pub fn charts(&self) -> ListRef<ChronicleNativeDashboardChartsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.charts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `filters` after provisioning.\n"]
    pub fn filters(&self) -> ListRef<ChronicleNativeDashboardFiltersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.filters", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ChronicleNativeDashboardTimeoutsElRef {
        ChronicleNativeDashboardTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ChronicleNativeDashboard {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ChronicleNativeDashboard {}
impl ToListMappable for ChronicleNativeDashboard {
    type O = ListRef<ChronicleNativeDashboardRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ChronicleNativeDashboard_ {
    fn extract_resource_type(&self) -> String {
        "google_chronicle_native_dashboard".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildChronicleNativeDashboard {
    pub tf_id: String,
    #[doc = "The display name/title of the dashboard visible to users."]
    pub display_name: PrimField<String>,
    #[doc = "The ID of the Chronicle instance."]
    pub instance: PrimField<String>,
    #[doc = "The location of the Chronicle instance."]
    pub location: PrimField<String>,
}
impl BuildChronicleNativeDashboard {
    pub fn build(self, stack: &mut Stack) -> ChronicleNativeDashboard {
        let out = ChronicleNativeDashboard(Rc::new(ChronicleNativeDashboard_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ChronicleNativeDashboardData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                access: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                display_name: self.display_name,
                id: core::default::Default::default(),
                instance: self.instance,
                is_pinned: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                type_: core::default::Default::default(),
                charts: core::default::Default::default(),
                filters: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ChronicleNativeDashboardRef {
    shared: StackShared,
    base: String,
}
impl Ref for ChronicleNativeDashboardRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ChronicleNativeDashboardRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `access` after provisioning.\nThe access level of the dashboard.\nPossible values:\nDASHBOARD_PRIVATE\nDASHBOARD_PUBLIC"]
    pub fn access(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.access", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe creation time of the dashboard."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_user_id` after provisioning.\nThe ID of the user who created the dashboard."]
    pub fn create_user_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_user_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dashboard_id` after provisioning.\nThe unique ID of the Dashboard."]
    pub fn dashboard_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dashboard_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA description of the dashboard."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe display name/title of the dashboard visible to users."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nServer-computed checksum for optimistic concurrency control,\nsent on update and delete requests."]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.etag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `fingerprint` after provisioning.\nThe server-generated fingerprint of the dashboard definition."]
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
    #[doc = "Get a reference to the value of field `instance` after provisioning.\nThe ID of the Chronicle instance."]
    pub fn instance(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `is_pinned` after provisioning.\nWhether the dashboard is pinned by the user."]
    pub fn is_pinned(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_pinned", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `last_viewed_time` after provisioning.\nThe time when this dashboard was last viewed."]
    pub fn last_viewed_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_viewed_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the Chronicle instance."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe full resource name of the dashboard."]
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
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe type of dashboard.\nPossible values:\nCURATED, PRIVATE, PUBLIC, CUSTOM, MARKETPLACE"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe time when the dashboard was last edited."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_user_id` after provisioning.\nThe ID of the user who last edited the dashboard."]
    pub fn update_user_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_user_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `charts` after provisioning.\n"]
    pub fn charts(&self) -> ListRef<ChronicleNativeDashboardChartsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.charts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `filters` after provisioning.\n"]
    pub fn filters(&self) -> ListRef<ChronicleNativeDashboardFiltersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.filters", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ChronicleNativeDashboardTimeoutsElRef {
        ChronicleNativeDashboardTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ChronicleNativeDashboardChartsElChartLayoutEl {
    span_x: PrimField<f64>,
    span_y: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_x: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_y: Option<PrimField<f64>>,
}
impl ChronicleNativeDashboardChartsElChartLayoutEl {
    #[doc = "Set the field `start_x`.\nThe starting X coordinate."]
    pub fn set_start_x(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.start_x = Some(v.into());
        self
    }
    #[doc = "Set the field `start_y`.\nThe starting Y coordinate."]
    pub fn set_start_y(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.start_y = Some(v.into());
        self
    }
}
impl ToListMappable for ChronicleNativeDashboardChartsElChartLayoutEl {
    type O = BlockAssignable<ChronicleNativeDashboardChartsElChartLayoutEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildChronicleNativeDashboardChartsElChartLayoutEl {
    #[doc = "The number of columns the chart spans."]
    pub span_x: PrimField<f64>,
    #[doc = "The number of rows the chart spans."]
    pub span_y: PrimField<f64>,
}
impl BuildChronicleNativeDashboardChartsElChartLayoutEl {
    pub fn build(self) -> ChronicleNativeDashboardChartsElChartLayoutEl {
        ChronicleNativeDashboardChartsElChartLayoutEl {
            span_x: self.span_x,
            span_y: self.span_y,
            start_x: core::default::Default::default(),
            start_y: core::default::Default::default(),
        }
    }
}
pub struct ChronicleNativeDashboardChartsElChartLayoutElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ChronicleNativeDashboardChartsElChartLayoutElRef {
    fn new(shared: StackShared, base: String) -> ChronicleNativeDashboardChartsElChartLayoutElRef {
        ChronicleNativeDashboardChartsElChartLayoutElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ChronicleNativeDashboardChartsElChartLayoutElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `span_x` after provisioning.\nThe number of columns the chart spans."]
    pub fn span_x(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.span_x", self.base))
    }
    #[doc = "Get a reference to the value of field `span_y` after provisioning.\nThe number of rows the chart spans."]
    pub fn span_y(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.span_y", self.base))
    }
    #[doc = "Get a reference to the value of field `start_x` after provisioning.\nThe starting X coordinate."]
    pub fn start_x(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.start_x", self.base))
    }
    #[doc = "Get a reference to the value of field `start_y` after provisioning.\nThe starting Y coordinate."]
    pub fn start_y(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.start_y", self.base))
    }
}
#[derive(Serialize, Default)]
struct ChronicleNativeDashboardChartsElDynamic {
    chart_layout: Option<DynamicBlock<ChronicleNativeDashboardChartsElChartLayoutEl>>,
}
#[derive(Serialize)]
pub struct ChronicleNativeDashboardChartsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dashboard_chart: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filters_ids: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    chart_layout: Option<Vec<ChronicleNativeDashboardChartsElChartLayoutEl>>,
    dynamic: ChronicleNativeDashboardChartsElDynamic,
}
impl ChronicleNativeDashboardChartsEl {
    #[doc = "Set the field `dashboard_chart`.\nThe resource name of the associated DashboardChart."]
    pub fn set_dashboard_chart(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dashboard_chart = Some(v.into());
        self
    }
    #[doc = "Set the field `filters_ids`.\nList of dashboard filter IDs applied to this chart."]
    pub fn set_filters_ids(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.filters_ids = Some(v.into());
        self
    }
    #[doc = "Set the field `chart_layout`.\n"]
    pub fn set_chart_layout(
        mut self,
        v: impl Into<BlockAssignable<ChronicleNativeDashboardChartsElChartLayoutEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.chart_layout = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.chart_layout = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ChronicleNativeDashboardChartsEl {
    type O = BlockAssignable<ChronicleNativeDashboardChartsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildChronicleNativeDashboardChartsEl {}
impl BuildChronicleNativeDashboardChartsEl {
    pub fn build(self) -> ChronicleNativeDashboardChartsEl {
        ChronicleNativeDashboardChartsEl {
            dashboard_chart: core::default::Default::default(),
            filters_ids: core::default::Default::default(),
            chart_layout: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ChronicleNativeDashboardChartsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ChronicleNativeDashboardChartsElRef {
    fn new(shared: StackShared, base: String) -> ChronicleNativeDashboardChartsElRef {
        ChronicleNativeDashboardChartsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ChronicleNativeDashboardChartsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `dashboard_chart` after provisioning.\nThe resource name of the associated DashboardChart."]
    pub fn dashboard_chart(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dashboard_chart", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `filters_ids` after provisioning.\nList of dashboard filter IDs applied to this chart."]
    pub fn filters_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.filters_ids", self.base))
    }
    #[doc = "Get a reference to the value of field `chart_layout` after provisioning.\n"]
    pub fn chart_layout(&self) -> ListRef<ChronicleNativeDashboardChartsElChartLayoutElRef> {
        ListRef::new(self.shared().clone(), format!("{}.chart_layout", self.base))
    }
}
#[derive(Serialize)]
pub struct ChronicleNativeDashboardFiltersElFilterOperatorAndFieldValuesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    field_values: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter_operator: Option<PrimField<String>>,
}
impl ChronicleNativeDashboardFiltersElFilterOperatorAndFieldValuesEl {
    #[doc = "Set the field `field_values`.\nThe values for the modifier. All operators should have a single\nvalue other than 'IN' and 'BETWEEN'."]
    pub fn set_field_values(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.field_values = Some(v.into());
        self
    }
    #[doc = "Set the field `filter_operator`.\nThe operator to apply to the field. Possible values: [\"EQUAL\", \"NOT_EQUAL\", \"IN\", \"GREATER_THAN\", \"GREATER_THAN_OR_EQUAL_TO\", \"LESS_THAN\", \"LESS_THAN_OR_EQUAL_TO\", \"BETWEEN\", \"PAST\", \"IS_NULL\", \"IS_NOT_NULL\", \"STARTS_WITH\", \"ENDS_WITH\", \"DOES_NOT_STARTS_WITH\", \"DOES_NOT_ENDS_WITH\", \"NOT_IN\", \"CONTAINS\", \"DOES_NOT_CONTAIN\"]"]
    pub fn set_filter_operator(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.filter_operator = Some(v.into());
        self
    }
}
impl ToListMappable for ChronicleNativeDashboardFiltersElFilterOperatorAndFieldValuesEl {
    type O = BlockAssignable<ChronicleNativeDashboardFiltersElFilterOperatorAndFieldValuesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildChronicleNativeDashboardFiltersElFilterOperatorAndFieldValuesEl {}
impl BuildChronicleNativeDashboardFiltersElFilterOperatorAndFieldValuesEl {
    pub fn build(self) -> ChronicleNativeDashboardFiltersElFilterOperatorAndFieldValuesEl {
        ChronicleNativeDashboardFiltersElFilterOperatorAndFieldValuesEl {
            field_values: core::default::Default::default(),
            filter_operator: core::default::Default::default(),
        }
    }
}
pub struct ChronicleNativeDashboardFiltersElFilterOperatorAndFieldValuesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ChronicleNativeDashboardFiltersElFilterOperatorAndFieldValuesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ChronicleNativeDashboardFiltersElFilterOperatorAndFieldValuesElRef {
        ChronicleNativeDashboardFiltersElFilterOperatorAndFieldValuesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ChronicleNativeDashboardFiltersElFilterOperatorAndFieldValuesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `field_values` after provisioning.\nThe values for the modifier. All operators should have a single\nvalue other than 'IN' and 'BETWEEN'."]
    pub fn field_values(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.field_values", self.base))
    }
    #[doc = "Get a reference to the value of field `filter_operator` after provisioning.\nThe operator to apply to the field. Possible values: [\"EQUAL\", \"NOT_EQUAL\", \"IN\", \"GREATER_THAN\", \"GREATER_THAN_OR_EQUAL_TO\", \"LESS_THAN\", \"LESS_THAN_OR_EQUAL_TO\", \"BETWEEN\", \"PAST\", \"IS_NULL\", \"IS_NOT_NULL\", \"STARTS_WITH\", \"ENDS_WITH\", \"DOES_NOT_STARTS_WITH\", \"DOES_NOT_ENDS_WITH\", \"NOT_IN\", \"CONTAINS\", \"DOES_NOT_CONTAIN\"]"]
    pub fn filter_operator(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.filter_operator", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ChronicleNativeDashboardFiltersElDynamic {
    filter_operator_and_field_values:
        Option<DynamicBlock<ChronicleNativeDashboardFiltersElFilterOperatorAndFieldValuesEl>>,
}
#[derive(Serialize)]
pub struct ChronicleNativeDashboardFiltersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    chart_ids: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_source: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    field_path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_mandatory: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_standard_time_range_filter: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_standard_time_range_filter_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter_operator_and_field_values:
        Option<Vec<ChronicleNativeDashboardFiltersElFilterOperatorAndFieldValuesEl>>,
    dynamic: ChronicleNativeDashboardFiltersElDynamic,
}
impl ChronicleNativeDashboardFiltersEl {
    #[doc = "Set the field `chart_ids`.\nThe IDs of charts that this filter applies to."]
    pub fn set_chart_ids(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.chart_ids = Some(v.into());
        self
    }
    #[doc = "Set the field `data_source`.\nThe data source for the filter.\nPossible values:\nUDM, ENTITY, INGESTION_METRICS, RULE_DETECTIONS, RULESETS, GLOBAL,\nIOC_MATCHES, RULES, SOAR_CASES, SOAR_PLAYBOOKS, SOAR_CASE_HISTORY,\nDATA_TABLE, INVESTIGATION, INVESTIGATION_FEEDBACK"]
    pub fn set_data_source(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.data_source = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\nThe display name of the filter."]
    pub fn set_display_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `field_path`.\nThe UDM field path being filtered."]
    pub fn set_field_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.field_path = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\nThe unique ID of the filter."]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }
    #[doc = "Set the field `is_mandatory`.\nWhether the filter is mandatory for the dashboard consumer."]
    pub fn set_is_mandatory(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.is_mandatory = Some(v.into());
        self
    }
    #[doc = "Set the field `is_standard_time_range_filter`.\nWhether the filter is a standard time range filter."]
    pub fn set_is_standard_time_range_filter(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.is_standard_time_range_filter = Some(v.into());
        self
    }
    #[doc = "Set the field `is_standard_time_range_filter_enabled`.\nWhether the standard time range filter is currently enabled."]
    pub fn set_is_standard_time_range_filter_enabled(
        mut self,
        v: impl Into<PrimField<bool>>,
    ) -> Self {
        self.is_standard_time_range_filter_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `filter_operator_and_field_values`.\n"]
    pub fn set_filter_operator_and_field_values(
        mut self,
        v: impl Into<BlockAssignable<ChronicleNativeDashboardFiltersElFilterOperatorAndFieldValuesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.filter_operator_and_field_values = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.filter_operator_and_field_values = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ChronicleNativeDashboardFiltersEl {
    type O = BlockAssignable<ChronicleNativeDashboardFiltersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildChronicleNativeDashboardFiltersEl {}
impl BuildChronicleNativeDashboardFiltersEl {
    pub fn build(self) -> ChronicleNativeDashboardFiltersEl {
        ChronicleNativeDashboardFiltersEl {
            chart_ids: core::default::Default::default(),
            data_source: core::default::Default::default(),
            display_name: core::default::Default::default(),
            field_path: core::default::Default::default(),
            id: core::default::Default::default(),
            is_mandatory: core::default::Default::default(),
            is_standard_time_range_filter: core::default::Default::default(),
            is_standard_time_range_filter_enabled: core::default::Default::default(),
            filter_operator_and_field_values: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ChronicleNativeDashboardFiltersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ChronicleNativeDashboardFiltersElRef {
    fn new(shared: StackShared, base: String) -> ChronicleNativeDashboardFiltersElRef {
        ChronicleNativeDashboardFiltersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ChronicleNativeDashboardFiltersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `chart_ids` after provisioning.\nThe IDs of charts that this filter applies to."]
    pub fn chart_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.chart_ids", self.base))
    }
    #[doc = "Get a reference to the value of field `data_source` after provisioning.\nThe data source for the filter.\nPossible values:\nUDM, ENTITY, INGESTION_METRICS, RULE_DETECTIONS, RULESETS, GLOBAL,\nIOC_MATCHES, RULES, SOAR_CASES, SOAR_PLAYBOOKS, SOAR_CASE_HISTORY,\nDATA_TABLE, INVESTIGATION, INVESTIGATION_FEEDBACK"]
    pub fn data_source(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.data_source", self.base))
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the filter."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
    #[doc = "Get a reference to the value of field `field_path` after provisioning.\nThe UDM field path being filtered."]
    pub fn field_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.field_path", self.base))
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe unique ID of the filter."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
    #[doc = "Get a reference to the value of field `is_mandatory` after provisioning.\nWhether the filter is mandatory for the dashboard consumer."]
    pub fn is_mandatory(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.is_mandatory", self.base))
    }
    #[doc = "Get a reference to the value of field `is_standard_time_range_filter` after provisioning.\nWhether the filter is a standard time range filter."]
    pub fn is_standard_time_range_filter(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_standard_time_range_filter", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `is_standard_time_range_filter_enabled` after provisioning.\nWhether the standard time range filter is currently enabled."]
    pub fn is_standard_time_range_filter_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_standard_time_range_filter_enabled", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `filter_operator_and_field_values` after provisioning.\n"]
    pub fn filter_operator_and_field_values(
        &self,
    ) -> ListRef<ChronicleNativeDashboardFiltersElFilterOperatorAndFieldValuesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.filter_operator_and_field_values", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ChronicleNativeDashboardTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ChronicleNativeDashboardTimeoutsEl {
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
impl ToListMappable for ChronicleNativeDashboardTimeoutsEl {
    type O = BlockAssignable<ChronicleNativeDashboardTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildChronicleNativeDashboardTimeoutsEl {}
impl BuildChronicleNativeDashboardTimeoutsEl {
    pub fn build(self) -> ChronicleNativeDashboardTimeoutsEl {
        ChronicleNativeDashboardTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ChronicleNativeDashboardTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ChronicleNativeDashboardTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ChronicleNativeDashboardTimeoutsElRef {
        ChronicleNativeDashboardTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ChronicleNativeDashboardTimeoutsElRef {
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
struct ChronicleNativeDashboardDynamic {
    charts: Option<DynamicBlock<ChronicleNativeDashboardChartsEl>>,
    filters: Option<DynamicBlock<ChronicleNativeDashboardFiltersEl>>,
}
