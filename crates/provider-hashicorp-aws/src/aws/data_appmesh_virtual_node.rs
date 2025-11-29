use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataAppmeshVirtualNodeData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
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
}

struct DataAppmeshVirtualNode_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataAppmeshVirtualNodeData>,
}

#[derive(Clone)]
pub struct DataAppmeshVirtualNode(Rc<DataAppmeshVirtualNode_>);

impl DataAppmeshVirtualNode {
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

    #[doc = "Get a reference to the value of field `spec` after provisioning.\n"]
    pub fn spec(&self) -> ListRef<DataAppmeshVirtualNodeSpecElRef> {
        ListRef::new(self.shared().clone(), format!("{}.spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

impl Referable for DataAppmeshVirtualNode {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataAppmeshVirtualNode { }

impl ToListMappable for DataAppmeshVirtualNode {
    type O = ListRef<DataAppmeshVirtualNodeRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataAppmeshVirtualNode_ {
    fn extract_datasource_type(&self) -> String {
        "aws_appmesh_virtual_node".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataAppmeshVirtualNode {
    pub tf_id: String,
    #[doc = ""]
    pub mesh_name: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildDataAppmeshVirtualNode {
    pub fn build(self, stack: &mut Stack) -> DataAppmeshVirtualNode {
        let out = DataAppmeshVirtualNode(Rc::new(DataAppmeshVirtualNode_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataAppmeshVirtualNodeData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                mesh_name: self.mesh_name,
                mesh_owner: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataAppmeshVirtualNodeRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualNodeRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataAppmeshVirtualNodeRef {
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

    #[doc = "Get a reference to the value of field `spec` after provisioning.\n"]
    pub fn spec(&self) -> ListRef<DataAppmeshVirtualNodeSpecElRef> {
        ListRef::new(self.shared().clone(), format!("{}.spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElFileEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate_chain: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    private_key: Option<PrimField<String>>,
}

impl DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElFileEl {
    #[doc = "Set the field `certificate_chain`.\n"]
    pub fn set_certificate_chain(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.certificate_chain = Some(v.into());
        self
    }

    #[doc = "Set the field `private_key`.\n"]
    pub fn set_private_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.private_key = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElFileEl {
    type O =
        BlockAssignable<DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElFileEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElFileEl {}

impl BuildDataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElFileEl {
    pub fn build(
        self,
    ) -> DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElFileEl {
        DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElFileEl {
            certificate_chain: core::default::Default::default(),
            private_key: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElFileElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElFileElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElFileElRef {
        DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElFileElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElFileElRef {
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
pub struct DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElSdsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_name: Option<PrimField<String>>,
}

impl DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElSdsEl {
    #[doc = "Set the field `secret_name`.\n"]
    pub fn set_secret_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.secret_name = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElSdsEl {
    type O =
        BlockAssignable<DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElSdsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElSdsEl {}

impl BuildDataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElSdsEl {
    pub fn build(self) -> DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElSdsEl {
        DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElSdsEl {
            secret_name: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElSdsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElSdsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElSdsElRef {
        DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElSdsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElSdsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `secret_name` after provisioning.\n"]
    pub fn secret_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    file: Option<ListField<DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElFileEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sds: Option<ListField<DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElSdsEl>>,
}

impl DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateEl {
    #[doc = "Set the field `file`.\n"]
    pub fn set_file(
        mut self,
        v:
            impl

                    Into<
                        ListField<
                            DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElFileEl,
                        >,
                    >,
    ) -> Self {
        self.file = Some(v.into());
        self
    }

    #[doc = "Set the field `sds`.\n"]
    pub fn set_sds(
        mut self,
        v:
            impl

                    Into<
                        ListField<
                            DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElSdsEl,
                        >,
                    >,
    ) -> Self {
        self.sds = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateEl {
    type O = BlockAssignable<DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateEl {}

impl BuildDataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateEl {
    pub fn build(self) -> DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateEl {
        DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateEl {
            file: core::default::Default::default(),
            sds: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElRef {
        DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `file` after provisioning.\n"]
    pub fn file(
        &self,
    ) -> ListRef<DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElFileElRef> {
        ListRef::new(self.shared().clone(), format!("{}.file", self.base))
    }

    #[doc = "Get a reference to the value of field `sds` after provisioning.\n"]
    pub fn sds(
        &self,
    ) -> ListRef<DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElSdsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sds", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    exact: Option<SetField<PrimField<String>>>,
}

impl DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl {
    #[doc = "Set the field `exact`.\n"]
    pub fn set_exact(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.exact = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl {
    type O =
        BlockAssignable<
            DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl {}

impl BuildDataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl {
    pub fn build(
        self,
    ) -> DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl {
        DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl {
            exact: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchElRef {
        DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `exact` after provisioning.\n"]
    pub fn exact(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.exact", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl {
    #[serde(rename = "match", skip_serializing_if = "Option::is_none")]
    match_: Option<
        ListField<
            DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl,
        >,
    >,
}

impl DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl {
    #[doc = "Set the field `match_`.\n"]
    pub fn set_match(
        mut self,
        v:
            impl

                    Into<
                        ListField<
                            DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl,
                        >,
                    >,
    ) -> Self {
        self.match_ = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl {
    type O =
        BlockAssignable<
            DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl {}

impl BuildDataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl {
    pub fn build(
        self,
    ) -> DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl {
        DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl {
            match_: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesElRef {
        DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(
        &self,
    ) -> ListRef<
        DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElAcmEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate_authority_arns: Option<SetField<PrimField<String>>>,
}

impl DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElAcmEl {
    #[doc = "Set the field `certificate_authority_arns`.\n"]
    pub fn set_certificate_authority_arns(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.certificate_authority_arns = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElAcmEl {
    type O =
        BlockAssignable<
            DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElAcmEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElAcmEl {}

impl BuildDataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElAcmEl {
    pub fn build(
        self,
    ) -> DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElAcmEl {
        DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElAcmEl {
            certificate_authority_arns: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElAcmElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElAcmElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElAcmElRef {
        DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElAcmElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElAcmElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificate_authority_arns` after provisioning.\n"]
    pub fn certificate_authority_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.certificate_authority_arns", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElFileEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate_chain: Option<PrimField<String>>,
}

impl DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElFileEl {
    #[doc = "Set the field `certificate_chain`.\n"]
    pub fn set_certificate_chain(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.certificate_chain = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElFileEl {
    type O =
        BlockAssignable<
            DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElFileEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElFileEl {}

impl BuildDataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElFileEl {
    pub fn build(
        self,
    ) -> DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElFileEl {
        DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElFileEl {
            certificate_chain: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElFileElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElFileElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElFileElRef {
        DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElFileElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElFileElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificate_chain` after provisioning.\n"]
    pub fn certificate_chain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.certificate_chain", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElSdsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_name: Option<PrimField<String>>,
}

impl DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElSdsEl {
    #[doc = "Set the field `secret_name`.\n"]
    pub fn set_secret_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.secret_name = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElSdsEl {
    type O =
        BlockAssignable<
            DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElSdsEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElSdsEl {}

impl BuildDataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElSdsEl {
    pub fn build(
        self,
    ) -> DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElSdsEl {
        DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElSdsEl {
            secret_name: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElSdsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElSdsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElSdsElRef {
        DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElSdsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElSdsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `secret_name` after provisioning.\n"]
    pub fn secret_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    acm: Option<
        ListField<DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElAcmEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    file: Option<
        ListField<DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElFileEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    sds: Option<
        ListField<DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElSdsEl>,
    >,
}

impl DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustEl {
    #[doc = "Set the field `acm`.\n"]
    pub fn set_acm(
        mut self,
        v:
            impl

                    Into<
                        ListField<
                            DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElAcmEl,
                        >,
                    >,
    ) -> Self {
        self.acm = Some(v.into());
        self
    }

    #[doc = "Set the field `file`.\n"]
    pub fn set_file(
        mut self,
        v:
            impl

                    Into<
                        ListField<
                            DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElFileEl,
                        >,
                    >,
    ) -> Self {
        self.file = Some(v.into());
        self
    }

    #[doc = "Set the field `sds`.\n"]
    pub fn set_sds(
        mut self,
        v:
            impl

                    Into<
                        ListField<
                            DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElSdsEl,
                        >,
                    >,
    ) -> Self {
        self.sds = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustEl {
    type O =
        BlockAssignable<DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustEl {}

impl BuildDataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustEl {
    pub fn build(
        self,
    ) -> DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustEl {
        DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustEl {
            acm: core::default::Default::default(),
            file: core::default::Default::default(),
            sds: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElRef {
        DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `acm` after provisioning.\n"]
    pub fn acm(
        &self,
    ) -> ListRef<DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElAcmElRef> {
        ListRef::new(self.shared().clone(), format!("{}.acm", self.base))
    }

    #[doc = "Get a reference to the value of field `file` after provisioning.\n"]
    pub fn file(
        &self,
    ) -> ListRef<
        DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElFileElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.file", self.base))
    }

    #[doc = "Get a reference to the value of field `sds` after provisioning.\n"]
    pub fn sds(
        &self,
    ) -> ListRef<DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElSdsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sds", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    subject_alternative_names: Option<
        ListField<
            DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    trust: Option<
        ListField<DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustEl>,
    >,
}

impl DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationEl {
    #[doc = "Set the field `subject_alternative_names`.\n"]
    pub fn set_subject_alternative_names(
        mut self,
        v:
            impl

                    Into<
                        ListField<
                            DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl,
                        >,
                    >,
    ) -> Self {
        self.subject_alternative_names = Some(v.into());
        self
    }

    #[doc = "Set the field `trust`.\n"]
    pub fn set_trust(
        mut self,
        v:
            impl

                    Into<
                        ListField<
                            DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustEl,
                        >,
                    >,
    ) -> Self {
        self.trust = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationEl {
    type O = BlockAssignable<DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationEl {}

impl BuildDataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationEl {
    pub fn build(self) -> DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationEl {
        DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationEl {
            subject_alternative_names: core::default::Default::default(),
            trust: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElRef {
        DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `subject_alternative_names` after provisioning.\n"]
    pub fn subject_alternative_names(
        &self,
    ) -> ListRef<
        DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.subject_alternative_names", self.base))
    }

    #[doc = "Get a reference to the value of field `trust` after provisioning.\n"]
    pub fn trust(
        &self,
    ) -> ListRef<DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElRef> {
        ListRef::new(self.shared().clone(), format!("{}.trust", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate: Option<ListField<DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enforce: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ports: Option<SetField<PrimField<f64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    validation: Option<ListField<DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationEl>>,
}

impl DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsEl {
    #[doc = "Set the field `certificate`.\n"]
    pub fn set_certificate(
        mut self,
        v:
            impl

                    Into<
                        ListField<
                            DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateEl,
                        >,
                    >,
    ) -> Self {
        self.certificate = Some(v.into());
        self
    }

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

    #[doc = "Set the field `validation`.\n"]
    pub fn set_validation(
        mut self,
        v:
            impl

                    Into<
                        ListField<
                            DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationEl,
                        >,
                    >,
    ) -> Self {
        self.validation = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsEl {
    type O = BlockAssignable<DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsEl {}

impl BuildDataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsEl {
    pub fn build(self) -> DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsEl {
        DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsEl {
            certificate: core::default::Default::default(),
            enforce: core::default::Default::default(),
            ports: core::default::Default::default(),
            validation: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElRef {
        DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificate` after provisioning.\n"]
    pub fn certificate(
        &self,
    ) -> ListRef<DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElRef> {
        ListRef::new(self.shared().clone(), format!("{}.certificate", self.base))
    }

    #[doc = "Get a reference to the value of field `enforce` after provisioning.\n"]
    pub fn enforce(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enforce", self.base))
    }

    #[doc = "Get a reference to the value of field `ports` after provisioning.\n"]
    pub fn ports(&self) -> SetRef<PrimExpr<f64>> {
        SetRef::new(self.shared().clone(), format!("{}.ports", self.base))
    }

    #[doc = "Get a reference to the value of field `validation` after provisioning.\n"]
    pub fn validation(
        &self,
    ) -> ListRef<DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.validation", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    tls: Option<ListField<DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsEl>>,
}

impl DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyEl {
    #[doc = "Set the field `tls`.\n"]
    pub fn set_tls(
        mut self,
        v: impl Into<ListField<DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsEl>>,
    ) -> Self {
        self.tls = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyEl {
    type O = BlockAssignable<DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyEl {}

impl BuildDataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyEl {
    pub fn build(self) -> DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyEl {
        DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyEl {
            tls: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElRef {
        DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `tls` after provisioning.\n"]
    pub fn tls(&self) -> ListRef<DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tls", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualNodeSpecElBackendElVirtualServiceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    client_policy: Option<ListField<DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    virtual_service_name: Option<PrimField<String>>,
}

impl DataAppmeshVirtualNodeSpecElBackendElVirtualServiceEl {
    #[doc = "Set the field `client_policy`.\n"]
    pub fn set_client_policy(
        mut self,
        v: impl Into<ListField<DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyEl>>,
    ) -> Self {
        self.client_policy = Some(v.into());
        self
    }

    #[doc = "Set the field `virtual_service_name`.\n"]
    pub fn set_virtual_service_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.virtual_service_name = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualNodeSpecElBackendElVirtualServiceEl {
    type O = BlockAssignable<DataAppmeshVirtualNodeSpecElBackendElVirtualServiceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualNodeSpecElBackendElVirtualServiceEl {}

impl BuildDataAppmeshVirtualNodeSpecElBackendElVirtualServiceEl {
    pub fn build(self) -> DataAppmeshVirtualNodeSpecElBackendElVirtualServiceEl {
        DataAppmeshVirtualNodeSpecElBackendElVirtualServiceEl {
            client_policy: core::default::Default::default(),
            virtual_service_name: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElRef {
        DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `client_policy` after provisioning.\n"]
    pub fn client_policy(&self) -> ListRef<DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.client_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `virtual_service_name` after provisioning.\n"]
    pub fn virtual_service_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.virtual_service_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualNodeSpecElBackendEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    virtual_service: Option<ListField<DataAppmeshVirtualNodeSpecElBackendElVirtualServiceEl>>,
}

impl DataAppmeshVirtualNodeSpecElBackendEl {
    #[doc = "Set the field `virtual_service`.\n"]
    pub fn set_virtual_service(
        mut self,
        v: impl Into<ListField<DataAppmeshVirtualNodeSpecElBackendElVirtualServiceEl>>,
    ) -> Self {
        self.virtual_service = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualNodeSpecElBackendEl {
    type O = BlockAssignable<DataAppmeshVirtualNodeSpecElBackendEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualNodeSpecElBackendEl {}

impl BuildDataAppmeshVirtualNodeSpecElBackendEl {
    pub fn build(self) -> DataAppmeshVirtualNodeSpecElBackendEl {
        DataAppmeshVirtualNodeSpecElBackendEl { virtual_service: core::default::Default::default() }
    }
}

pub struct DataAppmeshVirtualNodeSpecElBackendElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualNodeSpecElBackendElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshVirtualNodeSpecElBackendElRef {
        DataAppmeshVirtualNodeSpecElBackendElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualNodeSpecElBackendElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `virtual_service` after provisioning.\n"]
    pub fn virtual_service(&self) -> ListRef<DataAppmeshVirtualNodeSpecElBackendElVirtualServiceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.virtual_service", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate_chain: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    private_key: Option<PrimField<String>>,
}

impl DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileEl {
    #[doc = "Set the field `certificate_chain`.\n"]
    pub fn set_certificate_chain(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.certificate_chain = Some(v.into());
        self
    }

    #[doc = "Set the field `private_key`.\n"]
    pub fn set_private_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.private_key = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileEl {
    type O = BlockAssignable<DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileEl {}

impl BuildDataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileEl {
    pub fn build(self) -> DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileEl {
        DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileEl {
            certificate_chain: core::default::Default::default(),
            private_key: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileElRef {
        DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileElRef {
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
pub struct DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_name: Option<PrimField<String>>,
}

impl DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsEl {
    #[doc = "Set the field `secret_name`.\n"]
    pub fn set_secret_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.secret_name = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsEl {
    type O = BlockAssignable<DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsEl {}

impl BuildDataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsEl {
    pub fn build(self) -> DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsEl {
        DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsEl {
            secret_name: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsElRef {
        DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `secret_name` after provisioning.\n"]
    pub fn secret_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    file: Option<ListField<DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sds: Option<ListField<DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsEl>>,
}

impl DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateEl {
    #[doc = "Set the field `file`.\n"]
    pub fn set_file(
        mut self,
        v: impl Into<ListField<DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileEl>>,
    ) -> Self {
        self.file = Some(v.into());
        self
    }

    #[doc = "Set the field `sds`.\n"]
    pub fn set_sds(
        mut self,
        v: impl Into<ListField<DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsEl>>,
    ) -> Self {
        self.sds = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateEl {
    type O = BlockAssignable<DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateEl {}

impl BuildDataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateEl {
    pub fn build(self) -> DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateEl {
        DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateEl {
            file: core::default::Default::default(),
            sds: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElRef {
        DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `file` after provisioning.\n"]
    pub fn file(
        &self,
    ) -> ListRef<DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileElRef> {
        ListRef::new(self.shared().clone(), format!("{}.file", self.base))
    }

    #[doc = "Get a reference to the value of field `sds` after provisioning.\n"]
    pub fn sds(&self) -> ListRef<DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sds", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    exact: Option<SetField<PrimField<String>>>,
}

impl DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl {
    #[doc = "Set the field `exact`.\n"]
    pub fn set_exact(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.exact = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl {
    type O =
        BlockAssignable<
            DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl {}

impl BuildDataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl {
    pub fn build(
        self,
    ) -> DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl {
        DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl {
            exact: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchElRef {
        DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `exact` after provisioning.\n"]
    pub fn exact(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.exact", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl {
    #[serde(rename = "match", skip_serializing_if = "Option::is_none")]
    match_: Option<
        ListField<
            DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl,
        >,
    >,
}

impl DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl {
    #[doc = "Set the field `match_`.\n"]
    pub fn set_match(
        mut self,
        v:
            impl

                    Into<
                        ListField<
                            DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl,
                        >,
                    >,
    ) -> Self {
        self.match_ = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl {
    type O =
        BlockAssignable<
            DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl {}

impl BuildDataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl {
    pub fn build(
        self,
    ) -> DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl {
        DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl {
            match_: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElRef {
        DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(
        &self,
    ) -> ListRef<
        DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate_authority_arns: Option<SetField<PrimField<String>>>,
}

impl DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmEl {
    #[doc = "Set the field `certificate_authority_arns`.\n"]
    pub fn set_certificate_authority_arns(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.certificate_authority_arns = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmEl {
    type O =
        BlockAssignable<DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmEl {}

impl BuildDataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmEl {
    pub fn build(self) -> DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmEl {
        DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmEl {
            certificate_authority_arns: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmElRef {
        DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificate_authority_arns` after provisioning.\n"]
    pub fn certificate_authority_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.certificate_authority_arns", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate_chain: Option<PrimField<String>>,
}

impl DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileEl {
    #[doc = "Set the field `certificate_chain`.\n"]
    pub fn set_certificate_chain(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.certificate_chain = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileEl {
    type O =
        BlockAssignable<DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileEl {}

impl BuildDataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileEl {
    pub fn build(self) -> DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileEl {
        DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileEl {
            certificate_chain: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileElRef {
        DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificate_chain` after provisioning.\n"]
    pub fn certificate_chain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.certificate_chain", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_name: Option<PrimField<String>>,
}

impl DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsEl {
    #[doc = "Set the field `secret_name`.\n"]
    pub fn set_secret_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.secret_name = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsEl {
    type O =
        BlockAssignable<DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsEl {}

impl BuildDataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsEl {
    pub fn build(self) -> DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsEl {
        DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsEl {
            secret_name: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsElRef {
        DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `secret_name` after provisioning.\n"]
    pub fn secret_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    acm: Option<ListField<DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file: Option<ListField<DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sds: Option<ListField<DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsEl>>,
}

impl DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustEl {
    #[doc = "Set the field `acm`.\n"]
    pub fn set_acm(
        mut self,
        v:
            impl

                    Into<
                        ListField<
                            DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmEl,
                        >,
                    >,
    ) -> Self {
        self.acm = Some(v.into());
        self
    }

    #[doc = "Set the field `file`.\n"]
    pub fn set_file(
        mut self,
        v:
            impl

                    Into<
                        ListField<
                            DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileEl,
                        >,
                    >,
    ) -> Self {
        self.file = Some(v.into());
        self
    }

    #[doc = "Set the field `sds`.\n"]
    pub fn set_sds(
        mut self,
        v:
            impl

                    Into<
                        ListField<
                            DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsEl,
                        >,
                    >,
    ) -> Self {
        self.sds = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustEl {
    type O = BlockAssignable<DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustEl {}

impl BuildDataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustEl {
    pub fn build(self) -> DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustEl {
        DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustEl {
            acm: core::default::Default::default(),
            file: core::default::Default::default(),
            sds: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElRef {
        DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `acm` after provisioning.\n"]
    pub fn acm(
        &self,
    ) -> ListRef<DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmElRef> {
        ListRef::new(self.shared().clone(), format!("{}.acm", self.base))
    }

    #[doc = "Get a reference to the value of field `file` after provisioning.\n"]
    pub fn file(
        &self,
    ) -> ListRef<DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileElRef> {
        ListRef::new(self.shared().clone(), format!("{}.file", self.base))
    }

    #[doc = "Get a reference to the value of field `sds` after provisioning.\n"]
    pub fn sds(
        &self,
    ) -> ListRef<DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sds", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    subject_alternative_names: Option<
        ListField<
            DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    trust: Option<ListField<DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustEl>>,
}

impl DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationEl {
    #[doc = "Set the field `subject_alternative_names`.\n"]
    pub fn set_subject_alternative_names(
        mut self,
        v:
            impl

                    Into<
                        ListField<
                            DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl,
                        >,
                    >,
    ) -> Self {
        self.subject_alternative_names = Some(v.into());
        self
    }

    #[doc = "Set the field `trust`.\n"]
    pub fn set_trust(
        mut self,
        v: impl Into<ListField<DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustEl>>,
    ) -> Self {
        self.trust = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationEl {
    type O = BlockAssignable<DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationEl {}

impl BuildDataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationEl {
    pub fn build(self) -> DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationEl {
        DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationEl {
            subject_alternative_names: core::default::Default::default(),
            trust: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElRef {
        DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `subject_alternative_names` after provisioning.\n"]
    pub fn subject_alternative_names(
        &self,
    ) -> ListRef<
        DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.subject_alternative_names", self.base))
    }

    #[doc = "Get a reference to the value of field `trust` after provisioning.\n"]
    pub fn trust(
        &self,
    ) -> ListRef<DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElRef> {
        ListRef::new(self.shared().clone(), format!("{}.trust", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate: Option<ListField<DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enforce: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ports: Option<SetField<PrimField<f64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    validation: Option<ListField<DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationEl>>,
}

impl DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsEl {
    #[doc = "Set the field `certificate`.\n"]
    pub fn set_certificate(
        mut self,
        v: impl Into<ListField<DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateEl>>,
    ) -> Self {
        self.certificate = Some(v.into());
        self
    }

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

    #[doc = "Set the field `validation`.\n"]
    pub fn set_validation(
        mut self,
        v: impl Into<ListField<DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationEl>>,
    ) -> Self {
        self.validation = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsEl {
    type O = BlockAssignable<DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsEl {}

impl BuildDataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsEl {
    pub fn build(self) -> DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsEl {
        DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsEl {
            certificate: core::default::Default::default(),
            enforce: core::default::Default::default(),
            ports: core::default::Default::default(),
            validation: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElRef {
        DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificate` after provisioning.\n"]
    pub fn certificate(
        &self,
    ) -> ListRef<DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElRef> {
        ListRef::new(self.shared().clone(), format!("{}.certificate", self.base))
    }

    #[doc = "Get a reference to the value of field `enforce` after provisioning.\n"]
    pub fn enforce(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enforce", self.base))
    }

    #[doc = "Get a reference to the value of field `ports` after provisioning.\n"]
    pub fn ports(&self) -> SetRef<PrimExpr<f64>> {
        SetRef::new(self.shared().clone(), format!("{}.ports", self.base))
    }

    #[doc = "Get a reference to the value of field `validation` after provisioning.\n"]
    pub fn validation(&self) -> ListRef<DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.validation", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    tls: Option<ListField<DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsEl>>,
}

impl DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyEl {
    #[doc = "Set the field `tls`.\n"]
    pub fn set_tls(
        mut self,
        v: impl Into<ListField<DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsEl>>,
    ) -> Self {
        self.tls = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyEl {
    type O = BlockAssignable<DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyEl {}

impl BuildDataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyEl {
    pub fn build(self) -> DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyEl {
        DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyEl { tls: core::default::Default::default() }
    }
}

pub struct DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElRef {
        DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `tls` after provisioning.\n"]
    pub fn tls(&self) -> ListRef<DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tls", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualNodeSpecElBackendDefaultsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    client_policy: Option<ListField<DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyEl>>,
}

impl DataAppmeshVirtualNodeSpecElBackendDefaultsEl {
    #[doc = "Set the field `client_policy`.\n"]
    pub fn set_client_policy(
        mut self,
        v: impl Into<ListField<DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyEl>>,
    ) -> Self {
        self.client_policy = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualNodeSpecElBackendDefaultsEl {
    type O = BlockAssignable<DataAppmeshVirtualNodeSpecElBackendDefaultsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualNodeSpecElBackendDefaultsEl {}

impl BuildDataAppmeshVirtualNodeSpecElBackendDefaultsEl {
    pub fn build(self) -> DataAppmeshVirtualNodeSpecElBackendDefaultsEl {
        DataAppmeshVirtualNodeSpecElBackendDefaultsEl { client_policy: core::default::Default::default() }
    }
}

pub struct DataAppmeshVirtualNodeSpecElBackendDefaultsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualNodeSpecElBackendDefaultsElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshVirtualNodeSpecElBackendDefaultsElRef {
        DataAppmeshVirtualNodeSpecElBackendDefaultsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualNodeSpecElBackendDefaultsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `client_policy` after provisioning.\n"]
    pub fn client_policy(&self) -> ListRef<DataAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.client_policy", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualNodeSpecElListenerElConnectionPoolElGrpcEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_requests: Option<PrimField<f64>>,
}

impl DataAppmeshVirtualNodeSpecElListenerElConnectionPoolElGrpcEl {
    #[doc = "Set the field `max_requests`.\n"]
    pub fn set_max_requests(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_requests = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualNodeSpecElListenerElConnectionPoolElGrpcEl {
    type O = BlockAssignable<DataAppmeshVirtualNodeSpecElListenerElConnectionPoolElGrpcEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualNodeSpecElListenerElConnectionPoolElGrpcEl {}

impl BuildDataAppmeshVirtualNodeSpecElListenerElConnectionPoolElGrpcEl {
    pub fn build(self) -> DataAppmeshVirtualNodeSpecElListenerElConnectionPoolElGrpcEl {
        DataAppmeshVirtualNodeSpecElListenerElConnectionPoolElGrpcEl {
            max_requests: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualNodeSpecElListenerElConnectionPoolElGrpcElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualNodeSpecElListenerElConnectionPoolElGrpcElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshVirtualNodeSpecElListenerElConnectionPoolElGrpcElRef {
        DataAppmeshVirtualNodeSpecElListenerElConnectionPoolElGrpcElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualNodeSpecElListenerElConnectionPoolElGrpcElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_requests` after provisioning.\n"]
    pub fn max_requests(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_requests", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualNodeSpecElListenerElConnectionPoolElHttpEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_connections: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_pending_requests: Option<PrimField<f64>>,
}

impl DataAppmeshVirtualNodeSpecElListenerElConnectionPoolElHttpEl {
    #[doc = "Set the field `max_connections`.\n"]
    pub fn set_max_connections(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_connections = Some(v.into());
        self
    }

    #[doc = "Set the field `max_pending_requests`.\n"]
    pub fn set_max_pending_requests(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_pending_requests = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualNodeSpecElListenerElConnectionPoolElHttpEl {
    type O = BlockAssignable<DataAppmeshVirtualNodeSpecElListenerElConnectionPoolElHttpEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualNodeSpecElListenerElConnectionPoolElHttpEl {}

impl BuildDataAppmeshVirtualNodeSpecElListenerElConnectionPoolElHttpEl {
    pub fn build(self) -> DataAppmeshVirtualNodeSpecElListenerElConnectionPoolElHttpEl {
        DataAppmeshVirtualNodeSpecElListenerElConnectionPoolElHttpEl {
            max_connections: core::default::Default::default(),
            max_pending_requests: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualNodeSpecElListenerElConnectionPoolElHttpElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualNodeSpecElListenerElConnectionPoolElHttpElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshVirtualNodeSpecElListenerElConnectionPoolElHttpElRef {
        DataAppmeshVirtualNodeSpecElListenerElConnectionPoolElHttpElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualNodeSpecElListenerElConnectionPoolElHttpElRef {
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
pub struct DataAppmeshVirtualNodeSpecElListenerElConnectionPoolElHttp2El {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_requests: Option<PrimField<f64>>,
}

impl DataAppmeshVirtualNodeSpecElListenerElConnectionPoolElHttp2El {
    #[doc = "Set the field `max_requests`.\n"]
    pub fn set_max_requests(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_requests = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualNodeSpecElListenerElConnectionPoolElHttp2El {
    type O = BlockAssignable<DataAppmeshVirtualNodeSpecElListenerElConnectionPoolElHttp2El>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualNodeSpecElListenerElConnectionPoolElHttp2El {}

impl BuildDataAppmeshVirtualNodeSpecElListenerElConnectionPoolElHttp2El {
    pub fn build(self) -> DataAppmeshVirtualNodeSpecElListenerElConnectionPoolElHttp2El {
        DataAppmeshVirtualNodeSpecElListenerElConnectionPoolElHttp2El {
            max_requests: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualNodeSpecElListenerElConnectionPoolElHttp2ElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualNodeSpecElListenerElConnectionPoolElHttp2ElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshVirtualNodeSpecElListenerElConnectionPoolElHttp2ElRef {
        DataAppmeshVirtualNodeSpecElListenerElConnectionPoolElHttp2ElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualNodeSpecElListenerElConnectionPoolElHttp2ElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_requests` after provisioning.\n"]
    pub fn max_requests(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_requests", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualNodeSpecElListenerElConnectionPoolElTcpEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_connections: Option<PrimField<f64>>,
}

impl DataAppmeshVirtualNodeSpecElListenerElConnectionPoolElTcpEl {
    #[doc = "Set the field `max_connections`.\n"]
    pub fn set_max_connections(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_connections = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualNodeSpecElListenerElConnectionPoolElTcpEl {
    type O = BlockAssignable<DataAppmeshVirtualNodeSpecElListenerElConnectionPoolElTcpEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualNodeSpecElListenerElConnectionPoolElTcpEl {}

impl BuildDataAppmeshVirtualNodeSpecElListenerElConnectionPoolElTcpEl {
    pub fn build(self) -> DataAppmeshVirtualNodeSpecElListenerElConnectionPoolElTcpEl {
        DataAppmeshVirtualNodeSpecElListenerElConnectionPoolElTcpEl {
            max_connections: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualNodeSpecElListenerElConnectionPoolElTcpElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualNodeSpecElListenerElConnectionPoolElTcpElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshVirtualNodeSpecElListenerElConnectionPoolElTcpElRef {
        DataAppmeshVirtualNodeSpecElListenerElConnectionPoolElTcpElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualNodeSpecElListenerElConnectionPoolElTcpElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_connections` after provisioning.\n"]
    pub fn max_connections(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_connections", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualNodeSpecElListenerElConnectionPoolEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    grpc: Option<ListField<DataAppmeshVirtualNodeSpecElListenerElConnectionPoolElGrpcEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http: Option<ListField<DataAppmeshVirtualNodeSpecElListenerElConnectionPoolElHttpEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http2: Option<ListField<DataAppmeshVirtualNodeSpecElListenerElConnectionPoolElHttp2El>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tcp: Option<ListField<DataAppmeshVirtualNodeSpecElListenerElConnectionPoolElTcpEl>>,
}

impl DataAppmeshVirtualNodeSpecElListenerElConnectionPoolEl {
    #[doc = "Set the field `grpc`.\n"]
    pub fn set_grpc(
        mut self,
        v: impl Into<ListField<DataAppmeshVirtualNodeSpecElListenerElConnectionPoolElGrpcEl>>,
    ) -> Self {
        self.grpc = Some(v.into());
        self
    }

    #[doc = "Set the field `http`.\n"]
    pub fn set_http(
        mut self,
        v: impl Into<ListField<DataAppmeshVirtualNodeSpecElListenerElConnectionPoolElHttpEl>>,
    ) -> Self {
        self.http = Some(v.into());
        self
    }

    #[doc = "Set the field `http2`.\n"]
    pub fn set_http2(
        mut self,
        v: impl Into<ListField<DataAppmeshVirtualNodeSpecElListenerElConnectionPoolElHttp2El>>,
    ) -> Self {
        self.http2 = Some(v.into());
        self
    }

    #[doc = "Set the field `tcp`.\n"]
    pub fn set_tcp(
        mut self,
        v: impl Into<ListField<DataAppmeshVirtualNodeSpecElListenerElConnectionPoolElTcpEl>>,
    ) -> Self {
        self.tcp = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualNodeSpecElListenerElConnectionPoolEl {
    type O = BlockAssignable<DataAppmeshVirtualNodeSpecElListenerElConnectionPoolEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualNodeSpecElListenerElConnectionPoolEl {}

impl BuildDataAppmeshVirtualNodeSpecElListenerElConnectionPoolEl {
    pub fn build(self) -> DataAppmeshVirtualNodeSpecElListenerElConnectionPoolEl {
        DataAppmeshVirtualNodeSpecElListenerElConnectionPoolEl {
            grpc: core::default::Default::default(),
            http: core::default::Default::default(),
            http2: core::default::Default::default(),
            tcp: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualNodeSpecElListenerElConnectionPoolElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualNodeSpecElListenerElConnectionPoolElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshVirtualNodeSpecElListenerElConnectionPoolElRef {
        DataAppmeshVirtualNodeSpecElListenerElConnectionPoolElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualNodeSpecElListenerElConnectionPoolElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `grpc` after provisioning.\n"]
    pub fn grpc(&self) -> ListRef<DataAppmeshVirtualNodeSpecElListenerElConnectionPoolElGrpcElRef> {
        ListRef::new(self.shared().clone(), format!("{}.grpc", self.base))
    }

    #[doc = "Get a reference to the value of field `http` after provisioning.\n"]
    pub fn http(&self) -> ListRef<DataAppmeshVirtualNodeSpecElListenerElConnectionPoolElHttpElRef> {
        ListRef::new(self.shared().clone(), format!("{}.http", self.base))
    }

    #[doc = "Get a reference to the value of field `http2` after provisioning.\n"]
    pub fn http2(&self) -> ListRef<DataAppmeshVirtualNodeSpecElListenerElConnectionPoolElHttp2ElRef> {
        ListRef::new(self.shared().clone(), format!("{}.http2", self.base))
    }

    #[doc = "Get a reference to the value of field `tcp` after provisioning.\n"]
    pub fn tcp(&self) -> ListRef<DataAppmeshVirtualNodeSpecElListenerElConnectionPoolElTcpElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tcp", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualNodeSpecElListenerElHealthCheckEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    healthy_threshold: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    interval_millis: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protocol: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout_millis: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unhealthy_threshold: Option<PrimField<f64>>,
}

impl DataAppmeshVirtualNodeSpecElListenerElHealthCheckEl {
    #[doc = "Set the field `healthy_threshold`.\n"]
    pub fn set_healthy_threshold(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.healthy_threshold = Some(v.into());
        self
    }

    #[doc = "Set the field `interval_millis`.\n"]
    pub fn set_interval_millis(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.interval_millis = Some(v.into());
        self
    }

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

    #[doc = "Set the field `protocol`.\n"]
    pub fn set_protocol(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.protocol = Some(v.into());
        self
    }

    #[doc = "Set the field `timeout_millis`.\n"]
    pub fn set_timeout_millis(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.timeout_millis = Some(v.into());
        self
    }

    #[doc = "Set the field `unhealthy_threshold`.\n"]
    pub fn set_unhealthy_threshold(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.unhealthy_threshold = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualNodeSpecElListenerElHealthCheckEl {
    type O = BlockAssignable<DataAppmeshVirtualNodeSpecElListenerElHealthCheckEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualNodeSpecElListenerElHealthCheckEl {}

impl BuildDataAppmeshVirtualNodeSpecElListenerElHealthCheckEl {
    pub fn build(self) -> DataAppmeshVirtualNodeSpecElListenerElHealthCheckEl {
        DataAppmeshVirtualNodeSpecElListenerElHealthCheckEl {
            healthy_threshold: core::default::Default::default(),
            interval_millis: core::default::Default::default(),
            path: core::default::Default::default(),
            port: core::default::Default::default(),
            protocol: core::default::Default::default(),
            timeout_millis: core::default::Default::default(),
            unhealthy_threshold: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualNodeSpecElListenerElHealthCheckElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualNodeSpecElListenerElHealthCheckElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshVirtualNodeSpecElListenerElHealthCheckElRef {
        DataAppmeshVirtualNodeSpecElListenerElHealthCheckElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualNodeSpecElListenerElHealthCheckElRef {
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
pub struct DataAppmeshVirtualNodeSpecElListenerElOutlierDetectionElBaseEjectionDurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    unit: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<f64>>,
}

impl DataAppmeshVirtualNodeSpecElListenerElOutlierDetectionElBaseEjectionDurationEl {
    #[doc = "Set the field `unit`.\n"]
    pub fn set_unit(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.unit = Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.value = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualNodeSpecElListenerElOutlierDetectionElBaseEjectionDurationEl {
    type O = BlockAssignable<DataAppmeshVirtualNodeSpecElListenerElOutlierDetectionElBaseEjectionDurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualNodeSpecElListenerElOutlierDetectionElBaseEjectionDurationEl {}

impl BuildDataAppmeshVirtualNodeSpecElListenerElOutlierDetectionElBaseEjectionDurationEl {
    pub fn build(self) -> DataAppmeshVirtualNodeSpecElListenerElOutlierDetectionElBaseEjectionDurationEl {
        DataAppmeshVirtualNodeSpecElListenerElOutlierDetectionElBaseEjectionDurationEl {
            unit: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualNodeSpecElListenerElOutlierDetectionElBaseEjectionDurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualNodeSpecElListenerElOutlierDetectionElBaseEjectionDurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAppmeshVirtualNodeSpecElListenerElOutlierDetectionElBaseEjectionDurationElRef {
        DataAppmeshVirtualNodeSpecElListenerElOutlierDetectionElBaseEjectionDurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualNodeSpecElListenerElOutlierDetectionElBaseEjectionDurationElRef {
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
pub struct DataAppmeshVirtualNodeSpecElListenerElOutlierDetectionElIntervalEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    unit: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<f64>>,
}

impl DataAppmeshVirtualNodeSpecElListenerElOutlierDetectionElIntervalEl {
    #[doc = "Set the field `unit`.\n"]
    pub fn set_unit(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.unit = Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.value = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualNodeSpecElListenerElOutlierDetectionElIntervalEl {
    type O = BlockAssignable<DataAppmeshVirtualNodeSpecElListenerElOutlierDetectionElIntervalEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualNodeSpecElListenerElOutlierDetectionElIntervalEl {}

impl BuildDataAppmeshVirtualNodeSpecElListenerElOutlierDetectionElIntervalEl {
    pub fn build(self) -> DataAppmeshVirtualNodeSpecElListenerElOutlierDetectionElIntervalEl {
        DataAppmeshVirtualNodeSpecElListenerElOutlierDetectionElIntervalEl {
            unit: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualNodeSpecElListenerElOutlierDetectionElIntervalElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualNodeSpecElListenerElOutlierDetectionElIntervalElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAppmeshVirtualNodeSpecElListenerElOutlierDetectionElIntervalElRef {
        DataAppmeshVirtualNodeSpecElListenerElOutlierDetectionElIntervalElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualNodeSpecElListenerElOutlierDetectionElIntervalElRef {
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
pub struct DataAppmeshVirtualNodeSpecElListenerElOutlierDetectionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    base_ejection_duration: Option<
        ListField<DataAppmeshVirtualNodeSpecElListenerElOutlierDetectionElBaseEjectionDurationEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    interval: Option<ListField<DataAppmeshVirtualNodeSpecElListenerElOutlierDetectionElIntervalEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_ejection_percent: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_server_errors: Option<PrimField<f64>>,
}

impl DataAppmeshVirtualNodeSpecElListenerElOutlierDetectionEl {
    #[doc = "Set the field `base_ejection_duration`.\n"]
    pub fn set_base_ejection_duration(
        mut self,
        v: impl Into<ListField<DataAppmeshVirtualNodeSpecElListenerElOutlierDetectionElBaseEjectionDurationEl>>,
    ) -> Self {
        self.base_ejection_duration = Some(v.into());
        self
    }

    #[doc = "Set the field `interval`.\n"]
    pub fn set_interval(
        mut self,
        v: impl Into<ListField<DataAppmeshVirtualNodeSpecElListenerElOutlierDetectionElIntervalEl>>,
    ) -> Self {
        self.interval = Some(v.into());
        self
    }

    #[doc = "Set the field `max_ejection_percent`.\n"]
    pub fn set_max_ejection_percent(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_ejection_percent = Some(v.into());
        self
    }

    #[doc = "Set the field `max_server_errors`.\n"]
    pub fn set_max_server_errors(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_server_errors = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualNodeSpecElListenerElOutlierDetectionEl {
    type O = BlockAssignable<DataAppmeshVirtualNodeSpecElListenerElOutlierDetectionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualNodeSpecElListenerElOutlierDetectionEl {}

impl BuildDataAppmeshVirtualNodeSpecElListenerElOutlierDetectionEl {
    pub fn build(self) -> DataAppmeshVirtualNodeSpecElListenerElOutlierDetectionEl {
        DataAppmeshVirtualNodeSpecElListenerElOutlierDetectionEl {
            base_ejection_duration: core::default::Default::default(),
            interval: core::default::Default::default(),
            max_ejection_percent: core::default::Default::default(),
            max_server_errors: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualNodeSpecElListenerElOutlierDetectionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualNodeSpecElListenerElOutlierDetectionElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshVirtualNodeSpecElListenerElOutlierDetectionElRef {
        DataAppmeshVirtualNodeSpecElListenerElOutlierDetectionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualNodeSpecElListenerElOutlierDetectionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `base_ejection_duration` after provisioning.\n"]
    pub fn base_ejection_duration(
        &self,
    ) -> ListRef<DataAppmeshVirtualNodeSpecElListenerElOutlierDetectionElBaseEjectionDurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.base_ejection_duration", self.base))
    }

    #[doc = "Get a reference to the value of field `interval` after provisioning.\n"]
    pub fn interval(&self) -> ListRef<DataAppmeshVirtualNodeSpecElListenerElOutlierDetectionElIntervalElRef> {
        ListRef::new(self.shared().clone(), format!("{}.interval", self.base))
    }

    #[doc = "Get a reference to the value of field `max_ejection_percent` after provisioning.\n"]
    pub fn max_ejection_percent(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_ejection_percent", self.base))
    }

    #[doc = "Get a reference to the value of field `max_server_errors` after provisioning.\n"]
    pub fn max_server_errors(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_server_errors", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualNodeSpecElListenerElPortMappingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protocol: Option<PrimField<String>>,
}

impl DataAppmeshVirtualNodeSpecElListenerElPortMappingEl {
    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }

    #[doc = "Set the field `protocol`.\n"]
    pub fn set_protocol(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.protocol = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualNodeSpecElListenerElPortMappingEl {
    type O = BlockAssignable<DataAppmeshVirtualNodeSpecElListenerElPortMappingEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualNodeSpecElListenerElPortMappingEl {}

impl BuildDataAppmeshVirtualNodeSpecElListenerElPortMappingEl {
    pub fn build(self) -> DataAppmeshVirtualNodeSpecElListenerElPortMappingEl {
        DataAppmeshVirtualNodeSpecElListenerElPortMappingEl {
            port: core::default::Default::default(),
            protocol: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualNodeSpecElListenerElPortMappingElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualNodeSpecElListenerElPortMappingElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshVirtualNodeSpecElListenerElPortMappingElRef {
        DataAppmeshVirtualNodeSpecElListenerElPortMappingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualNodeSpecElListenerElPortMappingElRef {
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
pub struct DataAppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElIdleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    unit: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<f64>>,
}

impl DataAppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElIdleEl {
    #[doc = "Set the field `unit`.\n"]
    pub fn set_unit(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.unit = Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.value = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElIdleEl {
    type O = BlockAssignable<DataAppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElIdleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElIdleEl {}

impl BuildDataAppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElIdleEl {
    pub fn build(self) -> DataAppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElIdleEl {
        DataAppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElIdleEl {
            unit: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElIdleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElIdleElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElIdleElRef {
        DataAppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElIdleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElIdleElRef {
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
pub struct DataAppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElPerRequestEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    unit: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<f64>>,
}

impl DataAppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElPerRequestEl {
    #[doc = "Set the field `unit`.\n"]
    pub fn set_unit(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.unit = Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.value = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElPerRequestEl {
    type O = BlockAssignable<DataAppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElPerRequestEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElPerRequestEl {}

impl BuildDataAppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElPerRequestEl {
    pub fn build(self) -> DataAppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElPerRequestEl {
        DataAppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElPerRequestEl {
            unit: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElPerRequestElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElPerRequestElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElPerRequestElRef {
        DataAppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElPerRequestElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElPerRequestElRef {
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
pub struct DataAppmeshVirtualNodeSpecElListenerElTimeoutElGrpcEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    idle: Option<ListField<DataAppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElIdleEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_request: Option<ListField<DataAppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElPerRequestEl>>,
}

impl DataAppmeshVirtualNodeSpecElListenerElTimeoutElGrpcEl {
    #[doc = "Set the field `idle`.\n"]
    pub fn set_idle(
        mut self,
        v: impl Into<ListField<DataAppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElIdleEl>>,
    ) -> Self {
        self.idle = Some(v.into());
        self
    }

    #[doc = "Set the field `per_request`.\n"]
    pub fn set_per_request(
        mut self,
        v: impl Into<ListField<DataAppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElPerRequestEl>>,
    ) -> Self {
        self.per_request = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualNodeSpecElListenerElTimeoutElGrpcEl {
    type O = BlockAssignable<DataAppmeshVirtualNodeSpecElListenerElTimeoutElGrpcEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualNodeSpecElListenerElTimeoutElGrpcEl {}

impl BuildDataAppmeshVirtualNodeSpecElListenerElTimeoutElGrpcEl {
    pub fn build(self) -> DataAppmeshVirtualNodeSpecElListenerElTimeoutElGrpcEl {
        DataAppmeshVirtualNodeSpecElListenerElTimeoutElGrpcEl {
            idle: core::default::Default::default(),
            per_request: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElRef {
        DataAppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `idle` after provisioning.\n"]
    pub fn idle(&self) -> ListRef<DataAppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElIdleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.idle", self.base))
    }

    #[doc = "Get a reference to the value of field `per_request` after provisioning.\n"]
    pub fn per_request(&self) -> ListRef<DataAppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElPerRequestElRef> {
        ListRef::new(self.shared().clone(), format!("{}.per_request", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualNodeSpecElListenerElTimeoutElHttpElIdleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    unit: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<f64>>,
}

impl DataAppmeshVirtualNodeSpecElListenerElTimeoutElHttpElIdleEl {
    #[doc = "Set the field `unit`.\n"]
    pub fn set_unit(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.unit = Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.value = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualNodeSpecElListenerElTimeoutElHttpElIdleEl {
    type O = BlockAssignable<DataAppmeshVirtualNodeSpecElListenerElTimeoutElHttpElIdleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualNodeSpecElListenerElTimeoutElHttpElIdleEl {}

impl BuildDataAppmeshVirtualNodeSpecElListenerElTimeoutElHttpElIdleEl {
    pub fn build(self) -> DataAppmeshVirtualNodeSpecElListenerElTimeoutElHttpElIdleEl {
        DataAppmeshVirtualNodeSpecElListenerElTimeoutElHttpElIdleEl {
            unit: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualNodeSpecElListenerElTimeoutElHttpElIdleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualNodeSpecElListenerElTimeoutElHttpElIdleElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshVirtualNodeSpecElListenerElTimeoutElHttpElIdleElRef {
        DataAppmeshVirtualNodeSpecElListenerElTimeoutElHttpElIdleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualNodeSpecElListenerElTimeoutElHttpElIdleElRef {
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
pub struct DataAppmeshVirtualNodeSpecElListenerElTimeoutElHttpElPerRequestEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    unit: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<f64>>,
}

impl DataAppmeshVirtualNodeSpecElListenerElTimeoutElHttpElPerRequestEl {
    #[doc = "Set the field `unit`.\n"]
    pub fn set_unit(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.unit = Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.value = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualNodeSpecElListenerElTimeoutElHttpElPerRequestEl {
    type O = BlockAssignable<DataAppmeshVirtualNodeSpecElListenerElTimeoutElHttpElPerRequestEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualNodeSpecElListenerElTimeoutElHttpElPerRequestEl {}

impl BuildDataAppmeshVirtualNodeSpecElListenerElTimeoutElHttpElPerRequestEl {
    pub fn build(self) -> DataAppmeshVirtualNodeSpecElListenerElTimeoutElHttpElPerRequestEl {
        DataAppmeshVirtualNodeSpecElListenerElTimeoutElHttpElPerRequestEl {
            unit: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualNodeSpecElListenerElTimeoutElHttpElPerRequestElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualNodeSpecElListenerElTimeoutElHttpElPerRequestElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAppmeshVirtualNodeSpecElListenerElTimeoutElHttpElPerRequestElRef {
        DataAppmeshVirtualNodeSpecElListenerElTimeoutElHttpElPerRequestElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualNodeSpecElListenerElTimeoutElHttpElPerRequestElRef {
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
pub struct DataAppmeshVirtualNodeSpecElListenerElTimeoutElHttpEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    idle: Option<ListField<DataAppmeshVirtualNodeSpecElListenerElTimeoutElHttpElIdleEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_request: Option<ListField<DataAppmeshVirtualNodeSpecElListenerElTimeoutElHttpElPerRequestEl>>,
}

impl DataAppmeshVirtualNodeSpecElListenerElTimeoutElHttpEl {
    #[doc = "Set the field `idle`.\n"]
    pub fn set_idle(
        mut self,
        v: impl Into<ListField<DataAppmeshVirtualNodeSpecElListenerElTimeoutElHttpElIdleEl>>,
    ) -> Self {
        self.idle = Some(v.into());
        self
    }

    #[doc = "Set the field `per_request`.\n"]
    pub fn set_per_request(
        mut self,
        v: impl Into<ListField<DataAppmeshVirtualNodeSpecElListenerElTimeoutElHttpElPerRequestEl>>,
    ) -> Self {
        self.per_request = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualNodeSpecElListenerElTimeoutElHttpEl {
    type O = BlockAssignable<DataAppmeshVirtualNodeSpecElListenerElTimeoutElHttpEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualNodeSpecElListenerElTimeoutElHttpEl {}

impl BuildDataAppmeshVirtualNodeSpecElListenerElTimeoutElHttpEl {
    pub fn build(self) -> DataAppmeshVirtualNodeSpecElListenerElTimeoutElHttpEl {
        DataAppmeshVirtualNodeSpecElListenerElTimeoutElHttpEl {
            idle: core::default::Default::default(),
            per_request: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualNodeSpecElListenerElTimeoutElHttpElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualNodeSpecElListenerElTimeoutElHttpElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshVirtualNodeSpecElListenerElTimeoutElHttpElRef {
        DataAppmeshVirtualNodeSpecElListenerElTimeoutElHttpElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualNodeSpecElListenerElTimeoutElHttpElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `idle` after provisioning.\n"]
    pub fn idle(&self) -> ListRef<DataAppmeshVirtualNodeSpecElListenerElTimeoutElHttpElIdleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.idle", self.base))
    }

    #[doc = "Get a reference to the value of field `per_request` after provisioning.\n"]
    pub fn per_request(&self) -> ListRef<DataAppmeshVirtualNodeSpecElListenerElTimeoutElHttpElPerRequestElRef> {
        ListRef::new(self.shared().clone(), format!("{}.per_request", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElIdleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    unit: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<f64>>,
}

impl DataAppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElIdleEl {
    #[doc = "Set the field `unit`.\n"]
    pub fn set_unit(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.unit = Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.value = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElIdleEl {
    type O = BlockAssignable<DataAppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElIdleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElIdleEl {}

impl BuildDataAppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElIdleEl {
    pub fn build(self) -> DataAppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElIdleEl {
        DataAppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElIdleEl {
            unit: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElIdleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElIdleElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElIdleElRef {
        DataAppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElIdleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElIdleElRef {
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
pub struct DataAppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElPerRequestEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    unit: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<f64>>,
}

impl DataAppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElPerRequestEl {
    #[doc = "Set the field `unit`.\n"]
    pub fn set_unit(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.unit = Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.value = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElPerRequestEl {
    type O = BlockAssignable<DataAppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElPerRequestEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElPerRequestEl {}

impl BuildDataAppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElPerRequestEl {
    pub fn build(self) -> DataAppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElPerRequestEl {
        DataAppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElPerRequestEl {
            unit: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElPerRequestElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElPerRequestElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElPerRequestElRef {
        DataAppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElPerRequestElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElPerRequestElRef {
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
pub struct DataAppmeshVirtualNodeSpecElListenerElTimeoutElHttp2El {
    #[serde(skip_serializing_if = "Option::is_none")]
    idle: Option<ListField<DataAppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElIdleEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_request: Option<ListField<DataAppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElPerRequestEl>>,
}

impl DataAppmeshVirtualNodeSpecElListenerElTimeoutElHttp2El {
    #[doc = "Set the field `idle`.\n"]
    pub fn set_idle(
        mut self,
        v: impl Into<ListField<DataAppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElIdleEl>>,
    ) -> Self {
        self.idle = Some(v.into());
        self
    }

    #[doc = "Set the field `per_request`.\n"]
    pub fn set_per_request(
        mut self,
        v: impl Into<ListField<DataAppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElPerRequestEl>>,
    ) -> Self {
        self.per_request = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualNodeSpecElListenerElTimeoutElHttp2El {
    type O = BlockAssignable<DataAppmeshVirtualNodeSpecElListenerElTimeoutElHttp2El>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualNodeSpecElListenerElTimeoutElHttp2El {}

impl BuildDataAppmeshVirtualNodeSpecElListenerElTimeoutElHttp2El {
    pub fn build(self) -> DataAppmeshVirtualNodeSpecElListenerElTimeoutElHttp2El {
        DataAppmeshVirtualNodeSpecElListenerElTimeoutElHttp2El {
            idle: core::default::Default::default(),
            per_request: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElRef {
        DataAppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `idle` after provisioning.\n"]
    pub fn idle(&self) -> ListRef<DataAppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElIdleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.idle", self.base))
    }

    #[doc = "Get a reference to the value of field `per_request` after provisioning.\n"]
    pub fn per_request(&self) -> ListRef<DataAppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElPerRequestElRef> {
        ListRef::new(self.shared().clone(), format!("{}.per_request", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualNodeSpecElListenerElTimeoutElTcpElIdleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    unit: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<f64>>,
}

impl DataAppmeshVirtualNodeSpecElListenerElTimeoutElTcpElIdleEl {
    #[doc = "Set the field `unit`.\n"]
    pub fn set_unit(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.unit = Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.value = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualNodeSpecElListenerElTimeoutElTcpElIdleEl {
    type O = BlockAssignable<DataAppmeshVirtualNodeSpecElListenerElTimeoutElTcpElIdleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualNodeSpecElListenerElTimeoutElTcpElIdleEl {}

impl BuildDataAppmeshVirtualNodeSpecElListenerElTimeoutElTcpElIdleEl {
    pub fn build(self) -> DataAppmeshVirtualNodeSpecElListenerElTimeoutElTcpElIdleEl {
        DataAppmeshVirtualNodeSpecElListenerElTimeoutElTcpElIdleEl {
            unit: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualNodeSpecElListenerElTimeoutElTcpElIdleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualNodeSpecElListenerElTimeoutElTcpElIdleElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshVirtualNodeSpecElListenerElTimeoutElTcpElIdleElRef {
        DataAppmeshVirtualNodeSpecElListenerElTimeoutElTcpElIdleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualNodeSpecElListenerElTimeoutElTcpElIdleElRef {
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
pub struct DataAppmeshVirtualNodeSpecElListenerElTimeoutElTcpEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    idle: Option<ListField<DataAppmeshVirtualNodeSpecElListenerElTimeoutElTcpElIdleEl>>,
}

impl DataAppmeshVirtualNodeSpecElListenerElTimeoutElTcpEl {
    #[doc = "Set the field `idle`.\n"]
    pub fn set_idle(
        mut self,
        v: impl Into<ListField<DataAppmeshVirtualNodeSpecElListenerElTimeoutElTcpElIdleEl>>,
    ) -> Self {
        self.idle = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualNodeSpecElListenerElTimeoutElTcpEl {
    type O = BlockAssignable<DataAppmeshVirtualNodeSpecElListenerElTimeoutElTcpEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualNodeSpecElListenerElTimeoutElTcpEl {}

impl BuildDataAppmeshVirtualNodeSpecElListenerElTimeoutElTcpEl {
    pub fn build(self) -> DataAppmeshVirtualNodeSpecElListenerElTimeoutElTcpEl {
        DataAppmeshVirtualNodeSpecElListenerElTimeoutElTcpEl { idle: core::default::Default::default() }
    }
}

pub struct DataAppmeshVirtualNodeSpecElListenerElTimeoutElTcpElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualNodeSpecElListenerElTimeoutElTcpElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshVirtualNodeSpecElListenerElTimeoutElTcpElRef {
        DataAppmeshVirtualNodeSpecElListenerElTimeoutElTcpElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualNodeSpecElListenerElTimeoutElTcpElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `idle` after provisioning.\n"]
    pub fn idle(&self) -> ListRef<DataAppmeshVirtualNodeSpecElListenerElTimeoutElTcpElIdleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.idle", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualNodeSpecElListenerElTimeoutEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    grpc: Option<ListField<DataAppmeshVirtualNodeSpecElListenerElTimeoutElGrpcEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http: Option<ListField<DataAppmeshVirtualNodeSpecElListenerElTimeoutElHttpEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http2: Option<ListField<DataAppmeshVirtualNodeSpecElListenerElTimeoutElHttp2El>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tcp: Option<ListField<DataAppmeshVirtualNodeSpecElListenerElTimeoutElTcpEl>>,
}

impl DataAppmeshVirtualNodeSpecElListenerElTimeoutEl {
    #[doc = "Set the field `grpc`.\n"]
    pub fn set_grpc(mut self, v: impl Into<ListField<DataAppmeshVirtualNodeSpecElListenerElTimeoutElGrpcEl>>) -> Self {
        self.grpc = Some(v.into());
        self
    }

    #[doc = "Set the field `http`.\n"]
    pub fn set_http(mut self, v: impl Into<ListField<DataAppmeshVirtualNodeSpecElListenerElTimeoutElHttpEl>>) -> Self {
        self.http = Some(v.into());
        self
    }

    #[doc = "Set the field `http2`.\n"]
    pub fn set_http2(mut self, v: impl Into<ListField<DataAppmeshVirtualNodeSpecElListenerElTimeoutElHttp2El>>) -> Self {
        self.http2 = Some(v.into());
        self
    }

    #[doc = "Set the field `tcp`.\n"]
    pub fn set_tcp(mut self, v: impl Into<ListField<DataAppmeshVirtualNodeSpecElListenerElTimeoutElTcpEl>>) -> Self {
        self.tcp = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualNodeSpecElListenerElTimeoutEl {
    type O = BlockAssignable<DataAppmeshVirtualNodeSpecElListenerElTimeoutEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualNodeSpecElListenerElTimeoutEl {}

impl BuildDataAppmeshVirtualNodeSpecElListenerElTimeoutEl {
    pub fn build(self) -> DataAppmeshVirtualNodeSpecElListenerElTimeoutEl {
        DataAppmeshVirtualNodeSpecElListenerElTimeoutEl {
            grpc: core::default::Default::default(),
            http: core::default::Default::default(),
            http2: core::default::Default::default(),
            tcp: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualNodeSpecElListenerElTimeoutElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualNodeSpecElListenerElTimeoutElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshVirtualNodeSpecElListenerElTimeoutElRef {
        DataAppmeshVirtualNodeSpecElListenerElTimeoutElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualNodeSpecElListenerElTimeoutElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `grpc` after provisioning.\n"]
    pub fn grpc(&self) -> ListRef<DataAppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElRef> {
        ListRef::new(self.shared().clone(), format!("{}.grpc", self.base))
    }

    #[doc = "Get a reference to the value of field `http` after provisioning.\n"]
    pub fn http(&self) -> ListRef<DataAppmeshVirtualNodeSpecElListenerElTimeoutElHttpElRef> {
        ListRef::new(self.shared().clone(), format!("{}.http", self.base))
    }

    #[doc = "Get a reference to the value of field `http2` after provisioning.\n"]
    pub fn http2(&self) -> ListRef<DataAppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElRef> {
        ListRef::new(self.shared().clone(), format!("{}.http2", self.base))
    }

    #[doc = "Get a reference to the value of field `tcp` after provisioning.\n"]
    pub fn tcp(&self) -> ListRef<DataAppmeshVirtualNodeSpecElListenerElTimeoutElTcpElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tcp", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualNodeSpecElListenerElTlsElCertificateElAcmEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate_arn: Option<PrimField<String>>,
}

impl DataAppmeshVirtualNodeSpecElListenerElTlsElCertificateElAcmEl {
    #[doc = "Set the field `certificate_arn`.\n"]
    pub fn set_certificate_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.certificate_arn = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualNodeSpecElListenerElTlsElCertificateElAcmEl {
    type O = BlockAssignable<DataAppmeshVirtualNodeSpecElListenerElTlsElCertificateElAcmEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualNodeSpecElListenerElTlsElCertificateElAcmEl {}

impl BuildDataAppmeshVirtualNodeSpecElListenerElTlsElCertificateElAcmEl {
    pub fn build(self) -> DataAppmeshVirtualNodeSpecElListenerElTlsElCertificateElAcmEl {
        DataAppmeshVirtualNodeSpecElListenerElTlsElCertificateElAcmEl {
            certificate_arn: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualNodeSpecElListenerElTlsElCertificateElAcmElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualNodeSpecElListenerElTlsElCertificateElAcmElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshVirtualNodeSpecElListenerElTlsElCertificateElAcmElRef {
        DataAppmeshVirtualNodeSpecElListenerElTlsElCertificateElAcmElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualNodeSpecElListenerElTlsElCertificateElAcmElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificate_arn` after provisioning.\n"]
    pub fn certificate_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.certificate_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualNodeSpecElListenerElTlsElCertificateElFileEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate_chain: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    private_key: Option<PrimField<String>>,
}

impl DataAppmeshVirtualNodeSpecElListenerElTlsElCertificateElFileEl {
    #[doc = "Set the field `certificate_chain`.\n"]
    pub fn set_certificate_chain(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.certificate_chain = Some(v.into());
        self
    }

    #[doc = "Set the field `private_key`.\n"]
    pub fn set_private_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.private_key = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualNodeSpecElListenerElTlsElCertificateElFileEl {
    type O = BlockAssignable<DataAppmeshVirtualNodeSpecElListenerElTlsElCertificateElFileEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualNodeSpecElListenerElTlsElCertificateElFileEl {}

impl BuildDataAppmeshVirtualNodeSpecElListenerElTlsElCertificateElFileEl {
    pub fn build(self) -> DataAppmeshVirtualNodeSpecElListenerElTlsElCertificateElFileEl {
        DataAppmeshVirtualNodeSpecElListenerElTlsElCertificateElFileEl {
            certificate_chain: core::default::Default::default(),
            private_key: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualNodeSpecElListenerElTlsElCertificateElFileElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualNodeSpecElListenerElTlsElCertificateElFileElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshVirtualNodeSpecElListenerElTlsElCertificateElFileElRef {
        DataAppmeshVirtualNodeSpecElListenerElTlsElCertificateElFileElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualNodeSpecElListenerElTlsElCertificateElFileElRef {
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
pub struct DataAppmeshVirtualNodeSpecElListenerElTlsElCertificateElSdsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_name: Option<PrimField<String>>,
}

impl DataAppmeshVirtualNodeSpecElListenerElTlsElCertificateElSdsEl {
    #[doc = "Set the field `secret_name`.\n"]
    pub fn set_secret_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.secret_name = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualNodeSpecElListenerElTlsElCertificateElSdsEl {
    type O = BlockAssignable<DataAppmeshVirtualNodeSpecElListenerElTlsElCertificateElSdsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualNodeSpecElListenerElTlsElCertificateElSdsEl {}

impl BuildDataAppmeshVirtualNodeSpecElListenerElTlsElCertificateElSdsEl {
    pub fn build(self) -> DataAppmeshVirtualNodeSpecElListenerElTlsElCertificateElSdsEl {
        DataAppmeshVirtualNodeSpecElListenerElTlsElCertificateElSdsEl {
            secret_name: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualNodeSpecElListenerElTlsElCertificateElSdsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualNodeSpecElListenerElTlsElCertificateElSdsElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshVirtualNodeSpecElListenerElTlsElCertificateElSdsElRef {
        DataAppmeshVirtualNodeSpecElListenerElTlsElCertificateElSdsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualNodeSpecElListenerElTlsElCertificateElSdsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `secret_name` after provisioning.\n"]
    pub fn secret_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualNodeSpecElListenerElTlsElCertificateEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    acm: Option<ListField<DataAppmeshVirtualNodeSpecElListenerElTlsElCertificateElAcmEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file: Option<ListField<DataAppmeshVirtualNodeSpecElListenerElTlsElCertificateElFileEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sds: Option<ListField<DataAppmeshVirtualNodeSpecElListenerElTlsElCertificateElSdsEl>>,
}

impl DataAppmeshVirtualNodeSpecElListenerElTlsElCertificateEl {
    #[doc = "Set the field `acm`.\n"]
    pub fn set_acm(
        mut self,
        v: impl Into<ListField<DataAppmeshVirtualNodeSpecElListenerElTlsElCertificateElAcmEl>>,
    ) -> Self {
        self.acm = Some(v.into());
        self
    }

    #[doc = "Set the field `file`.\n"]
    pub fn set_file(
        mut self,
        v: impl Into<ListField<DataAppmeshVirtualNodeSpecElListenerElTlsElCertificateElFileEl>>,
    ) -> Self {
        self.file = Some(v.into());
        self
    }

    #[doc = "Set the field `sds`.\n"]
    pub fn set_sds(
        mut self,
        v: impl Into<ListField<DataAppmeshVirtualNodeSpecElListenerElTlsElCertificateElSdsEl>>,
    ) -> Self {
        self.sds = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualNodeSpecElListenerElTlsElCertificateEl {
    type O = BlockAssignable<DataAppmeshVirtualNodeSpecElListenerElTlsElCertificateEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualNodeSpecElListenerElTlsElCertificateEl {}

impl BuildDataAppmeshVirtualNodeSpecElListenerElTlsElCertificateEl {
    pub fn build(self) -> DataAppmeshVirtualNodeSpecElListenerElTlsElCertificateEl {
        DataAppmeshVirtualNodeSpecElListenerElTlsElCertificateEl {
            acm: core::default::Default::default(),
            file: core::default::Default::default(),
            sds: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualNodeSpecElListenerElTlsElCertificateElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualNodeSpecElListenerElTlsElCertificateElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshVirtualNodeSpecElListenerElTlsElCertificateElRef {
        DataAppmeshVirtualNodeSpecElListenerElTlsElCertificateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualNodeSpecElListenerElTlsElCertificateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `acm` after provisioning.\n"]
    pub fn acm(&self) -> ListRef<DataAppmeshVirtualNodeSpecElListenerElTlsElCertificateElAcmElRef> {
        ListRef::new(self.shared().clone(), format!("{}.acm", self.base))
    }

    #[doc = "Get a reference to the value of field `file` after provisioning.\n"]
    pub fn file(&self) -> ListRef<DataAppmeshVirtualNodeSpecElListenerElTlsElCertificateElFileElRef> {
        ListRef::new(self.shared().clone(), format!("{}.file", self.base))
    }

    #[doc = "Get a reference to the value of field `sds` after provisioning.\n"]
    pub fn sds(&self) -> ListRef<DataAppmeshVirtualNodeSpecElListenerElTlsElCertificateElSdsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sds", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    exact: Option<SetField<PrimField<String>>>,
}

impl DataAppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchEl {
    #[doc = "Set the field `exact`.\n"]
    pub fn set_exact(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.exact = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchEl {
    type O =
        BlockAssignable<DataAppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchEl {}

impl BuildDataAppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchEl {
    pub fn build(self) -> DataAppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchEl {
        DataAppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchEl {
            exact: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchElRef {
        DataAppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `exact` after provisioning.\n"]
    pub fn exact(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.exact", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesEl {
    #[serde(rename = "match", skip_serializing_if = "Option::is_none")]
    match_: Option<ListField<DataAppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchEl>>,
}

impl DataAppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesEl {
    #[doc = "Set the field `match_`.\n"]
    pub fn set_match(
        mut self,
        v:
            impl

                    Into<
                        ListField<
                            DataAppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchEl,
                        >,
                    >,
    ) -> Self {
        self.match_ = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesEl {
    type O = BlockAssignable<DataAppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesEl {}

impl BuildDataAppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesEl {
    pub fn build(self) -> DataAppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesEl {
        DataAppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesEl {
            match_: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesElRef {
        DataAppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(
        &self,
    ) -> ListRef<DataAppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchElRef> {
        ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElFileEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate_chain: Option<PrimField<String>>,
}

impl DataAppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElFileEl {
    #[doc = "Set the field `certificate_chain`.\n"]
    pub fn set_certificate_chain(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.certificate_chain = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElFileEl {
    type O = BlockAssignable<DataAppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElFileEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElFileEl {}

impl BuildDataAppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElFileEl {
    pub fn build(self) -> DataAppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElFileEl {
        DataAppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElFileEl {
            certificate_chain: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElFileElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElFileElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElFileElRef {
        DataAppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElFileElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElFileElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificate_chain` after provisioning.\n"]
    pub fn certificate_chain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.certificate_chain", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElSdsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_name: Option<PrimField<String>>,
}

impl DataAppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElSdsEl {
    #[doc = "Set the field `secret_name`.\n"]
    pub fn set_secret_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.secret_name = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElSdsEl {
    type O = BlockAssignable<DataAppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElSdsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElSdsEl {}

impl BuildDataAppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElSdsEl {
    pub fn build(self) -> DataAppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElSdsEl {
        DataAppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElSdsEl {
            secret_name: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElSdsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElSdsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElSdsElRef {
        DataAppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElSdsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElSdsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `secret_name` after provisioning.\n"]
    pub fn secret_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    file: Option<ListField<DataAppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElFileEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sds: Option<ListField<DataAppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElSdsEl>>,
}

impl DataAppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustEl {
    #[doc = "Set the field `file`.\n"]
    pub fn set_file(
        mut self,
        v: impl Into<ListField<DataAppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElFileEl>>,
    ) -> Self {
        self.file = Some(v.into());
        self
    }

    #[doc = "Set the field `sds`.\n"]
    pub fn set_sds(
        mut self,
        v: impl Into<ListField<DataAppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElSdsEl>>,
    ) -> Self {
        self.sds = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustEl {
    type O = BlockAssignable<DataAppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustEl {}

impl BuildDataAppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustEl {
    pub fn build(self) -> DataAppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustEl {
        DataAppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustEl {
            file: core::default::Default::default(),
            sds: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElRef {
        DataAppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `file` after provisioning.\n"]
    pub fn file(&self) -> ListRef<DataAppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElFileElRef> {
        ListRef::new(self.shared().clone(), format!("{}.file", self.base))
    }

    #[doc = "Get a reference to the value of field `sds` after provisioning.\n"]
    pub fn sds(&self) -> ListRef<DataAppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElSdsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sds", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualNodeSpecElListenerElTlsElValidationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    subject_alternative_names: Option<
        ListField<DataAppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    trust: Option<ListField<DataAppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustEl>>,
}

impl DataAppmeshVirtualNodeSpecElListenerElTlsElValidationEl {
    #[doc = "Set the field `subject_alternative_names`.\n"]
    pub fn set_subject_alternative_names(
        mut self,
        v: impl Into<ListField<DataAppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesEl>>,
    ) -> Self {
        self.subject_alternative_names = Some(v.into());
        self
    }

    #[doc = "Set the field `trust`.\n"]
    pub fn set_trust(
        mut self,
        v: impl Into<ListField<DataAppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustEl>>,
    ) -> Self {
        self.trust = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualNodeSpecElListenerElTlsElValidationEl {
    type O = BlockAssignable<DataAppmeshVirtualNodeSpecElListenerElTlsElValidationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualNodeSpecElListenerElTlsElValidationEl {}

impl BuildDataAppmeshVirtualNodeSpecElListenerElTlsElValidationEl {
    pub fn build(self) -> DataAppmeshVirtualNodeSpecElListenerElTlsElValidationEl {
        DataAppmeshVirtualNodeSpecElListenerElTlsElValidationEl {
            subject_alternative_names: core::default::Default::default(),
            trust: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualNodeSpecElListenerElTlsElValidationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualNodeSpecElListenerElTlsElValidationElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshVirtualNodeSpecElListenerElTlsElValidationElRef {
        DataAppmeshVirtualNodeSpecElListenerElTlsElValidationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualNodeSpecElListenerElTlsElValidationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `subject_alternative_names` after provisioning.\n"]
    pub fn subject_alternative_names(
        &self,
    ) -> ListRef<DataAppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.subject_alternative_names", self.base))
    }

    #[doc = "Get a reference to the value of field `trust` after provisioning.\n"]
    pub fn trust(&self) -> ListRef<DataAppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElRef> {
        ListRef::new(self.shared().clone(), format!("{}.trust", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualNodeSpecElListenerElTlsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate: Option<ListField<DataAppmeshVirtualNodeSpecElListenerElTlsElCertificateEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    validation: Option<ListField<DataAppmeshVirtualNodeSpecElListenerElTlsElValidationEl>>,
}

impl DataAppmeshVirtualNodeSpecElListenerElTlsEl {
    #[doc = "Set the field `certificate`.\n"]
    pub fn set_certificate(
        mut self,
        v: impl Into<ListField<DataAppmeshVirtualNodeSpecElListenerElTlsElCertificateEl>>,
    ) -> Self {
        self.certificate = Some(v.into());
        self
    }

    #[doc = "Set the field `mode`.\n"]
    pub fn set_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mode = Some(v.into());
        self
    }

    #[doc = "Set the field `validation`.\n"]
    pub fn set_validation(
        mut self,
        v: impl Into<ListField<DataAppmeshVirtualNodeSpecElListenerElTlsElValidationEl>>,
    ) -> Self {
        self.validation = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualNodeSpecElListenerElTlsEl {
    type O = BlockAssignable<DataAppmeshVirtualNodeSpecElListenerElTlsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualNodeSpecElListenerElTlsEl {}

impl BuildDataAppmeshVirtualNodeSpecElListenerElTlsEl {
    pub fn build(self) -> DataAppmeshVirtualNodeSpecElListenerElTlsEl {
        DataAppmeshVirtualNodeSpecElListenerElTlsEl {
            certificate: core::default::Default::default(),
            mode: core::default::Default::default(),
            validation: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualNodeSpecElListenerElTlsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualNodeSpecElListenerElTlsElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshVirtualNodeSpecElListenerElTlsElRef {
        DataAppmeshVirtualNodeSpecElListenerElTlsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualNodeSpecElListenerElTlsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificate` after provisioning.\n"]
    pub fn certificate(&self) -> ListRef<DataAppmeshVirtualNodeSpecElListenerElTlsElCertificateElRef> {
        ListRef::new(self.shared().clone(), format!("{}.certificate", self.base))
    }

    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc = "Get a reference to the value of field `validation` after provisioning.\n"]
    pub fn validation(&self) -> ListRef<DataAppmeshVirtualNodeSpecElListenerElTlsElValidationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.validation", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualNodeSpecElListenerEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    connection_pool: Option<ListField<DataAppmeshVirtualNodeSpecElListenerElConnectionPoolEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    health_check: Option<ListField<DataAppmeshVirtualNodeSpecElListenerElHealthCheckEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    outlier_detection: Option<ListField<DataAppmeshVirtualNodeSpecElListenerElOutlierDetectionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port_mapping: Option<ListField<DataAppmeshVirtualNodeSpecElListenerElPortMappingEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout: Option<ListField<DataAppmeshVirtualNodeSpecElListenerElTimeoutEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tls: Option<ListField<DataAppmeshVirtualNodeSpecElListenerElTlsEl>>,
}

impl DataAppmeshVirtualNodeSpecElListenerEl {
    #[doc = "Set the field `connection_pool`.\n"]
    pub fn set_connection_pool(
        mut self,
        v: impl Into<ListField<DataAppmeshVirtualNodeSpecElListenerElConnectionPoolEl>>,
    ) -> Self {
        self.connection_pool = Some(v.into());
        self
    }

    #[doc = "Set the field `health_check`.\n"]
    pub fn set_health_check(
        mut self,
        v: impl Into<ListField<DataAppmeshVirtualNodeSpecElListenerElHealthCheckEl>>,
    ) -> Self {
        self.health_check = Some(v.into());
        self
    }

    #[doc = "Set the field `outlier_detection`.\n"]
    pub fn set_outlier_detection(
        mut self,
        v: impl Into<ListField<DataAppmeshVirtualNodeSpecElListenerElOutlierDetectionEl>>,
    ) -> Self {
        self.outlier_detection = Some(v.into());
        self
    }

    #[doc = "Set the field `port_mapping`.\n"]
    pub fn set_port_mapping(
        mut self,
        v: impl Into<ListField<DataAppmeshVirtualNodeSpecElListenerElPortMappingEl>>,
    ) -> Self {
        self.port_mapping = Some(v.into());
        self
    }

    #[doc = "Set the field `timeout`.\n"]
    pub fn set_timeout(mut self, v: impl Into<ListField<DataAppmeshVirtualNodeSpecElListenerElTimeoutEl>>) -> Self {
        self.timeout = Some(v.into());
        self
    }

    #[doc = "Set the field `tls`.\n"]
    pub fn set_tls(mut self, v: impl Into<ListField<DataAppmeshVirtualNodeSpecElListenerElTlsEl>>) -> Self {
        self.tls = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualNodeSpecElListenerEl {
    type O = BlockAssignable<DataAppmeshVirtualNodeSpecElListenerEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualNodeSpecElListenerEl {}

impl BuildDataAppmeshVirtualNodeSpecElListenerEl {
    pub fn build(self) -> DataAppmeshVirtualNodeSpecElListenerEl {
        DataAppmeshVirtualNodeSpecElListenerEl {
            connection_pool: core::default::Default::default(),
            health_check: core::default::Default::default(),
            outlier_detection: core::default::Default::default(),
            port_mapping: core::default::Default::default(),
            timeout: core::default::Default::default(),
            tls: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualNodeSpecElListenerElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualNodeSpecElListenerElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshVirtualNodeSpecElListenerElRef {
        DataAppmeshVirtualNodeSpecElListenerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualNodeSpecElListenerElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `connection_pool` after provisioning.\n"]
    pub fn connection_pool(&self) -> ListRef<DataAppmeshVirtualNodeSpecElListenerElConnectionPoolElRef> {
        ListRef::new(self.shared().clone(), format!("{}.connection_pool", self.base))
    }

    #[doc = "Get a reference to the value of field `health_check` after provisioning.\n"]
    pub fn health_check(&self) -> ListRef<DataAppmeshVirtualNodeSpecElListenerElHealthCheckElRef> {
        ListRef::new(self.shared().clone(), format!("{}.health_check", self.base))
    }

    #[doc = "Get a reference to the value of field `outlier_detection` after provisioning.\n"]
    pub fn outlier_detection(&self) -> ListRef<DataAppmeshVirtualNodeSpecElListenerElOutlierDetectionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.outlier_detection", self.base))
    }

    #[doc = "Get a reference to the value of field `port_mapping` after provisioning.\n"]
    pub fn port_mapping(&self) -> ListRef<DataAppmeshVirtualNodeSpecElListenerElPortMappingElRef> {
        ListRef::new(self.shared().clone(), format!("{}.port_mapping", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> ListRef<DataAppmeshVirtualNodeSpecElListenerElTimeoutElRef> {
        ListRef::new(self.shared().clone(), format!("{}.timeout", self.base))
    }

    #[doc = "Get a reference to the value of field `tls` after provisioning.\n"]
    pub fn tls(&self) -> ListRef<DataAppmeshVirtualNodeSpecElListenerElTlsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tls", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualNodeSpecElLoggingElAccessLogElFileElFormatElJsonEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}

impl DataAppmeshVirtualNodeSpecElLoggingElAccessLogElFileElFormatElJsonEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualNodeSpecElLoggingElAccessLogElFileElFormatElJsonEl {
    type O = BlockAssignable<DataAppmeshVirtualNodeSpecElLoggingElAccessLogElFileElFormatElJsonEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualNodeSpecElLoggingElAccessLogElFileElFormatElJsonEl {}

impl BuildDataAppmeshVirtualNodeSpecElLoggingElAccessLogElFileElFormatElJsonEl {
    pub fn build(self) -> DataAppmeshVirtualNodeSpecElLoggingElAccessLogElFileElFormatElJsonEl {
        DataAppmeshVirtualNodeSpecElLoggingElAccessLogElFileElFormatElJsonEl {
            key: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualNodeSpecElLoggingElAccessLogElFileElFormatElJsonElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualNodeSpecElLoggingElAccessLogElFileElFormatElJsonElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAppmeshVirtualNodeSpecElLoggingElAccessLogElFileElFormatElJsonElRef {
        DataAppmeshVirtualNodeSpecElLoggingElAccessLogElFileElFormatElJsonElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualNodeSpecElLoggingElAccessLogElFileElFormatElJsonElRef {
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

#[derive(Serialize)]
pub struct DataAppmeshVirtualNodeSpecElLoggingElAccessLogElFileElFormatEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    json: Option<ListField<DataAppmeshVirtualNodeSpecElLoggingElAccessLogElFileElFormatElJsonEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<PrimField<String>>,
}

impl DataAppmeshVirtualNodeSpecElLoggingElAccessLogElFileElFormatEl {
    #[doc = "Set the field `json`.\n"]
    pub fn set_json(
        mut self,
        v: impl Into<ListField<DataAppmeshVirtualNodeSpecElLoggingElAccessLogElFileElFormatElJsonEl>>,
    ) -> Self {
        self.json = Some(v.into());
        self
    }

    #[doc = "Set the field `text`.\n"]
    pub fn set_text(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.text = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualNodeSpecElLoggingElAccessLogElFileElFormatEl {
    type O = BlockAssignable<DataAppmeshVirtualNodeSpecElLoggingElAccessLogElFileElFormatEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualNodeSpecElLoggingElAccessLogElFileElFormatEl {}

impl BuildDataAppmeshVirtualNodeSpecElLoggingElAccessLogElFileElFormatEl {
    pub fn build(self) -> DataAppmeshVirtualNodeSpecElLoggingElAccessLogElFileElFormatEl {
        DataAppmeshVirtualNodeSpecElLoggingElAccessLogElFileElFormatEl {
            json: core::default::Default::default(),
            text: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualNodeSpecElLoggingElAccessLogElFileElFormatElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualNodeSpecElLoggingElAccessLogElFileElFormatElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshVirtualNodeSpecElLoggingElAccessLogElFileElFormatElRef {
        DataAppmeshVirtualNodeSpecElLoggingElAccessLogElFileElFormatElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualNodeSpecElLoggingElAccessLogElFileElFormatElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `json` after provisioning.\n"]
    pub fn json(&self) -> ListRef<DataAppmeshVirtualNodeSpecElLoggingElAccessLogElFileElFormatElJsonElRef> {
        ListRef::new(self.shared().clone(), format!("{}.json", self.base))
    }

    #[doc = "Get a reference to the value of field `text` after provisioning.\n"]
    pub fn text(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.text", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualNodeSpecElLoggingElAccessLogElFileEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    format: Option<ListField<DataAppmeshVirtualNodeSpecElLoggingElAccessLogElFileElFormatEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
}

impl DataAppmeshVirtualNodeSpecElLoggingElAccessLogElFileEl {
    #[doc = "Set the field `format`.\n"]
    pub fn set_format(
        mut self,
        v: impl Into<ListField<DataAppmeshVirtualNodeSpecElLoggingElAccessLogElFileElFormatEl>>,
    ) -> Self {
        self.format = Some(v.into());
        self
    }

    #[doc = "Set the field `path`.\n"]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualNodeSpecElLoggingElAccessLogElFileEl {
    type O = BlockAssignable<DataAppmeshVirtualNodeSpecElLoggingElAccessLogElFileEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualNodeSpecElLoggingElAccessLogElFileEl {}

impl BuildDataAppmeshVirtualNodeSpecElLoggingElAccessLogElFileEl {
    pub fn build(self) -> DataAppmeshVirtualNodeSpecElLoggingElAccessLogElFileEl {
        DataAppmeshVirtualNodeSpecElLoggingElAccessLogElFileEl {
            format: core::default::Default::default(),
            path: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualNodeSpecElLoggingElAccessLogElFileElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualNodeSpecElLoggingElAccessLogElFileElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshVirtualNodeSpecElLoggingElAccessLogElFileElRef {
        DataAppmeshVirtualNodeSpecElLoggingElAccessLogElFileElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualNodeSpecElLoggingElAccessLogElFileElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `format` after provisioning.\n"]
    pub fn format(&self) -> ListRef<DataAppmeshVirtualNodeSpecElLoggingElAccessLogElFileElFormatElRef> {
        ListRef::new(self.shared().clone(), format!("{}.format", self.base))
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualNodeSpecElLoggingElAccessLogEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    file: Option<ListField<DataAppmeshVirtualNodeSpecElLoggingElAccessLogElFileEl>>,
}

impl DataAppmeshVirtualNodeSpecElLoggingElAccessLogEl {
    #[doc = "Set the field `file`.\n"]
    pub fn set_file(mut self, v: impl Into<ListField<DataAppmeshVirtualNodeSpecElLoggingElAccessLogElFileEl>>) -> Self {
        self.file = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualNodeSpecElLoggingElAccessLogEl {
    type O = BlockAssignable<DataAppmeshVirtualNodeSpecElLoggingElAccessLogEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualNodeSpecElLoggingElAccessLogEl {}

impl BuildDataAppmeshVirtualNodeSpecElLoggingElAccessLogEl {
    pub fn build(self) -> DataAppmeshVirtualNodeSpecElLoggingElAccessLogEl {
        DataAppmeshVirtualNodeSpecElLoggingElAccessLogEl { file: core::default::Default::default() }
    }
}

pub struct DataAppmeshVirtualNodeSpecElLoggingElAccessLogElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualNodeSpecElLoggingElAccessLogElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshVirtualNodeSpecElLoggingElAccessLogElRef {
        DataAppmeshVirtualNodeSpecElLoggingElAccessLogElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualNodeSpecElLoggingElAccessLogElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `file` after provisioning.\n"]
    pub fn file(&self) -> ListRef<DataAppmeshVirtualNodeSpecElLoggingElAccessLogElFileElRef> {
        ListRef::new(self.shared().clone(), format!("{}.file", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualNodeSpecElLoggingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    access_log: Option<ListField<DataAppmeshVirtualNodeSpecElLoggingElAccessLogEl>>,
}

impl DataAppmeshVirtualNodeSpecElLoggingEl {
    #[doc = "Set the field `access_log`.\n"]
    pub fn set_access_log(mut self, v: impl Into<ListField<DataAppmeshVirtualNodeSpecElLoggingElAccessLogEl>>) -> Self {
        self.access_log = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualNodeSpecElLoggingEl {
    type O = BlockAssignable<DataAppmeshVirtualNodeSpecElLoggingEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualNodeSpecElLoggingEl {}

impl BuildDataAppmeshVirtualNodeSpecElLoggingEl {
    pub fn build(self) -> DataAppmeshVirtualNodeSpecElLoggingEl {
        DataAppmeshVirtualNodeSpecElLoggingEl { access_log: core::default::Default::default() }
    }
}

pub struct DataAppmeshVirtualNodeSpecElLoggingElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualNodeSpecElLoggingElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshVirtualNodeSpecElLoggingElRef {
        DataAppmeshVirtualNodeSpecElLoggingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualNodeSpecElLoggingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `access_log` after provisioning.\n"]
    pub fn access_log(&self) -> ListRef<DataAppmeshVirtualNodeSpecElLoggingElAccessLogElRef> {
        ListRef::new(self.shared().clone(), format!("{}.access_log", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualNodeSpecElServiceDiscoveryElAwsCloudMapEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    attributes: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    namespace_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_name: Option<PrimField<String>>,
}

impl DataAppmeshVirtualNodeSpecElServiceDiscoveryElAwsCloudMapEl {
    #[doc = "Set the field `attributes`.\n"]
    pub fn set_attributes(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.attributes = Some(v.into());
        self
    }

    #[doc = "Set the field `namespace_name`.\n"]
    pub fn set_namespace_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.namespace_name = Some(v.into());
        self
    }

    #[doc = "Set the field `service_name`.\n"]
    pub fn set_service_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_name = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualNodeSpecElServiceDiscoveryElAwsCloudMapEl {
    type O = BlockAssignable<DataAppmeshVirtualNodeSpecElServiceDiscoveryElAwsCloudMapEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualNodeSpecElServiceDiscoveryElAwsCloudMapEl {}

impl BuildDataAppmeshVirtualNodeSpecElServiceDiscoveryElAwsCloudMapEl {
    pub fn build(self) -> DataAppmeshVirtualNodeSpecElServiceDiscoveryElAwsCloudMapEl {
        DataAppmeshVirtualNodeSpecElServiceDiscoveryElAwsCloudMapEl {
            attributes: core::default::Default::default(),
            namespace_name: core::default::Default::default(),
            service_name: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualNodeSpecElServiceDiscoveryElAwsCloudMapElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualNodeSpecElServiceDiscoveryElAwsCloudMapElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshVirtualNodeSpecElServiceDiscoveryElAwsCloudMapElRef {
        DataAppmeshVirtualNodeSpecElServiceDiscoveryElAwsCloudMapElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualNodeSpecElServiceDiscoveryElAwsCloudMapElRef {
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
pub struct DataAppmeshVirtualNodeSpecElServiceDiscoveryElDnsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    hostname: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_preference: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_type: Option<PrimField<String>>,
}

impl DataAppmeshVirtualNodeSpecElServiceDiscoveryElDnsEl {
    #[doc = "Set the field `hostname`.\n"]
    pub fn set_hostname(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.hostname = Some(v.into());
        self
    }

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

impl ToListMappable for DataAppmeshVirtualNodeSpecElServiceDiscoveryElDnsEl {
    type O = BlockAssignable<DataAppmeshVirtualNodeSpecElServiceDiscoveryElDnsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualNodeSpecElServiceDiscoveryElDnsEl {}

impl BuildDataAppmeshVirtualNodeSpecElServiceDiscoveryElDnsEl {
    pub fn build(self) -> DataAppmeshVirtualNodeSpecElServiceDiscoveryElDnsEl {
        DataAppmeshVirtualNodeSpecElServiceDiscoveryElDnsEl {
            hostname: core::default::Default::default(),
            ip_preference: core::default::Default::default(),
            response_type: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualNodeSpecElServiceDiscoveryElDnsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualNodeSpecElServiceDiscoveryElDnsElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshVirtualNodeSpecElServiceDiscoveryElDnsElRef {
        DataAppmeshVirtualNodeSpecElServiceDiscoveryElDnsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualNodeSpecElServiceDiscoveryElDnsElRef {
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

#[derive(Serialize)]
pub struct DataAppmeshVirtualNodeSpecElServiceDiscoveryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    aws_cloud_map: Option<ListField<DataAppmeshVirtualNodeSpecElServiceDiscoveryElAwsCloudMapEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dns: Option<ListField<DataAppmeshVirtualNodeSpecElServiceDiscoveryElDnsEl>>,
}

impl DataAppmeshVirtualNodeSpecElServiceDiscoveryEl {
    #[doc = "Set the field `aws_cloud_map`.\n"]
    pub fn set_aws_cloud_map(
        mut self,
        v: impl Into<ListField<DataAppmeshVirtualNodeSpecElServiceDiscoveryElAwsCloudMapEl>>,
    ) -> Self {
        self.aws_cloud_map = Some(v.into());
        self
    }

    #[doc = "Set the field `dns`.\n"]
    pub fn set_dns(mut self, v: impl Into<ListField<DataAppmeshVirtualNodeSpecElServiceDiscoveryElDnsEl>>) -> Self {
        self.dns = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualNodeSpecElServiceDiscoveryEl {
    type O = BlockAssignable<DataAppmeshVirtualNodeSpecElServiceDiscoveryEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualNodeSpecElServiceDiscoveryEl {}

impl BuildDataAppmeshVirtualNodeSpecElServiceDiscoveryEl {
    pub fn build(self) -> DataAppmeshVirtualNodeSpecElServiceDiscoveryEl {
        DataAppmeshVirtualNodeSpecElServiceDiscoveryEl {
            aws_cloud_map: core::default::Default::default(),
            dns: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualNodeSpecElServiceDiscoveryElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualNodeSpecElServiceDiscoveryElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshVirtualNodeSpecElServiceDiscoveryElRef {
        DataAppmeshVirtualNodeSpecElServiceDiscoveryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualNodeSpecElServiceDiscoveryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `aws_cloud_map` after provisioning.\n"]
    pub fn aws_cloud_map(&self) -> ListRef<DataAppmeshVirtualNodeSpecElServiceDiscoveryElAwsCloudMapElRef> {
        ListRef::new(self.shared().clone(), format!("{}.aws_cloud_map", self.base))
    }

    #[doc = "Get a reference to the value of field `dns` after provisioning.\n"]
    pub fn dns(&self) -> ListRef<DataAppmeshVirtualNodeSpecElServiceDiscoveryElDnsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.dns", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualNodeSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    backend: Option<SetField<DataAppmeshVirtualNodeSpecElBackendEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    backend_defaults: Option<ListField<DataAppmeshVirtualNodeSpecElBackendDefaultsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    listener: Option<ListField<DataAppmeshVirtualNodeSpecElListenerEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logging: Option<ListField<DataAppmeshVirtualNodeSpecElLoggingEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_discovery: Option<ListField<DataAppmeshVirtualNodeSpecElServiceDiscoveryEl>>,
}

impl DataAppmeshVirtualNodeSpecEl {
    #[doc = "Set the field `backend`.\n"]
    pub fn set_backend(mut self, v: impl Into<SetField<DataAppmeshVirtualNodeSpecElBackendEl>>) -> Self {
        self.backend = Some(v.into());
        self
    }

    #[doc = "Set the field `backend_defaults`.\n"]
    pub fn set_backend_defaults(
        mut self,
        v: impl Into<ListField<DataAppmeshVirtualNodeSpecElBackendDefaultsEl>>,
    ) -> Self {
        self.backend_defaults = Some(v.into());
        self
    }

    #[doc = "Set the field `listener`.\n"]
    pub fn set_listener(mut self, v: impl Into<ListField<DataAppmeshVirtualNodeSpecElListenerEl>>) -> Self {
        self.listener = Some(v.into());
        self
    }

    #[doc = "Set the field `logging`.\n"]
    pub fn set_logging(mut self, v: impl Into<ListField<DataAppmeshVirtualNodeSpecElLoggingEl>>) -> Self {
        self.logging = Some(v.into());
        self
    }

    #[doc = "Set the field `service_discovery`.\n"]
    pub fn set_service_discovery(
        mut self,
        v: impl Into<ListField<DataAppmeshVirtualNodeSpecElServiceDiscoveryEl>>,
    ) -> Self {
        self.service_discovery = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualNodeSpecEl {
    type O = BlockAssignable<DataAppmeshVirtualNodeSpecEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualNodeSpecEl {}

impl BuildDataAppmeshVirtualNodeSpecEl {
    pub fn build(self) -> DataAppmeshVirtualNodeSpecEl {
        DataAppmeshVirtualNodeSpecEl {
            backend: core::default::Default::default(),
            backend_defaults: core::default::Default::default(),
            listener: core::default::Default::default(),
            logging: core::default::Default::default(),
            service_discovery: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualNodeSpecElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualNodeSpecElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshVirtualNodeSpecElRef {
        DataAppmeshVirtualNodeSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualNodeSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `backend` after provisioning.\n"]
    pub fn backend(&self) -> SetRef<DataAppmeshVirtualNodeSpecElBackendElRef> {
        SetRef::new(self.shared().clone(), format!("{}.backend", self.base))
    }

    #[doc = "Get a reference to the value of field `backend_defaults` after provisioning.\n"]
    pub fn backend_defaults(&self) -> ListRef<DataAppmeshVirtualNodeSpecElBackendDefaultsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.backend_defaults", self.base))
    }

    #[doc = "Get a reference to the value of field `listener` after provisioning.\n"]
    pub fn listener(&self) -> ListRef<DataAppmeshVirtualNodeSpecElListenerElRef> {
        ListRef::new(self.shared().clone(), format!("{}.listener", self.base))
    }

    #[doc = "Get a reference to the value of field `logging` after provisioning.\n"]
    pub fn logging(&self) -> ListRef<DataAppmeshVirtualNodeSpecElLoggingElRef> {
        ListRef::new(self.shared().clone(), format!("{}.logging", self.base))
    }

    #[doc = "Get a reference to the value of field `service_discovery` after provisioning.\n"]
    pub fn service_discovery(&self) -> ListRef<DataAppmeshVirtualNodeSpecElServiceDiscoveryElRef> {
        ListRef::new(self.shared().clone(), format!("{}.service_discovery", self.base))
    }
}
