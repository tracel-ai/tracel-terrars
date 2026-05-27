use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct NetappActiveDirectoryData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    administrators: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    aes_encryption: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    backup_operators: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    dns: PrimField<String>,
    domain: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encrypt_dc_connections: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kdc_hostname: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kdc_ip: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ldap_signing: Option<PrimField<bool>>,
    location: PrimField<String>,
    name: PrimField<String>,
    net_bios_prefix: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nfs_users_with_ldap: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    organizational_unit: Option<PrimField<String>>,
    password: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_operators: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    site: Option<PrimField<String>>,
    username: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<NetappActiveDirectoryTimeoutsEl>,
}
struct NetappActiveDirectory_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<NetappActiveDirectoryData>,
}
#[derive(Clone)]
pub struct NetappActiveDirectory(Rc<NetappActiveDirectory_>);
impl NetappActiveDirectory {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(self, provider: &ProviderGoogle) -> Self {
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
    #[doc = "Set the field `administrators`.\nDomain user accounts to be added to the local Administrators group of the SMB service. Comma-separated list of domain users or groups. The Domain Admin group is automatically added when the service joins your domain as a hidden group."]
    pub fn set_administrators(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().administrators = Some(v.into());
        self
    }
    #[doc = "Set the field `aes_encryption`.\nEnables AES-128 and AES-256 encryption for Kerberos-based communication with Active Directory."]
    pub fn set_aes_encryption(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().aes_encryption = Some(v.into());
        self
    }
    #[doc = "Set the field `backup_operators`.\nDomain user/group accounts to be added to the Backup Operators group of the SMB service. The Backup Operators group allows members to backup and restore files regardless of whether they have read or write access to the files. Comma-separated list."]
    pub fn set_backup_operators(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().backup_operators = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nAn optional description of this resource."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `encrypt_dc_connections`.\nIf enabled, traffic between the SMB server to Domain Controller (DC) will be encrypted."]
    pub fn set_encrypt_dc_connections(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().encrypt_dc_connections = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `kdc_hostname`.\nHostname of the Active Directory server used as Kerberos Key Distribution Center. Only required for volumes using kerberized NFSv4.1"]
    pub fn set_kdc_hostname(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().kdc_hostname = Some(v.into());
        self
    }
    #[doc = "Set the field `kdc_ip`.\nIP address of the Active Directory server used as Kerberos Key Distribution Center."]
    pub fn set_kdc_ip(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().kdc_ip = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nLabels as key value pairs. Example: '{ \"owner\": \"Bob\", \"department\": \"finance\", \"purpose\": \"testing\" }'.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `ldap_signing`.\nSpecifies whether or not the LDAP traffic needs to be signed."]
    pub fn set_ldap_signing(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().ldap_signing = Some(v.into());
        self
    }
    #[doc = "Set the field `nfs_users_with_ldap`.\nLocal UNIX users on clients without valid user information in Active Directory are blocked from access to LDAP enabled volumes.\nThis option can be used to temporarily switch such volumes to AUTH_SYS authentication (user ID + 1-16 groups)."]
    pub fn set_nfs_users_with_ldap(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().nfs_users_with_ldap = Some(v.into());
        self
    }
    #[doc = "Set the field `organizational_unit`.\nName of the Organizational Unit where you intend to create the computer account for NetApp Volumes.\nDefaults to 'CN=Computers' if left empty."]
    pub fn set_organizational_unit(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().organizational_unit = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `security_operators`.\nDomain accounts that require elevated privileges such as 'SeSecurityPrivilege' to manage security logs. Comma-separated list."]
    pub fn set_security_operators(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().security_operators = Some(v.into());
        self
    }
    #[doc = "Set the field `site`.\nSpecifies an Active Directory site to manage domain controller selection.\nUse when Active Directory domain controllers in multiple regions are configured. Defaults to 'Default-First-Site-Name' if left empty."]
    pub fn set_site(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().site = Some(v.into());
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<NetappActiveDirectoryTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `administrators` after provisioning.\nDomain user accounts to be added to the local Administrators group of the SMB service. Comma-separated list of domain users or groups. The Domain Admin group is automatically added when the service joins your domain as a hidden group."]
    pub fn administrators(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.administrators", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `aes_encryption` after provisioning.\nEnables AES-128 and AES-256 encryption for Kerberos-based communication with Active Directory."]
    pub fn aes_encryption(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.aes_encryption", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `backup_operators` after provisioning.\nDomain user/group accounts to be added to the Backup Operators group of the SMB service. The Backup Operators group allows members to backup and restore files regardless of whether they have read or write access to the files. Comma-separated list."]
    pub fn backup_operators(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.backup_operators", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nCreate time of the active directory. A timestamp in RFC3339 UTC \"Zulu\" format. Examples: \"2023-06-22T09:13:01.617Z\"."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dns` after provisioning.\nComma separated list of DNS server IP addresses for the Active Directory domain."]
    pub fn dns(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dns", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `domain` after provisioning.\nFully qualified domain name for the Active Directory domain."]
    pub fn domain(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `encrypt_dc_connections` after provisioning.\nIf enabled, traffic between the SMB server to Domain Controller (DC) will be encrypted."]
    pub fn encrypt_dc_connections(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.encrypt_dc_connections", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `kdc_hostname` after provisioning.\nHostname of the Active Directory server used as Kerberos Key Distribution Center. Only required for volumes using kerberized NFSv4.1"]
    pub fn kdc_hostname(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kdc_hostname", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `kdc_ip` after provisioning.\nIP address of the Active Directory server used as Kerberos Key Distribution Center."]
    pub fn kdc_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kdc_ip", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nLabels as key value pairs. Example: '{ \"owner\": \"Bob\", \"department\": \"finance\", \"purpose\": \"testing\" }'.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ldap_signing` after provisioning.\nSpecifies whether or not the LDAP traffic needs to be signed."]
    pub fn ldap_signing(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ldap_signing", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nName of the region for the policy to apply to."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the Active Directory pool. Needs to be unique per location."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `net_bios_prefix` after provisioning.\nNetBIOS name prefix of the server to be created.\nA five-character random ID is generated automatically, for example, -6f9a, and appended to the prefix. The full UNC share path will have the following format:\n'\\\\NetBIOS_PREFIX-ABCD.DOMAIN_NAME\\SHARE_NAME'"]
    pub fn net_bios_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.net_bios_prefix", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `nfs_users_with_ldap` after provisioning.\nLocal UNIX users on clients without valid user information in Active Directory are blocked from access to LDAP enabled volumes.\nThis option can be used to temporarily switch such volumes to AUTH_SYS authentication (user ID + 1-16 groups)."]
    pub fn nfs_users_with_ldap(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.nfs_users_with_ldap", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `organizational_unit` after provisioning.\nName of the Organizational Unit where you intend to create the computer account for NetApp Volumes.\nDefaults to 'CN=Computers' if left empty."]
    pub fn organizational_unit(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.organizational_unit", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `password` after provisioning.\nPassword for specified username. Note - Manual changes done to the password will not be detected. Terraform will not re-apply the password, unless you use a new password in Terraform."]
    pub fn password(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.password", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `security_operators` after provisioning.\nDomain accounts that require elevated privileges such as 'SeSecurityPrivilege' to manage security logs. Comma-separated list."]
    pub fn security_operators(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.security_operators", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `site` after provisioning.\nSpecifies an Active Directory site to manage domain controller selection.\nUse when Active Directory domain controllers in multiple regions are configured. Defaults to 'Default-First-Site-Name' if left empty."]
    pub fn site(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.site", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe state of the Active Directory policy (not the Active Directory itself)."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state_details` after provisioning.\nThe state details of the Active Directory."]
    pub fn state_details(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state_details", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `username` after provisioning.\nUsername for the Active Directory account with permissions to create the compute account within the specified organizational unit."]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.username", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetappActiveDirectoryTimeoutsElRef {
        NetappActiveDirectoryTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for NetappActiveDirectory {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for NetappActiveDirectory {}
impl ToListMappable for NetappActiveDirectory {
    type O = ListRef<NetappActiveDirectoryRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for NetappActiveDirectory_ {
    fn extract_resource_type(&self) -> String {
        "google_netapp_active_directory".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildNetappActiveDirectory {
    pub tf_id: String,
    #[doc = "Comma separated list of DNS server IP addresses for the Active Directory domain."]
    pub dns: PrimField<String>,
    #[doc = "Fully qualified domain name for the Active Directory domain."]
    pub domain: PrimField<String>,
    #[doc = "Name of the region for the policy to apply to."]
    pub location: PrimField<String>,
    #[doc = "The resource name of the Active Directory pool. Needs to be unique per location."]
    pub name: PrimField<String>,
    #[doc = "NetBIOS name prefix of the server to be created.\nA five-character random ID is generated automatically, for example, -6f9a, and appended to the prefix. The full UNC share path will have the following format:\n'\\\\NetBIOS_PREFIX-ABCD.DOMAIN_NAME\\SHARE_NAME'"]
    pub net_bios_prefix: PrimField<String>,
    #[doc = "Password for specified username. Note - Manual changes done to the password will not be detected. Terraform will not re-apply the password, unless you use a new password in Terraform."]
    pub password: PrimField<String>,
    #[doc = "Username for the Active Directory account with permissions to create the compute account within the specified organizational unit."]
    pub username: PrimField<String>,
}
impl BuildNetappActiveDirectory {
    pub fn build(self, stack: &mut Stack) -> NetappActiveDirectory {
        let out = NetappActiveDirectory(Rc::new(NetappActiveDirectory_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(NetappActiveDirectoryData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                administrators: core::default::Default::default(),
                aes_encryption: core::default::Default::default(),
                backup_operators: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                dns: self.dns,
                domain: self.domain,
                encrypt_dc_connections: core::default::Default::default(),
                id: core::default::Default::default(),
                kdc_hostname: core::default::Default::default(),
                kdc_ip: core::default::Default::default(),
                labels: core::default::Default::default(),
                ldap_signing: core::default::Default::default(),
                location: self.location,
                name: self.name,
                net_bios_prefix: self.net_bios_prefix,
                nfs_users_with_ldap: core::default::Default::default(),
                organizational_unit: core::default::Default::default(),
                password: self.password,
                project: core::default::Default::default(),
                security_operators: core::default::Default::default(),
                site: core::default::Default::default(),
                username: self.username,
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct NetappActiveDirectoryRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetappActiveDirectoryRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl NetappActiveDirectoryRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `administrators` after provisioning.\nDomain user accounts to be added to the local Administrators group of the SMB service. Comma-separated list of domain users or groups. The Domain Admin group is automatically added when the service joins your domain as a hidden group."]
    pub fn administrators(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.administrators", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `aes_encryption` after provisioning.\nEnables AES-128 and AES-256 encryption for Kerberos-based communication with Active Directory."]
    pub fn aes_encryption(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.aes_encryption", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `backup_operators` after provisioning.\nDomain user/group accounts to be added to the Backup Operators group of the SMB service. The Backup Operators group allows members to backup and restore files regardless of whether they have read or write access to the files. Comma-separated list."]
    pub fn backup_operators(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.backup_operators", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nCreate time of the active directory. A timestamp in RFC3339 UTC \"Zulu\" format. Examples: \"2023-06-22T09:13:01.617Z\"."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dns` after provisioning.\nComma separated list of DNS server IP addresses for the Active Directory domain."]
    pub fn dns(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dns", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `domain` after provisioning.\nFully qualified domain name for the Active Directory domain."]
    pub fn domain(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `encrypt_dc_connections` after provisioning.\nIf enabled, traffic between the SMB server to Domain Controller (DC) will be encrypted."]
    pub fn encrypt_dc_connections(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.encrypt_dc_connections", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `kdc_hostname` after provisioning.\nHostname of the Active Directory server used as Kerberos Key Distribution Center. Only required for volumes using kerberized NFSv4.1"]
    pub fn kdc_hostname(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kdc_hostname", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `kdc_ip` after provisioning.\nIP address of the Active Directory server used as Kerberos Key Distribution Center."]
    pub fn kdc_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kdc_ip", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nLabels as key value pairs. Example: '{ \"owner\": \"Bob\", \"department\": \"finance\", \"purpose\": \"testing\" }'.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ldap_signing` after provisioning.\nSpecifies whether or not the LDAP traffic needs to be signed."]
    pub fn ldap_signing(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ldap_signing", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nName of the region for the policy to apply to."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the Active Directory pool. Needs to be unique per location."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `net_bios_prefix` after provisioning.\nNetBIOS name prefix of the server to be created.\nA five-character random ID is generated automatically, for example, -6f9a, and appended to the prefix. The full UNC share path will have the following format:\n'\\\\NetBIOS_PREFIX-ABCD.DOMAIN_NAME\\SHARE_NAME'"]
    pub fn net_bios_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.net_bios_prefix", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `nfs_users_with_ldap` after provisioning.\nLocal UNIX users on clients without valid user information in Active Directory are blocked from access to LDAP enabled volumes.\nThis option can be used to temporarily switch such volumes to AUTH_SYS authentication (user ID + 1-16 groups)."]
    pub fn nfs_users_with_ldap(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.nfs_users_with_ldap", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `organizational_unit` after provisioning.\nName of the Organizational Unit where you intend to create the computer account for NetApp Volumes.\nDefaults to 'CN=Computers' if left empty."]
    pub fn organizational_unit(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.organizational_unit", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `password` after provisioning.\nPassword for specified username. Note - Manual changes done to the password will not be detected. Terraform will not re-apply the password, unless you use a new password in Terraform."]
    pub fn password(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.password", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `security_operators` after provisioning.\nDomain accounts that require elevated privileges such as 'SeSecurityPrivilege' to manage security logs. Comma-separated list."]
    pub fn security_operators(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.security_operators", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `site` after provisioning.\nSpecifies an Active Directory site to manage domain controller selection.\nUse when Active Directory domain controllers in multiple regions are configured. Defaults to 'Default-First-Site-Name' if left empty."]
    pub fn site(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.site", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe state of the Active Directory policy (not the Active Directory itself)."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state_details` after provisioning.\nThe state details of the Active Directory."]
    pub fn state_details(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state_details", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `username` after provisioning.\nUsername for the Active Directory account with permissions to create the compute account within the specified organizational unit."]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.username", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetappActiveDirectoryTimeoutsElRef {
        NetappActiveDirectoryTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct NetappActiveDirectoryTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl NetappActiveDirectoryTimeoutsEl {
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
impl ToListMappable for NetappActiveDirectoryTimeoutsEl {
    type O = BlockAssignable<NetappActiveDirectoryTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetappActiveDirectoryTimeoutsEl {}
impl BuildNetappActiveDirectoryTimeoutsEl {
    pub fn build(self) -> NetappActiveDirectoryTimeoutsEl {
        NetappActiveDirectoryTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct NetappActiveDirectoryTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetappActiveDirectoryTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> NetappActiveDirectoryTimeoutsElRef {
        NetappActiveDirectoryTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetappActiveDirectoryTimeoutsElRef {
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
