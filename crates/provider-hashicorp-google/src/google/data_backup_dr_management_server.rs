use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataBackupDrManagementServerData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
}
struct DataBackupDrManagementServer_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataBackupDrManagementServerData>,
}
#[derive(Clone)]
pub struct DataBackupDrManagementServer(Rc<DataBackupDrManagementServer_>);
impl DataBackupDrManagementServer {
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
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the management server (management console)"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `management_uri` after provisioning.\nThe management console URI"]
    pub fn management_uri(&self) -> ListRef<DataBackupDrManagementServerManagementUriElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.management_uri", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of management server (management console)"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `networks` after provisioning.\nNetwork details to create management server (management console)."]
    pub fn networks(&self) -> ListRef<DataBackupDrManagementServerNetworksElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.networks", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `oauth2_client_id` after provisioning.\nThe oauth2ClientId of management console."]
    pub fn oauth2_client_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.oauth2_client_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe type of management server (management console). Default value: \"BACKUP_RESTORE\" Possible values: [\"BACKUP_RESTORE\"]"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
}
impl Referable for DataBackupDrManagementServer {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataBackupDrManagementServer {}
impl ToListMappable for DataBackupDrManagementServer {
    type O = ListRef<DataBackupDrManagementServerRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataBackupDrManagementServer_ {
    fn extract_datasource_type(&self) -> String {
        "google_backup_dr_management_server".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataBackupDrManagementServer {
    pub tf_id: String,
    #[doc = "The location for the management server (management console)"]
    pub location: PrimField<String>,
}
impl BuildDataBackupDrManagementServer {
    pub fn build(self, stack: &mut Stack) -> DataBackupDrManagementServer {
        let out = DataBackupDrManagementServer(Rc::new(DataBackupDrManagementServer_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataBackupDrManagementServerData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                location: self.location,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataBackupDrManagementServerRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBackupDrManagementServerRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataBackupDrManagementServerRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the management server (management console)"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `management_uri` after provisioning.\nThe management console URI"]
    pub fn management_uri(&self) -> ListRef<DataBackupDrManagementServerManagementUriElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.management_uri", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of management server (management console)"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `networks` after provisioning.\nNetwork details to create management server (management console)."]
    pub fn networks(&self) -> ListRef<DataBackupDrManagementServerNetworksElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.networks", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `oauth2_client_id` after provisioning.\nThe oauth2ClientId of management console."]
    pub fn oauth2_client_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.oauth2_client_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe type of management server (management console). Default value: \"BACKUP_RESTORE\" Possible values: [\"BACKUP_RESTORE\"]"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataBackupDrManagementServerManagementUriEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    api: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    web_ui: Option<PrimField<String>>,
}
impl DataBackupDrManagementServerManagementUriEl {
    #[doc = "Set the field `api`.\n"]
    pub fn set_api(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.api = Some(v.into());
        self
    }
    #[doc = "Set the field `web_ui`.\n"]
    pub fn set_web_ui(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.web_ui = Some(v.into());
        self
    }
}
impl ToListMappable for DataBackupDrManagementServerManagementUriEl {
    type O = BlockAssignable<DataBackupDrManagementServerManagementUriEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBackupDrManagementServerManagementUriEl {}
impl BuildDataBackupDrManagementServerManagementUriEl {
    pub fn build(self) -> DataBackupDrManagementServerManagementUriEl {
        DataBackupDrManagementServerManagementUriEl {
            api: core::default::Default::default(),
            web_ui: core::default::Default::default(),
        }
    }
}
pub struct DataBackupDrManagementServerManagementUriElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBackupDrManagementServerManagementUriElRef {
    fn new(shared: StackShared, base: String) -> DataBackupDrManagementServerManagementUriElRef {
        DataBackupDrManagementServerManagementUriElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBackupDrManagementServerManagementUriElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `api` after provisioning.\n"]
    pub fn api(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.api", self.base))
    }
    #[doc = "Get a reference to the value of field `web_ui` after provisioning.\n"]
    pub fn web_ui(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.web_ui", self.base))
    }
}
#[derive(Serialize)]
pub struct DataBackupDrManagementServerNetworksEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    peering_mode: Option<PrimField<String>>,
}
impl DataBackupDrManagementServerNetworksEl {
    #[doc = "Set the field `network`.\n"]
    pub fn set_network(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network = Some(v.into());
        self
    }
    #[doc = "Set the field `peering_mode`.\n"]
    pub fn set_peering_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.peering_mode = Some(v.into());
        self
    }
}
impl ToListMappable for DataBackupDrManagementServerNetworksEl {
    type O = BlockAssignable<DataBackupDrManagementServerNetworksEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBackupDrManagementServerNetworksEl {}
impl BuildDataBackupDrManagementServerNetworksEl {
    pub fn build(self) -> DataBackupDrManagementServerNetworksEl {
        DataBackupDrManagementServerNetworksEl {
            network: core::default::Default::default(),
            peering_mode: core::default::Default::default(),
        }
    }
}
pub struct DataBackupDrManagementServerNetworksElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBackupDrManagementServerNetworksElRef {
    fn new(shared: StackShared, base: String) -> DataBackupDrManagementServerNetworksElRef {
        DataBackupDrManagementServerNetworksElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBackupDrManagementServerNetworksElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\n"]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }
    #[doc = "Get a reference to the value of field `peering_mode` after provisioning.\n"]
    pub fn peering_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.peering_mode", self.base))
    }
}
