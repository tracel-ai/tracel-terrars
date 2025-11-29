use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataFsxOntapStorageVirtualMachineData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<Vec<DataFsxOntapStorageVirtualMachineFilterEl>>,
    dynamic: DataFsxOntapStorageVirtualMachineDynamic,
}

struct DataFsxOntapStorageVirtualMachine_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataFsxOntapStorageVirtualMachineData>,
}

#[derive(Clone)]
pub struct DataFsxOntapStorageVirtualMachine(Rc<DataFsxOntapStorageVirtualMachine_>);

impl DataFsxOntapStorageVirtualMachine {
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

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(self, v: impl Into<BlockAssignable<DataFsxOntapStorageVirtualMachineFilterEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().filter = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.filter = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `active_directory_configuration` after provisioning.\n"]
    pub fn active_directory_configuration(
        &self,
    ) -> ListRef<DataFsxOntapStorageVirtualMachineActiveDirectoryConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.active_directory_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `creation_time` after provisioning.\n"]
    pub fn creation_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.creation_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoints` after provisioning.\n"]
    pub fn endpoints(&self) -> ListRef<DataFsxOntapStorageVirtualMachineEndpointsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.endpoints", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `file_system_id` after provisioning.\n"]
    pub fn file_system_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.file_system_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lifecycle_status` after provisioning.\n"]
    pub fn lifecycle_status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.lifecycle_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lifecycle_transition_reason` after provisioning.\n"]
    pub fn lifecycle_transition_reason(&self) -> SetRef<DataFsxOntapStorageVirtualMachineLifecycleTransitionReasonElRef> {
        SetRef::new(self.shared().clone(), format!("{}.lifecycle_transition_reason", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subtype` after provisioning.\n"]
    pub fn subtype(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subtype", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `uuid` after provisioning.\n"]
    pub fn uuid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uuid", self.extract_ref()))
    }
}

