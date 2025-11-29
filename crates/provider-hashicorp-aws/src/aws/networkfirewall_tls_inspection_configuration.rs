use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct NetworkfirewallTlsInspectionConfigurationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_configuration: Option<ListField<NetworkfirewallTlsInspectionConfigurationEncryptionConfigurationEl>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<NetworkfirewallTlsInspectionConfigurationTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tls_inspection_configuration: Option<Vec<NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationEl>>,
    dynamic: NetworkfirewallTlsInspectionConfigurationDynamic,
}

struct NetworkfirewallTlsInspectionConfiguration_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<NetworkfirewallTlsInspectionConfigurationData>,
}

#[derive(Clone)]
pub struct NetworkfirewallTlsInspectionConfiguration(Rc<NetworkfirewallTlsInspectionConfiguration_>);

impl NetworkfirewallTlsInspectionConfiguration {
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
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
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc = "Set the field `encryption_configuration`.\n"]
    pub fn set_encryption_configuration(
        self,
        v: impl Into<ListField<NetworkfirewallTlsInspectionConfigurationEncryptionConfigurationEl>>,
    ) -> Self {
        self.0.data.borrow_mut().encryption_configuration = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<NetworkfirewallTlsInspectionConfigurationTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Set the field `tls_inspection_configuration`.\n"]
    pub fn set_tls_inspection_configuration(
        self,
        v: impl Into<BlockAssignable<NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().tls_inspection_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.tls_inspection_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificate_authority` after provisioning.\n"]
    pub fn certificate_authority(&self) -> ListRef<NetworkfirewallTlsInspectionConfigurationCertificateAuthorityElRef> {
        ListRef::new(self.shared().clone(), format!("{}.certificate_authority", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificates` after provisioning.\n"]
    pub fn certificates(&self) -> ListRef<NetworkfirewallTlsInspectionConfigurationCertificatesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.certificates", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_configuration` after provisioning.\n"]
    pub fn encryption_configuration(
        &self,
    ) -> ListRef<NetworkfirewallTlsInspectionConfigurationEncryptionConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.encryption_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `number_of_associations` after provisioning.\n"]
    pub fn number_of_associations(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.number_of_associations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tls_inspection_configuration_id` after provisioning.\n"]
    pub fn tls_inspection_configuration_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tls_inspection_configuration_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `update_token` after provisioning.\n"]
    pub fn update_token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update_token", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkfirewallTlsInspectionConfigurationTimeoutsElRef {
        NetworkfirewallTlsInspectionConfigurationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tls_inspection_configuration` after provisioning.\n"]
    pub fn tls_inspection_configuration(
        &self,
    ) -> ListRef<NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tls_inspection_configuration", self.extract_ref()))
    }
}

