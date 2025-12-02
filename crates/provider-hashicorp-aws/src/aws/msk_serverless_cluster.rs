use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct MskServerlessClusterData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    cluster_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_authentication: Option<Vec<MskServerlessClusterClientAuthenticationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<MskServerlessClusterTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_config: Option<Vec<MskServerlessClusterVpcConfigEl>>,
    dynamic: MskServerlessClusterDynamic,
}
struct MskServerlessCluster_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<MskServerlessClusterData>,
}
#[derive(Clone)]
pub struct MskServerlessCluster(Rc<MskServerlessCluster_>);
impl MskServerlessCluster {
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
    #[doc = "Set the field `client_authentication`.\n"]
    pub fn set_client_authentication(
        self,
        v: impl Into<BlockAssignable<MskServerlessClusterClientAuthenticationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().client_authentication = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.client_authentication = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<MskServerlessClusterTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Set the field `vpc_config`.\n"]
    pub fn set_vpc_config(
        self,
        v: impl Into<BlockAssignable<MskServerlessClusterVpcConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().vpc_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.vpc_config = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `bootstrap_brokers_sasl_iam` after provisioning.\n"]
    pub fn bootstrap_brokers_sasl_iam(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bootstrap_brokers_sasl_iam", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cluster_name` after provisioning.\n"]
    pub fn cluster_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cluster_uuid` after provisioning.\n"]
    pub fn cluster_uuid(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster_uuid", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `client_authentication` after provisioning.\n"]
    pub fn client_authentication(&self) -> ListRef<MskServerlessClusterClientAuthenticationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.client_authentication", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MskServerlessClusterTimeoutsElRef {
        MskServerlessClusterTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vpc_config` after provisioning.\n"]
    pub fn vpc_config(&self) -> ListRef<MskServerlessClusterVpcConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vpc_config", self.extract_ref()),
        )
    }
}
impl Referable for MskServerlessCluster {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for MskServerlessCluster {}
impl ToListMappable for MskServerlessCluster {
    type O = ListRef<MskServerlessClusterRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for MskServerlessCluster_ {
    fn extract_resource_type(&self) -> String {
        "aws_msk_serverless_cluster".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildMskServerlessCluster {
    pub tf_id: String,
    #[doc = ""]
    pub cluster_name: PrimField<String>,
}
impl BuildMskServerlessCluster {
    pub fn build(self, stack: &mut Stack) -> MskServerlessCluster {
        let out = MskServerlessCluster(Rc::new(MskServerlessCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(MskServerlessClusterData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                cluster_name: self.cluster_name,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                client_authentication: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                vpc_config: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct MskServerlessClusterRef {
    shared: StackShared,
    base: String,
}
impl Ref for MskServerlessClusterRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl MskServerlessClusterRef {
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
    #[doc = "Get a reference to the value of field `bootstrap_brokers_sasl_iam` after provisioning.\n"]
    pub fn bootstrap_brokers_sasl_iam(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bootstrap_brokers_sasl_iam", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cluster_name` after provisioning.\n"]
    pub fn cluster_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cluster_uuid` after provisioning.\n"]
    pub fn cluster_uuid(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster_uuid", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `client_authentication` after provisioning.\n"]
    pub fn client_authentication(&self) -> ListRef<MskServerlessClusterClientAuthenticationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.client_authentication", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MskServerlessClusterTimeoutsElRef {
        MskServerlessClusterTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vpc_config` after provisioning.\n"]
    pub fn vpc_config(&self) -> ListRef<MskServerlessClusterVpcConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vpc_config", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct MskServerlessClusterClientAuthenticationElSaslElIamEl {
    enabled: PrimField<bool>,
}
impl MskServerlessClusterClientAuthenticationElSaslElIamEl {}
impl ToListMappable for MskServerlessClusterClientAuthenticationElSaslElIamEl {
    type O = BlockAssignable<MskServerlessClusterClientAuthenticationElSaslElIamEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMskServerlessClusterClientAuthenticationElSaslElIamEl {
    #[doc = ""]
    pub enabled: PrimField<bool>,
}
impl BuildMskServerlessClusterClientAuthenticationElSaslElIamEl {
    pub fn build(self) -> MskServerlessClusterClientAuthenticationElSaslElIamEl {
        MskServerlessClusterClientAuthenticationElSaslElIamEl {
            enabled: self.enabled,
        }
    }
}
pub struct MskServerlessClusterClientAuthenticationElSaslElIamElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MskServerlessClusterClientAuthenticationElSaslElIamElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MskServerlessClusterClientAuthenticationElSaslElIamElRef {
        MskServerlessClusterClientAuthenticationElSaslElIamElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MskServerlessClusterClientAuthenticationElSaslElIamElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize, Default)]
struct MskServerlessClusterClientAuthenticationElSaslElDynamic {
    iam: Option<DynamicBlock<MskServerlessClusterClientAuthenticationElSaslElIamEl>>,
}
#[derive(Serialize)]
pub struct MskServerlessClusterClientAuthenticationElSaslEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    iam: Option<Vec<MskServerlessClusterClientAuthenticationElSaslElIamEl>>,
    dynamic: MskServerlessClusterClientAuthenticationElSaslElDynamic,
}
impl MskServerlessClusterClientAuthenticationElSaslEl {
    #[doc = "Set the field `iam`.\n"]
    pub fn set_iam(
        mut self,
        v: impl Into<BlockAssignable<MskServerlessClusterClientAuthenticationElSaslElIamEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.iam = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.iam = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for MskServerlessClusterClientAuthenticationElSaslEl {
    type O = BlockAssignable<MskServerlessClusterClientAuthenticationElSaslEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMskServerlessClusterClientAuthenticationElSaslEl {}
impl BuildMskServerlessClusterClientAuthenticationElSaslEl {
    pub fn build(self) -> MskServerlessClusterClientAuthenticationElSaslEl {
        MskServerlessClusterClientAuthenticationElSaslEl {
            iam: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct MskServerlessClusterClientAuthenticationElSaslElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MskServerlessClusterClientAuthenticationElSaslElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MskServerlessClusterClientAuthenticationElSaslElRef {
        MskServerlessClusterClientAuthenticationElSaslElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MskServerlessClusterClientAuthenticationElSaslElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `iam` after provisioning.\n"]
    pub fn iam(&self) -> ListRef<MskServerlessClusterClientAuthenticationElSaslElIamElRef> {
        ListRef::new(self.shared().clone(), format!("{}.iam", self.base))
    }
}
#[derive(Serialize, Default)]
struct MskServerlessClusterClientAuthenticationElDynamic {
    sasl: Option<DynamicBlock<MskServerlessClusterClientAuthenticationElSaslEl>>,
}
#[derive(Serialize)]
pub struct MskServerlessClusterClientAuthenticationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    sasl: Option<Vec<MskServerlessClusterClientAuthenticationElSaslEl>>,
    dynamic: MskServerlessClusterClientAuthenticationElDynamic,
}
impl MskServerlessClusterClientAuthenticationEl {
    #[doc = "Set the field `sasl`.\n"]
    pub fn set_sasl(
        mut self,
        v: impl Into<BlockAssignable<MskServerlessClusterClientAuthenticationElSaslEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.sasl = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.sasl = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for MskServerlessClusterClientAuthenticationEl {
    type O = BlockAssignable<MskServerlessClusterClientAuthenticationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMskServerlessClusterClientAuthenticationEl {}
impl BuildMskServerlessClusterClientAuthenticationEl {
    pub fn build(self) -> MskServerlessClusterClientAuthenticationEl {
        MskServerlessClusterClientAuthenticationEl {
            sasl: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct MskServerlessClusterClientAuthenticationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MskServerlessClusterClientAuthenticationElRef {
    fn new(shared: StackShared, base: String) -> MskServerlessClusterClientAuthenticationElRef {
        MskServerlessClusterClientAuthenticationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MskServerlessClusterClientAuthenticationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `sasl` after provisioning.\n"]
    pub fn sasl(&self) -> ListRef<MskServerlessClusterClientAuthenticationElSaslElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sasl", self.base))
    }
}
#[derive(Serialize)]
pub struct MskServerlessClusterTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}
impl MskServerlessClusterTimeoutsEl {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }
    #[doc = "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }
}
impl ToListMappable for MskServerlessClusterTimeoutsEl {
    type O = BlockAssignable<MskServerlessClusterTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMskServerlessClusterTimeoutsEl {}
impl BuildMskServerlessClusterTimeoutsEl {
    pub fn build(self) -> MskServerlessClusterTimeoutsEl {
        MskServerlessClusterTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}
pub struct MskServerlessClusterTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MskServerlessClusterTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> MskServerlessClusterTimeoutsElRef {
        MskServerlessClusterTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MskServerlessClusterTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }
    #[doc = "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }
}
#[derive(Serialize)]
pub struct MskServerlessClusterVpcConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    security_group_ids: Option<SetField<PrimField<String>>>,
    subnet_ids: SetField<PrimField<String>>,
}
impl MskServerlessClusterVpcConfigEl {
    #[doc = "Set the field `security_group_ids`.\n"]
    pub fn set_security_group_ids(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.security_group_ids = Some(v.into());
        self
    }
}
impl ToListMappable for MskServerlessClusterVpcConfigEl {
    type O = BlockAssignable<MskServerlessClusterVpcConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMskServerlessClusterVpcConfigEl {
    #[doc = ""]
    pub subnet_ids: SetField<PrimField<String>>,
}
impl BuildMskServerlessClusterVpcConfigEl {
    pub fn build(self) -> MskServerlessClusterVpcConfigEl {
        MskServerlessClusterVpcConfigEl {
            security_group_ids: core::default::Default::default(),
            subnet_ids: self.subnet_ids,
        }
    }
}
pub struct MskServerlessClusterVpcConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MskServerlessClusterVpcConfigElRef {
    fn new(shared: StackShared, base: String) -> MskServerlessClusterVpcConfigElRef {
        MskServerlessClusterVpcConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MskServerlessClusterVpcConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.security_group_ids", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.base))
    }
}
#[derive(Serialize, Default)]
struct MskServerlessClusterDynamic {
    client_authentication: Option<DynamicBlock<MskServerlessClusterClientAuthenticationEl>>,
    vpc_config: Option<DynamicBlock<MskServerlessClusterVpcConfigEl>>,
}
