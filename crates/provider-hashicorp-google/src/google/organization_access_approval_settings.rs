use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct OrganizationAccessApprovalSettingsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    active_key_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notification_emails: Option<SetField<PrimField<String>>>,
    organization_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enrolled_services: Option<Vec<OrganizationAccessApprovalSettingsEnrolledServicesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<OrganizationAccessApprovalSettingsTimeoutsEl>,
    dynamic: OrganizationAccessApprovalSettingsDynamic,
}
struct OrganizationAccessApprovalSettings_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<OrganizationAccessApprovalSettingsData>,
}
#[derive(Clone)]
pub struct OrganizationAccessApprovalSettings(Rc<OrganizationAccessApprovalSettings_>);
impl OrganizationAccessApprovalSettings {
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
    #[doc = "Set the field `active_key_version`.\nThe asymmetric crypto key version to use for signing approval requests.\nEmpty active_key_version indicates that a Google-managed key should be used for signing."]
    pub fn set_active_key_version(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().active_key_version = Some(v.into());
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
    #[doc = "Set the field `notification_emails`.\nA list of email addresses to which notifications relating to approval requests should be sent.\nNotifications relating to a resource will be sent to all emails in the settings of ancestor\nresources of that resource. A maximum of 50 email addresses are allowed."]
    pub fn set_notification_emails(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().notification_emails = Some(v.into());
        self
    }
    #[doc = "Set the field `enrolled_services`.\n"]
    pub fn set_enrolled_services(
        self,
        v: impl Into<BlockAssignable<OrganizationAccessApprovalSettingsEnrolledServicesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().enrolled_services = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.enrolled_services = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<OrganizationAccessApprovalSettingsTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `active_key_version` after provisioning.\nThe asymmetric crypto key version to use for signing approval requests.\nEmpty active_key_version indicates that a Google-managed key should be used for signing."]
    pub fn active_key_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.active_key_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ancestor_has_active_key_version` after provisioning.\nThis field will always be unset for the organization since organizations do not have ancestors."]
    pub fn ancestor_has_active_key_version(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ancestor_has_active_key_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enrolled_ancestor` after provisioning.\nThis field will always be unset for the organization since organizations do not have ancestors."]
    pub fn enrolled_ancestor(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enrolled_ancestor", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `invalid_key_version` after provisioning.\nIf the field is true, that indicates that there is some configuration issue with the active_key_version\nconfigured on this Organization (e.g. it doesn't exist or the Access Approval service account doesn't have the\ncorrect permissions on it, etc.)."]
    pub fn invalid_key_version(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.invalid_key_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the settings. Format is \"organizations/{organization_id}/accessApprovalSettings\""]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `notification_emails` after provisioning.\nA list of email addresses to which notifications relating to approval requests should be sent.\nNotifications relating to a resource will be sent to all emails in the settings of ancestor\nresources of that resource. A maximum of 50 email addresses are allowed."]
    pub fn notification_emails(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.notification_emails", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `organization_id` after provisioning.\nID of the organization of the access approval settings."]
    pub fn organization_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.organization_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> OrganizationAccessApprovalSettingsTimeoutsElRef {
        OrganizationAccessApprovalSettingsTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for OrganizationAccessApprovalSettings {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for OrganizationAccessApprovalSettings {}
impl ToListMappable for OrganizationAccessApprovalSettings {
    type O = ListRef<OrganizationAccessApprovalSettingsRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for OrganizationAccessApprovalSettings_ {
    fn extract_resource_type(&self) -> String {
        "google_organization_access_approval_settings".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildOrganizationAccessApprovalSettings {
    pub tf_id: String,
    #[doc = "ID of the organization of the access approval settings."]
    pub organization_id: PrimField<String>,
}
impl BuildOrganizationAccessApprovalSettings {
    pub fn build(self, stack: &mut Stack) -> OrganizationAccessApprovalSettings {
        let out =
            OrganizationAccessApprovalSettings(Rc::new(OrganizationAccessApprovalSettings_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(OrganizationAccessApprovalSettingsData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    lifecycle: core::default::Default::default(),
                    for_each: None,
                    active_key_version: core::default::Default::default(),
                    deletion_policy: core::default::Default::default(),
                    id: core::default::Default::default(),
                    notification_emails: core::default::Default::default(),
                    organization_id: self.organization_id,
                    enrolled_services: core::default::Default::default(),
                    timeouts: core::default::Default::default(),
                    dynamic: Default::default(),
                }),
            }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct OrganizationAccessApprovalSettingsRef {
    shared: StackShared,
    base: String,
}
impl Ref for OrganizationAccessApprovalSettingsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl OrganizationAccessApprovalSettingsRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `active_key_version` after provisioning.\nThe asymmetric crypto key version to use for signing approval requests.\nEmpty active_key_version indicates that a Google-managed key should be used for signing."]
    pub fn active_key_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.active_key_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ancestor_has_active_key_version` after provisioning.\nThis field will always be unset for the organization since organizations do not have ancestors."]
    pub fn ancestor_has_active_key_version(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ancestor_has_active_key_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enrolled_ancestor` after provisioning.\nThis field will always be unset for the organization since organizations do not have ancestors."]
    pub fn enrolled_ancestor(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enrolled_ancestor", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `invalid_key_version` after provisioning.\nIf the field is true, that indicates that there is some configuration issue with the active_key_version\nconfigured on this Organization (e.g. it doesn't exist or the Access Approval service account doesn't have the\ncorrect permissions on it, etc.)."]
    pub fn invalid_key_version(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.invalid_key_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the settings. Format is \"organizations/{organization_id}/accessApprovalSettings\""]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `notification_emails` after provisioning.\nA list of email addresses to which notifications relating to approval requests should be sent.\nNotifications relating to a resource will be sent to all emails in the settings of ancestor\nresources of that resource. A maximum of 50 email addresses are allowed."]
    pub fn notification_emails(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.notification_emails", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `organization_id` after provisioning.\nID of the organization of the access approval settings."]
    pub fn organization_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.organization_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> OrganizationAccessApprovalSettingsTimeoutsElRef {
        OrganizationAccessApprovalSettingsTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct OrganizationAccessApprovalSettingsEnrolledServicesEl {
    cloud_product: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enrollment_level: Option<PrimField<String>>,
}
impl OrganizationAccessApprovalSettingsEnrolledServicesEl {
    #[doc = "Set the field `enrollment_level`.\nThe enrollment level of the service. Default value: \"BLOCK_ALL\" Possible values: [\"BLOCK_ALL\"]"]
    pub fn set_enrollment_level(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.enrollment_level = Some(v.into());
        self
    }
}
impl ToListMappable for OrganizationAccessApprovalSettingsEnrolledServicesEl {
    type O = BlockAssignable<OrganizationAccessApprovalSettingsEnrolledServicesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOrganizationAccessApprovalSettingsEnrolledServicesEl {
    #[doc = "The product for which Access Approval will be enrolled. Allowed values are listed (case-sensitive):\n  all\n  appengine.googleapis.com\n  bigquery.googleapis.com\n  bigtable.googleapis.com\n  cloudkms.googleapis.com\n  compute.googleapis.com\n  dataflow.googleapis.com\n  iam.googleapis.com\n  pubsub.googleapis.com\n  storage.googleapis.com"]
    pub cloud_product: PrimField<String>,
}
impl BuildOrganizationAccessApprovalSettingsEnrolledServicesEl {
    pub fn build(self) -> OrganizationAccessApprovalSettingsEnrolledServicesEl {
        OrganizationAccessApprovalSettingsEnrolledServicesEl {
            cloud_product: self.cloud_product,
            enrollment_level: core::default::Default::default(),
        }
    }
}
pub struct OrganizationAccessApprovalSettingsEnrolledServicesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OrganizationAccessApprovalSettingsEnrolledServicesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> OrganizationAccessApprovalSettingsEnrolledServicesElRef {
        OrganizationAccessApprovalSettingsEnrolledServicesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OrganizationAccessApprovalSettingsEnrolledServicesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cloud_product` after provisioning.\nThe product for which Access Approval will be enrolled. Allowed values are listed (case-sensitive):\n  all\n  appengine.googleapis.com\n  bigquery.googleapis.com\n  bigtable.googleapis.com\n  cloudkms.googleapis.com\n  compute.googleapis.com\n  dataflow.googleapis.com\n  iam.googleapis.com\n  pubsub.googleapis.com\n  storage.googleapis.com"]
    pub fn cloud_product(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cloud_product", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enrollment_level` after provisioning.\nThe enrollment level of the service. Default value: \"BLOCK_ALL\" Possible values: [\"BLOCK_ALL\"]"]
    pub fn enrollment_level(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enrollment_level", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct OrganizationAccessApprovalSettingsTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl OrganizationAccessApprovalSettingsTimeoutsEl {
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
impl ToListMappable for OrganizationAccessApprovalSettingsTimeoutsEl {
    type O = BlockAssignable<OrganizationAccessApprovalSettingsTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOrganizationAccessApprovalSettingsTimeoutsEl {}
impl BuildOrganizationAccessApprovalSettingsTimeoutsEl {
    pub fn build(self) -> OrganizationAccessApprovalSettingsTimeoutsEl {
        OrganizationAccessApprovalSettingsTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct OrganizationAccessApprovalSettingsTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OrganizationAccessApprovalSettingsTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> OrganizationAccessApprovalSettingsTimeoutsElRef {
        OrganizationAccessApprovalSettingsTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OrganizationAccessApprovalSettingsTimeoutsElRef {
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
struct OrganizationAccessApprovalSettingsDynamic {
    enrolled_services: Option<DynamicBlock<OrganizationAccessApprovalSettingsEnrolledServicesEl>>,
}
