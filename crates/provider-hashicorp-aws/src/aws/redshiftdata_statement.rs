use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct RedshiftdataStatementData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cluster_identifier: Option<PrimField<String>>,
    database: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    db_user: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_arn: Option<PrimField<String>>,
    sql: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    statement_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    with_event: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    workgroup_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameters: Option<Vec<RedshiftdataStatementParametersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<RedshiftdataStatementTimeoutsEl>,
    dynamic: RedshiftdataStatementDynamic,
}
struct RedshiftdataStatement_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<RedshiftdataStatementData>,
}
#[derive(Clone)]
pub struct RedshiftdataStatement(Rc<RedshiftdataStatement_>);
impl RedshiftdataStatement {
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
    #[doc = "Set the field `cluster_identifier`.\n"]
    pub fn set_cluster_identifier(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().cluster_identifier = Some(v.into());
        self
    }
    #[doc = "Set the field `db_user`.\n"]
    pub fn set_db_user(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().db_user = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `secret_arn`.\n"]
    pub fn set_secret_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().secret_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `statement_name`.\n"]
    pub fn set_statement_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().statement_name = Some(v.into());
        self
    }
    #[doc = "Set the field `with_event`.\n"]
    pub fn set_with_event(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().with_event = Some(v.into());
        self
    }
    #[doc = "Set the field `workgroup_name`.\n"]
    pub fn set_workgroup_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().workgroup_name = Some(v.into());
        self
    }
    #[doc = "Set the field `parameters`.\n"]
    pub fn set_parameters(
        self,
        v: impl Into<BlockAssignable<RedshiftdataStatementParametersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().parameters = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.parameters = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<RedshiftdataStatementTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `cluster_identifier` after provisioning.\n"]
    pub fn cluster_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster_identifier", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `database` after provisioning.\n"]
    pub fn database(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.database", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `db_user` after provisioning.\n"]
    pub fn db_user(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.db_user", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `secret_arn` after provisioning.\n"]
    pub fn secret_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.secret_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `sql` after provisioning.\n"]
    pub fn sql(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sql", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `statement_name` after provisioning.\n"]
    pub fn statement_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.statement_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `with_event` after provisioning.\n"]
    pub fn with_event(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.with_event", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `workgroup_name` after provisioning.\n"]
    pub fn workgroup_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.workgroup_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> ListRef<RedshiftdataStatementParametersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.parameters", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> RedshiftdataStatementTimeoutsElRef {
        RedshiftdataStatementTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for RedshiftdataStatement {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for RedshiftdataStatement {}
impl ToListMappable for RedshiftdataStatement {
    type O = ListRef<RedshiftdataStatementRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for RedshiftdataStatement_ {
    fn extract_resource_type(&self) -> String {
        "aws_redshiftdata_statement".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildRedshiftdataStatement {
    pub tf_id: String,
    #[doc = ""]
    pub database: PrimField<String>,
    #[doc = ""]
    pub sql: PrimField<String>,
}
impl BuildRedshiftdataStatement {
    pub fn build(self, stack: &mut Stack) -> RedshiftdataStatement {
        let out = RedshiftdataStatement(Rc::new(RedshiftdataStatement_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(RedshiftdataStatementData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                cluster_identifier: core::default::Default::default(),
                database: self.database,
                db_user: core::default::Default::default(),
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                secret_arn: core::default::Default::default(),
                sql: self.sql,
                statement_name: core::default::Default::default(),
                with_event: core::default::Default::default(),
                workgroup_name: core::default::Default::default(),
                parameters: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct RedshiftdataStatementRef {
    shared: StackShared,
    base: String,
}
impl Ref for RedshiftdataStatementRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl RedshiftdataStatementRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cluster_identifier` after provisioning.\n"]
    pub fn cluster_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster_identifier", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `database` after provisioning.\n"]
    pub fn database(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.database", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `db_user` after provisioning.\n"]
    pub fn db_user(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.db_user", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `secret_arn` after provisioning.\n"]
    pub fn secret_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.secret_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `sql` after provisioning.\n"]
    pub fn sql(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sql", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `statement_name` after provisioning.\n"]
    pub fn statement_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.statement_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `with_event` after provisioning.\n"]
    pub fn with_event(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.with_event", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `workgroup_name` after provisioning.\n"]
    pub fn workgroup_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.workgroup_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> ListRef<RedshiftdataStatementParametersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.parameters", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> RedshiftdataStatementTimeoutsElRef {
        RedshiftdataStatementTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct RedshiftdataStatementParametersEl {
    name: PrimField<String>,
    value: PrimField<String>,
}
impl RedshiftdataStatementParametersEl {}
impl ToListMappable for RedshiftdataStatementParametersEl {
    type O = BlockAssignable<RedshiftdataStatementParametersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildRedshiftdataStatementParametersEl {
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildRedshiftdataStatementParametersEl {
    pub fn build(self) -> RedshiftdataStatementParametersEl {
        RedshiftdataStatementParametersEl {
            name: self.name,
            value: self.value,
        }
    }
}
pub struct RedshiftdataStatementParametersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for RedshiftdataStatementParametersElRef {
    fn new(shared: StackShared, base: String) -> RedshiftdataStatementParametersElRef {
        RedshiftdataStatementParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl RedshiftdataStatementParametersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct RedshiftdataStatementTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
}
impl RedshiftdataStatementTimeoutsEl {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }
}
impl ToListMappable for RedshiftdataStatementTimeoutsEl {
    type O = BlockAssignable<RedshiftdataStatementTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildRedshiftdataStatementTimeoutsEl {}
impl BuildRedshiftdataStatementTimeoutsEl {
    pub fn build(self) -> RedshiftdataStatementTimeoutsEl {
        RedshiftdataStatementTimeoutsEl {
            create: core::default::Default::default(),
        }
    }
}
pub struct RedshiftdataStatementTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for RedshiftdataStatementTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> RedshiftdataStatementTimeoutsElRef {
        RedshiftdataStatementTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl RedshiftdataStatementTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }
}
#[derive(Serialize, Default)]
struct RedshiftdataStatementDynamic {
    parameters: Option<DynamicBlock<RedshiftdataStatementParametersEl>>,
}
