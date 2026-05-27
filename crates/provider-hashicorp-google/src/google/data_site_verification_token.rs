use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataSiteVerificationTokenData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    identifier: PrimField<String>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    verification_method: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DataSiteVerificationTokenTimeoutsEl>,
}
struct DataSiteVerificationToken_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataSiteVerificationTokenData>,
}
#[derive(Clone)]
pub struct DataSiteVerificationToken(Rc<DataSiteVerificationToken_>);
impl DataSiteVerificationToken {
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
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DataSiteVerificationTokenTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `identifier` after provisioning.\nThe site identifier. If the type is set to SITE, the identifier is a URL. If the type is\nset to INET_DOMAIN, the identifier is a domain name."]
    pub fn identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.identifier", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `token` after provisioning.\nThe returned token for use in subsequent verification steps."]
    pub fn token(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.token", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe type of resource to be verified, either a domain or a web site. Possible values: [\"INET_DOMAIN\", \"SITE\"]"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `verification_method` after provisioning.\nThe verification method for the Site Verification system to use to verify\nthis site or domain. Possible values: [\"ANALYTICS\", \"DNS_CNAME\", \"DNS_TXT\", \"FILE\", \"META\", \"TAG_MANAGER\"]"]
    pub fn verification_method(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.verification_method", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataSiteVerificationTokenTimeoutsElRef {
        DataSiteVerificationTokenTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for DataSiteVerificationToken {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataSiteVerificationToken {}
impl ToListMappable for DataSiteVerificationToken {
    type O = ListRef<DataSiteVerificationTokenRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataSiteVerificationToken_ {
    fn extract_datasource_type(&self) -> String {
        "google_site_verification_token".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataSiteVerificationToken {
    pub tf_id: String,
    #[doc = "The site identifier. If the type is set to SITE, the identifier is a URL. If the type is\nset to INET_DOMAIN, the identifier is a domain name."]
    pub identifier: PrimField<String>,
    #[doc = "The type of resource to be verified, either a domain or a web site. Possible values: [\"INET_DOMAIN\", \"SITE\"]"]
    pub type_: PrimField<String>,
    #[doc = "The verification method for the Site Verification system to use to verify\nthis site or domain. Possible values: [\"ANALYTICS\", \"DNS_CNAME\", \"DNS_TXT\", \"FILE\", \"META\", \"TAG_MANAGER\"]"]
    pub verification_method: PrimField<String>,
}
impl BuildDataSiteVerificationToken {
    pub fn build(self, stack: &mut Stack) -> DataSiteVerificationToken {
        let out = DataSiteVerificationToken(Rc::new(DataSiteVerificationToken_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataSiteVerificationTokenData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                identifier: self.identifier,
                type_: self.type_,
                verification_method: self.verification_method,
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataSiteVerificationTokenRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataSiteVerificationTokenRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataSiteVerificationTokenRef {
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
    #[doc = "Get a reference to the value of field `identifier` after provisioning.\nThe site identifier. If the type is set to SITE, the identifier is a URL. If the type is\nset to INET_DOMAIN, the identifier is a domain name."]
    pub fn identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.identifier", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `token` after provisioning.\nThe returned token for use in subsequent verification steps."]
    pub fn token(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.token", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe type of resource to be verified, either a domain or a web site. Possible values: [\"INET_DOMAIN\", \"SITE\"]"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `verification_method` after provisioning.\nThe verification method for the Site Verification system to use to verify\nthis site or domain. Possible values: [\"ANALYTICS\", \"DNS_CNAME\", \"DNS_TXT\", \"FILE\", \"META\", \"TAG_MANAGER\"]"]
    pub fn verification_method(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.verification_method", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataSiteVerificationTokenTimeoutsElRef {
        DataSiteVerificationTokenTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataSiteVerificationTokenTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    read: Option<PrimField<String>>,
}
impl DataSiteVerificationTokenTimeoutsEl {
    #[doc = "Set the field `read`.\n"]
    pub fn set_read(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.read = Some(v.into());
        self
    }
}
impl ToListMappable for DataSiteVerificationTokenTimeoutsEl {
    type O = BlockAssignable<DataSiteVerificationTokenTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataSiteVerificationTokenTimeoutsEl {}
impl BuildDataSiteVerificationTokenTimeoutsEl {
    pub fn build(self) -> DataSiteVerificationTokenTimeoutsEl {
        DataSiteVerificationTokenTimeoutsEl {
            read: core::default::Default::default(),
        }
    }
}
pub struct DataSiteVerificationTokenTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataSiteVerificationTokenTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DataSiteVerificationTokenTimeoutsElRef {
        DataSiteVerificationTokenTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataSiteVerificationTokenTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }
}
