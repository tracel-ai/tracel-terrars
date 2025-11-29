use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataSesv2EmailIdentityMailFromAttributesData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    email_identity: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}

struct DataSesv2EmailIdentityMailFromAttributes_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataSesv2EmailIdentityMailFromAttributesData>,
}

#[derive(Clone)]
pub struct DataSesv2EmailIdentityMailFromAttributes(Rc<DataSesv2EmailIdentityMailFromAttributes_>);

impl DataSesv2EmailIdentityMailFromAttributes {
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `behavior_on_mx_failure` after provisioning.\n"]
    pub fn behavior_on_mx_failure(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.behavior_on_mx_failure", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `email_identity` after provisioning.\n"]
    pub fn email_identity(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.email_identity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mail_from_domain` after provisioning.\n"]
    pub fn mail_from_domain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mail_from_domain", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}

impl Referable for DataSesv2EmailIdentityMailFromAttributes {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataSesv2EmailIdentityMailFromAttributes { }

impl ToListMappable for DataSesv2EmailIdentityMailFromAttributes {
    type O = ListRef<DataSesv2EmailIdentityMailFromAttributesRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataSesv2EmailIdentityMailFromAttributes_ {
    fn extract_datasource_type(&self) -> String {
        "aws_sesv2_email_identity_mail_from_attributes".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataSesv2EmailIdentityMailFromAttributes {
    pub tf_id: String,
    #[doc = ""]
    pub email_identity: PrimField<String>,
}

impl BuildDataSesv2EmailIdentityMailFromAttributes {
    pub fn build(self, stack: &mut Stack) -> DataSesv2EmailIdentityMailFromAttributes {
        let out = DataSesv2EmailIdentityMailFromAttributes(Rc::new(DataSesv2EmailIdentityMailFromAttributes_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataSesv2EmailIdentityMailFromAttributesData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                email_identity: self.email_identity,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataSesv2EmailIdentityMailFromAttributesRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSesv2EmailIdentityMailFromAttributesRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataSesv2EmailIdentityMailFromAttributesRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `behavior_on_mx_failure` after provisioning.\n"]
    pub fn behavior_on_mx_failure(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.behavior_on_mx_failure", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `email_identity` after provisioning.\n"]
    pub fn email_identity(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.email_identity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mail_from_domain` after provisioning.\n"]
    pub fn mail_from_domain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mail_from_domain", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}
