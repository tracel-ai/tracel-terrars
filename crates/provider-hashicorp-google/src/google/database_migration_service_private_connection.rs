use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DatabaseMigrationServicePrivateConnectionData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    create_without_validation: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    location: PrimField<String>,
    private_connection_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    psc_interface_config:
        Option<Vec<DatabaseMigrationServicePrivateConnectionPscInterfaceConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DatabaseMigrationServicePrivateConnectionTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_peering_config: Option<Vec<DatabaseMigrationServicePrivateConnectionVpcPeeringConfigEl>>,
    dynamic: DatabaseMigrationServicePrivateConnectionDynamic,
}
struct DatabaseMigrationServicePrivateConnection_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DatabaseMigrationServicePrivateConnectionData>,
}
#[derive(Clone)]
pub struct DatabaseMigrationServicePrivateConnection(
    Rc<DatabaseMigrationServicePrivateConnection_>,
);
impl DatabaseMigrationServicePrivateConnection {
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
    #[doc = "Set the field `create_without_validation`.\nIf set to true, will skip validations."]
    pub fn set_create_without_validation(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().create_without_validation = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\nDisplay name."]
    pub fn set_display_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nLabels.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `psc_interface_config`.\n"]
    pub fn set_psc_interface_config(
        self,
        v: impl Into<BlockAssignable<DatabaseMigrationServicePrivateConnectionPscInterfaceConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().psc_interface_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.psc_interface_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(
        self,
        v: impl Into<DatabaseMigrationServicePrivateConnectionTimeoutsEl>,
    ) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Set the field `vpc_peering_config`.\n"]
    pub fn set_vpc_peering_config(
        self,
        v: impl Into<BlockAssignable<DatabaseMigrationServicePrivateConnectionVpcPeeringConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().vpc_peering_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.vpc_peering_config = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `create_without_validation` after provisioning.\nIf set to true, will skip validations."]
    pub fn create_without_validation(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_without_validation", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nDisplay name."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `error` after provisioning.\nThe PrivateConnection error in case of failure."]
    pub fn error(&self) -> ListRef<DatabaseMigrationServicePrivateConnectionErrorElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.error", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nLabels.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe name of the location this private connection is located in."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource's name."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `private_connection_id` after provisioning.\nThe private connectivity identifier."]
    pub fn private_connection_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.private_connection_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nState of the PrivateConnection."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `psc_interface_config` after provisioning.\n"]
    pub fn psc_interface_config(
        &self,
    ) -> ListRef<DatabaseMigrationServicePrivateConnectionPscInterfaceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.psc_interface_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DatabaseMigrationServicePrivateConnectionTimeoutsElRef {
        DatabaseMigrationServicePrivateConnectionTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vpc_peering_config` after provisioning.\n"]
    pub fn vpc_peering_config(
        &self,
    ) -> ListRef<DatabaseMigrationServicePrivateConnectionVpcPeeringConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vpc_peering_config", self.extract_ref()),
        )
    }
}
impl Referable for DatabaseMigrationServicePrivateConnection {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for DatabaseMigrationServicePrivateConnection {}
impl ToListMappable for DatabaseMigrationServicePrivateConnection {
    type O = ListRef<DatabaseMigrationServicePrivateConnectionRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for DatabaseMigrationServicePrivateConnection_ {
    fn extract_resource_type(&self) -> String {
        "google_database_migration_service_private_connection".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDatabaseMigrationServicePrivateConnection {
    pub tf_id: String,
    #[doc = "The name of the location this private connection is located in."]
    pub location: PrimField<String>,
    #[doc = "The private connectivity identifier."]
    pub private_connection_id: PrimField<String>,
}
impl BuildDatabaseMigrationServicePrivateConnection {
    pub fn build(self, stack: &mut Stack) -> DatabaseMigrationServicePrivateConnection {
        let out = DatabaseMigrationServicePrivateConnection(Rc::new(
            DatabaseMigrationServicePrivateConnection_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(DatabaseMigrationServicePrivateConnectionData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    lifecycle: core::default::Default::default(),
                    for_each: None,
                    create_without_validation: core::default::Default::default(),
                    deletion_policy: core::default::Default::default(),
                    display_name: core::default::Default::default(),
                    id: core::default::Default::default(),
                    labels: core::default::Default::default(),
                    location: self.location,
                    private_connection_id: self.private_connection_id,
                    project: core::default::Default::default(),
                    psc_interface_config: core::default::Default::default(),
                    timeouts: core::default::Default::default(),
                    vpc_peering_config: core::default::Default::default(),
                    dynamic: Default::default(),
                }),
            },
        ));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct DatabaseMigrationServicePrivateConnectionRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatabaseMigrationServicePrivateConnectionRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DatabaseMigrationServicePrivateConnectionRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_without_validation` after provisioning.\nIf set to true, will skip validations."]
    pub fn create_without_validation(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_without_validation", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nDisplay name."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `error` after provisioning.\nThe PrivateConnection error in case of failure."]
    pub fn error(&self) -> ListRef<DatabaseMigrationServicePrivateConnectionErrorElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.error", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nLabels.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe name of the location this private connection is located in."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource's name."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `private_connection_id` after provisioning.\nThe private connectivity identifier."]
    pub fn private_connection_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.private_connection_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nState of the PrivateConnection."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `psc_interface_config` after provisioning.\n"]
    pub fn psc_interface_config(
        &self,
    ) -> ListRef<DatabaseMigrationServicePrivateConnectionPscInterfaceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.psc_interface_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DatabaseMigrationServicePrivateConnectionTimeoutsElRef {
        DatabaseMigrationServicePrivateConnectionTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vpc_peering_config` after provisioning.\n"]
    pub fn vpc_peering_config(
        &self,
    ) -> ListRef<DatabaseMigrationServicePrivateConnectionVpcPeeringConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vpc_peering_config", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DatabaseMigrationServicePrivateConnectionErrorEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<PrimField<String>>,
}
impl DatabaseMigrationServicePrivateConnectionErrorEl {
    #[doc = "Set the field `details`.\n"]
    pub fn set_details(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.details = Some(v.into());
        self
    }
    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message = Some(v.into());
        self
    }
}
impl ToListMappable for DatabaseMigrationServicePrivateConnectionErrorEl {
    type O = BlockAssignable<DatabaseMigrationServicePrivateConnectionErrorEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatabaseMigrationServicePrivateConnectionErrorEl {}
impl BuildDatabaseMigrationServicePrivateConnectionErrorEl {
    pub fn build(self) -> DatabaseMigrationServicePrivateConnectionErrorEl {
        DatabaseMigrationServicePrivateConnectionErrorEl {
            details: core::default::Default::default(),
            message: core::default::Default::default(),
        }
    }
}
pub struct DatabaseMigrationServicePrivateConnectionErrorElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatabaseMigrationServicePrivateConnectionErrorElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatabaseMigrationServicePrivateConnectionErrorElRef {
        DatabaseMigrationServicePrivateConnectionErrorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatabaseMigrationServicePrivateConnectionErrorElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `details` after provisioning.\n"]
    pub fn details(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.details", self.base))
    }
    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
}
#[derive(Serialize)]
pub struct DatabaseMigrationServicePrivateConnectionPscInterfaceConfigEl {
    network_attachment: PrimField<String>,
}
impl DatabaseMigrationServicePrivateConnectionPscInterfaceConfigEl {}
impl ToListMappable for DatabaseMigrationServicePrivateConnectionPscInterfaceConfigEl {
    type O = BlockAssignable<DatabaseMigrationServicePrivateConnectionPscInterfaceConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatabaseMigrationServicePrivateConnectionPscInterfaceConfigEl {
    #[doc = "Fully qualified name of the Network Attachment that DMS will connect to.\nFormat: projects/{project}/regions/{region}/networkAttachments/{name}"]
    pub network_attachment: PrimField<String>,
}
impl BuildDatabaseMigrationServicePrivateConnectionPscInterfaceConfigEl {
    pub fn build(self) -> DatabaseMigrationServicePrivateConnectionPscInterfaceConfigEl {
        DatabaseMigrationServicePrivateConnectionPscInterfaceConfigEl {
            network_attachment: self.network_attachment,
        }
    }
}
pub struct DatabaseMigrationServicePrivateConnectionPscInterfaceConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatabaseMigrationServicePrivateConnectionPscInterfaceConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatabaseMigrationServicePrivateConnectionPscInterfaceConfigElRef {
        DatabaseMigrationServicePrivateConnectionPscInterfaceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatabaseMigrationServicePrivateConnectionPscInterfaceConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `network_attachment` after provisioning.\nFully qualified name of the Network Attachment that DMS will connect to.\nFormat: projects/{project}/regions/{region}/networkAttachments/{name}"]
    pub fn network_attachment(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network_attachment", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DatabaseMigrationServicePrivateConnectionTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl DatabaseMigrationServicePrivateConnectionTimeoutsEl {
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
impl ToListMappable for DatabaseMigrationServicePrivateConnectionTimeoutsEl {
    type O = BlockAssignable<DatabaseMigrationServicePrivateConnectionTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatabaseMigrationServicePrivateConnectionTimeoutsEl {}
impl BuildDatabaseMigrationServicePrivateConnectionTimeoutsEl {
    pub fn build(self) -> DatabaseMigrationServicePrivateConnectionTimeoutsEl {
        DatabaseMigrationServicePrivateConnectionTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct DatabaseMigrationServicePrivateConnectionTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatabaseMigrationServicePrivateConnectionTimeoutsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatabaseMigrationServicePrivateConnectionTimeoutsElRef {
        DatabaseMigrationServicePrivateConnectionTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatabaseMigrationServicePrivateConnectionTimeoutsElRef {
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
#[derive(Serialize)]
pub struct DatabaseMigrationServicePrivateConnectionVpcPeeringConfigEl {
    subnet: PrimField<String>,
    vpc_name: PrimField<String>,
}
impl DatabaseMigrationServicePrivateConnectionVpcPeeringConfigEl {}
impl ToListMappable for DatabaseMigrationServicePrivateConnectionVpcPeeringConfigEl {
    type O = BlockAssignable<DatabaseMigrationServicePrivateConnectionVpcPeeringConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatabaseMigrationServicePrivateConnectionVpcPeeringConfigEl {
    #[doc = "A free subnet for peering. (CIDR of /29)"]
    pub subnet: PrimField<String>,
    #[doc = "Fully qualified name of the VPC that Database Migration Service will peer to.\nFormat: projects/{project}/global/{networks}/{name}"]
    pub vpc_name: PrimField<String>,
}
impl BuildDatabaseMigrationServicePrivateConnectionVpcPeeringConfigEl {
    pub fn build(self) -> DatabaseMigrationServicePrivateConnectionVpcPeeringConfigEl {
        DatabaseMigrationServicePrivateConnectionVpcPeeringConfigEl {
            subnet: self.subnet,
            vpc_name: self.vpc_name,
        }
    }
}
pub struct DatabaseMigrationServicePrivateConnectionVpcPeeringConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatabaseMigrationServicePrivateConnectionVpcPeeringConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatabaseMigrationServicePrivateConnectionVpcPeeringConfigElRef {
        DatabaseMigrationServicePrivateConnectionVpcPeeringConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatabaseMigrationServicePrivateConnectionVpcPeeringConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `subnet` after provisioning.\nA free subnet for peering. (CIDR of /29)"]
    pub fn subnet(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subnet", self.base))
    }
    #[doc = "Get a reference to the value of field `vpc_name` after provisioning.\nFully qualified name of the VPC that Database Migration Service will peer to.\nFormat: projects/{project}/global/{networks}/{name}"]
    pub fn vpc_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_name", self.base))
    }
}
#[derive(Serialize, Default)]
struct DatabaseMigrationServicePrivateConnectionDynamic {
    psc_interface_config:
        Option<DynamicBlock<DatabaseMigrationServicePrivateConnectionPscInterfaceConfigEl>>,
    vpc_peering_config:
        Option<DynamicBlock<DatabaseMigrationServicePrivateConnectionVpcPeeringConfigEl>>,
}
