use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct DataEmrcontainersVirtualClusterData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    virtual_cluster_id: PrimField<String>,
}

struct DataEmrcontainersVirtualCluster_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataEmrcontainersVirtualClusterData>,
}

#[derive(Clone)]
pub struct DataEmrcontainersVirtualCluster(Rc<DataEmrcontainersVirtualCluster_>);

impl DataEmrcontainersVirtualCluster {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderAws) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
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

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `container_provider` after provisioning.\n"]
    pub fn container_provider(
        &self,
    ) -> ListRef<DataEmrcontainersVirtualClusterContainerProviderElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.container_provider", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created_at", self.extract_ref()),
        )
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

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `virtual_cluster_id` after provisioning.\n"]
    pub fn virtual_cluster_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.virtual_cluster_id", self.extract_ref()),
        )
    }
}

impl Referable for DataEmrcontainersVirtualCluster {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Datasource for DataEmrcontainersVirtualCluster {}

impl ToListMappable for DataEmrcontainersVirtualCluster {
    type O = ListRef<DataEmrcontainersVirtualClusterRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataEmrcontainersVirtualCluster_ {
    fn extract_datasource_type(&self) -> String {
        "aws_emrcontainers_virtual_cluster".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataEmrcontainersVirtualCluster {
    pub tf_id: String,
    #[doc = ""]
    pub virtual_cluster_id: PrimField<String>,
}

impl BuildDataEmrcontainersVirtualCluster {
    pub fn build(self, stack: &mut Stack) -> DataEmrcontainersVirtualCluster {
        let out = DataEmrcontainersVirtualCluster(Rc::new(DataEmrcontainersVirtualCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataEmrcontainersVirtualClusterData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                virtual_cluster_id: self.virtual_cluster_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataEmrcontainersVirtualClusterRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEmrcontainersVirtualClusterRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl DataEmrcontainersVirtualClusterRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `container_provider` after provisioning.\n"]
    pub fn container_provider(
        &self,
    ) -> ListRef<DataEmrcontainersVirtualClusterContainerProviderElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.container_provider", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created_at", self.extract_ref()),
        )
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

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `virtual_cluster_id` after provisioning.\n"]
    pub fn virtual_cluster_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.virtual_cluster_id", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DataEmrcontainersVirtualClusterContainerProviderElInfoElEksInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    namespace: Option<PrimField<String>>,
}

impl DataEmrcontainersVirtualClusterContainerProviderElInfoElEksInfoEl {
    #[doc = "Set the field `namespace`.\n"]
    pub fn set_namespace(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.namespace = Some(v.into());
        self
    }
}

impl ToListMappable for DataEmrcontainersVirtualClusterContainerProviderElInfoElEksInfoEl {
    type O = BlockAssignable<DataEmrcontainersVirtualClusterContainerProviderElInfoElEksInfoEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEmrcontainersVirtualClusterContainerProviderElInfoElEksInfoEl {}

impl BuildDataEmrcontainersVirtualClusterContainerProviderElInfoElEksInfoEl {
    pub fn build(self) -> DataEmrcontainersVirtualClusterContainerProviderElInfoElEksInfoEl {
        DataEmrcontainersVirtualClusterContainerProviderElInfoElEksInfoEl {
            namespace: core::default::Default::default(),
        }
    }
}

pub struct DataEmrcontainersVirtualClusterContainerProviderElInfoElEksInfoElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEmrcontainersVirtualClusterContainerProviderElInfoElEksInfoElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataEmrcontainersVirtualClusterContainerProviderElInfoElEksInfoElRef {
        DataEmrcontainersVirtualClusterContainerProviderElInfoElEksInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEmrcontainersVirtualClusterContainerProviderElInfoElEksInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `namespace` after provisioning.\n"]
    pub fn namespace(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.namespace", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEmrcontainersVirtualClusterContainerProviderElInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    eks_info: Option<ListField<DataEmrcontainersVirtualClusterContainerProviderElInfoElEksInfoEl>>,
}

impl DataEmrcontainersVirtualClusterContainerProviderElInfoEl {
    #[doc = "Set the field `eks_info`.\n"]
    pub fn set_eks_info(
        mut self,
        v: impl Into<ListField<DataEmrcontainersVirtualClusterContainerProviderElInfoElEksInfoEl>>,
    ) -> Self {
        self.eks_info = Some(v.into());
        self
    }
}

impl ToListMappable for DataEmrcontainersVirtualClusterContainerProviderElInfoEl {
    type O = BlockAssignable<DataEmrcontainersVirtualClusterContainerProviderElInfoEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEmrcontainersVirtualClusterContainerProviderElInfoEl {}

impl BuildDataEmrcontainersVirtualClusterContainerProviderElInfoEl {
    pub fn build(self) -> DataEmrcontainersVirtualClusterContainerProviderElInfoEl {
        DataEmrcontainersVirtualClusterContainerProviderElInfoEl {
            eks_info: core::default::Default::default(),
        }
    }
}

pub struct DataEmrcontainersVirtualClusterContainerProviderElInfoElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEmrcontainersVirtualClusterContainerProviderElInfoElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataEmrcontainersVirtualClusterContainerProviderElInfoElRef {
        DataEmrcontainersVirtualClusterContainerProviderElInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEmrcontainersVirtualClusterContainerProviderElInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `eks_info` after provisioning.\n"]
    pub fn eks_info(
        &self,
    ) -> ListRef<DataEmrcontainersVirtualClusterContainerProviderElInfoElEksInfoElRef> {
        ListRef::new(self.shared().clone(), format!("{}.eks_info", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEmrcontainersVirtualClusterContainerProviderEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    info: Option<ListField<DataEmrcontainersVirtualClusterContainerProviderElInfoEl>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}

impl DataEmrcontainersVirtualClusterContainerProviderEl {
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc = "Set the field `info`.\n"]
    pub fn set_info(
        mut self,
        v: impl Into<ListField<DataEmrcontainersVirtualClusterContainerProviderElInfoEl>>,
    ) -> Self {
        self.info = Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}

impl ToListMappable for DataEmrcontainersVirtualClusterContainerProviderEl {
    type O = BlockAssignable<DataEmrcontainersVirtualClusterContainerProviderEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEmrcontainersVirtualClusterContainerProviderEl {}

impl BuildDataEmrcontainersVirtualClusterContainerProviderEl {
    pub fn build(self) -> DataEmrcontainersVirtualClusterContainerProviderEl {
        DataEmrcontainersVirtualClusterContainerProviderEl {
            id: core::default::Default::default(),
            info: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct DataEmrcontainersVirtualClusterContainerProviderElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEmrcontainersVirtualClusterContainerProviderElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataEmrcontainersVirtualClusterContainerProviderElRef {
        DataEmrcontainersVirtualClusterContainerProviderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEmrcontainersVirtualClusterContainerProviderElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `info` after provisioning.\n"]
    pub fn info(&self) -> ListRef<DataEmrcontainersVirtualClusterContainerProviderElInfoElRef> {
        ListRef::new(self.shared().clone(), format!("{}.info", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
