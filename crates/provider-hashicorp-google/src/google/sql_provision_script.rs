use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct SqlProvisionScriptData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    database: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    instance: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    script: PrimField<String>,
}
struct SqlProvisionScript_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SqlProvisionScriptData>,
}
#[derive(Clone)]
pub struct SqlProvisionScript(Rc<SqlProvisionScript_>);
impl SqlProvisionScript {
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
    #[doc = "Set the field `database`.\nThe name of the database to which Terraform connects. Changing\n\t\t\t\tthis forces Terraform to connect to the new database and run the script. This argument is\n\t\t\t\trequired for Postgres instances. It's optional for MySQL, but some of your queries may require\n\t\t\t\ta database. You can create and use a database in the script or explicitly reference a\n\t\t\t\tgoogle_sql_database."]
    pub fn set_database(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().database = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nThe deletion policy for the resources created by the script. The default is \"ABANDON\".\n\t\t\t\tIt must be \"ABANDON\" to allow Terraform to abandon the resources. If you want to delete resources, add statements\n\t\t\t\tin the script such as \"drop … if exists\"."]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nThe description of the provision script."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `database` after provisioning.\nThe name of the database to which Terraform connects. Changing\n\t\t\t\tthis forces Terraform to connect to the new database and run the script. This argument is\n\t\t\t\trequired for Postgres instances. It's optional for MySQL, but some of your queries may require\n\t\t\t\ta database. You can create and use a database in the script or explicitly reference a\n\t\t\t\tgoogle_sql_database."]
    pub fn database(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.database", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nThe deletion policy for the resources created by the script. The default is \"ABANDON\".\n\t\t\t\tIt must be \"ABANDON\" to allow Terraform to abandon the resources. If you want to delete resources, add statements\n\t\t\t\tin the script such as \"drop … if exists\"."]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe description of the provision script."]
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
    #[doc = "Get a reference to the value of field `instance` after provisioning.\nThe name of the Cloud SQL instance. Changing this forces the script to be run on the new instance."]
    pub fn instance(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `script` after provisioning.\nThe SQL script to provision database resources. Its execution time limit is 30 s.\n\t\t\t\tChanging this forces the script to be rerun. Make sure the script is idempotent.\n\t\t\t\tYou can use statements like \"create if not exists …\" or\n\t\t\t\t\"if not exists (select …) then … end if\" to prevent existence-related errors. If it's not\n\t\t\t\tpossible to make a statement idempotent, you can run it once and then remove it from this script."]
    pub fn script(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.script", self.extract_ref()),
        )
    }
}
impl Referable for SqlProvisionScript {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for SqlProvisionScript {}
impl ToListMappable for SqlProvisionScript {
    type O = ListRef<SqlProvisionScriptRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for SqlProvisionScript_ {
    fn extract_resource_type(&self) -> String {
        "google_sql_provision_script".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildSqlProvisionScript {
    pub tf_id: String,
    #[doc = "The name of the Cloud SQL instance. Changing this forces the script to be run on the new instance."]
    pub instance: PrimField<String>,
    #[doc = "The SQL script to provision database resources. Its execution time limit is 30 s.\n\t\t\t\tChanging this forces the script to be rerun. Make sure the script is idempotent.\n\t\t\t\tYou can use statements like \"create if not exists …\" or\n\t\t\t\t\"if not exists (select …) then … end if\" to prevent existence-related errors. If it's not\n\t\t\t\tpossible to make a statement idempotent, you can run it once and then remove it from this script."]
    pub script: PrimField<String>,
}
impl BuildSqlProvisionScript {
    pub fn build(self, stack: &mut Stack) -> SqlProvisionScript {
        let out = SqlProvisionScript(Rc::new(SqlProvisionScript_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(SqlProvisionScriptData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                database: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                instance: self.instance,
                project: core::default::Default::default(),
                script: self.script,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct SqlProvisionScriptRef {
    shared: StackShared,
    base: String,
}
impl Ref for SqlProvisionScriptRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl SqlProvisionScriptRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `database` after provisioning.\nThe name of the database to which Terraform connects. Changing\n\t\t\t\tthis forces Terraform to connect to the new database and run the script. This argument is\n\t\t\t\trequired for Postgres instances. It's optional for MySQL, but some of your queries may require\n\t\t\t\ta database. You can create and use a database in the script or explicitly reference a\n\t\t\t\tgoogle_sql_database."]
    pub fn database(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.database", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nThe deletion policy for the resources created by the script. The default is \"ABANDON\".\n\t\t\t\tIt must be \"ABANDON\" to allow Terraform to abandon the resources. If you want to delete resources, add statements\n\t\t\t\tin the script such as \"drop … if exists\"."]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe description of the provision script."]
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
    #[doc = "Get a reference to the value of field `instance` after provisioning.\nThe name of the Cloud SQL instance. Changing this forces the script to be run on the new instance."]
    pub fn instance(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `script` after provisioning.\nThe SQL script to provision database resources. Its execution time limit is 30 s.\n\t\t\t\tChanging this forces the script to be rerun. Make sure the script is idempotent.\n\t\t\t\tYou can use statements like \"create if not exists …\" or\n\t\t\t\t\"if not exists (select …) then … end if\" to prevent existence-related errors. If it's not\n\t\t\t\tpossible to make a statement idempotent, you can run it once and then remove it from this script."]
    pub fn script(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.script", self.extract_ref()),
        )
    }
}
