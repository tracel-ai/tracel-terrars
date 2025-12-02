use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataIamAccessKeysData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    user: PrimField<String>,
}
struct DataIamAccessKeys_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataIamAccessKeysData>,
}
#[derive(Clone)]
pub struct DataIamAccessKeys(Rc<DataIamAccessKeys_>);
impl DataIamAccessKeys {
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
    #[doc = "Get a reference to the value of field `access_keys` after provisioning.\n"]
    pub fn access_keys(&self) -> SetRef<DataIamAccessKeysAccessKeysElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.access_keys", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `user` after provisioning.\n"]
    pub fn user(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user", self.extract_ref()),
        )
    }
}
impl Referable for DataIamAccessKeys {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataIamAccessKeys {}
impl ToListMappable for DataIamAccessKeys {
    type O = ListRef<DataIamAccessKeysRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataIamAccessKeys_ {
    fn extract_datasource_type(&self) -> String {
        "aws_iam_access_keys".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataIamAccessKeys {
    pub tf_id: String,
    #[doc = ""]
    pub user: PrimField<String>,
}
impl BuildDataIamAccessKeys {
    pub fn build(self, stack: &mut Stack) -> DataIamAccessKeys {
        let out = DataIamAccessKeys(Rc::new(DataIamAccessKeys_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataIamAccessKeysData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                user: self.user,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataIamAccessKeysRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataIamAccessKeysRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataIamAccessKeysRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `access_keys` after provisioning.\n"]
    pub fn access_keys(&self) -> SetRef<DataIamAccessKeysAccessKeysElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.access_keys", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `user` after provisioning.\n"]
    pub fn user(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataIamAccessKeysAccessKeysEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    access_key_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    create_date: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
}
impl DataIamAccessKeysAccessKeysEl {
    #[doc = "Set the field `access_key_id`.\n"]
    pub fn set_access_key_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.access_key_id = Some(v.into());
        self
    }
    #[doc = "Set the field `create_date`.\n"]
    pub fn set_create_date(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create_date = Some(v.into());
        self
    }
    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status = Some(v.into());
        self
    }
}
impl ToListMappable for DataIamAccessKeysAccessKeysEl {
    type O = BlockAssignable<DataIamAccessKeysAccessKeysEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataIamAccessKeysAccessKeysEl {}
impl BuildDataIamAccessKeysAccessKeysEl {
    pub fn build(self) -> DataIamAccessKeysAccessKeysEl {
        DataIamAccessKeysAccessKeysEl {
            access_key_id: core::default::Default::default(),
            create_date: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}
pub struct DataIamAccessKeysAccessKeysElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataIamAccessKeysAccessKeysElRef {
    fn new(shared: StackShared, base: String) -> DataIamAccessKeysAccessKeysElRef {
        DataIamAccessKeysAccessKeysElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataIamAccessKeysAccessKeysElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `access_key_id` after provisioning.\n"]
    pub fn access_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.access_key_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `create_date` after provisioning.\n"]
    pub fn create_date(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_date", self.base))
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}
