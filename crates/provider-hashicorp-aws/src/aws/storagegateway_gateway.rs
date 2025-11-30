use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct StoragegatewayGatewayData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    activation_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    average_download_rate_limit_in_bits_per_sec: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    average_upload_rate_limit_in_bits_per_sec: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloudwatch_log_group_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gateway_ip_address: Option<PrimField<String>>,
    gateway_name: PrimField<String>,
    gateway_timezone: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gateway_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gateway_vpc_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    medium_changer_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    smb_file_share_visibility: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    smb_guest_password: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    smb_security_strategy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tape_drive_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maintenance_start_time: Option<Vec<StoragegatewayGatewayMaintenanceStartTimeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    smb_active_directory_settings: Option<Vec<StoragegatewayGatewaySmbActiveDirectorySettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<StoragegatewayGatewayTimeoutsEl>,
    dynamic: StoragegatewayGatewayDynamic,
}

struct StoragegatewayGateway_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<StoragegatewayGatewayData>,
}

#[derive(Clone)]
pub struct StoragegatewayGateway(Rc<StoragegatewayGateway_>);

impl StoragegatewayGateway {
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

    #[doc = "Set the field `activation_key`.\n"]
    pub fn set_activation_key(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().activation_key = Some(v.into());
        self
    }

    #[doc = "Set the field `average_download_rate_limit_in_bits_per_sec`.\n"]
    pub fn set_average_download_rate_limit_in_bits_per_sec(
        self,
        v: impl Into<PrimField<f64>>,
    ) -> Self {
        self.0
            .data
            .borrow_mut()
            .average_download_rate_limit_in_bits_per_sec = Some(v.into());
        self
    }

    #[doc = "Set the field `average_upload_rate_limit_in_bits_per_sec`.\n"]
    pub fn set_average_upload_rate_limit_in_bits_per_sec(
        self,
        v: impl Into<PrimField<f64>>,
    ) -> Self {
        self.0
            .data
            .borrow_mut()
            .average_upload_rate_limit_in_bits_per_sec = Some(v.into());
        self
    }

    #[doc = "Set the field `cloudwatch_log_group_arn`.\n"]
    pub fn set_cloudwatch_log_group_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().cloudwatch_log_group_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `gateway_ip_address`.\n"]
    pub fn set_gateway_ip_address(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().gateway_ip_address = Some(v.into());
        self
    }

