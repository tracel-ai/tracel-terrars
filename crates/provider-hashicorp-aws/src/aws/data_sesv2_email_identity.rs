use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct DataSesv2EmailIdentityData {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
}

struct DataSesv2EmailIdentity_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataSesv2EmailIdentityData>,
}

#[derive(Clone)]
pub struct DataSesv2EmailIdentity(Rc<DataSesv2EmailIdentity_>);

impl DataSesv2EmailIdentity {
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

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
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

    #[doc = "Get a reference to the value of field `configuration_set_name` after provisioning.\n"]
    pub fn configuration_set_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.configuration_set_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `dkim_signing_attributes` after provisioning.\n"]
    pub fn dkim_signing_attributes(
        &self,
    ) -> ListRef<DataSesv2EmailIdentityDkimSigningAttributesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dkim_signing_attributes", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `email_identity` after provisioning.\n"]
    pub fn email_identity(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.email_identity", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `identity_type` after provisioning.\n"]
    pub fn identity_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.identity_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `verification_status` after provisioning.\n"]
    pub fn verification_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.verification_status", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `verified_for_sending_status` after provisioning.\n"]
    pub fn verified_for_sending_status(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.verified_for_sending_status", self.extract_ref()),
        )
    }
}

impl Referable for DataSesv2EmailIdentity {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Datasource for DataSesv2EmailIdentity {}

impl ToListMappable for DataSesv2EmailIdentity {
    type O = ListRef<DataSesv2EmailIdentityRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataSesv2EmailIdentity_ {
    fn extract_datasource_type(&self) -> String {
        "aws_sesv2_email_identity".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataSesv2EmailIdentity {
    pub tf_id: String,
    #[doc = ""]
    pub email_identity: PrimField<String>,
}

impl BuildDataSesv2EmailIdentity {
    pub fn build(self, stack: &mut Stack) -> DataSesv2EmailIdentity {
        let out = DataSesv2EmailIdentity(Rc::new(DataSesv2EmailIdentity_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataSesv2EmailIdentityData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                email_identity: self.email_identity,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataSesv2EmailIdentityRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSesv2EmailIdentityRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl DataSesv2EmailIdentityRef {
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

    #[doc = "Get a reference to the value of field `configuration_set_name` after provisioning.\n"]
    pub fn configuration_set_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.configuration_set_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `dkim_signing_attributes` after provisioning.\n"]
    pub fn dkim_signing_attributes(
        &self,
    ) -> ListRef<DataSesv2EmailIdentityDkimSigningAttributesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dkim_signing_attributes", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `email_identity` after provisioning.\n"]
    pub fn email_identity(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.email_identity", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `identity_type` after provisioning.\n"]
    pub fn identity_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.identity_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `verification_status` after provisioning.\n"]
    pub fn verification_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.verification_status", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `verified_for_sending_status` after provisioning.\n"]
    pub fn verified_for_sending_status(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.verified_for_sending_status", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DataSesv2EmailIdentityDkimSigningAttributesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    current_signing_key_length: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    domain_signing_private_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    domain_signing_selector: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_key_generation_timestamp: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next_signing_key_length: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    signing_attributes_origin: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tokens: Option<ListField<PrimField<String>>>,
}

impl DataSesv2EmailIdentityDkimSigningAttributesEl {
    #[doc = "Set the field `current_signing_key_length`.\n"]
    pub fn set_current_signing_key_length(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.current_signing_key_length = Some(v.into());
        self
    }

    #[doc = "Set the field `domain_signing_private_key`.\n"]
    pub fn set_domain_signing_private_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.domain_signing_private_key = Some(v.into());
        self
    }

    #[doc = "Set the field `domain_signing_selector`.\n"]
    pub fn set_domain_signing_selector(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.domain_signing_selector = Some(v.into());
        self
    }

    #[doc = "Set the field `last_key_generation_timestamp`.\n"]
    pub fn set_last_key_generation_timestamp(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.last_key_generation_timestamp = Some(v.into());
        self
    }

    #[doc = "Set the field `next_signing_key_length`.\n"]
    pub fn set_next_signing_key_length(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.next_signing_key_length = Some(v.into());
        self
    }

    #[doc = "Set the field `signing_attributes_origin`.\n"]
    pub fn set_signing_attributes_origin(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.signing_attributes_origin = Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status = Some(v.into());
        self
    }

    #[doc = "Set the field `tokens`.\n"]
    pub fn set_tokens(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.tokens = Some(v.into());
        self
    }
}

impl ToListMappable for DataSesv2EmailIdentityDkimSigningAttributesEl {
    type O = BlockAssignable<DataSesv2EmailIdentityDkimSigningAttributesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSesv2EmailIdentityDkimSigningAttributesEl {}

impl BuildDataSesv2EmailIdentityDkimSigningAttributesEl {
    pub fn build(self) -> DataSesv2EmailIdentityDkimSigningAttributesEl {
        DataSesv2EmailIdentityDkimSigningAttributesEl {
            current_signing_key_length: core::default::Default::default(),
            domain_signing_private_key: core::default::Default::default(),
            domain_signing_selector: core::default::Default::default(),
            last_key_generation_timestamp: core::default::Default::default(),
            next_signing_key_length: core::default::Default::default(),
            signing_attributes_origin: core::default::Default::default(),
            status: core::default::Default::default(),
            tokens: core::default::Default::default(),
        }
    }
}

pub struct DataSesv2EmailIdentityDkimSigningAttributesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSesv2EmailIdentityDkimSigningAttributesElRef {
    fn new(shared: StackShared, base: String) -> DataSesv2EmailIdentityDkimSigningAttributesElRef {
        DataSesv2EmailIdentityDkimSigningAttributesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSesv2EmailIdentityDkimSigningAttributesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `current_signing_key_length` after provisioning.\n"]
    pub fn current_signing_key_length(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.current_signing_key_length", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `domain_signing_private_key` after provisioning.\n"]
    pub fn domain_signing_private_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain_signing_private_key", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `domain_signing_selector` after provisioning.\n"]
    pub fn domain_signing_selector(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain_signing_selector", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `last_key_generation_timestamp` after provisioning.\n"]
    pub fn last_key_generation_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_key_generation_timestamp", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `next_signing_key_length` after provisioning.\n"]
    pub fn next_signing_key_length(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.next_signing_key_length", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `signing_attributes_origin` after provisioning.\n"]
    pub fn signing_attributes_origin(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.signing_attributes_origin", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc = "Get a reference to the value of field `tokens` after provisioning.\n"]
    pub fn tokens(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.tokens", self.base))
    }
}
