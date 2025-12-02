use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataBackupReportPlanData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
}
struct DataBackupReportPlan_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataBackupReportPlanData>,
}
#[derive(Clone)]
pub struct DataBackupReportPlan(Rc<DataBackupReportPlan_>);
impl DataBackupReportPlan {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(&self, provider: &ProviderAws) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
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
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `creation_time` after provisioning.\n"]
    pub fn creation_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creation_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deployment_status` after provisioning.\n"]
    pub fn deployment_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deployment_status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `report_delivery_channel` after provisioning.\n"]
    pub fn report_delivery_channel(
        &self,
    ) -> ListRef<DataBackupReportPlanReportDeliveryChannelElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.report_delivery_channel", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `report_setting` after provisioning.\n"]
    pub fn report_setting(&self) -> ListRef<DataBackupReportPlanReportSettingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.report_setting", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
}
impl Referable for DataBackupReportPlan {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataBackupReportPlan {}
impl ToListMappable for DataBackupReportPlan {
    type O = ListRef<DataBackupReportPlanRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataBackupReportPlan_ {
    fn extract_datasource_type(&self) -> String {
        "aws_backup_report_plan".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataBackupReportPlan {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
}
impl BuildDataBackupReportPlan {
    pub fn build(self, stack: &mut Stack) -> DataBackupReportPlan {
        let out = DataBackupReportPlan(Rc::new(DataBackupReportPlan_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataBackupReportPlanData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataBackupReportPlanRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBackupReportPlanRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataBackupReportPlanRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `creation_time` after provisioning.\n"]
    pub fn creation_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creation_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deployment_status` after provisioning.\n"]
    pub fn deployment_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deployment_status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `report_delivery_channel` after provisioning.\n"]
    pub fn report_delivery_channel(
        &self,
    ) -> ListRef<DataBackupReportPlanReportDeliveryChannelElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.report_delivery_channel", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `report_setting` after provisioning.\n"]
    pub fn report_setting(&self) -> ListRef<DataBackupReportPlanReportSettingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.report_setting", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataBackupReportPlanReportDeliveryChannelEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    formats: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_bucket_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_key_prefix: Option<PrimField<String>>,
}
impl DataBackupReportPlanReportDeliveryChannelEl {
    #[doc = "Set the field `formats`.\n"]
    pub fn set_formats(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.formats = Some(v.into());
        self
    }
    #[doc = "Set the field `s3_bucket_name`.\n"]
    pub fn set_s3_bucket_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.s3_bucket_name = Some(v.into());
        self
    }
    #[doc = "Set the field `s3_key_prefix`.\n"]
    pub fn set_s3_key_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.s3_key_prefix = Some(v.into());
        self
    }
}
impl ToListMappable for DataBackupReportPlanReportDeliveryChannelEl {
    type O = BlockAssignable<DataBackupReportPlanReportDeliveryChannelEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBackupReportPlanReportDeliveryChannelEl {}
impl BuildDataBackupReportPlanReportDeliveryChannelEl {
    pub fn build(self) -> DataBackupReportPlanReportDeliveryChannelEl {
        DataBackupReportPlanReportDeliveryChannelEl {
            formats: core::default::Default::default(),
            s3_bucket_name: core::default::Default::default(),
            s3_key_prefix: core::default::Default::default(),
        }
    }
}
pub struct DataBackupReportPlanReportDeliveryChannelElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBackupReportPlanReportDeliveryChannelElRef {
    fn new(shared: StackShared, base: String) -> DataBackupReportPlanReportDeliveryChannelElRef {
        DataBackupReportPlanReportDeliveryChannelElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBackupReportPlanReportDeliveryChannelElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `formats` after provisioning.\n"]
    pub fn formats(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.formats", self.base))
    }
    #[doc = "Get a reference to the value of field `s3_bucket_name` after provisioning.\n"]
    pub fn s3_bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.s3_bucket_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `s3_key_prefix` after provisioning.\n"]
    pub fn s3_key_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.s3_key_prefix", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataBackupReportPlanReportSettingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    accounts: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    framework_arns: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    number_of_frameworks: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    organization_units: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    regions: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    report_template: Option<PrimField<String>>,
}
impl DataBackupReportPlanReportSettingEl {
    #[doc = "Set the field `accounts`.\n"]
    pub fn set_accounts(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.accounts = Some(v.into());
        self
    }
    #[doc = "Set the field `framework_arns`.\n"]
    pub fn set_framework_arns(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.framework_arns = Some(v.into());
        self
    }
    #[doc = "Set the field `number_of_frameworks`.\n"]
    pub fn set_number_of_frameworks(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.number_of_frameworks = Some(v.into());
        self
    }
    #[doc = "Set the field `organization_units`.\n"]
    pub fn set_organization_units(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.organization_units = Some(v.into());
        self
    }
    #[doc = "Set the field `regions`.\n"]
    pub fn set_regions(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.regions = Some(v.into());
        self
    }
    #[doc = "Set the field `report_template`.\n"]
    pub fn set_report_template(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.report_template = Some(v.into());
        self
    }
}
impl ToListMappable for DataBackupReportPlanReportSettingEl {
    type O = BlockAssignable<DataBackupReportPlanReportSettingEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBackupReportPlanReportSettingEl {}
impl BuildDataBackupReportPlanReportSettingEl {
    pub fn build(self) -> DataBackupReportPlanReportSettingEl {
        DataBackupReportPlanReportSettingEl {
            accounts: core::default::Default::default(),
            framework_arns: core::default::Default::default(),
            number_of_frameworks: core::default::Default::default(),
            organization_units: core::default::Default::default(),
            regions: core::default::Default::default(),
            report_template: core::default::Default::default(),
        }
    }
}
pub struct DataBackupReportPlanReportSettingElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBackupReportPlanReportSettingElRef {
    fn new(shared: StackShared, base: String) -> DataBackupReportPlanReportSettingElRef {
        DataBackupReportPlanReportSettingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBackupReportPlanReportSettingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `accounts` after provisioning.\n"]
    pub fn accounts(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.accounts", self.base))
    }
    #[doc = "Get a reference to the value of field `framework_arns` after provisioning.\n"]
    pub fn framework_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.framework_arns", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `number_of_frameworks` after provisioning.\n"]
    pub fn number_of_frameworks(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.number_of_frameworks", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `organization_units` after provisioning.\n"]
    pub fn organization_units(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.organization_units", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `regions` after provisioning.\n"]
    pub fn regions(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.regions", self.base))
    }
    #[doc = "Get a reference to the value of field `report_template` after provisioning.\n"]
    pub fn report_template(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.report_template", self.base),
        )
    }
}