    #[doc = "Set the field `gateway_type`.\n"]
    pub fn set_gateway_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().gateway_type = Some(v.into());
        self
    }

    #[doc = "Set the field `gateway_vpc_endpoint`.\n"]
    pub fn set_gateway_vpc_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().gateway_vpc_endpoint = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `medium_changer_type`.\n"]
    pub fn set_medium_changer_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().medium_changer_type = Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `smb_file_share_visibility`.\n"]
    pub fn set_smb_file_share_visibility(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().smb_file_share_visibility = Some(v.into());
        self
    }

    #[doc = "Set the field `smb_guest_password`.\n"]
    pub fn set_smb_guest_password(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().smb_guest_password = Some(v.into());
        self
    }

    #[doc = "Set the field `smb_security_strategy`.\n"]
    pub fn set_smb_security_strategy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().smb_security_strategy = Some(v.into());
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

    #[doc = "Set the field `tape_drive_type`.\n"]
    pub fn set_tape_drive_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().tape_drive_type = Some(v.into());
        self
    }

    #[doc = "Set the field `maintenance_start_time`.\n"]
    pub fn set_maintenance_start_time(
        self,
        v: impl Into<BlockAssignable<StoragegatewayGatewayMaintenanceStartTimeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().maintenance_start_time = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.maintenance_start_time = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `smb_active_directory_settings`.\n"]
    pub fn set_smb_active_directory_settings(
        self,
        v: impl Into<BlockAssignable<StoragegatewayGatewaySmbActiveDirectorySettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().smb_active_directory_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .smb_active_directory_settings = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<StoragegatewayGatewayTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `activation_key` after provisioning.\n"]
    pub fn activation_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.activation_key", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `average_download_rate_limit_in_bits_per_sec` after provisioning.\n"]
    pub fn average_download_rate_limit_in_bits_per_sec(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.average_download_rate_limit_in_bits_per_sec",
                self.extract_ref()
            ),
        )
    }

    #[doc = "Get a reference to the value of field `average_upload_rate_limit_in_bits_per_sec` after provisioning.\n"]
    pub fn average_upload_rate_limit_in_bits_per_sec(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.average_upload_rate_limit_in_bits_per_sec",
                self.extract_ref()
            ),
        )
    }

    #[doc = "Get a reference to the value of field `cloudwatch_log_group_arn` after provisioning.\n"]
    pub fn cloudwatch_log_group_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cloudwatch_log_group_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `ec2_instance_id` after provisioning.\n"]
    pub fn ec2_instance_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ec2_instance_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `endpoint_type` after provisioning.\n"]
    pub fn endpoint_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.endpoint_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `gateway_id` after provisioning.\n"]
    pub fn gateway_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gateway_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `gateway_ip_address` after provisioning.\n"]
    pub fn gateway_ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gateway_ip_address", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `gateway_name` after provisioning.\n"]
    pub fn gateway_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gateway_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `gateway_network_interface` after provisioning.\n"]
    pub fn gateway_network_interface(
        &self,
    ) -> ListRef<StoragegatewayGatewayGatewayNetworkInterfaceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.gateway_network_interface", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `gateway_timezone` after provisioning.\n"]
    pub fn gateway_timezone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gateway_timezone", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `gateway_type` after provisioning.\n"]
    pub fn gateway_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gateway_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `gateway_vpc_endpoint` after provisioning.\n"]
    pub fn gateway_vpc_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gateway_vpc_endpoint", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `host_environment` after provisioning.\n"]
    pub fn host_environment(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.host_environment", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `medium_changer_type` after provisioning.\n"]
    pub fn medium_changer_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.medium_changer_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `smb_file_share_visibility` after provisioning.\n"]
    pub fn smb_file_share_visibility(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.smb_file_share_visibility", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `smb_guest_password` after provisioning.\n"]
    pub fn smb_guest_password(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.smb_guest_password", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `smb_security_strategy` after provisioning.\n"]
    pub fn smb_security_strategy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.smb_security_strategy", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `tape_drive_type` after provisioning.\n"]
    pub fn tape_drive_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.tape_drive_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `maintenance_start_time` after provisioning.\n"]
    pub fn maintenance_start_time(
        &self,
    ) -> ListRef<StoragegatewayGatewayMaintenanceStartTimeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.maintenance_start_time", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `smb_active_directory_settings` after provisioning.\n"]
    pub fn smb_active_directory_settings(
        &self,
    ) -> ListRef<StoragegatewayGatewaySmbActiveDirectorySettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.smb_active_directory_settings", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> StoragegatewayGatewayTimeoutsElRef {
        StoragegatewayGatewayTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for StoragegatewayGateway {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for StoragegatewayGateway {}

impl ToListMappable for StoragegatewayGateway {
    type O = ListRef<StoragegatewayGatewayRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for StoragegatewayGateway_ {
    fn extract_resource_type(&self) -> String {
        "aws_storagegateway_gateway".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildStoragegatewayGateway {
    pub tf_id: String,
    #[doc = ""]
    pub gateway_name: PrimField<String>,
    #[doc = ""]
    pub gateway_timezone: PrimField<String>,
}

impl BuildStoragegatewayGateway {
    pub fn build(self, stack: &mut Stack) -> StoragegatewayGateway {
        let out = StoragegatewayGateway(Rc::new(StoragegatewayGateway_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(StoragegatewayGatewayData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                activation_key: core::default::Default::default(),
                average_download_rate_limit_in_bits_per_sec: core::default::Default::default(),
                average_upload_rate_limit_in_bits_per_sec: core::default::Default::default(),
                cloudwatch_log_group_arn: core::default::Default::default(),
                gateway_ip_address: core::default::Default::default(),
                gateway_name: self.gateway_name,
                gateway_timezone: self.gateway_timezone,
                gateway_type: core::default::Default::default(),
                gateway_vpc_endpoint: core::default::Default::default(),
                id: core::default::Default::default(),
                medium_changer_type: core::default::Default::default(),
                region: core::default::Default::default(),
                smb_file_share_visibility: core::default::Default::default(),
                smb_guest_password: core::default::Default::default(),
                smb_security_strategy: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                tape_drive_type: core::default::Default::default(),
                maintenance_start_time: core::default::Default::default(),
                smb_active_directory_settings: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct StoragegatewayGatewayRef {
    shared: StackShared,
    base: String,
}

impl Ref for StoragegatewayGatewayRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl StoragegatewayGatewayRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `activation_key` after provisioning.\n"]
    pub fn activation_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.activation_key", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `average_download_rate_limit_in_bits_per_sec` after provisioning.\n"]
    pub fn average_download_rate_limit_in_bits_per_sec(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.average_download_rate_limit_in_bits_per_sec",
                self.extract_ref()
            ),
        )
    }

    #[doc = "Get a reference to the value of field `average_upload_rate_limit_in_bits_per_sec` after provisioning.\n"]
    pub fn average_upload_rate_limit_in_bits_per_sec(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.average_upload_rate_limit_in_bits_per_sec",
                self.extract_ref()
            ),
        )
    }

    #[doc = "Get a reference to the value of field `cloudwatch_log_group_arn` after provisioning.\n"]
    pub fn cloudwatch_log_group_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cloudwatch_log_group_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `ec2_instance_id` after provisioning.\n"]
    pub fn ec2_instance_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ec2_instance_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `endpoint_type` after provisioning.\n"]
    pub fn endpoint_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.endpoint_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `gateway_id` after provisioning.\n"]
    pub fn gateway_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gateway_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `gateway_ip_address` after provisioning.\n"]
    pub fn gateway_ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gateway_ip_address", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `gateway_name` after provisioning.\n"]
    pub fn gateway_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gateway_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `gateway_network_interface` after provisioning.\n"]
    pub fn gateway_network_interface(
        &self,
    ) -> ListRef<StoragegatewayGatewayGatewayNetworkInterfaceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.gateway_network_interface", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `gateway_timezone` after provisioning.\n"]
    pub fn gateway_timezone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gateway_timezone", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `gateway_type` after provisioning.\n"]
    pub fn gateway_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gateway_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `gateway_vpc_endpoint` after provisioning.\n"]
    pub fn gateway_vpc_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gateway_vpc_endpoint", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `host_environment` after provisioning.\n"]
    pub fn host_environment(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.host_environment", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `medium_changer_type` after provisioning.\n"]
    pub fn medium_changer_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.medium_changer_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `smb_file_share_visibility` after provisioning.\n"]
    pub fn smb_file_share_visibility(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.smb_file_share_visibility", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `smb_guest_password` after provisioning.\n"]
    pub fn smb_guest_password(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.smb_guest_password", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `smb_security_strategy` after provisioning.\n"]
    pub fn smb_security_strategy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.smb_security_strategy", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `tape_drive_type` after provisioning.\n"]
    pub fn tape_drive_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.tape_drive_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `maintenance_start_time` after provisioning.\n"]
    pub fn maintenance_start_time(
        &self,
    ) -> ListRef<StoragegatewayGatewayMaintenanceStartTimeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.maintenance_start_time", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `smb_active_directory_settings` after provisioning.\n"]
    pub fn smb_active_directory_settings(
        &self,
    ) -> ListRef<StoragegatewayGatewaySmbActiveDirectorySettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.smb_active_directory_settings", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> StoragegatewayGatewayTimeoutsElRef {
        StoragegatewayGatewayTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct StoragegatewayGatewayGatewayNetworkInterfaceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ipv4_address: Option<PrimField<String>>,
}

