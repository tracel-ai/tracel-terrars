use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct BackupReportPlanData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    report_delivery_channel: Option<Vec<BackupReportPlanReportDeliveryChannelEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    report_setting: Option<Vec<BackupReportPlanReportSettingEl>>,
    dynamic: BackupReportPlanDynamic,
}

struct BackupReportPlan_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<BackupReportPlanData>,
}

#[derive(Clone)]
pub struct BackupReportPlan(Rc<BackupReportPlan_>);

impl BackupReportPlan {
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
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
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags_all = Some(v.into());
        self
    }

    #[doc = "Set the field `report_delivery_channel`.\n"]
    pub fn set_report_delivery_channel(
        self,
        v: impl Into<BlockAssignable<BackupReportPlanReportDeliveryChannelEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().report_delivery_channel = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.report_delivery_channel = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `report_setting`.\n"]
    pub fn set_report_setting(self, v: impl Into<BlockAssignable<BackupReportPlanReportSettingEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().report_setting = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.report_setting = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `creation_time` after provisioning.\n"]
    pub fn creation_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.creation_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployment_status` after provisioning.\n"]
    pub fn deployment_status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.deployment_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `report_delivery_channel` after provisioning.\n"]
    pub fn report_delivery_channel(&self) -> ListRef<BackupReportPlanReportDeliveryChannelElRef> {
        ListRef::new(self.shared().clone(), format!("{}.report_delivery_channel", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `report_setting` after provisioning.\n"]
    pub fn report_setting(&self) -> ListRef<BackupReportPlanReportSettingElRef> {
        ListRef::new(self.shared().clone(), format!("{}.report_setting", self.extract_ref()))
    }
}

impl Referable for BackupReportPlan {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for BackupReportPlan { }

impl ToListMappable for BackupReportPlan {
    type O = ListRef<BackupReportPlanRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for BackupReportPlan_ {
    fn extract_resource_type(&self) -> String {
        "aws_backup_report_plan".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBackupReportPlan {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildBackupReportPlan {
    pub fn build(self, stack: &mut Stack) -> BackupReportPlan {
        let out = BackupReportPlan(Rc::new(BackupReportPlan_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(BackupReportPlanData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                report_delivery_channel: core::default::Default::default(),
                report_setting: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BackupReportPlanRef {
    shared: StackShared,
    base: String,
}

impl Ref for BackupReportPlanRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl BackupReportPlanRef {
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

    #[doc = "Get a reference to the value of field `creation_time` after provisioning.\n"]
    pub fn creation_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.creation_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployment_status` after provisioning.\n"]
    pub fn deployment_status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.deployment_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `report_delivery_channel` after provisioning.\n"]
    pub fn report_delivery_channel(&self) -> ListRef<BackupReportPlanReportDeliveryChannelElRef> {
        ListRef::new(self.shared().clone(), format!("{}.report_delivery_channel", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `report_setting` after provisioning.\n"]
    pub fn report_setting(&self) -> ListRef<BackupReportPlanReportSettingElRef> {
        ListRef::new(self.shared().clone(), format!("{}.report_setting", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct BackupReportPlanReportDeliveryChannelEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    formats: Option<SetField<PrimField<String>>>,
    s3_bucket_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_key_prefix: Option<PrimField<String>>,
}

impl BackupReportPlanReportDeliveryChannelEl {
    #[doc = "Set the field `formats`.\n"]
    pub fn set_formats(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.formats = Some(v.into());
        self
    }

    #[doc = "Set the field `s3_key_prefix`.\n"]
    pub fn set_s3_key_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.s3_key_prefix = Some(v.into());
        self
    }
}

impl ToListMappable for BackupReportPlanReportDeliveryChannelEl {
    type O = BlockAssignable<BackupReportPlanReportDeliveryChannelEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBackupReportPlanReportDeliveryChannelEl {
    #[doc = ""]
    pub s3_bucket_name: PrimField<String>,
}

impl BuildBackupReportPlanReportDeliveryChannelEl {
    pub fn build(self) -> BackupReportPlanReportDeliveryChannelEl {
        BackupReportPlanReportDeliveryChannelEl {
            formats: core::default::Default::default(),
            s3_bucket_name: self.s3_bucket_name,
            s3_key_prefix: core::default::Default::default(),
        }
    }
}

pub struct BackupReportPlanReportDeliveryChannelElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BackupReportPlanReportDeliveryChannelElRef {
    fn new(shared: StackShared, base: String) -> BackupReportPlanReportDeliveryChannelElRef {
        BackupReportPlanReportDeliveryChannelElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BackupReportPlanReportDeliveryChannelElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `formats` after provisioning.\n"]
    pub fn formats(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.formats", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_bucket_name` after provisioning.\n"]
    pub fn s3_bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_key_prefix` after provisioning.\n"]
    pub fn s3_key_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_key_prefix", self.base))
    }
}

#[derive(Serialize)]
pub struct BackupReportPlanReportSettingEl {
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
    report_template: PrimField<String>,
}

impl BackupReportPlanReportSettingEl {
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
}

impl ToListMappable for BackupReportPlanReportSettingEl {
    type O = BlockAssignable<BackupReportPlanReportSettingEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBackupReportPlanReportSettingEl {
    #[doc = ""]
    pub report_template: PrimField<String>,
}

impl BuildBackupReportPlanReportSettingEl {
    pub fn build(self) -> BackupReportPlanReportSettingEl {
        BackupReportPlanReportSettingEl {
            accounts: core::default::Default::default(),
            framework_arns: core::default::Default::default(),
            number_of_frameworks: core::default::Default::default(),
            organization_units: core::default::Default::default(),
            regions: core::default::Default::default(),
            report_template: self.report_template,
        }
    }
}

pub struct BackupReportPlanReportSettingElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BackupReportPlanReportSettingElRef {
    fn new(shared: StackShared, base: String) -> BackupReportPlanReportSettingElRef {
        BackupReportPlanReportSettingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BackupReportPlanReportSettingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `accounts` after provisioning.\n"]
    pub fn accounts(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.accounts", self.base))
    }

    #[doc = "Get a reference to the value of field `framework_arns` after provisioning.\n"]
    pub fn framework_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.framework_arns", self.base))
    }

    #[doc = "Get a reference to the value of field `number_of_frameworks` after provisioning.\n"]
    pub fn number_of_frameworks(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.number_of_frameworks", self.base))
    }

    #[doc = "Get a reference to the value of field `organization_units` after provisioning.\n"]
    pub fn organization_units(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.organization_units", self.base))
    }

    #[doc = "Get a reference to the value of field `regions` after provisioning.\n"]
    pub fn regions(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.regions", self.base))
    }

    #[doc = "Get a reference to the value of field `report_template` after provisioning.\n"]
    pub fn report_template(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.report_template", self.base))
    }
}

#[derive(Serialize, Default)]
struct BackupReportPlanDynamic {
    report_delivery_channel: Option<DynamicBlock<BackupReportPlanReportDeliveryChannelEl>>,
    report_setting: Option<DynamicBlock<BackupReportPlanReportSettingEl>>,
}
