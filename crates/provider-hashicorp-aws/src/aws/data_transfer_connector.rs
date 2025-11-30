use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct DataTransferConnectorData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}

struct DataTransferConnector_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataTransferConnectorData>,
}

#[derive(Clone)]
pub struct DataTransferConnector(Rc<DataTransferConnector_>);

impl DataTransferConnector {
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

    #[doc = "Get a reference to the value of field `access_role` after provisioning.\n"]
    pub fn access_role(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.access_role", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `as2_config` after provisioning.\n"]
    pub fn as2_config(&self) -> ListRef<DataTransferConnectorAs2ConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.as2_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logging_role` after provisioning.\n"]
    pub fn logging_role(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.logging_role", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `security_policy_name` after provisioning.\n"]
    pub fn security_policy_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.security_policy_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `service_managed_egress_ip_addresses` after provisioning.\n"]
    pub fn service_managed_egress_ip_addresses(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.service_managed_egress_ip_addresses", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `sftp_config` after provisioning.\n"]
    pub fn sftp_config(&self) -> ListRef<DataTransferConnectorSftpConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.sftp_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.extract_ref()))
    }
}

impl Referable for DataTransferConnector {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Datasource for DataTransferConnector {}

impl ToListMappable for DataTransferConnector {
    type O = ListRef<DataTransferConnectorRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataTransferConnector_ {
    fn extract_datasource_type(&self) -> String {
        "aws_transfer_connector".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataTransferConnector {
    pub tf_id: String,
    #[doc = ""]
    pub id: PrimField<String>,
}

impl BuildDataTransferConnector {
    pub fn build(self, stack: &mut Stack) -> DataTransferConnector {
        let out = DataTransferConnector(Rc::new(DataTransferConnector_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataTransferConnectorData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: self.id,
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataTransferConnectorRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataTransferConnectorRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl DataTransferConnectorRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `access_role` after provisioning.\n"]
    pub fn access_role(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.access_role", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `as2_config` after provisioning.\n"]
    pub fn as2_config(&self) -> ListRef<DataTransferConnectorAs2ConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.as2_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logging_role` after provisioning.\n"]
    pub fn logging_role(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.logging_role", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `security_policy_name` after provisioning.\n"]
    pub fn security_policy_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.security_policy_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `service_managed_egress_ip_addresses` after provisioning.\n"]
    pub fn service_managed_egress_ip_addresses(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.service_managed_egress_ip_addresses", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `sftp_config` after provisioning.\n"]
    pub fn sftp_config(&self) -> ListRef<DataTransferConnectorSftpConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.sftp_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataTransferConnectorAs2ConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    basic_auth_secret_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    compression: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_algorithm: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    local_profile_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mdn_response: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mdn_signing_algorithm: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_subject: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    partner_profile_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    singing_algorithm: Option<PrimField<String>>,
}

impl DataTransferConnectorAs2ConfigEl {
    #[doc = "Set the field `basic_auth_secret_id`.\n"]
    pub fn set_basic_auth_secret_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.basic_auth_secret_id = Some(v.into());
        self
    }

    #[doc = "Set the field `compression`.\n"]
    pub fn set_compression(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.compression = Some(v.into());
        self
    }

    #[doc = "Set the field `encryption_algorithm`.\n"]
    pub fn set_encryption_algorithm(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.encryption_algorithm = Some(v.into());
        self
    }

    #[doc = "Set the field `local_profile_id`.\n"]
    pub fn set_local_profile_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.local_profile_id = Some(v.into());
        self
    }

    #[doc = "Set the field `mdn_response`.\n"]
    pub fn set_mdn_response(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mdn_response = Some(v.into());
        self
    }

    #[doc = "Set the field `mdn_signing_algorithm`.\n"]
    pub fn set_mdn_signing_algorithm(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mdn_signing_algorithm = Some(v.into());
        self
    }

    #[doc = "Set the field `message_subject`.\n"]
    pub fn set_message_subject(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message_subject = Some(v.into());
        self
    }

    #[doc = "Set the field `partner_profile_id`.\n"]
    pub fn set_partner_profile_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.partner_profile_id = Some(v.into());
        self
    }

    #[doc = "Set the field `singing_algorithm`.\n"]
    pub fn set_singing_algorithm(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.singing_algorithm = Some(v.into());
        self
    }
}

