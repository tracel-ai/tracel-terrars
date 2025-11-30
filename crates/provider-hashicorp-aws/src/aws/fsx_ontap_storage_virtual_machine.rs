use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct FsxOntapStorageVirtualMachineData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    file_system_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    root_volume_security_style: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    svm_admin_password: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    active_directory_configuration:
        Option<Vec<FsxOntapStorageVirtualMachineActiveDirectoryConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<FsxOntapStorageVirtualMachineTimeoutsEl>,
    dynamic: FsxOntapStorageVirtualMachineDynamic,
}

struct FsxOntapStorageVirtualMachine_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<FsxOntapStorageVirtualMachineData>,
}

#[derive(Clone)]
pub struct FsxOntapStorageVirtualMachine(Rc<FsxOntapStorageVirtualMachine_>);

impl FsxOntapStorageVirtualMachine {
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

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `root_volume_security_style`.\n"]
    pub fn set_root_volume_security_style(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().root_volume_security_style = Some(v.into());
        self
    }

    #[doc = "Set the field `svm_admin_password`.\n"]
    pub fn set_svm_admin_password(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().svm_admin_password = Some(v.into());
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

    #[doc = "Set the field `active_directory_configuration`.\n"]
    pub fn set_active_directory_configuration(
        self,
        v: impl Into<BlockAssignable<FsxOntapStorageVirtualMachineActiveDirectoryConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().active_directory_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .active_directory_configuration = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<FsxOntapStorageVirtualMachineTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoints` after provisioning.\n"]
    pub fn endpoints(&self) -> ListRef<FsxOntapStorageVirtualMachineEndpointsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.endpoints", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `file_system_id` after provisioning.\n"]
    pub fn file_system_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.file_system_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `root_volume_security_style` after provisioning.\n"]
    pub fn root_volume_security_style(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.root_volume_security_style", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `subtype` after provisioning.\n"]
    pub fn subtype(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subtype", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `svm_admin_password` after provisioning.\n"]
    pub fn svm_admin_password(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.svm_admin_password", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `uuid` after provisioning.\n"]
    pub fn uuid(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.uuid", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `active_directory_configuration` after provisioning.\n"]
    pub fn active_directory_configuration(
        &self,
    ) -> ListRef<FsxOntapStorageVirtualMachineActiveDirectoryConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.active_directory_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FsxOntapStorageVirtualMachineTimeoutsElRef {
        FsxOntapStorageVirtualMachineTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for FsxOntapStorageVirtualMachine {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for FsxOntapStorageVirtualMachine {}

impl ToListMappable for FsxOntapStorageVirtualMachine {
    type O = ListRef<FsxOntapStorageVirtualMachineRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for FsxOntapStorageVirtualMachine_ {
    fn extract_resource_type(&self) -> String {
        "aws_fsx_ontap_storage_virtual_machine".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildFsxOntapStorageVirtualMachine {
    pub tf_id: String,
    #[doc = ""]
    pub file_system_id: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildFsxOntapStorageVirtualMachine {
    pub fn build(self, stack: &mut Stack) -> FsxOntapStorageVirtualMachine {
        let out = FsxOntapStorageVirtualMachine(Rc::new(FsxOntapStorageVirtualMachine_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(FsxOntapStorageVirtualMachineData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                file_system_id: self.file_system_id,
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                root_volume_security_style: core::default::Default::default(),
                svm_admin_password: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                active_directory_configuration: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct FsxOntapStorageVirtualMachineRef {
    shared: StackShared,
    base: String,
}

impl Ref for FsxOntapStorageVirtualMachineRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl FsxOntapStorageVirtualMachineRef {
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

    #[doc = "Get a reference to the value of field `endpoints` after provisioning.\n"]
    pub fn endpoints(&self) -> ListRef<FsxOntapStorageVirtualMachineEndpointsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.endpoints", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `file_system_id` after provisioning.\n"]
    pub fn file_system_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.file_system_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `root_volume_security_style` after provisioning.\n"]
    pub fn root_volume_security_style(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.root_volume_security_style", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `subtype` after provisioning.\n"]
    pub fn subtype(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subtype", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `svm_admin_password` after provisioning.\n"]
    pub fn svm_admin_password(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.svm_admin_password", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `uuid` after provisioning.\n"]
    pub fn uuid(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.uuid", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `active_directory_configuration` after provisioning.\n"]
    pub fn active_directory_configuration(
        &self,
    ) -> ListRef<FsxOntapStorageVirtualMachineActiveDirectoryConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.active_directory_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FsxOntapStorageVirtualMachineTimeoutsElRef {
        FsxOntapStorageVirtualMachineTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct FsxOntapStorageVirtualMachineEndpointsElIscsiEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dns_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_addresses: Option<SetField<PrimField<String>>>,
}

impl FsxOntapStorageVirtualMachineEndpointsElIscsiEl {
    #[doc = "Set the field `dns_name`.\n"]
    pub fn set_dns_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dns_name = Some(v.into());
        self
    }

    #[doc = "Set the field `ip_addresses`.\n"]
    pub fn set_ip_addresses(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.ip_addresses = Some(v.into());
        self
    }
}

impl ToListMappable for FsxOntapStorageVirtualMachineEndpointsElIscsiEl {
    type O = BlockAssignable<FsxOntapStorageVirtualMachineEndpointsElIscsiEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxOntapStorageVirtualMachineEndpointsElIscsiEl {}

impl BuildFsxOntapStorageVirtualMachineEndpointsElIscsiEl {
    pub fn build(self) -> FsxOntapStorageVirtualMachineEndpointsElIscsiEl {
        FsxOntapStorageVirtualMachineEndpointsElIscsiEl {
            dns_name: core::default::Default::default(),
            ip_addresses: core::default::Default::default(),
        }
    }
}

pub struct FsxOntapStorageVirtualMachineEndpointsElIscsiElRef {
    shared: StackShared,
    base: String,
}

impl Ref for FsxOntapStorageVirtualMachineEndpointsElIscsiElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> FsxOntapStorageVirtualMachineEndpointsElIscsiElRef {
        FsxOntapStorageVirtualMachineEndpointsElIscsiElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxOntapStorageVirtualMachineEndpointsElIscsiElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dns_name` after provisioning.\n"]
    pub fn dns_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dns_name", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_addresses` after provisioning.\n"]
    pub fn ip_addresses(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.ip_addresses", self.base))
    }
}

#[derive(Serialize)]
pub struct FsxOntapStorageVirtualMachineEndpointsElManagementEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dns_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_addresses: Option<SetField<PrimField<String>>>,
}

