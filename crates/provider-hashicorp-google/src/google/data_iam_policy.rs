use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataIamPolicyData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    audit_config: Option<Vec<DataIamPolicyAuditConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    binding: Option<Vec<DataIamPolicyBindingEl>>,
    dynamic: DataIamPolicyDynamic,
}
struct DataIamPolicy_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataIamPolicyData>,
}
#[derive(Clone)]
pub struct DataIamPolicy(Rc<DataIamPolicy_>);
impl DataIamPolicy {
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
    #[doc = "Set the field `audit_config`.\n"]
    pub fn set_audit_config(
        self,
        v: impl Into<BlockAssignable<DataIamPolicyAuditConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().audit_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.audit_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `binding`.\n"]
    pub fn set_binding(self, v: impl Into<BlockAssignable<DataIamPolicyBindingEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().binding = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.binding = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `policy_data` after provisioning.\n"]
    pub fn policy_data(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.policy_data", self.extract_ref()),
        )
    }
}
impl Referable for DataIamPolicy {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataIamPolicy {}
impl ToListMappable for DataIamPolicy {
    type O = ListRef<DataIamPolicyRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataIamPolicy_ {
    fn extract_datasource_type(&self) -> String {
        "google_iam_policy".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataIamPolicy {
    pub tf_id: String,
}
impl BuildDataIamPolicy {
    pub fn build(self, stack: &mut Stack) -> DataIamPolicy {
        let out = DataIamPolicy(Rc::new(DataIamPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataIamPolicyData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                audit_config: core::default::Default::default(),
                binding: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataIamPolicyRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataIamPolicyRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataIamPolicyRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `policy_data` after provisioning.\n"]
    pub fn policy_data(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.policy_data", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataIamPolicyAuditConfigElAuditLogConfigsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    exempted_members: Option<SetField<PrimField<String>>>,
    log_type: PrimField<String>,
}
impl DataIamPolicyAuditConfigElAuditLogConfigsEl {
    #[doc = "Set the field `exempted_members`.\n"]
    pub fn set_exempted_members(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.exempted_members = Some(v.into());
        self
    }
}
impl ToListMappable for DataIamPolicyAuditConfigElAuditLogConfigsEl {
    type O = BlockAssignable<DataIamPolicyAuditConfigElAuditLogConfigsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataIamPolicyAuditConfigElAuditLogConfigsEl {
    #[doc = ""]
    pub log_type: PrimField<String>,
}
impl BuildDataIamPolicyAuditConfigElAuditLogConfigsEl {
    pub fn build(self) -> DataIamPolicyAuditConfigElAuditLogConfigsEl {
        DataIamPolicyAuditConfigElAuditLogConfigsEl {
            exempted_members: core::default::Default::default(),
            log_type: self.log_type,
        }
    }
}
pub struct DataIamPolicyAuditConfigElAuditLogConfigsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataIamPolicyAuditConfigElAuditLogConfigsElRef {
    fn new(shared: StackShared, base: String) -> DataIamPolicyAuditConfigElAuditLogConfigsElRef {
        DataIamPolicyAuditConfigElAuditLogConfigsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataIamPolicyAuditConfigElAuditLogConfigsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `exempted_members` after provisioning.\n"]
    pub fn exempted_members(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.exempted_members", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `log_type` after provisioning.\n"]
    pub fn log_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_type", self.base))
    }
}
#[derive(Serialize, Default)]
struct DataIamPolicyAuditConfigElDynamic {
    audit_log_configs: Option<DynamicBlock<DataIamPolicyAuditConfigElAuditLogConfigsEl>>,
}
#[derive(Serialize)]
pub struct DataIamPolicyAuditConfigEl {
    service: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    audit_log_configs: Option<Vec<DataIamPolicyAuditConfigElAuditLogConfigsEl>>,
    dynamic: DataIamPolicyAuditConfigElDynamic,
}
impl DataIamPolicyAuditConfigEl {
    #[doc = "Set the field `audit_log_configs`.\n"]
    pub fn set_audit_log_configs(
        mut self,
        v: impl Into<BlockAssignable<DataIamPolicyAuditConfigElAuditLogConfigsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.audit_log_configs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.audit_log_configs = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataIamPolicyAuditConfigEl {
    type O = BlockAssignable<DataIamPolicyAuditConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataIamPolicyAuditConfigEl {
    #[doc = ""]
    pub service: PrimField<String>,
}
impl BuildDataIamPolicyAuditConfigEl {
    pub fn build(self) -> DataIamPolicyAuditConfigEl {
        DataIamPolicyAuditConfigEl {
            service: self.service,
            audit_log_configs: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataIamPolicyAuditConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataIamPolicyAuditConfigElRef {
    fn new(shared: StackShared, base: String) -> DataIamPolicyAuditConfigElRef {
        DataIamPolicyAuditConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataIamPolicyAuditConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `service` after provisioning.\n"]
    pub fn service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
}
#[derive(Serialize)]
pub struct DataIamPolicyBindingElConditionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    expression: PrimField<String>,
    title: PrimField<String>,
}
impl DataIamPolicyBindingElConditionEl {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
}
impl ToListMappable for DataIamPolicyBindingElConditionEl {
    type O = BlockAssignable<DataIamPolicyBindingElConditionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataIamPolicyBindingElConditionEl {
    #[doc = ""]
    pub expression: PrimField<String>,
    #[doc = ""]
    pub title: PrimField<String>,
}
impl BuildDataIamPolicyBindingElConditionEl {
    pub fn build(self) -> DataIamPolicyBindingElConditionEl {
        DataIamPolicyBindingElConditionEl {
            description: core::default::Default::default(),
            expression: self.expression,
            title: self.title,
        }
    }
}
pub struct DataIamPolicyBindingElConditionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataIamPolicyBindingElConditionElRef {
    fn new(shared: StackShared, base: String) -> DataIamPolicyBindingElConditionElRef {
        DataIamPolicyBindingElConditionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataIamPolicyBindingElConditionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `expression` after provisioning.\n"]
    pub fn expression(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }
    #[doc = "Get a reference to the value of field `title` after provisioning.\n"]
    pub fn title(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.title", self.base))
    }
}
#[derive(Serialize, Default)]
struct DataIamPolicyBindingElDynamic {
    condition: Option<DynamicBlock<DataIamPolicyBindingElConditionEl>>,
}
#[derive(Serialize)]
pub struct DataIamPolicyBindingEl {
    members: SetField<PrimField<String>>,
    role: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    condition: Option<Vec<DataIamPolicyBindingElConditionEl>>,
    dynamic: DataIamPolicyBindingElDynamic,
}
impl DataIamPolicyBindingEl {
    #[doc = "Set the field `condition`.\n"]
    pub fn set_condition(
        mut self,
        v: impl Into<BlockAssignable<DataIamPolicyBindingElConditionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.condition = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.condition = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataIamPolicyBindingEl {
    type O = BlockAssignable<DataIamPolicyBindingEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataIamPolicyBindingEl {
    #[doc = ""]
    pub members: SetField<PrimField<String>>,
    #[doc = ""]
    pub role: PrimField<String>,
}
impl BuildDataIamPolicyBindingEl {
    pub fn build(self) -> DataIamPolicyBindingEl {
        DataIamPolicyBindingEl {
            members: self.members,
            role: self.role,
            condition: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataIamPolicyBindingElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataIamPolicyBindingElRef {
    fn new(shared: StackShared, base: String) -> DataIamPolicyBindingElRef {
        DataIamPolicyBindingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataIamPolicyBindingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `members` after provisioning.\n"]
    pub fn members(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.members", self.base))
    }
    #[doc = "Get a reference to the value of field `role` after provisioning.\n"]
    pub fn role(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role", self.base))
    }
    #[doc = "Get a reference to the value of field `condition` after provisioning.\n"]
    pub fn condition(&self) -> ListRef<DataIamPolicyBindingElConditionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.condition", self.base))
    }
}
#[derive(Serialize, Default)]
struct DataIamPolicyDynamic {
    audit_config: Option<DynamicBlock<DataIamPolicyAuditConfigEl>>,
    binding: Option<DynamicBlock<DataIamPolicyBindingEl>>,
}
