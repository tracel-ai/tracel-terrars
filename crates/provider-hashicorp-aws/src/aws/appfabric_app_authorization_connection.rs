use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct AppfabricAppAuthorizationConnectionData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    app_authorization_arn: PrimField<String>,
    app_bundle_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auth_request: Option<Vec<AppfabricAppAuthorizationConnectionAuthRequestEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<AppfabricAppAuthorizationConnectionTimeoutsEl>,
    dynamic: AppfabricAppAuthorizationConnectionDynamic,
}
struct AppfabricAppAuthorizationConnection_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<AppfabricAppAuthorizationConnectionData>,
}
#[derive(Clone)]
pub struct AppfabricAppAuthorizationConnection(Rc<AppfabricAppAuthorizationConnection_>);
impl AppfabricAppAuthorizationConnection {
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
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `auth_request`.\n"]
    pub fn set_auth_request(
        self,
        v: impl Into<BlockAssignable<AppfabricAppAuthorizationConnectionAuthRequestEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().auth_request = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.auth_request = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<AppfabricAppAuthorizationConnectionTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `app` after provisioning.\n"]
    pub fn app(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.app", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `app_authorization_arn` after provisioning.\n"]
    pub fn app_authorization_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.app_authorization_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `app_bundle_arn` after provisioning.\n"]
    pub fn app_bundle_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.app_bundle_arn", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `tenant` after provisioning.\n"]
    pub fn tenant(&self) -> ListRef<AppfabricAppAuthorizationConnectionTenantElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tenant", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `auth_request` after provisioning.\n"]
    pub fn auth_request(&self) -> ListRef<AppfabricAppAuthorizationConnectionAuthRequestElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.auth_request", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AppfabricAppAuthorizationConnectionTimeoutsElRef {
        AppfabricAppAuthorizationConnectionTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for AppfabricAppAuthorizationConnection {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for AppfabricAppAuthorizationConnection {}
impl ToListMappable for AppfabricAppAuthorizationConnection {
    type O = ListRef<AppfabricAppAuthorizationConnectionRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for AppfabricAppAuthorizationConnection_ {
    fn extract_resource_type(&self) -> String {
        "aws_appfabric_app_authorization_connection".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildAppfabricAppAuthorizationConnection {
    pub tf_id: String,
    #[doc = ""]
    pub app_authorization_arn: PrimField<String>,
    #[doc = ""]
    pub app_bundle_arn: PrimField<String>,
}
impl BuildAppfabricAppAuthorizationConnection {
    pub fn build(self, stack: &mut Stack) -> AppfabricAppAuthorizationConnection {
        let out =
            AppfabricAppAuthorizationConnection(Rc::new(AppfabricAppAuthorizationConnection_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(AppfabricAppAuthorizationConnectionData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    lifecycle: core::default::Default::default(),
                    for_each: None,
                    app_authorization_arn: self.app_authorization_arn,
                    app_bundle_arn: self.app_bundle_arn,
                    region: core::default::Default::default(),
                    auth_request: core::default::Default::default(),
                    timeouts: core::default::Default::default(),
                    dynamic: Default::default(),
                }),
            }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct AppfabricAppAuthorizationConnectionRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppfabricAppAuthorizationConnectionRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl AppfabricAppAuthorizationConnectionRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `app` after provisioning.\n"]
    pub fn app(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.app", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `app_authorization_arn` after provisioning.\n"]
    pub fn app_authorization_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.app_authorization_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `app_bundle_arn` after provisioning.\n"]
    pub fn app_bundle_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.app_bundle_arn", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `tenant` after provisioning.\n"]
    pub fn tenant(&self) -> ListRef<AppfabricAppAuthorizationConnectionTenantElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tenant", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `auth_request` after provisioning.\n"]
    pub fn auth_request(&self) -> ListRef<AppfabricAppAuthorizationConnectionAuthRequestElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.auth_request", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AppfabricAppAuthorizationConnectionTimeoutsElRef {
        AppfabricAppAuthorizationConnectionTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct AppfabricAppAuthorizationConnectionTenantEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    tenant_display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tenant_identifier: Option<PrimField<String>>,
}
impl AppfabricAppAuthorizationConnectionTenantEl {
    #[doc = "Set the field `tenant_display_name`.\n"]
    pub fn set_tenant_display_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tenant_display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `tenant_identifier`.\n"]
    pub fn set_tenant_identifier(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tenant_identifier = Some(v.into());
        self
    }
}
impl ToListMappable for AppfabricAppAuthorizationConnectionTenantEl {
    type O = BlockAssignable<AppfabricAppAuthorizationConnectionTenantEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppfabricAppAuthorizationConnectionTenantEl {}
impl BuildAppfabricAppAuthorizationConnectionTenantEl {
    pub fn build(self) -> AppfabricAppAuthorizationConnectionTenantEl {
        AppfabricAppAuthorizationConnectionTenantEl {
            tenant_display_name: core::default::Default::default(),
            tenant_identifier: core::default::Default::default(),
        }
    }
}
pub struct AppfabricAppAuthorizationConnectionTenantElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppfabricAppAuthorizationConnectionTenantElRef {
    fn new(shared: StackShared, base: String) -> AppfabricAppAuthorizationConnectionTenantElRef {
        AppfabricAppAuthorizationConnectionTenantElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppfabricAppAuthorizationConnectionTenantElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `tenant_display_name` after provisioning.\n"]
    pub fn tenant_display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.tenant_display_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `tenant_identifier` after provisioning.\n"]
    pub fn tenant_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.tenant_identifier", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct AppfabricAppAuthorizationConnectionAuthRequestEl {
    code: PrimField<String>,
    redirect_uri: PrimField<String>,
}
impl AppfabricAppAuthorizationConnectionAuthRequestEl {}
impl ToListMappable for AppfabricAppAuthorizationConnectionAuthRequestEl {
    type O = BlockAssignable<AppfabricAppAuthorizationConnectionAuthRequestEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppfabricAppAuthorizationConnectionAuthRequestEl {
    #[doc = ""]
    pub code: PrimField<String>,
    #[doc = ""]
    pub redirect_uri: PrimField<String>,
}
impl BuildAppfabricAppAuthorizationConnectionAuthRequestEl {
    pub fn build(self) -> AppfabricAppAuthorizationConnectionAuthRequestEl {
        AppfabricAppAuthorizationConnectionAuthRequestEl {
            code: self.code,
            redirect_uri: self.redirect_uri,
        }
    }
}
pub struct AppfabricAppAuthorizationConnectionAuthRequestElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppfabricAppAuthorizationConnectionAuthRequestElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppfabricAppAuthorizationConnectionAuthRequestElRef {
        AppfabricAppAuthorizationConnectionAuthRequestElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppfabricAppAuthorizationConnectionAuthRequestElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }
    #[doc = "Get a reference to the value of field `redirect_uri` after provisioning.\n"]
    pub fn redirect_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.redirect_uri", self.base))
    }
}
#[derive(Serialize)]
pub struct AppfabricAppAuthorizationConnectionTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
}
impl AppfabricAppAuthorizationConnectionTimeoutsEl {
    #[doc = "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }
}
impl ToListMappable for AppfabricAppAuthorizationConnectionTimeoutsEl {
    type O = BlockAssignable<AppfabricAppAuthorizationConnectionTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppfabricAppAuthorizationConnectionTimeoutsEl {}
impl BuildAppfabricAppAuthorizationConnectionTimeoutsEl {
    pub fn build(self) -> AppfabricAppAuthorizationConnectionTimeoutsEl {
        AppfabricAppAuthorizationConnectionTimeoutsEl {
            create: core::default::Default::default(),
        }
    }
}
pub struct AppfabricAppAuthorizationConnectionTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppfabricAppAuthorizationConnectionTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> AppfabricAppAuthorizationConnectionTimeoutsElRef {
        AppfabricAppAuthorizationConnectionTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppfabricAppAuthorizationConnectionTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }
}
#[derive(Serialize, Default)]
struct AppfabricAppAuthorizationConnectionDynamic {
    auth_request: Option<DynamicBlock<AppfabricAppAuthorizationConnectionAuthRequestEl>>,
}
