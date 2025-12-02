use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataLaunchConfigurationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}
struct DataLaunchConfiguration_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataLaunchConfigurationData>,
}
#[derive(Clone)]
pub struct DataLaunchConfiguration(Rc<DataLaunchConfiguration_>);
impl DataLaunchConfiguration {
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
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `associate_public_ip_address` after provisioning.\n"]
    pub fn associate_public_ip_address(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.associate_public_ip_address", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ebs_block_device` after provisioning.\n"]
    pub fn ebs_block_device(&self) -> SetRef<DataLaunchConfigurationEbsBlockDeviceElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.ebs_block_device", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ebs_optimized` after provisioning.\n"]
    pub fn ebs_optimized(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ebs_optimized", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_monitoring` after provisioning.\n"]
    pub fn enable_monitoring(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_monitoring", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ephemeral_block_device` after provisioning.\n"]
    pub fn ephemeral_block_device(
        &self,
    ) -> SetRef<DataLaunchConfigurationEphemeralBlockDeviceElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.ephemeral_block_device", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `iam_instance_profile` after provisioning.\n"]
    pub fn iam_instance_profile(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.iam_instance_profile", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `image_id` after provisioning.\n"]
    pub fn image_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.image_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `key_name` after provisioning.\n"]
    pub fn key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.key_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `metadata_options` after provisioning.\n"]
    pub fn metadata_options(&self) -> ListRef<DataLaunchConfigurationMetadataOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.metadata_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `placement_tenancy` after provisioning.\n"]
    pub fn placement_tenancy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.placement_tenancy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `root_block_device` after provisioning.\n"]
    pub fn root_block_device(&self) -> ListRef<DataLaunchConfigurationRootBlockDeviceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.root_block_device", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `security_groups` after provisioning.\n"]
    pub fn security_groups(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.security_groups", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `spot_price` after provisioning.\n"]
    pub fn spot_price(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.spot_price", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `user_data` after provisioning.\n"]
    pub fn user_data(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_data", self.extract_ref()),
        )
    }
}
impl Referable for DataLaunchConfiguration {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataLaunchConfiguration {}
impl ToListMappable for DataLaunchConfiguration {
    type O = ListRef<DataLaunchConfigurationRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataLaunchConfiguration_ {
    fn extract_datasource_type(&self) -> String {
        "aws_launch_configuration".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataLaunchConfiguration {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
}
impl BuildDataLaunchConfiguration {
    pub fn build(self, stack: &mut Stack) -> DataLaunchConfiguration {
        let out = DataLaunchConfiguration(Rc::new(DataLaunchConfiguration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataLaunchConfigurationData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataLaunchConfigurationRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLaunchConfigurationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataLaunchConfigurationRef {
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
    #[doc = "Get a reference to the value of field `associate_public_ip_address` after provisioning.\n"]
    pub fn associate_public_ip_address(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.associate_public_ip_address", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ebs_block_device` after provisioning.\n"]
    pub fn ebs_block_device(&self) -> SetRef<DataLaunchConfigurationEbsBlockDeviceElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.ebs_block_device", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ebs_optimized` after provisioning.\n"]
    pub fn ebs_optimized(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ebs_optimized", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_monitoring` after provisioning.\n"]
    pub fn enable_monitoring(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_monitoring", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ephemeral_block_device` after provisioning.\n"]
    pub fn ephemeral_block_device(
        &self,
    ) -> SetRef<DataLaunchConfigurationEphemeralBlockDeviceElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.ephemeral_block_device", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `iam_instance_profile` after provisioning.\n"]
    pub fn iam_instance_profile(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.iam_instance_profile", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `image_id` after provisioning.\n"]
    pub fn image_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.image_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `key_name` after provisioning.\n"]
    pub fn key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.key_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `metadata_options` after provisioning.\n"]
    pub fn metadata_options(&self) -> ListRef<DataLaunchConfigurationMetadataOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.metadata_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `placement_tenancy` after provisioning.\n"]
    pub fn placement_tenancy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.placement_tenancy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `root_block_device` after provisioning.\n"]
    pub fn root_block_device(&self) -> ListRef<DataLaunchConfigurationRootBlockDeviceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.root_block_device", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `security_groups` after provisioning.\n"]
    pub fn security_groups(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.security_groups", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `spot_price` after provisioning.\n"]
    pub fn spot_price(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.spot_price", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `user_data` after provisioning.\n"]
    pub fn user_data(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_data", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataLaunchConfigurationEbsBlockDeviceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    delete_on_termination: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    device_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encrypted: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iops: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    no_device: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    snapshot_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    throughput: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volume_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volume_type: Option<PrimField<String>>,
}
impl DataLaunchConfigurationEbsBlockDeviceEl {
    #[doc = "Set the field `delete_on_termination`.\n"]
    pub fn set_delete_on_termination(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.delete_on_termination = Some(v.into());
        self
    }
    #[doc = "Set the field `device_name`.\n"]
    pub fn set_device_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.device_name = Some(v.into());
        self
    }
    #[doc = "Set the field `encrypted`.\n"]
    pub fn set_encrypted(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.encrypted = Some(v.into());
        self
    }
    #[doc = "Set the field `iops`.\n"]
    pub fn set_iops(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.iops = Some(v.into());
        self
    }
    #[doc = "Set the field `no_device`.\n"]
    pub fn set_no_device(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.no_device = Some(v.into());
        self
    }
    #[doc = "Set the field `snapshot_id`.\n"]
    pub fn set_snapshot_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.snapshot_id = Some(v.into());
        self
    }
    #[doc = "Set the field `throughput`.\n"]
    pub fn set_throughput(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.throughput = Some(v.into());
        self
    }
    #[doc = "Set the field `volume_size`.\n"]
    pub fn set_volume_size(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.volume_size = Some(v.into());
        self
    }
    #[doc = "Set the field `volume_type`.\n"]
    pub fn set_volume_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.volume_type = Some(v.into());
        self
    }
}
impl ToListMappable for DataLaunchConfigurationEbsBlockDeviceEl {
    type O = BlockAssignable<DataLaunchConfigurationEbsBlockDeviceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLaunchConfigurationEbsBlockDeviceEl {}
impl BuildDataLaunchConfigurationEbsBlockDeviceEl {
    pub fn build(self) -> DataLaunchConfigurationEbsBlockDeviceEl {
        DataLaunchConfigurationEbsBlockDeviceEl {
            delete_on_termination: core::default::Default::default(),
            device_name: core::default::Default::default(),
            encrypted: core::default::Default::default(),
            iops: core::default::Default::default(),
            no_device: core::default::Default::default(),
            snapshot_id: core::default::Default::default(),
            throughput: core::default::Default::default(),
            volume_size: core::default::Default::default(),
            volume_type: core::default::Default::default(),
        }
    }
}
pub struct DataLaunchConfigurationEbsBlockDeviceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLaunchConfigurationEbsBlockDeviceElRef {
    fn new(shared: StackShared, base: String) -> DataLaunchConfigurationEbsBlockDeviceElRef {
        DataLaunchConfigurationEbsBlockDeviceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLaunchConfigurationEbsBlockDeviceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `delete_on_termination` after provisioning.\n"]
    pub fn delete_on_termination(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delete_on_termination", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `device_name` after provisioning.\n"]
    pub fn device_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.device_name", self.base))
    }
    #[doc = "Get a reference to the value of field `encrypted` after provisioning.\n"]
    pub fn encrypted(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.encrypted", self.base))
    }
    #[doc = "Get a reference to the value of field `iops` after provisioning.\n"]
    pub fn iops(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.iops", self.base))
    }
    #[doc = "Get a reference to the value of field `no_device` after provisioning.\n"]
    pub fn no_device(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.no_device", self.base))
    }
    #[doc = "Get a reference to the value of field `snapshot_id` after provisioning.\n"]
    pub fn snapshot_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.snapshot_id", self.base))
    }
    #[doc = "Get a reference to the value of field `throughput` after provisioning.\n"]
    pub fn throughput(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.throughput", self.base))
    }
    #[doc = "Get a reference to the value of field `volume_size` after provisioning.\n"]
    pub fn volume_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.volume_size", self.base))
    }
    #[doc = "Get a reference to the value of field `volume_type` after provisioning.\n"]
    pub fn volume_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.volume_type", self.base))
    }
}
#[derive(Serialize)]
pub struct DataLaunchConfigurationEphemeralBlockDeviceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    device_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    virtual_name: Option<PrimField<String>>,
}
impl DataLaunchConfigurationEphemeralBlockDeviceEl {
    #[doc = "Set the field `device_name`.\n"]
    pub fn set_device_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.device_name = Some(v.into());
        self
    }
    #[doc = "Set the field `virtual_name`.\n"]
    pub fn set_virtual_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.virtual_name = Some(v.into());
        self
    }
}
impl ToListMappable for DataLaunchConfigurationEphemeralBlockDeviceEl {
    type O = BlockAssignable<DataLaunchConfigurationEphemeralBlockDeviceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLaunchConfigurationEphemeralBlockDeviceEl {}
impl BuildDataLaunchConfigurationEphemeralBlockDeviceEl {
    pub fn build(self) -> DataLaunchConfigurationEphemeralBlockDeviceEl {
        DataLaunchConfigurationEphemeralBlockDeviceEl {
            device_name: core::default::Default::default(),
            virtual_name: core::default::Default::default(),
        }
    }
}
pub struct DataLaunchConfigurationEphemeralBlockDeviceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLaunchConfigurationEphemeralBlockDeviceElRef {
    fn new(shared: StackShared, base: String) -> DataLaunchConfigurationEphemeralBlockDeviceElRef {
        DataLaunchConfigurationEphemeralBlockDeviceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLaunchConfigurationEphemeralBlockDeviceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `device_name` after provisioning.\n"]
    pub fn device_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.device_name", self.base))
    }
    #[doc = "Get a reference to the value of field `virtual_name` after provisioning.\n"]
    pub fn virtual_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.virtual_name", self.base))
    }
}
#[derive(Serialize)]
pub struct DataLaunchConfigurationMetadataOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    http_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http_put_response_hop_limit: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http_tokens: Option<PrimField<String>>,
}
impl DataLaunchConfigurationMetadataOptionsEl {
    #[doc = "Set the field `http_endpoint`.\n"]
    pub fn set_http_endpoint(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.http_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `http_put_response_hop_limit`.\n"]
    pub fn set_http_put_response_hop_limit(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.http_put_response_hop_limit = Some(v.into());
        self
    }
    #[doc = "Set the field `http_tokens`.\n"]
    pub fn set_http_tokens(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.http_tokens = Some(v.into());
        self
    }
}
impl ToListMappable for DataLaunchConfigurationMetadataOptionsEl {
    type O = BlockAssignable<DataLaunchConfigurationMetadataOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLaunchConfigurationMetadataOptionsEl {}
impl BuildDataLaunchConfigurationMetadataOptionsEl {
    pub fn build(self) -> DataLaunchConfigurationMetadataOptionsEl {
        DataLaunchConfigurationMetadataOptionsEl {
            http_endpoint: core::default::Default::default(),
            http_put_response_hop_limit: core::default::Default::default(),
            http_tokens: core::default::Default::default(),
        }
    }
}
pub struct DataLaunchConfigurationMetadataOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLaunchConfigurationMetadataOptionsElRef {
    fn new(shared: StackShared, base: String) -> DataLaunchConfigurationMetadataOptionsElRef {
        DataLaunchConfigurationMetadataOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLaunchConfigurationMetadataOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `http_endpoint` after provisioning.\n"]
    pub fn http_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.http_endpoint", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `http_put_response_hop_limit` after provisioning.\n"]
    pub fn http_put_response_hop_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.http_put_response_hop_limit", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `http_tokens` after provisioning.\n"]
    pub fn http_tokens(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.http_tokens", self.base))
    }
}
#[derive(Serialize)]
pub struct DataLaunchConfigurationRootBlockDeviceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    delete_on_termination: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encrypted: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iops: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    throughput: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volume_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volume_type: Option<PrimField<String>>,
}
impl DataLaunchConfigurationRootBlockDeviceEl {
    #[doc = "Set the field `delete_on_termination`.\n"]
    pub fn set_delete_on_termination(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.delete_on_termination = Some(v.into());
        self
    }
    #[doc = "Set the field `encrypted`.\n"]
    pub fn set_encrypted(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.encrypted = Some(v.into());
        self
    }
    #[doc = "Set the field `iops`.\n"]
    pub fn set_iops(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.iops = Some(v.into());
        self
    }
    #[doc = "Set the field `throughput`.\n"]
    pub fn set_throughput(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.throughput = Some(v.into());
        self
    }
    #[doc = "Set the field `volume_size`.\n"]
    pub fn set_volume_size(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.volume_size = Some(v.into());
        self
    }
    #[doc = "Set the field `volume_type`.\n"]
    pub fn set_volume_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.volume_type = Some(v.into());
        self
    }
}
impl ToListMappable for DataLaunchConfigurationRootBlockDeviceEl {
    type O = BlockAssignable<DataLaunchConfigurationRootBlockDeviceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLaunchConfigurationRootBlockDeviceEl {}
impl BuildDataLaunchConfigurationRootBlockDeviceEl {
    pub fn build(self) -> DataLaunchConfigurationRootBlockDeviceEl {
        DataLaunchConfigurationRootBlockDeviceEl {
            delete_on_termination: core::default::Default::default(),
            encrypted: core::default::Default::default(),
            iops: core::default::Default::default(),
            throughput: core::default::Default::default(),
            volume_size: core::default::Default::default(),
            volume_type: core::default::Default::default(),
        }
    }
}
pub struct DataLaunchConfigurationRootBlockDeviceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLaunchConfigurationRootBlockDeviceElRef {
    fn new(shared: StackShared, base: String) -> DataLaunchConfigurationRootBlockDeviceElRef {
        DataLaunchConfigurationRootBlockDeviceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLaunchConfigurationRootBlockDeviceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `delete_on_termination` after provisioning.\n"]
    pub fn delete_on_termination(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delete_on_termination", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `encrypted` after provisioning.\n"]
    pub fn encrypted(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.encrypted", self.base))
    }
    #[doc = "Get a reference to the value of field `iops` after provisioning.\n"]
    pub fn iops(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.iops", self.base))
    }
    #[doc = "Get a reference to the value of field `throughput` after provisioning.\n"]
    pub fn throughput(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.throughput", self.base))
    }
    #[doc = "Get a reference to the value of field `volume_size` after provisioning.\n"]
    pub fn volume_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.volume_size", self.base))
    }
    #[doc = "Get a reference to the value of field `volume_type` after provisioning.\n"]
    pub fn volume_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.volume_type", self.base))
    }
}
