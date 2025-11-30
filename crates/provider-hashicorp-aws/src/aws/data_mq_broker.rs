use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct DataMqBrokerData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    broker_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    broker_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
}

struct DataMqBroker_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataMqBrokerData>,
}

#[derive(Clone)]
pub struct DataMqBroker(Rc<DataMqBroker_>);

impl DataMqBroker {
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

    #[doc = "Set the field `broker_id`.\n"]
    pub fn set_broker_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().broker_id = Some(v.into());
        self
    }

    #[doc = "Set the field `broker_name`.\n"]
    pub fn set_broker_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().broker_name = Some(v.into());
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

    #[doc = "Get a reference to the value of field `authentication_strategy` after provisioning.\n"]
    pub fn authentication_strategy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.authentication_strategy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `auto_minor_version_upgrade` after provisioning.\n"]
    pub fn auto_minor_version_upgrade(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.auto_minor_version_upgrade", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `broker_id` after provisioning.\n"]
    pub fn broker_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.broker_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `broker_name` after provisioning.\n"]
    pub fn broker_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.broker_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> ListRef<DataMqBrokerConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `deployment_mode` after provisioning.\n"]
    pub fn deployment_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deployment_mode", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `encryption_options` after provisioning.\n"]
    pub fn encryption_options(&self) -> ListRef<DataMqBrokerEncryptionOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.encryption_options", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `engine_type` after provisioning.\n"]
    pub fn engine_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.engine_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `engine_version` after provisioning.\n"]
    pub fn engine_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.engine_version", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `host_instance_type` after provisioning.\n"]
    pub fn host_instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.host_instance_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instances` after provisioning.\n"]
    pub fn instances(&self) -> ListRef<DataMqBrokerInstancesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.instances", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `ldap_server_metadata` after provisioning.\n"]
    pub fn ldap_server_metadata(&self) -> ListRef<DataMqBrokerLdapServerMetadataElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ldap_server_metadata", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `logs` after provisioning.\n"]
    pub fn logs(&self) -> ListRef<DataMqBrokerLogsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.logs", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `maintenance_window_start_time` after provisioning.\n"]
    pub fn maintenance_window_start_time(
        &self,
    ) -> ListRef<DataMqBrokerMaintenanceWindowStartTimeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.maintenance_window_start_time", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `publicly_accessible` after provisioning.\n"]
    pub fn publicly_accessible(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.publicly_accessible", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `security_groups` after provisioning.\n"]
    pub fn security_groups(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.security_groups", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `storage_type` after provisioning.\n"]
    pub fn storage_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.storage_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.subnet_ids", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `user` after provisioning.\n"]
    pub fn user(&self) -> SetRef<DataMqBrokerUserElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.user", self.extract_ref()),
        )
    }
}

impl Referable for DataMqBroker {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Datasource for DataMqBroker {}

impl ToListMappable for DataMqBroker {
    type O = ListRef<DataMqBrokerRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataMqBroker_ {
    fn extract_datasource_type(&self) -> String {
        "aws_mq_broker".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataMqBroker {
    pub tf_id: String,
}

impl BuildDataMqBroker {
    pub fn build(self, stack: &mut Stack) -> DataMqBroker {
        let out = DataMqBroker(Rc::new(DataMqBroker_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataMqBrokerData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                broker_id: core::default::Default::default(),
                broker_name: core::default::Default::default(),
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataMqBrokerRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataMqBrokerRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl DataMqBrokerRef {
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

    #[doc = "Get a reference to the value of field `authentication_strategy` after provisioning.\n"]
    pub fn authentication_strategy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.authentication_strategy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `auto_minor_version_upgrade` after provisioning.\n"]
    pub fn auto_minor_version_upgrade(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.auto_minor_version_upgrade", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `broker_id` after provisioning.\n"]
    pub fn broker_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.broker_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `broker_name` after provisioning.\n"]
    pub fn broker_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.broker_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> ListRef<DataMqBrokerConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `deployment_mode` after provisioning.\n"]
    pub fn deployment_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deployment_mode", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `encryption_options` after provisioning.\n"]
    pub fn encryption_options(&self) -> ListRef<DataMqBrokerEncryptionOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.encryption_options", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `engine_type` after provisioning.\n"]
    pub fn engine_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.engine_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `engine_version` after provisioning.\n"]
    pub fn engine_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.engine_version", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `host_instance_type` after provisioning.\n"]
    pub fn host_instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.host_instance_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instances` after provisioning.\n"]
    pub fn instances(&self) -> ListRef<DataMqBrokerInstancesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.instances", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `ldap_server_metadata` after provisioning.\n"]
    pub fn ldap_server_metadata(&self) -> ListRef<DataMqBrokerLdapServerMetadataElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ldap_server_metadata", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `logs` after provisioning.\n"]
    pub fn logs(&self) -> ListRef<DataMqBrokerLogsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.logs", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `maintenance_window_start_time` after provisioning.\n"]
    pub fn maintenance_window_start_time(
        &self,
    ) -> ListRef<DataMqBrokerMaintenanceWindowStartTimeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.maintenance_window_start_time", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `publicly_accessible` after provisioning.\n"]
    pub fn publicly_accessible(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.publicly_accessible", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `security_groups` after provisioning.\n"]
    pub fn security_groups(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.security_groups", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `storage_type` after provisioning.\n"]
    pub fn storage_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.storage_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.subnet_ids", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `user` after provisioning.\n"]
    pub fn user(&self) -> SetRef<DataMqBrokerUserElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.user", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DataMqBrokerConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    revision: Option<PrimField<f64>>,
}

