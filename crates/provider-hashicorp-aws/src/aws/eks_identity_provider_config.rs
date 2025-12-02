use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct EksIdentityProviderConfigData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    cluster_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oidc: Option<Vec<EksIdentityProviderConfigOidcEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<EksIdentityProviderConfigTimeoutsEl>,
    dynamic: EksIdentityProviderConfigDynamic,
}
struct EksIdentityProviderConfig_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<EksIdentityProviderConfigData>,
}
#[derive(Clone)]
pub struct EksIdentityProviderConfig(Rc<EksIdentityProviderConfig_>);
impl EksIdentityProviderConfig {
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
    #[doc = "Set the field `oidc`.\n"]
    pub fn set_oidc(self, v: impl Into<BlockAssignable<EksIdentityProviderConfigOidcEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().oidc = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.oidc = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<EksIdentityProviderConfigTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `cluster_name` after provisioning.\n"]
    pub fn cluster_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster_name", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `oidc` after provisioning.\n"]
    pub fn oidc(&self) -> ListRef<EksIdentityProviderConfigOidcElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.oidc", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> EksIdentityProviderConfigTimeoutsElRef {
        EksIdentityProviderConfigTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for EksIdentityProviderConfig {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for EksIdentityProviderConfig {}
impl ToListMappable for EksIdentityProviderConfig {
    type O = ListRef<EksIdentityProviderConfigRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for EksIdentityProviderConfig_ {
    fn extract_resource_type(&self) -> String {
        "aws_eks_identity_provider_config".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildEksIdentityProviderConfig {
    pub tf_id: String,
    #[doc = ""]
    pub cluster_name: PrimField<String>,
}
impl BuildEksIdentityProviderConfig {
    pub fn build(self, stack: &mut Stack) -> EksIdentityProviderConfig {
        let out = EksIdentityProviderConfig(Rc::new(EksIdentityProviderConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(EksIdentityProviderConfigData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                cluster_name: self.cluster_name,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                oidc: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct EksIdentityProviderConfigRef {
    shared: StackShared,
    base: String,
}
impl Ref for EksIdentityProviderConfigRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl EksIdentityProviderConfigRef {
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
    #[doc = "Get a reference to the value of field `cluster_name` after provisioning.\n"]
    pub fn cluster_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster_name", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `oidc` after provisioning.\n"]
    pub fn oidc(&self) -> ListRef<EksIdentityProviderConfigOidcElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.oidc", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> EksIdentityProviderConfigTimeoutsElRef {
        EksIdentityProviderConfigTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct EksIdentityProviderConfigOidcEl {
    client_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    groups_claim: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    groups_prefix: Option<PrimField<String>>,
    identity_provider_config_name: PrimField<String>,
    issuer_url: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    required_claims: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    username_claim: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    username_prefix: Option<PrimField<String>>,
}
impl EksIdentityProviderConfigOidcEl {
    #[doc = "Set the field `groups_claim`.\n"]
    pub fn set_groups_claim(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.groups_claim = Some(v.into());
        self
    }
    #[doc = "Set the field `groups_prefix`.\n"]
    pub fn set_groups_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.groups_prefix = Some(v.into());
        self
    }
    #[doc = "Set the field `required_claims`.\n"]
    pub fn set_required_claims(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.required_claims = Some(v.into());
        self
    }
    #[doc = "Set the field `username_claim`.\n"]
    pub fn set_username_claim(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.username_claim = Some(v.into());
        self
    }
    #[doc = "Set the field `username_prefix`.\n"]
    pub fn set_username_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.username_prefix = Some(v.into());
        self
    }
}
impl ToListMappable for EksIdentityProviderConfigOidcEl {
    type O = BlockAssignable<EksIdentityProviderConfigOidcEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEksIdentityProviderConfigOidcEl {
    #[doc = ""]
    pub client_id: PrimField<String>,
    #[doc = ""]
    pub identity_provider_config_name: PrimField<String>,
    #[doc = ""]
    pub issuer_url: PrimField<String>,
}
impl BuildEksIdentityProviderConfigOidcEl {
    pub fn build(self) -> EksIdentityProviderConfigOidcEl {
        EksIdentityProviderConfigOidcEl {
            client_id: self.client_id,
            groups_claim: core::default::Default::default(),
            groups_prefix: core::default::Default::default(),
            identity_provider_config_name: self.identity_provider_config_name,
            issuer_url: self.issuer_url,
            required_claims: core::default::Default::default(),
            username_claim: core::default::Default::default(),
            username_prefix: core::default::Default::default(),
        }
    }
}
pub struct EksIdentityProviderConfigOidcElRef {
    shared: StackShared,
    base: String,
}
impl Ref for EksIdentityProviderConfigOidcElRef {
    fn new(shared: StackShared, base: String) -> EksIdentityProviderConfigOidcElRef {
        EksIdentityProviderConfigOidcElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl EksIdentityProviderConfigOidcElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `client_id` after provisioning.\n"]
    pub fn client_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }
    #[doc = "Get a reference to the value of field `groups_claim` after provisioning.\n"]
    pub fn groups_claim(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.groups_claim", self.base))
    }
    #[doc = "Get a reference to the value of field `groups_prefix` after provisioning.\n"]
    pub fn groups_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.groups_prefix", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `identity_provider_config_name` after provisioning.\n"]
    pub fn identity_provider_config_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.identity_provider_config_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `issuer_url` after provisioning.\n"]
    pub fn issuer_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.issuer_url", self.base))
    }
    #[doc = "Get a reference to the value of field `required_claims` after provisioning.\n"]
    pub fn required_claims(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.required_claims", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `username_claim` after provisioning.\n"]
    pub fn username_claim(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.username_claim", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `username_prefix` after provisioning.\n"]
    pub fn username_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.username_prefix", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct EksIdentityProviderConfigTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}
impl EksIdentityProviderConfigTimeoutsEl {
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
}
impl ToListMappable for EksIdentityProviderConfigTimeoutsEl {
    type O = BlockAssignable<EksIdentityProviderConfigTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEksIdentityProviderConfigTimeoutsEl {}
impl BuildEksIdentityProviderConfigTimeoutsEl {
    pub fn build(self) -> EksIdentityProviderConfigTimeoutsEl {
        EksIdentityProviderConfigTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}
pub struct EksIdentityProviderConfigTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for EksIdentityProviderConfigTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> EksIdentityProviderConfigTimeoutsElRef {
        EksIdentityProviderConfigTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl EksIdentityProviderConfigTimeoutsElRef {
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
}
#[derive(Serialize, Default)]
struct EksIdentityProviderConfigDynamic {
    oidc: Option<DynamicBlock<EksIdentityProviderConfigOidcEl>>,
}
