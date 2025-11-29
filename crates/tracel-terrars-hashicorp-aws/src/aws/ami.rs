use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct AmiData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    architecture: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    boot_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deprecation_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ena_support: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    imds_support: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kernel_id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ramdisk_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    root_device_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sriov_net_support: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tpm_support: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    uefi_data: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    virtualization_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ebs_block_device: Option<Vec<AmiEbsBlockDeviceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ephemeral_block_device: Option<Vec<AmiEphemeralBlockDeviceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<AmiTimeoutsEl>,
    dynamic: AmiDynamic,
}

struct Ami_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<AmiData>,
}

#[derive(Clone)]
pub struct Ami(Rc<Ami_>);

impl Ami {
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

    #[doc = "Set the field `architecture`.\n"]
    pub fn set_architecture(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().architecture = Some(v.into());
        self
    }

    #[doc = "Set the field `boot_mode`.\n"]
    pub fn set_boot_mode(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().boot_mode = Some(v.into());
        self
    }

    #[doc = "Set the field `deprecation_time`.\n"]
    pub fn set_deprecation_time(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deprecation_time = Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc = "Set the field `ena_support`.\n"]
    pub fn set_ena_support(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().ena_support = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `image_location`.\n"]
    pub fn set_image_location(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().image_location = Some(v.into());
        self
    }

    #[doc = "Set the field `imds_support`.\n"]
    pub fn set_imds_support(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().imds_support = Some(v.into());
        self
    }

    #[doc = "Set the field `kernel_id`.\n"]
    pub fn set_kernel_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().kernel_id = Some(v.into());
        self
    }

