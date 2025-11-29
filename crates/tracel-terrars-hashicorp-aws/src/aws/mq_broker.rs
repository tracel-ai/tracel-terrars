use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct MqBrokerData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    apply_immediately: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authentication_strategy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_minor_version_upgrade: Option<PrimField<bool>>,
    broker_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_replication_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_replication_primary_broker_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deployment_mode: Option<PrimField<String>>,
    engine_type: PrimField<String>,
    engine_version: PrimField<String>,
    host_instance_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    publicly_accessible: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_groups: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnet_ids: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    configuration: Option<Vec<MqBrokerConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_options: Option<Vec<MqBrokerEncryptionOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ldap_server_metadata: Option<Vec<MqBrokerLdapServerMetadataEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logs: Option<Vec<MqBrokerLogsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maintenance_window_start_time: Option<Vec<MqBrokerMaintenanceWindowStartTimeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<MqBrokerTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user: Option<Vec<MqBrokerUserEl>>,
    dynamic: MqBrokerDynamic,
}

struct MqBroker_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<MqBrokerData>,
}

#[derive(Clone)]
pub struct MqBroker(Rc<MqBroker_>);

impl MqBroker {
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

    #[doc = "Set the field `apply_immediately`.\n"]
    pub fn set_apply_immediately(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().apply_immediately = Some(v.into());
        self
    }

