use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataIamUsersData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name_regex: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path_prefix: Option<PrimField<String>>,
}

struct DataIamUsers_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataIamUsersData>,
}

#[derive(Clone)]
pub struct DataIamUsers(Rc<DataIamUsers_>);

impl DataIamUsers {
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

    #[doc = "Set the field `name_regex`.\n"]
    pub fn set_name_regex(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name_regex = Some(v.into());
        self
    }

    #[doc = "Set the field `path_prefix`.\n"]
    pub fn set_path_prefix(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().path_prefix = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arns` after provisioning.\n"]
    pub fn arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.arns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name_regex` after provisioning.\n"]
    pub fn name_regex(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name_regex", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `names` after provisioning.\n"]
    pub fn names(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.names", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `path_prefix` after provisioning.\n"]
    pub fn path_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path_prefix", self.extract_ref()))
    }
}

impl Referable for DataIamUsers {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataIamUsers { }

impl ToListMappable for DataIamUsers {
    type O = ListRef<DataIamUsersRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataIamUsers_ {
    fn extract_datasource_type(&self) -> String {
        "aws_iam_users".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataIamUsers {
    pub tf_id: String,
}

impl BuildDataIamUsers {
    pub fn build(self, stack: &mut Stack) -> DataIamUsers {
        let out = DataIamUsers(Rc::new(DataIamUsers_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataIamUsersData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                name_regex: core::default::Default::default(),
                path_prefix: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataIamUsersRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataIamUsersRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataIamUsersRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `arns` after provisioning.\n"]
    pub fn arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.arns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name_regex` after provisioning.\n"]
    pub fn name_regex(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name_regex", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `names` after provisioning.\n"]
    pub fn names(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.names", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `path_prefix` after provisioning.\n"]
    pub fn path_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path_prefix", self.extract_ref()))
    }
}