impl StoragegatewayGatewayGatewayNetworkInterfaceEl {
    #[doc = "Set the field `ipv4_address`.\n"]
    pub fn set_ipv4_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ipv4_address = Some(v.into());
        self
    }
}

impl ToListMappable for StoragegatewayGatewayGatewayNetworkInterfaceEl {
    type O = BlockAssignable<StoragegatewayGatewayGatewayNetworkInterfaceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStoragegatewayGatewayGatewayNetworkInterfaceEl {}

impl BuildStoragegatewayGatewayGatewayNetworkInterfaceEl {
    pub fn build(self) -> StoragegatewayGatewayGatewayNetworkInterfaceEl {
        StoragegatewayGatewayGatewayNetworkInterfaceEl {
            ipv4_address: core::default::Default::default(),
        }
    }
}

pub struct StoragegatewayGatewayGatewayNetworkInterfaceElRef {
    shared: StackShared,
    base: String,
}

impl Ref for StoragegatewayGatewayGatewayNetworkInterfaceElRef {
    fn new(shared: StackShared, base: String) -> StoragegatewayGatewayGatewayNetworkInterfaceElRef {
        StoragegatewayGatewayGatewayNetworkInterfaceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StoragegatewayGatewayGatewayNetworkInterfaceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ipv4_address` after provisioning.\n"]
    pub fn ipv4_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipv4_address", self.base))
    }
}

#[derive(Serialize)]
pub struct StoragegatewayGatewayMaintenanceStartTimeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    day_of_month: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    day_of_week: Option<PrimField<String>>,
    hour_of_day: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minute_of_hour: Option<PrimField<f64>>,
}

impl StoragegatewayGatewayMaintenanceStartTimeEl {
    #[doc = "Set the field `day_of_month`.\n"]
    pub fn set_day_of_month(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.day_of_month = Some(v.into());
        self
    }

    #[doc = "Set the field `day_of_week`.\n"]
    pub fn set_day_of_week(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.day_of_week = Some(v.into());
        self
    }

    #[doc = "Set the field `minute_of_hour`.\n"]
    pub fn set_minute_of_hour(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.minute_of_hour = Some(v.into());
        self
    }
}

impl ToListMappable for StoragegatewayGatewayMaintenanceStartTimeEl {
    type O = BlockAssignable<StoragegatewayGatewayMaintenanceStartTimeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStoragegatewayGatewayMaintenanceStartTimeEl {
    #[doc = ""]
    pub hour_of_day: PrimField<f64>,
}

impl BuildStoragegatewayGatewayMaintenanceStartTimeEl {
    pub fn build(self) -> StoragegatewayGatewayMaintenanceStartTimeEl {
        StoragegatewayGatewayMaintenanceStartTimeEl {
            day_of_month: core::default::Default::default(),
            day_of_week: core::default::Default::default(),
            hour_of_day: self.hour_of_day,
            minute_of_hour: core::default::Default::default(),
        }
    }
}

pub struct StoragegatewayGatewayMaintenanceStartTimeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for StoragegatewayGatewayMaintenanceStartTimeElRef {
    fn new(shared: StackShared, base: String) -> StoragegatewayGatewayMaintenanceStartTimeElRef {
        StoragegatewayGatewayMaintenanceStartTimeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StoragegatewayGatewayMaintenanceStartTimeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `day_of_month` after provisioning.\n"]
    pub fn day_of_month(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.day_of_month", self.base))
    }

    #[doc = "Get a reference to the value of field `day_of_week` after provisioning.\n"]
    pub fn day_of_week(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.day_of_week", self.base))
    }

    #[doc = "Get a reference to the value of field `hour_of_day` after provisioning.\n"]
    pub fn hour_of_day(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.hour_of_day", self.base))
    }

    #[doc = "Get a reference to the value of field `minute_of_hour` after provisioning.\n"]
    pub fn minute_of_hour(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.minute_of_hour", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct StoragegatewayGatewaySmbActiveDirectorySettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    domain_controllers: Option<SetField<PrimField<String>>>,
    domain_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    organizational_unit: Option<PrimField<String>>,
    password: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout_in_seconds: Option<PrimField<f64>>,
    username: PrimField<String>,
}

impl StoragegatewayGatewaySmbActiveDirectorySettingsEl {
    #[doc = "Set the field `domain_controllers`.\n"]
    pub fn set_domain_controllers(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.domain_controllers = Some(v.into());
        self
    }

    #[doc = "Set the field `organizational_unit`.\n"]
    pub fn set_organizational_unit(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.organizational_unit = Some(v.into());
        self
    }

    #[doc = "Set the field `timeout_in_seconds`.\n"]
    pub fn set_timeout_in_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.timeout_in_seconds = Some(v.into());
        self
    }
}

impl ToListMappable for StoragegatewayGatewaySmbActiveDirectorySettingsEl {
    type O = BlockAssignable<StoragegatewayGatewaySmbActiveDirectorySettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStoragegatewayGatewaySmbActiveDirectorySettingsEl {
    #[doc = ""]
    pub domain_name: PrimField<String>,
    #[doc = ""]
    pub password: PrimField<String>,
    #[doc = ""]
    pub username: PrimField<String>,
}

impl BuildStoragegatewayGatewaySmbActiveDirectorySettingsEl {
    pub fn build(self) -> StoragegatewayGatewaySmbActiveDirectorySettingsEl {
        StoragegatewayGatewaySmbActiveDirectorySettingsEl {
            domain_controllers: core::default::Default::default(),
            domain_name: self.domain_name,
            organizational_unit: core::default::Default::default(),
            password: self.password,
            timeout_in_seconds: core::default::Default::default(),
            username: self.username,
        }
    }
}

pub struct StoragegatewayGatewaySmbActiveDirectorySettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for StoragegatewayGatewaySmbActiveDirectorySettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> StoragegatewayGatewaySmbActiveDirectorySettingsElRef {
        StoragegatewayGatewaySmbActiveDirectorySettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StoragegatewayGatewaySmbActiveDirectorySettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `active_directory_status` after provisioning.\n"]
    pub fn active_directory_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.active_directory_status", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `domain_controllers` after provisioning.\n"]
    pub fn domain_controllers(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.domain_controllers", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.base))
    }

    #[doc = "Get a reference to the value of field `organizational_unit` after provisioning.\n"]
    pub fn organizational_unit(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.organizational_unit", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `password` after provisioning.\n"]
    pub fn password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout_in_seconds` after provisioning.\n"]
    pub fn timeout_in_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.timeout_in_seconds", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `username` after provisioning.\n"]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}

#[derive(Serialize)]
pub struct StoragegatewayGatewayTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
}

impl StoragegatewayGatewayTimeoutsEl {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }
}

impl ToListMappable for StoragegatewayGatewayTimeoutsEl {
    type O = BlockAssignable<StoragegatewayGatewayTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStoragegatewayGatewayTimeoutsEl {}

impl BuildStoragegatewayGatewayTimeoutsEl {
    pub fn build(self) -> StoragegatewayGatewayTimeoutsEl {
        StoragegatewayGatewayTimeoutsEl {
            create: core::default::Default::default(),
        }
    }
}

pub struct StoragegatewayGatewayTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for StoragegatewayGatewayTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> StoragegatewayGatewayTimeoutsElRef {
        StoragegatewayGatewayTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StoragegatewayGatewayTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }
}

#[derive(Serialize, Default)]
struct StoragegatewayGatewayDynamic {
    maintenance_start_time: Option<DynamicBlock<StoragegatewayGatewayMaintenanceStartTimeEl>>,
    smb_active_directory_settings:
        Option<DynamicBlock<StoragegatewayGatewaySmbActiveDirectorySettingsEl>>,
}