impl FsxOntapStorageVirtualMachineEndpointsElManagementEl {
    #[doc = "Set the field `dns_name`.\n"]
    pub fn set_dns_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dns_name = Some(v.into());
        self
    }

    #[doc = "Set the field `ip_addresses`.\n"]
    pub fn set_ip_addresses(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.ip_addresses = Some(v.into());
        self
    }
}

impl ToListMappable for FsxOntapStorageVirtualMachineEndpointsElManagementEl {
    type O = BlockAssignable<FsxOntapStorageVirtualMachineEndpointsElManagementEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxOntapStorageVirtualMachineEndpointsElManagementEl {}

impl BuildFsxOntapStorageVirtualMachineEndpointsElManagementEl {
    pub fn build(self) -> FsxOntapStorageVirtualMachineEndpointsElManagementEl {
        FsxOntapStorageVirtualMachineEndpointsElManagementEl {
            dns_name: core::default::Default::default(),
            ip_addresses: core::default::Default::default(),
        }
    }
}

pub struct FsxOntapStorageVirtualMachineEndpointsElManagementElRef {
    shared: StackShared,
    base: String,
}

impl Ref for FsxOntapStorageVirtualMachineEndpointsElManagementElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> FsxOntapStorageVirtualMachineEndpointsElManagementElRef {
        FsxOntapStorageVirtualMachineEndpointsElManagementElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxOntapStorageVirtualMachineEndpointsElManagementElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dns_name` after provisioning.\n"]
    pub fn dns_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dns_name", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_addresses` after provisioning.\n"]
    pub fn ip_addresses(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.ip_addresses", self.base))
    }
}

#[derive(Serialize)]
pub struct FsxOntapStorageVirtualMachineEndpointsElNfsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dns_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_addresses: Option<SetField<PrimField<String>>>,
}

