use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct AppmeshVirtualGatewayData {
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
    spec: Option<Vec<AppmeshVirtualGatewaySpecEl>>,
    dynamic: AppmeshVirtualGatewayDynamic,
}
struct AppmeshVirtualGateway_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<AppmeshVirtualGatewayData>,
}
#[derive(Clone)]
pub struct AppmeshVirtualGateway(Rc<AppmeshVirtualGateway_>);
impl AppmeshVirtualGateway {
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
    #[doc = "Set the field `mesh_owner`.\n"]
    pub fn set_mesh_owner(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().mesh_owner = Some(v.into());
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
    #[doc = "Set the field `spec`.\n"]
    pub fn set_spec(self, v: impl Into<BlockAssignable<AppmeshVirtualGatewaySpecEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().spec = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.spec = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `created_date` after provisioning.\n"]
    pub fn created_date(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created_date", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `last_updated_date` after provisioning.\n"]
    pub fn last_updated_date(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_updated_date", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `mesh_name` after provisioning.\n"]
    pub fn mesh_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.mesh_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `mesh_owner` after provisioning.\n"]
    pub fn mesh_owner(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.mesh_owner", self.extract_ref()),
        )
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
    #[doc = "Get a reference to the value of field `resource_owner` after provisioning.\n"]
    pub fn resource_owner(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_owner", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `spec` after provisioning.\n"]
    pub fn spec(&self) -> ListRef<AppmeshVirtualGatewaySpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.spec", self.extract_ref()),
        )
    }
}
impl Referable for AppmeshVirtualGateway {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for AppmeshVirtualGateway {}
impl ToListMappable for AppmeshVirtualGateway {
    type O = ListRef<AppmeshVirtualGatewayRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for AppmeshVirtualGateway_ {
    fn extract_resource_type(&self) -> String {
        "aws_appmesh_virtual_gateway".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildAppmeshVirtualGateway {
    pub tf_id: String,
    #[doc = ""]
    pub mesh_name: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
}
impl BuildAppmeshVirtualGateway {
    pub fn build(self, stack: &mut Stack) -> AppmeshVirtualGateway {
        let out = AppmeshVirtualGateway(Rc::new(AppmeshVirtualGateway_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(AppmeshVirtualGatewayData {
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
pub struct AppmeshVirtualGatewayRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppmeshVirtualGatewayRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl AppmeshVirtualGatewayRef {
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
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created_date", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `last_updated_date` after provisioning.\n"]
    pub fn last_updated_date(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_updated_date", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `mesh_name` after provisioning.\n"]
    pub fn mesh_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.mesh_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `mesh_owner` after provisioning.\n"]
    pub fn mesh_owner(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.mesh_owner", self.extract_ref()),
        )
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
    #[doc = "Get a reference to the value of field `resource_owner` after provisioning.\n"]
    pub fn resource_owner(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_owner", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `spec` after provisioning.\n"]
    pub fn spec(&self) -> ListRef<AppmeshVirtualGatewaySpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.spec", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileEl {
    certificate_chain: PrimField<String>,
    private_key: PrimField<String>,
}
impl AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileEl {}
impl ToListMappable
    for AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileEl
{
    type O = BlockAssignable<
        AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileEl {
    #[doc = ""]
    pub certificate_chain: PrimField<String>,
    #[doc = ""]
    pub private_key: PrimField<String>,
}
impl BuildAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileEl {
    pub fn build(
        self,
    ) -> AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileEl {
        AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileEl {
            certificate_chain: self.certificate_chain,
            private_key: self.private_key,
        }
    }
}
pub struct AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileElRef {
        AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `certificate_chain` after provisioning.\n"]
    pub fn certificate_chain(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.certificate_chain", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `private_key` after provisioning.\n"]
    pub fn private_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.private_key", self.base))
    }
}
#[derive(Serialize)]
pub struct AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsEl {
    secret_name: PrimField<String>,
}
impl AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsEl {}
impl ToListMappable
    for AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsEl
{
    type O = BlockAssignable<
        AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsEl {
    #[doc = ""]
    pub secret_name: PrimField<String>,
}
impl BuildAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsEl {
    pub fn build(
        self,
    ) -> AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsEl {
        AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsEl {
            secret_name: self.secret_name,
        }
    }
}
pub struct AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsElRef {
        AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `secret_name` after provisioning.\n"]
    pub fn secret_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret_name", self.base))
    }
}
#[derive(Serialize, Default)]
struct AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElDynamic {
    file: Option<
        DynamicBlock<
            AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileEl,
        >,
    >,
    sds: Option<
        DynamicBlock<
            AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    file: Option<
        Vec<AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    sds: Option<
        Vec<AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsEl>,
    >,
    dynamic: AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElDynamic,
}
impl AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateEl {
    #[doc = "Set the field `file`.\n"]
    pub fn set_file(
        mut self,
        v: impl Into<
            BlockAssignable<
                AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.file = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.file = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `sds`.\n"]
    pub fn set_sds(
        mut self,
        v: impl Into<
            BlockAssignable<
                AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.sds = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.sds = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateEl
{
    type O = BlockAssignable<
        AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateEl {}
impl BuildAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateEl {
    pub fn build(
        self,
    ) -> AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateEl {
        AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateEl {
            file: core::default::Default::default(),
            sds: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElRef {
        AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `file` after provisioning.\n"]
    pub fn file(
        &self,
    ) -> ListRef<
        AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.file", self.base))
    }
    #[doc = "Get a reference to the value of field `sds` after provisioning.\n"]
    pub fn sds(
        &self,
    ) -> ListRef<AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.sds", self.base))
    }
}
#[derive(Serialize)]
pub struct AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl
{
    exact: SetField<PrimField<String>>,
}
impl AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl { }
impl ToListMappable for AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl { type O = BlockAssignable < AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl
{
    #[doc = ""]
    pub exact: SetField<PrimField<String>>,
}
impl BuildAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl { pub fn build (self) -> AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl { AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl { exact : self . exact , } } }
pub struct AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchElRef { fn new (shared : StackShared , base : String) -> AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchElRef { AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchElRef { shared : shared , base : base . to_string () , } } }
impl AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `exact` after provisioning.\n"] pub fn exact (& self) -> SetRef < PrimExpr < String > > { SetRef :: new (self . shared () . clone () , format ! ("{}.exact" , self . base)) } }
#[derive(Serialize, Default)]
struct AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElDynamic { match_ : Option < DynamicBlock < AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl >> , }
#[derive(Serialize)]
pub struct AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl { # [serde (rename = "match" , skip_serializing_if = "Option::is_none")] match_ : Option < Vec < AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl > > , dynamic : AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElDynamic , }
impl AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl { # [doc = "Set the field `match_`.\n"] pub fn set_match (mut self , v : impl Into < BlockAssignable < AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . match_ = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . match_ = Some (d) ; } } self } }
impl ToListMappable for AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl { type O = BlockAssignable < AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl
{}
impl BuildAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl { pub fn build (self) -> AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl { AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl { match_ : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElRef { fn new (shared : StackShared , base : String) -> AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElRef { AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElRef { shared : shared , base : base . to_string () , } } }
impl AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `match_` after provisioning.\n"] pub fn match_ (& self) -> ListRef < AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.match" , self . base)) } }
#[derive(Serialize)]
pub struct AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmEl {
    certificate_authority_arns: SetField<PrimField<String>>,
}
impl AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmEl {}
impl ToListMappable
    for AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmEl
{
    type O = BlockAssignable<
        AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmEl
{
    #[doc = ""]
    pub certificate_authority_arns: SetField<PrimField<String>>,
}
impl BuildAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmEl {
    pub fn build(
        self,
    ) -> AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmEl
    {
        AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmEl {
            certificate_authority_arns: self.certificate_authority_arns,
        }
    }
}
pub struct AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmElRef
    {
        AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `certificate_authority_arns` after provisioning.\n"]
    pub fn certificate_authority_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.certificate_authority_arns", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileEl
{
    certificate_chain: PrimField<String>,
}
impl AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileEl {}
impl ToListMappable
    for AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileEl
{
    type O = BlockAssignable<
        AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileEl
{
    #[doc = ""]
    pub certificate_chain: PrimField<String>,
}
impl BuildAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileEl {
    pub fn build(
        self,
    ) -> AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileEl
    {
        AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileEl {
            certificate_chain: self.certificate_chain,
        }
    }
}
pub struct AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileElRef
    {
        AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileElRef { shared : shared , base : base . to_string () , }
    }
}
impl AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `certificate_chain` after provisioning.\n"]
    pub fn certificate_chain(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.certificate_chain", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsEl {
    secret_name: PrimField<String>,
}
impl AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsEl {}
impl ToListMappable
    for AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsEl
{
    type O = BlockAssignable<
        AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsEl
{
    #[doc = ""]
    pub secret_name: PrimField<String>,
}
impl BuildAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsEl {
    pub fn build(
        self,
    ) -> AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsEl
    {
        AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsEl {
            secret_name: self.secret_name,
        }
    }
}
pub struct AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsElRef
    {
        AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `secret_name` after provisioning.\n"]
    pub fn secret_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret_name", self.base))
    }
}
#[derive(Serialize, Default)]
struct AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElDynamic { acm : Option < DynamicBlock < AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmEl >> , file : Option < DynamicBlock < AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileEl >> , sds : Option < DynamicBlock < AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsEl >> , }
#[derive(Serialize)]
pub struct AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustEl { # [serde (skip_serializing_if = "Option::is_none")] acm : Option < Vec < AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmEl > > , # [serde (skip_serializing_if = "Option::is_none")] file : Option < Vec < AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileEl > > , # [serde (skip_serializing_if = "Option::is_none")] sds : Option < Vec < AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsEl > > , dynamic : AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElDynamic , }
impl AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustEl {
    #[doc = "Set the field `acm`.\n"]
    pub fn set_acm(
        mut self,
        v : impl Into < BlockAssignable < AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.acm = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.acm = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `file`.\n"]
    pub fn set_file(
        mut self,
        v : impl Into < BlockAssignable < AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.file = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.file = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `sds`.\n"]
    pub fn set_sds(
        mut self,
        v : impl Into < BlockAssignable < AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.sds = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.sds = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustEl
{
    type O = BlockAssignable<
        AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustEl
{}
impl BuildAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustEl {
    pub fn build(
        self,
    ) -> AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustEl {
        AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustEl {
            acm: core::default::Default::default(),
            file: core::default::Default::default(),
            sds: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElRef {
        AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `acm` after provisioning.\n"]
    pub fn acm(
        &self,
    ) -> ListRef<
        AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.acm", self.base))
    }
    #[doc = "Get a reference to the value of field `file` after provisioning.\n"]
    pub fn file(
        &self,
    ) -> ListRef<
        AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.file", self.base))
    }
    #[doc = "Get a reference to the value of field `sds` after provisioning.\n"]
    pub fn sds(
        &self,
    ) -> ListRef<
        AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.sds", self.base))
    }
}
#[derive(Serialize, Default)]
struct AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElDynamic { subject_alternative_names : Option < DynamicBlock < AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl >> , trust : Option < DynamicBlock < AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustEl >> , }
#[derive(Serialize)]
pub struct AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationEl { # [serde (skip_serializing_if = "Option::is_none")] subject_alternative_names : Option < Vec < AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl > > , # [serde (skip_serializing_if = "Option::is_none")] trust : Option < Vec < AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustEl > > , dynamic : AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElDynamic , }
impl AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationEl {
    #[doc = "Set the field `subject_alternative_names`.\n"]
    pub fn set_subject_alternative_names(
        mut self,
        v : impl Into < BlockAssignable < AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.subject_alternative_names = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.subject_alternative_names = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `trust`.\n"]
    pub fn set_trust(
        mut self,
        v: impl Into<
            BlockAssignable<
                AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.trust = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.trust = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationEl
{
    type O = BlockAssignable<
        AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationEl {}
impl BuildAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationEl {
    pub fn build(
        self,
    ) -> AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationEl {
        AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationEl {
            subject_alternative_names: core::default::Default::default(),
            trust: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElRef {
        AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `subject_alternative_names` after provisioning.\n"]    pub fn subject_alternative_names (& self) -> ListRef < AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.subject_alternative_names", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `trust` after provisioning.\n"]
    pub fn trust(
        &self,
    ) -> ListRef<
        AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.trust", self.base))
    }
}
#[derive(Serialize, Default)]
struct AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElDynamic {
    certificate: Option<
        DynamicBlock<AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateEl>,
    >,
    validation: Option<
        DynamicBlock<AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationEl>,
    >,
}
#[derive(Serialize)]
pub struct AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enforce: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ports: Option<SetField<PrimField<f64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate:
        Option<Vec<AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    validation:
        Option<Vec<AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationEl>>,
    dynamic: AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElDynamic,
}
impl AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsEl {
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
        v: impl Into<
            BlockAssignable<
                AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.certificate = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.certificate = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `validation`.\n"]
    pub fn set_validation(
        mut self,
        v: impl Into<
            BlockAssignable<
                AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.validation = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.validation = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsEl {
    type O = BlockAssignable<AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsEl {}
impl BuildAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsEl {
    pub fn build(self) -> AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsEl {
        AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsEl {
            enforce: core::default::Default::default(),
            ports: core::default::Default::default(),
            certificate: core::default::Default::default(),
            validation: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElRef {
        AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElRef {
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
    ) -> ListRef<AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.certificate", self.base))
    }
    #[doc = "Get a reference to the value of field `validation` after provisioning.\n"]
    pub fn validation(
        &self,
    ) -> ListRef<AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.validation", self.base))
    }
}
#[derive(Serialize, Default)]
struct AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElDynamic {
    tls: Option<DynamicBlock<AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsEl>>,
}
#[derive(Serialize)]
pub struct AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    tls: Option<Vec<AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsEl>>,
    dynamic: AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElDynamic,
}
impl AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyEl {
    #[doc = "Set the field `tls`.\n"]
    pub fn set_tls(
        mut self,
        v: impl Into<BlockAssignable<AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.tls = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.tls = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyEl {
    type O = BlockAssignable<AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyEl {}
impl BuildAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyEl {
    pub fn build(self) -> AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyEl {
        AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyEl {
            tls: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElRef {
        AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `tls` after provisioning.\n"]
    pub fn tls(
        &self,
    ) -> ListRef<AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tls", self.base))
    }
}
#[derive(Serialize, Default)]
struct AppmeshVirtualGatewaySpecElBackendDefaultsElDynamic {
    client_policy: Option<DynamicBlock<AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyEl>>,
}
#[derive(Serialize)]
pub struct AppmeshVirtualGatewaySpecElBackendDefaultsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    client_policy: Option<Vec<AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyEl>>,
    dynamic: AppmeshVirtualGatewaySpecElBackendDefaultsElDynamic,
}
impl AppmeshVirtualGatewaySpecElBackendDefaultsEl {
    #[doc = "Set the field `client_policy`.\n"]
    pub fn set_client_policy(
        mut self,
        v: impl Into<BlockAssignable<AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.client_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.client_policy = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for AppmeshVirtualGatewaySpecElBackendDefaultsEl {
    type O = BlockAssignable<AppmeshVirtualGatewaySpecElBackendDefaultsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppmeshVirtualGatewaySpecElBackendDefaultsEl {}
impl BuildAppmeshVirtualGatewaySpecElBackendDefaultsEl {
    pub fn build(self) -> AppmeshVirtualGatewaySpecElBackendDefaultsEl {
        AppmeshVirtualGatewaySpecElBackendDefaultsEl {
            client_policy: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AppmeshVirtualGatewaySpecElBackendDefaultsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppmeshVirtualGatewaySpecElBackendDefaultsElRef {
    fn new(shared: StackShared, base: String) -> AppmeshVirtualGatewaySpecElBackendDefaultsElRef {
        AppmeshVirtualGatewaySpecElBackendDefaultsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppmeshVirtualGatewaySpecElBackendDefaultsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `client_policy` after provisioning.\n"]
    pub fn client_policy(
        &self,
    ) -> ListRef<AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.client_policy", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct AppmeshVirtualGatewaySpecElListenerElConnectionPoolElGrpcEl {
    max_requests: PrimField<f64>,
}
impl AppmeshVirtualGatewaySpecElListenerElConnectionPoolElGrpcEl {}
impl ToListMappable for AppmeshVirtualGatewaySpecElListenerElConnectionPoolElGrpcEl {
    type O = BlockAssignable<AppmeshVirtualGatewaySpecElListenerElConnectionPoolElGrpcEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppmeshVirtualGatewaySpecElListenerElConnectionPoolElGrpcEl {
    #[doc = ""]
    pub max_requests: PrimField<f64>,
}
impl BuildAppmeshVirtualGatewaySpecElListenerElConnectionPoolElGrpcEl {
    pub fn build(self) -> AppmeshVirtualGatewaySpecElListenerElConnectionPoolElGrpcEl {
        AppmeshVirtualGatewaySpecElListenerElConnectionPoolElGrpcEl {
            max_requests: self.max_requests,
        }
    }
}
pub struct AppmeshVirtualGatewaySpecElListenerElConnectionPoolElGrpcElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppmeshVirtualGatewaySpecElListenerElConnectionPoolElGrpcElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppmeshVirtualGatewaySpecElListenerElConnectionPoolElGrpcElRef {
        AppmeshVirtualGatewaySpecElListenerElConnectionPoolElGrpcElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppmeshVirtualGatewaySpecElListenerElConnectionPoolElGrpcElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `max_requests` after provisioning.\n"]
    pub fn max_requests(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_requests", self.base))
    }
}
#[derive(Serialize)]
pub struct AppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttpEl {
    max_connections: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_pending_requests: Option<PrimField<f64>>,
}
impl AppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttpEl {
    #[doc = "Set the field `max_pending_requests`.\n"]
    pub fn set_max_pending_requests(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_pending_requests = Some(v.into());
        self
    }
}
impl ToListMappable for AppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttpEl {
    type O = BlockAssignable<AppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttpEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttpEl {
    #[doc = ""]
    pub max_connections: PrimField<f64>,
}
impl BuildAppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttpEl {
    pub fn build(self) -> AppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttpEl {
        AppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttpEl {
            max_connections: self.max_connections,
            max_pending_requests: core::default::Default::default(),
        }
    }
}
pub struct AppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttpElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttpElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttpElRef {
        AppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttpElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttpElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `max_connections` after provisioning.\n"]
    pub fn max_connections(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_connections", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_pending_requests` after provisioning.\n"]
    pub fn max_pending_requests(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_pending_requests", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct AppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttp2El {
    max_requests: PrimField<f64>,
}
impl AppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttp2El {}
impl ToListMappable for AppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttp2El {
    type O = BlockAssignable<AppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttp2El>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttp2El {
    #[doc = ""]
    pub max_requests: PrimField<f64>,
}
impl BuildAppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttp2El {
    pub fn build(self) -> AppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttp2El {
        AppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttp2El {
            max_requests: self.max_requests,
        }
    }
}
pub struct AppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttp2ElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttp2ElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttp2ElRef {
        AppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttp2ElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttp2ElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `max_requests` after provisioning.\n"]
    pub fn max_requests(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_requests", self.base))
    }
}
#[derive(Serialize, Default)]
struct AppmeshVirtualGatewaySpecElListenerElConnectionPoolElDynamic {
    grpc: Option<DynamicBlock<AppmeshVirtualGatewaySpecElListenerElConnectionPoolElGrpcEl>>,
    http: Option<DynamicBlock<AppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttpEl>>,
    http2: Option<DynamicBlock<AppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttp2El>>,
}
#[derive(Serialize)]
pub struct AppmeshVirtualGatewaySpecElListenerElConnectionPoolEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    grpc: Option<Vec<AppmeshVirtualGatewaySpecElListenerElConnectionPoolElGrpcEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http: Option<Vec<AppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttpEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http2: Option<Vec<AppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttp2El>>,
    dynamic: AppmeshVirtualGatewaySpecElListenerElConnectionPoolElDynamic,
}
impl AppmeshVirtualGatewaySpecElListenerElConnectionPoolEl {
    #[doc = "Set the field `grpc`.\n"]
    pub fn set_grpc(
        mut self,
        v: impl Into<BlockAssignable<AppmeshVirtualGatewaySpecElListenerElConnectionPoolElGrpcEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.grpc = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.grpc = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `http`.\n"]
    pub fn set_http(
        mut self,
        v: impl Into<BlockAssignable<AppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttpEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.http = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.http = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `http2`.\n"]
    pub fn set_http2(
        mut self,
        v: impl Into<BlockAssignable<AppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttp2El>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.http2 = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.http2 = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for AppmeshVirtualGatewaySpecElListenerElConnectionPoolEl {
    type O = BlockAssignable<AppmeshVirtualGatewaySpecElListenerElConnectionPoolEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppmeshVirtualGatewaySpecElListenerElConnectionPoolEl {}
impl BuildAppmeshVirtualGatewaySpecElListenerElConnectionPoolEl {
    pub fn build(self) -> AppmeshVirtualGatewaySpecElListenerElConnectionPoolEl {
        AppmeshVirtualGatewaySpecElListenerElConnectionPoolEl {
            grpc: core::default::Default::default(),
            http: core::default::Default::default(),
            http2: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AppmeshVirtualGatewaySpecElListenerElConnectionPoolElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppmeshVirtualGatewaySpecElListenerElConnectionPoolElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppmeshVirtualGatewaySpecElListenerElConnectionPoolElRef {
        AppmeshVirtualGatewaySpecElListenerElConnectionPoolElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppmeshVirtualGatewaySpecElListenerElConnectionPoolElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `grpc` after provisioning.\n"]
    pub fn grpc(&self) -> ListRef<AppmeshVirtualGatewaySpecElListenerElConnectionPoolElGrpcElRef> {
        ListRef::new(self.shared().clone(), format!("{}.grpc", self.base))
    }
    #[doc = "Get a reference to the value of field `http` after provisioning.\n"]
    pub fn http(&self) -> ListRef<AppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttpElRef> {
        ListRef::new(self.shared().clone(), format!("{}.http", self.base))
    }
    #[doc = "Get a reference to the value of field `http2` after provisioning.\n"]
    pub fn http2(
        &self,
    ) -> ListRef<AppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttp2ElRef> {
        ListRef::new(self.shared().clone(), format!("{}.http2", self.base))
    }
}
#[derive(Serialize)]
pub struct AppmeshVirtualGatewaySpecElListenerElHealthCheckEl {
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
impl AppmeshVirtualGatewaySpecElListenerElHealthCheckEl {
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
impl ToListMappable for AppmeshVirtualGatewaySpecElListenerElHealthCheckEl {
    type O = BlockAssignable<AppmeshVirtualGatewaySpecElListenerElHealthCheckEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppmeshVirtualGatewaySpecElListenerElHealthCheckEl {
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
impl BuildAppmeshVirtualGatewaySpecElListenerElHealthCheckEl {
    pub fn build(self) -> AppmeshVirtualGatewaySpecElListenerElHealthCheckEl {
        AppmeshVirtualGatewaySpecElListenerElHealthCheckEl {
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
pub struct AppmeshVirtualGatewaySpecElListenerElHealthCheckElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppmeshVirtualGatewaySpecElListenerElHealthCheckElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppmeshVirtualGatewaySpecElListenerElHealthCheckElRef {
        AppmeshVirtualGatewaySpecElListenerElHealthCheckElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppmeshVirtualGatewaySpecElListenerElHealthCheckElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `healthy_threshold` after provisioning.\n"]
    pub fn healthy_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.healthy_threshold", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `interval_millis` after provisioning.\n"]
    pub fn interval_millis(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.interval_millis", self.base),
        )
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
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.timeout_millis", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `unhealthy_threshold` after provisioning.\n"]
    pub fn unhealthy_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.unhealthy_threshold", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct AppmeshVirtualGatewaySpecElListenerElPortMappingEl {
    port: PrimField<f64>,
    protocol: PrimField<String>,
}
impl AppmeshVirtualGatewaySpecElListenerElPortMappingEl {}
impl ToListMappable for AppmeshVirtualGatewaySpecElListenerElPortMappingEl {
    type O = BlockAssignable<AppmeshVirtualGatewaySpecElListenerElPortMappingEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppmeshVirtualGatewaySpecElListenerElPortMappingEl {
    #[doc = ""]
    pub port: PrimField<f64>,
    #[doc = ""]
    pub protocol: PrimField<String>,
}
impl BuildAppmeshVirtualGatewaySpecElListenerElPortMappingEl {
    pub fn build(self) -> AppmeshVirtualGatewaySpecElListenerElPortMappingEl {
        AppmeshVirtualGatewaySpecElListenerElPortMappingEl {
            port: self.port,
            protocol: self.protocol,
        }
    }
}
pub struct AppmeshVirtualGatewaySpecElListenerElPortMappingElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppmeshVirtualGatewaySpecElListenerElPortMappingElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppmeshVirtualGatewaySpecElListenerElPortMappingElRef {
        AppmeshVirtualGatewaySpecElListenerElPortMappingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppmeshVirtualGatewaySpecElListenerElPortMappingElRef {
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
pub struct AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElAcmEl {
    certificate_arn: PrimField<String>,
}
impl AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElAcmEl {}
impl ToListMappable for AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElAcmEl {
    type O = BlockAssignable<AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElAcmEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppmeshVirtualGatewaySpecElListenerElTlsElCertificateElAcmEl {
    #[doc = ""]
    pub certificate_arn: PrimField<String>,
}
impl BuildAppmeshVirtualGatewaySpecElListenerElTlsElCertificateElAcmEl {
    pub fn build(self) -> AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElAcmEl {
        AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElAcmEl {
            certificate_arn: self.certificate_arn,
        }
    }
}
pub struct AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElAcmElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElAcmElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElAcmElRef {
        AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElAcmElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElAcmElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `certificate_arn` after provisioning.\n"]
    pub fn certificate_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.certificate_arn", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElFileEl {
    certificate_chain: PrimField<String>,
    private_key: PrimField<String>,
}
impl AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElFileEl {}
impl ToListMappable for AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElFileEl {
    type O = BlockAssignable<AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElFileEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppmeshVirtualGatewaySpecElListenerElTlsElCertificateElFileEl {
    #[doc = ""]
    pub certificate_chain: PrimField<String>,
    #[doc = ""]
    pub private_key: PrimField<String>,
}
impl BuildAppmeshVirtualGatewaySpecElListenerElTlsElCertificateElFileEl {
    pub fn build(self) -> AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElFileEl {
        AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElFileEl {
            certificate_chain: self.certificate_chain,
            private_key: self.private_key,
        }
    }
}
pub struct AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElFileElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElFileElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElFileElRef {
        AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElFileElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElFileElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `certificate_chain` after provisioning.\n"]
    pub fn certificate_chain(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.certificate_chain", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `private_key` after provisioning.\n"]
    pub fn private_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.private_key", self.base))
    }
}
#[derive(Serialize)]
pub struct AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElSdsEl {
    secret_name: PrimField<String>,
}
impl AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElSdsEl {}
impl ToListMappable for AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElSdsEl {
    type O = BlockAssignable<AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElSdsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppmeshVirtualGatewaySpecElListenerElTlsElCertificateElSdsEl {
    #[doc = ""]
    pub secret_name: PrimField<String>,
}
impl BuildAppmeshVirtualGatewaySpecElListenerElTlsElCertificateElSdsEl {
    pub fn build(self) -> AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElSdsEl {
        AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElSdsEl {
            secret_name: self.secret_name,
        }
    }
}
pub struct AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElSdsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElSdsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElSdsElRef {
        AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElSdsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElSdsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `secret_name` after provisioning.\n"]
    pub fn secret_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret_name", self.base))
    }
}
#[derive(Serialize, Default)]
struct AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElDynamic {
    acm: Option<DynamicBlock<AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElAcmEl>>,
    file: Option<DynamicBlock<AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElFileEl>>,
    sds: Option<DynamicBlock<AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElSdsEl>>,
}
#[derive(Serialize)]
pub struct AppmeshVirtualGatewaySpecElListenerElTlsElCertificateEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    acm: Option<Vec<AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElAcmEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file: Option<Vec<AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElFileEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sds: Option<Vec<AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElSdsEl>>,
    dynamic: AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElDynamic,
}
impl AppmeshVirtualGatewaySpecElListenerElTlsElCertificateEl {
    #[doc = "Set the field `acm`.\n"]
    pub fn set_acm(
        mut self,
        v: impl Into<BlockAssignable<AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElAcmEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.acm = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.acm = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `file`.\n"]
    pub fn set_file(
        mut self,
        v: impl Into<BlockAssignable<AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElFileEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.file = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.file = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `sds`.\n"]
    pub fn set_sds(
        mut self,
        v: impl Into<BlockAssignable<AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElSdsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.sds = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.sds = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for AppmeshVirtualGatewaySpecElListenerElTlsElCertificateEl {
    type O = BlockAssignable<AppmeshVirtualGatewaySpecElListenerElTlsElCertificateEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppmeshVirtualGatewaySpecElListenerElTlsElCertificateEl {}
impl BuildAppmeshVirtualGatewaySpecElListenerElTlsElCertificateEl {
    pub fn build(self) -> AppmeshVirtualGatewaySpecElListenerElTlsElCertificateEl {
        AppmeshVirtualGatewaySpecElListenerElTlsElCertificateEl {
            acm: core::default::Default::default(),
            file: core::default::Default::default(),
            sds: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElRef {
        AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `acm` after provisioning.\n"]
    pub fn acm(&self) -> ListRef<AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElAcmElRef> {
        ListRef::new(self.shared().clone(), format!("{}.acm", self.base))
    }
    #[doc = "Get a reference to the value of field `file` after provisioning.\n"]
    pub fn file(
        &self,
    ) -> ListRef<AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElFileElRef> {
        ListRef::new(self.shared().clone(), format!("{}.file", self.base))
    }
    #[doc = "Get a reference to the value of field `sds` after provisioning.\n"]
    pub fn sds(&self) -> ListRef<AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElSdsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sds", self.base))
    }
}
#[derive(Serialize)]
pub struct AppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchEl {
    exact: SetField<PrimField<String>>,
}
impl AppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchEl {}
impl ToListMappable
    for AppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchEl
{
    type O = BlockAssignable<
        AppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchEl
{
    #[doc = ""]
    pub exact: SetField<PrimField<String>>,
}
impl BuildAppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchEl {
    pub fn build(
        self,
    ) -> AppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchEl
    {
        AppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchEl {
            exact: self.exact,
        }
    }
}
pub struct AppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for AppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchElRef
    {
        AppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `exact` after provisioning.\n"]
    pub fn exact(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.exact", self.base))
    }
}
#[derive(Serialize, Default)]
struct AppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesElDynamic {
    match_: Option<
        DynamicBlock<
            AppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct AppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesEl {
    #[serde(rename = "match", skip_serializing_if = "Option::is_none")]
    match_: Option<
        Vec<AppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchEl>,
    >,
    dynamic: AppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesElDynamic,
}
impl AppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesEl {
    #[doc = "Set the field `match_`.\n"]
    pub fn set_match(
        mut self,
        v : impl Into < BlockAssignable < AppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.match_ = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.match_ = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for AppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesEl
{
    type O = BlockAssignable<
        AppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesEl {}
impl BuildAppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesEl {
    pub fn build(
        self,
    ) -> AppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesEl {
        AppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesEl {
            match_: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesElRef {
        AppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(
        &self,
    ) -> ListRef<
        AppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }
}
#[derive(Serialize)]
pub struct AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElFileEl {
    certificate_chain: PrimField<String>,
}
impl AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElFileEl {}
impl ToListMappable for AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElFileEl {
    type O = BlockAssignable<AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElFileEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElFileEl {
    #[doc = ""]
    pub certificate_chain: PrimField<String>,
}
impl BuildAppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElFileEl {
    pub fn build(self) -> AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElFileEl {
        AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElFileEl {
            certificate_chain: self.certificate_chain,
        }
    }
}
pub struct AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElFileElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElFileElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElFileElRef {
        AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElFileElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElFileElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `certificate_chain` after provisioning.\n"]
    pub fn certificate_chain(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.certificate_chain", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElSdsEl {
    secret_name: PrimField<String>,
}
impl AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElSdsEl {}
impl ToListMappable for AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElSdsEl {
    type O = BlockAssignable<AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElSdsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElSdsEl {
    #[doc = ""]
    pub secret_name: PrimField<String>,
}
impl BuildAppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElSdsEl {
    pub fn build(self) -> AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElSdsEl {
        AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElSdsEl {
            secret_name: self.secret_name,
        }
    }
}
pub struct AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElSdsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElSdsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElSdsElRef {
        AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElSdsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElSdsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `secret_name` after provisioning.\n"]
    pub fn secret_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret_name", self.base))
    }
}
#[derive(Serialize, Default)]
struct AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElDynamic {
    file: Option<DynamicBlock<AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElFileEl>>,
    sds: Option<DynamicBlock<AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElSdsEl>>,
}
#[derive(Serialize)]
pub struct AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    file: Option<Vec<AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElFileEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sds: Option<Vec<AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElSdsEl>>,
    dynamic: AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElDynamic,
}
impl AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustEl {
    #[doc = "Set the field `file`.\n"]
    pub fn set_file(
        mut self,
        v: impl Into<
            BlockAssignable<AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElFileEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.file = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.file = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `sds`.\n"]
    pub fn set_sds(
        mut self,
        v: impl Into<
            BlockAssignable<AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElSdsEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.sds = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.sds = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustEl {
    type O = BlockAssignable<AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustEl {}
impl BuildAppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustEl {
    pub fn build(self) -> AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustEl {
        AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustEl {
            file: core::default::Default::default(),
            sds: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElRef {
        AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `file` after provisioning.\n"]
    pub fn file(
        &self,
    ) -> ListRef<AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElFileElRef> {
        ListRef::new(self.shared().clone(), format!("{}.file", self.base))
    }
    #[doc = "Get a reference to the value of field `sds` after provisioning.\n"]
    pub fn sds(
        &self,
    ) -> ListRef<AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElSdsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sds", self.base))
    }
}
#[derive(Serialize, Default)]
struct AppmeshVirtualGatewaySpecElListenerElTlsElValidationElDynamic {
    subject_alternative_names: Option<
        DynamicBlock<
            AppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesEl,
        >,
    >,
    trust: Option<DynamicBlock<AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustEl>>,
}
#[derive(Serialize)]
pub struct AppmeshVirtualGatewaySpecElListenerElTlsElValidationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    subject_alternative_names: Option<
        Vec<AppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    trust: Option<Vec<AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustEl>>,
    dynamic: AppmeshVirtualGatewaySpecElListenerElTlsElValidationElDynamic,
}
impl AppmeshVirtualGatewaySpecElListenerElTlsElValidationEl {
    #[doc = "Set the field `subject_alternative_names`.\n"]
    pub fn set_subject_alternative_names(
        mut self,
        v: impl Into<
            BlockAssignable<
                AppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.subject_alternative_names = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.subject_alternative_names = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `trust`.\n"]
    pub fn set_trust(
        mut self,
        v: impl Into<BlockAssignable<AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.trust = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.trust = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for AppmeshVirtualGatewaySpecElListenerElTlsElValidationEl {
    type O = BlockAssignable<AppmeshVirtualGatewaySpecElListenerElTlsElValidationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppmeshVirtualGatewaySpecElListenerElTlsElValidationEl {}
impl BuildAppmeshVirtualGatewaySpecElListenerElTlsElValidationEl {
    pub fn build(self) -> AppmeshVirtualGatewaySpecElListenerElTlsElValidationEl {
        AppmeshVirtualGatewaySpecElListenerElTlsElValidationEl {
            subject_alternative_names: core::default::Default::default(),
            trust: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AppmeshVirtualGatewaySpecElListenerElTlsElValidationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppmeshVirtualGatewaySpecElListenerElTlsElValidationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppmeshVirtualGatewaySpecElListenerElTlsElValidationElRef {
        AppmeshVirtualGatewaySpecElListenerElTlsElValidationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppmeshVirtualGatewaySpecElListenerElTlsElValidationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `subject_alternative_names` after provisioning.\n"]
    pub fn subject_alternative_names(
        &self,
    ) -> ListRef<AppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.subject_alternative_names", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `trust` after provisioning.\n"]
    pub fn trust(
        &self,
    ) -> ListRef<AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElRef> {
        ListRef::new(self.shared().clone(), format!("{}.trust", self.base))
    }
}
#[derive(Serialize, Default)]
struct AppmeshVirtualGatewaySpecElListenerElTlsElDynamic {
    certificate: Option<DynamicBlock<AppmeshVirtualGatewaySpecElListenerElTlsElCertificateEl>>,
    validation: Option<DynamicBlock<AppmeshVirtualGatewaySpecElListenerElTlsElValidationEl>>,
}
#[derive(Serialize)]
pub struct AppmeshVirtualGatewaySpecElListenerElTlsEl {
    mode: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate: Option<Vec<AppmeshVirtualGatewaySpecElListenerElTlsElCertificateEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    validation: Option<Vec<AppmeshVirtualGatewaySpecElListenerElTlsElValidationEl>>,
    dynamic: AppmeshVirtualGatewaySpecElListenerElTlsElDynamic,
}
impl AppmeshVirtualGatewaySpecElListenerElTlsEl {
    #[doc = "Set the field `certificate`.\n"]
    pub fn set_certificate(
        mut self,
        v: impl Into<BlockAssignable<AppmeshVirtualGatewaySpecElListenerElTlsElCertificateEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.certificate = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.certificate = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `validation`.\n"]
    pub fn set_validation(
        mut self,
        v: impl Into<BlockAssignable<AppmeshVirtualGatewaySpecElListenerElTlsElValidationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.validation = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.validation = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for AppmeshVirtualGatewaySpecElListenerElTlsEl {
    type O = BlockAssignable<AppmeshVirtualGatewaySpecElListenerElTlsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppmeshVirtualGatewaySpecElListenerElTlsEl {
    #[doc = ""]
    pub mode: PrimField<String>,
}
impl BuildAppmeshVirtualGatewaySpecElListenerElTlsEl {
    pub fn build(self) -> AppmeshVirtualGatewaySpecElListenerElTlsEl {
        AppmeshVirtualGatewaySpecElListenerElTlsEl {
            mode: self.mode,
            certificate: core::default::Default::default(),
            validation: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AppmeshVirtualGatewaySpecElListenerElTlsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppmeshVirtualGatewaySpecElListenerElTlsElRef {
    fn new(shared: StackShared, base: String) -> AppmeshVirtualGatewaySpecElListenerElTlsElRef {
        AppmeshVirtualGatewaySpecElListenerElTlsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppmeshVirtualGatewaySpecElListenerElTlsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
    #[doc = "Get a reference to the value of field `certificate` after provisioning.\n"]
    pub fn certificate(
        &self,
    ) -> ListRef<AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElRef> {
        ListRef::new(self.shared().clone(), format!("{}.certificate", self.base))
    }
    #[doc = "Get a reference to the value of field `validation` after provisioning.\n"]
    pub fn validation(&self) -> ListRef<AppmeshVirtualGatewaySpecElListenerElTlsElValidationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.validation", self.base))
    }
}
#[derive(Serialize, Default)]
struct AppmeshVirtualGatewaySpecElListenerElDynamic {
    connection_pool: Option<DynamicBlock<AppmeshVirtualGatewaySpecElListenerElConnectionPoolEl>>,
    health_check: Option<DynamicBlock<AppmeshVirtualGatewaySpecElListenerElHealthCheckEl>>,
    port_mapping: Option<DynamicBlock<AppmeshVirtualGatewaySpecElListenerElPortMappingEl>>,
    tls: Option<DynamicBlock<AppmeshVirtualGatewaySpecElListenerElTlsEl>>,
}
#[derive(Serialize)]
pub struct AppmeshVirtualGatewaySpecElListenerEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    connection_pool: Option<Vec<AppmeshVirtualGatewaySpecElListenerElConnectionPoolEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    health_check: Option<Vec<AppmeshVirtualGatewaySpecElListenerElHealthCheckEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port_mapping: Option<Vec<AppmeshVirtualGatewaySpecElListenerElPortMappingEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tls: Option<Vec<AppmeshVirtualGatewaySpecElListenerElTlsEl>>,
    dynamic: AppmeshVirtualGatewaySpecElListenerElDynamic,
}
impl AppmeshVirtualGatewaySpecElListenerEl {
    #[doc = "Set the field `connection_pool`.\n"]
    pub fn set_connection_pool(
        mut self,
        v: impl Into<BlockAssignable<AppmeshVirtualGatewaySpecElListenerElConnectionPoolEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.connection_pool = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.connection_pool = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `health_check`.\n"]
    pub fn set_health_check(
        mut self,
        v: impl Into<BlockAssignable<AppmeshVirtualGatewaySpecElListenerElHealthCheckEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.health_check = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.health_check = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `port_mapping`.\n"]
    pub fn set_port_mapping(
        mut self,
        v: impl Into<BlockAssignable<AppmeshVirtualGatewaySpecElListenerElPortMappingEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.port_mapping = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.port_mapping = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `tls`.\n"]
    pub fn set_tls(
        mut self,
        v: impl Into<BlockAssignable<AppmeshVirtualGatewaySpecElListenerElTlsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.tls = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.tls = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for AppmeshVirtualGatewaySpecElListenerEl {
    type O = BlockAssignable<AppmeshVirtualGatewaySpecElListenerEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppmeshVirtualGatewaySpecElListenerEl {}
impl BuildAppmeshVirtualGatewaySpecElListenerEl {
    pub fn build(self) -> AppmeshVirtualGatewaySpecElListenerEl {
        AppmeshVirtualGatewaySpecElListenerEl {
            connection_pool: core::default::Default::default(),
            health_check: core::default::Default::default(),
            port_mapping: core::default::Default::default(),
            tls: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AppmeshVirtualGatewaySpecElListenerElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppmeshVirtualGatewaySpecElListenerElRef {
    fn new(shared: StackShared, base: String) -> AppmeshVirtualGatewaySpecElListenerElRef {
        AppmeshVirtualGatewaySpecElListenerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppmeshVirtualGatewaySpecElListenerElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `connection_pool` after provisioning.\n"]
    pub fn connection_pool(
        &self,
    ) -> ListRef<AppmeshVirtualGatewaySpecElListenerElConnectionPoolElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.connection_pool", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `health_check` after provisioning.\n"]
    pub fn health_check(&self) -> ListRef<AppmeshVirtualGatewaySpecElListenerElHealthCheckElRef> {
        ListRef::new(self.shared().clone(), format!("{}.health_check", self.base))
    }
    #[doc = "Get a reference to the value of field `port_mapping` after provisioning.\n"]
    pub fn port_mapping(&self) -> ListRef<AppmeshVirtualGatewaySpecElListenerElPortMappingElRef> {
        ListRef::new(self.shared().clone(), format!("{}.port_mapping", self.base))
    }
    #[doc = "Get a reference to the value of field `tls` after provisioning.\n"]
    pub fn tls(&self) -> ListRef<AppmeshVirtualGatewaySpecElListenerElTlsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tls", self.base))
    }
}
#[derive(Serialize)]
pub struct AppmeshVirtualGatewaySpecElLoggingElAccessLogElFileElFormatElJsonEl {
    key: PrimField<String>,
    value: PrimField<String>,
}
impl AppmeshVirtualGatewaySpecElLoggingElAccessLogElFileElFormatElJsonEl {}
impl ToListMappable for AppmeshVirtualGatewaySpecElLoggingElAccessLogElFileElFormatElJsonEl {
    type O = BlockAssignable<AppmeshVirtualGatewaySpecElLoggingElAccessLogElFileElFormatElJsonEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppmeshVirtualGatewaySpecElLoggingElAccessLogElFileElFormatElJsonEl {
    #[doc = ""]
    pub key: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildAppmeshVirtualGatewaySpecElLoggingElAccessLogElFileElFormatElJsonEl {
    pub fn build(self) -> AppmeshVirtualGatewaySpecElLoggingElAccessLogElFileElFormatElJsonEl {
        AppmeshVirtualGatewaySpecElLoggingElAccessLogElFileElFormatElJsonEl {
            key: self.key,
            value: self.value,
        }
    }
}
pub struct AppmeshVirtualGatewaySpecElLoggingElAccessLogElFileElFormatElJsonElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppmeshVirtualGatewaySpecElLoggingElAccessLogElFileElFormatElJsonElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppmeshVirtualGatewaySpecElLoggingElAccessLogElFileElFormatElJsonElRef {
        AppmeshVirtualGatewaySpecElLoggingElAccessLogElFileElFormatElJsonElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppmeshVirtualGatewaySpecElLoggingElAccessLogElFileElFormatElJsonElRef {
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
struct AppmeshVirtualGatewaySpecElLoggingElAccessLogElFileElFormatElDynamic {
    json: Option<DynamicBlock<AppmeshVirtualGatewaySpecElLoggingElAccessLogElFileElFormatElJsonEl>>,
}
#[derive(Serialize)]
pub struct AppmeshVirtualGatewaySpecElLoggingElAccessLogElFileElFormatEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    json: Option<Vec<AppmeshVirtualGatewaySpecElLoggingElAccessLogElFileElFormatElJsonEl>>,
    dynamic: AppmeshVirtualGatewaySpecElLoggingElAccessLogElFileElFormatElDynamic,
}
impl AppmeshVirtualGatewaySpecElLoggingElAccessLogElFileElFormatEl {
    #[doc = "Set the field `text`.\n"]
    pub fn set_text(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.text = Some(v.into());
        self
    }
    #[doc = "Set the field `json`.\n"]
    pub fn set_json(
        mut self,
        v: impl Into<
            BlockAssignable<AppmeshVirtualGatewaySpecElLoggingElAccessLogElFileElFormatElJsonEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.json = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.json = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for AppmeshVirtualGatewaySpecElLoggingElAccessLogElFileElFormatEl {
    type O = BlockAssignable<AppmeshVirtualGatewaySpecElLoggingElAccessLogElFileElFormatEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppmeshVirtualGatewaySpecElLoggingElAccessLogElFileElFormatEl {}
impl BuildAppmeshVirtualGatewaySpecElLoggingElAccessLogElFileElFormatEl {
    pub fn build(self) -> AppmeshVirtualGatewaySpecElLoggingElAccessLogElFileElFormatEl {
        AppmeshVirtualGatewaySpecElLoggingElAccessLogElFileElFormatEl {
            text: core::default::Default::default(),
            json: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AppmeshVirtualGatewaySpecElLoggingElAccessLogElFileElFormatElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppmeshVirtualGatewaySpecElLoggingElAccessLogElFileElFormatElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppmeshVirtualGatewaySpecElLoggingElAccessLogElFileElFormatElRef {
        AppmeshVirtualGatewaySpecElLoggingElAccessLogElFileElFormatElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppmeshVirtualGatewaySpecElLoggingElAccessLogElFileElFormatElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `text` after provisioning.\n"]
    pub fn text(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.text", self.base))
    }
    #[doc = "Get a reference to the value of field `json` after provisioning.\n"]
    pub fn json(
        &self,
    ) -> ListRef<AppmeshVirtualGatewaySpecElLoggingElAccessLogElFileElFormatElJsonElRef> {
        ListRef::new(self.shared().clone(), format!("{}.json", self.base))
    }
}
#[derive(Serialize, Default)]
struct AppmeshVirtualGatewaySpecElLoggingElAccessLogElFileElDynamic {
    format: Option<DynamicBlock<AppmeshVirtualGatewaySpecElLoggingElAccessLogElFileElFormatEl>>,
}
#[derive(Serialize)]
pub struct AppmeshVirtualGatewaySpecElLoggingElAccessLogElFileEl {
    path: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    format: Option<Vec<AppmeshVirtualGatewaySpecElLoggingElAccessLogElFileElFormatEl>>,
    dynamic: AppmeshVirtualGatewaySpecElLoggingElAccessLogElFileElDynamic,
}
impl AppmeshVirtualGatewaySpecElLoggingElAccessLogElFileEl {
    #[doc = "Set the field `format`.\n"]
    pub fn set_format(
        mut self,
        v: impl Into<BlockAssignable<AppmeshVirtualGatewaySpecElLoggingElAccessLogElFileElFormatEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.format = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.format = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for AppmeshVirtualGatewaySpecElLoggingElAccessLogElFileEl {
    type O = BlockAssignable<AppmeshVirtualGatewaySpecElLoggingElAccessLogElFileEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppmeshVirtualGatewaySpecElLoggingElAccessLogElFileEl {
    #[doc = ""]
    pub path: PrimField<String>,
}
impl BuildAppmeshVirtualGatewaySpecElLoggingElAccessLogElFileEl {
    pub fn build(self) -> AppmeshVirtualGatewaySpecElLoggingElAccessLogElFileEl {
        AppmeshVirtualGatewaySpecElLoggingElAccessLogElFileEl {
            path: self.path,
            format: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AppmeshVirtualGatewaySpecElLoggingElAccessLogElFileElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppmeshVirtualGatewaySpecElLoggingElAccessLogElFileElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppmeshVirtualGatewaySpecElLoggingElAccessLogElFileElRef {
        AppmeshVirtualGatewaySpecElLoggingElAccessLogElFileElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppmeshVirtualGatewaySpecElLoggingElAccessLogElFileElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
    #[doc = "Get a reference to the value of field `format` after provisioning.\n"]
    pub fn format(
        &self,
    ) -> ListRef<AppmeshVirtualGatewaySpecElLoggingElAccessLogElFileElFormatElRef> {
        ListRef::new(self.shared().clone(), format!("{}.format", self.base))
    }
}
#[derive(Serialize, Default)]
struct AppmeshVirtualGatewaySpecElLoggingElAccessLogElDynamic {
    file: Option<DynamicBlock<AppmeshVirtualGatewaySpecElLoggingElAccessLogElFileEl>>,
}
#[derive(Serialize)]
pub struct AppmeshVirtualGatewaySpecElLoggingElAccessLogEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    file: Option<Vec<AppmeshVirtualGatewaySpecElLoggingElAccessLogElFileEl>>,
    dynamic: AppmeshVirtualGatewaySpecElLoggingElAccessLogElDynamic,
}
impl AppmeshVirtualGatewaySpecElLoggingElAccessLogEl {
    #[doc = "Set the field `file`.\n"]
    pub fn set_file(
        mut self,
        v: impl Into<BlockAssignable<AppmeshVirtualGatewaySpecElLoggingElAccessLogElFileEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.file = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.file = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for AppmeshVirtualGatewaySpecElLoggingElAccessLogEl {
    type O = BlockAssignable<AppmeshVirtualGatewaySpecElLoggingElAccessLogEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppmeshVirtualGatewaySpecElLoggingElAccessLogEl {}
impl BuildAppmeshVirtualGatewaySpecElLoggingElAccessLogEl {
    pub fn build(self) -> AppmeshVirtualGatewaySpecElLoggingElAccessLogEl {
        AppmeshVirtualGatewaySpecElLoggingElAccessLogEl {
            file: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AppmeshVirtualGatewaySpecElLoggingElAccessLogElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppmeshVirtualGatewaySpecElLoggingElAccessLogElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppmeshVirtualGatewaySpecElLoggingElAccessLogElRef {
        AppmeshVirtualGatewaySpecElLoggingElAccessLogElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppmeshVirtualGatewaySpecElLoggingElAccessLogElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `file` after provisioning.\n"]
    pub fn file(&self) -> ListRef<AppmeshVirtualGatewaySpecElLoggingElAccessLogElFileElRef> {
        ListRef::new(self.shared().clone(), format!("{}.file", self.base))
    }
}
#[derive(Serialize, Default)]
struct AppmeshVirtualGatewaySpecElLoggingElDynamic {
    access_log: Option<DynamicBlock<AppmeshVirtualGatewaySpecElLoggingElAccessLogEl>>,
}
#[derive(Serialize)]
pub struct AppmeshVirtualGatewaySpecElLoggingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    access_log: Option<Vec<AppmeshVirtualGatewaySpecElLoggingElAccessLogEl>>,
    dynamic: AppmeshVirtualGatewaySpecElLoggingElDynamic,
}
impl AppmeshVirtualGatewaySpecElLoggingEl {
    #[doc = "Set the field `access_log`.\n"]
    pub fn set_access_log(
        mut self,
        v: impl Into<BlockAssignable<AppmeshVirtualGatewaySpecElLoggingElAccessLogEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.access_log = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.access_log = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for AppmeshVirtualGatewaySpecElLoggingEl {
    type O = BlockAssignable<AppmeshVirtualGatewaySpecElLoggingEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppmeshVirtualGatewaySpecElLoggingEl {}
impl BuildAppmeshVirtualGatewaySpecElLoggingEl {
    pub fn build(self) -> AppmeshVirtualGatewaySpecElLoggingEl {
        AppmeshVirtualGatewaySpecElLoggingEl {
            access_log: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AppmeshVirtualGatewaySpecElLoggingElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppmeshVirtualGatewaySpecElLoggingElRef {
    fn new(shared: StackShared, base: String) -> AppmeshVirtualGatewaySpecElLoggingElRef {
        AppmeshVirtualGatewaySpecElLoggingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppmeshVirtualGatewaySpecElLoggingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `access_log` after provisioning.\n"]
    pub fn access_log(&self) -> ListRef<AppmeshVirtualGatewaySpecElLoggingElAccessLogElRef> {
        ListRef::new(self.shared().clone(), format!("{}.access_log", self.base))
    }
}
#[derive(Serialize, Default)]
struct AppmeshVirtualGatewaySpecElDynamic {
    backend_defaults: Option<DynamicBlock<AppmeshVirtualGatewaySpecElBackendDefaultsEl>>,
    listener: Option<DynamicBlock<AppmeshVirtualGatewaySpecElListenerEl>>,
    logging: Option<DynamicBlock<AppmeshVirtualGatewaySpecElLoggingEl>>,
}
#[derive(Serialize)]
pub struct AppmeshVirtualGatewaySpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    backend_defaults: Option<Vec<AppmeshVirtualGatewaySpecElBackendDefaultsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    listener: Option<Vec<AppmeshVirtualGatewaySpecElListenerEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logging: Option<Vec<AppmeshVirtualGatewaySpecElLoggingEl>>,
    dynamic: AppmeshVirtualGatewaySpecElDynamic,
}
impl AppmeshVirtualGatewaySpecEl {
    #[doc = "Set the field `backend_defaults`.\n"]
    pub fn set_backend_defaults(
        mut self,
        v: impl Into<BlockAssignable<AppmeshVirtualGatewaySpecElBackendDefaultsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.backend_defaults = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.backend_defaults = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `listener`.\n"]
    pub fn set_listener(
        mut self,
        v: impl Into<BlockAssignable<AppmeshVirtualGatewaySpecElListenerEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.listener = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.listener = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `logging`.\n"]
    pub fn set_logging(
        mut self,
        v: impl Into<BlockAssignable<AppmeshVirtualGatewaySpecElLoggingEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.logging = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.logging = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for AppmeshVirtualGatewaySpecEl {
    type O = BlockAssignable<AppmeshVirtualGatewaySpecEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppmeshVirtualGatewaySpecEl {}
impl BuildAppmeshVirtualGatewaySpecEl {
    pub fn build(self) -> AppmeshVirtualGatewaySpecEl {
        AppmeshVirtualGatewaySpecEl {
            backend_defaults: core::default::Default::default(),
            listener: core::default::Default::default(),
            logging: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AppmeshVirtualGatewaySpecElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppmeshVirtualGatewaySpecElRef {
    fn new(shared: StackShared, base: String) -> AppmeshVirtualGatewaySpecElRef {
        AppmeshVirtualGatewaySpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppmeshVirtualGatewaySpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `backend_defaults` after provisioning.\n"]
    pub fn backend_defaults(&self) -> ListRef<AppmeshVirtualGatewaySpecElBackendDefaultsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.backend_defaults", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `listener` after provisioning.\n"]
    pub fn listener(&self) -> ListRef<AppmeshVirtualGatewaySpecElListenerElRef> {
        ListRef::new(self.shared().clone(), format!("{}.listener", self.base))
    }
    #[doc = "Get a reference to the value of field `logging` after provisioning.\n"]
    pub fn logging(&self) -> ListRef<AppmeshVirtualGatewaySpecElLoggingElRef> {
        ListRef::new(self.shared().clone(), format!("{}.logging", self.base))
    }
}
#[derive(Serialize, Default)]
struct AppmeshVirtualGatewayDynamic {
    spec: Option<DynamicBlock<AppmeshVirtualGatewaySpecEl>>,
}
