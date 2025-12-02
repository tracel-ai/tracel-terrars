use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DbProxyDefaultTargetGroupData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    db_proxy_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    connection_pool_config: Option<Vec<DbProxyDefaultTargetGroupConnectionPoolConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DbProxyDefaultTargetGroupTimeoutsEl>,
    dynamic: DbProxyDefaultTargetGroupDynamic,
}
struct DbProxyDefaultTargetGroup_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DbProxyDefaultTargetGroupData>,
}
#[derive(Clone)]
pub struct DbProxyDefaultTargetGroup(Rc<DbProxyDefaultTargetGroup_>);
impl DbProxyDefaultTargetGroup {
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
    #[doc = "Set the field `connection_pool_config`.\n"]
    pub fn set_connection_pool_config(
        self,
        v: impl Into<BlockAssignable<DbProxyDefaultTargetGroupConnectionPoolConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().connection_pool_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.connection_pool_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DbProxyDefaultTargetGroupTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `db_proxy_name` after provisioning.\n"]
    pub fn db_proxy_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.db_proxy_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `connection_pool_config` after provisioning.\n"]
    pub fn connection_pool_config(
        &self,
    ) -> ListRef<DbProxyDefaultTargetGroupConnectionPoolConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.connection_pool_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DbProxyDefaultTargetGroupTimeoutsElRef {
        DbProxyDefaultTargetGroupTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for DbProxyDefaultTargetGroup {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for DbProxyDefaultTargetGroup {}
impl ToListMappable for DbProxyDefaultTargetGroup {
    type O = ListRef<DbProxyDefaultTargetGroupRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for DbProxyDefaultTargetGroup_ {
    fn extract_resource_type(&self) -> String {
        "aws_db_proxy_default_target_group".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDbProxyDefaultTargetGroup {
    pub tf_id: String,
    #[doc = ""]
    pub db_proxy_name: PrimField<String>,
}
impl BuildDbProxyDefaultTargetGroup {
    pub fn build(self, stack: &mut Stack) -> DbProxyDefaultTargetGroup {
        let out = DbProxyDefaultTargetGroup(Rc::new(DbProxyDefaultTargetGroup_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DbProxyDefaultTargetGroupData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                db_proxy_name: self.db_proxy_name,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                connection_pool_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct DbProxyDefaultTargetGroupRef {
    shared: StackShared,
    base: String,
}
impl Ref for DbProxyDefaultTargetGroupRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DbProxyDefaultTargetGroupRef {
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
    #[doc = "Get a reference to the value of field `db_proxy_name` after provisioning.\n"]
    pub fn db_proxy_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.db_proxy_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `connection_pool_config` after provisioning.\n"]
    pub fn connection_pool_config(
        &self,
    ) -> ListRef<DbProxyDefaultTargetGroupConnectionPoolConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.connection_pool_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DbProxyDefaultTargetGroupTimeoutsElRef {
        DbProxyDefaultTargetGroupTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DbProxyDefaultTargetGroupConnectionPoolConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    connection_borrow_timeout: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    init_query: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_connections_percent: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_idle_connections_percent: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    session_pinning_filters: Option<SetField<PrimField<String>>>,
}
impl DbProxyDefaultTargetGroupConnectionPoolConfigEl {
    #[doc = "Set the field `connection_borrow_timeout`.\n"]
    pub fn set_connection_borrow_timeout(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.connection_borrow_timeout = Some(v.into());
        self
    }
    #[doc = "Set the field `init_query`.\n"]
    pub fn set_init_query(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.init_query = Some(v.into());
        self
    }
    #[doc = "Set the field `max_connections_percent`.\n"]
    pub fn set_max_connections_percent(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_connections_percent = Some(v.into());
        self
    }
    #[doc = "Set the field `max_idle_connections_percent`.\n"]
    pub fn set_max_idle_connections_percent(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_idle_connections_percent = Some(v.into());
        self
    }
    #[doc = "Set the field `session_pinning_filters`.\n"]
    pub fn set_session_pinning_filters(
        mut self,
        v: impl Into<SetField<PrimField<String>>>,
    ) -> Self {
        self.session_pinning_filters = Some(v.into());
        self
    }
}
impl ToListMappable for DbProxyDefaultTargetGroupConnectionPoolConfigEl {
    type O = BlockAssignable<DbProxyDefaultTargetGroupConnectionPoolConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDbProxyDefaultTargetGroupConnectionPoolConfigEl {}
impl BuildDbProxyDefaultTargetGroupConnectionPoolConfigEl {
    pub fn build(self) -> DbProxyDefaultTargetGroupConnectionPoolConfigEl {
        DbProxyDefaultTargetGroupConnectionPoolConfigEl {
            connection_borrow_timeout: core::default::Default::default(),
            init_query: core::default::Default::default(),
            max_connections_percent: core::default::Default::default(),
            max_idle_connections_percent: core::default::Default::default(),
            session_pinning_filters: core::default::Default::default(),
        }
    }
}
pub struct DbProxyDefaultTargetGroupConnectionPoolConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DbProxyDefaultTargetGroupConnectionPoolConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DbProxyDefaultTargetGroupConnectionPoolConfigElRef {
        DbProxyDefaultTargetGroupConnectionPoolConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DbProxyDefaultTargetGroupConnectionPoolConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `connection_borrow_timeout` after provisioning.\n"]
    pub fn connection_borrow_timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.connection_borrow_timeout", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `init_query` after provisioning.\n"]
    pub fn init_query(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.init_query", self.base))
    }
    #[doc = "Get a reference to the value of field `max_connections_percent` after provisioning.\n"]
    pub fn max_connections_percent(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_connections_percent", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_idle_connections_percent` after provisioning.\n"]
    pub fn max_idle_connections_percent(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_idle_connections_percent", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `session_pinning_filters` after provisioning.\n"]
    pub fn session_pinning_filters(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.session_pinning_filters", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DbProxyDefaultTargetGroupTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl DbProxyDefaultTargetGroupTimeoutsEl {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }
    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}
impl ToListMappable for DbProxyDefaultTargetGroupTimeoutsEl {
    type O = BlockAssignable<DbProxyDefaultTargetGroupTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDbProxyDefaultTargetGroupTimeoutsEl {}
impl BuildDbProxyDefaultTargetGroupTimeoutsEl {
    pub fn build(self) -> DbProxyDefaultTargetGroupTimeoutsEl {
        DbProxyDefaultTargetGroupTimeoutsEl {
            create: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct DbProxyDefaultTargetGroupTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DbProxyDefaultTargetGroupTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DbProxyDefaultTargetGroupTimeoutsElRef {
        DbProxyDefaultTargetGroupTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DbProxyDefaultTargetGroupTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }
    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
#[derive(Serialize, Default)]
struct DbProxyDefaultTargetGroupDynamic {
    connection_pool_config: Option<DynamicBlock<DbProxyDefaultTargetGroupConnectionPoolConfigEl>>,
}
