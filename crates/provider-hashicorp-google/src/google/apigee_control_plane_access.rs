use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ApigeeControlPlaneAccessData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    analytics_publisher_identities: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    synchronizer_identities: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ApigeeControlPlaneAccessTimeoutsEl>,
}
struct ApigeeControlPlaneAccess_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ApigeeControlPlaneAccessData>,
}
#[derive(Clone)]
pub struct ApigeeControlPlaneAccess(Rc<ApigeeControlPlaneAccess_>);
impl ApigeeControlPlaneAccess {
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
    #[doc = "Set the field `analytics_publisher_identities`.\nArray of service accounts authorized to publish analytics data to the control plane, each specified using the following format: 'serviceAccount:service-account-name'.\n\nThe 'service-account-name' is formatted like an email address. For example: serviceAccount@my_project_id.iam.gserviceaccount.com\n\nYou might specify multiple service accounts, for example, if you have multiple environments and wish to assign a unique service account to each one."]
    pub fn set_analytics_publisher_identities(
        self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.0.data.borrow_mut().analytics_publisher_identities = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `synchronizer_identities`.\nArray of service accounts to grant access to control plane resources (for the Synchronizer component), each specified using the following format: 'serviceAccount:service-account-name'.\n\nThe 'service-account-name' is formatted like an email address. For example: serviceAccount@my_project_id.iam.gserviceaccount.com\n\nYou might specify multiple service accounts, for example, if you have multiple environments and wish to assign a unique service account to each one.\n\nThe service accounts must have **Apigee Synchronizer Manager** role. See also [Create service accounts](https://cloud.google.com/apigee/docs/hybrid/v1.8/sa-about#create-the-service-accounts)."]
    pub fn set_synchronizer_identities(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().synchronizer_identities = Some(v.into());
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ApigeeControlPlaneAccessTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `analytics_publisher_identities` after provisioning.\nArray of service accounts authorized to publish analytics data to the control plane, each specified using the following format: 'serviceAccount:service-account-name'.\n\nThe 'service-account-name' is formatted like an email address. For example: serviceAccount@my_project_id.iam.gserviceaccount.com\n\nYou might specify multiple service accounts, for example, if you have multiple environments and wish to assign a unique service account to each one."]
    pub fn analytics_publisher_identities(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.analytics_publisher_identities", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the Apigee organization."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `synchronizer_identities` after provisioning.\nArray of service accounts to grant access to control plane resources (for the Synchronizer component), each specified using the following format: 'serviceAccount:service-account-name'.\n\nThe 'service-account-name' is formatted like an email address. For example: serviceAccount@my_project_id.iam.gserviceaccount.com\n\nYou might specify multiple service accounts, for example, if you have multiple environments and wish to assign a unique service account to each one.\n\nThe service accounts must have **Apigee Synchronizer Manager** role. See also [Create service accounts](https://cloud.google.com/apigee/docs/hybrid/v1.8/sa-about#create-the-service-accounts)."]
    pub fn synchronizer_identities(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.synchronizer_identities", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ApigeeControlPlaneAccessTimeoutsElRef {
        ApigeeControlPlaneAccessTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ApigeeControlPlaneAccess {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ApigeeControlPlaneAccess {}
impl ToListMappable for ApigeeControlPlaneAccess {
    type O = ListRef<ApigeeControlPlaneAccessRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ApigeeControlPlaneAccess_ {
    fn extract_resource_type(&self) -> String {
        "google_apigee_control_plane_access".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildApigeeControlPlaneAccess {
    pub tf_id: String,
    #[doc = "Name of the Apigee organization."]
    pub name: PrimField<String>,
}
impl BuildApigeeControlPlaneAccess {
    pub fn build(self, stack: &mut Stack) -> ApigeeControlPlaneAccess {
        let out = ApigeeControlPlaneAccess(Rc::new(ApigeeControlPlaneAccess_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ApigeeControlPlaneAccessData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                analytics_publisher_identities: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                synchronizer_identities: core::default::Default::default(),
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ApigeeControlPlaneAccessRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApigeeControlPlaneAccessRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ApigeeControlPlaneAccessRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `analytics_publisher_identities` after provisioning.\nArray of service accounts authorized to publish analytics data to the control plane, each specified using the following format: 'serviceAccount:service-account-name'.\n\nThe 'service-account-name' is formatted like an email address. For example: serviceAccount@my_project_id.iam.gserviceaccount.com\n\nYou might specify multiple service accounts, for example, if you have multiple environments and wish to assign a unique service account to each one."]
    pub fn analytics_publisher_identities(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.analytics_publisher_identities", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the Apigee organization."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `synchronizer_identities` after provisioning.\nArray of service accounts to grant access to control plane resources (for the Synchronizer component), each specified using the following format: 'serviceAccount:service-account-name'.\n\nThe 'service-account-name' is formatted like an email address. For example: serviceAccount@my_project_id.iam.gserviceaccount.com\n\nYou might specify multiple service accounts, for example, if you have multiple environments and wish to assign a unique service account to each one.\n\nThe service accounts must have **Apigee Synchronizer Manager** role. See also [Create service accounts](https://cloud.google.com/apigee/docs/hybrid/v1.8/sa-about#create-the-service-accounts)."]
    pub fn synchronizer_identities(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.synchronizer_identities", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ApigeeControlPlaneAccessTimeoutsElRef {
        ApigeeControlPlaneAccessTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ApigeeControlPlaneAccessTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ApigeeControlPlaneAccessTimeoutsEl {
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
impl ToListMappable for ApigeeControlPlaneAccessTimeoutsEl {
    type O = BlockAssignable<ApigeeControlPlaneAccessTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApigeeControlPlaneAccessTimeoutsEl {}
impl BuildApigeeControlPlaneAccessTimeoutsEl {
    pub fn build(self) -> ApigeeControlPlaneAccessTimeoutsEl {
        ApigeeControlPlaneAccessTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ApigeeControlPlaneAccessTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApigeeControlPlaneAccessTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ApigeeControlPlaneAccessTimeoutsElRef {
        ApigeeControlPlaneAccessTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApigeeControlPlaneAccessTimeoutsElRef {
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
