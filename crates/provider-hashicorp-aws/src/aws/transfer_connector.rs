use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct TransferConnectorData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    access_role: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logging_role: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_policy_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    url: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    as2_config: Option<Vec<TransferConnectorAs2ConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sftp_config: Option<Vec<TransferConnectorSftpConfigEl>>,
    dynamic: TransferConnectorDynamic,
}
struct TransferConnector_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<TransferConnectorData>,
}
#[derive(Clone)]
pub struct TransferConnector(Rc<TransferConnector_>);
impl TransferConnector {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(self, provider: &ProviderAws) -> Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }
    pub fn set_create_before_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.create_before_destroy = v;
        self
    }
    pub fn set_prevent_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.prevent_destroy = v;
        self
    }
    pub fn ignore_changes_to_all(self) -> Self {
        self.0.data.borrow_mut().lifecycle.ignore_changes =
            Some(IgnoreChanges::All(IgnoreChangesAll::All));
        self
    }
    pub fn ignore_changes_to_attr(self, attr: impl ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                Some(i) => match i {
                    IgnoreChanges::All(_) => true,
                    IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    }
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }
    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0
            .data
            .borrow_mut()
            .lifecycle
            .replace_triggered_by
            .push(r.extract_ref());
        self
    }
    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0
            .data
            .borrow_mut()
            .lifecycle
            .replace_triggered_by
            .push(attr.to_string());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `logging_role`.\n"]
    pub fn set_logging_role(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().logging_role = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `security_policy_name`.\n"]
    pub fn set_security_policy_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().security_policy_name = Some(v.into());
        self
    }
    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }
    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags_all = Some(v.into());
        self
    }
    #[doc = "Set the field `as2_config`.\n"]
    pub fn set_as2_config(
        self,
        v: impl Into<BlockAssignable<TransferConnectorAs2ConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().as2_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.as2_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `sftp_config`.\n"]
    pub fn set_sftp_config(
        self,
        v: impl Into<BlockAssignable<TransferConnectorSftpConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().sftp_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.sftp_config = Some(d);
            }
        }
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
    #[doc = "Get a reference to the value of field `connector_id` after provisioning.\n"]
    pub fn connector_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.connector_id", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `as2_config` after provisioning.\n"]
    pub fn as2_config(&self) -> ListRef<TransferConnectorAs2ConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.as2_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `sftp_config` after provisioning.\n"]
    pub fn sftp_config(&self) -> ListRef<TransferConnectorSftpConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.sftp_config", self.extract_ref()),
        )
    }
}
impl Referable for TransferConnector {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for TransferConnector {}
impl ToListMappable for TransferConnector {
    type O = ListRef<TransferConnectorRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for TransferConnector_ {
    fn extract_resource_type(&self) -> String {
        "aws_transfer_connector".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildTransferConnector {
    pub tf_id: String,
    #[doc = ""]
    pub access_role: PrimField<String>,
    #[doc = ""]
    pub url: PrimField<String>,
}
impl BuildTransferConnector {
    pub fn build(self, stack: &mut Stack) -> TransferConnector {
        let out = TransferConnector(Rc::new(TransferConnector_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(TransferConnectorData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                access_role: self.access_role,
                id: core::default::Default::default(),
                logging_role: core::default::Default::default(),
                region: core::default::Default::default(),
                security_policy_name: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                url: self.url,
                as2_config: core::default::Default::default(),
                sftp_config: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct TransferConnectorRef {
    shared: StackShared,
    base: String,
}
impl Ref for TransferConnectorRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl TransferConnectorRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
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
    #[doc = "Get a reference to the value of field `connector_id` after provisioning.\n"]
    pub fn connector_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.connector_id", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `as2_config` after provisioning.\n"]
    pub fn as2_config(&self) -> ListRef<TransferConnectorAs2ConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.as2_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `sftp_config` after provisioning.\n"]
    pub fn sftp_config(&self) -> ListRef<TransferConnectorSftpConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.sftp_config", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct TransferConnectorAs2ConfigEl {
    compression: PrimField<String>,
    encryption_algorithm: PrimField<String>,
    local_profile_id: PrimField<String>,
    mdn_response: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mdn_signing_algorithm: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_subject: Option<PrimField<String>>,
    partner_profile_id: PrimField<String>,
    signing_algorithm: PrimField<String>,
}
impl TransferConnectorAs2ConfigEl {
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
}
impl ToListMappable for TransferConnectorAs2ConfigEl {
    type O = BlockAssignable<TransferConnectorAs2ConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildTransferConnectorAs2ConfigEl {
    #[doc = ""]
    pub compression: PrimField<String>,
    #[doc = ""]
    pub encryption_algorithm: PrimField<String>,
    #[doc = ""]
    pub local_profile_id: PrimField<String>,
    #[doc = ""]
    pub mdn_response: PrimField<String>,
    #[doc = ""]
    pub partner_profile_id: PrimField<String>,
    #[doc = ""]
    pub signing_algorithm: PrimField<String>,
}
impl BuildTransferConnectorAs2ConfigEl {
    pub fn build(self) -> TransferConnectorAs2ConfigEl {
        TransferConnectorAs2ConfigEl {
            compression: self.compression,
            encryption_algorithm: self.encryption_algorithm,
            local_profile_id: self.local_profile_id,
            mdn_response: self.mdn_response,
            mdn_signing_algorithm: core::default::Default::default(),
            message_subject: core::default::Default::default(),
            partner_profile_id: self.partner_profile_id,
            signing_algorithm: self.signing_algorithm,
        }
    }
}
pub struct TransferConnectorAs2ConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for TransferConnectorAs2ConfigElRef {
    fn new(shared: StackShared, base: String) -> TransferConnectorAs2ConfigElRef {
        TransferConnectorAs2ConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl TransferConnectorAs2ConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
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
    #[doc = "Get a reference to the value of field `signing_algorithm` after provisioning.\n"]
    pub fn signing_algorithm(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.signing_algorithm", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct TransferConnectorSftpConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    trusted_host_keys: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_secret_id: Option<PrimField<String>>,
}
impl TransferConnectorSftpConfigEl {
    #[doc = "Set the field `trusted_host_keys`.\n"]
    pub fn set_trusted_host_keys(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.trusted_host_keys = Some(v.into());
        self
    }
    #[doc = "Set the field `user_secret_id`.\n"]
    pub fn set_user_secret_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.user_secret_id = Some(v.into());
        self
    }
}
impl ToListMappable for TransferConnectorSftpConfigEl {
    type O = BlockAssignable<TransferConnectorSftpConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildTransferConnectorSftpConfigEl {}
impl BuildTransferConnectorSftpConfigEl {
    pub fn build(self) -> TransferConnectorSftpConfigEl {
        TransferConnectorSftpConfigEl {
            trusted_host_keys: core::default::Default::default(),
            user_secret_id: core::default::Default::default(),
        }
    }
}
pub struct TransferConnectorSftpConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for TransferConnectorSftpConfigElRef {
    fn new(shared: StackShared, base: String) -> TransferConnectorSftpConfigElRef {
        TransferConnectorSftpConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl TransferConnectorSftpConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `trusted_host_keys` after provisioning.\n"]
    pub fn trusted_host_keys(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
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
#[derive(Serialize, Default)]
struct TransferConnectorDynamic {
    as2_config: Option<DynamicBlock<TransferConnectorAs2ConfigEl>>,
    sftp_config: Option<DynamicBlock<TransferConnectorSftpConfigEl>>,
}
