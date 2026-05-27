use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataSqlCaCertsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    instance: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
}
struct DataSqlCaCerts_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataSqlCaCertsData>,
}
#[derive(Clone)]
pub struct DataSqlCaCerts(Rc<DataSqlCaCerts_>);
impl DataSqlCaCerts {
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
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `active_version` after provisioning.\n"]
    pub fn active_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.active_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `certs` after provisioning.\n"]
    pub fn certs(&self) -> ListRef<DataSqlCaCertsCertsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.certs", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `instance` after provisioning.\n"]
    pub fn instance(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance", self.extract_ref()),
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
impl Referable for DataSqlCaCerts {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataSqlCaCerts {}
impl ToListMappable for DataSqlCaCerts {
    type O = ListRef<DataSqlCaCertsRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataSqlCaCerts_ {
    fn extract_datasource_type(&self) -> String {
        "google_sql_ca_certs".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataSqlCaCerts {
    pub tf_id: String,
    #[doc = ""]
    pub instance: PrimField<String>,
}
impl BuildDataSqlCaCerts {
    pub fn build(self, stack: &mut Stack) -> DataSqlCaCerts {
        let out = DataSqlCaCerts(Rc::new(DataSqlCaCerts_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataSqlCaCertsData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                instance: self.instance,
                project: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataSqlCaCertsRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataSqlCaCertsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataSqlCaCertsRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `active_version` after provisioning.\n"]
    pub fn active_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.active_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `certs` after provisioning.\n"]
    pub fn certs(&self) -> ListRef<DataSqlCaCertsCertsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.certs", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `instance` after provisioning.\n"]
    pub fn instance(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance", self.extract_ref()),
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
pub struct DataSqlCaCertsCertsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cert: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    common_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    create_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expiration_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sha1_fingerprint: Option<PrimField<String>>,
}
impl DataSqlCaCertsCertsEl {
    #[doc = "Set the field `cert`.\n"]
    pub fn set_cert(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cert = Some(v.into());
        self
    }
    #[doc = "Set the field `common_name`.\n"]
    pub fn set_common_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.common_name = Some(v.into());
        self
    }
    #[doc = "Set the field `create_time`.\n"]
    pub fn set_create_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create_time = Some(v.into());
        self
    }
    #[doc = "Set the field `expiration_time`.\n"]
    pub fn set_expiration_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.expiration_time = Some(v.into());
        self
    }
    #[doc = "Set the field `sha1_fingerprint`.\n"]
    pub fn set_sha1_fingerprint(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sha1_fingerprint = Some(v.into());
        self
    }
}
impl ToListMappable for DataSqlCaCertsCertsEl {
    type O = BlockAssignable<DataSqlCaCertsCertsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataSqlCaCertsCertsEl {}
impl BuildDataSqlCaCertsCertsEl {
    pub fn build(self) -> DataSqlCaCertsCertsEl {
        DataSqlCaCertsCertsEl {
            cert: core::default::Default::default(),
            common_name: core::default::Default::default(),
            create_time: core::default::Default::default(),
            expiration_time: core::default::Default::default(),
            sha1_fingerprint: core::default::Default::default(),
        }
    }
}
pub struct DataSqlCaCertsCertsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataSqlCaCertsCertsElRef {
    fn new(shared: StackShared, base: String) -> DataSqlCaCertsCertsElRef {
        DataSqlCaCertsCertsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataSqlCaCertsCertsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cert` after provisioning.\n"]
    pub fn cert(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cert", self.base))
    }
    #[doc = "Get a reference to the value of field `common_name` after provisioning.\n"]
    pub fn common_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.common_name", self.base))
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.base))
    }
    #[doc = "Get a reference to the value of field `expiration_time` after provisioning.\n"]
    pub fn expiration_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.expiration_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `sha1_fingerprint` after provisioning.\n"]
    pub fn sha1_fingerprint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sha1_fingerprint", self.base),
        )
    }
}