impl FsxOntapStorageVirtualMachineEndpointsElNfsEl {
    #[doc = "Set the field `dns_name`.\n"]
    pub fn set_dns_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dns_name = Some(v.into());
        self
    }

    #[doc = "Set the field `ip_addresses`.\n"]
    pub fn set_ip_addresses(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.ip_addresses = Some(v.into());
        self
    }
}

impl ToListMappable for FsxOntapStorageVirtualMachineEndpointsElNfsEl {
    type O = BlockAssignable<FsxOntapStorageVirtualMachineEndpointsElNfsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxOntapStorageVirtualMachineEndpointsElNfsEl {}

impl BuildFsxOntapStorageVirtualMachineEndpointsElNfsEl {
    pub fn build(self) -> FsxOntapStorageVirtualMachineEndpointsElNfsEl {
        FsxOntapStorageVirtualMachineEndpointsElNfsEl {
            dns_name: core::default::Default::default(),
            ip_addresses: core::default::Default::default(),
        }
    }
}

pub struct FsxOntapStorageVirtualMachineEndpointsElNfsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for FsxOntapStorageVirtualMachineEndpointsElNfsElRef {
    fn new(shared: StackShared, base: String) -> FsxOntapStorageVirtualMachineEndpointsElNfsElRef {
        FsxOntapStorageVirtualMachineEndpointsElNfsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxOntapStorageVirtualMachineEndpointsElNfsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dns_name` after provisioning.\n"]
    pub fn dns_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dns_name", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_addresses` after provisioning.\n"]
    pub fn ip_addresses(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.ip_addresses", self.base))
    }
}

#[derive(Serialize)]
pub struct FsxOntapStorageVirtualMachineEndpointsElSmbEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dns_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_addresses: Option<SetField<PrimField<String>>>,
}

impl FsxOntapStorageVirtualMachineEndpointsElSmbEl {
    #[doc = "Set the field `dns_name`.\n"]
    pub fn set_dns_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dns_name = Some(v.into());
        self
    }

    #[doc = "Set the field `ip_addresses`.\n"]
    pub fn set_ip_addresses(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.ip_addresses = Some(v.into());
        self
    }
}