    #[doc = "Set the field `authentication_strategy`.\n"]
    pub fn set_authentication_strategy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().authentication_strategy = Some(v.into());
        self
    }

    #[doc = "Set the field `auto_minor_version_upgrade`.\n"]
    pub fn set_auto_minor_version_upgrade(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().auto_minor_version_upgrade = Some(v.into());
        self
    }

    #[doc = "Set the field `data_replication_mode`.\n"]
    pub fn set_data_replication_mode(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().data_replication_mode = Some(v.into());
        self
    }

    #[doc = "Set the field `data_replication_primary_broker_arn`.\n"]
    pub fn set_data_replication_primary_broker_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().data_replication_primary_broker_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `deployment_mode`.\n"]
    pub fn set_deployment_mode(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deployment_mode = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `publicly_accessible`.\n"]
    pub fn set_publicly_accessible(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().publicly_accessible = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `security_groups`.\n"]
    pub fn set_security_groups(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().security_groups = Some(v.into());
        self
    }

    #[doc = "Set the field `storage_type`.\n"]
    pub fn set_storage_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().storage_type = Some(v.into());
        self
    }

    #[doc = "Set the field `subnet_ids`.\n"]
    pub fn set_subnet_ids(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().subnet_ids = Some(v.into());
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

    #[doc = "Set the field `configuration`.\n"]
    pub fn set_configuration(self, v: impl Into<BlockAssignable<MqBrokerConfigurationEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `encryption_options`.\n"]
    pub fn set_encryption_options(self, v: impl Into<BlockAssignable<MqBrokerEncryptionOptionsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().encryption_options = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.encryption_options = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ldap_server_metadata`.\n"]
    pub fn set_ldap_server_metadata(self, v: impl Into<BlockAssignable<MqBrokerLdapServerMetadataEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ldap_server_metadata = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ldap_server_metadata = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `logs`.\n"]
    pub fn set_logs(self, v: impl Into<BlockAssignable<MqBrokerLogsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().logs = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.logs = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `maintenance_window_start_time`.\n"]
    pub fn set_maintenance_window_start_time(
        self,
        v: impl Into<BlockAssignable<MqBrokerMaintenanceWindowStartTimeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().maintenance_window_start_time = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.maintenance_window_start_time = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<MqBrokerTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Set the field `user`.\n"]
    pub fn set_user(self, v: impl Into<BlockAssignable<MqBrokerUserEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().user = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.user = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `apply_immediately` after provisioning.\n"]
    pub fn apply_immediately(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.apply_immediately", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `authentication_strategy` after provisioning.\n"]
    pub fn authentication_strategy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.authentication_strategy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_minor_version_upgrade` after provisioning.\n"]
    pub fn auto_minor_version_upgrade(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_minor_version_upgrade", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `broker_name` after provisioning.\n"]
    pub fn broker_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.broker_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_replication_mode` after provisioning.\n"]
    pub fn data_replication_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.data_replication_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_replication_primary_broker_arn` after provisioning.\n"]
    pub fn data_replication_primary_broker_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.data_replication_primary_broker_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployment_mode` after provisioning.\n"]
    pub fn deployment_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.deployment_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `engine_type` after provisioning.\n"]
    pub fn engine_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.engine_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `engine_version` after provisioning.\n"]
    pub fn engine_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.engine_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `host_instance_type` after provisioning.\n"]
    pub fn host_instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host_instance_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instances` after provisioning.\n"]
    pub fn instances(&self) -> ListRef<MqBrokerInstancesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.instances", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pending_data_replication_mode` after provisioning.\n"]
    pub fn pending_data_replication_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.pending_data_replication_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `publicly_accessible` after provisioning.\n"]
    pub fn publicly_accessible(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.publicly_accessible", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_groups` after provisioning.\n"]
    pub fn security_groups(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.security_groups", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_type` after provisioning.\n"]
    pub fn storage_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.storage_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> ListRef<MqBrokerConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_options` after provisioning.\n"]
    pub fn encryption_options(&self) -> ListRef<MqBrokerEncryptionOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.encryption_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ldap_server_metadata` after provisioning.\n"]
    pub fn ldap_server_metadata(&self) -> ListRef<MqBrokerLdapServerMetadataElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ldap_server_metadata", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logs` after provisioning.\n"]
    pub fn logs(&self) -> ListRef<MqBrokerLogsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.logs", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_window_start_time` after provisioning.\n"]
    pub fn maintenance_window_start_time(&self) -> ListRef<MqBrokerMaintenanceWindowStartTimeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.maintenance_window_start_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MqBrokerTimeoutsElRef {
        MqBrokerTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for MqBroker {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for MqBroker { }

impl ToListMappable for MqBroker {
    type O = ListRef<MqBrokerRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for MqBroker_ {
    fn extract_resource_type(&self) -> String {
        "aws_mq_broker".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildMqBroker {
    pub tf_id: String,
    #[doc = ""]
    pub broker_name: PrimField<String>,
    #[doc = ""]
    pub engine_type: PrimField<String>,
    #[doc = ""]
    pub engine_version: PrimField<String>,
    #[doc = ""]
    pub host_instance_type: PrimField<String>,
}

impl BuildMqBroker {
    pub fn build(self, stack: &mut Stack) -> MqBroker {
        let out = MqBroker(Rc::new(MqBroker_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(MqBrokerData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                apply_immediately: core::default::Default::default(),
                authentication_strategy: core::default::Default::default(),
                auto_minor_version_upgrade: core::default::Default::default(),
                broker_name: self.broker_name,
                data_replication_mode: core::default::Default::default(),
                data_replication_primary_broker_arn: core::default::Default::default(),
                deployment_mode: core::default::Default::default(),
                engine_type: self.engine_type,
                engine_version: self.engine_version,
                host_instance_type: self.host_instance_type,
                id: core::default::Default::default(),
                publicly_accessible: core::default::Default::default(),
                region: core::default::Default::default(),
                security_groups: core::default::Default::default(),
                storage_type: core::default::Default::default(),
                subnet_ids: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                configuration: core::default::Default::default(),
                encryption_options: core::default::Default::default(),
                ldap_server_metadata: core::default::Default::default(),
                logs: core::default::Default::default(),
                maintenance_window_start_time: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                user: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct MqBrokerRef {
    shared: StackShared,
    base: String,
}

impl Ref for MqBrokerRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl MqBrokerRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `apply_immediately` after provisioning.\n"]
    pub fn apply_immediately(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.apply_immediately", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `authentication_strategy` after provisioning.\n"]
    pub fn authentication_strategy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.authentication_strategy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_minor_version_upgrade` after provisioning.\n"]
    pub fn auto_minor_version_upgrade(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_minor_version_upgrade", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `broker_name` after provisioning.\n"]
    pub fn broker_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.broker_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_replication_mode` after provisioning.\n"]
    pub fn data_replication_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.data_replication_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_replication_primary_broker_arn` after provisioning.\n"]
    pub fn data_replication_primary_broker_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.data_replication_primary_broker_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployment_mode` after provisioning.\n"]
    pub fn deployment_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.deployment_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `engine_type` after provisioning.\n"]
    pub fn engine_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.engine_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `engine_version` after provisioning.\n"]
    pub fn engine_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.engine_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `host_instance_type` after provisioning.\n"]
    pub fn host_instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host_instance_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instances` after provisioning.\n"]
    pub fn instances(&self) -> ListRef<MqBrokerInstancesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.instances", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pending_data_replication_mode` after provisioning.\n"]
    pub fn pending_data_replication_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.pending_data_replication_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `publicly_accessible` after provisioning.\n"]
    pub fn publicly_accessible(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.publicly_accessible", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_groups` after provisioning.\n"]
    pub fn security_groups(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.security_groups", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_type` after provisioning.\n"]
    pub fn storage_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.storage_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> ListRef<MqBrokerConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_options` after provisioning.\n"]
    pub fn encryption_options(&self) -> ListRef<MqBrokerEncryptionOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.encryption_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ldap_server_metadata` after provisioning.\n"]
    pub fn ldap_server_metadata(&self) -> ListRef<MqBrokerLdapServerMetadataElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ldap_server_metadata", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logs` after provisioning.\n"]
    pub fn logs(&self) -> ListRef<MqBrokerLogsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.logs", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_window_start_time` after provisioning.\n"]
    pub fn maintenance_window_start_time(&self) -> ListRef<MqBrokerMaintenanceWindowStartTimeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.maintenance_window_start_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MqBrokerTimeoutsElRef {
        MqBrokerTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct MqBrokerInstancesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    console_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    endpoints: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_address: Option<PrimField<String>>,
}

impl MqBrokerInstancesEl {
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

impl ToListMappable for MqBrokerInstancesEl {
    type O = BlockAssignable<MqBrokerInstancesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMqBrokerInstancesEl {}

impl BuildMqBrokerInstancesEl {
    pub fn build(self) -> MqBrokerInstancesEl {
        MqBrokerInstancesEl {
            console_url: core::default::Default::default(),
            endpoints: core::default::Default::default(),
            ip_address: core::default::Default::default(),
        }
    }
}

pub struct MqBrokerInstancesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for MqBrokerInstancesElRef {
    fn new(shared: StackShared, base: String) -> MqBrokerInstancesElRef {
        MqBrokerInstancesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MqBrokerInstancesElRef {
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
pub struct MqBrokerConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    revision: Option<PrimField<f64>>,
}

impl MqBrokerConfigurationEl {
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

impl ToListMappable for MqBrokerConfigurationEl {
    type O = BlockAssignable<MqBrokerConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMqBrokerConfigurationEl {}

impl BuildMqBrokerConfigurationEl {
    pub fn build(self) -> MqBrokerConfigurationEl {
        MqBrokerConfigurationEl {
            id: core::default::Default::default(),
            revision: core::default::Default::default(),
        }
    }
}

pub struct MqBrokerConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for MqBrokerConfigurationElRef {
    fn new(shared: StackShared, base: String) -> MqBrokerConfigurationElRef {
        MqBrokerConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MqBrokerConfigurationElRef {
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
pub struct MqBrokerEncryptionOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_aws_owned_key: Option<PrimField<bool>>,
}

impl MqBrokerEncryptionOptionsEl {
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

impl ToListMappable for MqBrokerEncryptionOptionsEl {
    type O = BlockAssignable<MqBrokerEncryptionOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMqBrokerEncryptionOptionsEl {}

impl BuildMqBrokerEncryptionOptionsEl {
    pub fn build(self) -> MqBrokerEncryptionOptionsEl {
        MqBrokerEncryptionOptionsEl {
            kms_key_id: core::default::Default::default(),
            use_aws_owned_key: core::default::Default::default(),
        }
    }
}

pub struct MqBrokerEncryptionOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for MqBrokerEncryptionOptionsElRef {
    fn new(shared: StackShared, base: String) -> MqBrokerEncryptionOptionsElRef {
        MqBrokerEncryptionOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MqBrokerEncryptionOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.base))
    }

    #[doc = "Get a reference to the value of field `use_aws_owned_key` after provisioning.\n"]
    pub fn use_aws_owned_key(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.use_aws_owned_key", self.base))
    }
}

#[derive(Serialize)]
pub struct MqBrokerLdapServerMetadataEl {
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

impl MqBrokerLdapServerMetadataEl {
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

impl ToListMappable for MqBrokerLdapServerMetadataEl {
    type O = BlockAssignable<MqBrokerLdapServerMetadataEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMqBrokerLdapServerMetadataEl {}

impl BuildMqBrokerLdapServerMetadataEl {
    pub fn build(self) -> MqBrokerLdapServerMetadataEl {
        MqBrokerLdapServerMetadataEl {
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

pub struct MqBrokerLdapServerMetadataElRef {
    shared: StackShared,
    base: String,
}

impl Ref for MqBrokerLdapServerMetadataElRef {
    fn new(shared: StackShared, base: String) -> MqBrokerLdapServerMetadataElRef {
        MqBrokerLdapServerMetadataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MqBrokerLdapServerMetadataElRef {
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
        PrimExpr::new(self.shared().clone(), format!("{}.role_search_matching", self.base))
    }

    #[doc = "Get a reference to the value of field `role_search_subtree` after provisioning.\n"]
    pub fn role_search_subtree(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_search_subtree", self.base))
    }

    #[doc = "Get a reference to the value of field `service_account_password` after provisioning.\n"]
    pub fn service_account_password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_account_password", self.base))
    }

    #[doc = "Get a reference to the value of field `service_account_username` after provisioning.\n"]
    pub fn service_account_username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_account_username", self.base))
    }

    #[doc = "Get a reference to the value of field `user_base` after provisioning.\n"]
    pub fn user_base(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_base", self.base))
    }

    #[doc = "Get a reference to the value of field `user_role_name` after provisioning.\n"]
    pub fn user_role_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_role_name", self.base))
    }

    #[doc = "Get a reference to the value of field `user_search_matching` after provisioning.\n"]
    pub fn user_search_matching(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_search_matching", self.base))
    }

    #[doc = "Get a reference to the value of field `user_search_subtree` after provisioning.\n"]
    pub fn user_search_subtree(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_search_subtree", self.base))
    }
}

#[derive(Serialize)]
pub struct MqBrokerLogsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    audit: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    general: Option<PrimField<bool>>,
}

impl MqBrokerLogsEl {
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

impl ToListMappable for MqBrokerLogsEl {
    type O = BlockAssignable<MqBrokerLogsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMqBrokerLogsEl {}

impl BuildMqBrokerLogsEl {
    pub fn build(self) -> MqBrokerLogsEl {
        MqBrokerLogsEl {
            audit: core::default::Default::default(),
            general: core::default::Default::default(),
        }
    }
}

pub struct MqBrokerLogsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for MqBrokerLogsElRef {
    fn new(shared: StackShared, base: String) -> MqBrokerLogsElRef {
        MqBrokerLogsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MqBrokerLogsElRef {
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
pub struct MqBrokerMaintenanceWindowStartTimeEl {
    day_of_week: PrimField<String>,
    time_of_day: PrimField<String>,
    time_zone: PrimField<String>,
}

impl MqBrokerMaintenanceWindowStartTimeEl { }

impl ToListMappable for MqBrokerMaintenanceWindowStartTimeEl {
    type O = BlockAssignable<MqBrokerMaintenanceWindowStartTimeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMqBrokerMaintenanceWindowStartTimeEl {
    #[doc = ""]
    pub day_of_week: PrimField<String>,
    #[doc = ""]
    pub time_of_day: PrimField<String>,
    #[doc = ""]
    pub time_zone: PrimField<String>,
}

impl BuildMqBrokerMaintenanceWindowStartTimeEl {
    pub fn build(self) -> MqBrokerMaintenanceWindowStartTimeEl {
        MqBrokerMaintenanceWindowStartTimeEl {
            day_of_week: self.day_of_week,
            time_of_day: self.time_of_day,
            time_zone: self.time_zone,
        }
    }
}

pub struct MqBrokerMaintenanceWindowStartTimeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for MqBrokerMaintenanceWindowStartTimeElRef {
    fn new(shared: StackShared, base: String) -> MqBrokerMaintenanceWindowStartTimeElRef {
        MqBrokerMaintenanceWindowStartTimeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MqBrokerMaintenanceWindowStartTimeElRef {
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
pub struct MqBrokerTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl MqBrokerTimeoutsEl {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }

    #[doc = "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }

    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}

impl ToListMappable for MqBrokerTimeoutsEl {
    type O = BlockAssignable<MqBrokerTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMqBrokerTimeoutsEl {}

impl BuildMqBrokerTimeoutsEl {
    pub fn build(self) -> MqBrokerTimeoutsEl {
        MqBrokerTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct MqBrokerTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for MqBrokerTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> MqBrokerTimeoutsElRef {
        MqBrokerTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MqBrokerTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc = "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize)]
pub struct MqBrokerUserEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    console_access: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    groups: Option<SetField<PrimField<String>>>,
    password: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    replication_user: Option<PrimField<bool>>,
    username: PrimField<String>,
}

impl MqBrokerUserEl {
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
}

impl ToListMappable for MqBrokerUserEl {
    type O = BlockAssignable<MqBrokerUserEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMqBrokerUserEl {
    #[doc = ""]
    pub password: PrimField<String>,
    #[doc = ""]
    pub username: PrimField<String>,
}

impl BuildMqBrokerUserEl {
    pub fn build(self) -> MqBrokerUserEl {
        MqBrokerUserEl {
            console_access: core::default::Default::default(),
            groups: core::default::Default::default(),
            password: self.password,
            replication_user: core::default::Default::default(),
            username: self.username,
        }
    }
}

pub struct MqBrokerUserElRef {
    shared: StackShared,
    base: String,
}

impl Ref for MqBrokerUserElRef {
    fn new(shared: StackShared, base: String) -> MqBrokerUserElRef {
        MqBrokerUserElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MqBrokerUserElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `console_access` after provisioning.\n"]
    pub fn console_access(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.console_access", self.base))
    }

    #[doc = "Get a reference to the value of field `groups` after provisioning.\n"]
    pub fn groups(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.groups", self.base))
    }

    #[doc = "Get a reference to the value of field `password` after provisioning.\n"]
    pub fn password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }

    #[doc = "Get a reference to the value of field `replication_user` after provisioning.\n"]
    pub fn replication_user(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.replication_user", self.base))
    }

    #[doc = "Get a reference to the value of field `username` after provisioning.\n"]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}

#[derive(Serialize, Default)]
struct MqBrokerDynamic {
    configuration: Option<DynamicBlock<MqBrokerConfigurationEl>>,
    encryption_options: Option<DynamicBlock<MqBrokerEncryptionOptionsEl>>,
    ldap_server_metadata: Option<DynamicBlock<MqBrokerLdapServerMetadataEl>>,
    logs: Option<DynamicBlock<MqBrokerLogsEl>>,
    maintenance_window_start_time: Option<DynamicBlock<MqBrokerMaintenanceWindowStartTimeEl>>,
    user: Option<DynamicBlock<MqBrokerUserEl>>,
}
