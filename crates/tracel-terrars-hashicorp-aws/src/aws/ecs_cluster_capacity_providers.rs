use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct EcsClusterCapacityProvidersData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    capacity_providers: Option<SetField<PrimField<String>>>,
    cluster_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_capacity_provider_strategy: Option<Vec<EcsClusterCapacityProvidersDefaultCapacityProviderStrategyEl>>,
    dynamic: EcsClusterCapacityProvidersDynamic,
}

struct EcsClusterCapacityProviders_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<EcsClusterCapacityProvidersData>,
}

#[derive(Clone)]
pub struct EcsClusterCapacityProviders(Rc<EcsClusterCapacityProviders_>);

impl EcsClusterCapacityProviders {
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

    #[doc = "Set the field `capacity_providers`.\n"]
    pub fn set_capacity_providers(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().capacity_providers = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `default_capacity_provider_strategy`.\n"]
    pub fn set_default_capacity_provider_strategy(
        self,
        v: impl Into<BlockAssignable<EcsClusterCapacityProvidersDefaultCapacityProviderStrategyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().default_capacity_provider_strategy = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.default_capacity_provider_strategy = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `capacity_providers` after provisioning.\n"]
    pub fn capacity_providers(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.capacity_providers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_name` after provisioning.\n"]
    pub fn cluster_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}

impl Referable for EcsClusterCapacityProviders {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for EcsClusterCapacityProviders { }

impl ToListMappable for EcsClusterCapacityProviders {
    type O = ListRef<EcsClusterCapacityProvidersRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for EcsClusterCapacityProviders_ {
    fn extract_resource_type(&self) -> String {
        "aws_ecs_cluster_capacity_providers".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildEcsClusterCapacityProviders {
    pub tf_id: String,
    #[doc = ""]
    pub cluster_name: PrimField<String>,
}

impl BuildEcsClusterCapacityProviders {
    pub fn build(self, stack: &mut Stack) -> EcsClusterCapacityProviders {
        let out = EcsClusterCapacityProviders(Rc::new(EcsClusterCapacityProviders_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(EcsClusterCapacityProvidersData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                capacity_providers: core::default::Default::default(),
                cluster_name: self.cluster_name,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                default_capacity_provider_strategy: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct EcsClusterCapacityProvidersRef {
    shared: StackShared,
    base: String,
}

impl Ref for EcsClusterCapacityProvidersRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl EcsClusterCapacityProvidersRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `capacity_providers` after provisioning.\n"]
    pub fn capacity_providers(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.capacity_providers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_name` after provisioning.\n"]
    pub fn cluster_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct EcsClusterCapacityProvidersDefaultCapacityProviderStrategyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    base: Option<PrimField<f64>>,
    capacity_provider: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weight: Option<PrimField<f64>>,
}

impl EcsClusterCapacityProvidersDefaultCapacityProviderStrategyEl {
    #[doc = "Set the field `base`.\n"]
    pub fn set_base(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.base = Some(v.into());
        self
    }

    #[doc = "Set the field `weight`.\n"]
    pub fn set_weight(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.weight = Some(v.into());
        self
    }
}

impl ToListMappable for EcsClusterCapacityProvidersDefaultCapacityProviderStrategyEl {
    type O = BlockAssignable<EcsClusterCapacityProvidersDefaultCapacityProviderStrategyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsClusterCapacityProvidersDefaultCapacityProviderStrategyEl {
    #[doc = ""]
    pub capacity_provider: PrimField<String>,
}

impl BuildEcsClusterCapacityProvidersDefaultCapacityProviderStrategyEl {
    pub fn build(self) -> EcsClusterCapacityProvidersDefaultCapacityProviderStrategyEl {
        EcsClusterCapacityProvidersDefaultCapacityProviderStrategyEl {
            base: core::default::Default::default(),
            capacity_provider: self.capacity_provider,
            weight: core::default::Default::default(),
        }
    }
}

pub struct EcsClusterCapacityProvidersDefaultCapacityProviderStrategyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EcsClusterCapacityProvidersDefaultCapacityProviderStrategyElRef {
    fn new(shared: StackShared, base: String) -> EcsClusterCapacityProvidersDefaultCapacityProviderStrategyElRef {
        EcsClusterCapacityProvidersDefaultCapacityProviderStrategyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsClusterCapacityProvidersDefaultCapacityProviderStrategyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `base` after provisioning.\n"]
    pub fn base(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.base", self.base))
    }

    #[doc = "Get a reference to the value of field `capacity_provider` after provisioning.\n"]
    pub fn capacity_provider(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.capacity_provider", self.base))
    }

    #[doc = "Get a reference to the value of field `weight` after provisioning.\n"]
    pub fn weight(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.weight", self.base))
    }
}

#[derive(Serialize, Default)]
struct EcsClusterCapacityProvidersDynamic {
    default_capacity_provider_strategy: Option<
        DynamicBlock<EcsClusterCapacityProvidersDefaultCapacityProviderStrategyEl>,
    >,
}
