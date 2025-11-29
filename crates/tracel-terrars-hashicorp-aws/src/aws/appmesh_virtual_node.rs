use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct AppmeshVirtualNodeData {
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
    mesh_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mesh_owner: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spec: Option<Vec<AppmeshVirtualNodeSpecEl>>,
    dynamic: AppmeshVirtualNodeDynamic,
}

struct AppmeshVirtualNode_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<AppmeshVirtualNodeData>,
}

#[derive(Clone)]
pub struct AppmeshVirtualNode(Rc<AppmeshVirtualNode_>);

impl AppmeshVirtualNode {
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

    #[doc = "Set the field `mesh_owner`.\n"]
    pub fn set_mesh_owner(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().mesh_owner = Some(v.into());
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

    #[doc = "Set the field `spec`.\n"]
    pub fn set_spec(self, v: impl Into<BlockAssignable<AppmeshVirtualNodeSpecEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().spec = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.spec = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_date` after provisioning.\n"]
    pub fn created_date(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_updated_date` after provisioning.\n"]
    pub fn last_updated_date(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.last_updated_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mesh_name` after provisioning.\n"]
    pub fn mesh_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mesh_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mesh_owner` after provisioning.\n"]
    pub fn mesh_owner(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mesh_owner", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `resource_owner` after provisioning.\n"]
    pub fn resource_owner(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_owner", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spec` after provisioning.\n"]
    pub fn spec(&self) -> ListRef<AppmeshVirtualNodeSpecElRef> {
        ListRef::new(self.shared().clone(), format!("{}.spec", self.extract_ref()))
    }
}

impl Referable for AppmeshVirtualNode {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for AppmeshVirtualNode { }

impl ToListMappable for AppmeshVirtualNode {
    type O = ListRef<AppmeshVirtualNodeRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for AppmeshVirtualNode_ {
    fn extract_resource_type(&self) -> String {
        "aws_appmesh_virtual_node".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAppmeshVirtualNode {
    pub tf_id: String,
    #[doc = ""]
    pub mesh_name: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildAppmeshVirtualNode {
    pub fn build(self, stack: &mut Stack) -> AppmeshVirtualNode {
        let out = AppmeshVirtualNode(Rc::new(AppmeshVirtualNode_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(AppmeshVirtualNodeData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                mesh_name: self.mesh_name,
                mesh_owner: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                spec: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AppmeshVirtualNodeRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshVirtualNodeRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl AppmeshVirtualNodeRef {
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

    #[doc = "Get a reference to the value of field `created_date` after provisioning.\n"]
    pub fn created_date(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_updated_date` after provisioning.\n"]
    pub fn last_updated_date(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.last_updated_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mesh_name` after provisioning.\n"]
    pub fn mesh_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mesh_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mesh_owner` after provisioning.\n"]
    pub fn mesh_owner(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mesh_owner", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `resource_owner` after provisioning.\n"]
    pub fn resource_owner(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_owner", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spec` after provisioning.\n"]
    pub fn spec(&self) -> ListRef<AppmeshVirtualNodeSpecElRef> {
        ListRef::new(self.shared().clone(), format!("{}.spec", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElFileEl {
    certificate_chain: PrimField<String>,
    private_key: PrimField<String>,
}

impl AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElFileEl { }

impl ToListMappable for AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElFileEl {
    type O =
        BlockAssignable<AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElFileEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElFileEl {
    #[doc = ""]
    pub certificate_chain: PrimField<String>,
    #[doc = ""]
    pub private_key: PrimField<String>,
}

impl BuildAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElFileEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElFileEl {
        AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElFileEl {
            certificate_chain: self.certificate_chain,
            private_key: self.private_key,
        }
    }
}

pub struct AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElFileElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElFileElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElFileElRef {
        AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElFileElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElFileElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificate_chain` after provisioning.\n"]
    pub fn certificate_chain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.certificate_chain", self.base))
    }

    #[doc = "Get a reference to the value of field `private_key` after provisioning.\n"]
    pub fn private_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.private_key", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElSdsEl {
    secret_name: PrimField<String>,
}

impl AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElSdsEl { }

impl ToListMappable for AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElSdsEl {
    type O = BlockAssignable<AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElSdsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElSdsEl {
    #[doc = ""]
    pub secret_name: PrimField<String>,
}

impl BuildAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElSdsEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElSdsEl {
        AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElSdsEl {
            secret_name: self.secret_name,
        }
    }
}

pub struct AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElSdsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElSdsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElSdsElRef {
        AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElSdsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElSdsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `secret_name` after provisioning.\n"]
    pub fn secret_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElDynamic {
    file: Option<
        DynamicBlock<AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElFileEl>,
    >,
    sds: Option<
        DynamicBlock<AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElSdsEl>,
    >,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    file: Option<Vec<AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElFileEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sds: Option<Vec<AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElSdsEl>>,
    dynamic: AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElDynamic,
}

impl AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateEl {
    #[doc = "Set the field `file`.\n"]
    pub fn set_file(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElFileEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.file = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.file = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sds`.\n"]
    pub fn set_sds(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElSdsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.sds = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.sds = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateEl {
    type O = BlockAssignable<AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateEl {}

impl BuildAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateEl {
        AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateEl {
            file: core::default::Default::default(),
            sds: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElRef {
        AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `file` after provisioning.\n"]
    pub fn file(
        &self,
    ) -> ListRef<AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElFileElRef> {
        ListRef::new(self.shared().clone(), format!("{}.file", self.base))
    }

    #[doc = "Get a reference to the value of field `sds` after provisioning.\n"]
    pub fn sds(
        &self,
    ) -> ListRef<AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElSdsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sds", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl {
    exact: SetField<PrimField<String>>,
}

impl AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl {

}

impl ToListMappable for AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl {
    type O =
        BlockAssignable<
            AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl {
    #[doc = ""]
    pub exact: SetField<PrimField<String>>,
}

impl BuildAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl {
    pub fn build(
        self,
    ) -> AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl {
        AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl {
            exact: self.exact,
        }
    }
}

pub struct AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchElRef {
        AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `exact` after provisioning.\n"]
    pub fn exact(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.exact", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesElDynamic {
    match_: Option<
        DynamicBlock<
            AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl {
    #[serde(rename = "match", skip_serializing_if = "Option::is_none")]
    match_: Option<
        Vec<
            AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl,
        >,
    >,
    dynamic: AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesElDynamic,
}

impl AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl {
    #[doc = "Set the field `match_`.\n"]
    pub fn set_match(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.match_ = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.match_ = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl {
    type O =
        BlockAssignable<
            AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl {}

impl BuildAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl {
    pub fn build(
        self,
    ) -> AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl {
        AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl {
            match_: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesElRef {
        AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(
        &self,
    ) -> ListRef<
        AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElAcmEl {
    certificate_authority_arns: SetField<PrimField<String>>,
}

impl AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElAcmEl { }

impl ToListMappable for AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElAcmEl {
    type O =
        BlockAssignable<
            AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElAcmEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElAcmEl {
    #[doc = ""]
    pub certificate_authority_arns: SetField<PrimField<String>>,
}

impl BuildAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElAcmEl {
    pub fn build(
        self,
    ) -> AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElAcmEl {
        AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElAcmEl {
            certificate_authority_arns: self.certificate_authority_arns,
        }
    }
}

pub struct AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElAcmElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElAcmElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElAcmElRef {
        AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElAcmElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElAcmElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificate_authority_arns` after provisioning.\n"]
    pub fn certificate_authority_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.certificate_authority_arns", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElFileEl {
    certificate_chain: PrimField<String>,
}

impl AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElFileEl { }

impl ToListMappable for AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElFileEl {
    type O =
        BlockAssignable<
            AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElFileEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElFileEl {
    #[doc = ""]
    pub certificate_chain: PrimField<String>,
}

impl BuildAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElFileEl {
    pub fn build(
        self,
    ) -> AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElFileEl {
        AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElFileEl {
            certificate_chain: self.certificate_chain,
        }
    }
}

pub struct AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElFileElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElFileElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElFileElRef {
        AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElFileElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElFileElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificate_chain` after provisioning.\n"]
    pub fn certificate_chain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.certificate_chain", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElSdsEl {
    secret_name: PrimField<String>,
}

impl AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElSdsEl { }

impl ToListMappable for AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElSdsEl {
    type O =
        BlockAssignable<
            AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElSdsEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElSdsEl {
    #[doc = ""]
    pub secret_name: PrimField<String>,
}

impl BuildAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElSdsEl {
    pub fn build(
        self,
    ) -> AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElSdsEl {
        AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElSdsEl {
            secret_name: self.secret_name,
        }
    }
}

pub struct AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElSdsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElSdsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElSdsElRef {
        AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElSdsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElSdsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `secret_name` after provisioning.\n"]
    pub fn secret_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElDynamic {
    acm: Option<
        DynamicBlock<AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElAcmEl>,
    >,
    file: Option<
        DynamicBlock<AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElFileEl>,
    >,
    sds: Option<
        DynamicBlock<AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElSdsEl>,
    >,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    acm: Option<Vec<AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElAcmEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file: Option<Vec<AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElFileEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sds: Option<Vec<AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElSdsEl>>,
    dynamic: AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElDynamic,
}

impl AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustEl {
    #[doc = "Set the field `acm`.\n"]
    pub fn set_acm(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElAcmEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.acm = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.acm = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `file`.\n"]
    pub fn set_file(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElFileEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.file = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.file = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sds`.\n"]
    pub fn set_sds(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElSdsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.sds = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.sds = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustEl {
    type O =
        BlockAssignable<AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustEl {}

impl BuildAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustEl {
        AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustEl {
            acm: core::default::Default::default(),
            file: core::default::Default::default(),
            sds: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElRef {
        AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `acm` after provisioning.\n"]
    pub fn acm(
        &self,
    ) -> ListRef<AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElAcmElRef> {
        ListRef::new(self.shared().clone(), format!("{}.acm", self.base))
    }

    #[doc = "Get a reference to the value of field `file` after provisioning.\n"]
    pub fn file(
        &self,
    ) -> ListRef<AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElFileElRef> {
        ListRef::new(self.shared().clone(), format!("{}.file", self.base))
    }

    #[doc = "Get a reference to the value of field `sds` after provisioning.\n"]
    pub fn sds(
        &self,
    ) -> ListRef<AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElSdsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sds", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElDynamic {
    subject_alternative_names: Option<
        DynamicBlock<
            AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl,
        >,
    >,
    trust: Option<
        DynamicBlock<AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustEl>,
    >,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    subject_alternative_names: Option<
        Vec<AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    trust: Option<Vec<AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustEl>>,
    dynamic: AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElDynamic,
}

impl AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationEl {
    #[doc = "Set the field `subject_alternative_names`.\n"]
    pub fn set_subject_alternative_names(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.subject_alternative_names = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.subject_alternative_names = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `trust`.\n"]
    pub fn set_trust(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.trust = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.trust = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationEl {
    type O = BlockAssignable<AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationEl {}

impl BuildAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationEl {
        AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationEl {
            subject_alternative_names: core::default::Default::default(),
            trust: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElRef {
        AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `subject_alternative_names` after provisioning.\n"]
    pub fn subject_alternative_names(
        &self,
    ) -> ListRef<
        AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.subject_alternative_names", self.base))
    }

    #[doc = "Get a reference to the value of field `trust` after provisioning.\n"]
    pub fn trust(
        &self,
    ) -> ListRef<AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElRef> {
        ListRef::new(self.shared().clone(), format!("{}.trust", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElDynamic {
    certificate: Option<
        DynamicBlock<AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateEl>,
    >,
    validation: Option<
        DynamicBlock<AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationEl>,
    >,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enforce: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ports: Option<SetField<PrimField<f64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate: Option<Vec<AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    validation: Option<Vec<AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationEl>>,
    dynamic: AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElDynamic,
}

impl AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsEl {
    #[doc = "Set the field `enforce`.\n"]
    pub fn set_enforce(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enforce = Some(v.into());
        self
    }

    #[doc = "Set the field `ports`.\n"]
    pub fn set_ports(mut self, v: impl Into<SetField<PrimField<f64>>>) -> Self {
        self.ports = Some(v.into());
        self
    }

    #[doc = "Set the field `certificate`.\n"]
    pub fn set_certificate(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.certificate = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.certificate = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `validation`.\n"]
    pub fn set_validation(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.validation = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.validation = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsEl {
    type O = BlockAssignable<AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsEl {}

impl BuildAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsEl {
        AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsEl {
            enforce: core::default::Default::default(),
            ports: core::default::Default::default(),
            certificate: core::default::Default::default(),
            validation: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElRef {
        AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enforce` after provisioning.\n"]
    pub fn enforce(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enforce", self.base))
    }

    #[doc = "Get a reference to the value of field `ports` after provisioning.\n"]
    pub fn ports(&self) -> SetRef<PrimExpr<f64>> {
        SetRef::new(self.shared().clone(), format!("{}.ports", self.base))
    }

    #[doc = "Get a reference to the value of field `certificate` after provisioning.\n"]
    pub fn certificate(
        &self,
    ) -> ListRef<AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElRef> {
        ListRef::new(self.shared().clone(), format!("{}.certificate", self.base))
    }

    #[doc = "Get a reference to the value of field `validation` after provisioning.\n"]
    pub fn validation(
        &self,
    ) -> ListRef<AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.validation", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElDynamic {
    tls: Option<DynamicBlock<AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsEl>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    tls: Option<Vec<AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsEl>>,
    dynamic: AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElDynamic,
}

impl AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyEl {
    #[doc = "Set the field `tls`.\n"]
    pub fn set_tls(
        mut self,
        v: impl Into<BlockAssignable<AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.tls = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.tls = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyEl {
    type O = BlockAssignable<AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyEl {}

impl BuildAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyEl {
        AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyEl {
            tls: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElRef {
    fn new(shared: StackShared, base: String) -> AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElRef {
        AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `tls` after provisioning.\n"]
    pub fn tls(&self) -> ListRef<AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tls", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualNodeSpecElBackendElVirtualServiceElDynamic {
    client_policy: Option<DynamicBlock<AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyEl>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElBackendElVirtualServiceEl {
    virtual_service_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_policy: Option<Vec<AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyEl>>,
    dynamic: AppmeshVirtualNodeSpecElBackendElVirtualServiceElDynamic,
}

impl AppmeshVirtualNodeSpecElBackendElVirtualServiceEl {
    #[doc = "Set the field `client_policy`.\n"]
    pub fn set_client_policy(
        mut self,
        v: impl Into<BlockAssignable<AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.client_policy = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.client_policy = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppmeshVirtualNodeSpecElBackendElVirtualServiceEl {
    type O = BlockAssignable<AppmeshVirtualNodeSpecElBackendElVirtualServiceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElBackendElVirtualServiceEl {
    #[doc = ""]
    pub virtual_service_name: PrimField<String>,
}

impl BuildAppmeshVirtualNodeSpecElBackendElVirtualServiceEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElBackendElVirtualServiceEl {
        AppmeshVirtualNodeSpecElBackendElVirtualServiceEl {
            virtual_service_name: self.virtual_service_name,
            client_policy: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppmeshVirtualNodeSpecElBackendElVirtualServiceElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshVirtualNodeSpecElBackendElVirtualServiceElRef {
    fn new(shared: StackShared, base: String) -> AppmeshVirtualNodeSpecElBackendElVirtualServiceElRef {
        AppmeshVirtualNodeSpecElBackendElVirtualServiceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElBackendElVirtualServiceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `virtual_service_name` after provisioning.\n"]
    pub fn virtual_service_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.virtual_service_name", self.base))
    }

    #[doc = "Get a reference to the value of field `client_policy` after provisioning.\n"]
    pub fn client_policy(&self) -> ListRef<AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.client_policy", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualNodeSpecElBackendElDynamic {
    virtual_service: Option<DynamicBlock<AppmeshVirtualNodeSpecElBackendElVirtualServiceEl>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElBackendEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    virtual_service: Option<Vec<AppmeshVirtualNodeSpecElBackendElVirtualServiceEl>>,
    dynamic: AppmeshVirtualNodeSpecElBackendElDynamic,
}

impl AppmeshVirtualNodeSpecElBackendEl {
    #[doc = "Set the field `virtual_service`.\n"]
    pub fn set_virtual_service(
        mut self,
        v: impl Into<BlockAssignable<AppmeshVirtualNodeSpecElBackendElVirtualServiceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.virtual_service = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.virtual_service = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppmeshVirtualNodeSpecElBackendEl {
    type O = BlockAssignable<AppmeshVirtualNodeSpecElBackendEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElBackendEl {}

impl BuildAppmeshVirtualNodeSpecElBackendEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElBackendEl {
        AppmeshVirtualNodeSpecElBackendEl {
            virtual_service: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppmeshVirtualNodeSpecElBackendElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshVirtualNodeSpecElBackendElRef {
    fn new(shared: StackShared, base: String) -> AppmeshVirtualNodeSpecElBackendElRef {
        AppmeshVirtualNodeSpecElBackendElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElBackendElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `virtual_service` after provisioning.\n"]
    pub fn virtual_service(&self) -> ListRef<AppmeshVirtualNodeSpecElBackendElVirtualServiceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.virtual_service", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileEl {
    certificate_chain: PrimField<String>,
    private_key: PrimField<String>,
}

impl AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileEl { }

impl ToListMappable for AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileEl {
    type O = BlockAssignable<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileEl {
    #[doc = ""]
    pub certificate_chain: PrimField<String>,
    #[doc = ""]
    pub private_key: PrimField<String>,
}

impl BuildAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileEl {
        AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileEl {
            certificate_chain: self.certificate_chain,
            private_key: self.private_key,
        }
    }
}

pub struct AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileElRef {
        AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificate_chain` after provisioning.\n"]
    pub fn certificate_chain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.certificate_chain", self.base))
    }

    #[doc = "Get a reference to the value of field `private_key` after provisioning.\n"]
    pub fn private_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.private_key", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsEl {
    secret_name: PrimField<String>,
}

impl AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsEl { }

impl ToListMappable for AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsEl {
    type O = BlockAssignable<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsEl {
    #[doc = ""]
    pub secret_name: PrimField<String>,
}

impl BuildAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsEl {
        AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsEl {
            secret_name: self.secret_name,
        }
    }
}

pub struct AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsElRef {
        AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `secret_name` after provisioning.\n"]
    pub fn secret_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElDynamic {
    file: Option<DynamicBlock<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileEl>>,
    sds: Option<DynamicBlock<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsEl>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    file: Option<Vec<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sds: Option<Vec<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsEl>>,
    dynamic: AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElDynamic,
}

impl AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateEl {
    #[doc = "Set the field `file`.\n"]
    pub fn set_file(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.file = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.file = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sds`.\n"]
    pub fn set_sds(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.sds = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.sds = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateEl {
    type O = BlockAssignable<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateEl {}

impl BuildAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateEl {
        AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateEl {
            file: core::default::Default::default(),
            sds: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElRef {
        AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `file` after provisioning.\n"]
    pub fn file(&self) -> ListRef<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileElRef> {
        ListRef::new(self.shared().clone(), format!("{}.file", self.base))
    }

    #[doc = "Get a reference to the value of field `sds` after provisioning.\n"]
    pub fn sds(&self) -> ListRef<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sds", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl {
    exact: SetField<PrimField<String>>,
}

impl AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl { }

impl ToListMappable for AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl {
    type O =
        BlockAssignable<
            AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl {
    #[doc = ""]
    pub exact: SetField<PrimField<String>>,
}

impl BuildAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl {
    pub fn build(
        self,
    ) -> AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl {
        AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl {
            exact: self.exact,
        }
    }
}

pub struct AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchElRef {
        AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `exact` after provisioning.\n"]
    pub fn exact(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.exact", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElDynamic {
    match_: Option<
        DynamicBlock<
            AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl {
    #[serde(rename = "match", skip_serializing_if = "Option::is_none")]
    match_: Option<
        Vec<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl>,
    >,
    dynamic: AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElDynamic,
}

impl AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl {
    #[doc = "Set the field `match_`.\n"]
    pub fn set_match(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.match_ = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.match_ = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl {
    type O =
        BlockAssignable<
            AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl {}

impl BuildAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl {
    pub fn build(
        self,
    ) -> AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl {
        AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl {
            match_: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElRef {
        AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(
        &self,
    ) -> ListRef<
        AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmEl {
    certificate_authority_arns: SetField<PrimField<String>>,
}

impl AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmEl { }

impl ToListMappable for AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmEl {
    type O = BlockAssignable<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmEl {
    #[doc = ""]
    pub certificate_authority_arns: SetField<PrimField<String>>,
}

impl BuildAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmEl {
        AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmEl {
            certificate_authority_arns: self.certificate_authority_arns,
        }
    }
}

pub struct AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmElRef {
        AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificate_authority_arns` after provisioning.\n"]
    pub fn certificate_authority_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.certificate_authority_arns", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileEl {
    certificate_chain: PrimField<String>,
}

impl AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileEl { }

impl ToListMappable for AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileEl {
    type O = BlockAssignable<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileEl {
    #[doc = ""]
    pub certificate_chain: PrimField<String>,
}

impl BuildAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileEl {
        AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileEl {
            certificate_chain: self.certificate_chain,
        }
    }
}

pub struct AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileElRef {
        AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificate_chain` after provisioning.\n"]
    pub fn certificate_chain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.certificate_chain", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsEl {
    secret_name: PrimField<String>,
}

impl AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsEl { }

impl ToListMappable for AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsEl {
    type O = BlockAssignable<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsEl {
    #[doc = ""]
    pub secret_name: PrimField<String>,
}

impl BuildAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsEl {
        AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsEl {
            secret_name: self.secret_name,
        }
    }
}

pub struct AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsElRef {
        AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `secret_name` after provisioning.\n"]
    pub fn secret_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElDynamic {
    acm: Option<DynamicBlock<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmEl>>,
    file: Option<
        DynamicBlock<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileEl>,
    >,
    sds: Option<DynamicBlock<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsEl>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    acm: Option<Vec<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file: Option<Vec<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sds: Option<Vec<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsEl>>,
    dynamic: AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElDynamic,
}

impl AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustEl {
    #[doc = "Set the field `acm`.\n"]
    pub fn set_acm(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.acm = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.acm = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `file`.\n"]
    pub fn set_file(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.file = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.file = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sds`.\n"]
    pub fn set_sds(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.sds = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.sds = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustEl {
    type O = BlockAssignable<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustEl {}

impl BuildAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustEl {
        AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustEl {
            acm: core::default::Default::default(),
            file: core::default::Default::default(),
            sds: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElRef {
        AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `acm` after provisioning.\n"]
    pub fn acm(
        &self,
    ) -> ListRef<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmElRef> {
        ListRef::new(self.shared().clone(), format!("{}.acm", self.base))
    }

    #[doc = "Get a reference to the value of field `file` after provisioning.\n"]
    pub fn file(
        &self,
    ) -> ListRef<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileElRef> {
        ListRef::new(self.shared().clone(), format!("{}.file", self.base))
    }

    #[doc = "Get a reference to the value of field `sds` after provisioning.\n"]
    pub fn sds(
        &self,
    ) -> ListRef<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sds", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElDynamic {
    subject_alternative_names: Option<
        DynamicBlock<
            AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl,
        >,
    >,
    trust: Option<DynamicBlock<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustEl>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    subject_alternative_names: Option<
        Vec<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    trust: Option<Vec<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustEl>>,
    dynamic: AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElDynamic,
}

impl AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationEl {
    #[doc = "Set the field `subject_alternative_names`.\n"]
    pub fn set_subject_alternative_names(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.subject_alternative_names = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.subject_alternative_names = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `trust`.\n"]
    pub fn set_trust(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.trust = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.trust = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationEl {
    type O = BlockAssignable<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationEl {}

impl BuildAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationEl {
        AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationEl {
            subject_alternative_names: core::default::Default::default(),
            trust: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElRef {
        AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `subject_alternative_names` after provisioning.\n"]
    pub fn subject_alternative_names(
        &self,
    ) -> ListRef<
        AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.subject_alternative_names", self.base))
    }

    #[doc = "Get a reference to the value of field `trust` after provisioning.\n"]
    pub fn trust(&self) -> ListRef<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElRef> {
        ListRef::new(self.shared().clone(), format!("{}.trust", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElDynamic {
    certificate: Option<DynamicBlock<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateEl>>,
    validation: Option<DynamicBlock<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationEl>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enforce: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ports: Option<SetField<PrimField<f64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate: Option<Vec<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    validation: Option<Vec<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationEl>>,
    dynamic: AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElDynamic,
}

impl AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsEl {
    #[doc = "Set the field `enforce`.\n"]
    pub fn set_enforce(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enforce = Some(v.into());
        self
    }

    #[doc = "Set the field `ports`.\n"]
    pub fn set_ports(mut self, v: impl Into<SetField<PrimField<f64>>>) -> Self {
        self.ports = Some(v.into());
        self
    }

    #[doc = "Set the field `certificate`.\n"]
    pub fn set_certificate(
        mut self,
        v: impl Into<BlockAssignable<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.certificate = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.certificate = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `validation`.\n"]
    pub fn set_validation(
        mut self,
        v: impl Into<BlockAssignable<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.validation = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.validation = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsEl {
    type O = BlockAssignable<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsEl {}

impl BuildAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsEl {
        AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsEl {
            enforce: core::default::Default::default(),
            ports: core::default::Default::default(),
            certificate: core::default::Default::default(),
            validation: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElRef {
    fn new(shared: StackShared, base: String) -> AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElRef {
        AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enforce` after provisioning.\n"]
    pub fn enforce(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enforce", self.base))
    }

    #[doc = "Get a reference to the value of field `ports` after provisioning.\n"]
    pub fn ports(&self) -> SetRef<PrimExpr<f64>> {
        SetRef::new(self.shared().clone(), format!("{}.ports", self.base))
    }

    #[doc = "Get a reference to the value of field `certificate` after provisioning.\n"]
    pub fn certificate(&self) -> ListRef<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElRef> {
        ListRef::new(self.shared().clone(), format!("{}.certificate", self.base))
    }

    #[doc = "Get a reference to the value of field `validation` after provisioning.\n"]
    pub fn validation(&self) -> ListRef<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.validation", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElDynamic {
    tls: Option<DynamicBlock<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsEl>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    tls: Option<Vec<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsEl>>,
    dynamic: AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElDynamic,
}

impl AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyEl {
    #[doc = "Set the field `tls`.\n"]
    pub fn set_tls(
        mut self,
        v: impl Into<BlockAssignable<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.tls = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.tls = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyEl {
    type O = BlockAssignable<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyEl {}

impl BuildAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyEl {
        AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyEl {
            tls: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElRef {
    fn new(shared: StackShared, base: String) -> AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElRef {
        AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `tls` after provisioning.\n"]
    pub fn tls(&self) -> ListRef<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tls", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualNodeSpecElBackendDefaultsElDynamic {
    client_policy: Option<DynamicBlock<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyEl>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElBackendDefaultsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    client_policy: Option<Vec<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyEl>>,
    dynamic: AppmeshVirtualNodeSpecElBackendDefaultsElDynamic,
}

impl AppmeshVirtualNodeSpecElBackendDefaultsEl {
    #[doc = "Set the field `client_policy`.\n"]
    pub fn set_client_policy(
        mut self,
        v: impl Into<BlockAssignable<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.client_policy = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.client_policy = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppmeshVirtualNodeSpecElBackendDefaultsEl {
    type O = BlockAssignable<AppmeshVirtualNodeSpecElBackendDefaultsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElBackendDefaultsEl {}

impl BuildAppmeshVirtualNodeSpecElBackendDefaultsEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElBackendDefaultsEl {
        AppmeshVirtualNodeSpecElBackendDefaultsEl {
            client_policy: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppmeshVirtualNodeSpecElBackendDefaultsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshVirtualNodeSpecElBackendDefaultsElRef {
    fn new(shared: StackShared, base: String) -> AppmeshVirtualNodeSpecElBackendDefaultsElRef {
        AppmeshVirtualNodeSpecElBackendDefaultsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElBackendDefaultsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `client_policy` after provisioning.\n"]
    pub fn client_policy(&self) -> ListRef<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.client_policy", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElListenerElConnectionPoolElGrpcEl {
    max_requests: PrimField<f64>,
}

impl AppmeshVirtualNodeSpecElListenerElConnectionPoolElGrpcEl { }

impl ToListMappable for AppmeshVirtualNodeSpecElListenerElConnectionPoolElGrpcEl {
    type O = BlockAssignable<AppmeshVirtualNodeSpecElListenerElConnectionPoolElGrpcEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElListenerElConnectionPoolElGrpcEl {
    #[doc = ""]
    pub max_requests: PrimField<f64>,
}

impl BuildAppmeshVirtualNodeSpecElListenerElConnectionPoolElGrpcEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElListenerElConnectionPoolElGrpcEl {
        AppmeshVirtualNodeSpecElListenerElConnectionPoolElGrpcEl { max_requests: self.max_requests }
    }
}

pub struct AppmeshVirtualNodeSpecElListenerElConnectionPoolElGrpcElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshVirtualNodeSpecElListenerElConnectionPoolElGrpcElRef {
    fn new(shared: StackShared, base: String) -> AppmeshVirtualNodeSpecElListenerElConnectionPoolElGrpcElRef {
        AppmeshVirtualNodeSpecElListenerElConnectionPoolElGrpcElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElListenerElConnectionPoolElGrpcElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_requests` after provisioning.\n"]
    pub fn max_requests(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_requests", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElListenerElConnectionPoolElHttpEl {
    max_connections: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_pending_requests: Option<PrimField<f64>>,
}

impl AppmeshVirtualNodeSpecElListenerElConnectionPoolElHttpEl {
    #[doc = "Set the field `max_pending_requests`.\n"]
    pub fn set_max_pending_requests(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_pending_requests = Some(v.into());
        self
    }
}

impl ToListMappable for AppmeshVirtualNodeSpecElListenerElConnectionPoolElHttpEl {
    type O = BlockAssignable<AppmeshVirtualNodeSpecElListenerElConnectionPoolElHttpEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElListenerElConnectionPoolElHttpEl {
    #[doc = ""]
    pub max_connections: PrimField<f64>,
}

impl BuildAppmeshVirtualNodeSpecElListenerElConnectionPoolElHttpEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElListenerElConnectionPoolElHttpEl {
        AppmeshVirtualNodeSpecElListenerElConnectionPoolElHttpEl {
            max_connections: self.max_connections,
            max_pending_requests: core::default::Default::default(),
        }
    }
}

pub struct AppmeshVirtualNodeSpecElListenerElConnectionPoolElHttpElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshVirtualNodeSpecElListenerElConnectionPoolElHttpElRef {
    fn new(shared: StackShared, base: String) -> AppmeshVirtualNodeSpecElListenerElConnectionPoolElHttpElRef {
        AppmeshVirtualNodeSpecElListenerElConnectionPoolElHttpElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElListenerElConnectionPoolElHttpElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_connections` after provisioning.\n"]
    pub fn max_connections(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_connections", self.base))
    }

    #[doc = "Get a reference to the value of field `max_pending_requests` after provisioning.\n"]
    pub fn max_pending_requests(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_pending_requests", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElListenerElConnectionPoolElHttp2El {
    max_requests: PrimField<f64>,
}

impl AppmeshVirtualNodeSpecElListenerElConnectionPoolElHttp2El { }

impl ToListMappable for AppmeshVirtualNodeSpecElListenerElConnectionPoolElHttp2El {
    type O = BlockAssignable<AppmeshVirtualNodeSpecElListenerElConnectionPoolElHttp2El>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElListenerElConnectionPoolElHttp2El {
    #[doc = ""]
    pub max_requests: PrimField<f64>,
}

impl BuildAppmeshVirtualNodeSpecElListenerElConnectionPoolElHttp2El {
    pub fn build(self) -> AppmeshVirtualNodeSpecElListenerElConnectionPoolElHttp2El {
        AppmeshVirtualNodeSpecElListenerElConnectionPoolElHttp2El { max_requests: self.max_requests }
    }
}

pub struct AppmeshVirtualNodeSpecElListenerElConnectionPoolElHttp2ElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshVirtualNodeSpecElListenerElConnectionPoolElHttp2ElRef {
    fn new(shared: StackShared, base: String) -> AppmeshVirtualNodeSpecElListenerElConnectionPoolElHttp2ElRef {
        AppmeshVirtualNodeSpecElListenerElConnectionPoolElHttp2ElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElListenerElConnectionPoolElHttp2ElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_requests` after provisioning.\n"]
    pub fn max_requests(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_requests", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElListenerElConnectionPoolElTcpEl {
    max_connections: PrimField<f64>,
}

impl AppmeshVirtualNodeSpecElListenerElConnectionPoolElTcpEl { }

impl ToListMappable for AppmeshVirtualNodeSpecElListenerElConnectionPoolElTcpEl {
    type O = BlockAssignable<AppmeshVirtualNodeSpecElListenerElConnectionPoolElTcpEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElListenerElConnectionPoolElTcpEl {
    #[doc = ""]
    pub max_connections: PrimField<f64>,
}

impl BuildAppmeshVirtualNodeSpecElListenerElConnectionPoolElTcpEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElListenerElConnectionPoolElTcpEl {
        AppmeshVirtualNodeSpecElListenerElConnectionPoolElTcpEl { max_connections: self.max_connections }
    }
}

pub struct AppmeshVirtualNodeSpecElListenerElConnectionPoolElTcpElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshVirtualNodeSpecElListenerElConnectionPoolElTcpElRef {
    fn new(shared: StackShared, base: String) -> AppmeshVirtualNodeSpecElListenerElConnectionPoolElTcpElRef {
        AppmeshVirtualNodeSpecElListenerElConnectionPoolElTcpElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElListenerElConnectionPoolElTcpElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_connections` after provisioning.\n"]
    pub fn max_connections(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_connections", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualNodeSpecElListenerElConnectionPoolElDynamic {
    grpc: Option<DynamicBlock<AppmeshVirtualNodeSpecElListenerElConnectionPoolElGrpcEl>>,
    http: Option<DynamicBlock<AppmeshVirtualNodeSpecElListenerElConnectionPoolElHttpEl>>,
    http2: Option<DynamicBlock<AppmeshVirtualNodeSpecElListenerElConnectionPoolElHttp2El>>,
    tcp: Option<DynamicBlock<AppmeshVirtualNodeSpecElListenerElConnectionPoolElTcpEl>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElListenerElConnectionPoolEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    grpc: Option<Vec<AppmeshVirtualNodeSpecElListenerElConnectionPoolElGrpcEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http: Option<Vec<AppmeshVirtualNodeSpecElListenerElConnectionPoolElHttpEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http2: Option<Vec<AppmeshVirtualNodeSpecElListenerElConnectionPoolElHttp2El>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tcp: Option<Vec<AppmeshVirtualNodeSpecElListenerElConnectionPoolElTcpEl>>,
    dynamic: AppmeshVirtualNodeSpecElListenerElConnectionPoolElDynamic,
}

impl AppmeshVirtualNodeSpecElListenerElConnectionPoolEl {
    #[doc = "Set the field `grpc`.\n"]
    pub fn set_grpc(
        mut self,
        v: impl Into<BlockAssignable<AppmeshVirtualNodeSpecElListenerElConnectionPoolElGrpcEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.grpc = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.grpc = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `http`.\n"]
    pub fn set_http(
        mut self,
        v: impl Into<BlockAssignable<AppmeshVirtualNodeSpecElListenerElConnectionPoolElHttpEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.http = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.http = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `http2`.\n"]
    pub fn set_http2(
        mut self,
        v: impl Into<BlockAssignable<AppmeshVirtualNodeSpecElListenerElConnectionPoolElHttp2El>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.http2 = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.http2 = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tcp`.\n"]
    pub fn set_tcp(
        mut self,
        v: impl Into<BlockAssignable<AppmeshVirtualNodeSpecElListenerElConnectionPoolElTcpEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.tcp = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.tcp = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppmeshVirtualNodeSpecElListenerElConnectionPoolEl {
    type O = BlockAssignable<AppmeshVirtualNodeSpecElListenerElConnectionPoolEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElListenerElConnectionPoolEl {}

impl BuildAppmeshVirtualNodeSpecElListenerElConnectionPoolEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElListenerElConnectionPoolEl {
        AppmeshVirtualNodeSpecElListenerElConnectionPoolEl {
            grpc: core::default::Default::default(),
            http: core::default::Default::default(),
            http2: core::default::Default::default(),
            tcp: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppmeshVirtualNodeSpecElListenerElConnectionPoolElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshVirtualNodeSpecElListenerElConnectionPoolElRef {
    fn new(shared: StackShared, base: String) -> AppmeshVirtualNodeSpecElListenerElConnectionPoolElRef {
        AppmeshVirtualNodeSpecElListenerElConnectionPoolElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElListenerElConnectionPoolElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `grpc` after provisioning.\n"]
    pub fn grpc(&self) -> ListRef<AppmeshVirtualNodeSpecElListenerElConnectionPoolElGrpcElRef> {
        ListRef::new(self.shared().clone(), format!("{}.grpc", self.base))
    }

    #[doc = "Get a reference to the value of field `http` after provisioning.\n"]
    pub fn http(&self) -> ListRef<AppmeshVirtualNodeSpecElListenerElConnectionPoolElHttpElRef> {
        ListRef::new(self.shared().clone(), format!("{}.http", self.base))
    }

    #[doc = "Get a reference to the value of field `http2` after provisioning.\n"]
    pub fn http2(&self) -> ListRef<AppmeshVirtualNodeSpecElListenerElConnectionPoolElHttp2ElRef> {
        ListRef::new(self.shared().clone(), format!("{}.http2", self.base))
    }

    #[doc = "Get a reference to the value of field `tcp` after provisioning.\n"]
    pub fn tcp(&self) -> ListRef<AppmeshVirtualNodeSpecElListenerElConnectionPoolElTcpElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tcp", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElListenerElHealthCheckEl {
    healthy_threshold: PrimField<f64>,
    interval_millis: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    protocol: PrimField<String>,
    timeout_millis: PrimField<f64>,
    unhealthy_threshold: PrimField<f64>,
}

impl AppmeshVirtualNodeSpecElListenerElHealthCheckEl {
    #[doc = "Set the field `path`.\n"]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
}

impl ToListMappable for AppmeshVirtualNodeSpecElListenerElHealthCheckEl {
    type O = BlockAssignable<AppmeshVirtualNodeSpecElListenerElHealthCheckEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElListenerElHealthCheckEl {
    #[doc = ""]
    pub healthy_threshold: PrimField<f64>,
    #[doc = ""]
    pub interval_millis: PrimField<f64>,
    #[doc = ""]
    pub protocol: PrimField<String>,
    #[doc = ""]
    pub timeout_millis: PrimField<f64>,
    #[doc = ""]
    pub unhealthy_threshold: PrimField<f64>,
}

impl BuildAppmeshVirtualNodeSpecElListenerElHealthCheckEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElListenerElHealthCheckEl {
        AppmeshVirtualNodeSpecElListenerElHealthCheckEl {
            healthy_threshold: self.healthy_threshold,
            interval_millis: self.interval_millis,
            path: core::default::Default::default(),
            port: core::default::Default::default(),
            protocol: self.protocol,
            timeout_millis: self.timeout_millis,
            unhealthy_threshold: self.unhealthy_threshold,
        }
    }
}

pub struct AppmeshVirtualNodeSpecElListenerElHealthCheckElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshVirtualNodeSpecElListenerElHealthCheckElRef {
    fn new(shared: StackShared, base: String) -> AppmeshVirtualNodeSpecElListenerElHealthCheckElRef {
        AppmeshVirtualNodeSpecElListenerElHealthCheckElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElListenerElHealthCheckElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `healthy_threshold` after provisioning.\n"]
    pub fn healthy_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.healthy_threshold", self.base))
    }

    #[doc = "Get a reference to the value of field `interval_millis` after provisioning.\n"]
    pub fn interval_millis(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.interval_millis", self.base))
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout_millis` after provisioning.\n"]
    pub fn timeout_millis(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.timeout_millis", self.base))
    }

    #[doc = "Get a reference to the value of field `unhealthy_threshold` after provisioning.\n"]
    pub fn unhealthy_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.unhealthy_threshold", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElListenerElOutlierDetectionElBaseEjectionDurationEl {
    unit: PrimField<String>,
    value: PrimField<f64>,
}

impl AppmeshVirtualNodeSpecElListenerElOutlierDetectionElBaseEjectionDurationEl { }

impl ToListMappable for AppmeshVirtualNodeSpecElListenerElOutlierDetectionElBaseEjectionDurationEl {
    type O = BlockAssignable<AppmeshVirtualNodeSpecElListenerElOutlierDetectionElBaseEjectionDurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElListenerElOutlierDetectionElBaseEjectionDurationEl {
    #[doc = ""]
    pub unit: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<f64>,
}

impl BuildAppmeshVirtualNodeSpecElListenerElOutlierDetectionElBaseEjectionDurationEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElListenerElOutlierDetectionElBaseEjectionDurationEl {
        AppmeshVirtualNodeSpecElListenerElOutlierDetectionElBaseEjectionDurationEl {
            unit: self.unit,
            value: self.value,
        }
    }
}

pub struct AppmeshVirtualNodeSpecElListenerElOutlierDetectionElBaseEjectionDurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshVirtualNodeSpecElListenerElOutlierDetectionElBaseEjectionDurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppmeshVirtualNodeSpecElListenerElOutlierDetectionElBaseEjectionDurationElRef {
        AppmeshVirtualNodeSpecElListenerElOutlierDetectionElBaseEjectionDurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElListenerElOutlierDetectionElBaseEjectionDurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElListenerElOutlierDetectionElIntervalEl {
    unit: PrimField<String>,
    value: PrimField<f64>,
}

impl AppmeshVirtualNodeSpecElListenerElOutlierDetectionElIntervalEl { }

impl ToListMappable for AppmeshVirtualNodeSpecElListenerElOutlierDetectionElIntervalEl {
    type O = BlockAssignable<AppmeshVirtualNodeSpecElListenerElOutlierDetectionElIntervalEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElListenerElOutlierDetectionElIntervalEl {
    #[doc = ""]
    pub unit: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<f64>,
}

impl BuildAppmeshVirtualNodeSpecElListenerElOutlierDetectionElIntervalEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElListenerElOutlierDetectionElIntervalEl {
        AppmeshVirtualNodeSpecElListenerElOutlierDetectionElIntervalEl {
            unit: self.unit,
            value: self.value,
        }
    }
}

pub struct AppmeshVirtualNodeSpecElListenerElOutlierDetectionElIntervalElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshVirtualNodeSpecElListenerElOutlierDetectionElIntervalElRef {
    fn new(shared: StackShared, base: String) -> AppmeshVirtualNodeSpecElListenerElOutlierDetectionElIntervalElRef {
        AppmeshVirtualNodeSpecElListenerElOutlierDetectionElIntervalElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElListenerElOutlierDetectionElIntervalElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualNodeSpecElListenerElOutlierDetectionElDynamic {
    base_ejection_duration: Option<
        DynamicBlock<AppmeshVirtualNodeSpecElListenerElOutlierDetectionElBaseEjectionDurationEl>,
    >,
    interval: Option<DynamicBlock<AppmeshVirtualNodeSpecElListenerElOutlierDetectionElIntervalEl>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElListenerElOutlierDetectionEl {
    max_ejection_percent: PrimField<f64>,
    max_server_errors: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    base_ejection_duration: Option<Vec<AppmeshVirtualNodeSpecElListenerElOutlierDetectionElBaseEjectionDurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    interval: Option<Vec<AppmeshVirtualNodeSpecElListenerElOutlierDetectionElIntervalEl>>,
    dynamic: AppmeshVirtualNodeSpecElListenerElOutlierDetectionElDynamic,
}

impl AppmeshVirtualNodeSpecElListenerElOutlierDetectionEl {
    #[doc = "Set the field `base_ejection_duration`.\n"]
    pub fn set_base_ejection_duration(
        mut self,
        v: impl Into<BlockAssignable<AppmeshVirtualNodeSpecElListenerElOutlierDetectionElBaseEjectionDurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.base_ejection_duration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.base_ejection_duration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `interval`.\n"]
    pub fn set_interval(
        mut self,
        v: impl Into<BlockAssignable<AppmeshVirtualNodeSpecElListenerElOutlierDetectionElIntervalEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.interval = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.interval = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppmeshVirtualNodeSpecElListenerElOutlierDetectionEl {
    type O = BlockAssignable<AppmeshVirtualNodeSpecElListenerElOutlierDetectionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElListenerElOutlierDetectionEl {
    #[doc = ""]
    pub max_ejection_percent: PrimField<f64>,
    #[doc = ""]
    pub max_server_errors: PrimField<f64>,
}

impl BuildAppmeshVirtualNodeSpecElListenerElOutlierDetectionEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElListenerElOutlierDetectionEl {
        AppmeshVirtualNodeSpecElListenerElOutlierDetectionEl {
            max_ejection_percent: self.max_ejection_percent,
            max_server_errors: self.max_server_errors,
            base_ejection_duration: core::default::Default::default(),
            interval: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppmeshVirtualNodeSpecElListenerElOutlierDetectionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshVirtualNodeSpecElListenerElOutlierDetectionElRef {
    fn new(shared: StackShared, base: String) -> AppmeshVirtualNodeSpecElListenerElOutlierDetectionElRef {
        AppmeshVirtualNodeSpecElListenerElOutlierDetectionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElListenerElOutlierDetectionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_ejection_percent` after provisioning.\n"]
    pub fn max_ejection_percent(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_ejection_percent", self.base))
    }

    #[doc = "Get a reference to the value of field `max_server_errors` after provisioning.\n"]
    pub fn max_server_errors(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_server_errors", self.base))
    }

    #[doc = "Get a reference to the value of field `base_ejection_duration` after provisioning.\n"]
    pub fn base_ejection_duration(
        &self,
    ) -> ListRef<AppmeshVirtualNodeSpecElListenerElOutlierDetectionElBaseEjectionDurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.base_ejection_duration", self.base))
    }

    #[doc = "Get a reference to the value of field `interval` after provisioning.\n"]
    pub fn interval(&self) -> ListRef<AppmeshVirtualNodeSpecElListenerElOutlierDetectionElIntervalElRef> {
        ListRef::new(self.shared().clone(), format!("{}.interval", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElListenerElPortMappingEl {
    port: PrimField<f64>,
    protocol: PrimField<String>,
}

impl AppmeshVirtualNodeSpecElListenerElPortMappingEl { }

impl ToListMappable for AppmeshVirtualNodeSpecElListenerElPortMappingEl {
    type O = BlockAssignable<AppmeshVirtualNodeSpecElListenerElPortMappingEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElListenerElPortMappingEl {
    #[doc = ""]
    pub port: PrimField<f64>,
    #[doc = ""]
    pub protocol: PrimField<String>,
}

impl BuildAppmeshVirtualNodeSpecElListenerElPortMappingEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElListenerElPortMappingEl {
        AppmeshVirtualNodeSpecElListenerElPortMappingEl {
            port: self.port,
            protocol: self.protocol,
        }
    }
}

pub struct AppmeshVirtualNodeSpecElListenerElPortMappingElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshVirtualNodeSpecElListenerElPortMappingElRef {
    fn new(shared: StackShared, base: String) -> AppmeshVirtualNodeSpecElListenerElPortMappingElRef {
        AppmeshVirtualNodeSpecElListenerElPortMappingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElListenerElPortMappingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElIdleEl {
    unit: PrimField<String>,
    value: PrimField<f64>,
}

impl AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElIdleEl { }

impl ToListMappable for AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElIdleEl {
    type O = BlockAssignable<AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElIdleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElIdleEl {
    #[doc = ""]
    pub unit: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<f64>,
}

impl BuildAppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElIdleEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElIdleEl {
        AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElIdleEl {
            unit: self.unit,
            value: self.value,
        }
    }
}

pub struct AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElIdleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElIdleElRef {
    fn new(shared: StackShared, base: String) -> AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElIdleElRef {
        AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElIdleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElIdleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElPerRequestEl {
    unit: PrimField<String>,
    value: PrimField<f64>,
}

impl AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElPerRequestEl { }

impl ToListMappable for AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElPerRequestEl {
    type O = BlockAssignable<AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElPerRequestEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElPerRequestEl {
    #[doc = ""]
    pub unit: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<f64>,
}

impl BuildAppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElPerRequestEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElPerRequestEl {
        AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElPerRequestEl {
            unit: self.unit,
            value: self.value,
        }
    }
}

pub struct AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElPerRequestElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElPerRequestElRef {
    fn new(shared: StackShared, base: String) -> AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElPerRequestElRef {
        AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElPerRequestElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElPerRequestElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElDynamic {
    idle: Option<DynamicBlock<AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElIdleEl>>,
    per_request: Option<DynamicBlock<AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElPerRequestEl>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    idle: Option<Vec<AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElIdleEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_request: Option<Vec<AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElPerRequestEl>>,
    dynamic: AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElDynamic,
}

impl AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcEl {
    #[doc = "Set the field `idle`.\n"]
    pub fn set_idle(
        mut self,
        v: impl Into<BlockAssignable<AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElIdleEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.idle = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.idle = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `per_request`.\n"]
    pub fn set_per_request(
        mut self,
        v: impl Into<BlockAssignable<AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElPerRequestEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.per_request = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.per_request = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcEl {
    type O = BlockAssignable<AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElListenerElTimeoutElGrpcEl {}

impl BuildAppmeshVirtualNodeSpecElListenerElTimeoutElGrpcEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcEl {
        AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcEl {
            idle: core::default::Default::default(),
            per_request: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElRef {
    fn new(shared: StackShared, base: String) -> AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElRef {
        AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `idle` after provisioning.\n"]
    pub fn idle(&self) -> ListRef<AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElIdleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.idle", self.base))
    }

    #[doc = "Get a reference to the value of field `per_request` after provisioning.\n"]
    pub fn per_request(&self) -> ListRef<AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElPerRequestElRef> {
        ListRef::new(self.shared().clone(), format!("{}.per_request", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElListenerElTimeoutElHttpElIdleEl {
    unit: PrimField<String>,
    value: PrimField<f64>,
}

impl AppmeshVirtualNodeSpecElListenerElTimeoutElHttpElIdleEl { }

impl ToListMappable for AppmeshVirtualNodeSpecElListenerElTimeoutElHttpElIdleEl {
    type O = BlockAssignable<AppmeshVirtualNodeSpecElListenerElTimeoutElHttpElIdleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElListenerElTimeoutElHttpElIdleEl {
    #[doc = ""]
    pub unit: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<f64>,
}

impl BuildAppmeshVirtualNodeSpecElListenerElTimeoutElHttpElIdleEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElListenerElTimeoutElHttpElIdleEl {
        AppmeshVirtualNodeSpecElListenerElTimeoutElHttpElIdleEl {
            unit: self.unit,
            value: self.value,
        }
    }
}

pub struct AppmeshVirtualNodeSpecElListenerElTimeoutElHttpElIdleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshVirtualNodeSpecElListenerElTimeoutElHttpElIdleElRef {
    fn new(shared: StackShared, base: String) -> AppmeshVirtualNodeSpecElListenerElTimeoutElHttpElIdleElRef {
        AppmeshVirtualNodeSpecElListenerElTimeoutElHttpElIdleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElListenerElTimeoutElHttpElIdleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElListenerElTimeoutElHttpElPerRequestEl {
    unit: PrimField<String>,
    value: PrimField<f64>,
}

impl AppmeshVirtualNodeSpecElListenerElTimeoutElHttpElPerRequestEl { }

impl ToListMappable for AppmeshVirtualNodeSpecElListenerElTimeoutElHttpElPerRequestEl {
    type O = BlockAssignable<AppmeshVirtualNodeSpecElListenerElTimeoutElHttpElPerRequestEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElListenerElTimeoutElHttpElPerRequestEl {
    #[doc = ""]
    pub unit: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<f64>,
}

impl BuildAppmeshVirtualNodeSpecElListenerElTimeoutElHttpElPerRequestEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElListenerElTimeoutElHttpElPerRequestEl {
        AppmeshVirtualNodeSpecElListenerElTimeoutElHttpElPerRequestEl {
            unit: self.unit,
            value: self.value,
        }
    }
}

pub struct AppmeshVirtualNodeSpecElListenerElTimeoutElHttpElPerRequestElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshVirtualNodeSpecElListenerElTimeoutElHttpElPerRequestElRef {
    fn new(shared: StackShared, base: String) -> AppmeshVirtualNodeSpecElListenerElTimeoutElHttpElPerRequestElRef {
        AppmeshVirtualNodeSpecElListenerElTimeoutElHttpElPerRequestElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElListenerElTimeoutElHttpElPerRequestElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualNodeSpecElListenerElTimeoutElHttpElDynamic {
    idle: Option<DynamicBlock<AppmeshVirtualNodeSpecElListenerElTimeoutElHttpElIdleEl>>,
    per_request: Option<DynamicBlock<AppmeshVirtualNodeSpecElListenerElTimeoutElHttpElPerRequestEl>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElListenerElTimeoutElHttpEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    idle: Option<Vec<AppmeshVirtualNodeSpecElListenerElTimeoutElHttpElIdleEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_request: Option<Vec<AppmeshVirtualNodeSpecElListenerElTimeoutElHttpElPerRequestEl>>,
    dynamic: AppmeshVirtualNodeSpecElListenerElTimeoutElHttpElDynamic,
}

impl AppmeshVirtualNodeSpecElListenerElTimeoutElHttpEl {
    #[doc = "Set the field `idle`.\n"]
    pub fn set_idle(
        mut self,
        v: impl Into<BlockAssignable<AppmeshVirtualNodeSpecElListenerElTimeoutElHttpElIdleEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.idle = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.idle = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `per_request`.\n"]
    pub fn set_per_request(
        mut self,
        v: impl Into<BlockAssignable<AppmeshVirtualNodeSpecElListenerElTimeoutElHttpElPerRequestEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.per_request = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.per_request = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppmeshVirtualNodeSpecElListenerElTimeoutElHttpEl {
    type O = BlockAssignable<AppmeshVirtualNodeSpecElListenerElTimeoutElHttpEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElListenerElTimeoutElHttpEl {}

impl BuildAppmeshVirtualNodeSpecElListenerElTimeoutElHttpEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElListenerElTimeoutElHttpEl {
        AppmeshVirtualNodeSpecElListenerElTimeoutElHttpEl {
            idle: core::default::Default::default(),
            per_request: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppmeshVirtualNodeSpecElListenerElTimeoutElHttpElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshVirtualNodeSpecElListenerElTimeoutElHttpElRef {
    fn new(shared: StackShared, base: String) -> AppmeshVirtualNodeSpecElListenerElTimeoutElHttpElRef {
        AppmeshVirtualNodeSpecElListenerElTimeoutElHttpElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElListenerElTimeoutElHttpElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `idle` after provisioning.\n"]
    pub fn idle(&self) -> ListRef<AppmeshVirtualNodeSpecElListenerElTimeoutElHttpElIdleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.idle", self.base))
    }

    #[doc = "Get a reference to the value of field `per_request` after provisioning.\n"]
    pub fn per_request(&self) -> ListRef<AppmeshVirtualNodeSpecElListenerElTimeoutElHttpElPerRequestElRef> {
        ListRef::new(self.shared().clone(), format!("{}.per_request", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElIdleEl {
    unit: PrimField<String>,
    value: PrimField<f64>,
}

impl AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElIdleEl { }

impl ToListMappable for AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElIdleEl {
    type O = BlockAssignable<AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElIdleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElIdleEl {
    #[doc = ""]
    pub unit: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<f64>,
}

impl BuildAppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElIdleEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElIdleEl {
        AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElIdleEl {
            unit: self.unit,
            value: self.value,
        }
    }
}

pub struct AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElIdleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElIdleElRef {
    fn new(shared: StackShared, base: String) -> AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElIdleElRef {
        AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElIdleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElIdleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElPerRequestEl {
    unit: PrimField<String>,
    value: PrimField<f64>,
}

impl AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElPerRequestEl { }

impl ToListMappable for AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElPerRequestEl {
    type O = BlockAssignable<AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElPerRequestEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElPerRequestEl {
    #[doc = ""]
    pub unit: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<f64>,
}

impl BuildAppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElPerRequestEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElPerRequestEl {
        AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElPerRequestEl {
            unit: self.unit,
            value: self.value,
        }
    }
}

pub struct AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElPerRequestElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElPerRequestElRef {
    fn new(shared: StackShared, base: String) -> AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElPerRequestElRef {
        AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElPerRequestElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElPerRequestElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElDynamic {
    idle: Option<DynamicBlock<AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElIdleEl>>,
    per_request: Option<DynamicBlock<AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElPerRequestEl>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2El {
    #[serde(skip_serializing_if = "Option::is_none")]
    idle: Option<Vec<AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElIdleEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_request: Option<Vec<AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElPerRequestEl>>,
    dynamic: AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElDynamic,
}

impl AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2El {
    #[doc = "Set the field `idle`.\n"]
    pub fn set_idle(
        mut self,
        v: impl Into<BlockAssignable<AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElIdleEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.idle = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.idle = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `per_request`.\n"]
    pub fn set_per_request(
        mut self,
        v: impl Into<BlockAssignable<AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElPerRequestEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.per_request = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.per_request = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2El {
    type O = BlockAssignable<AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2El>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElListenerElTimeoutElHttp2El {}

impl BuildAppmeshVirtualNodeSpecElListenerElTimeoutElHttp2El {
    pub fn build(self) -> AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2El {
        AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2El {
            idle: core::default::Default::default(),
            per_request: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElRef {
    fn new(shared: StackShared, base: String) -> AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElRef {
        AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `idle` after provisioning.\n"]
    pub fn idle(&self) -> ListRef<AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElIdleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.idle", self.base))
    }

    #[doc = "Get a reference to the value of field `per_request` after provisioning.\n"]
    pub fn per_request(&self) -> ListRef<AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElPerRequestElRef> {
        ListRef::new(self.shared().clone(), format!("{}.per_request", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElListenerElTimeoutElTcpElIdleEl {
    unit: PrimField<String>,
    value: PrimField<f64>,
}

impl AppmeshVirtualNodeSpecElListenerElTimeoutElTcpElIdleEl { }

impl ToListMappable for AppmeshVirtualNodeSpecElListenerElTimeoutElTcpElIdleEl {
    type O = BlockAssignable<AppmeshVirtualNodeSpecElListenerElTimeoutElTcpElIdleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElListenerElTimeoutElTcpElIdleEl {
    #[doc = ""]
    pub unit: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<f64>,
}

impl BuildAppmeshVirtualNodeSpecElListenerElTimeoutElTcpElIdleEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElListenerElTimeoutElTcpElIdleEl {
        AppmeshVirtualNodeSpecElListenerElTimeoutElTcpElIdleEl {
            unit: self.unit,
            value: self.value,
        }
    }
}

pub struct AppmeshVirtualNodeSpecElListenerElTimeoutElTcpElIdleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshVirtualNodeSpecElListenerElTimeoutElTcpElIdleElRef {
    fn new(shared: StackShared, base: String) -> AppmeshVirtualNodeSpecElListenerElTimeoutElTcpElIdleElRef {
        AppmeshVirtualNodeSpecElListenerElTimeoutElTcpElIdleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElListenerElTimeoutElTcpElIdleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualNodeSpecElListenerElTimeoutElTcpElDynamic {
    idle: Option<DynamicBlock<AppmeshVirtualNodeSpecElListenerElTimeoutElTcpElIdleEl>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElListenerElTimeoutElTcpEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    idle: Option<Vec<AppmeshVirtualNodeSpecElListenerElTimeoutElTcpElIdleEl>>,
    dynamic: AppmeshVirtualNodeSpecElListenerElTimeoutElTcpElDynamic,
}

impl AppmeshVirtualNodeSpecElListenerElTimeoutElTcpEl {
    #[doc = "Set the field `idle`.\n"]
    pub fn set_idle(
        mut self,
        v: impl Into<BlockAssignable<AppmeshVirtualNodeSpecElListenerElTimeoutElTcpElIdleEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.idle = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.idle = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppmeshVirtualNodeSpecElListenerElTimeoutElTcpEl {
    type O = BlockAssignable<AppmeshVirtualNodeSpecElListenerElTimeoutElTcpEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElListenerElTimeoutElTcpEl {}

impl BuildAppmeshVirtualNodeSpecElListenerElTimeoutElTcpEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElListenerElTimeoutElTcpEl {
        AppmeshVirtualNodeSpecElListenerElTimeoutElTcpEl {
            idle: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppmeshVirtualNodeSpecElListenerElTimeoutElTcpElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshVirtualNodeSpecElListenerElTimeoutElTcpElRef {
    fn new(shared: StackShared, base: String) -> AppmeshVirtualNodeSpecElListenerElTimeoutElTcpElRef {
        AppmeshVirtualNodeSpecElListenerElTimeoutElTcpElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElListenerElTimeoutElTcpElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `idle` after provisioning.\n"]
    pub fn idle(&self) -> ListRef<AppmeshVirtualNodeSpecElListenerElTimeoutElTcpElIdleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.idle", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualNodeSpecElListenerElTimeoutElDynamic {
    grpc: Option<DynamicBlock<AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcEl>>,
    http: Option<DynamicBlock<AppmeshVirtualNodeSpecElListenerElTimeoutElHttpEl>>,
    http2: Option<DynamicBlock<AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2El>>,
    tcp: Option<DynamicBlock<AppmeshVirtualNodeSpecElListenerElTimeoutElTcpEl>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElListenerElTimeoutEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    grpc: Option<Vec<AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http: Option<Vec<AppmeshVirtualNodeSpecElListenerElTimeoutElHttpEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http2: Option<Vec<AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2El>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tcp: Option<Vec<AppmeshVirtualNodeSpecElListenerElTimeoutElTcpEl>>,
    dynamic: AppmeshVirtualNodeSpecElListenerElTimeoutElDynamic,
}

impl AppmeshVirtualNodeSpecElListenerElTimeoutEl {
    #[doc = "Set the field `grpc`.\n"]
    pub fn set_grpc(mut self, v: impl Into<BlockAssignable<AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.grpc = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.grpc = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `http`.\n"]
    pub fn set_http(mut self, v: impl Into<BlockAssignable<AppmeshVirtualNodeSpecElListenerElTimeoutElHttpEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.http = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.http = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `http2`.\n"]
    pub fn set_http2(
        mut self,
        v: impl Into<BlockAssignable<AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2El>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.http2 = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.http2 = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tcp`.\n"]
    pub fn set_tcp(mut self, v: impl Into<BlockAssignable<AppmeshVirtualNodeSpecElListenerElTimeoutElTcpEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.tcp = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.tcp = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppmeshVirtualNodeSpecElListenerElTimeoutEl {
    type O = BlockAssignable<AppmeshVirtualNodeSpecElListenerElTimeoutEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElListenerElTimeoutEl {}

impl BuildAppmeshVirtualNodeSpecElListenerElTimeoutEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElListenerElTimeoutEl {
        AppmeshVirtualNodeSpecElListenerElTimeoutEl {
            grpc: core::default::Default::default(),
            http: core::default::Default::default(),
            http2: core::default::Default::default(),
            tcp: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppmeshVirtualNodeSpecElListenerElTimeoutElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshVirtualNodeSpecElListenerElTimeoutElRef {
    fn new(shared: StackShared, base: String) -> AppmeshVirtualNodeSpecElListenerElTimeoutElRef {
        AppmeshVirtualNodeSpecElListenerElTimeoutElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElListenerElTimeoutElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `grpc` after provisioning.\n"]
    pub fn grpc(&self) -> ListRef<AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElRef> {
        ListRef::new(self.shared().clone(), format!("{}.grpc", self.base))
    }

    #[doc = "Get a reference to the value of field `http` after provisioning.\n"]
    pub fn http(&self) -> ListRef<AppmeshVirtualNodeSpecElListenerElTimeoutElHttpElRef> {
        ListRef::new(self.shared().clone(), format!("{}.http", self.base))
    }

    #[doc = "Get a reference to the value of field `http2` after provisioning.\n"]
    pub fn http2(&self) -> ListRef<AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElRef> {
        ListRef::new(self.shared().clone(), format!("{}.http2", self.base))
    }

    #[doc = "Get a reference to the value of field `tcp` after provisioning.\n"]
    pub fn tcp(&self) -> ListRef<AppmeshVirtualNodeSpecElListenerElTimeoutElTcpElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tcp", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElListenerElTlsElCertificateElAcmEl {
    certificate_arn: PrimField<String>,
}

impl AppmeshVirtualNodeSpecElListenerElTlsElCertificateElAcmEl { }

impl ToListMappable for AppmeshVirtualNodeSpecElListenerElTlsElCertificateElAcmEl {
    type O = BlockAssignable<AppmeshVirtualNodeSpecElListenerElTlsElCertificateElAcmEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElListenerElTlsElCertificateElAcmEl {
    #[doc = ""]
    pub certificate_arn: PrimField<String>,
}

impl BuildAppmeshVirtualNodeSpecElListenerElTlsElCertificateElAcmEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElListenerElTlsElCertificateElAcmEl {
        AppmeshVirtualNodeSpecElListenerElTlsElCertificateElAcmEl { certificate_arn: self.certificate_arn }
    }
}

pub struct AppmeshVirtualNodeSpecElListenerElTlsElCertificateElAcmElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshVirtualNodeSpecElListenerElTlsElCertificateElAcmElRef {
    fn new(shared: StackShared, base: String) -> AppmeshVirtualNodeSpecElListenerElTlsElCertificateElAcmElRef {
        AppmeshVirtualNodeSpecElListenerElTlsElCertificateElAcmElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElListenerElTlsElCertificateElAcmElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificate_arn` after provisioning.\n"]
    pub fn certificate_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.certificate_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElListenerElTlsElCertificateElFileEl {
    certificate_chain: PrimField<String>,
    private_key: PrimField<String>,
}

impl AppmeshVirtualNodeSpecElListenerElTlsElCertificateElFileEl { }

impl ToListMappable for AppmeshVirtualNodeSpecElListenerElTlsElCertificateElFileEl {
    type O = BlockAssignable<AppmeshVirtualNodeSpecElListenerElTlsElCertificateElFileEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElListenerElTlsElCertificateElFileEl {
    #[doc = ""]
    pub certificate_chain: PrimField<String>,
    #[doc = ""]
    pub private_key: PrimField<String>,
}

impl BuildAppmeshVirtualNodeSpecElListenerElTlsElCertificateElFileEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElListenerElTlsElCertificateElFileEl {
        AppmeshVirtualNodeSpecElListenerElTlsElCertificateElFileEl {
            certificate_chain: self.certificate_chain,
            private_key: self.private_key,
        }
    }
}

pub struct AppmeshVirtualNodeSpecElListenerElTlsElCertificateElFileElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshVirtualNodeSpecElListenerElTlsElCertificateElFileElRef {
    fn new(shared: StackShared, base: String) -> AppmeshVirtualNodeSpecElListenerElTlsElCertificateElFileElRef {
        AppmeshVirtualNodeSpecElListenerElTlsElCertificateElFileElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElListenerElTlsElCertificateElFileElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificate_chain` after provisioning.\n"]
    pub fn certificate_chain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.certificate_chain", self.base))
    }

    #[doc = "Get a reference to the value of field `private_key` after provisioning.\n"]
    pub fn private_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.private_key", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElListenerElTlsElCertificateElSdsEl {
    secret_name: PrimField<String>,
}

impl AppmeshVirtualNodeSpecElListenerElTlsElCertificateElSdsEl { }

impl ToListMappable for AppmeshVirtualNodeSpecElListenerElTlsElCertificateElSdsEl {
    type O = BlockAssignable<AppmeshVirtualNodeSpecElListenerElTlsElCertificateElSdsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElListenerElTlsElCertificateElSdsEl {
    #[doc = ""]
    pub secret_name: PrimField<String>,
}

impl BuildAppmeshVirtualNodeSpecElListenerElTlsElCertificateElSdsEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElListenerElTlsElCertificateElSdsEl {
        AppmeshVirtualNodeSpecElListenerElTlsElCertificateElSdsEl { secret_name: self.secret_name }
    }
}

pub struct AppmeshVirtualNodeSpecElListenerElTlsElCertificateElSdsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshVirtualNodeSpecElListenerElTlsElCertificateElSdsElRef {
    fn new(shared: StackShared, base: String) -> AppmeshVirtualNodeSpecElListenerElTlsElCertificateElSdsElRef {
        AppmeshVirtualNodeSpecElListenerElTlsElCertificateElSdsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElListenerElTlsElCertificateElSdsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `secret_name` after provisioning.\n"]
    pub fn secret_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualNodeSpecElListenerElTlsElCertificateElDynamic {
    acm: Option<DynamicBlock<AppmeshVirtualNodeSpecElListenerElTlsElCertificateElAcmEl>>,
    file: Option<DynamicBlock<AppmeshVirtualNodeSpecElListenerElTlsElCertificateElFileEl>>,
    sds: Option<DynamicBlock<AppmeshVirtualNodeSpecElListenerElTlsElCertificateElSdsEl>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElListenerElTlsElCertificateEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    acm: Option<Vec<AppmeshVirtualNodeSpecElListenerElTlsElCertificateElAcmEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file: Option<Vec<AppmeshVirtualNodeSpecElListenerElTlsElCertificateElFileEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sds: Option<Vec<AppmeshVirtualNodeSpecElListenerElTlsElCertificateElSdsEl>>,
    dynamic: AppmeshVirtualNodeSpecElListenerElTlsElCertificateElDynamic,
}

impl AppmeshVirtualNodeSpecElListenerElTlsElCertificateEl {
    #[doc = "Set the field `acm`.\n"]
    pub fn set_acm(
        mut self,
        v: impl Into<BlockAssignable<AppmeshVirtualNodeSpecElListenerElTlsElCertificateElAcmEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.acm = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.acm = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `file`.\n"]
    pub fn set_file(
        mut self,
        v: impl Into<BlockAssignable<AppmeshVirtualNodeSpecElListenerElTlsElCertificateElFileEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.file = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.file = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sds`.\n"]
    pub fn set_sds(
        mut self,
        v: impl Into<BlockAssignable<AppmeshVirtualNodeSpecElListenerElTlsElCertificateElSdsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.sds = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.sds = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppmeshVirtualNodeSpecElListenerElTlsElCertificateEl {
    type O = BlockAssignable<AppmeshVirtualNodeSpecElListenerElTlsElCertificateEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElListenerElTlsElCertificateEl {}

impl BuildAppmeshVirtualNodeSpecElListenerElTlsElCertificateEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElListenerElTlsElCertificateEl {
        AppmeshVirtualNodeSpecElListenerElTlsElCertificateEl {
            acm: core::default::Default::default(),
            file: core::default::Default::default(),
            sds: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppmeshVirtualNodeSpecElListenerElTlsElCertificateElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshVirtualNodeSpecElListenerElTlsElCertificateElRef {
    fn new(shared: StackShared, base: String) -> AppmeshVirtualNodeSpecElListenerElTlsElCertificateElRef {
        AppmeshVirtualNodeSpecElListenerElTlsElCertificateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElListenerElTlsElCertificateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `acm` after provisioning.\n"]
    pub fn acm(&self) -> ListRef<AppmeshVirtualNodeSpecElListenerElTlsElCertificateElAcmElRef> {
        ListRef::new(self.shared().clone(), format!("{}.acm", self.base))
    }

    #[doc = "Get a reference to the value of field `file` after provisioning.\n"]
    pub fn file(&self) -> ListRef<AppmeshVirtualNodeSpecElListenerElTlsElCertificateElFileElRef> {
        ListRef::new(self.shared().clone(), format!("{}.file", self.base))
    }

    #[doc = "Get a reference to the value of field `sds` after provisioning.\n"]
    pub fn sds(&self) -> ListRef<AppmeshVirtualNodeSpecElListenerElTlsElCertificateElSdsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sds", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchEl {
    exact: SetField<PrimField<String>>,
}

impl AppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchEl { }

impl ToListMappable for AppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchEl {
    type O = BlockAssignable<AppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchEl {
    #[doc = ""]
    pub exact: SetField<PrimField<String>>,
}

impl BuildAppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchEl {
        AppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchEl { exact: self.exact }
    }
}

pub struct AppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchElRef {
        AppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `exact` after provisioning.\n"]
    pub fn exact(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.exact", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesElDynamic {
    match_: Option<
        DynamicBlock<AppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchEl>,
    >,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesEl {
    #[serde(rename = "match", skip_serializing_if = "Option::is_none")]
    match_: Option<Vec<AppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchEl>>,
    dynamic: AppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesElDynamic,
}

impl AppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesEl {
    #[doc = "Set the field `match_`.\n"]
    pub fn set_match(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            AppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.match_ = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.match_ = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesEl {
    type O = BlockAssignable<AppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesEl {}

impl BuildAppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesEl {
        AppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesEl {
            match_: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesElRef {
        AppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(
        &self,
    ) -> ListRef<AppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchElRef> {
        ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElFileEl {
    certificate_chain: PrimField<String>,
}

impl AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElFileEl { }

impl ToListMappable for AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElFileEl {
    type O = BlockAssignable<AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElFileEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElFileEl {
    #[doc = ""]
    pub certificate_chain: PrimField<String>,
}

impl BuildAppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElFileEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElFileEl {
        AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElFileEl {
            certificate_chain: self.certificate_chain,
        }
    }
}

pub struct AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElFileElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElFileElRef {
    fn new(shared: StackShared, base: String) -> AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElFileElRef {
        AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElFileElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElFileElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificate_chain` after provisioning.\n"]
    pub fn certificate_chain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.certificate_chain", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElSdsEl {
    secret_name: PrimField<String>,
}

impl AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElSdsEl { }

impl ToListMappable for AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElSdsEl {
    type O = BlockAssignable<AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElSdsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElSdsEl {
    #[doc = ""]
    pub secret_name: PrimField<String>,
}

impl BuildAppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElSdsEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElSdsEl {
        AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElSdsEl { secret_name: self.secret_name }
    }
}

pub struct AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElSdsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElSdsElRef {
    fn new(shared: StackShared, base: String) -> AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElSdsElRef {
        AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElSdsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElSdsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `secret_name` after provisioning.\n"]
    pub fn secret_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElDynamic {
    file: Option<DynamicBlock<AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElFileEl>>,
    sds: Option<DynamicBlock<AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElSdsEl>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    file: Option<Vec<AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElFileEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sds: Option<Vec<AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElSdsEl>>,
    dynamic: AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElDynamic,
}

impl AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustEl {
    #[doc = "Set the field `file`.\n"]
    pub fn set_file(
        mut self,
        v: impl Into<BlockAssignable<AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElFileEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.file = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.file = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sds`.\n"]
    pub fn set_sds(
        mut self,
        v: impl Into<BlockAssignable<AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElSdsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.sds = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.sds = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustEl {
    type O = BlockAssignable<AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustEl {}

impl BuildAppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustEl {
        AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustEl {
            file: core::default::Default::default(),
            sds: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElRef {
    fn new(shared: StackShared, base: String) -> AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElRef {
        AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `file` after provisioning.\n"]
    pub fn file(&self) -> ListRef<AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElFileElRef> {
        ListRef::new(self.shared().clone(), format!("{}.file", self.base))
    }

    #[doc = "Get a reference to the value of field `sds` after provisioning.\n"]
    pub fn sds(&self) -> ListRef<AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElSdsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sds", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualNodeSpecElListenerElTlsElValidationElDynamic {
    subject_alternative_names: Option<
        DynamicBlock<AppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesEl>,
    >,
    trust: Option<DynamicBlock<AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustEl>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElListenerElTlsElValidationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    subject_alternative_names: Option<Vec<AppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    trust: Option<Vec<AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustEl>>,
    dynamic: AppmeshVirtualNodeSpecElListenerElTlsElValidationElDynamic,
}

impl AppmeshVirtualNodeSpecElListenerElTlsElValidationEl {
    #[doc = "Set the field `subject_alternative_names`.\n"]
    pub fn set_subject_alternative_names(
        mut self,
        v: impl Into<BlockAssignable<AppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.subject_alternative_names = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.subject_alternative_names = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `trust`.\n"]
    pub fn set_trust(
        mut self,
        v: impl Into<BlockAssignable<AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.trust = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.trust = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppmeshVirtualNodeSpecElListenerElTlsElValidationEl {
    type O = BlockAssignable<AppmeshVirtualNodeSpecElListenerElTlsElValidationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElListenerElTlsElValidationEl {}

impl BuildAppmeshVirtualNodeSpecElListenerElTlsElValidationEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElListenerElTlsElValidationEl {
        AppmeshVirtualNodeSpecElListenerElTlsElValidationEl {
            subject_alternative_names: core::default::Default::default(),
            trust: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppmeshVirtualNodeSpecElListenerElTlsElValidationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshVirtualNodeSpecElListenerElTlsElValidationElRef {
    fn new(shared: StackShared, base: String) -> AppmeshVirtualNodeSpecElListenerElTlsElValidationElRef {
        AppmeshVirtualNodeSpecElListenerElTlsElValidationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElListenerElTlsElValidationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `subject_alternative_names` after provisioning.\n"]
    pub fn subject_alternative_names(
        &self,
    ) -> ListRef<AppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.subject_alternative_names", self.base))
    }

    #[doc = "Get a reference to the value of field `trust` after provisioning.\n"]
    pub fn trust(&self) -> ListRef<AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElRef> {
        ListRef::new(self.shared().clone(), format!("{}.trust", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualNodeSpecElListenerElTlsElDynamic {
    certificate: Option<DynamicBlock<AppmeshVirtualNodeSpecElListenerElTlsElCertificateEl>>,
    validation: Option<DynamicBlock<AppmeshVirtualNodeSpecElListenerElTlsElValidationEl>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElListenerElTlsEl {
    mode: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate: Option<Vec<AppmeshVirtualNodeSpecElListenerElTlsElCertificateEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    validation: Option<Vec<AppmeshVirtualNodeSpecElListenerElTlsElValidationEl>>,
    dynamic: AppmeshVirtualNodeSpecElListenerElTlsElDynamic,
}

impl AppmeshVirtualNodeSpecElListenerElTlsEl {
    #[doc = "Set the field `certificate`.\n"]
    pub fn set_certificate(
        mut self,
        v: impl Into<BlockAssignable<AppmeshVirtualNodeSpecElListenerElTlsElCertificateEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.certificate = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.certificate = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `validation`.\n"]
    pub fn set_validation(
        mut self,
        v: impl Into<BlockAssignable<AppmeshVirtualNodeSpecElListenerElTlsElValidationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.validation = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.validation = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppmeshVirtualNodeSpecElListenerElTlsEl {
    type O = BlockAssignable<AppmeshVirtualNodeSpecElListenerElTlsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElListenerElTlsEl {
    #[doc = ""]
    pub mode: PrimField<String>,
}

impl BuildAppmeshVirtualNodeSpecElListenerElTlsEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElListenerElTlsEl {
        AppmeshVirtualNodeSpecElListenerElTlsEl {
            mode: self.mode,
            certificate: core::default::Default::default(),
            validation: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppmeshVirtualNodeSpecElListenerElTlsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshVirtualNodeSpecElListenerElTlsElRef {
    fn new(shared: StackShared, base: String) -> AppmeshVirtualNodeSpecElListenerElTlsElRef {
        AppmeshVirtualNodeSpecElListenerElTlsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElListenerElTlsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc = "Get a reference to the value of field `certificate` after provisioning.\n"]
    pub fn certificate(&self) -> ListRef<AppmeshVirtualNodeSpecElListenerElTlsElCertificateElRef> {
        ListRef::new(self.shared().clone(), format!("{}.certificate", self.base))
    }

    #[doc = "Get a reference to the value of field `validation` after provisioning.\n"]
    pub fn validation(&self) -> ListRef<AppmeshVirtualNodeSpecElListenerElTlsElValidationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.validation", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualNodeSpecElListenerElDynamic {
    connection_pool: Option<DynamicBlock<AppmeshVirtualNodeSpecElListenerElConnectionPoolEl>>,
    health_check: Option<DynamicBlock<AppmeshVirtualNodeSpecElListenerElHealthCheckEl>>,
    outlier_detection: Option<DynamicBlock<AppmeshVirtualNodeSpecElListenerElOutlierDetectionEl>>,
    port_mapping: Option<DynamicBlock<AppmeshVirtualNodeSpecElListenerElPortMappingEl>>,
    timeout: Option<DynamicBlock<AppmeshVirtualNodeSpecElListenerElTimeoutEl>>,
    tls: Option<DynamicBlock<AppmeshVirtualNodeSpecElListenerElTlsEl>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElListenerEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    connection_pool: Option<Vec<AppmeshVirtualNodeSpecElListenerElConnectionPoolEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    health_check: Option<Vec<AppmeshVirtualNodeSpecElListenerElHealthCheckEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    outlier_detection: Option<Vec<AppmeshVirtualNodeSpecElListenerElOutlierDetectionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port_mapping: Option<Vec<AppmeshVirtualNodeSpecElListenerElPortMappingEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout: Option<Vec<AppmeshVirtualNodeSpecElListenerElTimeoutEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tls: Option<Vec<AppmeshVirtualNodeSpecElListenerElTlsEl>>,
    dynamic: AppmeshVirtualNodeSpecElListenerElDynamic,
}

impl AppmeshVirtualNodeSpecElListenerEl {
    #[doc = "Set the field `connection_pool`.\n"]
    pub fn set_connection_pool(
        mut self,
        v: impl Into<BlockAssignable<AppmeshVirtualNodeSpecElListenerElConnectionPoolEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.connection_pool = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.connection_pool = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `health_check`.\n"]
    pub fn set_health_check(
        mut self,
        v: impl Into<BlockAssignable<AppmeshVirtualNodeSpecElListenerElHealthCheckEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.health_check = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.health_check = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `outlier_detection`.\n"]
    pub fn set_outlier_detection(
        mut self,
        v: impl Into<BlockAssignable<AppmeshVirtualNodeSpecElListenerElOutlierDetectionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.outlier_detection = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.outlier_detection = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `port_mapping`.\n"]
    pub fn set_port_mapping(
        mut self,
        v: impl Into<BlockAssignable<AppmeshVirtualNodeSpecElListenerElPortMappingEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.port_mapping = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.port_mapping = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeout`.\n"]
    pub fn set_timeout(mut self, v: impl Into<BlockAssignable<AppmeshVirtualNodeSpecElListenerElTimeoutEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.timeout = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.timeout = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tls`.\n"]
    pub fn set_tls(mut self, v: impl Into<BlockAssignable<AppmeshVirtualNodeSpecElListenerElTlsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.tls = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.tls = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppmeshVirtualNodeSpecElListenerEl {
    type O = BlockAssignable<AppmeshVirtualNodeSpecElListenerEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElListenerEl {}

impl BuildAppmeshVirtualNodeSpecElListenerEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElListenerEl {
        AppmeshVirtualNodeSpecElListenerEl {
            connection_pool: core::default::Default::default(),
            health_check: core::default::Default::default(),
            outlier_detection: core::default::Default::default(),
            port_mapping: core::default::Default::default(),
            timeout: core::default::Default::default(),
            tls: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppmeshVirtualNodeSpecElListenerElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshVirtualNodeSpecElListenerElRef {
    fn new(shared: StackShared, base: String) -> AppmeshVirtualNodeSpecElListenerElRef {
        AppmeshVirtualNodeSpecElListenerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElListenerElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `connection_pool` after provisioning.\n"]
    pub fn connection_pool(&self) -> ListRef<AppmeshVirtualNodeSpecElListenerElConnectionPoolElRef> {
        ListRef::new(self.shared().clone(), format!("{}.connection_pool", self.base))
    }

    #[doc = "Get a reference to the value of field `health_check` after provisioning.\n"]
    pub fn health_check(&self) -> ListRef<AppmeshVirtualNodeSpecElListenerElHealthCheckElRef> {
        ListRef::new(self.shared().clone(), format!("{}.health_check", self.base))
    }

    #[doc = "Get a reference to the value of field `outlier_detection` after provisioning.\n"]
    pub fn outlier_detection(&self) -> ListRef<AppmeshVirtualNodeSpecElListenerElOutlierDetectionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.outlier_detection", self.base))
    }

    #[doc = "Get a reference to the value of field `port_mapping` after provisioning.\n"]
    pub fn port_mapping(&self) -> ListRef<AppmeshVirtualNodeSpecElListenerElPortMappingElRef> {
        ListRef::new(self.shared().clone(), format!("{}.port_mapping", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> ListRef<AppmeshVirtualNodeSpecElListenerElTimeoutElRef> {
        ListRef::new(self.shared().clone(), format!("{}.timeout", self.base))
    }

    #[doc = "Get a reference to the value of field `tls` after provisioning.\n"]
    pub fn tls(&self) -> ListRef<AppmeshVirtualNodeSpecElListenerElTlsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tls", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElLoggingElAccessLogElFileElFormatElJsonEl {
    key: PrimField<String>,
    value: PrimField<String>,
}

impl AppmeshVirtualNodeSpecElLoggingElAccessLogElFileElFormatElJsonEl { }

impl ToListMappable for AppmeshVirtualNodeSpecElLoggingElAccessLogElFileElFormatElJsonEl {
    type O = BlockAssignable<AppmeshVirtualNodeSpecElLoggingElAccessLogElFileElFormatElJsonEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElLoggingElAccessLogElFileElFormatElJsonEl {
    #[doc = ""]
    pub key: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildAppmeshVirtualNodeSpecElLoggingElAccessLogElFileElFormatElJsonEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElLoggingElAccessLogElFileElFormatElJsonEl {
        AppmeshVirtualNodeSpecElLoggingElAccessLogElFileElFormatElJsonEl {
            key: self.key,
            value: self.value,
        }
    }
}

pub struct AppmeshVirtualNodeSpecElLoggingElAccessLogElFileElFormatElJsonElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshVirtualNodeSpecElLoggingElAccessLogElFileElFormatElJsonElRef {
    fn new(shared: StackShared, base: String) -> AppmeshVirtualNodeSpecElLoggingElAccessLogElFileElFormatElJsonElRef {
        AppmeshVirtualNodeSpecElLoggingElAccessLogElFileElFormatElJsonElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElLoggingElAccessLogElFileElFormatElJsonElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualNodeSpecElLoggingElAccessLogElFileElFormatElDynamic {
    json: Option<DynamicBlock<AppmeshVirtualNodeSpecElLoggingElAccessLogElFileElFormatElJsonEl>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElLoggingElAccessLogElFileElFormatEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    json: Option<Vec<AppmeshVirtualNodeSpecElLoggingElAccessLogElFileElFormatElJsonEl>>,
    dynamic: AppmeshVirtualNodeSpecElLoggingElAccessLogElFileElFormatElDynamic,
}

impl AppmeshVirtualNodeSpecElLoggingElAccessLogElFileElFormatEl {
    #[doc = "Set the field `text`.\n"]
    pub fn set_text(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.text = Some(v.into());
        self
    }

    #[doc = "Set the field `json`.\n"]
    pub fn set_json(
        mut self,
        v: impl Into<BlockAssignable<AppmeshVirtualNodeSpecElLoggingElAccessLogElFileElFormatElJsonEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.json = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.json = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppmeshVirtualNodeSpecElLoggingElAccessLogElFileElFormatEl {
    type O = BlockAssignable<AppmeshVirtualNodeSpecElLoggingElAccessLogElFileElFormatEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElLoggingElAccessLogElFileElFormatEl {}

impl BuildAppmeshVirtualNodeSpecElLoggingElAccessLogElFileElFormatEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElLoggingElAccessLogElFileElFormatEl {
        AppmeshVirtualNodeSpecElLoggingElAccessLogElFileElFormatEl {
            text: core::default::Default::default(),
            json: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppmeshVirtualNodeSpecElLoggingElAccessLogElFileElFormatElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshVirtualNodeSpecElLoggingElAccessLogElFileElFormatElRef {
    fn new(shared: StackShared, base: String) -> AppmeshVirtualNodeSpecElLoggingElAccessLogElFileElFormatElRef {
        AppmeshVirtualNodeSpecElLoggingElAccessLogElFileElFormatElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElLoggingElAccessLogElFileElFormatElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `text` after provisioning.\n"]
    pub fn text(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.text", self.base))
    }

    #[doc = "Get a reference to the value of field `json` after provisioning.\n"]
    pub fn json(&self) -> ListRef<AppmeshVirtualNodeSpecElLoggingElAccessLogElFileElFormatElJsonElRef> {
        ListRef::new(self.shared().clone(), format!("{}.json", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualNodeSpecElLoggingElAccessLogElFileElDynamic {
    format: Option<DynamicBlock<AppmeshVirtualNodeSpecElLoggingElAccessLogElFileElFormatEl>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElLoggingElAccessLogElFileEl {
    path: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    format: Option<Vec<AppmeshVirtualNodeSpecElLoggingElAccessLogElFileElFormatEl>>,
    dynamic: AppmeshVirtualNodeSpecElLoggingElAccessLogElFileElDynamic,
}

impl AppmeshVirtualNodeSpecElLoggingElAccessLogElFileEl {
    #[doc = "Set the field `format`.\n"]
    pub fn set_format(
        mut self,
        v: impl Into<BlockAssignable<AppmeshVirtualNodeSpecElLoggingElAccessLogElFileElFormatEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.format = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.format = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppmeshVirtualNodeSpecElLoggingElAccessLogElFileEl {
    type O = BlockAssignable<AppmeshVirtualNodeSpecElLoggingElAccessLogElFileEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElLoggingElAccessLogElFileEl {
    #[doc = ""]
    pub path: PrimField<String>,
}

impl BuildAppmeshVirtualNodeSpecElLoggingElAccessLogElFileEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElLoggingElAccessLogElFileEl {
        AppmeshVirtualNodeSpecElLoggingElAccessLogElFileEl {
            path: self.path,
            format: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppmeshVirtualNodeSpecElLoggingElAccessLogElFileElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshVirtualNodeSpecElLoggingElAccessLogElFileElRef {
    fn new(shared: StackShared, base: String) -> AppmeshVirtualNodeSpecElLoggingElAccessLogElFileElRef {
        AppmeshVirtualNodeSpecElLoggingElAccessLogElFileElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElLoggingElAccessLogElFileElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc = "Get a reference to the value of field `format` after provisioning.\n"]
    pub fn format(&self) -> ListRef<AppmeshVirtualNodeSpecElLoggingElAccessLogElFileElFormatElRef> {
        ListRef::new(self.shared().clone(), format!("{}.format", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualNodeSpecElLoggingElAccessLogElDynamic {
    file: Option<DynamicBlock<AppmeshVirtualNodeSpecElLoggingElAccessLogElFileEl>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElLoggingElAccessLogEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    file: Option<Vec<AppmeshVirtualNodeSpecElLoggingElAccessLogElFileEl>>,
    dynamic: AppmeshVirtualNodeSpecElLoggingElAccessLogElDynamic,
}

impl AppmeshVirtualNodeSpecElLoggingElAccessLogEl {
    #[doc = "Set the field `file`.\n"]
    pub fn set_file(
        mut self,
        v: impl Into<BlockAssignable<AppmeshVirtualNodeSpecElLoggingElAccessLogElFileEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.file = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.file = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppmeshVirtualNodeSpecElLoggingElAccessLogEl {
    type O = BlockAssignable<AppmeshVirtualNodeSpecElLoggingElAccessLogEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElLoggingElAccessLogEl {}

impl BuildAppmeshVirtualNodeSpecElLoggingElAccessLogEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElLoggingElAccessLogEl {
        AppmeshVirtualNodeSpecElLoggingElAccessLogEl {
            file: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppmeshVirtualNodeSpecElLoggingElAccessLogElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshVirtualNodeSpecElLoggingElAccessLogElRef {
    fn new(shared: StackShared, base: String) -> AppmeshVirtualNodeSpecElLoggingElAccessLogElRef {
        AppmeshVirtualNodeSpecElLoggingElAccessLogElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElLoggingElAccessLogElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `file` after provisioning.\n"]
    pub fn file(&self) -> ListRef<AppmeshVirtualNodeSpecElLoggingElAccessLogElFileElRef> {
        ListRef::new(self.shared().clone(), format!("{}.file", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualNodeSpecElLoggingElDynamic {
    access_log: Option<DynamicBlock<AppmeshVirtualNodeSpecElLoggingElAccessLogEl>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElLoggingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    access_log: Option<Vec<AppmeshVirtualNodeSpecElLoggingElAccessLogEl>>,
    dynamic: AppmeshVirtualNodeSpecElLoggingElDynamic,
}

impl AppmeshVirtualNodeSpecElLoggingEl {
    #[doc = "Set the field `access_log`.\n"]
    pub fn set_access_log(
        mut self,
        v: impl Into<BlockAssignable<AppmeshVirtualNodeSpecElLoggingElAccessLogEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.access_log = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.access_log = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppmeshVirtualNodeSpecElLoggingEl {
    type O = BlockAssignable<AppmeshVirtualNodeSpecElLoggingEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElLoggingEl {}

impl BuildAppmeshVirtualNodeSpecElLoggingEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElLoggingEl {
        AppmeshVirtualNodeSpecElLoggingEl {
            access_log: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppmeshVirtualNodeSpecElLoggingElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshVirtualNodeSpecElLoggingElRef {
    fn new(shared: StackShared, base: String) -> AppmeshVirtualNodeSpecElLoggingElRef {
        AppmeshVirtualNodeSpecElLoggingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElLoggingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `access_log` after provisioning.\n"]
    pub fn access_log(&self) -> ListRef<AppmeshVirtualNodeSpecElLoggingElAccessLogElRef> {
        ListRef::new(self.shared().clone(), format!("{}.access_log", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElServiceDiscoveryElAwsCloudMapEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    attributes: Option<RecField<PrimField<String>>>,
    namespace_name: PrimField<String>,
    service_name: PrimField<String>,
}

impl AppmeshVirtualNodeSpecElServiceDiscoveryElAwsCloudMapEl {
    #[doc = "Set the field `attributes`.\n"]
    pub fn set_attributes(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.attributes = Some(v.into());
        self
    }
}

impl ToListMappable for AppmeshVirtualNodeSpecElServiceDiscoveryElAwsCloudMapEl {
    type O = BlockAssignable<AppmeshVirtualNodeSpecElServiceDiscoveryElAwsCloudMapEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElServiceDiscoveryElAwsCloudMapEl {
    #[doc = ""]
    pub namespace_name: PrimField<String>,
    #[doc = ""]
    pub service_name: PrimField<String>,
}

impl BuildAppmeshVirtualNodeSpecElServiceDiscoveryElAwsCloudMapEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElServiceDiscoveryElAwsCloudMapEl {
        AppmeshVirtualNodeSpecElServiceDiscoveryElAwsCloudMapEl {
            attributes: core::default::Default::default(),
            namespace_name: self.namespace_name,
            service_name: self.service_name,
        }
    }
}

pub struct AppmeshVirtualNodeSpecElServiceDiscoveryElAwsCloudMapElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshVirtualNodeSpecElServiceDiscoveryElAwsCloudMapElRef {
    fn new(shared: StackShared, base: String) -> AppmeshVirtualNodeSpecElServiceDiscoveryElAwsCloudMapElRef {
        AppmeshVirtualNodeSpecElServiceDiscoveryElAwsCloudMapElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElServiceDiscoveryElAwsCloudMapElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `attributes` after provisioning.\n"]
    pub fn attributes(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.attributes", self.base))
    }

    #[doc = "Get a reference to the value of field `namespace_name` after provisioning.\n"]
    pub fn namespace_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.namespace_name", self.base))
    }

    #[doc = "Get a reference to the value of field `service_name` after provisioning.\n"]
    pub fn service_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_name", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElServiceDiscoveryElDnsEl {
    hostname: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_preference: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_type: Option<PrimField<String>>,
}

impl AppmeshVirtualNodeSpecElServiceDiscoveryElDnsEl {
    #[doc = "Set the field `ip_preference`.\n"]
    pub fn set_ip_preference(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ip_preference = Some(v.into());
        self
    }

    #[doc = "Set the field `response_type`.\n"]
    pub fn set_response_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.response_type = Some(v.into());
        self
    }
}

impl ToListMappable for AppmeshVirtualNodeSpecElServiceDiscoveryElDnsEl {
    type O = BlockAssignable<AppmeshVirtualNodeSpecElServiceDiscoveryElDnsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElServiceDiscoveryElDnsEl {
    #[doc = ""]
    pub hostname: PrimField<String>,
}

impl BuildAppmeshVirtualNodeSpecElServiceDiscoveryElDnsEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElServiceDiscoveryElDnsEl {
        AppmeshVirtualNodeSpecElServiceDiscoveryElDnsEl {
            hostname: self.hostname,
            ip_preference: core::default::Default::default(),
            response_type: core::default::Default::default(),
        }
    }
}

pub struct AppmeshVirtualNodeSpecElServiceDiscoveryElDnsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshVirtualNodeSpecElServiceDiscoveryElDnsElRef {
    fn new(shared: StackShared, base: String) -> AppmeshVirtualNodeSpecElServiceDiscoveryElDnsElRef {
        AppmeshVirtualNodeSpecElServiceDiscoveryElDnsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElServiceDiscoveryElDnsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hostname` after provisioning.\n"]
    pub fn hostname(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_preference` after provisioning.\n"]
    pub fn ip_preference(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip_preference", self.base))
    }

    #[doc = "Get a reference to the value of field `response_type` after provisioning.\n"]
    pub fn response_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.response_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualNodeSpecElServiceDiscoveryElDynamic {
    aws_cloud_map: Option<DynamicBlock<AppmeshVirtualNodeSpecElServiceDiscoveryElAwsCloudMapEl>>,
    dns: Option<DynamicBlock<AppmeshVirtualNodeSpecElServiceDiscoveryElDnsEl>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElServiceDiscoveryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    aws_cloud_map: Option<Vec<AppmeshVirtualNodeSpecElServiceDiscoveryElAwsCloudMapEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dns: Option<Vec<AppmeshVirtualNodeSpecElServiceDiscoveryElDnsEl>>,
    dynamic: AppmeshVirtualNodeSpecElServiceDiscoveryElDynamic,
}

impl AppmeshVirtualNodeSpecElServiceDiscoveryEl {
    #[doc = "Set the field `aws_cloud_map`.\n"]
    pub fn set_aws_cloud_map(
        mut self,
        v: impl Into<BlockAssignable<AppmeshVirtualNodeSpecElServiceDiscoveryElAwsCloudMapEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.aws_cloud_map = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.aws_cloud_map = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `dns`.\n"]
    pub fn set_dns(mut self, v: impl Into<BlockAssignable<AppmeshVirtualNodeSpecElServiceDiscoveryElDnsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.dns = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.dns = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppmeshVirtualNodeSpecElServiceDiscoveryEl {
    type O = BlockAssignable<AppmeshVirtualNodeSpecElServiceDiscoveryEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElServiceDiscoveryEl {}

impl BuildAppmeshVirtualNodeSpecElServiceDiscoveryEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElServiceDiscoveryEl {
        AppmeshVirtualNodeSpecElServiceDiscoveryEl {
            aws_cloud_map: core::default::Default::default(),
            dns: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppmeshVirtualNodeSpecElServiceDiscoveryElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshVirtualNodeSpecElServiceDiscoveryElRef {
    fn new(shared: StackShared, base: String) -> AppmeshVirtualNodeSpecElServiceDiscoveryElRef {
        AppmeshVirtualNodeSpecElServiceDiscoveryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElServiceDiscoveryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `aws_cloud_map` after provisioning.\n"]
    pub fn aws_cloud_map(&self) -> ListRef<AppmeshVirtualNodeSpecElServiceDiscoveryElAwsCloudMapElRef> {
        ListRef::new(self.shared().clone(), format!("{}.aws_cloud_map", self.base))
    }

    #[doc = "Get a reference to the value of field `dns` after provisioning.\n"]
    pub fn dns(&self) -> ListRef<AppmeshVirtualNodeSpecElServiceDiscoveryElDnsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.dns", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualNodeSpecElDynamic {
    backend: Option<DynamicBlock<AppmeshVirtualNodeSpecElBackendEl>>,
    backend_defaults: Option<DynamicBlock<AppmeshVirtualNodeSpecElBackendDefaultsEl>>,
    listener: Option<DynamicBlock<AppmeshVirtualNodeSpecElListenerEl>>,
    logging: Option<DynamicBlock<AppmeshVirtualNodeSpecElLoggingEl>>,
    service_discovery: Option<DynamicBlock<AppmeshVirtualNodeSpecElServiceDiscoveryEl>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    backend: Option<Vec<AppmeshVirtualNodeSpecElBackendEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    backend_defaults: Option<Vec<AppmeshVirtualNodeSpecElBackendDefaultsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    listener: Option<Vec<AppmeshVirtualNodeSpecElListenerEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logging: Option<Vec<AppmeshVirtualNodeSpecElLoggingEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_discovery: Option<Vec<AppmeshVirtualNodeSpecElServiceDiscoveryEl>>,
    dynamic: AppmeshVirtualNodeSpecElDynamic,
}

impl AppmeshVirtualNodeSpecEl {
    #[doc = "Set the field `backend`.\n"]
    pub fn set_backend(mut self, v: impl Into<BlockAssignable<AppmeshVirtualNodeSpecElBackendEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.backend = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.backend = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `backend_defaults`.\n"]
    pub fn set_backend_defaults(
        mut self,
        v: impl Into<BlockAssignable<AppmeshVirtualNodeSpecElBackendDefaultsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.backend_defaults = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.backend_defaults = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `listener`.\n"]
    pub fn set_listener(mut self, v: impl Into<BlockAssignable<AppmeshVirtualNodeSpecElListenerEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.listener = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.listener = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `logging`.\n"]
    pub fn set_logging(mut self, v: impl Into<BlockAssignable<AppmeshVirtualNodeSpecElLoggingEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.logging = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.logging = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `service_discovery`.\n"]
    pub fn set_service_discovery(
        mut self,
        v: impl Into<BlockAssignable<AppmeshVirtualNodeSpecElServiceDiscoveryEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.service_discovery = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.service_discovery = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppmeshVirtualNodeSpecEl {
    type O = BlockAssignable<AppmeshVirtualNodeSpecEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecEl {}

impl BuildAppmeshVirtualNodeSpecEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecEl {
        AppmeshVirtualNodeSpecEl {
            backend: core::default::Default::default(),
            backend_defaults: core::default::Default::default(),
            listener: core::default::Default::default(),
            logging: core::default::Default::default(),
            service_discovery: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppmeshVirtualNodeSpecElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshVirtualNodeSpecElRef {
    fn new(shared: StackShared, base: String) -> AppmeshVirtualNodeSpecElRef {
        AppmeshVirtualNodeSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `backend_defaults` after provisioning.\n"]
    pub fn backend_defaults(&self) -> ListRef<AppmeshVirtualNodeSpecElBackendDefaultsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.backend_defaults", self.base))
    }

    #[doc = "Get a reference to the value of field `listener` after provisioning.\n"]
    pub fn listener(&self) -> ListRef<AppmeshVirtualNodeSpecElListenerElRef> {
        ListRef::new(self.shared().clone(), format!("{}.listener", self.base))
    }

    #[doc = "Get a reference to the value of field `logging` after provisioning.\n"]
    pub fn logging(&self) -> ListRef<AppmeshVirtualNodeSpecElLoggingElRef> {
        ListRef::new(self.shared().clone(), format!("{}.logging", self.base))
    }

    #[doc = "Get a reference to the value of field `service_discovery` after provisioning.\n"]
    pub fn service_discovery(&self) -> ListRef<AppmeshVirtualNodeSpecElServiceDiscoveryElRef> {
        ListRef::new(self.shared().clone(), format!("{}.service_discovery", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualNodeDynamic {
    spec: Option<DynamicBlock<AppmeshVirtualNodeSpecEl>>,
}
