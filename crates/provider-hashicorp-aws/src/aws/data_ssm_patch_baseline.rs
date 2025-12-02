use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataSsmPatchBaselineData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_baseline: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name_prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    operating_system: Option<PrimField<String>>,
    owner: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}
struct DataSsmPatchBaseline_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataSsmPatchBaselineData>,
}
#[derive(Clone)]
pub struct DataSsmPatchBaseline(Rc<DataSsmPatchBaseline_>);
impl DataSsmPatchBaseline {
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
    #[doc = "Set the field `default_baseline`.\n"]
    pub fn set_default_baseline(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().default_baseline = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `name_prefix`.\n"]
    pub fn set_name_prefix(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name_prefix = Some(v.into());
        self
    }
    #[doc = "Set the field `operating_system`.\n"]
    pub fn set_operating_system(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().operating_system = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `approval_rule` after provisioning.\n"]
    pub fn approval_rule(&self) -> ListRef<DataSsmPatchBaselineApprovalRuleElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.approval_rule", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `approved_patches` after provisioning.\n"]
    pub fn approved_patches(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.approved_patches", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `approved_patches_compliance_level` after provisioning.\n"]
    pub fn approved_patches_compliance_level(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.approved_patches_compliance_level", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `approved_patches_enable_non_security` after provisioning.\n"]
    pub fn approved_patches_enable_non_security(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.approved_patches_enable_non_security",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `available_security_updates_compliance_status` after provisioning.\n"]
    pub fn available_security_updates_compliance_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.available_security_updates_compliance_status",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `default_baseline` after provisioning.\n"]
    pub fn default_baseline(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_baseline", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `global_filter` after provisioning.\n"]
    pub fn global_filter(&self) -> ListRef<DataSsmPatchBaselineGlobalFilterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.global_filter", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `json` after provisioning.\n"]
    pub fn json(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.json", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name_prefix` after provisioning.\n"]
    pub fn name_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name_prefix", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `operating_system` after provisioning.\n"]
    pub fn operating_system(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.operating_system", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `owner` after provisioning.\n"]
    pub fn owner(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.owner", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rejected_patches` after provisioning.\n"]
    pub fn rejected_patches(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rejected_patches", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rejected_patches_action` after provisioning.\n"]
    pub fn rejected_patches_action(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rejected_patches_action", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> ListRef<DataSsmPatchBaselineSourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source", self.extract_ref()),
        )
    }
}
impl Referable for DataSsmPatchBaseline {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataSsmPatchBaseline {}
impl ToListMappable for DataSsmPatchBaseline {
    type O = ListRef<DataSsmPatchBaselineRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataSsmPatchBaseline_ {
    fn extract_datasource_type(&self) -> String {
        "aws_ssm_patch_baseline".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataSsmPatchBaseline {
    pub tf_id: String,
    #[doc = ""]
    pub owner: PrimField<String>,
}
impl BuildDataSsmPatchBaseline {
    pub fn build(self, stack: &mut Stack) -> DataSsmPatchBaseline {
        let out = DataSsmPatchBaseline(Rc::new(DataSsmPatchBaseline_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataSsmPatchBaselineData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                default_baseline: core::default::Default::default(),
                id: core::default::Default::default(),
                name_prefix: core::default::Default::default(),
                operating_system: core::default::Default::default(),
                owner: self.owner,
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataSsmPatchBaselineRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataSsmPatchBaselineRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataSsmPatchBaselineRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `approval_rule` after provisioning.\n"]
    pub fn approval_rule(&self) -> ListRef<DataSsmPatchBaselineApprovalRuleElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.approval_rule", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `approved_patches` after provisioning.\n"]
    pub fn approved_patches(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.approved_patches", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `approved_patches_compliance_level` after provisioning.\n"]
    pub fn approved_patches_compliance_level(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.approved_patches_compliance_level", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `approved_patches_enable_non_security` after provisioning.\n"]
    pub fn approved_patches_enable_non_security(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.approved_patches_enable_non_security",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `available_security_updates_compliance_status` after provisioning.\n"]
    pub fn available_security_updates_compliance_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.available_security_updates_compliance_status",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `default_baseline` after provisioning.\n"]
    pub fn default_baseline(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_baseline", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `global_filter` after provisioning.\n"]
    pub fn global_filter(&self) -> ListRef<DataSsmPatchBaselineGlobalFilterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.global_filter", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `json` after provisioning.\n"]
    pub fn json(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.json", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name_prefix` after provisioning.\n"]
    pub fn name_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name_prefix", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `operating_system` after provisioning.\n"]
    pub fn operating_system(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.operating_system", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `owner` after provisioning.\n"]
    pub fn owner(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.owner", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rejected_patches` after provisioning.\n"]
    pub fn rejected_patches(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rejected_patches", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rejected_patches_action` after provisioning.\n"]
    pub fn rejected_patches_action(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rejected_patches_action", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> ListRef<DataSsmPatchBaselineSourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataSsmPatchBaselineApprovalRuleElPatchFilterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<ListField<PrimField<String>>>,
}
impl DataSsmPatchBaselineApprovalRuleElPatchFilterEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }
    #[doc = "Set the field `values`.\n"]
    pub fn set_values(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}
impl ToListMappable for DataSsmPatchBaselineApprovalRuleElPatchFilterEl {
    type O = BlockAssignable<DataSsmPatchBaselineApprovalRuleElPatchFilterEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataSsmPatchBaselineApprovalRuleElPatchFilterEl {}
impl BuildDataSsmPatchBaselineApprovalRuleElPatchFilterEl {
    pub fn build(self) -> DataSsmPatchBaselineApprovalRuleElPatchFilterEl {
        DataSsmPatchBaselineApprovalRuleElPatchFilterEl {
            key: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}
pub struct DataSsmPatchBaselineApprovalRuleElPatchFilterElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataSsmPatchBaselineApprovalRuleElPatchFilterElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataSsmPatchBaselineApprovalRuleElPatchFilterElRef {
        DataSsmPatchBaselineApprovalRuleElPatchFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataSsmPatchBaselineApprovalRuleElPatchFilterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}
#[derive(Serialize)]
pub struct DataSsmPatchBaselineApprovalRuleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    approve_after_days: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    approve_until_date: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    compliance_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_non_security: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    patch_filter: Option<ListField<DataSsmPatchBaselineApprovalRuleElPatchFilterEl>>,
}
impl DataSsmPatchBaselineApprovalRuleEl {
    #[doc = "Set the field `approve_after_days`.\n"]
    pub fn set_approve_after_days(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.approve_after_days = Some(v.into());
        self
    }
    #[doc = "Set the field `approve_until_date`.\n"]
    pub fn set_approve_until_date(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.approve_until_date = Some(v.into());
        self
    }
    #[doc = "Set the field `compliance_level`.\n"]
    pub fn set_compliance_level(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.compliance_level = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_non_security`.\n"]
    pub fn set_enable_non_security(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_non_security = Some(v.into());
        self
    }
    #[doc = "Set the field `patch_filter`.\n"]
    pub fn set_patch_filter(
        mut self,
        v: impl Into<ListField<DataSsmPatchBaselineApprovalRuleElPatchFilterEl>>,
    ) -> Self {
        self.patch_filter = Some(v.into());
        self
    }
}
impl ToListMappable for DataSsmPatchBaselineApprovalRuleEl {
    type O = BlockAssignable<DataSsmPatchBaselineApprovalRuleEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataSsmPatchBaselineApprovalRuleEl {}
impl BuildDataSsmPatchBaselineApprovalRuleEl {
    pub fn build(self) -> DataSsmPatchBaselineApprovalRuleEl {
        DataSsmPatchBaselineApprovalRuleEl {
            approve_after_days: core::default::Default::default(),
            approve_until_date: core::default::Default::default(),
            compliance_level: core::default::Default::default(),
            enable_non_security: core::default::Default::default(),
            patch_filter: core::default::Default::default(),
        }
    }
}
pub struct DataSsmPatchBaselineApprovalRuleElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataSsmPatchBaselineApprovalRuleElRef {
    fn new(shared: StackShared, base: String) -> DataSsmPatchBaselineApprovalRuleElRef {
        DataSsmPatchBaselineApprovalRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataSsmPatchBaselineApprovalRuleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `approve_after_days` after provisioning.\n"]
    pub fn approve_after_days(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.approve_after_days", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `approve_until_date` after provisioning.\n"]
    pub fn approve_until_date(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.approve_until_date", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `compliance_level` after provisioning.\n"]
    pub fn compliance_level(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.compliance_level", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_non_security` after provisioning.\n"]
    pub fn enable_non_security(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_non_security", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `patch_filter` after provisioning.\n"]
    pub fn patch_filter(&self) -> ListRef<DataSsmPatchBaselineApprovalRuleElPatchFilterElRef> {
        ListRef::new(self.shared().clone(), format!("{}.patch_filter", self.base))
    }
}
#[derive(Serialize)]
pub struct DataSsmPatchBaselineGlobalFilterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<ListField<PrimField<String>>>,
}
impl DataSsmPatchBaselineGlobalFilterEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }
    #[doc = "Set the field `values`.\n"]
    pub fn set_values(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}
impl ToListMappable for DataSsmPatchBaselineGlobalFilterEl {
    type O = BlockAssignable<DataSsmPatchBaselineGlobalFilterEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataSsmPatchBaselineGlobalFilterEl {}
impl BuildDataSsmPatchBaselineGlobalFilterEl {
    pub fn build(self) -> DataSsmPatchBaselineGlobalFilterEl {
        DataSsmPatchBaselineGlobalFilterEl {
            key: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}
pub struct DataSsmPatchBaselineGlobalFilterElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataSsmPatchBaselineGlobalFilterElRef {
    fn new(shared: StackShared, base: String) -> DataSsmPatchBaselineGlobalFilterElRef {
        DataSsmPatchBaselineGlobalFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataSsmPatchBaselineGlobalFilterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}
#[derive(Serialize)]
pub struct DataSsmPatchBaselineSourceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    configuration: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    products: Option<ListField<PrimField<String>>>,
}
impl DataSsmPatchBaselineSourceEl {
    #[doc = "Set the field `configuration`.\n"]
    pub fn set_configuration(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.configuration = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `products`.\n"]
    pub fn set_products(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.products = Some(v.into());
        self
    }
}
impl ToListMappable for DataSsmPatchBaselineSourceEl {
    type O = BlockAssignable<DataSsmPatchBaselineSourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataSsmPatchBaselineSourceEl {}
impl BuildDataSsmPatchBaselineSourceEl {
    pub fn build(self) -> DataSsmPatchBaselineSourceEl {
        DataSsmPatchBaselineSourceEl {
            configuration: core::default::Default::default(),
            name: core::default::Default::default(),
            products: core::default::Default::default(),
        }
    }
}
pub struct DataSsmPatchBaselineSourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataSsmPatchBaselineSourceElRef {
    fn new(shared: StackShared, base: String) -> DataSsmPatchBaselineSourceElRef {
        DataSsmPatchBaselineSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataSsmPatchBaselineSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.configuration", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `products` after provisioning.\n"]
    pub fn products(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.products", self.base))
    }
}
