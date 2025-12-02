use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ConfigConfigurationAggregatorData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    account_aggregation_source:
        Option<Vec<ConfigConfigurationAggregatorAccountAggregationSourceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    organization_aggregation_source:
        Option<Vec<ConfigConfigurationAggregatorOrganizationAggregationSourceEl>>,
    dynamic: ConfigConfigurationAggregatorDynamic,
}
struct ConfigConfigurationAggregator_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ConfigConfigurationAggregatorData>,
}
#[derive(Clone)]
pub struct ConfigConfigurationAggregator(Rc<ConfigConfigurationAggregator_>);
impl ConfigConfigurationAggregator {
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
    #[doc = "Set the field `account_aggregation_source`.\n"]
    pub fn set_account_aggregation_source(
        self,
        v: impl Into<BlockAssignable<ConfigConfigurationAggregatorAccountAggregationSourceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().account_aggregation_source = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.account_aggregation_source = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `organization_aggregation_source`.\n"]
    pub fn set_organization_aggregation_source(
        self,
        v: impl Into<BlockAssignable<ConfigConfigurationAggregatorOrganizationAggregationSourceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().organization_aggregation_source = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .organization_aggregation_source = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
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
    #[doc = "Get a reference to the value of field `account_aggregation_source` after provisioning.\n"]
    pub fn account_aggregation_source(
        &self,
    ) -> ListRef<ConfigConfigurationAggregatorAccountAggregationSourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.account_aggregation_source", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `organization_aggregation_source` after provisioning.\n"]
    pub fn organization_aggregation_source(
        &self,
    ) -> ListRef<ConfigConfigurationAggregatorOrganizationAggregationSourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.organization_aggregation_source", self.extract_ref()),
        )
    }
}
impl Referable for ConfigConfigurationAggregator {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ConfigConfigurationAggregator {}
impl ToListMappable for ConfigConfigurationAggregator {
    type O = ListRef<ConfigConfigurationAggregatorRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ConfigConfigurationAggregator_ {
    fn extract_resource_type(&self) -> String {
        "aws_config_configuration_aggregator".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildConfigConfigurationAggregator {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
}
impl BuildConfigConfigurationAggregator {
    pub fn build(self, stack: &mut Stack) -> ConfigConfigurationAggregator {
        let out = ConfigConfigurationAggregator(Rc::new(ConfigConfigurationAggregator_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ConfigConfigurationAggregatorData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                account_aggregation_source: core::default::Default::default(),
                organization_aggregation_source: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ConfigConfigurationAggregatorRef {
    shared: StackShared,
    base: String,
}
impl Ref for ConfigConfigurationAggregatorRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ConfigConfigurationAggregatorRef {
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
    #[doc = "Get a reference to the value of field `account_aggregation_source` after provisioning.\n"]
    pub fn account_aggregation_source(
        &self,
    ) -> ListRef<ConfigConfigurationAggregatorAccountAggregationSourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.account_aggregation_source", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `organization_aggregation_source` after provisioning.\n"]
    pub fn organization_aggregation_source(
        &self,
    ) -> ListRef<ConfigConfigurationAggregatorOrganizationAggregationSourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.organization_aggregation_source", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ConfigConfigurationAggregatorAccountAggregationSourceEl {
    account_ids: ListField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    all_regions: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    regions: Option<ListField<PrimField<String>>>,
}
impl ConfigConfigurationAggregatorAccountAggregationSourceEl {
    #[doc = "Set the field `all_regions`.\n"]
    pub fn set_all_regions(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.all_regions = Some(v.into());
        self
    }
    #[doc = "Set the field `regions`.\n"]
    pub fn set_regions(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.regions = Some(v.into());
        self
    }
}
impl ToListMappable for ConfigConfigurationAggregatorAccountAggregationSourceEl {
    type O = BlockAssignable<ConfigConfigurationAggregatorAccountAggregationSourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildConfigConfigurationAggregatorAccountAggregationSourceEl {
    #[doc = ""]
    pub account_ids: ListField<PrimField<String>>,
}
impl BuildConfigConfigurationAggregatorAccountAggregationSourceEl {
    pub fn build(self) -> ConfigConfigurationAggregatorAccountAggregationSourceEl {
        ConfigConfigurationAggregatorAccountAggregationSourceEl {
            account_ids: self.account_ids,
            all_regions: core::default::Default::default(),
            regions: core::default::Default::default(),
        }
    }
}
pub struct ConfigConfigurationAggregatorAccountAggregationSourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ConfigConfigurationAggregatorAccountAggregationSourceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ConfigConfigurationAggregatorAccountAggregationSourceElRef {
        ConfigConfigurationAggregatorAccountAggregationSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ConfigConfigurationAggregatorAccountAggregationSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `account_ids` after provisioning.\n"]
    pub fn account_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.account_ids", self.base))
    }
    #[doc = "Get a reference to the value of field `all_regions` after provisioning.\n"]
    pub fn all_regions(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.all_regions", self.base))
    }
    #[doc = "Get a reference to the value of field `regions` after provisioning.\n"]
    pub fn regions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.regions", self.base))
    }
}
#[derive(Serialize)]
pub struct ConfigConfigurationAggregatorOrganizationAggregationSourceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    all_regions: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    regions: Option<ListField<PrimField<String>>>,
    role_arn: PrimField<String>,
}
impl ConfigConfigurationAggregatorOrganizationAggregationSourceEl {
    #[doc = "Set the field `all_regions`.\n"]
    pub fn set_all_regions(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.all_regions = Some(v.into());
        self
    }
    #[doc = "Set the field `regions`.\n"]
    pub fn set_regions(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.regions = Some(v.into());
        self
    }
}
impl ToListMappable for ConfigConfigurationAggregatorOrganizationAggregationSourceEl {
    type O = BlockAssignable<ConfigConfigurationAggregatorOrganizationAggregationSourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildConfigConfigurationAggregatorOrganizationAggregationSourceEl {
    #[doc = ""]
    pub role_arn: PrimField<String>,
}
impl BuildConfigConfigurationAggregatorOrganizationAggregationSourceEl {
    pub fn build(self) -> ConfigConfigurationAggregatorOrganizationAggregationSourceEl {
        ConfigConfigurationAggregatorOrganizationAggregationSourceEl {
            all_regions: core::default::Default::default(),
            regions: core::default::Default::default(),
            role_arn: self.role_arn,
        }
    }
}
pub struct ConfigConfigurationAggregatorOrganizationAggregationSourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ConfigConfigurationAggregatorOrganizationAggregationSourceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ConfigConfigurationAggregatorOrganizationAggregationSourceElRef {
        ConfigConfigurationAggregatorOrganizationAggregationSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ConfigConfigurationAggregatorOrganizationAggregationSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `all_regions` after provisioning.\n"]
    pub fn all_regions(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.all_regions", self.base))
    }
    #[doc = "Get a reference to the value of field `regions` after provisioning.\n"]
    pub fn regions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.regions", self.base))
    }
    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }
}
#[derive(Serialize, Default)]
struct ConfigConfigurationAggregatorDynamic {
    account_aggregation_source:
        Option<DynamicBlock<ConfigConfigurationAggregatorAccountAggregationSourceEl>>,
    organization_aggregation_source:
        Option<DynamicBlock<ConfigConfigurationAggregatorOrganizationAggregationSourceEl>>,
}
