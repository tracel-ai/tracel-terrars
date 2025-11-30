use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct EcrReplicationConfigurationData {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    replication_configuration: Option<Vec<EcrReplicationConfigurationReplicationConfigurationEl>>,
    dynamic: EcrReplicationConfigurationDynamic,
}

struct EcrReplicationConfiguration_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<EcrReplicationConfigurationData>,
}

#[derive(Clone)]
pub struct EcrReplicationConfiguration(Rc<EcrReplicationConfiguration_>);

impl EcrReplicationConfiguration {
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

    #[doc = "Set the field `replication_configuration`.\n"]
    pub fn set_replication_configuration(
        self,
        v: impl Into<BlockAssignable<EcrReplicationConfigurationReplicationConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().replication_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.replication_configuration = Some(d);
            }
        }
        self
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

    #[doc = "Get a reference to the value of field `registry_id` after provisioning.\n"]
    pub fn registry_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.registry_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `replication_configuration` after provisioning.\n"]
    pub fn replication_configuration(
        &self,
    ) -> ListRef<EcrReplicationConfigurationReplicationConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.replication_configuration", self.extract_ref()),
        )
    }
}

impl Referable for EcrReplicationConfiguration {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for EcrReplicationConfiguration {}

impl ToListMappable for EcrReplicationConfiguration {
    type O = ListRef<EcrReplicationConfigurationRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for EcrReplicationConfiguration_ {
    fn extract_resource_type(&self) -> String {
        "aws_ecr_replication_configuration".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildEcrReplicationConfiguration {
    pub tf_id: String,
}

impl BuildEcrReplicationConfiguration {
    pub fn build(self, stack: &mut Stack) -> EcrReplicationConfiguration {
        let out = EcrReplicationConfiguration(Rc::new(EcrReplicationConfiguration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(EcrReplicationConfigurationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                replication_configuration: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct EcrReplicationConfigurationRef {
    shared: StackShared,
    base: String,
}

impl Ref for EcrReplicationConfigurationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl EcrReplicationConfigurationRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
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

    #[doc = "Get a reference to the value of field `registry_id` after provisioning.\n"]
    pub fn registry_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.registry_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `replication_configuration` after provisioning.\n"]
    pub fn replication_configuration(
        &self,
    ) -> ListRef<EcrReplicationConfigurationReplicationConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.replication_configuration", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct EcrReplicationConfigurationReplicationConfigurationElRuleElDestinationEl {
    region: PrimField<String>,
    registry_id: PrimField<String>,
}

impl EcrReplicationConfigurationReplicationConfigurationElRuleElDestinationEl {}

impl ToListMappable for EcrReplicationConfigurationReplicationConfigurationElRuleElDestinationEl {
    type O =
        BlockAssignable<EcrReplicationConfigurationReplicationConfigurationElRuleElDestinationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcrReplicationConfigurationReplicationConfigurationElRuleElDestinationEl {
    #[doc = ""]
    pub region: PrimField<String>,
    #[doc = ""]
    pub registry_id: PrimField<String>,
}

impl BuildEcrReplicationConfigurationReplicationConfigurationElRuleElDestinationEl {
    pub fn build(self) -> EcrReplicationConfigurationReplicationConfigurationElRuleElDestinationEl {
        EcrReplicationConfigurationReplicationConfigurationElRuleElDestinationEl {
            region: self.region,
            registry_id: self.registry_id,
        }
    }
}

pub struct EcrReplicationConfigurationReplicationConfigurationElRuleElDestinationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EcrReplicationConfigurationReplicationConfigurationElRuleElDestinationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> EcrReplicationConfigurationReplicationConfigurationElRuleElDestinationElRef {
        EcrReplicationConfigurationReplicationConfigurationElRuleElDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcrReplicationConfigurationReplicationConfigurationElRuleElDestinationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }

    #[doc = "Get a reference to the value of field `registry_id` after provisioning.\n"]
    pub fn registry_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.registry_id", self.base))
    }
}

#[derive(Serialize)]
pub struct EcrReplicationConfigurationReplicationConfigurationElRuleElRepositoryFilterEl {
    filter: PrimField<String>,
    filter_type: PrimField<String>,
}

impl EcrReplicationConfigurationReplicationConfigurationElRuleElRepositoryFilterEl {}

impl ToListMappable
    for EcrReplicationConfigurationReplicationConfigurationElRuleElRepositoryFilterEl
{
    type O = BlockAssignable<
        EcrReplicationConfigurationReplicationConfigurationElRuleElRepositoryFilterEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcrReplicationConfigurationReplicationConfigurationElRuleElRepositoryFilterEl {
    #[doc = ""]
    pub filter: PrimField<String>,
    #[doc = ""]
    pub filter_type: PrimField<String>,
}

impl BuildEcrReplicationConfigurationReplicationConfigurationElRuleElRepositoryFilterEl {
    pub fn build(
        self,
    ) -> EcrReplicationConfigurationReplicationConfigurationElRuleElRepositoryFilterEl {
        EcrReplicationConfigurationReplicationConfigurationElRuleElRepositoryFilterEl {
            filter: self.filter,
            filter_type: self.filter_type,
        }
    }
}

pub struct EcrReplicationConfigurationReplicationConfigurationElRuleElRepositoryFilterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EcrReplicationConfigurationReplicationConfigurationElRuleElRepositoryFilterElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> EcrReplicationConfigurationReplicationConfigurationElRuleElRepositoryFilterElRef {
        EcrReplicationConfigurationReplicationConfigurationElRuleElRepositoryFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcrReplicationConfigurationReplicationConfigurationElRuleElRepositoryFilterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.filter", self.base))
    }

    #[doc = "Get a reference to the value of field `filter_type` after provisioning.\n"]
    pub fn filter_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.filter_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct EcrReplicationConfigurationReplicationConfigurationElRuleElDynamic {
    destination: Option<
        DynamicBlock<EcrReplicationConfigurationReplicationConfigurationElRuleElDestinationEl>,
    >,
    repository_filter: Option<
        DynamicBlock<EcrReplicationConfigurationReplicationConfigurationElRuleElRepositoryFilterEl>,
    >,
}

#[derive(Serialize)]
pub struct EcrReplicationConfigurationReplicationConfigurationElRuleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    destination:
        Option<Vec<EcrReplicationConfigurationReplicationConfigurationElRuleElDestinationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repository_filter:
        Option<Vec<EcrReplicationConfigurationReplicationConfigurationElRuleElRepositoryFilterEl>>,
    dynamic: EcrReplicationConfigurationReplicationConfigurationElRuleElDynamic,
}

impl EcrReplicationConfigurationReplicationConfigurationElRuleEl {
    #[doc = "Set the field `destination`.\n"]
    pub fn set_destination(
        mut self,
        v: impl Into<
            BlockAssignable<
                EcrReplicationConfigurationReplicationConfigurationElRuleElDestinationEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.destination = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.destination = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `repository_filter`.\n"]
    pub fn set_repository_filter(
        mut self,
        v: impl Into<
            BlockAssignable<
                EcrReplicationConfigurationReplicationConfigurationElRuleElRepositoryFilterEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.repository_filter = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.repository_filter = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for EcrReplicationConfigurationReplicationConfigurationElRuleEl {
    type O = BlockAssignable<EcrReplicationConfigurationReplicationConfigurationElRuleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcrReplicationConfigurationReplicationConfigurationElRuleEl {}

impl BuildEcrReplicationConfigurationReplicationConfigurationElRuleEl {
    pub fn build(self) -> EcrReplicationConfigurationReplicationConfigurationElRuleEl {
        EcrReplicationConfigurationReplicationConfigurationElRuleEl {
            destination: core::default::Default::default(),
            repository_filter: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct EcrReplicationConfigurationReplicationConfigurationElRuleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EcrReplicationConfigurationReplicationConfigurationElRuleElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> EcrReplicationConfigurationReplicationConfigurationElRuleElRef {
        EcrReplicationConfigurationReplicationConfigurationElRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcrReplicationConfigurationReplicationConfigurationElRuleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(
        &self,
    ) -> ListRef<EcrReplicationConfigurationReplicationConfigurationElRuleElDestinationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.destination", self.base))
    }

    #[doc = "Get a reference to the value of field `repository_filter` after provisioning.\n"]
    pub fn repository_filter(
        &self,
    ) -> ListRef<EcrReplicationConfigurationReplicationConfigurationElRuleElRepositoryFilterElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.repository_filter", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct EcrReplicationConfigurationReplicationConfigurationElDynamic {
    rule: Option<DynamicBlock<EcrReplicationConfigurationReplicationConfigurationElRuleEl>>,
}

#[derive(Serialize)]
pub struct EcrReplicationConfigurationReplicationConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    rule: Option<Vec<EcrReplicationConfigurationReplicationConfigurationElRuleEl>>,
    dynamic: EcrReplicationConfigurationReplicationConfigurationElDynamic,
}

impl EcrReplicationConfigurationReplicationConfigurationEl {
    #[doc = "Set the field `rule`.\n"]
    pub fn set_rule(
        mut self,
        v: impl Into<BlockAssignable<EcrReplicationConfigurationReplicationConfigurationElRuleEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.rule = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.rule = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for EcrReplicationConfigurationReplicationConfigurationEl {
    type O = BlockAssignable<EcrReplicationConfigurationReplicationConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcrReplicationConfigurationReplicationConfigurationEl {}

impl BuildEcrReplicationConfigurationReplicationConfigurationEl {
    pub fn build(self) -> EcrReplicationConfigurationReplicationConfigurationEl {
        EcrReplicationConfigurationReplicationConfigurationEl {
            rule: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct EcrReplicationConfigurationReplicationConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EcrReplicationConfigurationReplicationConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> EcrReplicationConfigurationReplicationConfigurationElRef {
        EcrReplicationConfigurationReplicationConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcrReplicationConfigurationReplicationConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> ListRef<EcrReplicationConfigurationReplicationConfigurationElRuleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.rule", self.base))
    }
}

#[derive(Serialize, Default)]
struct EcrReplicationConfigurationDynamic {
    replication_configuration:
        Option<DynamicBlock<EcrReplicationConfigurationReplicationConfigurationEl>>,
}