    #[doc = "Set the field `ramdisk_id`.\n"]
    pub fn set_ramdisk_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().ramdisk_id = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `root_device_name`.\n"]
    pub fn set_root_device_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().root_device_name = Some(v.into());
        self
    }

    #[doc = "Set the field `sriov_net_support`.\n"]
    pub fn set_sriov_net_support(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().sriov_net_support = Some(v.into());
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

    #[doc = "Set the field `tpm_support`.\n"]
    pub fn set_tpm_support(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().tpm_support = Some(v.into());
        self
    }

    #[doc = "Set the field `uefi_data`.\n"]
    pub fn set_uefi_data(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().uefi_data = Some(v.into());
        self
    }

    #[doc = "Set the field `virtualization_type`.\n"]
    pub fn set_virtualization_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().virtualization_type = Some(v.into());
        self
    }

    #[doc = "Set the field `ebs_block_device`.\n"]
    pub fn set_ebs_block_device(self, v: impl Into<BlockAssignable<AmiEbsBlockDeviceEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ebs_block_device = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ebs_block_device = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ephemeral_block_device`.\n"]
    pub fn set_ephemeral_block_device(self, v: impl Into<BlockAssignable<AmiEphemeralBlockDeviceEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ephemeral_block_device = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ephemeral_block_device = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<AmiTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `architecture` after provisioning.\n"]
    pub fn architecture(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.architecture", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `boot_mode` after provisioning.\n"]
    pub fn boot_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.boot_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deprecation_time` after provisioning.\n"]
    pub fn deprecation_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.deprecation_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ena_support` after provisioning.\n"]
    pub fn ena_support(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.ena_support", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hypervisor` after provisioning.\n"]
    pub fn hypervisor(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.hypervisor", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_location` after provisioning.\n"]
    pub fn image_location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_owner_alias` after provisioning.\n"]
    pub fn image_owner_alias(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_owner_alias", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_type` after provisioning.\n"]
    pub fn image_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `imds_support` after provisioning.\n"]
    pub fn imds_support(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.imds_support", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kernel_id` after provisioning.\n"]
    pub fn kernel_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kernel_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_launched_time` after provisioning.\n"]
    pub fn last_launched_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.last_launched_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `manage_ebs_snapshots` after provisioning.\n"]
    pub fn manage_ebs_snapshots(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.manage_ebs_snapshots", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.owner_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `platform` after provisioning.\n"]
    pub fn platform(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.platform", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `platform_details` after provisioning.\n"]
    pub fn platform_details(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.platform_details", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `public` after provisioning.\n"]
    pub fn public(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.public", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ramdisk_id` after provisioning.\n"]
    pub fn ramdisk_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ramdisk_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `root_device_name` after provisioning.\n"]
    pub fn root_device_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.root_device_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `root_snapshot_id` after provisioning.\n"]
    pub fn root_snapshot_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.root_snapshot_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sriov_net_support` after provisioning.\n"]
    pub fn sriov_net_support(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sriov_net_support", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tpm_support` after provisioning.\n"]
    pub fn tpm_support(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tpm_support", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `uefi_data` after provisioning.\n"]
    pub fn uefi_data(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uefi_data", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `usage_operation` after provisioning.\n"]
    pub fn usage_operation(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.usage_operation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `virtualization_type` after provisioning.\n"]
    pub fn virtualization_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.virtualization_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AmiTimeoutsElRef {
        AmiTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for Ami {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for Ami { }

impl ToListMappable for Ami {
    type O = ListRef<AmiRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for Ami_ {
    fn extract_resource_type(&self) -> String {
        "aws_ami".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAmi {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildAmi {
    pub fn build(self, stack: &mut Stack) -> Ami {
        let out = Ami(Rc::new(Ami_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(AmiData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                architecture: core::default::Default::default(),
                boot_mode: core::default::Default::default(),
                deprecation_time: core::default::Default::default(),
                description: core::default::Default::default(),
                ena_support: core::default::Default::default(),
                id: core::default::Default::default(),
                image_location: core::default::Default::default(),
                imds_support: core::default::Default::default(),
                kernel_id: core::default::Default::default(),
                name: self.name,
                ramdisk_id: core::default::Default::default(),
                region: core::default::Default::default(),
                root_device_name: core::default::Default::default(),
                sriov_net_support: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                tpm_support: core::default::Default::default(),
                uefi_data: core::default::Default::default(),
                virtualization_type: core::default::Default::default(),
                ebs_block_device: core::default::Default::default(),
                ephemeral_block_device: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AmiRef {
    shared: StackShared,
    base: String,
}

impl Ref for AmiRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl AmiRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `architecture` after provisioning.\n"]
    pub fn architecture(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.architecture", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `boot_mode` after provisioning.\n"]
    pub fn boot_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.boot_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deprecation_time` after provisioning.\n"]
    pub fn deprecation_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.deprecation_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ena_support` after provisioning.\n"]
    pub fn ena_support(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.ena_support", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hypervisor` after provisioning.\n"]
    pub fn hypervisor(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.hypervisor", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_location` after provisioning.\n"]
    pub fn image_location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_owner_alias` after provisioning.\n"]
    pub fn image_owner_alias(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_owner_alias", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_type` after provisioning.\n"]
    pub fn image_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `imds_support` after provisioning.\n"]
    pub fn imds_support(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.imds_support", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kernel_id` after provisioning.\n"]
    pub fn kernel_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kernel_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_launched_time` after provisioning.\n"]
    pub fn last_launched_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.last_launched_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `manage_ebs_snapshots` after provisioning.\n"]
    pub fn manage_ebs_snapshots(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.manage_ebs_snapshots", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.owner_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `platform` after provisioning.\n"]
    pub fn platform(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.platform", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `platform_details` after provisioning.\n"]
    pub fn platform_details(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.platform_details", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `public` after provisioning.\n"]
    pub fn public(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.public", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ramdisk_id` after provisioning.\n"]
    pub fn ramdisk_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ramdisk_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `root_device_name` after provisioning.\n"]
    pub fn root_device_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.root_device_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `root_snapshot_id` after provisioning.\n"]
    pub fn root_snapshot_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.root_snapshot_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sriov_net_support` after provisioning.\n"]
    pub fn sriov_net_support(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sriov_net_support", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tpm_support` after provisioning.\n"]
    pub fn tpm_support(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tpm_support", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `uefi_data` after provisioning.\n"]
    pub fn uefi_data(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uefi_data", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `usage_operation` after provisioning.\n"]
    pub fn usage_operation(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.usage_operation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `virtualization_type` after provisioning.\n"]
    pub fn virtualization_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.virtualization_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AmiTimeoutsElRef {
        AmiTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct AmiEbsBlockDeviceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    delete_on_termination: Option<PrimField<bool>>,
    device_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encrypted: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iops: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    outpost_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    snapshot_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    throughput: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volume_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volume_type: Option<PrimField<String>>,
}

impl AmiEbsBlockDeviceEl {
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

    #[doc = "Set the field `outpost_arn`.\n"]
    pub fn set_outpost_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.outpost_arn = Some(v.into());
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

impl ToListMappable for AmiEbsBlockDeviceEl {
    type O = BlockAssignable<AmiEbsBlockDeviceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAmiEbsBlockDeviceEl {
    #[doc = ""]
    pub device_name: PrimField<String>,
}

impl BuildAmiEbsBlockDeviceEl {
    pub fn build(self) -> AmiEbsBlockDeviceEl {
        AmiEbsBlockDeviceEl {
            delete_on_termination: core::default::Default::default(),
            device_name: self.device_name,
            encrypted: core::default::Default::default(),
            iops: core::default::Default::default(),
            outpost_arn: core::default::Default::default(),
            snapshot_id: core::default::Default::default(),
            throughput: core::default::Default::default(),
            volume_size: core::default::Default::default(),
            volume_type: core::default::Default::default(),
        }
    }
}

pub struct AmiEbsBlockDeviceElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AmiEbsBlockDeviceElRef {
    fn new(shared: StackShared, base: String) -> AmiEbsBlockDeviceElRef {
        AmiEbsBlockDeviceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AmiEbsBlockDeviceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `delete_on_termination` after provisioning.\n"]
    pub fn delete_on_termination(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete_on_termination", self.base))
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

    #[doc = "Get a reference to the value of field `outpost_arn` after provisioning.\n"]
    pub fn outpost_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.outpost_arn", self.base))
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
pub struct AmiEphemeralBlockDeviceEl {
    device_name: PrimField<String>,
    virtual_name: PrimField<String>,
}

impl AmiEphemeralBlockDeviceEl { }

impl ToListMappable for AmiEphemeralBlockDeviceEl {
    type O = BlockAssignable<AmiEphemeralBlockDeviceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAmiEphemeralBlockDeviceEl {
    #[doc = ""]
    pub device_name: PrimField<String>,
    #[doc = ""]
    pub virtual_name: PrimField<String>,
}

impl BuildAmiEphemeralBlockDeviceEl {
    pub fn build(self) -> AmiEphemeralBlockDeviceEl {
        AmiEphemeralBlockDeviceEl {
            device_name: self.device_name,
            virtual_name: self.virtual_name,
        }
    }
}

pub struct AmiEphemeralBlockDeviceElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AmiEphemeralBlockDeviceElRef {
    fn new(shared: StackShared, base: String) -> AmiEphemeralBlockDeviceElRef {
        AmiEphemeralBlockDeviceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AmiEphemeralBlockDeviceElRef {
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
pub struct AmiTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl AmiTimeoutsEl {
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

impl ToListMappable for AmiTimeoutsEl {
    type O = BlockAssignable<AmiTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAmiTimeoutsEl {}

impl BuildAmiTimeoutsEl {
    pub fn build(self) -> AmiTimeoutsEl {
        AmiTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct AmiTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AmiTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> AmiTimeoutsElRef {
        AmiTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AmiTimeoutsElRef {
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

#[derive(Serialize, Default)]
struct AmiDynamic {
    ebs_block_device: Option<DynamicBlock<AmiEbsBlockDeviceEl>>,
    ephemeral_block_device: Option<DynamicBlock<AmiEphemeralBlockDeviceEl>>,
}
