use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataIamGroupData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    group_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
}

struct DataIamGroup_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataIamGroupData>,
}

#[derive(Clone)]
pub struct DataIamGroup(Rc<DataIamGroup_>);

impl DataIamGroup {
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

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `group_id` after provisioning.\n"]
    pub fn group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `group_name` after provisioning.\n"]
    pub fn group_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `users` after provisioning.\n"]
    pub fn users(&self) -> ListRef<DataIamGroupUsersElRef> {
        ListRef::new(self.shared().clone(), format!("{}.users", self.extract_ref()))
    }
}

impl Referable for DataIamGroup {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataIamGroup { }

impl ToListMappable for DataIamGroup {
    type O = ListRef<DataIamGroupRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataIamGroup_ {
    fn extract_datasource_type(&self) -> String {
        "aws_iam_group".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataIamGroup {
    pub tf_id: String,
    #[doc = ""]
    pub group_name: PrimField<String>,
}

impl BuildDataIamGroup {
    pub fn build(self, stack: &mut Stack) -> DataIamGroup {
        let out = DataIamGroup(Rc::new(DataIamGroup_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataIamGroupData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                group_name: self.group_name,
                id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataIamGroupRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataIamGroupRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataIamGroupRef {
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

    #[doc = "Get a reference to the value of field `group_id` after provisioning.\n"]
    pub fn group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `group_name` after provisioning.\n"]
    pub fn group_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `users` after provisioning.\n"]
    pub fn users(&self) -> ListRef<DataIamGroupUsersElRef> {
        ListRef::new(self.shared().clone(), format!("{}.users", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataIamGroupUsersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_name: Option<PrimField<String>>,
}

impl DataIamGroupUsersEl {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.arn = Some(v.into());
        self
    }

    #[doc = "Set the field `path`.\n"]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }

    #[doc = "Set the field `user_id`.\n"]
    pub fn set_user_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.user_id = Some(v.into());
        self
    }

    #[doc = "Set the field `user_name`.\n"]
    pub fn set_user_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.user_name = Some(v.into());
        self
    }
}

impl ToListMappable for DataIamGroupUsersEl {
    type O = BlockAssignable<DataIamGroupUsersEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataIamGroupUsersEl {}

impl BuildDataIamGroupUsersEl {
    pub fn build(self) -> DataIamGroupUsersEl {
        DataIamGroupUsersEl {
            arn: core::default::Default::default(),
            path: core::default::Default::default(),
            user_id: core::default::Default::default(),
            user_name: core::default::Default::default(),
        }
    }
}

pub struct DataIamGroupUsersElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataIamGroupUsersElRef {
    fn new(shared: StackShared, base: String) -> DataIamGroupUsersElRef {
        DataIamGroupUsersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataIamGroupUsersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc = "Get a reference to the value of field `user_id` after provisioning.\n"]
    pub fn user_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_id", self.base))
    }

    #[doc = "Get a reference to the value of field `user_name` after provisioning.\n"]
    pub fn user_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_name", self.base))
    }
}
