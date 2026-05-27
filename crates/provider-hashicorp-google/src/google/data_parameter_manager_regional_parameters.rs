use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataParameterManagerRegionalParametersData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
}
struct DataParameterManagerRegionalParameters_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataParameterManagerRegionalParametersData>,
}
#[derive(Clone)]
pub struct DataParameterManagerRegionalParameters(Rc<DataParameterManagerRegionalParameters_>);
impl DataParameterManagerRegionalParameters {
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
    #[doc = "Set the field `filter`.\nFilter string, adhering to the rules in List-operation filtering. List only parameters matching the filter. \nIf filter is empty, all regional parameters are listed from specific location."]
    pub fn set_filter(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().filter = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `filter` after provisioning.\nFilter string, adhering to the rules in List-operation filtering. List only parameters matching the filter. \nIf filter is empty, all regional parameters are listed from specific location."]
    pub fn filter(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.filter", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> ListRef<DataParameterManagerRegionalParametersParametersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.parameters", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
}
impl Referable for DataParameterManagerRegionalParameters {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataParameterManagerRegionalParameters {}
impl ToListMappable for DataParameterManagerRegionalParameters {
    type O = ListRef<DataParameterManagerRegionalParametersRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataParameterManagerRegionalParameters_ {
    fn extract_datasource_type(&self) -> String {
        "google_parameter_manager_regional_parameters".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataParameterManagerRegionalParameters {
    pub tf_id: String,
    #[doc = ""]
    pub location: PrimField<String>,
}
impl BuildDataParameterManagerRegionalParameters {
    pub fn build(self, stack: &mut Stack) -> DataParameterManagerRegionalParameters {
        let out = DataParameterManagerRegionalParameters(Rc::new(
            DataParameterManagerRegionalParameters_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(DataParameterManagerRegionalParametersData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    for_each: None,
                    filter: core::default::Default::default(),
                    id: core::default::Default::default(),
                    location: self.location,
                    project: core::default::Default::default(),
                }),
            },
        ));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataParameterManagerRegionalParametersRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataParameterManagerRegionalParametersRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataParameterManagerRegionalParametersRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `filter` after provisioning.\nFilter string, adhering to the rules in List-operation filtering. List only parameters matching the filter. \nIf filter is empty, all regional parameters are listed from specific location."]
    pub fn filter(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.filter", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> ListRef<DataParameterManagerRegionalParametersParametersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.parameters", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataParameterManagerRegionalParametersParametersElPolicyMemberEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    iam_policy_name_principal: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iam_policy_uid_principal: Option<PrimField<String>>,
}
impl DataParameterManagerRegionalParametersParametersElPolicyMemberEl {
    #[doc = "Set the field `iam_policy_name_principal`.\n"]
    pub fn set_iam_policy_name_principal(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.iam_policy_name_principal = Some(v.into());
        self
    }
    #[doc = "Set the field `iam_policy_uid_principal`.\n"]
    pub fn set_iam_policy_uid_principal(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.iam_policy_uid_principal = Some(v.into());
        self
    }
}
impl ToListMappable for DataParameterManagerRegionalParametersParametersElPolicyMemberEl {
    type O = BlockAssignable<DataParameterManagerRegionalParametersParametersElPolicyMemberEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataParameterManagerRegionalParametersParametersElPolicyMemberEl {}
impl BuildDataParameterManagerRegionalParametersParametersElPolicyMemberEl {
    pub fn build(self) -> DataParameterManagerRegionalParametersParametersElPolicyMemberEl {
        DataParameterManagerRegionalParametersParametersElPolicyMemberEl {
            iam_policy_name_principal: core::default::Default::default(),
            iam_policy_uid_principal: core::default::Default::default(),
        }
    }
}
pub struct DataParameterManagerRegionalParametersParametersElPolicyMemberElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataParameterManagerRegionalParametersParametersElPolicyMemberElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataParameterManagerRegionalParametersParametersElPolicyMemberElRef {
        DataParameterManagerRegionalParametersParametersElPolicyMemberElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataParameterManagerRegionalParametersParametersElPolicyMemberElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `iam_policy_name_principal` after provisioning.\n"]
    pub fn iam_policy_name_principal(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.iam_policy_name_principal", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `iam_policy_uid_principal` after provisioning.\n"]
    pub fn iam_policy_uid_principal(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.iam_policy_uid_principal", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataParameterManagerRegionalParametersParametersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    effective_labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    format: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameter_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    policy_member:
        Option<ListField<DataParameterManagerRegionalParametersParametersElPolicyMemberEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    terraform_labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update_time: Option<PrimField<String>>,
}
impl DataParameterManagerRegionalParametersParametersEl {
    #[doc = "Set the field `create_time`.\n"]
    pub fn set_create_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create_time = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\n"]
    pub fn set_deletion_policy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `effective_labels`.\n"]
    pub fn set_effective_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.effective_labels = Some(v.into());
        self
    }
    #[doc = "Set the field `format`.\n"]
    pub fn set_format(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.format = Some(v.into());
        self
    }
    #[doc = "Set the field `kms_key`.\n"]
    pub fn set_kms_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\n"]
    pub fn set_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.labels = Some(v.into());
        self
    }
    #[doc = "Set the field `location`.\n"]
    pub fn set_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.location = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `parameter_id`.\n"]
    pub fn set_parameter_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.parameter_id = Some(v.into());
        self
    }
    #[doc = "Set the field `policy_member`.\n"]
    pub fn set_policy_member(
        mut self,
        v: impl Into<ListField<DataParameterManagerRegionalParametersParametersElPolicyMemberEl>>,
    ) -> Self {
        self.policy_member = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.project = Some(v.into());
        self
    }
    #[doc = "Set the field `terraform_labels`.\n"]
    pub fn set_terraform_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.terraform_labels = Some(v.into());
        self
    }
    #[doc = "Set the field `update_time`.\n"]
    pub fn set_update_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update_time = Some(v.into());
        self
    }
}
impl ToListMappable for DataParameterManagerRegionalParametersParametersEl {
    type O = BlockAssignable<DataParameterManagerRegionalParametersParametersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataParameterManagerRegionalParametersParametersEl {}
impl BuildDataParameterManagerRegionalParametersParametersEl {
    pub fn build(self) -> DataParameterManagerRegionalParametersParametersEl {
        DataParameterManagerRegionalParametersParametersEl {
            create_time: core::default::Default::default(),
            deletion_policy: core::default::Default::default(),
            effective_labels: core::default::Default::default(),
            format: core::default::Default::default(),
            kms_key: core::default::Default::default(),
            labels: core::default::Default::default(),
            location: core::default::Default::default(),
            name: core::default::Default::default(),
            parameter_id: core::default::Default::default(),
            policy_member: core::default::Default::default(),
            project: core::default::Default::default(),
            terraform_labels: core::default::Default::default(),
            update_time: core::default::Default::default(),
        }
    }
}
pub struct DataParameterManagerRegionalParametersParametersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataParameterManagerRegionalParametersParametersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataParameterManagerRegionalParametersParametersElRef {
        DataParameterManagerRegionalParametersParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataParameterManagerRegionalParametersParametersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.base))
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\n"]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `format` after provisioning.\n"]
    pub fn format(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.format", self.base))
    }
    #[doc = "Get a reference to the value of field `kms_key` after provisioning.\n"]
    pub fn kms_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key", self.base))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\n"]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `parameter_id` after provisioning.\n"]
    pub fn parameter_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.parameter_id", self.base))
    }
    #[doc = "Get a reference to the value of field `policy_member` after provisioning.\n"]
    pub fn policy_member(
        &self,
    ) -> ListRef<DataParameterManagerRegionalParametersParametersElPolicyMemberElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.policy_member", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.base))
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\n"]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n"]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }
}
