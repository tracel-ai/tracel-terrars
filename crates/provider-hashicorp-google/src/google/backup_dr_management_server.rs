use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct BackupDrManagementServerData {
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
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    networks: Option<Vec<BackupDrManagementServerNetworksEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<BackupDrManagementServerTimeoutsEl>,
    dynamic: BackupDrManagementServerDynamic,
}
struct BackupDrManagementServer_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<BackupDrManagementServerData>,
}
#[derive(Clone)]
pub struct BackupDrManagementServer(Rc<BackupDrManagementServer_>);
impl BackupDrManagementServer {
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
    #[doc = "Set the field `type_`.\nThe type of management server (management console). Default value: \"BACKUP_RESTORE\" Possible values: [\"BACKUP_RESTORE\"]"]
    pub fn set_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().type_ = Some(v.into());
        self
    }
    #[doc = "Set the field `networks`.\n"]
    pub fn set_networks(
        self,
        v: impl Into<BlockAssignable<BackupDrManagementServerNetworksEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().networks = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.networks = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<BackupDrManagementServerTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
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
    pub fn management_uri(&self) -> ListRef<BackupDrManagementServerManagementUriElRef> {
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
    #[doc = "Get a reference to the value of field `networks` after provisioning.\n"]
    pub fn networks(&self) -> ListRef<BackupDrManagementServerNetworksElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.networks", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BackupDrManagementServerTimeoutsElRef {
        BackupDrManagementServerTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for BackupDrManagementServer {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for BackupDrManagementServer {}
impl ToListMappable for BackupDrManagementServer {
    type O = ListRef<BackupDrManagementServerRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for BackupDrManagementServer_ {
    fn extract_resource_type(&self) -> String {
        "google_backup_dr_management_server".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildBackupDrManagementServer {
    pub tf_id: String,
    #[doc = "The location for the management server (management console)"]
    pub location: PrimField<String>,
    #[doc = "The name of management server (management console)"]
    pub name: PrimField<String>,
}
impl BuildBackupDrManagementServer {
    pub fn build(self, stack: &mut Stack) -> BackupDrManagementServer {
        let out = BackupDrManagementServer(Rc::new(BackupDrManagementServer_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(BackupDrManagementServerData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                name: self.name,
                project: core::default::Default::default(),
                type_: core::default::Default::default(),
                networks: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct BackupDrManagementServerRef {
    shared: StackShared,
    base: String,
}
impl Ref for BackupDrManagementServerRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl BackupDrManagementServerRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
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
    pub fn management_uri(&self) -> ListRef<BackupDrManagementServerManagementUriElRef> {
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
    #[doc = "Get a reference to the value of field `networks` after provisioning.\n"]
    pub fn networks(&self) -> ListRef<BackupDrManagementServerNetworksElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.networks", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BackupDrManagementServerTimeoutsElRef {
        BackupDrManagementServerTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct BackupDrManagementServerManagementUriEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    api: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    web_ui: Option<PrimField<String>>,
}
impl BackupDrManagementServerManagementUriEl {
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
impl ToListMappable for BackupDrManagementServerManagementUriEl {
    type O = BlockAssignable<BackupDrManagementServerManagementUriEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBackupDrManagementServerManagementUriEl {}
impl BuildBackupDrManagementServerManagementUriEl {
    pub fn build(self) -> BackupDrManagementServerManagementUriEl {
        BackupDrManagementServerManagementUriEl {
            api: core::default::Default::default(),
            web_ui: core::default::Default::default(),
        }
    }
}
pub struct BackupDrManagementServerManagementUriElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BackupDrManagementServerManagementUriElRef {
    fn new(shared: StackShared, base: String) -> BackupDrManagementServerManagementUriElRef {
        BackupDrManagementServerManagementUriElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BackupDrManagementServerManagementUriElRef {
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
pub struct BackupDrManagementServerNetworksEl {
    network: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    peering_mode: Option<PrimField<String>>,
}
impl BackupDrManagementServerNetworksEl {
    #[doc = "Set the field `peering_mode`.\nType of Network peeringMode Default value: \"PRIVATE_SERVICE_ACCESS\" Possible values: [\"PRIVATE_SERVICE_ACCESS\"]"]
    pub fn set_peering_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.peering_mode = Some(v.into());
        self
    }
}
impl ToListMappable for BackupDrManagementServerNetworksEl {
    type O = BlockAssignable<BackupDrManagementServerNetworksEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBackupDrManagementServerNetworksEl {
    #[doc = "Network with format 'projects/{{project_id}}/global/networks/{{network_id}}'"]
    pub network: PrimField<String>,
}
impl BuildBackupDrManagementServerNetworksEl {
    pub fn build(self) -> BackupDrManagementServerNetworksEl {
        BackupDrManagementServerNetworksEl {
            network: self.network,
            peering_mode: core::default::Default::default(),
        }
    }
}
pub struct BackupDrManagementServerNetworksElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BackupDrManagementServerNetworksElRef {
    fn new(shared: StackShared, base: String) -> BackupDrManagementServerNetworksElRef {
        BackupDrManagementServerNetworksElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BackupDrManagementServerNetworksElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\nNetwork with format 'projects/{{project_id}}/global/networks/{{network_id}}'"]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }
    #[doc = "Get a reference to the value of field `peering_mode` after provisioning.\nType of Network peeringMode Default value: \"PRIVATE_SERVICE_ACCESS\" Possible values: [\"PRIVATE_SERVICE_ACCESS\"]"]
    pub fn peering_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.peering_mode", self.base))
    }
}
#[derive(Serialize)]
pub struct BackupDrManagementServerTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}
impl BackupDrManagementServerTimeoutsEl {
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
}
impl ToListMappable for BackupDrManagementServerTimeoutsEl {
    type O = BlockAssignable<BackupDrManagementServerTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBackupDrManagementServerTimeoutsEl {}
impl BuildBackupDrManagementServerTimeoutsEl {
    pub fn build(self) -> BackupDrManagementServerTimeoutsEl {
        BackupDrManagementServerTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}
pub struct BackupDrManagementServerTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BackupDrManagementServerTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> BackupDrManagementServerTimeoutsElRef {
        BackupDrManagementServerTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BackupDrManagementServerTimeoutsElRef {
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
}
#[derive(Serialize, Default)]
struct BackupDrManagementServerDynamic {
    networks: Option<DynamicBlock<BackupDrManagementServerNetworksEl>>,
}