impl ToListMappable for DataTransferConnectorAs2ConfigEl {
    type O = BlockAssignable<DataTransferConnectorAs2ConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataTransferConnectorAs2ConfigEl {}

impl BuildDataTransferConnectorAs2ConfigEl {
    pub fn build(self) -> DataTransferConnectorAs2ConfigEl {
        DataTransferConnectorAs2ConfigEl {
            basic_auth_secret_id: core::default::Default::default(),
            compression: core::default::Default::default(),
            encryption_algorithm: core::default::Default::default(),
            local_profile_id: core::default::Default::default(),
            mdn_response: core::default::Default::default(),
            mdn_signing_algorithm: core::default::Default::default(),
            message_subject: core::default::Default::default(),
            partner_profile_id: core::default::Default::default(),
            singing_algorithm: core::default::Default::default(),
        }
    }
}

pub struct DataTransferConnectorAs2ConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataTransferConnectorAs2ConfigElRef {
    fn new(shared: StackShared, base: String) -> DataTransferConnectorAs2ConfigElRef {
        DataTransferConnectorAs2ConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataTransferConnectorAs2ConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `basic_auth_secret_id` after provisioning.\n"]
    pub fn basic_auth_secret_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.basic_auth_secret_id", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `compression` after provisioning.\n"]
    pub fn compression(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.compression", self.base))
    }

    #[doc = "Get a reference to the value of field `encryption_algorithm` after provisioning.\n"]
    pub fn encryption_algorithm(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.encryption_algorithm", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `local_profile_id` after provisioning.\n"]
    pub fn local_profile_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.local_profile_id", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `mdn_response` after provisioning.\n"]
    pub fn mdn_response(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mdn_response", self.base))
    }

    #[doc = "Get a reference to the value of field `mdn_signing_algorithm` after provisioning.\n"]
    pub fn mdn_signing_algorithm(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.mdn_signing_algorithm", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `message_subject` after provisioning.\n"]
    pub fn message_subject(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.message_subject", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `partner_profile_id` after provisioning.\n"]
    pub fn partner_profile_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.partner_profile_id", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `singing_algorithm` after provisioning.\n"]
    pub fn singing_algorithm(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.singing_algorithm", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataTransferConnectorSftpConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    trusted_host_keys: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_secret_id: Option<PrimField<String>>,
}

impl DataTransferConnectorSftpConfigEl {
    #[doc = "Set the field `trusted_host_keys`.\n"]
    pub fn set_trusted_host_keys(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.trusted_host_keys = Some(v.into());
        self
    }

    #[doc = "Set the field `user_secret_id`.\n"]
    pub fn set_user_secret_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.user_secret_id = Some(v.into());
        self
    }
}

impl ToListMappable for DataTransferConnectorSftpConfigEl {
    type O = BlockAssignable<DataTransferConnectorSftpConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataTransferConnectorSftpConfigEl {}

impl BuildDataTransferConnectorSftpConfigEl {
    pub fn build(self) -> DataTransferConnectorSftpConfigEl {
        DataTransferConnectorSftpConfigEl {
            trusted_host_keys: core::default::Default::default(),
            user_secret_id: core::default::Default::default(),
        }
    }
}

pub struct DataTransferConnectorSftpConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataTransferConnectorSftpConfigElRef {
    fn new(shared: StackShared, base: String) -> DataTransferConnectorSftpConfigElRef {
        DataTransferConnectorSftpConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataTransferConnectorSftpConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `trusted_host_keys` after provisioning.\n"]
    pub fn trusted_host_keys(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.trusted_host_keys", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `user_secret_id` after provisioning.\n"]
    pub fn user_secret_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_secret_id", self.base),
        )
    }
}
