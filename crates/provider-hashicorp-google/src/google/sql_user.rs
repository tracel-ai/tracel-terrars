use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct SqlUserData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    database_roles: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    host: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    instance: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    password: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    password_wo: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    password_wo_version: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    password_policy: Option<Vec<SqlUserPasswordPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<SqlUserTimeoutsEl>,
    dynamic: SqlUserDynamic,
}
struct SqlUser_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SqlUserData>,
}
#[derive(Clone)]
pub struct SqlUser(Rc<SqlUser_>);
impl SqlUser {
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
    #[doc = "Set the field `database_roles`.\nA list of database roles to be assigned to the user. This option is only available for MySQL and PostgreSQL instances."]
    pub fn set_database_roles(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().database_roles = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `host`.\nThe host the user can connect from. This is only supported for MySQL instances. Don't set this field for PostgreSQL instances. Can be an IP address. Changing this forces a new resource to be created."]
    pub fn set_host(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().host = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `password`.\nThe password for the user. Can be updated. For Postgres instances this is a Required field, unless type is set to\n\t\t\t\teither CLOUD_IAM_USER or CLOUD_IAM_SERVICE_ACCOUNT."]
    pub fn set_password(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().password = Some(v.into());
        self
    }
    #[doc = "Set the field `password_wo`.\nThe password for the user. Can be updated. For Postgres instances this is a Required field, unless type is set to\n\t\t\t\teither CLOUD_IAM_USER or CLOUD_IAM_SERVICE_ACCOUNT."]
    pub fn set_password_wo(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().password_wo = Some(v.into());
        self
    }
    #[doc = "Set the field `password_wo_version`.\nThe version of the password_wo."]
    pub fn set_password_wo_version(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().password_wo_version = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\nThe user type. It determines the method to authenticate the user during login.\n\t\t\t\tThe default is the database's built-in user type."]
    pub fn set_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().type_ = Some(v.into());
        self
    }
    #[doc = "Set the field `password_policy`.\n"]
    pub fn set_password_policy(
        self,
        v: impl Into<BlockAssignable<SqlUserPasswordPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().password_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.password_policy = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<SqlUserTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `database_roles` after provisioning.\nA list of database roles to be assigned to the user. This option is only available for MySQL and PostgreSQL instances."]
    pub fn database_roles(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.database_roles", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `host` after provisioning.\nThe host the user can connect from. This is only supported for MySQL instances. Don't set this field for PostgreSQL instances. Can be an IP address. Changing this forces a new resource to be created."]
    pub fn host(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.host", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `iam_email` after provisioning.\nThe email address for MySQL IAM database users."]
    pub fn iam_email(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.iam_email", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `instance` after provisioning.\nThe name of the Cloud SQL instance. Changing this forces a new resource to be created."]
    pub fn instance(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the user. Changing this forces a new resource to be created."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `password` after provisioning.\nThe password for the user. Can be updated. For Postgres instances this is a Required field, unless type is set to\n\t\t\t\teither CLOUD_IAM_USER or CLOUD_IAM_SERVICE_ACCOUNT."]
    pub fn password(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.password", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `password_wo` after provisioning.\nThe password for the user. Can be updated. For Postgres instances this is a Required field, unless type is set to\n\t\t\t\teither CLOUD_IAM_USER or CLOUD_IAM_SERVICE_ACCOUNT."]
    pub fn password_wo(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.password_wo", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `password_wo_version` after provisioning.\nThe version of the password_wo."]
    pub fn password_wo_version(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.password_wo_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `sql_server_user_details` after provisioning.\n"]
    pub fn sql_server_user_details(&self) -> ListRef<SqlUserSqlServerUserDetailsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.sql_server_user_details", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe user type. It determines the method to authenticate the user during login.\n\t\t\t\tThe default is the database's built-in user type."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `password_policy` after provisioning.\n"]
    pub fn password_policy(&self) -> ListRef<SqlUserPasswordPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.password_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SqlUserTimeoutsElRef {
        SqlUserTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for SqlUser {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for SqlUser {}
impl ToListMappable for SqlUser {
    type O = ListRef<SqlUserRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for SqlUser_ {
    fn extract_resource_type(&self) -> String {
        "google_sql_user".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildSqlUser {
    pub tf_id: String,
    #[doc = "The name of the Cloud SQL instance. Changing this forces a new resource to be created."]
    pub instance: PrimField<String>,
    #[doc = "The name of the user. Changing this forces a new resource to be created."]
    pub name: PrimField<String>,
}
impl BuildSqlUser {
    pub fn build(self, stack: &mut Stack) -> SqlUser {
        let out = SqlUser(Rc::new(SqlUser_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(SqlUserData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                database_roles: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                host: core::default::Default::default(),
                id: core::default::Default::default(),
                instance: self.instance,
                name: self.name,
                password: core::default::Default::default(),
                password_wo: core::default::Default::default(),
                password_wo_version: core::default::Default::default(),
                project: core::default::Default::default(),
                type_: core::default::Default::default(),
                password_policy: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct SqlUserRef {
    shared: StackShared,
    base: String,
}
impl Ref for SqlUserRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl SqlUserRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `database_roles` after provisioning.\nA list of database roles to be assigned to the user. This option is only available for MySQL and PostgreSQL instances."]
    pub fn database_roles(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.database_roles", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `host` after provisioning.\nThe host the user can connect from. This is only supported for MySQL instances. Don't set this field for PostgreSQL instances. Can be an IP address. Changing this forces a new resource to be created."]
    pub fn host(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.host", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `iam_email` after provisioning.\nThe email address for MySQL IAM database users."]
    pub fn iam_email(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.iam_email", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `instance` after provisioning.\nThe name of the Cloud SQL instance. Changing this forces a new resource to be created."]
    pub fn instance(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the user. Changing this forces a new resource to be created."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `password` after provisioning.\nThe password for the user. Can be updated. For Postgres instances this is a Required field, unless type is set to\n\t\t\t\teither CLOUD_IAM_USER or CLOUD_IAM_SERVICE_ACCOUNT."]
    pub fn password(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.password", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `password_wo` after provisioning.\nThe password for the user. Can be updated. For Postgres instances this is a Required field, unless type is set to\n\t\t\t\teither CLOUD_IAM_USER or CLOUD_IAM_SERVICE_ACCOUNT."]
    pub fn password_wo(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.password_wo", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `password_wo_version` after provisioning.\nThe version of the password_wo."]
    pub fn password_wo_version(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.password_wo_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `sql_server_user_details` after provisioning.\n"]
    pub fn sql_server_user_details(&self) -> ListRef<SqlUserSqlServerUserDetailsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.sql_server_user_details", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe user type. It determines the method to authenticate the user during login.\n\t\t\t\tThe default is the database's built-in user type."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `password_policy` after provisioning.\n"]
    pub fn password_policy(&self) -> ListRef<SqlUserPasswordPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.password_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SqlUserTimeoutsElRef {
        SqlUserTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct SqlUserSqlServerUserDetailsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    server_roles: Option<ListField<PrimField<String>>>,
}
impl SqlUserSqlServerUserDetailsEl {
    #[doc = "Set the field `disabled`.\n"]
    pub fn set_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disabled = Some(v.into());
        self
    }
    #[doc = "Set the field `server_roles`.\n"]
    pub fn set_server_roles(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.server_roles = Some(v.into());
        self
    }
}
impl ToListMappable for SqlUserSqlServerUserDetailsEl {
    type O = BlockAssignable<SqlUserSqlServerUserDetailsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSqlUserSqlServerUserDetailsEl {}
impl BuildSqlUserSqlServerUserDetailsEl {
    pub fn build(self) -> SqlUserSqlServerUserDetailsEl {
        SqlUserSqlServerUserDetailsEl {
            disabled: core::default::Default::default(),
            server_roles: core::default::Default::default(),
        }
    }
}
pub struct SqlUserSqlServerUserDetailsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SqlUserSqlServerUserDetailsElRef {
    fn new(shared: StackShared, base: String) -> SqlUserSqlServerUserDetailsElRef {
        SqlUserSqlServerUserDetailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SqlUserSqlServerUserDetailsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
    #[doc = "Get a reference to the value of field `server_roles` after provisioning.\n"]
    pub fn server_roles(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.server_roles", self.base))
    }
}
#[derive(Serialize)]
pub struct SqlUserPasswordPolicyElStatusEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    locked: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    password_expiration_time: Option<PrimField<String>>,
}
impl SqlUserPasswordPolicyElStatusEl {
    #[doc = "Set the field `locked`.\n"]
    pub fn set_locked(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.locked = Some(v.into());
        self
    }
    #[doc = "Set the field `password_expiration_time`.\n"]
    pub fn set_password_expiration_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.password_expiration_time = Some(v.into());
        self
    }
}
impl ToListMappable for SqlUserPasswordPolicyElStatusEl {
    type O = BlockAssignable<SqlUserPasswordPolicyElStatusEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSqlUserPasswordPolicyElStatusEl {}
impl BuildSqlUserPasswordPolicyElStatusEl {
    pub fn build(self) -> SqlUserPasswordPolicyElStatusEl {
        SqlUserPasswordPolicyElStatusEl {
            locked: core::default::Default::default(),
            password_expiration_time: core::default::Default::default(),
        }
    }
}
pub struct SqlUserPasswordPolicyElStatusElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SqlUserPasswordPolicyElStatusElRef {
    fn new(shared: StackShared, base: String) -> SqlUserPasswordPolicyElStatusElRef {
        SqlUserPasswordPolicyElStatusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SqlUserPasswordPolicyElStatusElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `locked` after provisioning.\n"]
    pub fn locked(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.locked", self.base))
    }
    #[doc = "Get a reference to the value of field `password_expiration_time` after provisioning.\n"]
    pub fn password_expiration_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.password_expiration_time", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct SqlUserPasswordPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_failed_attempts: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_failed_attempts_check: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_password_verification: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    password_expiration_duration: Option<PrimField<String>>,
}
impl SqlUserPasswordPolicyEl {
    #[doc = "Set the field `allowed_failed_attempts`.\nNumber of failed attempts allowed before the user get locked."]
    pub fn set_allowed_failed_attempts(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.allowed_failed_attempts = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_failed_attempts_check`.\nIf true, the check that will lock user after too many failed login attempts will be enabled."]
    pub fn set_enable_failed_attempts_check(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_failed_attempts_check = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_password_verification`.\nIf true, the user must specify the current password before changing the password. This flag is supported only for MySQL."]
    pub fn set_enable_password_verification(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_password_verification = Some(v.into());
        self
    }
    #[doc = "Set the field `password_expiration_duration`.\nPassword expiration duration with one week grace period."]
    pub fn set_password_expiration_duration(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.password_expiration_duration = Some(v.into());
        self
    }
}
impl ToListMappable for SqlUserPasswordPolicyEl {
    type O = BlockAssignable<SqlUserPasswordPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSqlUserPasswordPolicyEl {}
impl BuildSqlUserPasswordPolicyEl {
    pub fn build(self) -> SqlUserPasswordPolicyEl {
        SqlUserPasswordPolicyEl {
            allowed_failed_attempts: core::default::Default::default(),
            enable_failed_attempts_check: core::default::Default::default(),
            enable_password_verification: core::default::Default::default(),
            password_expiration_duration: core::default::Default::default(),
        }
    }
}
pub struct SqlUserPasswordPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SqlUserPasswordPolicyElRef {
    fn new(shared: StackShared, base: String) -> SqlUserPasswordPolicyElRef {
        SqlUserPasswordPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SqlUserPasswordPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allowed_failed_attempts` after provisioning.\nNumber of failed attempts allowed before the user get locked."]
    pub fn allowed_failed_attempts(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allowed_failed_attempts", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_failed_attempts_check` after provisioning.\nIf true, the check that will lock user after too many failed login attempts will be enabled."]
    pub fn enable_failed_attempts_check(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_failed_attempts_check", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_password_verification` after provisioning.\nIf true, the user must specify the current password before changing the password. This flag is supported only for MySQL."]
    pub fn enable_password_verification(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_password_verification", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `password_expiration_duration` after provisioning.\nPassword expiration duration with one week grace period."]
    pub fn password_expiration_duration(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.password_expiration_duration", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> ListRef<SqlUserPasswordPolicyElStatusElRef> {
        ListRef::new(self.shared().clone(), format!("{}.status", self.base))
    }
}
#[derive(Serialize)]
pub struct SqlUserTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl SqlUserTimeoutsEl {
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
impl ToListMappable for SqlUserTimeoutsEl {
    type O = BlockAssignable<SqlUserTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSqlUserTimeoutsEl {}
impl BuildSqlUserTimeoutsEl {
    pub fn build(self) -> SqlUserTimeoutsEl {
        SqlUserTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct SqlUserTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SqlUserTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> SqlUserTimeoutsElRef {
        SqlUserTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SqlUserTimeoutsElRef {
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
struct SqlUserDynamic {
    password_policy: Option<DynamicBlock<SqlUserPasswordPolicyEl>>,
}
