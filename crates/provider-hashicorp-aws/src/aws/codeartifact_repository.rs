use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct CodeartifactRepositoryData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    domain: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    domain_owner: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    repository: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    external_connections: Option<Vec<CodeartifactRepositoryExternalConnectionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    upstream: Option<Vec<CodeartifactRepositoryUpstreamEl>>,
    dynamic: CodeartifactRepositoryDynamic,
}

struct CodeartifactRepository_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CodeartifactRepositoryData>,
}

#[derive(Clone)]
pub struct CodeartifactRepository(Rc<CodeartifactRepository_>);

impl CodeartifactRepository {
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

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc = "Set the field `domain_owner`.\n"]
    pub fn set_domain_owner(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().domain_owner = Some(v.into());
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

    #[doc = "Set the field `external_connections`.\n"]
    pub fn set_external_connections(
        self,
        v: impl Into<BlockAssignable<CodeartifactRepositoryExternalConnectionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().external_connections = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.external_connections = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `upstream`.\n"]
    pub fn set_upstream(
        self,
        v: impl Into<BlockAssignable<CodeartifactRepositoryUpstreamEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().upstream = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.upstream = Some(d);
            }
        }
        self
    }

    #[doc = "Get a reference to the value of field `administrator_account` after provisioning.\n"]
    pub fn administrator_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.administrator_account", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `domain` after provisioning.\n"]
    pub fn domain(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `domain_owner` after provisioning.\n"]
    pub fn domain_owner(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain_owner", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `repository` after provisioning.\n"]
    pub fn repository(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.repository", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `external_connections` after provisioning.\n"]
    pub fn external_connections(&self) -> ListRef<CodeartifactRepositoryExternalConnectionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.external_connections", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `upstream` after provisioning.\n"]
    pub fn upstream(&self) -> ListRef<CodeartifactRepositoryUpstreamElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.upstream", self.extract_ref()),
        )
    }
}

impl Referable for CodeartifactRepository {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for CodeartifactRepository {}

impl ToListMappable for CodeartifactRepository {
    type O = ListRef<CodeartifactRepositoryRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for CodeartifactRepository_ {
    fn extract_resource_type(&self) -> String {
        "aws_codeartifact_repository".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCodeartifactRepository {
    pub tf_id: String,
    #[doc = ""]
    pub domain: PrimField<String>,
    #[doc = ""]
    pub repository: PrimField<String>,
}

impl BuildCodeartifactRepository {
    pub fn build(self, stack: &mut Stack) -> CodeartifactRepository {
        let out = CodeartifactRepository(Rc::new(CodeartifactRepository_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(CodeartifactRepositoryData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                description: core::default::Default::default(),
                domain: self.domain,
                domain_owner: core::default::Default::default(),
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                repository: self.repository,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                external_connections: core::default::Default::default(),
                upstream: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CodeartifactRepositoryRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodeartifactRepositoryRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl CodeartifactRepositoryRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `administrator_account` after provisioning.\n"]
    pub fn administrator_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.administrator_account", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `domain` after provisioning.\n"]
    pub fn domain(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `domain_owner` after provisioning.\n"]
    pub fn domain_owner(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain_owner", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `repository` after provisioning.\n"]
    pub fn repository(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.repository", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `external_connections` after provisioning.\n"]
    pub fn external_connections(&self) -> ListRef<CodeartifactRepositoryExternalConnectionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.external_connections", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `upstream` after provisioning.\n"]
    pub fn upstream(&self) -> ListRef<CodeartifactRepositoryUpstreamElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.upstream", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct CodeartifactRepositoryExternalConnectionsEl {
    external_connection_name: PrimField<String>,
}

impl CodeartifactRepositoryExternalConnectionsEl {}

impl ToListMappable for CodeartifactRepositoryExternalConnectionsEl {
    type O = BlockAssignable<CodeartifactRepositoryExternalConnectionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodeartifactRepositoryExternalConnectionsEl {
    #[doc = ""]
    pub external_connection_name: PrimField<String>,
}

impl BuildCodeartifactRepositoryExternalConnectionsEl {
    pub fn build(self) -> CodeartifactRepositoryExternalConnectionsEl {
        CodeartifactRepositoryExternalConnectionsEl {
            external_connection_name: self.external_connection_name,
        }
    }
}

pub struct CodeartifactRepositoryExternalConnectionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodeartifactRepositoryExternalConnectionsElRef {
    fn new(shared: StackShared, base: String) -> CodeartifactRepositoryExternalConnectionsElRef {
        CodeartifactRepositoryExternalConnectionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodeartifactRepositoryExternalConnectionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `external_connection_name` after provisioning.\n"]
    pub fn external_connection_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.external_connection_name", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `package_format` after provisioning.\n"]
    pub fn package_format(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.package_format", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct CodeartifactRepositoryUpstreamEl {
    repository_name: PrimField<String>,
}

impl CodeartifactRepositoryUpstreamEl {}

impl ToListMappable for CodeartifactRepositoryUpstreamEl {
    type O = BlockAssignable<CodeartifactRepositoryUpstreamEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodeartifactRepositoryUpstreamEl {
    #[doc = ""]
    pub repository_name: PrimField<String>,
}

impl BuildCodeartifactRepositoryUpstreamEl {
    pub fn build(self) -> CodeartifactRepositoryUpstreamEl {
        CodeartifactRepositoryUpstreamEl {
            repository_name: self.repository_name,
        }
    }
}

pub struct CodeartifactRepositoryUpstreamElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodeartifactRepositoryUpstreamElRef {
    fn new(shared: StackShared, base: String) -> CodeartifactRepositoryUpstreamElRef {
        CodeartifactRepositoryUpstreamElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodeartifactRepositoryUpstreamElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `repository_name` after provisioning.\n"]
    pub fn repository_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.repository_name", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct CodeartifactRepositoryDynamic {
    external_connections: Option<DynamicBlock<CodeartifactRepositoryExternalConnectionsEl>>,
    upstream: Option<DynamicBlock<CodeartifactRepositoryUpstreamEl>>,
}
