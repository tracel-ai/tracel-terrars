use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataVmwareengineDatastoreData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
}
struct DataVmwareengineDatastore_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataVmwareengineDatastoreData>,
}
#[derive(Clone)]
pub struct DataVmwareengineDatastore(Rc<DataVmwareengineDatastore_>);
impl DataVmwareengineDatastore {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(&self, provider: &ProviderGoogle) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
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
    #[doc = "Get a reference to the value of field `nfs_datastore` after provisioning.\nThe NFS datastore configuration."]
    pub fn nfs_datastore(&self) -> ListRef<DataVmwareengineDatastoreNfsDatastoreElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.nfs_datastore", self.extract_ref()),
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
}
impl Referable for DataVmwareengineDatastore {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataVmwareengineDatastore {}
impl ToListMappable for DataVmwareengineDatastore {
    type O = ListRef<DataVmwareengineDatastoreRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataVmwareengineDatastore_ {
    fn extract_datasource_type(&self) -> String {
        "google_vmwareengine_datastore".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataVmwareengineDatastore {
    pub tf_id: String,
    #[doc = "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub location: PrimField<String>,
    #[doc = "The user-provided identifier of the datastore to be created.\nThis identifier must be unique among each 'Datastore' within the parent\nand becomes the final token in the name URI.\nThe identifier must meet the following requirements:\n\n* Only contains 1-63 alphanumeric characters and hyphens\n* Begins with an alphabetical character\n* Ends with a non-hyphen character\n* Not formatted as a UUID\n* Complies with [RFC 1034](https://datatracker.ietf.org/doc/html/rfc1034)\n(section 3.5)"]
    pub name: PrimField<String>,
}
impl BuildDataVmwareengineDatastore {
    pub fn build(self, stack: &mut Stack) -> DataVmwareengineDatastore {
        let out = DataVmwareengineDatastore(Rc::new(DataVmwareengineDatastore_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataVmwareengineDatastoreData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                location: self.location,
                name: self.name,
                project: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataVmwareengineDatastoreRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataVmwareengineDatastoreRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataVmwareengineDatastoreRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
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
    #[doc = "Get a reference to the value of field `nfs_datastore` after provisioning.\nThe NFS datastore configuration."]
    pub fn nfs_datastore(&self) -> ListRef<DataVmwareengineDatastoreNfsDatastoreElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.nfs_datastore", self.extract_ref()),
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
}
#[derive(Serialize)]
pub struct DataVmwareengineDatastoreNfsDatastoreElGoogleFileServiceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    filestore_instance: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    netapp_volume: Option<PrimField<String>>,
}
impl DataVmwareengineDatastoreNfsDatastoreElGoogleFileServiceEl {
    #[doc = "Set the field `filestore_instance`.\n"]
    pub fn set_filestore_instance(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.filestore_instance = Some(v.into());
        self
    }
    #[doc = "Set the field `netapp_volume`.\n"]
    pub fn set_netapp_volume(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.netapp_volume = Some(v.into());
        self
    }
}
impl ToListMappable for DataVmwareengineDatastoreNfsDatastoreElGoogleFileServiceEl {
    type O = BlockAssignable<DataVmwareengineDatastoreNfsDatastoreElGoogleFileServiceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataVmwareengineDatastoreNfsDatastoreElGoogleFileServiceEl {}
impl BuildDataVmwareengineDatastoreNfsDatastoreElGoogleFileServiceEl {
    pub fn build(self) -> DataVmwareengineDatastoreNfsDatastoreElGoogleFileServiceEl {
        DataVmwareengineDatastoreNfsDatastoreElGoogleFileServiceEl {
            filestore_instance: core::default::Default::default(),
            netapp_volume: core::default::Default::default(),
        }
    }
}
pub struct DataVmwareengineDatastoreNfsDatastoreElGoogleFileServiceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataVmwareengineDatastoreNfsDatastoreElGoogleFileServiceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataVmwareengineDatastoreNfsDatastoreElGoogleFileServiceElRef {
        DataVmwareengineDatastoreNfsDatastoreElGoogleFileServiceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataVmwareengineDatastoreNfsDatastoreElGoogleFileServiceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `filestore_instance` after provisioning.\n"]
    pub fn filestore_instance(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.filestore_instance", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `netapp_volume` after provisioning.\n"]
    pub fn netapp_volume(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.netapp_volume", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataVmwareengineDatastoreNfsDatastoreElThirdPartyFileServiceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    file_share: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    servers: Option<ListField<PrimField<String>>>,
}
impl DataVmwareengineDatastoreNfsDatastoreElThirdPartyFileServiceEl {
    #[doc = "Set the field `file_share`.\n"]
    pub fn set_file_share(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.file_share = Some(v.into());
        self
    }
    #[doc = "Set the field `network`.\n"]
    pub fn set_network(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network = Some(v.into());
        self
    }
    #[doc = "Set the field `servers`.\n"]
    pub fn set_servers(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.servers = Some(v.into());
        self
    }
}
impl ToListMappable for DataVmwareengineDatastoreNfsDatastoreElThirdPartyFileServiceEl {
    type O = BlockAssignable<DataVmwareengineDatastoreNfsDatastoreElThirdPartyFileServiceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataVmwareengineDatastoreNfsDatastoreElThirdPartyFileServiceEl {}
impl BuildDataVmwareengineDatastoreNfsDatastoreElThirdPartyFileServiceEl {
    pub fn build(self) -> DataVmwareengineDatastoreNfsDatastoreElThirdPartyFileServiceEl {
        DataVmwareengineDatastoreNfsDatastoreElThirdPartyFileServiceEl {
            file_share: core::default::Default::default(),
            network: core::default::Default::default(),
            servers: core::default::Default::default(),
        }
    }
}
pub struct DataVmwareengineDatastoreNfsDatastoreElThirdPartyFileServiceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataVmwareengineDatastoreNfsDatastoreElThirdPartyFileServiceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataVmwareengineDatastoreNfsDatastoreElThirdPartyFileServiceElRef {
        DataVmwareengineDatastoreNfsDatastoreElThirdPartyFileServiceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataVmwareengineDatastoreNfsDatastoreElThirdPartyFileServiceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `file_share` after provisioning.\n"]
    pub fn file_share(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.file_share", self.base))
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\n"]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }
    #[doc = "Get a reference to the value of field `servers` after provisioning.\n"]
    pub fn servers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.servers", self.base))
    }
}
#[derive(Serialize)]
pub struct DataVmwareengineDatastoreNfsDatastoreEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    google_file_service:
        Option<ListField<DataVmwareengineDatastoreNfsDatastoreElGoogleFileServiceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    third_party_file_service:
        Option<ListField<DataVmwareengineDatastoreNfsDatastoreElThirdPartyFileServiceEl>>,
}
impl DataVmwareengineDatastoreNfsDatastoreEl {
    #[doc = "Set the field `google_file_service`.\n"]
    pub fn set_google_file_service(
        mut self,
        v: impl Into<ListField<DataVmwareengineDatastoreNfsDatastoreElGoogleFileServiceEl>>,
    ) -> Self {
        self.google_file_service = Some(v.into());
        self
    }
    #[doc = "Set the field `third_party_file_service`.\n"]
    pub fn set_third_party_file_service(
        mut self,
        v: impl Into<ListField<DataVmwareengineDatastoreNfsDatastoreElThirdPartyFileServiceEl>>,
    ) -> Self {
        self.third_party_file_service = Some(v.into());
        self
    }
}
impl ToListMappable for DataVmwareengineDatastoreNfsDatastoreEl {
    type O = BlockAssignable<DataVmwareengineDatastoreNfsDatastoreEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataVmwareengineDatastoreNfsDatastoreEl {}
impl BuildDataVmwareengineDatastoreNfsDatastoreEl {
    pub fn build(self) -> DataVmwareengineDatastoreNfsDatastoreEl {
        DataVmwareengineDatastoreNfsDatastoreEl {
            google_file_service: core::default::Default::default(),
            third_party_file_service: core::default::Default::default(),
        }
    }
}
pub struct DataVmwareengineDatastoreNfsDatastoreElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataVmwareengineDatastoreNfsDatastoreElRef {
    fn new(shared: StackShared, base: String) -> DataVmwareengineDatastoreNfsDatastoreElRef {
        DataVmwareengineDatastoreNfsDatastoreElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataVmwareengineDatastoreNfsDatastoreElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `google_file_service` after provisioning.\n"]
    pub fn google_file_service(
        &self,
    ) -> ListRef<DataVmwareengineDatastoreNfsDatastoreElGoogleFileServiceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.google_file_service", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `third_party_file_service` after provisioning.\n"]
    pub fn third_party_file_service(
        &self,
    ) -> ListRef<DataVmwareengineDatastoreNfsDatastoreElThirdPartyFileServiceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.third_party_file_service", self.base),
        )
    }
}