impl ToListMappable for FsxOntapStorageVirtualMachineEndpointsElSmbEl {
    type O = BlockAssignable<FsxOntapStorageVirtualMachineEndpointsElSmbEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxOntapStorageVirtualMachineEndpointsElSmbEl {}

impl BuildFsxOntapStorageVirtualMachineEndpointsElSmbEl {
    pub fn build(self) -> FsxOntapStorageVirtualMachineEndpointsElSmbEl {
        FsxOntapStorageVirtualMachineEndpointsElSmbEl {
            dns_name: core::default::Default::default(),
            ip_addresses: core::default::Default::default(),
        }
    }
}

pub struct FsxOntapStorageVirtualMachineEndpointsElSmbElRef {
    shared: StackShared,
    base: String,
}

impl Ref for FsxOntapStorageVirtualMachineEndpointsElSmbElRef {
    fn new(shared: StackShared, base: String) -> FsxOntapStorageVirtualMachineEndpointsElSmbElRef {
        FsxOntapStorageVirtualMachineEndpointsElSmbElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxOntapStorageVirtualMachineEndpointsElSmbElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dns_name` after provisioning.\n"]
    pub fn dns_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dns_name", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_addresses` after provisioning.\n"]
    pub fn ip_addresses(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.ip_addresses", self.base))
    }
}

#[derive(Serialize)]
pub struct FsxOntapStorageVirtualMachineEndpointsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    iscsi: Option<ListField<FsxOntapStorageVirtualMachineEndpointsElIscsiEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    management: Option<ListField<FsxOntapStorageVirtualMachineEndpointsElManagementEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nfs: Option<ListField<FsxOntapStorageVirtualMachineEndpointsElNfsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    smb: Option<ListField<FsxOntapStorageVirtualMachineEndpointsElSmbEl>>,
}

impl FsxOntapStorageVirtualMachineEndpointsEl {
    #[doc = "Set the field `iscsi`.\n"]
    pub fn set_iscsi(
        mut self,
        v: impl Into<ListField<FsxOntapStorageVirtualMachineEndpointsElIscsiEl>>,
    ) -> Self {
        self.iscsi = Some(v.into());
        self
    }

    #[doc = "Set the field `management`.\n"]
    pub fn set_management(
        mut self,
        v: impl Into<ListField<FsxOntapStorageVirtualMachineEndpointsElManagementEl>>,
    ) -> Self {
        self.management = Some(v.into());
        self
    }

    #[doc = "Set the field `nfs`.\n"]
    pub fn set_nfs(
        mut self,
        v: impl Into<ListField<FsxOntapStorageVirtualMachineEndpointsElNfsEl>>,
    ) -> Self {
        self.nfs = Some(v.into());
        self
    }

    #[doc = "Set the field `smb`.\n"]
    pub fn set_smb(
        mut self,
        v: impl Into<ListField<FsxOntapStorageVirtualMachineEndpointsElSmbEl>>,
    ) -> Self {
        self.smb = Some(v.into());
        self
    }
}

impl ToListMappable for FsxOntapStorageVirtualMachineEndpointsEl {
    type O = BlockAssignable<FsxOntapStorageVirtualMachineEndpointsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxOntapStorageVirtualMachineEndpointsEl {}

impl BuildFsxOntapStorageVirtualMachineEndpointsEl {
    pub fn build(self) -> FsxOntapStorageVirtualMachineEndpointsEl {
        FsxOntapStorageVirtualMachineEndpointsEl {
            iscsi: core::default::Default::default(),
            management: core::default::Default::default(),
            nfs: core::default::Default::default(),
            smb: core::default::Default::default(),
        }
    }
}

pub struct FsxOntapStorageVirtualMachineEndpointsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for FsxOntapStorageVirtualMachineEndpointsElRef {
    fn new(shared: StackShared, base: String) -> FsxOntapStorageVirtualMachineEndpointsElRef {
        FsxOntapStorageVirtualMachineEndpointsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxOntapStorageVirtualMachineEndpointsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `iscsi` after provisioning.\n"]
    pub fn iscsi(&self) -> ListRef<FsxOntapStorageVirtualMachineEndpointsElIscsiElRef> {
        ListRef::new(self.shared().clone(), format!("{}.iscsi", self.base))
    }

    #[doc = "Get a reference to the value of field `management` after provisioning.\n"]
    pub fn management(&self) -> ListRef<FsxOntapStorageVirtualMachineEndpointsElManagementElRef> {
        ListRef::new(self.shared().clone(), format!("{}.management", self.base))
    }

    #[doc = "Get a reference to the value of field `nfs` after provisioning.\n"]
    pub fn nfs(&self) -> ListRef<FsxOntapStorageVirtualMachineEndpointsElNfsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.nfs", self.base))
    }