impl Referable for DataFsxOntapStorageVirtualMachine {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataFsxOntapStorageVirtualMachine { }

impl ToListMappable for DataFsxOntapStorageVirtualMachine {
    type O = ListRef<DataFsxOntapStorageVirtualMachineRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataFsxOntapStorageVirtualMachine_ {
    fn extract_datasource_type(&self) -> String {
        "aws_fsx_ontap_storage_virtual_machine".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataFsxOntapStorageVirtualMachine {
    pub tf_id: String,
}

impl BuildDataFsxOntapStorageVirtualMachine {
    pub fn build(self, stack: &mut Stack) -> DataFsxOntapStorageVirtualMachine {
        let out = DataFsxOntapStorageVirtualMachine(Rc::new(DataFsxOntapStorageVirtualMachine_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataFsxOntapStorageVirtualMachineData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                filter: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataFsxOntapStorageVirtualMachineRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataFsxOntapStorageVirtualMachineRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataFsxOntapStorageVirtualMachineRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `active_directory_configuration` after provisioning.\n"]
    pub fn active_directory_configuration(
        &self,
    ) -> ListRef<DataFsxOntapStorageVirtualMachineActiveDirectoryConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.active_directory_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `creation_time` after provisioning.\n"]
    pub fn creation_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.creation_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoints` after provisioning.\n"]
    pub fn endpoints(&self) -> ListRef<DataFsxOntapStorageVirtualMachineEndpointsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.endpoints", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `file_system_id` after provisioning.\n"]
    pub fn file_system_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.file_system_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lifecycle_status` after provisioning.\n"]
    pub fn lifecycle_status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.lifecycle_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lifecycle_transition_reason` after provisioning.\n"]
    pub fn lifecycle_transition_reason(&self) -> SetRef<DataFsxOntapStorageVirtualMachineLifecycleTransitionReasonElRef> {
        SetRef::new(self.shared().clone(), format!("{}.lifecycle_transition_reason", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subtype` after provisioning.\n"]
    pub fn subtype(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subtype", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `uuid` after provisioning.\n"]
    pub fn uuid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uuid", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataFsxOntapStorageVirtualMachineActiveDirectoryConfigurationElSelfManagedActiveDirectoryConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dns_ips: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    domain_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file_system_administrators_group: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    organizational_unit_distinguished_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    username: Option<PrimField<String>>,
}

impl DataFsxOntapStorageVirtualMachineActiveDirectoryConfigurationElSelfManagedActiveDirectoryConfigurationEl {
    #[doc = "Set the field `dns_ips`.\n"]
    pub fn set_dns_ips(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.dns_ips = Some(v.into());
        self
    }

    #[doc = "Set the field `domain_name`.\n"]
    pub fn set_domain_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.domain_name = Some(v.into());
        self
    }

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

    #[doc = "Set the field `username`.\n"]
    pub fn set_username(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.username = Some(v.into());
        self
    }
}

impl ToListMappable for DataFsxOntapStorageVirtualMachineActiveDirectoryConfigurationElSelfManagedActiveDirectoryConfigurationEl {
    type O =
        BlockAssignable<
            DataFsxOntapStorageVirtualMachineActiveDirectoryConfigurationElSelfManagedActiveDirectoryConfigurationEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataFsxOntapStorageVirtualMachineActiveDirectoryConfigurationElSelfManagedActiveDirectoryConfigurationEl {}

impl BuildDataFsxOntapStorageVirtualMachineActiveDirectoryConfigurationElSelfManagedActiveDirectoryConfigurationEl {
    pub fn build(
        self,
    ) -> DataFsxOntapStorageVirtualMachineActiveDirectoryConfigurationElSelfManagedActiveDirectoryConfigurationEl {
        DataFsxOntapStorageVirtualMachineActiveDirectoryConfigurationElSelfManagedActiveDirectoryConfigurationEl {
            dns_ips: core::default::Default::default(),
            domain_name: core::default::Default::default(),
            file_system_administrators_group: core::default::Default::default(),
            organizational_unit_distinguished_name: core::default::Default::default(),
            username: core::default::Default::default(),
        }
    }
}

pub struct DataFsxOntapStorageVirtualMachineActiveDirectoryConfigurationElSelfManagedActiveDirectoryConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataFsxOntapStorageVirtualMachineActiveDirectoryConfigurationElSelfManagedActiveDirectoryConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataFsxOntapStorageVirtualMachineActiveDirectoryConfigurationElSelfManagedActiveDirectoryConfigurationElRef {
        DataFsxOntapStorageVirtualMachineActiveDirectoryConfigurationElSelfManagedActiveDirectoryConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataFsxOntapStorageVirtualMachineActiveDirectoryConfigurationElSelfManagedActiveDirectoryConfigurationElRef {
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

    #[doc = "Get a reference to the value of field `username` after provisioning.\n"]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}

#[derive(Serialize)]
pub struct DataFsxOntapStorageVirtualMachineActiveDirectoryConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    netbios_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    self_managed_active_directory_configuration: Option<
        ListField<
            DataFsxOntapStorageVirtualMachineActiveDirectoryConfigurationElSelfManagedActiveDirectoryConfigurationEl,
        >,
    >,
}

impl DataFsxOntapStorageVirtualMachineActiveDirectoryConfigurationEl {
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
                        ListField<
                            DataFsxOntapStorageVirtualMachineActiveDirectoryConfigurationElSelfManagedActiveDirectoryConfigurationEl,
                        >,
                    >,
    ) -> Self {
        self.self_managed_active_directory_configuration = Some(v.into());
        self
    }
}

impl ToListMappable for DataFsxOntapStorageVirtualMachineActiveDirectoryConfigurationEl {
    type O = BlockAssignable<DataFsxOntapStorageVirtualMachineActiveDirectoryConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataFsxOntapStorageVirtualMachineActiveDirectoryConfigurationEl {}

impl BuildDataFsxOntapStorageVirtualMachineActiveDirectoryConfigurationEl {
    pub fn build(self) -> DataFsxOntapStorageVirtualMachineActiveDirectoryConfigurationEl {
        DataFsxOntapStorageVirtualMachineActiveDirectoryConfigurationEl {
            netbios_name: core::default::Default::default(),
            self_managed_active_directory_configuration: core::default::Default::default(),
        }
    }
}

pub struct DataFsxOntapStorageVirtualMachineActiveDirectoryConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataFsxOntapStorageVirtualMachineActiveDirectoryConfigurationElRef {
    fn new(shared: StackShared, base: String) -> DataFsxOntapStorageVirtualMachineActiveDirectoryConfigurationElRef {
        DataFsxOntapStorageVirtualMachineActiveDirectoryConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataFsxOntapStorageVirtualMachineActiveDirectoryConfigurationElRef {
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
        DataFsxOntapStorageVirtualMachineActiveDirectoryConfigurationElSelfManagedActiveDirectoryConfigurationElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.self_managed_active_directory_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct DataFsxOntapStorageVirtualMachineEndpointsElIscsiEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dns_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_addresses: Option<SetField<PrimField<String>>>,
}

impl DataFsxOntapStorageVirtualMachineEndpointsElIscsiEl {
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

impl ToListMappable for DataFsxOntapStorageVirtualMachineEndpointsElIscsiEl {
    type O = BlockAssignable<DataFsxOntapStorageVirtualMachineEndpointsElIscsiEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataFsxOntapStorageVirtualMachineEndpointsElIscsiEl {}

impl BuildDataFsxOntapStorageVirtualMachineEndpointsElIscsiEl {
    pub fn build(self) -> DataFsxOntapStorageVirtualMachineEndpointsElIscsiEl {
        DataFsxOntapStorageVirtualMachineEndpointsElIscsiEl {
            dns_name: core::default::Default::default(),
            ip_addresses: core::default::Default::default(),
        }
    }
}

pub struct DataFsxOntapStorageVirtualMachineEndpointsElIscsiElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataFsxOntapStorageVirtualMachineEndpointsElIscsiElRef {
    fn new(shared: StackShared, base: String) -> DataFsxOntapStorageVirtualMachineEndpointsElIscsiElRef {
        DataFsxOntapStorageVirtualMachineEndpointsElIscsiElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataFsxOntapStorageVirtualMachineEndpointsElIscsiElRef {
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
pub struct DataFsxOntapStorageVirtualMachineEndpointsElManagementEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dns_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_addresses: Option<SetField<PrimField<String>>>,
}

impl DataFsxOntapStorageVirtualMachineEndpointsElManagementEl {
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

impl ToListMappable for DataFsxOntapStorageVirtualMachineEndpointsElManagementEl {
    type O = BlockAssignable<DataFsxOntapStorageVirtualMachineEndpointsElManagementEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataFsxOntapStorageVirtualMachineEndpointsElManagementEl {}

impl BuildDataFsxOntapStorageVirtualMachineEndpointsElManagementEl {
    pub fn build(self) -> DataFsxOntapStorageVirtualMachineEndpointsElManagementEl {
        DataFsxOntapStorageVirtualMachineEndpointsElManagementEl {
            dns_name: core::default::Default::default(),
            ip_addresses: core::default::Default::default(),
        }
    }
}

pub struct DataFsxOntapStorageVirtualMachineEndpointsElManagementElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataFsxOntapStorageVirtualMachineEndpointsElManagementElRef {
    fn new(shared: StackShared, base: String) -> DataFsxOntapStorageVirtualMachineEndpointsElManagementElRef {
        DataFsxOntapStorageVirtualMachineEndpointsElManagementElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataFsxOntapStorageVirtualMachineEndpointsElManagementElRef {
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
pub struct DataFsxOntapStorageVirtualMachineEndpointsElNfsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dns_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_addresses: Option<SetField<PrimField<String>>>,
}

impl DataFsxOntapStorageVirtualMachineEndpointsElNfsEl {
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

impl ToListMappable for DataFsxOntapStorageVirtualMachineEndpointsElNfsEl {
    type O = BlockAssignable<DataFsxOntapStorageVirtualMachineEndpointsElNfsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataFsxOntapStorageVirtualMachineEndpointsElNfsEl {}

impl BuildDataFsxOntapStorageVirtualMachineEndpointsElNfsEl {
    pub fn build(self) -> DataFsxOntapStorageVirtualMachineEndpointsElNfsEl {
        DataFsxOntapStorageVirtualMachineEndpointsElNfsEl {
            dns_name: core::default::Default::default(),
            ip_addresses: core::default::Default::default(),
        }
    }
}

pub struct DataFsxOntapStorageVirtualMachineEndpointsElNfsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataFsxOntapStorageVirtualMachineEndpointsElNfsElRef {
    fn new(shared: StackShared, base: String) -> DataFsxOntapStorageVirtualMachineEndpointsElNfsElRef {
        DataFsxOntapStorageVirtualMachineEndpointsElNfsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataFsxOntapStorageVirtualMachineEndpointsElNfsElRef {
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
pub struct DataFsxOntapStorageVirtualMachineEndpointsElSmbEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dns_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_addresses: Option<SetField<PrimField<String>>>,
}

impl DataFsxOntapStorageVirtualMachineEndpointsElSmbEl {
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

impl ToListMappable for DataFsxOntapStorageVirtualMachineEndpointsElSmbEl {
    type O = BlockAssignable<DataFsxOntapStorageVirtualMachineEndpointsElSmbEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataFsxOntapStorageVirtualMachineEndpointsElSmbEl {}

impl BuildDataFsxOntapStorageVirtualMachineEndpointsElSmbEl {
    pub fn build(self) -> DataFsxOntapStorageVirtualMachineEndpointsElSmbEl {
        DataFsxOntapStorageVirtualMachineEndpointsElSmbEl {
            dns_name: core::default::Default::default(),
            ip_addresses: core::default::Default::default(),
        }
    }
}

pub struct DataFsxOntapStorageVirtualMachineEndpointsElSmbElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataFsxOntapStorageVirtualMachineEndpointsElSmbElRef {
    fn new(shared: StackShared, base: String) -> DataFsxOntapStorageVirtualMachineEndpointsElSmbElRef {
        DataFsxOntapStorageVirtualMachineEndpointsElSmbElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataFsxOntapStorageVirtualMachineEndpointsElSmbElRef {
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
pub struct DataFsxOntapStorageVirtualMachineEndpointsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    iscsi: Option<ListField<DataFsxOntapStorageVirtualMachineEndpointsElIscsiEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    management: Option<ListField<DataFsxOntapStorageVirtualMachineEndpointsElManagementEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nfs: Option<ListField<DataFsxOntapStorageVirtualMachineEndpointsElNfsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    smb: Option<ListField<DataFsxOntapStorageVirtualMachineEndpointsElSmbEl>>,
}

impl DataFsxOntapStorageVirtualMachineEndpointsEl {
    #[doc = "Set the field `iscsi`.\n"]
    pub fn set_iscsi(mut self, v: impl Into<ListField<DataFsxOntapStorageVirtualMachineEndpointsElIscsiEl>>) -> Self {
        self.iscsi = Some(v.into());
        self
    }

    #[doc = "Set the field `management`.\n"]
    pub fn set_management(
        mut self,
        v: impl Into<ListField<DataFsxOntapStorageVirtualMachineEndpointsElManagementEl>>,
    ) -> Self {
        self.management = Some(v.into());
        self
    }

    #[doc = "Set the field `nfs`.\n"]
    pub fn set_nfs(mut self, v: impl Into<ListField<DataFsxOntapStorageVirtualMachineEndpointsElNfsEl>>) -> Self {
        self.nfs = Some(v.into());
        self
    }

    #[doc = "Set the field `smb`.\n"]
    pub fn set_smb(mut self, v: impl Into<ListField<DataFsxOntapStorageVirtualMachineEndpointsElSmbEl>>) -> Self {
        self.smb = Some(v.into());
        self
    }
}

impl ToListMappable for DataFsxOntapStorageVirtualMachineEndpointsEl {
    type O = BlockAssignable<DataFsxOntapStorageVirtualMachineEndpointsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataFsxOntapStorageVirtualMachineEndpointsEl {}

impl BuildDataFsxOntapStorageVirtualMachineEndpointsEl {
    pub fn build(self) -> DataFsxOntapStorageVirtualMachineEndpointsEl {
        DataFsxOntapStorageVirtualMachineEndpointsEl {
            iscsi: core::default::Default::default(),
            management: core::default::Default::default(),
            nfs: core::default::Default::default(),
            smb: core::default::Default::default(),
        }
    }
}

pub struct DataFsxOntapStorageVirtualMachineEndpointsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataFsxOntapStorageVirtualMachineEndpointsElRef {
    fn new(shared: StackShared, base: String) -> DataFsxOntapStorageVirtualMachineEndpointsElRef {
        DataFsxOntapStorageVirtualMachineEndpointsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataFsxOntapStorageVirtualMachineEndpointsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `iscsi` after provisioning.\n"]
    pub fn iscsi(&self) -> ListRef<DataFsxOntapStorageVirtualMachineEndpointsElIscsiElRef> {
        ListRef::new(self.shared().clone(), format!("{}.iscsi", self.base))
    }

    #[doc = "Get a reference to the value of field `management` after provisioning.\n"]
    pub fn management(&self) -> ListRef<DataFsxOntapStorageVirtualMachineEndpointsElManagementElRef> {
        ListRef::new(self.shared().clone(), format!("{}.management", self.base))
    }

    #[doc = "Get a reference to the value of field `nfs` after provisioning.\n"]
    pub fn nfs(&self) -> ListRef<DataFsxOntapStorageVirtualMachineEndpointsElNfsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.nfs", self.base))
    }

    #[doc = "Get a reference to the value of field `smb` after provisioning.\n"]
    pub fn smb(&self) -> ListRef<DataFsxOntapStorageVirtualMachineEndpointsElSmbElRef> {
        ListRef::new(self.shared().clone(), format!("{}.smb", self.base))
    }
}

#[derive(Serialize)]
pub struct DataFsxOntapStorageVirtualMachineLifecycleTransitionReasonEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<PrimField<String>>,
}

impl DataFsxOntapStorageVirtualMachineLifecycleTransitionReasonEl {
    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message = Some(v.into());
        self
    }
}

impl ToListMappable for DataFsxOntapStorageVirtualMachineLifecycleTransitionReasonEl {
    type O = BlockAssignable<DataFsxOntapStorageVirtualMachineLifecycleTransitionReasonEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataFsxOntapStorageVirtualMachineLifecycleTransitionReasonEl {}

impl BuildDataFsxOntapStorageVirtualMachineLifecycleTransitionReasonEl {
    pub fn build(self) -> DataFsxOntapStorageVirtualMachineLifecycleTransitionReasonEl {
        DataFsxOntapStorageVirtualMachineLifecycleTransitionReasonEl { message: core::default::Default::default() }
    }
}

pub struct DataFsxOntapStorageVirtualMachineLifecycleTransitionReasonElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataFsxOntapStorageVirtualMachineLifecycleTransitionReasonElRef {
    fn new(shared: StackShared, base: String) -> DataFsxOntapStorageVirtualMachineLifecycleTransitionReasonElRef {
        DataFsxOntapStorageVirtualMachineLifecycleTransitionReasonElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataFsxOntapStorageVirtualMachineLifecycleTransitionReasonElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
}

#[derive(Serialize)]
pub struct DataFsxOntapStorageVirtualMachineFilterEl {
    name: PrimField<String>,
    values: ListField<PrimField<String>>,
}

impl DataFsxOntapStorageVirtualMachineFilterEl { }

impl ToListMappable for DataFsxOntapStorageVirtualMachineFilterEl {
    type O = BlockAssignable<DataFsxOntapStorageVirtualMachineFilterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataFsxOntapStorageVirtualMachineFilterEl {
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub values: ListField<PrimField<String>>,
}

impl BuildDataFsxOntapStorageVirtualMachineFilterEl {
    pub fn build(self) -> DataFsxOntapStorageVirtualMachineFilterEl {
        DataFsxOntapStorageVirtualMachineFilterEl {
            name: self.name,
            values: self.values,
        }
    }
}

pub struct DataFsxOntapStorageVirtualMachineFilterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataFsxOntapStorageVirtualMachineFilterElRef {
    fn new(shared: StackShared, base: String) -> DataFsxOntapStorageVirtualMachineFilterElRef {
        DataFsxOntapStorageVirtualMachineFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataFsxOntapStorageVirtualMachineFilterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataFsxOntapStorageVirtualMachineDynamic {
    filter: Option<DynamicBlock<DataFsxOntapStorageVirtualMachineFilterEl>>,
}
