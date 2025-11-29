use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct EmrcontainersVirtualClusterData {
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
    container_provider: Option<Vec<EmrcontainersVirtualClusterContainerProviderEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<EmrcontainersVirtualClusterTimeoutsEl>,
    dynamic: EmrcontainersVirtualClusterDynamic,
}

struct EmrcontainersVirtualCluster_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<EmrcontainersVirtualClusterData>,
}

#[derive(Clone)]
pub struct EmrcontainersVirtualCluster(Rc<EmrcontainersVirtualCluster_>);

impl EmrcontainersVirtualCluster {
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

    #[doc = "Set the field `container_provider`.\n"]
    pub fn set_container_provider(
        self,
        v: impl Into<BlockAssignable<EmrcontainersVirtualClusterContainerProviderEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().container_provider = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.container_provider = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<EmrcontainersVirtualClusterTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
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
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `container_provider` after provisioning.\n"]
    pub fn container_provider(&self) -> ListRef<EmrcontainersVirtualClusterContainerProviderElRef> {
        ListRef::new(self.shared().clone(), format!("{}.container_provider", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> EmrcontainersVirtualClusterTimeoutsElRef {
        EmrcontainersVirtualClusterTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for EmrcontainersVirtualCluster {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for EmrcontainersVirtualCluster { }

impl ToListMappable for EmrcontainersVirtualCluster {
    type O = ListRef<EmrcontainersVirtualClusterRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for EmrcontainersVirtualCluster_ {
    fn extract_resource_type(&self) -> String {
        "aws_emrcontainers_virtual_cluster".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildEmrcontainersVirtualCluster {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildEmrcontainersVirtualCluster {
    pub fn build(self, stack: &mut Stack) -> EmrcontainersVirtualCluster {
        let out = EmrcontainersVirtualCluster(Rc::new(EmrcontainersVirtualCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(EmrcontainersVirtualClusterData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                container_provider: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct EmrcontainersVirtualClusterRef {
    shared: StackShared,
    base: String,
}

impl Ref for EmrcontainersVirtualClusterRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl EmrcontainersVirtualClusterRef {
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
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `container_provider` after provisioning.\n"]
    pub fn container_provider(&self) -> ListRef<EmrcontainersVirtualClusterContainerProviderElRef> {
        ListRef::new(self.shared().clone(), format!("{}.container_provider", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> EmrcontainersVirtualClusterTimeoutsElRef {
        EmrcontainersVirtualClusterTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct EmrcontainersVirtualClusterContainerProviderElInfoElEksInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    namespace: Option<PrimField<String>>,
}

impl EmrcontainersVirtualClusterContainerProviderElInfoElEksInfoEl {
    #[doc = "Set the field `namespace`.\n"]
    pub fn set_namespace(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.namespace = Some(v.into());
        self
    }
}

impl ToListMappable for EmrcontainersVirtualClusterContainerProviderElInfoElEksInfoEl {
    type O = BlockAssignable<EmrcontainersVirtualClusterContainerProviderElInfoElEksInfoEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrcontainersVirtualClusterContainerProviderElInfoElEksInfoEl {}

impl BuildEmrcontainersVirtualClusterContainerProviderElInfoElEksInfoEl {
    pub fn build(self) -> EmrcontainersVirtualClusterContainerProviderElInfoElEksInfoEl {
        EmrcontainersVirtualClusterContainerProviderElInfoElEksInfoEl {
            namespace: core::default::Default::default(),
        }
    }
}

pub struct EmrcontainersVirtualClusterContainerProviderElInfoElEksInfoElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EmrcontainersVirtualClusterContainerProviderElInfoElEksInfoElRef {
    fn new(shared: StackShared, base: String) -> EmrcontainersVirtualClusterContainerProviderElInfoElEksInfoElRef {
        EmrcontainersVirtualClusterContainerProviderElInfoElEksInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrcontainersVirtualClusterContainerProviderElInfoElEksInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `namespace` after provisioning.\n"]
    pub fn namespace(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.namespace", self.base))
    }
}

#[derive(Serialize, Default)]
struct EmrcontainersVirtualClusterContainerProviderElInfoElDynamic {
    eks_info: Option<DynamicBlock<EmrcontainersVirtualClusterContainerProviderElInfoElEksInfoEl>>,
}

#[derive(Serialize)]
pub struct EmrcontainersVirtualClusterContainerProviderElInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    eks_info: Option<Vec<EmrcontainersVirtualClusterContainerProviderElInfoElEksInfoEl>>,
    dynamic: EmrcontainersVirtualClusterContainerProviderElInfoElDynamic,
}

impl EmrcontainersVirtualClusterContainerProviderElInfoEl {
    #[doc = "Set the field `eks_info`.\n"]
    pub fn set_eks_info(
        mut self,
        v: impl Into<BlockAssignable<EmrcontainersVirtualClusterContainerProviderElInfoElEksInfoEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.eks_info = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.eks_info = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for EmrcontainersVirtualClusterContainerProviderElInfoEl {
    type O = BlockAssignable<EmrcontainersVirtualClusterContainerProviderElInfoEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrcontainersVirtualClusterContainerProviderElInfoEl {}

impl BuildEmrcontainersVirtualClusterContainerProviderElInfoEl {
    pub fn build(self) -> EmrcontainersVirtualClusterContainerProviderElInfoEl {
        EmrcontainersVirtualClusterContainerProviderElInfoEl {
            eks_info: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct EmrcontainersVirtualClusterContainerProviderElInfoElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EmrcontainersVirtualClusterContainerProviderElInfoElRef {
    fn new(shared: StackShared, base: String) -> EmrcontainersVirtualClusterContainerProviderElInfoElRef {
        EmrcontainersVirtualClusterContainerProviderElInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrcontainersVirtualClusterContainerProviderElInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `eks_info` after provisioning.\n"]
    pub fn eks_info(&self) -> ListRef<EmrcontainersVirtualClusterContainerProviderElInfoElEksInfoElRef> {
        ListRef::new(self.shared().clone(), format!("{}.eks_info", self.base))
    }
}

#[derive(Serialize, Default)]
struct EmrcontainersVirtualClusterContainerProviderElDynamic {
    info: Option<DynamicBlock<EmrcontainersVirtualClusterContainerProviderElInfoEl>>,
}

#[derive(Serialize)]
pub struct EmrcontainersVirtualClusterContainerProviderEl {
    id: PrimField<String>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    info: Option<Vec<EmrcontainersVirtualClusterContainerProviderElInfoEl>>,
    dynamic: EmrcontainersVirtualClusterContainerProviderElDynamic,
}

impl EmrcontainersVirtualClusterContainerProviderEl {
    #[doc = "Set the field `info`.\n"]
    pub fn set_info(
        mut self,
        v: impl Into<BlockAssignable<EmrcontainersVirtualClusterContainerProviderElInfoEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.info = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.info = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for EmrcontainersVirtualClusterContainerProviderEl {
    type O = BlockAssignable<EmrcontainersVirtualClusterContainerProviderEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrcontainersVirtualClusterContainerProviderEl {
    #[doc = ""]
    pub id: PrimField<String>,
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl BuildEmrcontainersVirtualClusterContainerProviderEl {
    pub fn build(self) -> EmrcontainersVirtualClusterContainerProviderEl {
        EmrcontainersVirtualClusterContainerProviderEl {
            id: self.id,
            type_: self.type_,
            info: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct EmrcontainersVirtualClusterContainerProviderElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EmrcontainersVirtualClusterContainerProviderElRef {
    fn new(shared: StackShared, base: String) -> EmrcontainersVirtualClusterContainerProviderElRef {
        EmrcontainersVirtualClusterContainerProviderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrcontainersVirtualClusterContainerProviderElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `info` after provisioning.\n"]
    pub fn info(&self) -> ListRef<EmrcontainersVirtualClusterContainerProviderElInfoElRef> {
        ListRef::new(self.shared().clone(), format!("{}.info", self.base))
    }
}

#[derive(Serialize)]
pub struct EmrcontainersVirtualClusterTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}

impl EmrcontainersVirtualClusterTimeoutsEl {
    #[doc = "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }
}

impl ToListMappable for EmrcontainersVirtualClusterTimeoutsEl {
    type O = BlockAssignable<EmrcontainersVirtualClusterTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrcontainersVirtualClusterTimeoutsEl {}

impl BuildEmrcontainersVirtualClusterTimeoutsEl {
    pub fn build(self) -> EmrcontainersVirtualClusterTimeoutsEl {
        EmrcontainersVirtualClusterTimeoutsEl { delete: core::default::Default::default() }
    }
}

pub struct EmrcontainersVirtualClusterTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EmrcontainersVirtualClusterTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> EmrcontainersVirtualClusterTimeoutsElRef {
        EmrcontainersVirtualClusterTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrcontainersVirtualClusterTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }
}

#[derive(Serialize, Default)]
struct EmrcontainersVirtualClusterDynamic {
    container_provider: Option<DynamicBlock<EmrcontainersVirtualClusterContainerProviderEl>>,
}
