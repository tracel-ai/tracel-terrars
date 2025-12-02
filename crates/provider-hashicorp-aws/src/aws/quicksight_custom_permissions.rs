use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct QuicksightCustomPermissionsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    aws_account_id: Option<PrimField<String>>,
    custom_permissions_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    capabilities: Option<Vec<QuicksightCustomPermissionsCapabilitiesEl>>,
    dynamic: QuicksightCustomPermissionsDynamic,
}
struct QuicksightCustomPermissions_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<QuicksightCustomPermissionsData>,
}
#[derive(Clone)]
pub struct QuicksightCustomPermissions(Rc<QuicksightCustomPermissions_>);
impl QuicksightCustomPermissions {
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
    #[doc = "Set the field `aws_account_id`.\n"]
    pub fn set_aws_account_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().aws_account_id = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }
    #[doc = "Set the field `capabilities`.\n"]
    pub fn set_capabilities(
        self,
        v: impl Into<BlockAssignable<QuicksightCustomPermissionsCapabilitiesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().capabilities = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.capabilities = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `aws_account_id` after provisioning.\n"]
    pub fn aws_account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.aws_account_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `custom_permissions_name` after provisioning.\n"]
    pub fn custom_permissions_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.custom_permissions_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `capabilities` after provisioning.\n"]
    pub fn capabilities(&self) -> ListRef<QuicksightCustomPermissionsCapabilitiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.capabilities", self.extract_ref()),
        )
    }
}
impl Referable for QuicksightCustomPermissions {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for QuicksightCustomPermissions {}
impl ToListMappable for QuicksightCustomPermissions {
    type O = ListRef<QuicksightCustomPermissionsRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for QuicksightCustomPermissions_ {
    fn extract_resource_type(&self) -> String {
        "aws_quicksight_custom_permissions".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildQuicksightCustomPermissions {
    pub tf_id: String,
    #[doc = ""]
    pub custom_permissions_name: PrimField<String>,
}
impl BuildQuicksightCustomPermissions {
    pub fn build(self, stack: &mut Stack) -> QuicksightCustomPermissions {
        let out = QuicksightCustomPermissions(Rc::new(QuicksightCustomPermissions_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(QuicksightCustomPermissionsData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                aws_account_id: core::default::Default::default(),
                custom_permissions_name: self.custom_permissions_name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                capabilities: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct QuicksightCustomPermissionsRef {
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightCustomPermissionsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl QuicksightCustomPermissionsRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `aws_account_id` after provisioning.\n"]
    pub fn aws_account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.aws_account_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `custom_permissions_name` after provisioning.\n"]
    pub fn custom_permissions_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.custom_permissions_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `capabilities` after provisioning.\n"]
    pub fn capabilities(&self) -> ListRef<QuicksightCustomPermissionsCapabilitiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.capabilities", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct QuicksightCustomPermissionsCapabilitiesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    add_or_run_anomaly_detection_for_analyses: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    create_and_update_dashboard_email_reports: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    create_and_update_data_sources: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    create_and_update_datasets: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    create_and_update_themes: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    create_and_update_threshold_alerts: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    create_shared_folders: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    create_spice_dataset: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    export_to_csv: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    export_to_csv_in_scheduled_reports: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    export_to_excel: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    export_to_excel_in_scheduled_reports: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    export_to_pdf: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    export_to_pdf_in_scheduled_reports: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_content_in_scheduled_reports_email: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    print_reports: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rename_shared_folders: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    share_analyses: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    share_dashboards: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    share_data_sources: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    share_datasets: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subscribe_dashboard_email_reports: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    view_account_spice_capacity: Option<PrimField<String>>,
}
impl QuicksightCustomPermissionsCapabilitiesEl {
    #[doc = "Set the field `add_or_run_anomaly_detection_for_analyses`.\n"]
    pub fn set_add_or_run_anomaly_detection_for_analyses(
        mut self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.add_or_run_anomaly_detection_for_analyses = Some(v.into());
        self
    }
    #[doc = "Set the field `create_and_update_dashboard_email_reports`.\n"]
    pub fn set_create_and_update_dashboard_email_reports(
        mut self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.create_and_update_dashboard_email_reports = Some(v.into());
        self
    }
    #[doc = "Set the field `create_and_update_data_sources`.\n"]
    pub fn set_create_and_update_data_sources(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create_and_update_data_sources = Some(v.into());
        self
    }
    #[doc = "Set the field `create_and_update_datasets`.\n"]
    pub fn set_create_and_update_datasets(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create_and_update_datasets = Some(v.into());
        self
    }
    #[doc = "Set the field `create_and_update_themes`.\n"]
    pub fn set_create_and_update_themes(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create_and_update_themes = Some(v.into());
        self
    }
    #[doc = "Set the field `create_and_update_threshold_alerts`.\n"]
    pub fn set_create_and_update_threshold_alerts(
        mut self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.create_and_update_threshold_alerts = Some(v.into());
        self
    }
    #[doc = "Set the field `create_shared_folders`.\n"]
    pub fn set_create_shared_folders(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create_shared_folders = Some(v.into());
        self
    }
    #[doc = "Set the field `create_spice_dataset`.\n"]
    pub fn set_create_spice_dataset(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create_spice_dataset = Some(v.into());
        self
    }
    #[doc = "Set the field `export_to_csv`.\n"]
    pub fn set_export_to_csv(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.export_to_csv = Some(v.into());
        self
    }
    #[doc = "Set the field `export_to_csv_in_scheduled_reports`.\n"]
    pub fn set_export_to_csv_in_scheduled_reports(
        mut self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.export_to_csv_in_scheduled_reports = Some(v.into());
        self
    }
    #[doc = "Set the field `export_to_excel`.\n"]
    pub fn set_export_to_excel(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.export_to_excel = Some(v.into());
        self
    }
    #[doc = "Set the field `export_to_excel_in_scheduled_reports`.\n"]
    pub fn set_export_to_excel_in_scheduled_reports(
        mut self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.export_to_excel_in_scheduled_reports = Some(v.into());
        self
    }
    #[doc = "Set the field `export_to_pdf`.\n"]
    pub fn set_export_to_pdf(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.export_to_pdf = Some(v.into());
        self
    }
    #[doc = "Set the field `export_to_pdf_in_scheduled_reports`.\n"]
    pub fn set_export_to_pdf_in_scheduled_reports(
        mut self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.export_to_pdf_in_scheduled_reports = Some(v.into());
        self
    }
    #[doc = "Set the field `include_content_in_scheduled_reports_email`.\n"]
    pub fn set_include_content_in_scheduled_reports_email(
        mut self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.include_content_in_scheduled_reports_email = Some(v.into());
        self
    }
    #[doc = "Set the field `print_reports`.\n"]
    pub fn set_print_reports(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.print_reports = Some(v.into());
        self
    }
    #[doc = "Set the field `rename_shared_folders`.\n"]
    pub fn set_rename_shared_folders(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.rename_shared_folders = Some(v.into());
        self
    }
    #[doc = "Set the field `share_analyses`.\n"]
    pub fn set_share_analyses(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.share_analyses = Some(v.into());
        self
    }
    #[doc = "Set the field `share_dashboards`.\n"]
    pub fn set_share_dashboards(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.share_dashboards = Some(v.into());
        self
    }
    #[doc = "Set the field `share_data_sources`.\n"]
    pub fn set_share_data_sources(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.share_data_sources = Some(v.into());
        self
    }
    #[doc = "Set the field `share_datasets`.\n"]
    pub fn set_share_datasets(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.share_datasets = Some(v.into());
        self
    }
    #[doc = "Set the field `subscribe_dashboard_email_reports`.\n"]
    pub fn set_subscribe_dashboard_email_reports(
        mut self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.subscribe_dashboard_email_reports = Some(v.into());
        self
    }
    #[doc = "Set the field `view_account_spice_capacity`.\n"]
    pub fn set_view_account_spice_capacity(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.view_account_spice_capacity = Some(v.into());
        self
    }
}
impl ToListMappable for QuicksightCustomPermissionsCapabilitiesEl {
    type O = BlockAssignable<QuicksightCustomPermissionsCapabilitiesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildQuicksightCustomPermissionsCapabilitiesEl {}
impl BuildQuicksightCustomPermissionsCapabilitiesEl {
    pub fn build(self) -> QuicksightCustomPermissionsCapabilitiesEl {
        QuicksightCustomPermissionsCapabilitiesEl {
            add_or_run_anomaly_detection_for_analyses: core::default::Default::default(),
            create_and_update_dashboard_email_reports: core::default::Default::default(),
            create_and_update_data_sources: core::default::Default::default(),
            create_and_update_datasets: core::default::Default::default(),
            create_and_update_themes: core::default::Default::default(),
            create_and_update_threshold_alerts: core::default::Default::default(),
            create_shared_folders: core::default::Default::default(),
            create_spice_dataset: core::default::Default::default(),
            export_to_csv: core::default::Default::default(),
            export_to_csv_in_scheduled_reports: core::default::Default::default(),
            export_to_excel: core::default::Default::default(),
            export_to_excel_in_scheduled_reports: core::default::Default::default(),
            export_to_pdf: core::default::Default::default(),
            export_to_pdf_in_scheduled_reports: core::default::Default::default(),
            include_content_in_scheduled_reports_email: core::default::Default::default(),
            print_reports: core::default::Default::default(),
            rename_shared_folders: core::default::Default::default(),
            share_analyses: core::default::Default::default(),
            share_dashboards: core::default::Default::default(),
            share_data_sources: core::default::Default::default(),
            share_datasets: core::default::Default::default(),
            subscribe_dashboard_email_reports: core::default::Default::default(),
            view_account_spice_capacity: core::default::Default::default(),
        }
    }
}
pub struct QuicksightCustomPermissionsCapabilitiesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightCustomPermissionsCapabilitiesElRef {
    fn new(shared: StackShared, base: String) -> QuicksightCustomPermissionsCapabilitiesElRef {
        QuicksightCustomPermissionsCapabilitiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl QuicksightCustomPermissionsCapabilitiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `add_or_run_anomaly_detection_for_analyses` after provisioning.\n"]
    pub fn add_or_run_anomaly_detection_for_analyses(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.add_or_run_anomaly_detection_for_analyses", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `create_and_update_dashboard_email_reports` after provisioning.\n"]
    pub fn create_and_update_dashboard_email_reports(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_and_update_dashboard_email_reports", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `create_and_update_data_sources` after provisioning.\n"]
    pub fn create_and_update_data_sources(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_and_update_data_sources", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `create_and_update_datasets` after provisioning.\n"]
    pub fn create_and_update_datasets(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_and_update_datasets", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `create_and_update_themes` after provisioning.\n"]
    pub fn create_and_update_themes(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_and_update_themes", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `create_and_update_threshold_alerts` after provisioning.\n"]
    pub fn create_and_update_threshold_alerts(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_and_update_threshold_alerts", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `create_shared_folders` after provisioning.\n"]
    pub fn create_shared_folders(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_shared_folders", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `create_spice_dataset` after provisioning.\n"]
    pub fn create_spice_dataset(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_spice_dataset", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `export_to_csv` after provisioning.\n"]
    pub fn export_to_csv(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.export_to_csv", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `export_to_csv_in_scheduled_reports` after provisioning.\n"]
    pub fn export_to_csv_in_scheduled_reports(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.export_to_csv_in_scheduled_reports", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `export_to_excel` after provisioning.\n"]
    pub fn export_to_excel(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.export_to_excel", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `export_to_excel_in_scheduled_reports` after provisioning.\n"]
    pub fn export_to_excel_in_scheduled_reports(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.export_to_excel_in_scheduled_reports", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `export_to_pdf` after provisioning.\n"]
    pub fn export_to_pdf(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.export_to_pdf", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `export_to_pdf_in_scheduled_reports` after provisioning.\n"]
    pub fn export_to_pdf_in_scheduled_reports(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.export_to_pdf_in_scheduled_reports", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `include_content_in_scheduled_reports_email` after provisioning.\n"]
    pub fn include_content_in_scheduled_reports_email(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.include_content_in_scheduled_reports_email", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `print_reports` after provisioning.\n"]
    pub fn print_reports(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.print_reports", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `rename_shared_folders` after provisioning.\n"]
    pub fn rename_shared_folders(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rename_shared_folders", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `share_analyses` after provisioning.\n"]
    pub fn share_analyses(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.share_analyses", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `share_dashboards` after provisioning.\n"]
    pub fn share_dashboards(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.share_dashboards", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `share_data_sources` after provisioning.\n"]
    pub fn share_data_sources(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.share_data_sources", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `share_datasets` after provisioning.\n"]
    pub fn share_datasets(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.share_datasets", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `subscribe_dashboard_email_reports` after provisioning.\n"]
    pub fn subscribe_dashboard_email_reports(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subscribe_dashboard_email_reports", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `view_account_spice_capacity` after provisioning.\n"]
    pub fn view_account_spice_capacity(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.view_account_spice_capacity", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct QuicksightCustomPermissionsDynamic {
    capabilities: Option<DynamicBlock<QuicksightCustomPermissionsCapabilitiesEl>>,
}