    #[doc = "Get a reference to the value of field `smb` after provisioning.\n"]
    pub fn smb(&self) -> ListRef<FsxOntapStorageVirtualMachineEndpointsElSmbElRef> {
        ListRef::new(self.shared().clone(), format!("{}.smb", self.base))
    }
}

#[derive(Serialize)]
pub struct FsxOntapStorageVirtualMachineActiveDirectoryConfigurationElSelfManagedActiveDirectoryConfigurationEl
{
    dns_ips: SetField<PrimField<String>>,
    domain_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file_system_administrators_group: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    organizational_unit_distinguished_name: Option<PrimField<String>>,
    password: PrimField<String>,
    username: PrimField<String>,
}

impl FsxOntapStorageVirtualMachineActiveDirectoryConfigurationElSelfManagedActiveDirectoryConfigurationEl {
    #[doc = "Set the field `file_system_administrators_group`.\n"]
    pub fn set_file_system_administrators_group(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.file_system_administrators_group = Some(v.into());
        self
    }

    #[doc = "Set the field `organizational_unit_distinguished_name`.\n"]
    pub fn set_organizational_unit_distinguished_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.organizational_unit_distinguished_name = Some(v.into());
        self
    }
}

impl ToListMappable for FsxOntapStorageVirtualMachineActiveDirectoryConfigurationElSelfManagedActiveDirectoryConfigurationEl {
    type O =
        BlockAssignable<
            FsxOntapStorageVirtualMachineActiveDirectoryConfigurationElSelfManagedActiveDirectoryConfigurationEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxOntapStorageVirtualMachineActiveDirectoryConfigurationElSelfManagedActiveDirectoryConfigurationEl
{
    #[doc = ""]
    pub dns_ips: SetField<PrimField<String>>,
    #[doc = ""]
    pub domain_name: PrimField<String>,
    #[doc = ""]
    pub password: PrimField<String>,
    #[doc = ""]
    pub username: PrimField<String>,
}

impl BuildFsxOntapStorageVirtualMachineActiveDirectoryConfigurationElSelfManagedActiveDirectoryConfigurationEl {
    pub fn build(
        self,
    ) -> FsxOntapStorageVirtualMachineActiveDirectoryConfigurationElSelfManagedActiveDirectoryConfigurationEl {
        FsxOntapStorageVirtualMachineActiveDirectoryConfigurationElSelfManagedActiveDirectoryConfigurationEl {
            dns_ips: self.dns_ips,
            domain_name: self.domain_name,
            file_system_administrators_group: core::default::Default::default(),
            organizational_unit_distinguished_name: core::default::Default::default(),
            password: self.password,
            username: self.username,
        }
    }
}

pub struct FsxOntapStorageVirtualMachineActiveDirectoryConfigurationElSelfManagedActiveDirectoryConfigurationElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for FsxOntapStorageVirtualMachineActiveDirectoryConfigurationElSelfManagedActiveDirectoryConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> FsxOntapStorageVirtualMachineActiveDirectoryConfigurationElSelfManagedActiveDirectoryConfigurationElRef {
        FsxOntapStorageVirtualMachineActiveDirectoryConfigurationElSelfManagedActiveDirectoryConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxOntapStorageVirtualMachineActiveDirectoryConfigurationElSelfManagedActiveDirectoryConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dns_ips` after provisioning.\n"]
    pub fn dns_ips(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.dns_ips", self.base))
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.base))
    }

    #[doc = "Get a reference to the value of field `file_system_administrators_group` after provisioning.\n"]
    pub fn file_system_administrators_group(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.file_system_administrators_group", self.base))
    }

    #[doc = "Get a reference to the value of field `organizational_unit_distinguished_name` after provisioning.\n"]
    pub fn organizational_unit_distinguished_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.organizational_unit_distinguished_name", self.base))
    }

    #[doc = "Get a reference to the value of field `password` after provisioning.\n"]
    pub fn password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }

    #[doc = "Get a reference to the value of field `username` after provisioning.\n"]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}

#[derive(Serialize, Default)]
struct FsxOntapStorageVirtualMachineActiveDirectoryConfigurationElDynamic {
    self_managed_active_directory_configuration: Option<
        DynamicBlock<
            FsxOntapStorageVirtualMachineActiveDirectoryConfigurationElSelfManagedActiveDirectoryConfigurationEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct FsxOntapStorageVirtualMachineActiveDirectoryConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    netbios_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    self_managed_active_directory_configuration: Option<
        Vec<FsxOntapStorageVirtualMachineActiveDirectoryConfigurationElSelfManagedActiveDirectoryConfigurationEl>,
    >,
    dynamic: FsxOntapStorageVirtualMachineActiveDirectoryConfigurationElDynamic,
}

impl FsxOntapStorageVirtualMachineActiveDirectoryConfigurationEl {
    #[doc = "Set the field `netbios_name`.\n"]
    pub fn set_netbios_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.netbios_name = Some(v.into());
        self
    }

    #[doc = "Set the field `self_managed_active_directory_configuration`.\n"]
    pub fn set_self_managed_active_directory_configuration(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            FsxOntapStorageVirtualMachineActiveDirectoryConfigurationElSelfManagedActiveDirectoryConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.self_managed_active_directory_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.self_managed_active_directory_configuration = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for FsxOntapStorageVirtualMachineActiveDirectoryConfigurationEl {
    type O = BlockAssignable<FsxOntapStorageVirtualMachineActiveDirectoryConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxOntapStorageVirtualMachineActiveDirectoryConfigurationEl {}

impl BuildFsxOntapStorageVirtualMachineActiveDirectoryConfigurationEl {
    pub fn build(self) -> FsxOntapStorageVirtualMachineActiveDirectoryConfigurationEl {
        FsxOntapStorageVirtualMachineActiveDirectoryConfigurationEl {
            netbios_name: core::default::Default::default(),
            self_managed_active_directory_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct FsxOntapStorageVirtualMachineActiveDirectoryConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for FsxOntapStorageVirtualMachineActiveDirectoryConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> FsxOntapStorageVirtualMachineActiveDirectoryConfigurationElRef {
        FsxOntapStorageVirtualMachineActiveDirectoryConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxOntapStorageVirtualMachineActiveDirectoryConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `netbios_name` after provisioning.\n"]
    pub fn netbios_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.netbios_name", self.base))
    }

    #[doc = "Get a reference to the value of field `self_managed_active_directory_configuration` after provisioning.\n"]
    pub fn self_managed_active_directory_configuration(
        &self,
    ) -> ListRef<
        FsxOntapStorageVirtualMachineActiveDirectoryConfigurationElSelfManagedActiveDirectoryConfigurationElRef,
    >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.self_managed_active_directory_configuration", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct FsxOntapStorageVirtualMachineTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl FsxOntapStorageVirtualMachineTimeoutsEl {
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

impl ToListMappable for FsxOntapStorageVirtualMachineTimeoutsEl {
    type O = BlockAssignable<FsxOntapStorageVirtualMachineTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxOntapStorageVirtualMachineTimeoutsEl {}

impl BuildFsxOntapStorageVirtualMachineTimeoutsEl {
    pub fn build(self) -> FsxOntapStorageVirtualMachineTimeoutsEl {
        FsxOntapStorageVirtualMachineTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct FsxOntapStorageVirtualMachineTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for FsxOntapStorageVirtualMachineTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> FsxOntapStorageVirtualMachineTimeoutsElRef {
        FsxOntapStorageVirtualMachineTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxOntapStorageVirtualMachineTimeoutsElRef {
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
struct FsxOntapStorageVirtualMachineDynamic {
    active_directory_configuration:
        Option<DynamicBlock<FsxOntapStorageVirtualMachineActiveDirectoryConfigurationEl>>,
}
