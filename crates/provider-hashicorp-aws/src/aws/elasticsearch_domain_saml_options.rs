use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ElasticsearchDomainSamlOptionsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    domain_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    saml_options: Option<Vec<ElasticsearchDomainSamlOptionsSamlOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ElasticsearchDomainSamlOptionsTimeoutsEl>,
    dynamic: ElasticsearchDomainSamlOptionsDynamic,
}
struct ElasticsearchDomainSamlOptions_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ElasticsearchDomainSamlOptionsData>,
}
#[derive(Clone)]
pub struct ElasticsearchDomainSamlOptions(Rc<ElasticsearchDomainSamlOptions_>);
impl ElasticsearchDomainSamlOptions {
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
    #[doc = "Set the field `saml_options`.\n"]
    pub fn set_saml_options(
        self,
        v: impl Into<BlockAssignable<ElasticsearchDomainSamlOptionsSamlOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().saml_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.saml_options = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ElasticsearchDomainSamlOptionsTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain_name", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `saml_options` after provisioning.\n"]
    pub fn saml_options(&self) -> ListRef<ElasticsearchDomainSamlOptionsSamlOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.saml_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ElasticsearchDomainSamlOptionsTimeoutsElRef {
        ElasticsearchDomainSamlOptionsTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ElasticsearchDomainSamlOptions {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ElasticsearchDomainSamlOptions {}
impl ToListMappable for ElasticsearchDomainSamlOptions {
    type O = ListRef<ElasticsearchDomainSamlOptionsRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ElasticsearchDomainSamlOptions_ {
    fn extract_resource_type(&self) -> String {
        "aws_elasticsearch_domain_saml_options".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildElasticsearchDomainSamlOptions {
    pub tf_id: String,
    #[doc = ""]
    pub domain_name: PrimField<String>,
}
impl BuildElasticsearchDomainSamlOptions {
    pub fn build(self, stack: &mut Stack) -> ElasticsearchDomainSamlOptions {
        let out = ElasticsearchDomainSamlOptions(Rc::new(ElasticsearchDomainSamlOptions_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ElasticsearchDomainSamlOptionsData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                domain_name: self.domain_name,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                saml_options: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ElasticsearchDomainSamlOptionsRef {
    shared: StackShared,
    base: String,
}
impl Ref for ElasticsearchDomainSamlOptionsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ElasticsearchDomainSamlOptionsRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain_name", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `saml_options` after provisioning.\n"]
    pub fn saml_options(&self) -> ListRef<ElasticsearchDomainSamlOptionsSamlOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.saml_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ElasticsearchDomainSamlOptionsTimeoutsElRef {
        ElasticsearchDomainSamlOptionsTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ElasticsearchDomainSamlOptionsSamlOptionsElIdpEl {
    entity_id: PrimField<String>,
    metadata_content: PrimField<String>,
}
impl ElasticsearchDomainSamlOptionsSamlOptionsElIdpEl {}
impl ToListMappable for ElasticsearchDomainSamlOptionsSamlOptionsElIdpEl {
    type O = BlockAssignable<ElasticsearchDomainSamlOptionsSamlOptionsElIdpEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildElasticsearchDomainSamlOptionsSamlOptionsElIdpEl {
    #[doc = ""]
    pub entity_id: PrimField<String>,
    #[doc = ""]
    pub metadata_content: PrimField<String>,
}
impl BuildElasticsearchDomainSamlOptionsSamlOptionsElIdpEl {
    pub fn build(self) -> ElasticsearchDomainSamlOptionsSamlOptionsElIdpEl {
        ElasticsearchDomainSamlOptionsSamlOptionsElIdpEl {
            entity_id: self.entity_id,
            metadata_content: self.metadata_content,
        }
    }
}
pub struct ElasticsearchDomainSamlOptionsSamlOptionsElIdpElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ElasticsearchDomainSamlOptionsSamlOptionsElIdpElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ElasticsearchDomainSamlOptionsSamlOptionsElIdpElRef {
        ElasticsearchDomainSamlOptionsSamlOptionsElIdpElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ElasticsearchDomainSamlOptionsSamlOptionsElIdpElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `entity_id` after provisioning.\n"]
    pub fn entity_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.entity_id", self.base))
    }
    #[doc = "Get a reference to the value of field `metadata_content` after provisioning.\n"]
    pub fn metadata_content(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.metadata_content", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ElasticsearchDomainSamlOptionsSamlOptionsElDynamic {
    idp: Option<DynamicBlock<ElasticsearchDomainSamlOptionsSamlOptionsElIdpEl>>,
}
#[derive(Serialize)]
pub struct ElasticsearchDomainSamlOptionsSamlOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    master_backend_role: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    master_user_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    roles_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    session_timeout_minutes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subject_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    idp: Option<Vec<ElasticsearchDomainSamlOptionsSamlOptionsElIdpEl>>,
    dynamic: ElasticsearchDomainSamlOptionsSamlOptionsElDynamic,
}
impl ElasticsearchDomainSamlOptionsSamlOptionsEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `master_backend_role`.\n"]
    pub fn set_master_backend_role(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.master_backend_role = Some(v.into());
        self
    }
    #[doc = "Set the field `master_user_name`.\n"]
    pub fn set_master_user_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.master_user_name = Some(v.into());
        self
    }
    #[doc = "Set the field `roles_key`.\n"]
    pub fn set_roles_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.roles_key = Some(v.into());
        self
    }
    #[doc = "Set the field `session_timeout_minutes`.\n"]
    pub fn set_session_timeout_minutes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.session_timeout_minutes = Some(v.into());
        self
    }
    #[doc = "Set the field `subject_key`.\n"]
    pub fn set_subject_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.subject_key = Some(v.into());
        self
    }
    #[doc = "Set the field `idp`.\n"]
    pub fn set_idp(
        mut self,
        v: impl Into<BlockAssignable<ElasticsearchDomainSamlOptionsSamlOptionsElIdpEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.idp = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.idp = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ElasticsearchDomainSamlOptionsSamlOptionsEl {
    type O = BlockAssignable<ElasticsearchDomainSamlOptionsSamlOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildElasticsearchDomainSamlOptionsSamlOptionsEl {}
impl BuildElasticsearchDomainSamlOptionsSamlOptionsEl {
    pub fn build(self) -> ElasticsearchDomainSamlOptionsSamlOptionsEl {
        ElasticsearchDomainSamlOptionsSamlOptionsEl {
            enabled: core::default::Default::default(),
            master_backend_role: core::default::Default::default(),
            master_user_name: core::default::Default::default(),
            roles_key: core::default::Default::default(),
            session_timeout_minutes: core::default::Default::default(),
            subject_key: core::default::Default::default(),
            idp: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ElasticsearchDomainSamlOptionsSamlOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ElasticsearchDomainSamlOptionsSamlOptionsElRef {
    fn new(shared: StackShared, base: String) -> ElasticsearchDomainSamlOptionsSamlOptionsElRef {
        ElasticsearchDomainSamlOptionsSamlOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ElasticsearchDomainSamlOptionsSamlOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
    #[doc = "Get a reference to the value of field `master_backend_role` after provisioning.\n"]
    pub fn master_backend_role(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.master_backend_role", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `master_user_name` after provisioning.\n"]
    pub fn master_user_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.master_user_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `roles_key` after provisioning.\n"]
    pub fn roles_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.roles_key", self.base))
    }
    #[doc = "Get a reference to the value of field `session_timeout_minutes` after provisioning.\n"]
    pub fn session_timeout_minutes(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.session_timeout_minutes", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `subject_key` after provisioning.\n"]
    pub fn subject_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subject_key", self.base))
    }
    #[doc = "Get a reference to the value of field `idp` after provisioning.\n"]
    pub fn idp(&self) -> ListRef<ElasticsearchDomainSamlOptionsSamlOptionsElIdpElRef> {
        ListRef::new(self.shared().clone(), format!("{}.idp", self.base))
    }
}
#[derive(Serialize)]
pub struct ElasticsearchDomainSamlOptionsTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ElasticsearchDomainSamlOptionsTimeoutsEl {
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
impl ToListMappable for ElasticsearchDomainSamlOptionsTimeoutsEl {
    type O = BlockAssignable<ElasticsearchDomainSamlOptionsTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildElasticsearchDomainSamlOptionsTimeoutsEl {}
impl BuildElasticsearchDomainSamlOptionsTimeoutsEl {
    pub fn build(self) -> ElasticsearchDomainSamlOptionsTimeoutsEl {
        ElasticsearchDomainSamlOptionsTimeoutsEl {
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ElasticsearchDomainSamlOptionsTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ElasticsearchDomainSamlOptionsTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ElasticsearchDomainSamlOptionsTimeoutsElRef {
        ElasticsearchDomainSamlOptionsTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ElasticsearchDomainSamlOptionsTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
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
struct ElasticsearchDomainSamlOptionsDynamic {
    saml_options: Option<DynamicBlock<ElasticsearchDomainSamlOptionsSamlOptionsEl>>,
}
