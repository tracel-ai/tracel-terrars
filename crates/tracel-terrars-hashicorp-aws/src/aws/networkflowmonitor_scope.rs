use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct NetworkflowmonitorScopeData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target: Option<Vec<NetworkflowmonitorScopeTargetEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<NetworkflowmonitorScopeTimeoutsEl>,
    dynamic: NetworkflowmonitorScopeDynamic,
}

struct NetworkflowmonitorScope_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<NetworkflowmonitorScopeData>,
}

#[derive(Clone)]
pub struct NetworkflowmonitorScope(Rc<NetworkflowmonitorScope_>);

impl NetworkflowmonitorScope {
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
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
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc = "Set the field `target`.\n"]
    pub fn set_target(self, v: impl Into<BlockAssignable<NetworkflowmonitorScopeTargetEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().target = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.target = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<NetworkflowmonitorScopeTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scope_arn` after provisioning.\n"]
    pub fn scope_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scope_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scope_id` after provisioning.\n"]
    pub fn scope_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scope_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkflowmonitorScopeTimeoutsElRef {
        NetworkflowmonitorScopeTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for NetworkflowmonitorScope {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for NetworkflowmonitorScope { }

impl ToListMappable for NetworkflowmonitorScope {
    type O = ListRef<NetworkflowmonitorScopeRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for NetworkflowmonitorScope_ {
    fn extract_resource_type(&self) -> String {
        "aws_networkflowmonitor_scope".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildNetworkflowmonitorScope {
    pub tf_id: String,
}

impl BuildNetworkflowmonitorScope {
    pub fn build(self, stack: &mut Stack) -> NetworkflowmonitorScope {
        let out = NetworkflowmonitorScope(Rc::new(NetworkflowmonitorScope_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(NetworkflowmonitorScopeData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                target: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct NetworkflowmonitorScopeRef {
    shared: StackShared,
    base: String,
}

impl Ref for NetworkflowmonitorScopeRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl NetworkflowmonitorScopeRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scope_arn` after provisioning.\n"]
    pub fn scope_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scope_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scope_id` after provisioning.\n"]
    pub fn scope_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scope_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkflowmonitorScopeTimeoutsElRef {
        NetworkflowmonitorScopeTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct NetworkflowmonitorScopeTargetElTargetIdentifierElTargetIdEl {
    account_id: PrimField<String>,
}

impl NetworkflowmonitorScopeTargetElTargetIdentifierElTargetIdEl { }

impl ToListMappable for NetworkflowmonitorScopeTargetElTargetIdentifierElTargetIdEl {
    type O = BlockAssignable<NetworkflowmonitorScopeTargetElTargetIdentifierElTargetIdEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkflowmonitorScopeTargetElTargetIdentifierElTargetIdEl {
    #[doc = ""]
    pub account_id: PrimField<String>,
}

impl BuildNetworkflowmonitorScopeTargetElTargetIdentifierElTargetIdEl {
    pub fn build(self) -> NetworkflowmonitorScopeTargetElTargetIdentifierElTargetIdEl {
        NetworkflowmonitorScopeTargetElTargetIdentifierElTargetIdEl { account_id: self.account_id }
    }
}

pub struct NetworkflowmonitorScopeTargetElTargetIdentifierElTargetIdElRef {
    shared: StackShared,
    base: String,
}

impl Ref for NetworkflowmonitorScopeTargetElTargetIdentifierElTargetIdElRef {
    fn new(shared: StackShared, base: String) -> NetworkflowmonitorScopeTargetElTargetIdentifierElTargetIdElRef {
        NetworkflowmonitorScopeTargetElTargetIdentifierElTargetIdElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkflowmonitorScopeTargetElTargetIdentifierElTargetIdElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkflowmonitorScopeTargetElTargetIdentifierElDynamic {
    target_id: Option<DynamicBlock<NetworkflowmonitorScopeTargetElTargetIdentifierElTargetIdEl>>,
}

#[derive(Serialize)]
pub struct NetworkflowmonitorScopeTargetElTargetIdentifierEl {
    target_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_id: Option<Vec<NetworkflowmonitorScopeTargetElTargetIdentifierElTargetIdEl>>,
    dynamic: NetworkflowmonitorScopeTargetElTargetIdentifierElDynamic,
}

impl NetworkflowmonitorScopeTargetElTargetIdentifierEl {
    #[doc = "Set the field `target_id`.\n"]
    pub fn set_target_id(
        mut self,
        v: impl Into<BlockAssignable<NetworkflowmonitorScopeTargetElTargetIdentifierElTargetIdEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.target_id = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.target_id = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for NetworkflowmonitorScopeTargetElTargetIdentifierEl {
    type O = BlockAssignable<NetworkflowmonitorScopeTargetElTargetIdentifierEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkflowmonitorScopeTargetElTargetIdentifierEl {
    #[doc = ""]
    pub target_type: PrimField<String>,
}

impl BuildNetworkflowmonitorScopeTargetElTargetIdentifierEl {
    pub fn build(self) -> NetworkflowmonitorScopeTargetElTargetIdentifierEl {
        NetworkflowmonitorScopeTargetElTargetIdentifierEl {
            target_type: self.target_type,
            target_id: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct NetworkflowmonitorScopeTargetElTargetIdentifierElRef {
    shared: StackShared,
    base: String,
}

impl Ref for NetworkflowmonitorScopeTargetElTargetIdentifierElRef {
    fn new(shared: StackShared, base: String) -> NetworkflowmonitorScopeTargetElTargetIdentifierElRef {
        NetworkflowmonitorScopeTargetElTargetIdentifierElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkflowmonitorScopeTargetElTargetIdentifierElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `target_type` after provisioning.\n"]
    pub fn target_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_type", self.base))
    }

    #[doc = "Get a reference to the value of field `target_id` after provisioning.\n"]
    pub fn target_id(&self) -> ListRef<NetworkflowmonitorScopeTargetElTargetIdentifierElTargetIdElRef> {
        ListRef::new(self.shared().clone(), format!("{}.target_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkflowmonitorScopeTargetElDynamic {
    target_identifier: Option<DynamicBlock<NetworkflowmonitorScopeTargetElTargetIdentifierEl>>,
}

#[derive(Serialize)]
pub struct NetworkflowmonitorScopeTargetEl {
    region: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_identifier: Option<Vec<NetworkflowmonitorScopeTargetElTargetIdentifierEl>>,
    dynamic: NetworkflowmonitorScopeTargetElDynamic,
}

impl NetworkflowmonitorScopeTargetEl {
    #[doc = "Set the field `target_identifier`.\n"]
    pub fn set_target_identifier(
        mut self,
        v: impl Into<BlockAssignable<NetworkflowmonitorScopeTargetElTargetIdentifierEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.target_identifier = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.target_identifier = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for NetworkflowmonitorScopeTargetEl {
    type O = BlockAssignable<NetworkflowmonitorScopeTargetEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkflowmonitorScopeTargetEl {
    #[doc = ""]
    pub region: PrimField<String>,
}

impl BuildNetworkflowmonitorScopeTargetEl {
    pub fn build(self) -> NetworkflowmonitorScopeTargetEl {
        NetworkflowmonitorScopeTargetEl {
            region: self.region,
            target_identifier: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct NetworkflowmonitorScopeTargetElRef {
    shared: StackShared,
    base: String,
}

impl Ref for NetworkflowmonitorScopeTargetElRef {
    fn new(shared: StackShared, base: String) -> NetworkflowmonitorScopeTargetElRef {
        NetworkflowmonitorScopeTargetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkflowmonitorScopeTargetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }

    #[doc = "Get a reference to the value of field `target_identifier` after provisioning.\n"]
    pub fn target_identifier(&self) -> ListRef<NetworkflowmonitorScopeTargetElTargetIdentifierElRef> {
        ListRef::new(self.shared().clone(), format!("{}.target_identifier", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkflowmonitorScopeTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl NetworkflowmonitorScopeTimeoutsEl {
    #[doc =
        "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }

    #[doc =
        "Set the field `delete`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }

    #[doc =
        "Set the field `update`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}

impl ToListMappable for NetworkflowmonitorScopeTimeoutsEl {
    type O = BlockAssignable<NetworkflowmonitorScopeTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkflowmonitorScopeTimeoutsEl {}

impl BuildNetworkflowmonitorScopeTimeoutsEl {
    pub fn build(self) -> NetworkflowmonitorScopeTimeoutsEl {
        NetworkflowmonitorScopeTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct NetworkflowmonitorScopeTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for NetworkflowmonitorScopeTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> NetworkflowmonitorScopeTimeoutsElRef {
        NetworkflowmonitorScopeTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkflowmonitorScopeTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc =
        "Get a reference to the value of field `delete` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc =
        "Get a reference to the value of field `update` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkflowmonitorScopeDynamic {
    target: Option<DynamicBlock<NetworkflowmonitorScopeTargetEl>>,
}