impl DataMqBrokerConfigurationEl {
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc = "Set the field `revision`.\n"]
    pub fn set_revision(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.revision = Some(v.into());
        self
    }
}

impl ToListMappable for DataMqBrokerConfigurationEl {
    type O = BlockAssignable<DataMqBrokerConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMqBrokerConfigurationEl {}

impl BuildDataMqBrokerConfigurationEl {
    pub fn build(self) -> DataMqBrokerConfigurationEl {
        DataMqBrokerConfigurationEl {
            id: core::default::Default::default(),
            revision: core::default::Default::default(),
        }
    }
}

pub struct DataMqBrokerConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataMqBrokerConfigurationElRef {
    fn new(shared: StackShared, base: String) -> DataMqBrokerConfigurationElRef {
        DataMqBrokerConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMqBrokerConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `revision` after provisioning.\n"]
    pub fn revision(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.revision", self.base))
    }
}

#[derive(Serialize)]
pub struct DataMqBrokerEncryptionOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_aws_owned_key: Option<PrimField<bool>>,
}

impl DataMqBrokerEncryptionOptionsEl {
    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_id = Some(v.into());
        self
    }

    #[doc = "Set the field `use_aws_owned_key`.\n"]
    pub fn set_use_aws_owned_key(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.use_aws_owned_key = Some(v.into());
        self
    }
}

impl ToListMappable for DataMqBrokerEncryptionOptionsEl {
    type O = BlockAssignable<DataMqBrokerEncryptionOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMqBrokerEncryptionOptionsEl {}

impl BuildDataMqBrokerEncryptionOptionsEl {
    pub fn build(self) -> DataMqBrokerEncryptionOptionsEl {
        DataMqBrokerEncryptionOptionsEl {
            kms_key_id: core::default::Default::default(),
            use_aws_owned_key: core::default::Default::default(),
        }
    }
}

pub struct DataMqBrokerEncryptionOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataMqBrokerEncryptionOptionsElRef {
    fn new(shared: StackShared, base: String) -> DataMqBrokerEncryptionOptionsElRef {
        DataMqBrokerEncryptionOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMqBrokerEncryptionOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.base))
    }

    #[doc = "Get a reference to the value of field `use_aws_owned_key` after provisioning.\n"]
    pub fn use_aws_owned_key(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.use_aws_owned_key", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataMqBrokerInstancesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    console_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    endpoints: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_address: Option<PrimField<String>>,
}

impl DataMqBrokerInstancesEl {
    #[doc = "Set the field `console_url`.\n"]
    pub fn set_console_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.console_url = Some(v.into());
        self
    }

    #[doc = "Set the field `endpoints`.\n"]
    pub fn set_endpoints(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.endpoints = Some(v.into());
        self
    }

    #[doc = "Set the field `ip_address`.\n"]
    pub fn set_ip_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ip_address = Some(v.into());
        self
    }
}

impl ToListMappable for DataMqBrokerInstancesEl {
    type O = BlockAssignable<DataMqBrokerInstancesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMqBrokerInstancesEl {}

impl BuildDataMqBrokerInstancesEl {
    pub fn build(self) -> DataMqBrokerInstancesEl {
        DataMqBrokerInstancesEl {
            console_url: core::default::Default::default(),
            endpoints: core::default::Default::default(),
            ip_address: core::default::Default::default(),
        }
    }
}

pub struct DataMqBrokerInstancesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataMqBrokerInstancesElRef {
    fn new(shared: StackShared, base: String) -> DataMqBrokerInstancesElRef {
        DataMqBrokerInstancesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMqBrokerInstancesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `console_url` after provisioning.\n"]
    pub fn console_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.console_url", self.base))
    }

    #[doc = "Get a reference to the value of field `endpoints` after provisioning.\n"]
    pub fn endpoints(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.endpoints", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_address` after provisioning.\n"]
    pub fn ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip_address", self.base))
    }
}

