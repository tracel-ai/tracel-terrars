use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct SecuritylakeCustomLogSourceData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    event_classes: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    source_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    configuration: Option<Vec<SecuritylakeCustomLogSourceConfigurationEl>>,
    dynamic: SecuritylakeCustomLogSourceDynamic,
}
struct SecuritylakeCustomLogSource_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SecuritylakeCustomLogSourceData>,
}
#[derive(Clone)]
pub struct SecuritylakeCustomLogSource(Rc<SecuritylakeCustomLogSource_>);
impl SecuritylakeCustomLogSource {
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
    #[doc = "Set the field `event_classes`.\n"]
    pub fn set_event_classes(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().event_classes = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `source_version`.\n"]
    pub fn set_source_version(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().source_version = Some(v.into());
        self
    }
    #[doc = "Set the field `configuration`.\n"]
    pub fn set_configuration(
        self,
        v: impl Into<BlockAssignable<SecuritylakeCustomLogSourceConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.configuration = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `attributes` after provisioning.\n"]
    pub fn attributes(&self) -> ListRef<SecuritylakeCustomLogSourceAttributesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.attributes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `event_classes` after provisioning.\n"]
    pub fn event_classes(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.event_classes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `provider_details` after provisioning.\n"]
    pub fn provider_details(&self) -> ListRef<SecuritylakeCustomLogSourceProviderDetailsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.provider_details", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_name` after provisioning.\n"]
    pub fn source_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_version` after provisioning.\n"]
    pub fn source_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> ListRef<SecuritylakeCustomLogSourceConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.configuration", self.extract_ref()),
        )
    }
}
impl Referable for SecuritylakeCustomLogSource {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for SecuritylakeCustomLogSource {}
impl ToListMappable for SecuritylakeCustomLogSource {
    type O = ListRef<SecuritylakeCustomLogSourceRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for SecuritylakeCustomLogSource_ {
    fn extract_resource_type(&self) -> String {
        "aws_securitylake_custom_log_source".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildSecuritylakeCustomLogSource {
    pub tf_id: String,
    #[doc = ""]
    pub source_name: PrimField<String>,
}
impl BuildSecuritylakeCustomLogSource {
    pub fn build(self, stack: &mut Stack) -> SecuritylakeCustomLogSource {
        let out = SecuritylakeCustomLogSource(Rc::new(SecuritylakeCustomLogSource_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(SecuritylakeCustomLogSourceData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                event_classes: core::default::Default::default(),
                region: core::default::Default::default(),
                source_name: self.source_name,
                source_version: core::default::Default::default(),
                configuration: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct SecuritylakeCustomLogSourceRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecuritylakeCustomLogSourceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl SecuritylakeCustomLogSourceRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `attributes` after provisioning.\n"]
    pub fn attributes(&self) -> ListRef<SecuritylakeCustomLogSourceAttributesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.attributes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `event_classes` after provisioning.\n"]
    pub fn event_classes(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.event_classes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `provider_details` after provisioning.\n"]
    pub fn provider_details(&self) -> ListRef<SecuritylakeCustomLogSourceProviderDetailsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.provider_details", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_name` after provisioning.\n"]
    pub fn source_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_version` after provisioning.\n"]
    pub fn source_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> ListRef<SecuritylakeCustomLogSourceConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.configuration", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct SecuritylakeCustomLogSourceAttributesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    crawler_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    database_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    table_arn: Option<PrimField<String>>,
}
impl SecuritylakeCustomLogSourceAttributesEl {
    #[doc = "Set the field `crawler_arn`.\n"]
    pub fn set_crawler_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.crawler_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `database_arn`.\n"]
    pub fn set_database_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.database_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `table_arn`.\n"]
    pub fn set_table_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.table_arn = Some(v.into());
        self
    }
}
impl ToListMappable for SecuritylakeCustomLogSourceAttributesEl {
    type O = BlockAssignable<SecuritylakeCustomLogSourceAttributesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecuritylakeCustomLogSourceAttributesEl {}
impl BuildSecuritylakeCustomLogSourceAttributesEl {
    pub fn build(self) -> SecuritylakeCustomLogSourceAttributesEl {
        SecuritylakeCustomLogSourceAttributesEl {
            crawler_arn: core::default::Default::default(),
            database_arn: core::default::Default::default(),
            table_arn: core::default::Default::default(),
        }
    }
}
pub struct SecuritylakeCustomLogSourceAttributesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecuritylakeCustomLogSourceAttributesElRef {
    fn new(shared: StackShared, base: String) -> SecuritylakeCustomLogSourceAttributesElRef {
        SecuritylakeCustomLogSourceAttributesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecuritylakeCustomLogSourceAttributesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `crawler_arn` after provisioning.\n"]
    pub fn crawler_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.crawler_arn", self.base))
    }
    #[doc = "Get a reference to the value of field `database_arn` after provisioning.\n"]
    pub fn database_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.database_arn", self.base))
    }
    #[doc = "Get a reference to the value of field `table_arn` after provisioning.\n"]
    pub fn table_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.table_arn", self.base))
    }
}
#[derive(Serialize)]
pub struct SecuritylakeCustomLogSourceProviderDetailsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    role_arn: Option<PrimField<String>>,
}
impl SecuritylakeCustomLogSourceProviderDetailsEl {
    #[doc = "Set the field `location`.\n"]
    pub fn set_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.location = Some(v.into());
        self
    }
    #[doc = "Set the field `role_arn`.\n"]
    pub fn set_role_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.role_arn = Some(v.into());
        self
    }
}
impl ToListMappable for SecuritylakeCustomLogSourceProviderDetailsEl {
    type O = BlockAssignable<SecuritylakeCustomLogSourceProviderDetailsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecuritylakeCustomLogSourceProviderDetailsEl {}
impl BuildSecuritylakeCustomLogSourceProviderDetailsEl {
    pub fn build(self) -> SecuritylakeCustomLogSourceProviderDetailsEl {
        SecuritylakeCustomLogSourceProviderDetailsEl {
            location: core::default::Default::default(),
            role_arn: core::default::Default::default(),
        }
    }
}
pub struct SecuritylakeCustomLogSourceProviderDetailsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecuritylakeCustomLogSourceProviderDetailsElRef {
    fn new(shared: StackShared, base: String) -> SecuritylakeCustomLogSourceProviderDetailsElRef {
        SecuritylakeCustomLogSourceProviderDetailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecuritylakeCustomLogSourceProviderDetailsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }
    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }
}
#[derive(Serialize)]
pub struct SecuritylakeCustomLogSourceConfigurationElCrawlerConfigurationEl {
    role_arn: PrimField<String>,
}
impl SecuritylakeCustomLogSourceConfigurationElCrawlerConfigurationEl {}
impl ToListMappable for SecuritylakeCustomLogSourceConfigurationElCrawlerConfigurationEl {
    type O = BlockAssignable<SecuritylakeCustomLogSourceConfigurationElCrawlerConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecuritylakeCustomLogSourceConfigurationElCrawlerConfigurationEl {
    #[doc = ""]
    pub role_arn: PrimField<String>,
}
impl BuildSecuritylakeCustomLogSourceConfigurationElCrawlerConfigurationEl {
    pub fn build(self) -> SecuritylakeCustomLogSourceConfigurationElCrawlerConfigurationEl {
        SecuritylakeCustomLogSourceConfigurationElCrawlerConfigurationEl {
            role_arn: self.role_arn,
        }
    }
}
pub struct SecuritylakeCustomLogSourceConfigurationElCrawlerConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecuritylakeCustomLogSourceConfigurationElCrawlerConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecuritylakeCustomLogSourceConfigurationElCrawlerConfigurationElRef {
        SecuritylakeCustomLogSourceConfigurationElCrawlerConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecuritylakeCustomLogSourceConfigurationElCrawlerConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }
}
#[derive(Serialize)]
pub struct SecuritylakeCustomLogSourceConfigurationElProviderIdentityEl {
    external_id: PrimField<String>,
    principal: PrimField<String>,
}
impl SecuritylakeCustomLogSourceConfigurationElProviderIdentityEl {}
impl ToListMappable for SecuritylakeCustomLogSourceConfigurationElProviderIdentityEl {
    type O = BlockAssignable<SecuritylakeCustomLogSourceConfigurationElProviderIdentityEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecuritylakeCustomLogSourceConfigurationElProviderIdentityEl {
    #[doc = ""]
    pub external_id: PrimField<String>,
    #[doc = ""]
    pub principal: PrimField<String>,
}
impl BuildSecuritylakeCustomLogSourceConfigurationElProviderIdentityEl {
    pub fn build(self) -> SecuritylakeCustomLogSourceConfigurationElProviderIdentityEl {
        SecuritylakeCustomLogSourceConfigurationElProviderIdentityEl {
            external_id: self.external_id,
            principal: self.principal,
        }
    }
}
pub struct SecuritylakeCustomLogSourceConfigurationElProviderIdentityElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecuritylakeCustomLogSourceConfigurationElProviderIdentityElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecuritylakeCustomLogSourceConfigurationElProviderIdentityElRef {
        SecuritylakeCustomLogSourceConfigurationElProviderIdentityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecuritylakeCustomLogSourceConfigurationElProviderIdentityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `external_id` after provisioning.\n"]
    pub fn external_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.external_id", self.base))
    }
    #[doc = "Get a reference to the value of field `principal` after provisioning.\n"]
    pub fn principal(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.principal", self.base))
    }
}
#[derive(Serialize, Default)]
struct SecuritylakeCustomLogSourceConfigurationElDynamic {
    crawler_configuration:
        Option<DynamicBlock<SecuritylakeCustomLogSourceConfigurationElCrawlerConfigurationEl>>,
    provider_identity:
        Option<DynamicBlock<SecuritylakeCustomLogSourceConfigurationElProviderIdentityEl>>,
}
#[derive(Serialize)]
pub struct SecuritylakeCustomLogSourceConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    crawler_configuration:
        Option<Vec<SecuritylakeCustomLogSourceConfigurationElCrawlerConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider_identity: Option<Vec<SecuritylakeCustomLogSourceConfigurationElProviderIdentityEl>>,
    dynamic: SecuritylakeCustomLogSourceConfigurationElDynamic,
}
impl SecuritylakeCustomLogSourceConfigurationEl {
    #[doc = "Set the field `crawler_configuration`.\n"]
    pub fn set_crawler_configuration(
        mut self,
        v: impl Into<BlockAssignable<SecuritylakeCustomLogSourceConfigurationElCrawlerConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.crawler_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.crawler_configuration = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `provider_identity`.\n"]
    pub fn set_provider_identity(
        mut self,
        v: impl Into<BlockAssignable<SecuritylakeCustomLogSourceConfigurationElProviderIdentityEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.provider_identity = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.provider_identity = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for SecuritylakeCustomLogSourceConfigurationEl {
    type O = BlockAssignable<SecuritylakeCustomLogSourceConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecuritylakeCustomLogSourceConfigurationEl {}
impl BuildSecuritylakeCustomLogSourceConfigurationEl {
    pub fn build(self) -> SecuritylakeCustomLogSourceConfigurationEl {
        SecuritylakeCustomLogSourceConfigurationEl {
            crawler_configuration: core::default::Default::default(),
            provider_identity: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct SecuritylakeCustomLogSourceConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecuritylakeCustomLogSourceConfigurationElRef {
    fn new(shared: StackShared, base: String) -> SecuritylakeCustomLogSourceConfigurationElRef {
        SecuritylakeCustomLogSourceConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecuritylakeCustomLogSourceConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `crawler_configuration` after provisioning.\n"]
    pub fn crawler_configuration(
        &self,
    ) -> ListRef<SecuritylakeCustomLogSourceConfigurationElCrawlerConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.crawler_configuration", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `provider_identity` after provisioning.\n"]
    pub fn provider_identity(
        &self,
    ) -> ListRef<SecuritylakeCustomLogSourceConfigurationElProviderIdentityElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.provider_identity", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct SecuritylakeCustomLogSourceDynamic {
    configuration: Option<DynamicBlock<SecuritylakeCustomLogSourceConfigurationEl>>,
}
