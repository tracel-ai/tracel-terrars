use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataAppmeshVirtualGatewayData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    mesh_name: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
}

struct DataAppmeshVirtualGateway_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataAppmeshVirtualGatewayData>,
}

#[derive(Clone)]
pub struct DataAppmeshVirtualGateway(Rc<DataAppmeshVirtualGateway_>);

impl DataAppmeshVirtualGateway {
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
    pub fn spec(&self) -> ListRef<DataAppmeshVirtualGatewaySpecElRef> {
        ListRef::new(self.shared().clone(), format!("{}.spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

impl Referable for DataAppmeshVirtualGateway {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataAppmeshVirtualGateway { }

impl ToListMappable for DataAppmeshVirtualGateway {
    type O = ListRef<DataAppmeshVirtualGatewayRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataAppmeshVirtualGateway_ {
    fn extract_datasource_type(&self) -> String {
        "aws_appmesh_virtual_gateway".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataAppmeshVirtualGateway {
    pub tf_id: String,
    #[doc = ""]
    pub mesh_name: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildDataAppmeshVirtualGateway {
    pub fn build(self, stack: &mut Stack) -> DataAppmeshVirtualGateway {
        let out = DataAppmeshVirtualGateway(Rc::new(DataAppmeshVirtualGateway_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataAppmeshVirtualGatewayData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                mesh_name: self.mesh_name,
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataAppmeshVirtualGatewayRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualGatewayRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataAppmeshVirtualGatewayRef {
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
    pub fn spec(&self) -> ListRef<DataAppmeshVirtualGatewaySpecElRef> {
        ListRef::new(self.shared().clone(), format!("{}.spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate_chain: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    private_key: Option<PrimField<String>>,
}

impl DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileEl {
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

impl ToListMappable for DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileEl {
    type O = BlockAssignable<DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileEl {}

impl BuildDataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileEl {
    pub fn build(self) -> DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileEl {
        DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileEl {
            certificate_chain: core::default::Default::default(),
            private_key: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileElRef {
        DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileElRef {
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
pub struct DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_name: Option<PrimField<String>>,
}

impl DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsEl {
    #[doc = "Set the field `secret_name`.\n"]
    pub fn set_secret_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.secret_name = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsEl {
    type O = BlockAssignable<DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsEl {}

impl BuildDataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsEl {
    pub fn build(self) -> DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsEl {
        DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsEl {
            secret_name: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsElRef {
        DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `secret_name` after provisioning.\n"]
    pub fn secret_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    file: Option<ListField<DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sds: Option<ListField<DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsEl>>,
}

impl DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateEl {
    #[doc = "Set the field `file`.\n"]
    pub fn set_file(
        mut self,
        v:
            impl

                    Into<
                        ListField<
                            DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileEl,
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
                            DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsEl,
                        >,
                    >,
    ) -> Self {
        self.sds = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateEl {
    type O = BlockAssignable<DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateEl {}

impl BuildDataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateEl {
    pub fn build(self) -> DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateEl {
        DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateEl {
            file: core::default::Default::default(),
            sds: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElRef {
        DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `file` after provisioning.\n"]
    pub fn file(
        &self,
    ) -> ListRef<DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileElRef> {
        ListRef::new(self.shared().clone(), format!("{}.file", self.base))
    }

    #[doc = "Get a reference to the value of field `sds` after provisioning.\n"]
    pub fn sds(
        &self,
    ) -> ListRef<DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sds", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    exact: Option<SetField<PrimField<String>>>,
}

impl DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl {
    #[doc = "Set the field `exact`.\n"]
    pub fn set_exact(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.exact = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl {
    type O =
        BlockAssignable<
            DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl {}

impl BuildDataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl {
    pub fn build(
        self,
    ) -> DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl {
        DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl {
            exact: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchElRef {
        DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `exact` after provisioning.\n"]
    pub fn exact(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.exact", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl {
    #[serde(rename = "match", skip_serializing_if = "Option::is_none")]
    match_: Option<
        ListField<
            DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl,
        >,
    >,
}

impl DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl {
    #[doc = "Set the field `match_`.\n"]
    pub fn set_match(
        mut self,
        v:
            impl

                    Into<
                        ListField<
                            DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl,
                        >,
                    >,
    ) -> Self {
        self.match_ = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl {
    type O =
        BlockAssignable<
            DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl {}

impl BuildDataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl {
    pub fn build(
        self,
    ) -> DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl {
        DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl {
            match_: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElRef {
        DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(
        &self,
    ) -> ListRef<
        DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate_authority_arns: Option<SetField<PrimField<String>>>,
}

impl DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmEl {
    #[doc = "Set the field `certificate_authority_arns`.\n"]
    pub fn set_certificate_authority_arns(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.certificate_authority_arns = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmEl {
    type O =
        BlockAssignable<DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmEl {}

impl BuildDataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmEl {
    pub fn build(
        self,
    ) -> DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmEl {
        DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmEl {
            certificate_authority_arns: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmElRef {
        DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificate_authority_arns` after provisioning.\n"]
    pub fn certificate_authority_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.certificate_authority_arns", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate_chain: Option<PrimField<String>>,
}

impl DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileEl {
    #[doc = "Set the field `certificate_chain`.\n"]
    pub fn set_certificate_chain(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.certificate_chain = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileEl {
    type O =
        BlockAssignable<
            DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileEl {}

impl BuildDataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileEl {
    pub fn build(
        self,
    ) -> DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileEl {
        DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileEl {
            certificate_chain: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileElRef {
        DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificate_chain` after provisioning.\n"]
    pub fn certificate_chain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.certificate_chain", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_name: Option<PrimField<String>>,
}

impl DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsEl {
    #[doc = "Set the field `secret_name`.\n"]
    pub fn set_secret_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.secret_name = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsEl {
    type O =
        BlockAssignable<DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsEl {}

impl BuildDataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsEl {
    pub fn build(
        self,
    ) -> DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsEl {
        DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsEl {
            secret_name: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsElRef {
        DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `secret_name` after provisioning.\n"]
    pub fn secret_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    acm: Option<ListField<DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file: Option<ListField<DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sds: Option<ListField<DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsEl>>,
}

impl DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustEl {
    #[doc = "Set the field `acm`.\n"]
    pub fn set_acm(
        mut self,
        v:
            impl

                    Into<
                        ListField<
                            DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmEl,
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
                            DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileEl,
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
                            DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsEl,
                        >,
                    >,
    ) -> Self {
        self.sds = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustEl {
    type O = BlockAssignable<DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustEl {}

impl BuildDataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustEl {
    pub fn build(self) -> DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustEl {
        DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustEl {
            acm: core::default::Default::default(),
            file: core::default::Default::default(),
            sds: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElRef {
        DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `acm` after provisioning.\n"]
    pub fn acm(
        &self,
    ) -> ListRef<DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmElRef> {
        ListRef::new(self.shared().clone(), format!("{}.acm", self.base))
    }

    #[doc = "Get a reference to the value of field `file` after provisioning.\n"]
    pub fn file(
        &self,
    ) -> ListRef<DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileElRef> {
        ListRef::new(self.shared().clone(), format!("{}.file", self.base))
    }

    #[doc = "Get a reference to the value of field `sds` after provisioning.\n"]
    pub fn sds(
        &self,
    ) -> ListRef<DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sds", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    subject_alternative_names: Option<
        ListField<
            DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    trust: Option<ListField<DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustEl>>,
}

impl DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationEl {
    #[doc = "Set the field `subject_alternative_names`.\n"]
    pub fn set_subject_alternative_names(
        mut self,
        v:
            impl

                    Into<
                        ListField<
                            DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl,
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
                            DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustEl,
                        >,
                    >,
    ) -> Self {
        self.trust = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationEl {
    type O = BlockAssignable<DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationEl {}

impl BuildDataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationEl {
    pub fn build(self) -> DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationEl {
        DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationEl {
            subject_alternative_names: core::default::Default::default(),
            trust: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElRef {
        DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `subject_alternative_names` after provisioning.\n"]
    pub fn subject_alternative_names(
        &self,
    ) -> ListRef<
        DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.subject_alternative_names", self.base))
    }

    #[doc = "Get a reference to the value of field `trust` after provisioning.\n"]
    pub fn trust(
        &self,
    ) -> ListRef<DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElRef> {
        ListRef::new(self.shared().clone(), format!("{}.trust", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate: Option<ListField<DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enforce: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ports: Option<SetField<PrimField<f64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    validation: Option<ListField<DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationEl>>,
}

impl DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsEl {
    #[doc = "Set the field `certificate`.\n"]
    pub fn set_certificate(
        mut self,
        v: impl Into<ListField<DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateEl>>,
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
        v: impl Into<ListField<DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationEl>>,
    ) -> Self {
        self.validation = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsEl {
    type O = BlockAssignable<DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsEl {}

impl BuildDataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsEl {
    pub fn build(self) -> DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsEl {
        DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsEl {
            certificate: core::default::Default::default(),
            enforce: core::default::Default::default(),
            ports: core::default::Default::default(),
            validation: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElRef {
        DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificate` after provisioning.\n"]
    pub fn certificate(
        &self,
    ) -> ListRef<DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElRef> {
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
    ) -> ListRef<DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.validation", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    tls: Option<ListField<DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsEl>>,
}

impl DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyEl {
    #[doc = "Set the field `tls`.\n"]
    pub fn set_tls(
        mut self,
        v: impl Into<ListField<DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsEl>>,
    ) -> Self {
        self.tls = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyEl {
    type O = BlockAssignable<DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyEl {}

impl BuildDataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyEl {
    pub fn build(self) -> DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyEl {
        DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyEl { tls: core::default::Default::default() }
    }
}

pub struct DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElRef {
        DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `tls` after provisioning.\n"]
    pub fn tls(&self) -> ListRef<DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tls", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualGatewaySpecElBackendDefaultsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    client_policy: Option<ListField<DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyEl>>,
}

impl DataAppmeshVirtualGatewaySpecElBackendDefaultsEl {
    #[doc = "Set the field `client_policy`.\n"]
    pub fn set_client_policy(
        mut self,
        v: impl Into<ListField<DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyEl>>,
    ) -> Self {
        self.client_policy = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualGatewaySpecElBackendDefaultsEl {
    type O = BlockAssignable<DataAppmeshVirtualGatewaySpecElBackendDefaultsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualGatewaySpecElBackendDefaultsEl {}

impl BuildDataAppmeshVirtualGatewaySpecElBackendDefaultsEl {
    pub fn build(self) -> DataAppmeshVirtualGatewaySpecElBackendDefaultsEl {
        DataAppmeshVirtualGatewaySpecElBackendDefaultsEl { client_policy: core::default::Default::default() }
    }
}

pub struct DataAppmeshVirtualGatewaySpecElBackendDefaultsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualGatewaySpecElBackendDefaultsElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshVirtualGatewaySpecElBackendDefaultsElRef {
        DataAppmeshVirtualGatewaySpecElBackendDefaultsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualGatewaySpecElBackendDefaultsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `client_policy` after provisioning.\n"]
    pub fn client_policy(&self) -> ListRef<DataAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.client_policy", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualGatewaySpecElListenerElConnectionPoolElGrpcEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_requests: Option<PrimField<f64>>,
}

impl DataAppmeshVirtualGatewaySpecElListenerElConnectionPoolElGrpcEl {
    #[doc = "Set the field `max_requests`.\n"]
    pub fn set_max_requests(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_requests = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualGatewaySpecElListenerElConnectionPoolElGrpcEl {
    type O = BlockAssignable<DataAppmeshVirtualGatewaySpecElListenerElConnectionPoolElGrpcEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualGatewaySpecElListenerElConnectionPoolElGrpcEl {}

impl BuildDataAppmeshVirtualGatewaySpecElListenerElConnectionPoolElGrpcEl {
    pub fn build(self) -> DataAppmeshVirtualGatewaySpecElListenerElConnectionPoolElGrpcEl {
        DataAppmeshVirtualGatewaySpecElListenerElConnectionPoolElGrpcEl {
            max_requests: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualGatewaySpecElListenerElConnectionPoolElGrpcElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualGatewaySpecElListenerElConnectionPoolElGrpcElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshVirtualGatewaySpecElListenerElConnectionPoolElGrpcElRef {
        DataAppmeshVirtualGatewaySpecElListenerElConnectionPoolElGrpcElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualGatewaySpecElListenerElConnectionPoolElGrpcElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_requests` after provisioning.\n"]
    pub fn max_requests(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_requests", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttpEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_connections: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_pending_requests: Option<PrimField<f64>>,
}

impl DataAppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttpEl {
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

impl ToListMappable for DataAppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttpEl {
    type O = BlockAssignable<DataAppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttpEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttpEl {}

impl BuildDataAppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttpEl {
    pub fn build(self) -> DataAppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttpEl {
        DataAppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttpEl {
            max_connections: core::default::Default::default(),
            max_pending_requests: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttpElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttpElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttpElRef {
        DataAppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttpElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttpElRef {
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
pub struct DataAppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttp2El {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_requests: Option<PrimField<f64>>,
}

impl DataAppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttp2El {
    #[doc = "Set the field `max_requests`.\n"]
    pub fn set_max_requests(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_requests = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttp2El {
    type O = BlockAssignable<DataAppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttp2El>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttp2El {}

impl BuildDataAppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttp2El {
    pub fn build(self) -> DataAppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttp2El {
        DataAppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttp2El {
            max_requests: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttp2ElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttp2ElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttp2ElRef {
        DataAppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttp2ElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttp2ElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_requests` after provisioning.\n"]
    pub fn max_requests(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_requests", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualGatewaySpecElListenerElConnectionPoolEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    grpc: Option<ListField<DataAppmeshVirtualGatewaySpecElListenerElConnectionPoolElGrpcEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http: Option<ListField<DataAppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttpEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http2: Option<ListField<DataAppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttp2El>>,
}

impl DataAppmeshVirtualGatewaySpecElListenerElConnectionPoolEl {
    #[doc = "Set the field `grpc`.\n"]
    pub fn set_grpc(
        mut self,
        v: impl Into<ListField<DataAppmeshVirtualGatewaySpecElListenerElConnectionPoolElGrpcEl>>,
    ) -> Self {
        self.grpc = Some(v.into());
        self
    }

    #[doc = "Set the field `http`.\n"]
    pub fn set_http(
        mut self,
        v: impl Into<ListField<DataAppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttpEl>>,
    ) -> Self {
        self.http = Some(v.into());
        self
    }

    #[doc = "Set the field `http2`.\n"]
    pub fn set_http2(
        mut self,
        v: impl Into<ListField<DataAppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttp2El>>,
    ) -> Self {
        self.http2 = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualGatewaySpecElListenerElConnectionPoolEl {
    type O = BlockAssignable<DataAppmeshVirtualGatewaySpecElListenerElConnectionPoolEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualGatewaySpecElListenerElConnectionPoolEl {}

impl BuildDataAppmeshVirtualGatewaySpecElListenerElConnectionPoolEl {
    pub fn build(self) -> DataAppmeshVirtualGatewaySpecElListenerElConnectionPoolEl {
        DataAppmeshVirtualGatewaySpecElListenerElConnectionPoolEl {
            grpc: core::default::Default::default(),
            http: core::default::Default::default(),
            http2: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualGatewaySpecElListenerElConnectionPoolElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualGatewaySpecElListenerElConnectionPoolElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshVirtualGatewaySpecElListenerElConnectionPoolElRef {
        DataAppmeshVirtualGatewaySpecElListenerElConnectionPoolElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualGatewaySpecElListenerElConnectionPoolElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `grpc` after provisioning.\n"]
    pub fn grpc(&self) -> ListRef<DataAppmeshVirtualGatewaySpecElListenerElConnectionPoolElGrpcElRef> {
        ListRef::new(self.shared().clone(), format!("{}.grpc", self.base))
    }

    #[doc = "Get a reference to the value of field `http` after provisioning.\n"]
    pub fn http(&self) -> ListRef<DataAppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttpElRef> {
        ListRef::new(self.shared().clone(), format!("{}.http", self.base))
    }

    #[doc = "Get a reference to the value of field `http2` after provisioning.\n"]
    pub fn http2(&self) -> ListRef<DataAppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttp2ElRef> {
        ListRef::new(self.shared().clone(), format!("{}.http2", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualGatewaySpecElListenerElHealthCheckEl {
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

impl DataAppmeshVirtualGatewaySpecElListenerElHealthCheckEl {
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

impl ToListMappable for DataAppmeshVirtualGatewaySpecElListenerElHealthCheckEl {
    type O = BlockAssignable<DataAppmeshVirtualGatewaySpecElListenerElHealthCheckEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualGatewaySpecElListenerElHealthCheckEl {}

impl BuildDataAppmeshVirtualGatewaySpecElListenerElHealthCheckEl {
    pub fn build(self) -> DataAppmeshVirtualGatewaySpecElListenerElHealthCheckEl {
        DataAppmeshVirtualGatewaySpecElListenerElHealthCheckEl {
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

pub struct DataAppmeshVirtualGatewaySpecElListenerElHealthCheckElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualGatewaySpecElListenerElHealthCheckElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshVirtualGatewaySpecElListenerElHealthCheckElRef {
        DataAppmeshVirtualGatewaySpecElListenerElHealthCheckElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualGatewaySpecElListenerElHealthCheckElRef {
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
pub struct DataAppmeshVirtualGatewaySpecElListenerElPortMappingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protocol: Option<PrimField<String>>,
}

impl DataAppmeshVirtualGatewaySpecElListenerElPortMappingEl {
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

impl ToListMappable for DataAppmeshVirtualGatewaySpecElListenerElPortMappingEl {
    type O = BlockAssignable<DataAppmeshVirtualGatewaySpecElListenerElPortMappingEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualGatewaySpecElListenerElPortMappingEl {}

impl BuildDataAppmeshVirtualGatewaySpecElListenerElPortMappingEl {
    pub fn build(self) -> DataAppmeshVirtualGatewaySpecElListenerElPortMappingEl {
        DataAppmeshVirtualGatewaySpecElListenerElPortMappingEl {
            port: core::default::Default::default(),
            protocol: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualGatewaySpecElListenerElPortMappingElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualGatewaySpecElListenerElPortMappingElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshVirtualGatewaySpecElListenerElPortMappingElRef {
        DataAppmeshVirtualGatewaySpecElListenerElPortMappingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualGatewaySpecElListenerElPortMappingElRef {
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
pub struct DataAppmeshVirtualGatewaySpecElListenerElTlsElCertificateElAcmEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate_arn: Option<PrimField<String>>,
}

impl DataAppmeshVirtualGatewaySpecElListenerElTlsElCertificateElAcmEl {
    #[doc = "Set the field `certificate_arn`.\n"]
    pub fn set_certificate_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.certificate_arn = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualGatewaySpecElListenerElTlsElCertificateElAcmEl {
    type O = BlockAssignable<DataAppmeshVirtualGatewaySpecElListenerElTlsElCertificateElAcmEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualGatewaySpecElListenerElTlsElCertificateElAcmEl {}

impl BuildDataAppmeshVirtualGatewaySpecElListenerElTlsElCertificateElAcmEl {
    pub fn build(self) -> DataAppmeshVirtualGatewaySpecElListenerElTlsElCertificateElAcmEl {
        DataAppmeshVirtualGatewaySpecElListenerElTlsElCertificateElAcmEl {
            certificate_arn: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualGatewaySpecElListenerElTlsElCertificateElAcmElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualGatewaySpecElListenerElTlsElCertificateElAcmElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshVirtualGatewaySpecElListenerElTlsElCertificateElAcmElRef {
        DataAppmeshVirtualGatewaySpecElListenerElTlsElCertificateElAcmElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualGatewaySpecElListenerElTlsElCertificateElAcmElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificate_arn` after provisioning.\n"]
    pub fn certificate_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.certificate_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualGatewaySpecElListenerElTlsElCertificateElFileEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate_chain: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    private_key: Option<PrimField<String>>,
}

impl DataAppmeshVirtualGatewaySpecElListenerElTlsElCertificateElFileEl {
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

impl ToListMappable for DataAppmeshVirtualGatewaySpecElListenerElTlsElCertificateElFileEl {
    type O = BlockAssignable<DataAppmeshVirtualGatewaySpecElListenerElTlsElCertificateElFileEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualGatewaySpecElListenerElTlsElCertificateElFileEl {}

impl BuildDataAppmeshVirtualGatewaySpecElListenerElTlsElCertificateElFileEl {
    pub fn build(self) -> DataAppmeshVirtualGatewaySpecElListenerElTlsElCertificateElFileEl {
        DataAppmeshVirtualGatewaySpecElListenerElTlsElCertificateElFileEl {
            certificate_chain: core::default::Default::default(),
            private_key: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualGatewaySpecElListenerElTlsElCertificateElFileElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualGatewaySpecElListenerElTlsElCertificateElFileElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAppmeshVirtualGatewaySpecElListenerElTlsElCertificateElFileElRef {
        DataAppmeshVirtualGatewaySpecElListenerElTlsElCertificateElFileElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualGatewaySpecElListenerElTlsElCertificateElFileElRef {
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
pub struct DataAppmeshVirtualGatewaySpecElListenerElTlsElCertificateElSdsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_name: Option<PrimField<String>>,
}

impl DataAppmeshVirtualGatewaySpecElListenerElTlsElCertificateElSdsEl {
    #[doc = "Set the field `secret_name`.\n"]
    pub fn set_secret_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.secret_name = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualGatewaySpecElListenerElTlsElCertificateElSdsEl {
    type O = BlockAssignable<DataAppmeshVirtualGatewaySpecElListenerElTlsElCertificateElSdsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualGatewaySpecElListenerElTlsElCertificateElSdsEl {}

impl BuildDataAppmeshVirtualGatewaySpecElListenerElTlsElCertificateElSdsEl {
    pub fn build(self) -> DataAppmeshVirtualGatewaySpecElListenerElTlsElCertificateElSdsEl {
        DataAppmeshVirtualGatewaySpecElListenerElTlsElCertificateElSdsEl {
            secret_name: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualGatewaySpecElListenerElTlsElCertificateElSdsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualGatewaySpecElListenerElTlsElCertificateElSdsElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshVirtualGatewaySpecElListenerElTlsElCertificateElSdsElRef {
        DataAppmeshVirtualGatewaySpecElListenerElTlsElCertificateElSdsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualGatewaySpecElListenerElTlsElCertificateElSdsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `secret_name` after provisioning.\n"]
    pub fn secret_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualGatewaySpecElListenerElTlsElCertificateEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    acm: Option<ListField<DataAppmeshVirtualGatewaySpecElListenerElTlsElCertificateElAcmEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file: Option<ListField<DataAppmeshVirtualGatewaySpecElListenerElTlsElCertificateElFileEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sds: Option<ListField<DataAppmeshVirtualGatewaySpecElListenerElTlsElCertificateElSdsEl>>,
}

impl DataAppmeshVirtualGatewaySpecElListenerElTlsElCertificateEl {
    #[doc = "Set the field `acm`.\n"]
    pub fn set_acm(
        mut self,
        v: impl Into<ListField<DataAppmeshVirtualGatewaySpecElListenerElTlsElCertificateElAcmEl>>,
    ) -> Self {
        self.acm = Some(v.into());
        self
    }

    #[doc = "Set the field `file`.\n"]
    pub fn set_file(
        mut self,
        v: impl Into<ListField<DataAppmeshVirtualGatewaySpecElListenerElTlsElCertificateElFileEl>>,
    ) -> Self {
        self.file = Some(v.into());
        self
    }

    #[doc = "Set the field `sds`.\n"]
    pub fn set_sds(
        mut self,
        v: impl Into<ListField<DataAppmeshVirtualGatewaySpecElListenerElTlsElCertificateElSdsEl>>,
    ) -> Self {
        self.sds = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualGatewaySpecElListenerElTlsElCertificateEl {
    type O = BlockAssignable<DataAppmeshVirtualGatewaySpecElListenerElTlsElCertificateEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualGatewaySpecElListenerElTlsElCertificateEl {}

impl BuildDataAppmeshVirtualGatewaySpecElListenerElTlsElCertificateEl {
    pub fn build(self) -> DataAppmeshVirtualGatewaySpecElListenerElTlsElCertificateEl {
        DataAppmeshVirtualGatewaySpecElListenerElTlsElCertificateEl {
            acm: core::default::Default::default(),
            file: core::default::Default::default(),
            sds: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualGatewaySpecElListenerElTlsElCertificateElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualGatewaySpecElListenerElTlsElCertificateElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshVirtualGatewaySpecElListenerElTlsElCertificateElRef {
        DataAppmeshVirtualGatewaySpecElListenerElTlsElCertificateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualGatewaySpecElListenerElTlsElCertificateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `acm` after provisioning.\n"]
    pub fn acm(&self) -> ListRef<DataAppmeshVirtualGatewaySpecElListenerElTlsElCertificateElAcmElRef> {
        ListRef::new(self.shared().clone(), format!("{}.acm", self.base))
    }

    #[doc = "Get a reference to the value of field `file` after provisioning.\n"]
    pub fn file(&self) -> ListRef<DataAppmeshVirtualGatewaySpecElListenerElTlsElCertificateElFileElRef> {
        ListRef::new(self.shared().clone(), format!("{}.file", self.base))
    }

    #[doc = "Get a reference to the value of field `sds` after provisioning.\n"]
    pub fn sds(&self) -> ListRef<DataAppmeshVirtualGatewaySpecElListenerElTlsElCertificateElSdsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sds", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    exact: Option<SetField<PrimField<String>>>,
}

impl DataAppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchEl {
    #[doc = "Set the field `exact`.\n"]
    pub fn set_exact(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.exact = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchEl {
    type O =
        BlockAssignable<DataAppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchEl {}

impl BuildDataAppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchEl {
    pub fn build(self) -> DataAppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchEl {
        DataAppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchEl {
            exact: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchElRef {
        DataAppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `exact` after provisioning.\n"]
    pub fn exact(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.exact", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesEl {
    #[serde(rename = "match", skip_serializing_if = "Option::is_none")]
    match_: Option<
        ListField<DataAppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchEl>,
    >,
}

impl DataAppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesEl {
    #[doc = "Set the field `match_`.\n"]
    pub fn set_match(
        mut self,
        v:
            impl

                    Into<
                        ListField<
                            DataAppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchEl,
                        >,
                    >,
    ) -> Self {
        self.match_ = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesEl {
    type O = BlockAssignable<DataAppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesEl {}

impl BuildDataAppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesEl {
    pub fn build(self) -> DataAppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesEl {
        DataAppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesEl {
            match_: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesElRef {
        DataAppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(
        &self,
    ) -> ListRef<DataAppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchElRef> {
        ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElFileEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate_chain: Option<PrimField<String>>,
}

impl DataAppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElFileEl {
    #[doc = "Set the field `certificate_chain`.\n"]
    pub fn set_certificate_chain(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.certificate_chain = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElFileEl {
    type O = BlockAssignable<DataAppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElFileEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElFileEl {}

impl BuildDataAppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElFileEl {
    pub fn build(self) -> DataAppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElFileEl {
        DataAppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElFileEl {
            certificate_chain: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElFileElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElFileElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElFileElRef {
        DataAppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElFileElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElFileElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificate_chain` after provisioning.\n"]
    pub fn certificate_chain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.certificate_chain", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElSdsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_name: Option<PrimField<String>>,
}

impl DataAppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElSdsEl {
    #[doc = "Set the field `secret_name`.\n"]
    pub fn set_secret_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.secret_name = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElSdsEl {
    type O = BlockAssignable<DataAppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElSdsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElSdsEl {}

impl BuildDataAppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElSdsEl {
    pub fn build(self) -> DataAppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElSdsEl {
        DataAppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElSdsEl {
            secret_name: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElSdsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElSdsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElSdsElRef {
        DataAppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElSdsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElSdsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `secret_name` after provisioning.\n"]
    pub fn secret_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    file: Option<ListField<DataAppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElFileEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sds: Option<ListField<DataAppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElSdsEl>>,
}

impl DataAppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustEl {
    #[doc = "Set the field `file`.\n"]
    pub fn set_file(
        mut self,
        v: impl Into<ListField<DataAppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElFileEl>>,
    ) -> Self {
        self.file = Some(v.into());
        self
    }

    #[doc = "Set the field `sds`.\n"]
    pub fn set_sds(
        mut self,
        v: impl Into<ListField<DataAppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElSdsEl>>,
    ) -> Self {
        self.sds = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustEl {
    type O = BlockAssignable<DataAppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustEl {}

impl BuildDataAppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustEl {
    pub fn build(self) -> DataAppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustEl {
        DataAppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustEl {
            file: core::default::Default::default(),
            sds: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElRef {
        DataAppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `file` after provisioning.\n"]
    pub fn file(&self) -> ListRef<DataAppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElFileElRef> {
        ListRef::new(self.shared().clone(), format!("{}.file", self.base))
    }

    #[doc = "Get a reference to the value of field `sds` after provisioning.\n"]
    pub fn sds(&self) -> ListRef<DataAppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElSdsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sds", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualGatewaySpecElListenerElTlsElValidationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    subject_alternative_names: Option<
        ListField<DataAppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    trust: Option<ListField<DataAppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustEl>>,
}

impl DataAppmeshVirtualGatewaySpecElListenerElTlsElValidationEl {
    #[doc = "Set the field `subject_alternative_names`.\n"]
    pub fn set_subject_alternative_names(
        mut self,
        v: impl Into<ListField<DataAppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesEl>>,
    ) -> Self {
        self.subject_alternative_names = Some(v.into());
        self
    }

    #[doc = "Set the field `trust`.\n"]
    pub fn set_trust(
        mut self,
        v: impl Into<ListField<DataAppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustEl>>,
    ) -> Self {
        self.trust = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualGatewaySpecElListenerElTlsElValidationEl {
    type O = BlockAssignable<DataAppmeshVirtualGatewaySpecElListenerElTlsElValidationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualGatewaySpecElListenerElTlsElValidationEl {}

impl BuildDataAppmeshVirtualGatewaySpecElListenerElTlsElValidationEl {
    pub fn build(self) -> DataAppmeshVirtualGatewaySpecElListenerElTlsElValidationEl {
        DataAppmeshVirtualGatewaySpecElListenerElTlsElValidationEl {
            subject_alternative_names: core::default::Default::default(),
            trust: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualGatewaySpecElListenerElTlsElValidationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualGatewaySpecElListenerElTlsElValidationElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshVirtualGatewaySpecElListenerElTlsElValidationElRef {
        DataAppmeshVirtualGatewaySpecElListenerElTlsElValidationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualGatewaySpecElListenerElTlsElValidationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `subject_alternative_names` after provisioning.\n"]
    pub fn subject_alternative_names(
        &self,
    ) -> ListRef<DataAppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.subject_alternative_names", self.base))
    }

    #[doc = "Get a reference to the value of field `trust` after provisioning.\n"]
    pub fn trust(&self) -> ListRef<DataAppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElRef> {
        ListRef::new(self.shared().clone(), format!("{}.trust", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualGatewaySpecElListenerElTlsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate: Option<ListField<DataAppmeshVirtualGatewaySpecElListenerElTlsElCertificateEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    validation: Option<ListField<DataAppmeshVirtualGatewaySpecElListenerElTlsElValidationEl>>,
}

impl DataAppmeshVirtualGatewaySpecElListenerElTlsEl {
    #[doc = "Set the field `certificate`.\n"]
    pub fn set_certificate(
        mut self,
        v: impl Into<ListField<DataAppmeshVirtualGatewaySpecElListenerElTlsElCertificateEl>>,
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
        v: impl Into<ListField<DataAppmeshVirtualGatewaySpecElListenerElTlsElValidationEl>>,
    ) -> Self {
        self.validation = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualGatewaySpecElListenerElTlsEl {
    type O = BlockAssignable<DataAppmeshVirtualGatewaySpecElListenerElTlsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualGatewaySpecElListenerElTlsEl {}

impl BuildDataAppmeshVirtualGatewaySpecElListenerElTlsEl {
    pub fn build(self) -> DataAppmeshVirtualGatewaySpecElListenerElTlsEl {
        DataAppmeshVirtualGatewaySpecElListenerElTlsEl {
            certificate: core::default::Default::default(),
            mode: core::default::Default::default(),
            validation: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualGatewaySpecElListenerElTlsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualGatewaySpecElListenerElTlsElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshVirtualGatewaySpecElListenerElTlsElRef {
        DataAppmeshVirtualGatewaySpecElListenerElTlsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualGatewaySpecElListenerElTlsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificate` after provisioning.\n"]
    pub fn certificate(&self) -> ListRef<DataAppmeshVirtualGatewaySpecElListenerElTlsElCertificateElRef> {
        ListRef::new(self.shared().clone(), format!("{}.certificate", self.base))
    }

    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc = "Get a reference to the value of field `validation` after provisioning.\n"]
    pub fn validation(&self) -> ListRef<DataAppmeshVirtualGatewaySpecElListenerElTlsElValidationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.validation", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualGatewaySpecElListenerEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    connection_pool: Option<ListField<DataAppmeshVirtualGatewaySpecElListenerElConnectionPoolEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    health_check: Option<ListField<DataAppmeshVirtualGatewaySpecElListenerElHealthCheckEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port_mapping: Option<ListField<DataAppmeshVirtualGatewaySpecElListenerElPortMappingEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tls: Option<ListField<DataAppmeshVirtualGatewaySpecElListenerElTlsEl>>,
}

impl DataAppmeshVirtualGatewaySpecElListenerEl {
    #[doc = "Set the field `connection_pool`.\n"]
    pub fn set_connection_pool(
        mut self,
        v: impl Into<ListField<DataAppmeshVirtualGatewaySpecElListenerElConnectionPoolEl>>,
    ) -> Self {
        self.connection_pool = Some(v.into());
        self
    }

    #[doc = "Set the field `health_check`.\n"]
    pub fn set_health_check(
        mut self,
        v: impl Into<ListField<DataAppmeshVirtualGatewaySpecElListenerElHealthCheckEl>>,
    ) -> Self {
        self.health_check = Some(v.into());
        self
    }

    #[doc = "Set the field `port_mapping`.\n"]
    pub fn set_port_mapping(
        mut self,
        v: impl Into<ListField<DataAppmeshVirtualGatewaySpecElListenerElPortMappingEl>>,
    ) -> Self {
        self.port_mapping = Some(v.into());
        self
    }

    #[doc = "Set the field `tls`.\n"]
    pub fn set_tls(mut self, v: impl Into<ListField<DataAppmeshVirtualGatewaySpecElListenerElTlsEl>>) -> Self {
        self.tls = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualGatewaySpecElListenerEl {
    type O = BlockAssignable<DataAppmeshVirtualGatewaySpecElListenerEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualGatewaySpecElListenerEl {}

impl BuildDataAppmeshVirtualGatewaySpecElListenerEl {
    pub fn build(self) -> DataAppmeshVirtualGatewaySpecElListenerEl {
        DataAppmeshVirtualGatewaySpecElListenerEl {
            connection_pool: core::default::Default::default(),
            health_check: core::default::Default::default(),
            port_mapping: core::default::Default::default(),
            tls: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualGatewaySpecElListenerElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualGatewaySpecElListenerElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshVirtualGatewaySpecElListenerElRef {
        DataAppmeshVirtualGatewaySpecElListenerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualGatewaySpecElListenerElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `connection_pool` after provisioning.\n"]
    pub fn connection_pool(&self) -> ListRef<DataAppmeshVirtualGatewaySpecElListenerElConnectionPoolElRef> {
        ListRef::new(self.shared().clone(), format!("{}.connection_pool", self.base))
    }

    #[doc = "Get a reference to the value of field `health_check` after provisioning.\n"]
    pub fn health_check(&self) -> ListRef<DataAppmeshVirtualGatewaySpecElListenerElHealthCheckElRef> {
        ListRef::new(self.shared().clone(), format!("{}.health_check", self.base))
    }

    #[doc = "Get a reference to the value of field `port_mapping` after provisioning.\n"]
    pub fn port_mapping(&self) -> ListRef<DataAppmeshVirtualGatewaySpecElListenerElPortMappingElRef> {
        ListRef::new(self.shared().clone(), format!("{}.port_mapping", self.base))
    }

    #[doc = "Get a reference to the value of field `tls` after provisioning.\n"]
    pub fn tls(&self) -> ListRef<DataAppmeshVirtualGatewaySpecElListenerElTlsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tls", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualGatewaySpecElLoggingElAccessLogElFileElFormatElJsonEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}

impl DataAppmeshVirtualGatewaySpecElLoggingElAccessLogElFileElFormatElJsonEl {
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

impl ToListMappable for DataAppmeshVirtualGatewaySpecElLoggingElAccessLogElFileElFormatElJsonEl {
    type O = BlockAssignable<DataAppmeshVirtualGatewaySpecElLoggingElAccessLogElFileElFormatElJsonEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualGatewaySpecElLoggingElAccessLogElFileElFormatElJsonEl {}

impl BuildDataAppmeshVirtualGatewaySpecElLoggingElAccessLogElFileElFormatElJsonEl {
    pub fn build(self) -> DataAppmeshVirtualGatewaySpecElLoggingElAccessLogElFileElFormatElJsonEl {
        DataAppmeshVirtualGatewaySpecElLoggingElAccessLogElFileElFormatElJsonEl {
            key: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualGatewaySpecElLoggingElAccessLogElFileElFormatElJsonElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualGatewaySpecElLoggingElAccessLogElFileElFormatElJsonElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAppmeshVirtualGatewaySpecElLoggingElAccessLogElFileElFormatElJsonElRef {
        DataAppmeshVirtualGatewaySpecElLoggingElAccessLogElFileElFormatElJsonElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualGatewaySpecElLoggingElAccessLogElFileElFormatElJsonElRef {
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
pub struct DataAppmeshVirtualGatewaySpecElLoggingElAccessLogElFileElFormatEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    json: Option<ListField<DataAppmeshVirtualGatewaySpecElLoggingElAccessLogElFileElFormatElJsonEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<PrimField<String>>,
}

impl DataAppmeshVirtualGatewaySpecElLoggingElAccessLogElFileElFormatEl {
    #[doc = "Set the field `json`.\n"]
    pub fn set_json(
        mut self,
        v: impl Into<ListField<DataAppmeshVirtualGatewaySpecElLoggingElAccessLogElFileElFormatElJsonEl>>,
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

impl ToListMappable for DataAppmeshVirtualGatewaySpecElLoggingElAccessLogElFileElFormatEl {
    type O = BlockAssignable<DataAppmeshVirtualGatewaySpecElLoggingElAccessLogElFileElFormatEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualGatewaySpecElLoggingElAccessLogElFileElFormatEl {}

impl BuildDataAppmeshVirtualGatewaySpecElLoggingElAccessLogElFileElFormatEl {
    pub fn build(self) -> DataAppmeshVirtualGatewaySpecElLoggingElAccessLogElFileElFormatEl {
        DataAppmeshVirtualGatewaySpecElLoggingElAccessLogElFileElFormatEl {
            json: core::default::Default::default(),
            text: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualGatewaySpecElLoggingElAccessLogElFileElFormatElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualGatewaySpecElLoggingElAccessLogElFileElFormatElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAppmeshVirtualGatewaySpecElLoggingElAccessLogElFileElFormatElRef {
        DataAppmeshVirtualGatewaySpecElLoggingElAccessLogElFileElFormatElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualGatewaySpecElLoggingElAccessLogElFileElFormatElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `json` after provisioning.\n"]
    pub fn json(&self) -> ListRef<DataAppmeshVirtualGatewaySpecElLoggingElAccessLogElFileElFormatElJsonElRef> {
        ListRef::new(self.shared().clone(), format!("{}.json", self.base))
    }

    #[doc = "Get a reference to the value of field `text` after provisioning.\n"]
    pub fn text(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.text", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualGatewaySpecElLoggingElAccessLogElFileEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    format: Option<ListField<DataAppmeshVirtualGatewaySpecElLoggingElAccessLogElFileElFormatEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
}

impl DataAppmeshVirtualGatewaySpecElLoggingElAccessLogElFileEl {
    #[doc = "Set the field `format`.\n"]
    pub fn set_format(
        mut self,
        v: impl Into<ListField<DataAppmeshVirtualGatewaySpecElLoggingElAccessLogElFileElFormatEl>>,
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

impl ToListMappable for DataAppmeshVirtualGatewaySpecElLoggingElAccessLogElFileEl {
    type O = BlockAssignable<DataAppmeshVirtualGatewaySpecElLoggingElAccessLogElFileEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualGatewaySpecElLoggingElAccessLogElFileEl {}

impl BuildDataAppmeshVirtualGatewaySpecElLoggingElAccessLogElFileEl {
    pub fn build(self) -> DataAppmeshVirtualGatewaySpecElLoggingElAccessLogElFileEl {
        DataAppmeshVirtualGatewaySpecElLoggingElAccessLogElFileEl {
            format: core::default::Default::default(),
            path: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualGatewaySpecElLoggingElAccessLogElFileElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualGatewaySpecElLoggingElAccessLogElFileElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshVirtualGatewaySpecElLoggingElAccessLogElFileElRef {
        DataAppmeshVirtualGatewaySpecElLoggingElAccessLogElFileElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualGatewaySpecElLoggingElAccessLogElFileElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `format` after provisioning.\n"]
    pub fn format(&self) -> ListRef<DataAppmeshVirtualGatewaySpecElLoggingElAccessLogElFileElFormatElRef> {
        ListRef::new(self.shared().clone(), format!("{}.format", self.base))
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualGatewaySpecElLoggingElAccessLogEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    file: Option<ListField<DataAppmeshVirtualGatewaySpecElLoggingElAccessLogElFileEl>>,
}

impl DataAppmeshVirtualGatewaySpecElLoggingElAccessLogEl {
    #[doc = "Set the field `file`.\n"]
    pub fn set_file(
        mut self,
        v: impl Into<ListField<DataAppmeshVirtualGatewaySpecElLoggingElAccessLogElFileEl>>,
    ) -> Self {
        self.file = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualGatewaySpecElLoggingElAccessLogEl {
    type O = BlockAssignable<DataAppmeshVirtualGatewaySpecElLoggingElAccessLogEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualGatewaySpecElLoggingElAccessLogEl {}

impl BuildDataAppmeshVirtualGatewaySpecElLoggingElAccessLogEl {
    pub fn build(self) -> DataAppmeshVirtualGatewaySpecElLoggingElAccessLogEl {
        DataAppmeshVirtualGatewaySpecElLoggingElAccessLogEl { file: core::default::Default::default() }
    }
}

pub struct DataAppmeshVirtualGatewaySpecElLoggingElAccessLogElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualGatewaySpecElLoggingElAccessLogElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshVirtualGatewaySpecElLoggingElAccessLogElRef {
        DataAppmeshVirtualGatewaySpecElLoggingElAccessLogElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualGatewaySpecElLoggingElAccessLogElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `file` after provisioning.\n"]
    pub fn file(&self) -> ListRef<DataAppmeshVirtualGatewaySpecElLoggingElAccessLogElFileElRef> {
        ListRef::new(self.shared().clone(), format!("{}.file", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualGatewaySpecElLoggingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    access_log: Option<ListField<DataAppmeshVirtualGatewaySpecElLoggingElAccessLogEl>>,
}

impl DataAppmeshVirtualGatewaySpecElLoggingEl {
    #[doc = "Set the field `access_log`.\n"]
    pub fn set_access_log(
        mut self,
        v: impl Into<ListField<DataAppmeshVirtualGatewaySpecElLoggingElAccessLogEl>>,
    ) -> Self {
        self.access_log = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualGatewaySpecElLoggingEl {
    type O = BlockAssignable<DataAppmeshVirtualGatewaySpecElLoggingEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualGatewaySpecElLoggingEl {}

impl BuildDataAppmeshVirtualGatewaySpecElLoggingEl {
    pub fn build(self) -> DataAppmeshVirtualGatewaySpecElLoggingEl {
        DataAppmeshVirtualGatewaySpecElLoggingEl { access_log: core::default::Default::default() }
    }
}

pub struct DataAppmeshVirtualGatewaySpecElLoggingElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualGatewaySpecElLoggingElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshVirtualGatewaySpecElLoggingElRef {
        DataAppmeshVirtualGatewaySpecElLoggingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualGatewaySpecElLoggingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `access_log` after provisioning.\n"]
    pub fn access_log(&self) -> ListRef<DataAppmeshVirtualGatewaySpecElLoggingElAccessLogElRef> {
        ListRef::new(self.shared().clone(), format!("{}.access_log", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualGatewaySpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    backend_defaults: Option<ListField<DataAppmeshVirtualGatewaySpecElBackendDefaultsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    listener: Option<ListField<DataAppmeshVirtualGatewaySpecElListenerEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logging: Option<ListField<DataAppmeshVirtualGatewaySpecElLoggingEl>>,
}

impl DataAppmeshVirtualGatewaySpecEl {
    #[doc = "Set the field `backend_defaults`.\n"]
    pub fn set_backend_defaults(
        mut self,
        v: impl Into<ListField<DataAppmeshVirtualGatewaySpecElBackendDefaultsEl>>,
    ) -> Self {
        self.backend_defaults = Some(v.into());
        self
    }

    #[doc = "Set the field `listener`.\n"]
    pub fn set_listener(mut self, v: impl Into<ListField<DataAppmeshVirtualGatewaySpecElListenerEl>>) -> Self {
        self.listener = Some(v.into());
        self
    }

    #[doc = "Set the field `logging`.\n"]
    pub fn set_logging(mut self, v: impl Into<ListField<DataAppmeshVirtualGatewaySpecElLoggingEl>>) -> Self {
        self.logging = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualGatewaySpecEl {
    type O = BlockAssignable<DataAppmeshVirtualGatewaySpecEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualGatewaySpecEl {}

impl BuildDataAppmeshVirtualGatewaySpecEl {
    pub fn build(self) -> DataAppmeshVirtualGatewaySpecEl {
        DataAppmeshVirtualGatewaySpecEl {
            backend_defaults: core::default::Default::default(),
            listener: core::default::Default::default(),
            logging: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualGatewaySpecElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualGatewaySpecElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshVirtualGatewaySpecElRef {
        DataAppmeshVirtualGatewaySpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualGatewaySpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `backend_defaults` after provisioning.\n"]
    pub fn backend_defaults(&self) -> ListRef<DataAppmeshVirtualGatewaySpecElBackendDefaultsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.backend_defaults", self.base))
    }

    #[doc = "Get a reference to the value of field `listener` after provisioning.\n"]
    pub fn listener(&self) -> ListRef<DataAppmeshVirtualGatewaySpecElListenerElRef> {
        ListRef::new(self.shared().clone(), format!("{}.listener", self.base))
    }

    #[doc = "Get a reference to the value of field `logging` after provisioning.\n"]
    pub fn logging(&self) -> ListRef<DataAppmeshVirtualGatewaySpecElLoggingElRef> {
        ListRef::new(self.shared().clone(), format!("{}.logging", self.base))
    }
}
