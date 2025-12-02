use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataIamRoleData {
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
    tags: Option<RecField<PrimField<String>>>,
}
struct DataIamRole_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataIamRoleData>,
}
#[derive(Clone)]
pub struct DataIamRole(Rc<DataIamRole_>);
impl DataIamRole {
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
    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `assume_role_policy` after provisioning.\n"]
    pub fn assume_role_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.assume_role_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_date` after provisioning.\n"]
    pub fn create_date(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_date", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `max_session_duration` after provisioning.\n"]
    pub fn max_session_duration(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_session_duration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.path", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `permissions_boundary` after provisioning.\n"]
    pub fn permissions_boundary(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.permissions_boundary", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `role_last_used` after provisioning.\n"]
    pub fn role_last_used(&self) -> ListRef<DataIamRoleRoleLastUsedElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.role_last_used", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `unique_id` after provisioning.\n"]
    pub fn unique_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.unique_id", self.extract_ref()),
        )
    }
}
impl Referable for DataIamRole {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataIamRole {}
impl ToListMappable for DataIamRole {
    type O = ListRef<DataIamRoleRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataIamRole_ {
    fn extract_datasource_type(&self) -> String {
        "aws_iam_role".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataIamRole {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
}
impl BuildDataIamRole {
    pub fn build(self, stack: &mut Stack) -> DataIamRole {
        let out = DataIamRole(Rc::new(DataIamRole_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataIamRoleData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                name: self.name,
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataIamRoleRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataIamRoleRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataIamRoleRef {
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
    #[doc = "Get a reference to the value of field `assume_role_policy` after provisioning.\n"]
    pub fn assume_role_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.assume_role_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_date` after provisioning.\n"]
    pub fn create_date(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_date", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `max_session_duration` after provisioning.\n"]
    pub fn max_session_duration(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_session_duration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.path", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `permissions_boundary` after provisioning.\n"]
    pub fn permissions_boundary(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.permissions_boundary", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `role_last_used` after provisioning.\n"]
    pub fn role_last_used(&self) -> ListRef<DataIamRoleRoleLastUsedElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.role_last_used", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `unique_id` after provisioning.\n"]
    pub fn unique_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.unique_id", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataIamRoleRoleLastUsedEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    last_used_date: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}
impl DataIamRoleRoleLastUsedEl {
    #[doc = "Set the field `last_used_date`.\n"]
    pub fn set_last_used_date(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.last_used_date = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\n"]
    pub fn set_region(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.region = Some(v.into());
        self
    }
}
impl ToListMappable for DataIamRoleRoleLastUsedEl {
    type O = BlockAssignable<DataIamRoleRoleLastUsedEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataIamRoleRoleLastUsedEl {}
impl BuildDataIamRoleRoleLastUsedEl {
    pub fn build(self) -> DataIamRoleRoleLastUsedEl {
        DataIamRoleRoleLastUsedEl {
            last_used_date: core::default::Default::default(),
            region: core::default::Default::default(),
        }
    }
}
pub struct DataIamRoleRoleLastUsedElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataIamRoleRoleLastUsedElRef {
    fn new(shared: StackShared, base: String) -> DataIamRoleRoleLastUsedElRef {
        DataIamRoleRoleLastUsedElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataIamRoleRoleLastUsedElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `last_used_date` after provisioning.\n"]
    pub fn last_used_date(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_used_date", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }
}
