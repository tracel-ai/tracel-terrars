use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct OrganizationsAccountData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    close_on_deletion: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    create_govcloud: Option<PrimField<bool>>,
    email: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iam_user_access_to_billing: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    role_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<OrganizationsAccountTimeoutsEl>,
}

struct OrganizationsAccount_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<OrganizationsAccountData>,
}

#[derive(Clone)]
pub struct OrganizationsAccount(Rc<OrganizationsAccount_>);

impl OrganizationsAccount {
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
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
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc = "Set the field `close_on_deletion`.\n"]
    pub fn set_close_on_deletion(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().close_on_deletion = Some(v.into());
        self
    }

    #[doc = "Set the field `create_govcloud`.\n"]
    pub fn set_create_govcloud(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().create_govcloud = Some(v.into());
        self
    }

    #[doc = "Set the field `iam_user_access_to_billing`.\n"]
    pub fn set_iam_user_access_to_billing(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().iam_user_access_to_billing = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `parent_id`.\n"]
    pub fn set_parent_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().parent_id = Some(v.into());
        self
    }

    #[doc = "Set the field `role_name`.\n"]
    pub fn set_role_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().role_name = Some(v.into());
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

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<OrganizationsAccountTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `close_on_deletion` after provisioning.\n"]
    pub fn close_on_deletion(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.close_on_deletion", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `create_govcloud` after provisioning.\n"]
    pub fn create_govcloud(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_govcloud", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.email", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `govcloud_id` after provisioning.\n"]
    pub fn govcloud_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.govcloud_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `iam_user_access_to_billing` after provisioning.\n"]
    pub fn iam_user_access_to_billing(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.iam_user_access_to_billing", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `joined_method` after provisioning.\n"]
    pub fn joined_method(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.joined_method", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `joined_timestamp` after provisioning.\n"]
    pub fn joined_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.joined_timestamp", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parent_id` after provisioning.\n"]
    pub fn parent_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.parent_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role_name` after provisioning.\n"]
    pub fn role_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> OrganizationsAccountTimeoutsElRef {
        OrganizationsAccountTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for OrganizationsAccount {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for OrganizationsAccount { }

impl ToListMappable for OrganizationsAccount {
    type O = ListRef<OrganizationsAccountRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for OrganizationsAccount_ {
    fn extract_resource_type(&self) -> String {
        "aws_organizations_account".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildOrganizationsAccount {
    pub tf_id: String,
    #[doc = ""]
    pub email: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildOrganizationsAccount {
    pub fn build(self, stack: &mut Stack) -> OrganizationsAccount {
        let out = OrganizationsAccount(Rc::new(OrganizationsAccount_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(OrganizationsAccountData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                close_on_deletion: core::default::Default::default(),
                create_govcloud: core::default::Default::default(),
                email: self.email,
                iam_user_access_to_billing: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                parent_id: core::default::Default::default(),
                role_name: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct OrganizationsAccountRef {
    shared: StackShared,
    base: String,
}

impl Ref for OrganizationsAccountRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl OrganizationsAccountRef {
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

    #[doc = "Get a reference to the value of field `close_on_deletion` after provisioning.\n"]
    pub fn close_on_deletion(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.close_on_deletion", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `create_govcloud` after provisioning.\n"]
    pub fn create_govcloud(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_govcloud", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.email", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `govcloud_id` after provisioning.\n"]
    pub fn govcloud_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.govcloud_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `iam_user_access_to_billing` after provisioning.\n"]
    pub fn iam_user_access_to_billing(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.iam_user_access_to_billing", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `joined_method` after provisioning.\n"]
    pub fn joined_method(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.joined_method", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `joined_timestamp` after provisioning.\n"]
    pub fn joined_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.joined_timestamp", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parent_id` after provisioning.\n"]
    pub fn parent_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.parent_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role_name` after provisioning.\n"]
    pub fn role_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> OrganizationsAccountTimeoutsElRef {
        OrganizationsAccountTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct OrganizationsAccountTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl OrganizationsAccountTimeoutsEl {
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

impl ToListMappable for OrganizationsAccountTimeoutsEl {
    type O = BlockAssignable<OrganizationsAccountTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOrganizationsAccountTimeoutsEl {}

impl BuildOrganizationsAccountTimeoutsEl {
    pub fn build(self) -> OrganizationsAccountTimeoutsEl {
        OrganizationsAccountTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct OrganizationsAccountTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for OrganizationsAccountTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> OrganizationsAccountTimeoutsElRef {
        OrganizationsAccountTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OrganizationsAccountTimeoutsElRef {
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
