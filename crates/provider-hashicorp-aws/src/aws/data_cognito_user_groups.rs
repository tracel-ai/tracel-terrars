use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataCognitoUserGroupsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    user_pool_id: PrimField<String>,
}
struct DataCognitoUserGroups_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataCognitoUserGroupsData>,
}
#[derive(Clone)]
pub struct DataCognitoUserGroups(Rc<DataCognitoUserGroups_>);
impl DataCognitoUserGroups {
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
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `groups` after provisioning.\n"]
    pub fn groups(&self) -> ListRef<DataCognitoUserGroupsGroupsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.groups", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `user_pool_id` after provisioning.\n"]
    pub fn user_pool_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_pool_id", self.extract_ref()),
        )
    }
}
impl Referable for DataCognitoUserGroups {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataCognitoUserGroups {}
impl ToListMappable for DataCognitoUserGroups {
    type O = ListRef<DataCognitoUserGroupsRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataCognitoUserGroups_ {
    fn extract_datasource_type(&self) -> String {
        "aws_cognito_user_groups".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataCognitoUserGroups {
    pub tf_id: String,
    #[doc = ""]
    pub user_pool_id: PrimField<String>,
}
impl BuildDataCognitoUserGroups {
    pub fn build(self, stack: &mut Stack) -> DataCognitoUserGroups {
        let out = DataCognitoUserGroups(Rc::new(DataCognitoUserGroups_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataCognitoUserGroupsData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                region: core::default::Default::default(),
                user_pool_id: self.user_pool_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataCognitoUserGroupsRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCognitoUserGroupsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataCognitoUserGroupsRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `groups` after provisioning.\n"]
    pub fn groups(&self) -> ListRef<DataCognitoUserGroupsGroupsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.groups", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `user_pool_id` after provisioning.\n"]
    pub fn user_pool_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_pool_id", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataCognitoUserGroupsGroupsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    precedence: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    role_arn: Option<PrimField<String>>,
}
impl DataCognitoUserGroupsGroupsEl {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `group_name`.\n"]
    pub fn set_group_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.group_name = Some(v.into());
        self
    }
    #[doc = "Set the field `precedence`.\n"]
    pub fn set_precedence(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.precedence = Some(v.into());
        self
    }
    #[doc = "Set the field `role_arn`.\n"]
    pub fn set_role_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.role_arn = Some(v.into());
        self
    }
}
impl ToListMappable for DataCognitoUserGroupsGroupsEl {
    type O = BlockAssignable<DataCognitoUserGroupsGroupsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCognitoUserGroupsGroupsEl {}
impl BuildDataCognitoUserGroupsGroupsEl {
    pub fn build(self) -> DataCognitoUserGroupsGroupsEl {
        DataCognitoUserGroupsGroupsEl {
            description: core::default::Default::default(),
            group_name: core::default::Default::default(),
            precedence: core::default::Default::default(),
            role_arn: core::default::Default::default(),
        }
    }
}
pub struct DataCognitoUserGroupsGroupsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCognitoUserGroupsGroupsElRef {
    fn new(shared: StackShared, base: String) -> DataCognitoUserGroupsGroupsElRef {
        DataCognitoUserGroupsGroupsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCognitoUserGroupsGroupsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `group_name` after provisioning.\n"]
    pub fn group_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_name", self.base))
    }
    #[doc = "Get a reference to the value of field `precedence` after provisioning.\n"]
    pub fn precedence(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.precedence", self.base))
    }
    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }
}