#[derive(Serialize)]
pub struct DataMqBrokerLdapServerMetadataEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    hosts: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    role_base: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    role_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    role_search_matching: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    role_search_subtree: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_account_password: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_account_username: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_base: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_role_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_search_matching: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_search_subtree: Option<PrimField<bool>>,
}

impl DataMqBrokerLdapServerMetadataEl {
    #[doc = "Set the field `hosts`.\n"]
    pub fn set_hosts(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.hosts = Some(v.into());
        self
    }

    #[doc = "Set the field `role_base`.\n"]
    pub fn set_role_base(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.role_base = Some(v.into());
        self
    }

    #[doc = "Set the field `role_name`.\n"]
    pub fn set_role_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.role_name = Some(v.into());
        self
    }

    #[doc = "Set the field `role_search_matching`.\n"]
    pub fn set_role_search_matching(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.role_search_matching = Some(v.into());
        self
    }

    #[doc = "Set the field `role_search_subtree`.\n"]
    pub fn set_role_search_subtree(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.role_search_subtree = Some(v.into());
        self
    }

    #[doc = "Set the field `service_account_password`.\n"]
    pub fn set_service_account_password(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_account_password = Some(v.into());
        self
    }

    #[doc = "Set the field `service_account_username`.\n"]
    pub fn set_service_account_username(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_account_username = Some(v.into());
        self
    }

    #[doc = "Set the field `user_base`.\n"]
    pub fn set_user_base(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.user_base = Some(v.into());
        self
    }

    #[doc = "Set the field `user_role_name`.\n"]
    pub fn set_user_role_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.user_role_name = Some(v.into());
        self
    }

    #[doc = "Set the field `user_search_matching`.\n"]
    pub fn set_user_search_matching(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.user_search_matching = Some(v.into());
        self
    }

    #[doc = "Set the field `user_search_subtree`.\n"]
    pub fn set_user_search_subtree(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.user_search_subtree = Some(v.into());
        self
    }
}

