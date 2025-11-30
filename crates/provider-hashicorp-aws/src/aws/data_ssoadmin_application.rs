use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct DataSsoadminApplicationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    application_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}

struct DataSsoadminApplication_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataSsoadminApplicationData>,
}

#[derive(Clone)]
pub struct DataSsoadminApplication(Rc<DataSsoadminApplication_>);

impl DataSsoadminApplication {
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

    #[doc = "Get a reference to the value of field `application_account` after provisioning.\n"]
    pub fn application_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.application_account", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `application_arn` after provisioning.\n"]
    pub fn application_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.application_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `application_provider_arn` after provisioning.\n"]
    pub fn application_provider_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.application_provider_arn", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `instance_arn` after provisioning.\n"]
    pub fn instance_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `portal_options` after provisioning.\n"]
    pub fn portal_options(&self) -> ListRef<DataSsoadminApplicationPortalOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.portal_options", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }
}

impl Referable for DataSsoadminApplication {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Datasource for DataSsoadminApplication {}

impl ToListMappable for DataSsoadminApplication {
    type O = ListRef<DataSsoadminApplicationRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataSsoadminApplication_ {
    fn extract_datasource_type(&self) -> String {
        "aws_ssoadmin_application".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataSsoadminApplication {
    pub tf_id: String,
    #[doc = ""]
    pub application_arn: PrimField<String>,
}

impl BuildDataSsoadminApplication {
    pub fn build(self, stack: &mut Stack) -> DataSsoadminApplication {
        let out = DataSsoadminApplication(Rc::new(DataSsoadminApplication_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataSsoadminApplicationData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                application_arn: self.application_arn,
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataSsoadminApplicationRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSsoadminApplicationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl DataSsoadminApplicationRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `application_account` after provisioning.\n"]
    pub fn application_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.application_account", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `application_arn` after provisioning.\n"]
    pub fn application_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.application_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `application_provider_arn` after provisioning.\n"]
    pub fn application_provider_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.application_provider_arn", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `instance_arn` after provisioning.\n"]
    pub fn instance_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `portal_options` after provisioning.\n"]
    pub fn portal_options(&self) -> ListRef<DataSsoadminApplicationPortalOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.portal_options", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DataSsoadminApplicationPortalOptionsElSignInOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    application_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    origin: Option<PrimField<String>>,
}

impl DataSsoadminApplicationPortalOptionsElSignInOptionsEl {
    #[doc = "Set the field `application_url`.\n"]
    pub fn set_application_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.application_url = Some(v.into());
        self
    }

    #[doc = "Set the field `origin`.\n"]
    pub fn set_origin(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.origin = Some(v.into());
        self
    }
}

impl ToListMappable for DataSsoadminApplicationPortalOptionsElSignInOptionsEl {
    type O = BlockAssignable<DataSsoadminApplicationPortalOptionsElSignInOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSsoadminApplicationPortalOptionsElSignInOptionsEl {}

impl BuildDataSsoadminApplicationPortalOptionsElSignInOptionsEl {
    pub fn build(self) -> DataSsoadminApplicationPortalOptionsElSignInOptionsEl {
        DataSsoadminApplicationPortalOptionsElSignInOptionsEl {
            application_url: core::default::Default::default(),
            origin: core::default::Default::default(),
        }
    }
}

pub struct DataSsoadminApplicationPortalOptionsElSignInOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSsoadminApplicationPortalOptionsElSignInOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataSsoadminApplicationPortalOptionsElSignInOptionsElRef {
        DataSsoadminApplicationPortalOptionsElSignInOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSsoadminApplicationPortalOptionsElSignInOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `application_url` after provisioning.\n"]
    pub fn application_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.application_url", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `origin` after provisioning.\n"]
    pub fn origin(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.origin", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSsoadminApplicationPortalOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    sign_in_options: Option<ListField<DataSsoadminApplicationPortalOptionsElSignInOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    visibility: Option<PrimField<String>>,
}

impl DataSsoadminApplicationPortalOptionsEl {
    #[doc = "Set the field `sign_in_options`.\n"]
    pub fn set_sign_in_options(
        mut self,
        v: impl Into<ListField<DataSsoadminApplicationPortalOptionsElSignInOptionsEl>>,
    ) -> Self {
        self.sign_in_options = Some(v.into());
        self
    }

    #[doc = "Set the field `visibility`.\n"]
    pub fn set_visibility(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.visibility = Some(v.into());
        self
    }
}

impl ToListMappable for DataSsoadminApplicationPortalOptionsEl {
    type O = BlockAssignable<DataSsoadminApplicationPortalOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSsoadminApplicationPortalOptionsEl {}

impl BuildDataSsoadminApplicationPortalOptionsEl {
    pub fn build(self) -> DataSsoadminApplicationPortalOptionsEl {
        DataSsoadminApplicationPortalOptionsEl {
            sign_in_options: core::default::Default::default(),
            visibility: core::default::Default::default(),
        }
    }
}

pub struct DataSsoadminApplicationPortalOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSsoadminApplicationPortalOptionsElRef {
    fn new(shared: StackShared, base: String) -> DataSsoadminApplicationPortalOptionsElRef {
        DataSsoadminApplicationPortalOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSsoadminApplicationPortalOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `sign_in_options` after provisioning.\n"]
    pub fn sign_in_options(
        &self,
    ) -> ListRef<DataSsoadminApplicationPortalOptionsElSignInOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.sign_in_options", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `visibility` after provisioning.\n"]
    pub fn visibility(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.visibility", self.base))
    }
}
