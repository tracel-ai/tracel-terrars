use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct VmwareengineDatastoreData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nfs_datastore: Option<Vec<VmwareengineDatastoreNfsDatastoreEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<VmwareengineDatastoreTimeoutsEl>,
    dynamic: VmwareengineDatastoreDynamic,
}
struct VmwareengineDatastore_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<VmwareengineDatastoreData>,
}
#[derive(Clone)]
pub struct VmwareengineDatastore(Rc<VmwareengineDatastore_>);
impl VmwareengineDatastore {
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
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nUser-provided description for this datastore"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `nfs_datastore`.\n"]
    pub fn set_nfs_datastore(
        self,
        v: impl Into<BlockAssignable<VmwareengineDatastoreNfsDatastoreEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().nfs_datastore = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.nfs_datastore = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<VmwareengineDatastoreTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `clusters` after provisioning.\nClusters to which the datastore is attached."]
    pub fn clusters(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.clusters", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nCreation time of this resource."]
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nUser-provided description for this datastore"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe user-provided identifier of the datastore to be created.\nThis identifier must be unique among each 'Datastore' within the parent\nand becomes the final token in the name URI.\nThe identifier must meet the following requirements:\n\n* Only contains 1-63 alphanumeric characters and hyphens\n* Begins with an alphabetical character\n* Ends with a non-hyphen character\n* Not formatted as a UUID\n* Complies with [RFC 1034](https://datatracker.ietf.org/doc/html/rfc1034)\n(section 3.5)"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe state of the Datastore.\nPossible values:\nCREATING\nACTIVE\nUPDATING\nDELETING\nSOFT_DELETING\nSOFT_DELETED"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nSystem-generated unique identifier for the resource."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nLast update time of this resource."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `nfs_datastore` after provisioning.\n"]
    pub fn nfs_datastore(&self) -> ListRef<VmwareengineDatastoreNfsDatastoreElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.nfs_datastore", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VmwareengineDatastoreTimeoutsElRef {
        VmwareengineDatastoreTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for VmwareengineDatastore {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for VmwareengineDatastore {}
impl ToListMappable for VmwareengineDatastore {
    type O = ListRef<VmwareengineDatastoreRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for VmwareengineDatastore_ {
    fn extract_resource_type(&self) -> String {
        "google_vmwareengine_datastore".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildVmwareengineDatastore {
    pub tf_id: String,
    #[doc = "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub location: PrimField<String>,
    #[doc = "The user-provided identifier of the datastore to be created.\nThis identifier must be unique among each 'Datastore' within the parent\nand becomes the final token in the name URI.\nThe identifier must meet the following requirements:\n\n* Only contains 1-63 alphanumeric characters and hyphens\n* Begins with an alphabetical character\n* Ends with a non-hyphen character\n* Not formatted as a UUID\n* Complies with [RFC 1034](https://datatracker.ietf.org/doc/html/rfc1034)\n(section 3.5)"]
    pub name: PrimField<String>,
}
impl BuildVmwareengineDatastore {
    pub fn build(self, stack: &mut Stack) -> VmwareengineDatastore {
        let out = VmwareengineDatastore(Rc::new(VmwareengineDatastore_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(VmwareengineDatastoreData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                name: self.name,
                project: core::default::Default::default(),
                nfs_datastore: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct VmwareengineDatastoreRef {
    shared: StackShared,
    base: String,
}
impl Ref for VmwareengineDatastoreRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl VmwareengineDatastoreRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `clusters` after provisioning.\nClusters to which the datastore is attached."]
    pub fn clusters(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.clusters", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nCreation time of this resource."]
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nUser-provided description for this datastore"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe user-provided identifier of the datastore to be created.\nThis identifier must be unique among each 'Datastore' within the parent\nand becomes the final token in the name URI.\nThe identifier must meet the following requirements:\n\n* Only contains 1-63 alphanumeric characters and hyphens\n* Begins with an alphabetical character\n* Ends with a non-hyphen character\n* Not formatted as a UUID\n* Complies with [RFC 1034](https://datatracker.ietf.org/doc/html/rfc1034)\n(section 3.5)"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe state of the Datastore.\nPossible values:\nCREATING\nACTIVE\nUPDATING\nDELETING\nSOFT_DELETING\nSOFT_DELETED"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nSystem-generated unique identifier for the resource."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nLast update time of this resource."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `nfs_datastore` after provisioning.\n"]
    pub fn nfs_datastore(&self) -> ListRef<VmwareengineDatastoreNfsDatastoreElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.nfs_datastore", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VmwareengineDatastoreTimeoutsElRef {
        VmwareengineDatastoreTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct VmwareengineDatastoreNfsDatastoreElGoogleFileServiceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    filestore_instance: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    netapp_volume: Option<PrimField<String>>,
}
impl VmwareengineDatastoreNfsDatastoreElGoogleFileServiceEl {
    #[doc = "Set the field `filestore_instance`.\nGoogle filestore instance resource name\ne.g. projects/my-project/locations/me-west1-b/instances/my-instance"]
    pub fn set_filestore_instance(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.filestore_instance = Some(v.into());
        self
    }
    #[doc = "Set the field `netapp_volume`.\nGoogle netapp volume resource name\ne.g. projects/my-project/locations/me-west1-b/volumes/my-volume"]
    pub fn set_netapp_volume(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.netapp_volume = Some(v.into());
        self
    }
}
impl ToListMappable for VmwareengineDatastoreNfsDatastoreElGoogleFileServiceEl {
    type O = BlockAssignable<VmwareengineDatastoreNfsDatastoreElGoogleFileServiceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVmwareengineDatastoreNfsDatastoreElGoogleFileServiceEl {}
impl BuildVmwareengineDatastoreNfsDatastoreElGoogleFileServiceEl {
    pub fn build(self) -> VmwareengineDatastoreNfsDatastoreElGoogleFileServiceEl {
        VmwareengineDatastoreNfsDatastoreElGoogleFileServiceEl {
            filestore_instance: core::default::Default::default(),
            netapp_volume: core::default::Default::default(),
        }
    }
}
pub struct VmwareengineDatastoreNfsDatastoreElGoogleFileServiceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VmwareengineDatastoreNfsDatastoreElGoogleFileServiceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> VmwareengineDatastoreNfsDatastoreElGoogleFileServiceElRef {
        VmwareengineDatastoreNfsDatastoreElGoogleFileServiceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VmwareengineDatastoreNfsDatastoreElGoogleFileServiceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `filestore_instance` after provisioning.\nGoogle filestore instance resource name\ne.g. projects/my-project/locations/me-west1-b/instances/my-instance"]
    pub fn filestore_instance(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.filestore_instance", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `netapp_volume` after provisioning.\nGoogle netapp volume resource name\ne.g. projects/my-project/locations/me-west1-b/volumes/my-volume"]
    pub fn netapp_volume(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.netapp_volume", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct VmwareengineDatastoreNfsDatastoreElThirdPartyFileServiceEl {
    file_share: PrimField<String>,
    network: PrimField<String>,
    servers: ListField<PrimField<String>>,
}
impl VmwareengineDatastoreNfsDatastoreElThirdPartyFileServiceEl {}
impl ToListMappable for VmwareengineDatastoreNfsDatastoreElThirdPartyFileServiceEl {
    type O = BlockAssignable<VmwareengineDatastoreNfsDatastoreElThirdPartyFileServiceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVmwareengineDatastoreNfsDatastoreElThirdPartyFileServiceEl {
    #[doc = "Required\nMount Folder name"]
    pub file_share: PrimField<String>,
    #[doc = "Required to identify vpc peering used for NFS access\nnetwork name of NFS's vpc\ne.g. projects/project-id/global/networks/my-network_id"]
    pub network: PrimField<String>,
    #[doc = "Server IP addresses of the NFS file service.\nNFS v3, provide a single IP address or DNS name.\nMultiple servers can be supported in future when NFS 4.1 protocol support\nis enabled."]
    pub servers: ListField<PrimField<String>>,
}
impl BuildVmwareengineDatastoreNfsDatastoreElThirdPartyFileServiceEl {
    pub fn build(self) -> VmwareengineDatastoreNfsDatastoreElThirdPartyFileServiceEl {
        VmwareengineDatastoreNfsDatastoreElThirdPartyFileServiceEl {
            file_share: self.file_share,
            network: self.network,
            servers: self.servers,
        }
    }
}
pub struct VmwareengineDatastoreNfsDatastoreElThirdPartyFileServiceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VmwareengineDatastoreNfsDatastoreElThirdPartyFileServiceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> VmwareengineDatastoreNfsDatastoreElThirdPartyFileServiceElRef {
        VmwareengineDatastoreNfsDatastoreElThirdPartyFileServiceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VmwareengineDatastoreNfsDatastoreElThirdPartyFileServiceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `file_share` after provisioning.\nRequired\nMount Folder name"]
    pub fn file_share(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.file_share", self.base))
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\nRequired to identify vpc peering used for NFS access\nnetwork name of NFS's vpc\ne.g. projects/project-id/global/networks/my-network_id"]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }
    #[doc = "Get a reference to the value of field `servers` after provisioning.\nServer IP addresses of the NFS file service.\nNFS v3, provide a single IP address or DNS name.\nMultiple servers can be supported in future when NFS 4.1 protocol support\nis enabled."]
    pub fn servers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.servers", self.base))
    }
}
#[derive(Serialize, Default)]
struct VmwareengineDatastoreNfsDatastoreElDynamic {
    google_file_service:
        Option<DynamicBlock<VmwareengineDatastoreNfsDatastoreElGoogleFileServiceEl>>,
    third_party_file_service:
        Option<DynamicBlock<VmwareengineDatastoreNfsDatastoreElThirdPartyFileServiceEl>>,
}
#[derive(Serialize)]
pub struct VmwareengineDatastoreNfsDatastoreEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    google_file_service: Option<Vec<VmwareengineDatastoreNfsDatastoreElGoogleFileServiceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    third_party_file_service:
        Option<Vec<VmwareengineDatastoreNfsDatastoreElThirdPartyFileServiceEl>>,
    dynamic: VmwareengineDatastoreNfsDatastoreElDynamic,
}
impl VmwareengineDatastoreNfsDatastoreEl {
    #[doc = "Set the field `google_file_service`.\n"]
    pub fn set_google_file_service(
        mut self,
        v: impl Into<BlockAssignable<VmwareengineDatastoreNfsDatastoreElGoogleFileServiceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.google_file_service = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.google_file_service = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `third_party_file_service`.\n"]
    pub fn set_third_party_file_service(
        mut self,
        v: impl Into<BlockAssignable<VmwareengineDatastoreNfsDatastoreElThirdPartyFileServiceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.third_party_file_service = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.third_party_file_service = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for VmwareengineDatastoreNfsDatastoreEl {
    type O = BlockAssignable<VmwareengineDatastoreNfsDatastoreEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVmwareengineDatastoreNfsDatastoreEl {}
impl BuildVmwareengineDatastoreNfsDatastoreEl {
    pub fn build(self) -> VmwareengineDatastoreNfsDatastoreEl {
        VmwareengineDatastoreNfsDatastoreEl {
            google_file_service: core::default::Default::default(),
            third_party_file_service: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct VmwareengineDatastoreNfsDatastoreElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VmwareengineDatastoreNfsDatastoreElRef {
    fn new(shared: StackShared, base: String) -> VmwareengineDatastoreNfsDatastoreElRef {
        VmwareengineDatastoreNfsDatastoreElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VmwareengineDatastoreNfsDatastoreElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `google_file_service` after provisioning.\n"]
    pub fn google_file_service(
        &self,
    ) -> ListRef<VmwareengineDatastoreNfsDatastoreElGoogleFileServiceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.google_file_service", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `third_party_file_service` after provisioning.\n"]
    pub fn third_party_file_service(
        &self,
    ) -> ListRef<VmwareengineDatastoreNfsDatastoreElThirdPartyFileServiceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.third_party_file_service", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct VmwareengineDatastoreTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl VmwareengineDatastoreTimeoutsEl {
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
impl ToListMappable for VmwareengineDatastoreTimeoutsEl {
    type O = BlockAssignable<VmwareengineDatastoreTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVmwareengineDatastoreTimeoutsEl {}
impl BuildVmwareengineDatastoreTimeoutsEl {
    pub fn build(self) -> VmwareengineDatastoreTimeoutsEl {
        VmwareengineDatastoreTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct VmwareengineDatastoreTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VmwareengineDatastoreTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> VmwareengineDatastoreTimeoutsElRef {
        VmwareengineDatastoreTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VmwareengineDatastoreTimeoutsElRef {
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
struct VmwareengineDatastoreDynamic {
    nfs_datastore: Option<DynamicBlock<VmwareengineDatastoreNfsDatastoreEl>>,
}
