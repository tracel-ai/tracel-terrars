use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct NeptunegraphGraphData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_protection: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    graph_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    graph_name_prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_identifier: Option<PrimField<String>>,
    provisioned_memory: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    public_connectivity: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    replica_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<NeptunegraphGraphTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vector_search_configuration: Option<Vec<NeptunegraphGraphVectorSearchConfigurationEl>>,
    dynamic: NeptunegraphGraphDynamic,
}

struct NeptunegraphGraph_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<NeptunegraphGraphData>,
}

#[derive(Clone)]
pub struct NeptunegraphGraph(Rc<NeptunegraphGraph_>);

impl NeptunegraphGraph {
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
        "Set the field `deletion_protection`.\nA value that indicates whether the graph has deletion protection enabled. The graph can't be deleted when deletion protection is enabled."]
    pub fn set_deletion_protection(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().deletion_protection = Some(v.into());
        self
    }

    #[doc =
        "Set the field `graph_name`.\nThe graph name. For example: my-graph-1.\n\t\t\t\t\t\t\t\tThe name must contain from 1 to 63 letters, numbers, or hyphens, \n\t\t\t\t\t\t\t\tand its first character must be a letter. It cannot end with a hyphen or contain two consecutive hyphens.\n\t\t\t\t\t\t\t\tIf you don't specify a graph name, a unique graph name is generated for you using the prefix graph-for, \n\t\t\t\t\t\t\t\tfollowed by a combination of Stack Name and a UUID."]
    pub fn set_graph_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().graph_name = Some(v.into());
        self
    }

    #[doc =
        "Set the field `graph_name_prefix`.\nAllows user to specify name prefix and have remainder of name automatically generated."]
    pub fn set_graph_name_prefix(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().graph_name_prefix = Some(v.into());
        self
    }

    #[doc =
        "Set the field `kms_key_identifier`.\nSpecifies a KMS key to use to encrypt data in the new graph.  Value must be ARN of KMS Key."]
    pub fn set_kms_key_identifier(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().kms_key_identifier = Some(v.into());
        self
    }

    #[doc =
        "Set the field `public_connectivity`.\nSpecifies whether or not the graph can be reachable over the internet. \n\t\t\t\t\t\t\t\tAll access to graphs is IAM authenticated.\n\t\t\t\t\t\t\t\tWhen the graph is publicly available, its domain name system (DNS) endpoint resolves to \n\t\t\t\t\t\t\t\tthe public IP address from the internet. When the graph isn't publicly available, you need \n\t\t\t\t\t\t\t\tto create a PrivateGraphEndpoint in a given VPC to ensure the DNS name resolves to a private \n\t\t\t\t\t\t\t\tIP address that is reachable from the VPC."]
    pub fn set_public_connectivity(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().public_connectivity = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `replica_count`.\nThe number of replicas in other AZs.  Value must be between 0 and 2."]
    pub fn set_replica_count(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().replica_count = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<NeptunegraphGraphTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Set the field `vector_search_configuration`.\n"]
    pub fn set_vector_search_configuration(
        self,
        v: impl Into<BlockAssignable<NeptunegraphGraphVectorSearchConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().vector_search_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.vector_search_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_protection` after provisioning.\nA value that indicates whether the graph has deletion protection enabled. The graph can't be deleted when deletion protection is enabled."]
    pub fn deletion_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `graph_name` after provisioning.\nThe graph name. For example: my-graph-1.\n\t\t\t\t\t\t\t\tThe name must contain from 1 to 63 letters, numbers, or hyphens, \n\t\t\t\t\t\t\t\tand its first character must be a letter. It cannot end with a hyphen or contain two consecutive hyphens.\n\t\t\t\t\t\t\t\tIf you don't specify a graph name, a unique graph name is generated for you using the prefix graph-for, \n\t\t\t\t\t\t\t\tfollowed by a combination of Stack Name and a UUID."]
    pub fn graph_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.graph_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `graph_name_prefix` after provisioning.\nAllows user to specify name prefix and have remainder of name automatically generated."]
    pub fn graph_name_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.graph_name_prefix", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `kms_key_identifier` after provisioning.\nSpecifies a KMS key to use to encrypt data in the new graph.  Value must be ARN of KMS Key."]
    pub fn kms_key_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_identifier", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `provisioned_memory` after provisioning.\nThe provisioned memory-optimized Neptune Capacity Units (m-NCUs) to use for the graph."]
    pub fn provisioned_memory(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.provisioned_memory", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `public_connectivity` after provisioning.\nSpecifies whether or not the graph can be reachable over the internet. \n\t\t\t\t\t\t\t\tAll access to graphs is IAM authenticated.\n\t\t\t\t\t\t\t\tWhen the graph is publicly available, its domain name system (DNS) endpoint resolves to \n\t\t\t\t\t\t\t\tthe public IP address from the internet. When the graph isn't publicly available, you need \n\t\t\t\t\t\t\t\tto create a PrivateGraphEndpoint in a given VPC to ensure the DNS name resolves to a private \n\t\t\t\t\t\t\t\tIP address that is reachable from the VPC."]
    pub fn public_connectivity(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.public_connectivity", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `replica_count` after provisioning.\nThe number of replicas in other AZs.  Value must be between 0 and 2."]
    pub fn replica_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.replica_count", self.extract_ref()))
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
    pub fn timeouts(&self) -> NeptunegraphGraphTimeoutsElRef {
        NeptunegraphGraphTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vector_search_configuration` after provisioning.\n"]
    pub fn vector_search_configuration(&self) -> ListRef<NeptunegraphGraphVectorSearchConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.vector_search_configuration", self.extract_ref()))
    }
}

impl Referable for NeptunegraphGraph {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for NeptunegraphGraph { }

impl ToListMappable for NeptunegraphGraph {
    type O = ListRef<NeptunegraphGraphRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for NeptunegraphGraph_ {
    fn extract_resource_type(&self) -> String {
        "aws_neptunegraph_graph".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildNeptunegraphGraph {
    pub tf_id: String,
    #[doc = "The provisioned memory-optimized Neptune Capacity Units (m-NCUs) to use for the graph."]
    pub provisioned_memory: PrimField<f64>,
}

impl BuildNeptunegraphGraph {
    pub fn build(self, stack: &mut Stack) -> NeptunegraphGraph {
        let out = NeptunegraphGraph(Rc::new(NeptunegraphGraph_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(NeptunegraphGraphData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_protection: core::default::Default::default(),
                graph_name: core::default::Default::default(),
                graph_name_prefix: core::default::Default::default(),
                kms_key_identifier: core::default::Default::default(),
                provisioned_memory: self.provisioned_memory,
                public_connectivity: core::default::Default::default(),
                region: core::default::Default::default(),
                replica_count: core::default::Default::default(),
                tags: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                vector_search_configuration: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct NeptunegraphGraphRef {
    shared: StackShared,
    base: String,
}

impl Ref for NeptunegraphGraphRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl NeptunegraphGraphRef {
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

    #[doc =
        "Get a reference to the value of field `deletion_protection` after provisioning.\nA value that indicates whether the graph has deletion protection enabled. The graph can't be deleted when deletion protection is enabled."]
    pub fn deletion_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `graph_name` after provisioning.\nThe graph name. For example: my-graph-1.\n\t\t\t\t\t\t\t\tThe name must contain from 1 to 63 letters, numbers, or hyphens, \n\t\t\t\t\t\t\t\tand its first character must be a letter. It cannot end with a hyphen or contain two consecutive hyphens.\n\t\t\t\t\t\t\t\tIf you don't specify a graph name, a unique graph name is generated for you using the prefix graph-for, \n\t\t\t\t\t\t\t\tfollowed by a combination of Stack Name and a UUID."]
    pub fn graph_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.graph_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `graph_name_prefix` after provisioning.\nAllows user to specify name prefix and have remainder of name automatically generated."]
    pub fn graph_name_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.graph_name_prefix", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `kms_key_identifier` after provisioning.\nSpecifies a KMS key to use to encrypt data in the new graph.  Value must be ARN of KMS Key."]
    pub fn kms_key_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_identifier", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `provisioned_memory` after provisioning.\nThe provisioned memory-optimized Neptune Capacity Units (m-NCUs) to use for the graph."]
    pub fn provisioned_memory(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.provisioned_memory", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `public_connectivity` after provisioning.\nSpecifies whether or not the graph can be reachable over the internet. \n\t\t\t\t\t\t\t\tAll access to graphs is IAM authenticated.\n\t\t\t\t\t\t\t\tWhen the graph is publicly available, its domain name system (DNS) endpoint resolves to \n\t\t\t\t\t\t\t\tthe public IP address from the internet. When the graph isn't publicly available, you need \n\t\t\t\t\t\t\t\tto create a PrivateGraphEndpoint in a given VPC to ensure the DNS name resolves to a private \n\t\t\t\t\t\t\t\tIP address that is reachable from the VPC."]
    pub fn public_connectivity(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.public_connectivity", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `replica_count` after provisioning.\nThe number of replicas in other AZs.  Value must be between 0 and 2."]
    pub fn replica_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.replica_count", self.extract_ref()))
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
    pub fn timeouts(&self) -> NeptunegraphGraphTimeoutsElRef {
        NeptunegraphGraphTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vector_search_configuration` after provisioning.\n"]
    pub fn vector_search_configuration(&self) -> ListRef<NeptunegraphGraphVectorSearchConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.vector_search_configuration", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct NeptunegraphGraphTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl NeptunegraphGraphTimeoutsEl {
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

impl ToListMappable for NeptunegraphGraphTimeoutsEl {
    type O = BlockAssignable<NeptunegraphGraphTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNeptunegraphGraphTimeoutsEl {}

impl BuildNeptunegraphGraphTimeoutsEl {
    pub fn build(self) -> NeptunegraphGraphTimeoutsEl {
        NeptunegraphGraphTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct NeptunegraphGraphTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for NeptunegraphGraphTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> NeptunegraphGraphTimeoutsElRef {
        NeptunegraphGraphTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NeptunegraphGraphTimeoutsElRef {
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

#[derive(Serialize)]
pub struct NeptunegraphGraphVectorSearchConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    vector_search_dimension: Option<PrimField<f64>>,
}

impl NeptunegraphGraphVectorSearchConfigurationEl {
    #[doc =
        "Set the field `vector_search_dimension`.\nSpecifies the number of dimensions for vector embeddings.  Value must be between 1 and 65,535."]
    pub fn set_vector_search_dimension(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.vector_search_dimension = Some(v.into());
        self
    }
}

impl ToListMappable for NeptunegraphGraphVectorSearchConfigurationEl {
    type O = BlockAssignable<NeptunegraphGraphVectorSearchConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNeptunegraphGraphVectorSearchConfigurationEl {}

impl BuildNeptunegraphGraphVectorSearchConfigurationEl {
    pub fn build(self) -> NeptunegraphGraphVectorSearchConfigurationEl {
        NeptunegraphGraphVectorSearchConfigurationEl { vector_search_dimension: core::default::Default::default() }
    }
}

pub struct NeptunegraphGraphVectorSearchConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for NeptunegraphGraphVectorSearchConfigurationElRef {
    fn new(shared: StackShared, base: String) -> NeptunegraphGraphVectorSearchConfigurationElRef {
        NeptunegraphGraphVectorSearchConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NeptunegraphGraphVectorSearchConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `vector_search_dimension` after provisioning.\nSpecifies the number of dimensions for vector embeddings.  Value must be between 1 and 65,535."]
    pub fn vector_search_dimension(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.vector_search_dimension", self.base))
    }
}

#[derive(Serialize, Default)]
struct NeptunegraphGraphDynamic {
    vector_search_configuration: Option<DynamicBlock<NeptunegraphGraphVectorSearchConfigurationEl>>,
}