impl Referable for NetworkfirewallTlsInspectionConfiguration {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for NetworkfirewallTlsInspectionConfiguration { }

impl ToListMappable for NetworkfirewallTlsInspectionConfiguration {
    type O = ListRef<NetworkfirewallTlsInspectionConfigurationRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for NetworkfirewallTlsInspectionConfiguration_ {
    fn extract_resource_type(&self) -> String {
        "aws_networkfirewall_tls_inspection_configuration".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildNetworkfirewallTlsInspectionConfiguration {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildNetworkfirewallTlsInspectionConfiguration {
    pub fn build(self, stack: &mut Stack) -> NetworkfirewallTlsInspectionConfiguration {
        let out = NetworkfirewallTlsInspectionConfiguration(Rc::new(NetworkfirewallTlsInspectionConfiguration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(NetworkfirewallTlsInspectionConfigurationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                description: core::default::Default::default(),
                encryption_configuration: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                tls_inspection_configuration: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct NetworkfirewallTlsInspectionConfigurationRef {
    shared: StackShared,
    base: String,
}

impl Ref for NetworkfirewallTlsInspectionConfigurationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl NetworkfirewallTlsInspectionConfigurationRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificate_authority` after provisioning.\n"]
    pub fn certificate_authority(&self) -> ListRef<NetworkfirewallTlsInspectionConfigurationCertificateAuthorityElRef> {
        ListRef::new(self.shared().clone(), format!("{}.certificate_authority", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificates` after provisioning.\n"]
    pub fn certificates(&self) -> ListRef<NetworkfirewallTlsInspectionConfigurationCertificatesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.certificates", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_configuration` after provisioning.\n"]
    pub fn encryption_configuration(
        &self,
    ) -> ListRef<NetworkfirewallTlsInspectionConfigurationEncryptionConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.encryption_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `number_of_associations` after provisioning.\n"]
    pub fn number_of_associations(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.number_of_associations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tls_inspection_configuration_id` after provisioning.\n"]
    pub fn tls_inspection_configuration_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tls_inspection_configuration_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `update_token` after provisioning.\n"]
    pub fn update_token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update_token", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkfirewallTlsInspectionConfigurationTimeoutsElRef {
        NetworkfirewallTlsInspectionConfigurationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tls_inspection_configuration` after provisioning.\n"]
    pub fn tls_inspection_configuration(
        &self,
    ) -> ListRef<NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tls_inspection_configuration", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct NetworkfirewallTlsInspectionConfigurationCertificateAuthorityEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate_serial: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status_message: Option<PrimField<String>>,
}

impl NetworkfirewallTlsInspectionConfigurationCertificateAuthorityEl {
    #[doc = "Set the field `certificate_arn`.\n"]
    pub fn set_certificate_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.certificate_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `certificate_serial`.\n"]
    pub fn set_certificate_serial(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.certificate_serial = Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status = Some(v.into());
        self
    }

    #[doc = "Set the field `status_message`.\n"]
    pub fn set_status_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status_message = Some(v.into());
        self
    }
}

impl ToListMappable for NetworkfirewallTlsInspectionConfigurationCertificateAuthorityEl {
    type O = BlockAssignable<NetworkfirewallTlsInspectionConfigurationCertificateAuthorityEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallTlsInspectionConfigurationCertificateAuthorityEl {}

impl BuildNetworkfirewallTlsInspectionConfigurationCertificateAuthorityEl {
    pub fn build(self) -> NetworkfirewallTlsInspectionConfigurationCertificateAuthorityEl {
        NetworkfirewallTlsInspectionConfigurationCertificateAuthorityEl {
            certificate_arn: core::default::Default::default(),
            certificate_serial: core::default::Default::default(),
            status: core::default::Default::default(),
            status_message: core::default::Default::default(),
        }
    }
}

pub struct NetworkfirewallTlsInspectionConfigurationCertificateAuthorityElRef {
    shared: StackShared,
    base: String,
}

impl Ref for NetworkfirewallTlsInspectionConfigurationCertificateAuthorityElRef {
    fn new(shared: StackShared, base: String) -> NetworkfirewallTlsInspectionConfigurationCertificateAuthorityElRef {
        NetworkfirewallTlsInspectionConfigurationCertificateAuthorityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallTlsInspectionConfigurationCertificateAuthorityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificate_arn` after provisioning.\n"]
    pub fn certificate_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.certificate_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `certificate_serial` after provisioning.\n"]
    pub fn certificate_serial(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.certificate_serial", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc = "Get a reference to the value of field `status_message` after provisioning.\n"]
    pub fn status_message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status_message", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkfirewallTlsInspectionConfigurationCertificatesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate_serial: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status_message: Option<PrimField<String>>,
}

impl NetworkfirewallTlsInspectionConfigurationCertificatesEl {
    #[doc = "Set the field `certificate_arn`.\n"]
    pub fn set_certificate_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.certificate_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `certificate_serial`.\n"]
    pub fn set_certificate_serial(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.certificate_serial = Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status = Some(v.into());
        self
    }

    #[doc = "Set the field `status_message`.\n"]
    pub fn set_status_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status_message = Some(v.into());
        self
    }
}

impl ToListMappable for NetworkfirewallTlsInspectionConfigurationCertificatesEl {
    type O = BlockAssignable<NetworkfirewallTlsInspectionConfigurationCertificatesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallTlsInspectionConfigurationCertificatesEl {}

impl BuildNetworkfirewallTlsInspectionConfigurationCertificatesEl {
    pub fn build(self) -> NetworkfirewallTlsInspectionConfigurationCertificatesEl {
        NetworkfirewallTlsInspectionConfigurationCertificatesEl {
            certificate_arn: core::default::Default::default(),
            certificate_serial: core::default::Default::default(),
            status: core::default::Default::default(),
            status_message: core::default::Default::default(),
        }
    }
}

pub struct NetworkfirewallTlsInspectionConfigurationCertificatesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for NetworkfirewallTlsInspectionConfigurationCertificatesElRef {
    fn new(shared: StackShared, base: String) -> NetworkfirewallTlsInspectionConfigurationCertificatesElRef {
        NetworkfirewallTlsInspectionConfigurationCertificatesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallTlsInspectionConfigurationCertificatesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificate_arn` after provisioning.\n"]
    pub fn certificate_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.certificate_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `certificate_serial` after provisioning.\n"]
    pub fn certificate_serial(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.certificate_serial", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc = "Get a reference to the value of field `status_message` after provisioning.\n"]
    pub fn status_message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status_message", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkfirewallTlsInspectionConfigurationEncryptionConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key_id: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}

impl NetworkfirewallTlsInspectionConfigurationEncryptionConfigurationEl {
    #[doc = "Set the field `key_id`.\n"]
    pub fn set_key_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key_id = Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}

impl ToListMappable for NetworkfirewallTlsInspectionConfigurationEncryptionConfigurationEl {
    type O = BlockAssignable<NetworkfirewallTlsInspectionConfigurationEncryptionConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallTlsInspectionConfigurationEncryptionConfigurationEl {}

impl BuildNetworkfirewallTlsInspectionConfigurationEncryptionConfigurationEl {
    pub fn build(self) -> NetworkfirewallTlsInspectionConfigurationEncryptionConfigurationEl {
        NetworkfirewallTlsInspectionConfigurationEncryptionConfigurationEl {
            key_id: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct NetworkfirewallTlsInspectionConfigurationEncryptionConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for NetworkfirewallTlsInspectionConfigurationEncryptionConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkfirewallTlsInspectionConfigurationEncryptionConfigurationElRef {
        NetworkfirewallTlsInspectionConfigurationEncryptionConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallTlsInspectionConfigurationEncryptionConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key_id` after provisioning.\n"]
    pub fn key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_id", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkfirewallTlsInspectionConfigurationTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl NetworkfirewallTlsInspectionConfigurationTimeoutsEl {
    #[doc =
        "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }

    #[doc =
        "Set the field `delete`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }

    #[doc =
        "Set the field `update`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}

impl ToListMappable for NetworkfirewallTlsInspectionConfigurationTimeoutsEl {
    type O = BlockAssignable<NetworkfirewallTlsInspectionConfigurationTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallTlsInspectionConfigurationTimeoutsEl {}

impl BuildNetworkfirewallTlsInspectionConfigurationTimeoutsEl {
    pub fn build(self) -> NetworkfirewallTlsInspectionConfigurationTimeoutsEl {
        NetworkfirewallTlsInspectionConfigurationTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct NetworkfirewallTlsInspectionConfigurationTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for NetworkfirewallTlsInspectionConfigurationTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> NetworkfirewallTlsInspectionConfigurationTimeoutsElRef {
        NetworkfirewallTlsInspectionConfigurationTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallTlsInspectionConfigurationTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc =
        "Get a reference to the value of field `delete` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc =
        "Get a reference to the value of field `update` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElCheckCertificateRevocationStatusEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    revoked_status_action: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unknown_status_action: Option<PrimField<String>>,
}

impl NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElCheckCertificateRevocationStatusEl {
    #[doc = "Set the field `revoked_status_action`.\n"]
    pub fn set_revoked_status_action(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.revoked_status_action = Some(v.into());
        self
    }

    #[doc = "Set the field `unknown_status_action`.\n"]
    pub fn set_unknown_status_action(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.unknown_status_action = Some(v.into());
        self
    }
}

impl ToListMappable for NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElCheckCertificateRevocationStatusEl {
    type O =
        BlockAssignable<
            NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElCheckCertificateRevocationStatusEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElCheckCertificateRevocationStatusEl {}

impl BuildNetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElCheckCertificateRevocationStatusEl {
    pub fn build(
        self,
    ) -> NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElCheckCertificateRevocationStatusEl {
        NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElCheckCertificateRevocationStatusEl {
            revoked_status_action: core::default::Default::default(),
            unknown_status_action: core::default::Default::default(),
        }
    }
}

pub struct NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElCheckCertificateRevocationStatusElRef {
    shared: StackShared,
    base: String,
}

impl Ref for NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElCheckCertificateRevocationStatusElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElCheckCertificateRevocationStatusElRef {
        NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElCheckCertificateRevocationStatusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElCheckCertificateRevocationStatusElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `revoked_status_action` after provisioning.\n"]
    pub fn revoked_status_action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.revoked_status_action", self.base))
    }

    #[doc = "Get a reference to the value of field `unknown_status_action` after provisioning.\n"]
    pub fn unknown_status_action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.unknown_status_action", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElScopeElDestinationEl {
    address_definition: PrimField<String>,
}

impl NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElScopeElDestinationEl {

}

impl ToListMappable for NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElScopeElDestinationEl {
    type O =
        BlockAssignable<
            NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElScopeElDestinationEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElScopeElDestinationEl {
    #[doc = ""]
    pub address_definition: PrimField<String>,
}

impl BuildNetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElScopeElDestinationEl {
    pub fn build(
        self,
    ) -> NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElScopeElDestinationEl {
        NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElScopeElDestinationEl {
            address_definition: self.address_definition,
        }
    }
}

pub struct NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElScopeElDestinationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElScopeElDestinationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElScopeElDestinationElRef {
        NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElScopeElDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElScopeElDestinationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `address_definition` after provisioning.\n"]
    pub fn address_definition(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.address_definition", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElScopeElDestinationPortsEl {
    from_port: PrimField<f64>,
    to_port: PrimField<f64>,
}

impl NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElScopeElDestinationPortsEl {

}

impl ToListMappable for NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElScopeElDestinationPortsEl {
    type O =
        BlockAssignable<
            NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElScopeElDestinationPortsEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElScopeElDestinationPortsEl {
    #[doc = ""]
    pub from_port: PrimField<f64>,
    #[doc = ""]
    pub to_port: PrimField<f64>,
}

impl BuildNetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElScopeElDestinationPortsEl {
    pub fn build(
        self,
    ) -> NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElScopeElDestinationPortsEl {
        NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElScopeElDestinationPortsEl {
            from_port: self.from_port,
            to_port: self.to_port,
        }
    }
}

pub struct NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElScopeElDestinationPortsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElScopeElDestinationPortsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElScopeElDestinationPortsElRef {
        NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElScopeElDestinationPortsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElScopeElDestinationPortsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `from_port` after provisioning.\n"]
    pub fn from_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.from_port", self.base))
    }

    #[doc = "Get a reference to the value of field `to_port` after provisioning.\n"]
    pub fn to_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.to_port", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElScopeElSourceEl {
    address_definition: PrimField<String>,
}

impl NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElScopeElSourceEl {

}

impl ToListMappable for NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElScopeElSourceEl {
    type O =
        BlockAssignable<
            NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElScopeElSourceEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElScopeElSourceEl {
    #[doc = ""]
    pub address_definition: PrimField<String>,
}

impl BuildNetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElScopeElSourceEl {
    pub fn build(
        self,
    ) -> NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElScopeElSourceEl {
        NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElScopeElSourceEl {
            address_definition: self.address_definition,
        }
    }
}

pub struct NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElScopeElSourceElRef {
    shared: StackShared,
    base: String,
}

impl Ref for NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElScopeElSourceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElScopeElSourceElRef {
        NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElScopeElSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElScopeElSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `address_definition` after provisioning.\n"]
    pub fn address_definition(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.address_definition", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElScopeElSourcePortsEl {
    from_port: PrimField<f64>,
    to_port: PrimField<f64>,
}

impl NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElScopeElSourcePortsEl {

}

impl ToListMappable for NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElScopeElSourcePortsEl {
    type O =
        BlockAssignable<
            NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElScopeElSourcePortsEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElScopeElSourcePortsEl {
    #[doc = ""]
    pub from_port: PrimField<f64>,
    #[doc = ""]
    pub to_port: PrimField<f64>,
}

impl BuildNetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElScopeElSourcePortsEl {
    pub fn build(
        self,
    ) -> NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElScopeElSourcePortsEl {
        NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElScopeElSourcePortsEl {
            from_port: self.from_port,
            to_port: self.to_port,
        }
    }
}

pub struct NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElScopeElSourcePortsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElScopeElSourcePortsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElScopeElSourcePortsElRef {
        NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElScopeElSourcePortsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElScopeElSourcePortsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `from_port` after provisioning.\n"]
    pub fn from_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.from_port", self.base))
    }

    #[doc = "Get a reference to the value of field `to_port` after provisioning.\n"]
    pub fn to_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.to_port", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElScopeElDynamic {
    destination: Option<
        DynamicBlock<
            NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElScopeElDestinationEl,
        >,
    >,
    destination_ports: Option<
        DynamicBlock<
            NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElScopeElDestinationPortsEl,
        >,
    >,
    source: Option<
        DynamicBlock<
            NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElScopeElSourceEl,
        >,
    >,
    source_ports: Option<
        DynamicBlock<
            NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElScopeElSourcePortsEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElScopeEl {
    protocols: SetField<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination: Option<
        Vec<
            NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElScopeElDestinationEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_ports: Option<
        Vec<
            NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElScopeElDestinationPortsEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<
        Vec<
            NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElScopeElSourceEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_ports: Option<
        Vec<
            NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElScopeElSourcePortsEl,
        >,
    >,
    dynamic: NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElScopeElDynamic,
}

impl NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElScopeEl {
    #[doc = "Set the field `destination`.\n"]
    pub fn set_destination(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElScopeElDestinationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.destination = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.destination = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `destination_ports`.\n"]
    pub fn set_destination_ports(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElScopeElDestinationPortsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.destination_ports = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.destination_ports = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `source`.\n"]
    pub fn set_source(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElScopeElSourceEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.source = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.source = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `source_ports`.\n"]
    pub fn set_source_ports(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElScopeElSourcePortsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.source_ports = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.source_ports = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElScopeEl {
    type O =
        BlockAssignable<
            NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElScopeEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElScopeEl {
    #[doc = ""]
    pub protocols: SetField<PrimField<f64>>,
}

impl BuildNetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElScopeEl {
    pub fn build(
        self,
    ) -> NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElScopeEl {
        NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElScopeEl {
            protocols: self.protocols,
            destination: core::default::Default::default(),
            destination_ports: core::default::Default::default(),
            source: core::default::Default::default(),
            source_ports: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElScopeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElScopeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElScopeElRef {
        NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElScopeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElScopeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `protocols` after provisioning.\n"]
    pub fn protocols(&self) -> SetRef<PrimExpr<f64>> {
        SetRef::new(self.shared().clone(), format!("{}.protocols", self.base))
    }

    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(
        &self,
    ) -> ListRef<
        NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElScopeElDestinationElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.destination", self.base))
    }

    #[doc = "Get a reference to the value of field `destination_ports` after provisioning.\n"]
    pub fn destination_ports(
        &self,
    ) -> ListRef<
        NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElScopeElDestinationPortsElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.destination_ports", self.base))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(
        &self,
    ) -> ListRef<
        NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElScopeElSourceElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.source", self.base))
    }

    #[doc = "Get a reference to the value of field `source_ports` after provisioning.\n"]
    pub fn source_ports(
        &self,
    ) -> ListRef<
        NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElScopeElSourcePortsElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.source_ports", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElServerCertificateEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_arn: Option<PrimField<String>>,
}

impl NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElServerCertificateEl {
    #[doc = "Set the field `resource_arn`.\n"]
    pub fn set_resource_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.resource_arn = Some(v.into());
        self
    }
}

impl ToListMappable for NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElServerCertificateEl {
    type O =
        BlockAssignable<
            NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElServerCertificateEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElServerCertificateEl {}

impl BuildNetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElServerCertificateEl {
    pub fn build(
        self,
    ) -> NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElServerCertificateEl {
        NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElServerCertificateEl {
            resource_arn: core::default::Default::default(),
        }
    }
}

pub struct NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElServerCertificateElRef {
    shared: StackShared,
    base: String,
}

impl Ref for NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElServerCertificateElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElServerCertificateElRef {
        NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElServerCertificateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElServerCertificateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `resource_arn` after provisioning.\n"]
    pub fn resource_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElDynamic {
    check_certificate_revocation_status: Option<
        DynamicBlock<
            NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElCheckCertificateRevocationStatusEl,
        >,
    >,
    scope: Option<
        DynamicBlock<
            NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElScopeEl,
        >,
    >,
    server_certificate: Option<
        DynamicBlock<
            NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElServerCertificateEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate_authority_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    check_certificate_revocation_status: Option<
        Vec<
            NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElCheckCertificateRevocationStatusEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<
        Vec<
            NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElScopeEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    server_certificate: Option<
        Vec<
            NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElServerCertificateEl,
        >,
    >,
    dynamic: NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElDynamic,
}

impl NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationEl {
    #[doc = "Set the field `certificate_authority_arn`.\n"]
    pub fn set_certificate_authority_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.certificate_authority_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `check_certificate_revocation_status`.\n"]
    pub fn set_check_certificate_revocation_status(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElCheckCertificateRevocationStatusEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.check_certificate_revocation_status = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.check_certificate_revocation_status = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `scope`.\n"]
    pub fn set_scope(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElScopeEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.scope = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.scope = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `server_certificate`.\n"]
    pub fn set_server_certificate(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElServerCertificateEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.server_certificate = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.server_certificate = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationEl {
    type O =
        BlockAssignable<
            NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationEl {}

impl BuildNetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationEl {
    pub fn build(
        self,
    ) -> NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationEl {
        NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationEl {
            certificate_authority_arn: core::default::Default::default(),
            check_certificate_revocation_status: core::default::Default::default(),
            scope: core::default::Default::default(),
            server_certificate: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElRef {
        NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificate_authority_arn` after provisioning.\n"]
    pub fn certificate_authority_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.certificate_authority_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `check_certificate_revocation_status` after provisioning.\n"]
    pub fn check_certificate_revocation_status(
        &self,
    ) -> ListRef<
        NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElCheckCertificateRevocationStatusElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.check_certificate_revocation_status", self.base))
    }

    #[doc = "Get a reference to the value of field `scope` after provisioning.\n"]
    pub fn scope(
        &self,
    ) -> ListRef<
        NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElScopeElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.scope", self.base))
    }

    #[doc = "Get a reference to the value of field `server_certificate` after provisioning.\n"]
    pub fn server_certificate(
        &self,
    ) -> ListRef<
        NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElServerCertificateElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.server_certificate", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElDynamic {
    server_certificate_configuration: Option<
        DynamicBlock<
            NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    server_certificate_configuration: Option<
        Vec<NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationEl>,
    >,
    dynamic: NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElDynamic,
}

impl NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationEl {
    #[doc = "Set the field `server_certificate_configuration`.\n"]
    pub fn set_server_certificate_configuration(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.server_certificate_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.server_certificate_configuration = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationEl {
    type O = BlockAssignable<NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationEl {}

impl BuildNetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationEl {
    pub fn build(self) -> NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationEl {
        NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationEl {
            server_certificate_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElRef {
        NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `server_certificate_configuration` after provisioning.\n"]
    pub fn server_certificate_configuration(
        &self,
    ) -> ListRef<
        NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationElServerCertificateConfigurationElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.server_certificate_configuration", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkfirewallTlsInspectionConfigurationDynamic {
    tls_inspection_configuration: Option<
        DynamicBlock<NetworkfirewallTlsInspectionConfigurationTlsInspectionConfigurationEl>,
    >,
}