impl ToListMappable for DataMqBrokerLdapServerMetadataEl {
    type O = BlockAssignable<DataMqBrokerLdapServerMetadataEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMqBrokerLdapServerMetadataEl {}

impl BuildDataMqBrokerLdapServerMetadataEl {
    pub fn build(self) -> DataMqBrokerLdapServerMetadataEl {
        DataMqBrokerLdapServerMetadataEl {
            hosts: core::default::Default::default(),
            role_base: core::default::Default::default(),
            role_name: core::default::Default::default(),
            role_search_matching: core::default::Default::default(),
            role_search_subtree: core::default::Default::default(),
            service_account_password: core::default::Default::default(),
            service_account_username: core::default::Default::default(),
            user_base: core::default::Default::default(),
            user_role_name: core::default::Default::default(),
            user_search_matching: core::default::Default::default(),
            user_search_subtree: core::default::Default::default(),
        }
    }
}

pub struct DataMqBrokerLdapServerMetadataElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataMqBrokerLdapServerMetadataElRef {
    fn new(shared: StackShared, base: String) -> DataMqBrokerLdapServerMetadataElRef {
        DataMqBrokerLdapServerMetadataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMqBrokerLdapServerMetadataElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hosts` after provisioning.\n"]
    pub fn hosts(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.hosts", self.base))
    }

    #[doc = "Get a reference to the value of field `role_base` after provisioning.\n"]
    pub fn role_base(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_base", self.base))
    }

    #[doc = "Get a reference to the value of field `role_name` after provisioning.\n"]
    pub fn role_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_name", self.base))
    }

    #[doc = "Get a reference to the value of field `role_search_matching` after provisioning.\n"]
    pub fn role_search_matching(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.role_search_matching", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `role_search_subtree` after provisioning.\n"]
    pub fn role_search_subtree(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.role_search_subtree", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `service_account_password` after provisioning.\n"]
    pub fn service_account_password(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account_password", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `service_account_username` after provisioning.\n"]
    pub fn service_account_username(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account_username", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `user_base` after provisioning.\n"]
    pub fn user_base(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_base", self.base))
    }

    #[doc = "Get a reference to the value of field `user_role_name` after provisioning.\n"]
    pub fn user_role_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_role_name", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `user_search_matching` after provisioning.\n"]
    pub fn user_search_matching(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_search_matching", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `user_search_subtree` after provisioning.\n"]
    pub fn user_search_subtree(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_search_subtree", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataMqBrokerLogsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    audit: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    general: Option<PrimField<bool>>,
}

impl DataMqBrokerLogsEl {
    #[doc = "Set the field `audit`.\n"]
    pub fn set_audit(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.audit = Some(v.into());
        self
    }

    #[doc = "Set the field `general`.\n"]
    pub fn set_general(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.general = Some(v.into());
        self
    }
}

impl ToListMappable for DataMqBrokerLogsEl {
    type O = BlockAssignable<DataMqBrokerLogsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMqBrokerLogsEl {}

impl BuildDataMqBrokerLogsEl {
    pub fn build(self) -> DataMqBrokerLogsEl {
        DataMqBrokerLogsEl {
            audit: core::default::Default::default(),
            general: core::default::Default::default(),
        }
    }
}

pub struct DataMqBrokerLogsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataMqBrokerLogsElRef {
    fn new(shared: StackShared, base: String) -> DataMqBrokerLogsElRef {
        DataMqBrokerLogsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMqBrokerLogsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `audit` after provisioning.\n"]
    pub fn audit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.audit", self.base))
    }

    #[doc = "Get a reference to the value of field `general` after provisioning.\n"]
    pub fn general(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.general", self.base))
    }
}

#[derive(Serialize)]
pub struct DataMqBrokerMaintenanceWindowStartTimeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    day_of_week: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time_of_day: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time_zone: Option<PrimField<String>>,
}

impl DataMqBrokerMaintenanceWindowStartTimeEl {
    #[doc = "Set the field `day_of_week`.\n"]
    pub fn set_day_of_week(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.day_of_week = Some(v.into());
        self
    }

    #[doc = "Set the field `time_of_day`.\n"]
    pub fn set_time_of_day(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.time_of_day = Some(v.into());
        self
    }

    #[doc = "Set the field `time_zone`.\n"]
    pub fn set_time_zone(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.time_zone = Some(v.into());
        self
    }
}

impl ToListMappable for DataMqBrokerMaintenanceWindowStartTimeEl {
    type O = BlockAssignable<DataMqBrokerMaintenanceWindowStartTimeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMqBrokerMaintenanceWindowStartTimeEl {}

impl BuildDataMqBrokerMaintenanceWindowStartTimeEl {
    pub fn build(self) -> DataMqBrokerMaintenanceWindowStartTimeEl {
        DataMqBrokerMaintenanceWindowStartTimeEl {
            day_of_week: core::default::Default::default(),
            time_of_day: core::default::Default::default(),
            time_zone: core::default::Default::default(),
        }
    }
}

pub struct DataMqBrokerMaintenanceWindowStartTimeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataMqBrokerMaintenanceWindowStartTimeElRef {
    fn new(shared: StackShared, base: String) -> DataMqBrokerMaintenanceWindowStartTimeElRef {
        DataMqBrokerMaintenanceWindowStartTimeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMqBrokerMaintenanceWindowStartTimeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `day_of_week` after provisioning.\n"]
    pub fn day_of_week(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.day_of_week", self.base))
    }

    #[doc = "Get a reference to the value of field `time_of_day` after provisioning.\n"]
    pub fn time_of_day(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.time_of_day", self.base))
    }

    #[doc = "Get a reference to the value of field `time_zone` after provisioning.\n"]
    pub fn time_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.time_zone", self.base))
    }
}

#[derive(Serialize)]
pub struct DataMqBrokerUserEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    console_access: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    groups: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    replication_user: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    username: Option<PrimField<String>>,
}

impl DataMqBrokerUserEl {
    #[doc = "Set the field `console_access`.\n"]
    pub fn set_console_access(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.console_access = Some(v.into());
        self
    }

    #[doc = "Set the field `groups`.\n"]
    pub fn set_groups(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.groups = Some(v.into());
        self
    }

    #[doc = "Set the field `replication_user`.\n"]
    pub fn set_replication_user(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.replication_user = Some(v.into());
        self
    }

    #[doc = "Set the field `username`.\n"]
    pub fn set_username(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.username = Some(v.into());
        self
    }
}

impl ToListMappable for DataMqBrokerUserEl {
    type O = BlockAssignable<DataMqBrokerUserEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMqBrokerUserEl {}

impl BuildDataMqBrokerUserEl {
    pub fn build(self) -> DataMqBrokerUserEl {
        DataMqBrokerUserEl {
            console_access: core::default::Default::default(),
            groups: core::default::Default::default(),
            replication_user: core::default::Default::default(),
            username: core::default::Default::default(),
        }
    }
}

pub struct DataMqBrokerUserElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataMqBrokerUserElRef {
    fn new(shared: StackShared, base: String) -> DataMqBrokerUserElRef {
        DataMqBrokerUserElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMqBrokerUserElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `console_access` after provisioning.\n"]
    pub fn console_access(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.console_access", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `groups` after provisioning.\n"]
    pub fn groups(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.groups", self.base))
    }

    #[doc = "Get a reference to the value of field `replication_user` after provisioning.\n"]
    pub fn replication_user(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.replication_user", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `username` after provisioning.\n"]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}
