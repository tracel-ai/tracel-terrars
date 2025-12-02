use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct RedshiftserverlessNamespaceData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    admin_password_secret_kms_key_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    admin_user_password: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    admin_user_password_wo: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    admin_user_password_wo_version: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    admin_username: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    db_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_iam_role_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iam_roles: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_exports: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    manage_admin_password: Option<PrimField<bool>>,
    namespace_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
}
struct RedshiftserverlessNamespace_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<RedshiftserverlessNamespaceData>,
}
#[derive(Clone)]
pub struct RedshiftserverlessNamespace(Rc<RedshiftserverlessNamespace_>);
impl RedshiftserverlessNamespace {
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
    #[doc = "Set the field `admin_password_secret_kms_key_id`.\n"]
    pub fn set_admin_password_secret_kms_key_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().admin_password_secret_kms_key_id = Some(v.into());
        self
    }
    #[doc = "Set the field `admin_user_password`.\n"]
    pub fn set_admin_user_password(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().admin_user_password = Some(v.into());
        self
    }
    #[doc = "Set the field `admin_user_password_wo`.\n"]
    pub fn set_admin_user_password_wo(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().admin_user_password_wo = Some(v.into());
        self
    }
    #[doc = "Set the field `admin_user_password_wo_version`.\n"]
    pub fn set_admin_user_password_wo_version(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().admin_user_password_wo_version = Some(v.into());
        self
    }
    #[doc = "Set the field `admin_username`.\n"]
    pub fn set_admin_username(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().admin_username = Some(v.into());
        self
    }
    #[doc = "Set the field `db_name`.\n"]
    pub fn set_db_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().db_name = Some(v.into());
        self
    }
    #[doc = "Set the field `default_iam_role_arn`.\n"]
    pub fn set_default_iam_role_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().default_iam_role_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `iam_roles`.\n"]
    pub fn set_iam_roles(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().iam_roles = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().kms_key_id = Some(v.into());
        self
    }
    #[doc = "Set the field `log_exports`.\n"]
    pub fn set_log_exports(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().log_exports = Some(v.into());
        self
    }
    #[doc = "Set the field `manage_admin_password`.\n"]
    pub fn set_manage_admin_password(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().manage_admin_password = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
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
    #[doc = "Get a reference to the value of field `admin_password_secret_arn` after provisioning.\n"]
    pub fn admin_password_secret_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.admin_password_secret_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `admin_password_secret_kms_key_id` after provisioning.\n"]
    pub fn admin_password_secret_kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.admin_password_secret_kms_key_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `admin_user_password` after provisioning.\n"]
    pub fn admin_user_password(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.admin_user_password", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `admin_user_password_wo` after provisioning.\n"]
    pub fn admin_user_password_wo(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.admin_user_password_wo", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `admin_user_password_wo_version` after provisioning.\n"]
    pub fn admin_user_password_wo_version(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.admin_user_password_wo_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `admin_username` after provisioning.\n"]
    pub fn admin_username(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.admin_username", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `db_name` after provisioning.\n"]
    pub fn db_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.db_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `default_iam_role_arn` after provisioning.\n"]
    pub fn default_iam_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_iam_role_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `iam_roles` after provisioning.\n"]
    pub fn iam_roles(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.iam_roles", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `log_exports` after provisioning.\n"]
    pub fn log_exports(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.log_exports", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `manage_admin_password` after provisioning.\n"]
    pub fn manage_admin_password(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.manage_admin_password", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `namespace_id` after provisioning.\n"]
    pub fn namespace_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.namespace_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `namespace_name` after provisioning.\n"]
    pub fn namespace_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.namespace_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }
}
impl Referable for RedshiftserverlessNamespace {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for RedshiftserverlessNamespace {}
impl ToListMappable for RedshiftserverlessNamespace {
    type O = ListRef<RedshiftserverlessNamespaceRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for RedshiftserverlessNamespace_ {
    fn extract_resource_type(&self) -> String {
        "aws_redshiftserverless_namespace".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildRedshiftserverlessNamespace {
    pub tf_id: String,
    #[doc = ""]
    pub namespace_name: PrimField<String>,
}
impl BuildRedshiftserverlessNamespace {
    pub fn build(self, stack: &mut Stack) -> RedshiftserverlessNamespace {
        let out = RedshiftserverlessNamespace(Rc::new(RedshiftserverlessNamespace_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(RedshiftserverlessNamespaceData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                admin_password_secret_kms_key_id: core::default::Default::default(),
                admin_user_password: core::default::Default::default(),
                admin_user_password_wo: core::default::Default::default(),
                admin_user_password_wo_version: core::default::Default::default(),
                admin_username: core::default::Default::default(),
                db_name: core::default::Default::default(),
                default_iam_role_arn: core::default::Default::default(),
                iam_roles: core::default::Default::default(),
                id: core::default::Default::default(),
                kms_key_id: core::default::Default::default(),
                log_exports: core::default::Default::default(),
                manage_admin_password: core::default::Default::default(),
                namespace_name: self.namespace_name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct RedshiftserverlessNamespaceRef {
    shared: StackShared,
    base: String,
}
impl Ref for RedshiftserverlessNamespaceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl RedshiftserverlessNamespaceRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `admin_password_secret_arn` after provisioning.\n"]
    pub fn admin_password_secret_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.admin_password_secret_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `admin_password_secret_kms_key_id` after provisioning.\n"]
    pub fn admin_password_secret_kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.admin_password_secret_kms_key_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `admin_user_password` after provisioning.\n"]
    pub fn admin_user_password(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.admin_user_password", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `admin_user_password_wo` after provisioning.\n"]
    pub fn admin_user_password_wo(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.admin_user_password_wo", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `admin_user_password_wo_version` after provisioning.\n"]
    pub fn admin_user_password_wo_version(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.admin_user_password_wo_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `admin_username` after provisioning.\n"]
    pub fn admin_username(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.admin_username", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `db_name` after provisioning.\n"]
    pub fn db_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.db_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `default_iam_role_arn` after provisioning.\n"]
    pub fn default_iam_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_iam_role_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `iam_roles` after provisioning.\n"]
    pub fn iam_roles(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.iam_roles", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `log_exports` after provisioning.\n"]
    pub fn log_exports(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.log_exports", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `manage_admin_password` after provisioning.\n"]
    pub fn manage_admin_password(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.manage_admin_password", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `namespace_id` after provisioning.\n"]
    pub fn namespace_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.namespace_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `namespace_name` after provisioning.\n"]
    pub fn namespace_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.namespace_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }
}
