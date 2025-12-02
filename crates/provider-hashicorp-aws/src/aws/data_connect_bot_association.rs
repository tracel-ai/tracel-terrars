use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataConnectBotAssociationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    instance_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lex_bot: Option<Vec<DataConnectBotAssociationLexBotEl>>,
    dynamic: DataConnectBotAssociationDynamic,
}
struct DataConnectBotAssociation_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataConnectBotAssociationData>,
}
#[derive(Clone)]
pub struct DataConnectBotAssociation(Rc<DataConnectBotAssociation_>);
impl DataConnectBotAssociation {
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
    #[doc = "Set the field `lex_bot`.\n"]
    pub fn set_lex_bot(
        self,
        v: impl Into<BlockAssignable<DataConnectBotAssociationLexBotEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().lex_bot = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.lex_bot = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `instance_id` after provisioning.\n"]
    pub fn instance_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `lex_bot` after provisioning.\n"]
    pub fn lex_bot(&self) -> ListRef<DataConnectBotAssociationLexBotElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.lex_bot", self.extract_ref()),
        )
    }
}
impl Referable for DataConnectBotAssociation {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataConnectBotAssociation {}
impl ToListMappable for DataConnectBotAssociation {
    type O = ListRef<DataConnectBotAssociationRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataConnectBotAssociation_ {
    fn extract_datasource_type(&self) -> String {
        "aws_connect_bot_association".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataConnectBotAssociation {
    pub tf_id: String,
    #[doc = ""]
    pub instance_id: PrimField<String>,
}
impl BuildDataConnectBotAssociation {
    pub fn build(self, stack: &mut Stack) -> DataConnectBotAssociation {
        let out = DataConnectBotAssociation(Rc::new(DataConnectBotAssociation_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataConnectBotAssociationData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                instance_id: self.instance_id,
                region: core::default::Default::default(),
                lex_bot: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataConnectBotAssociationRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataConnectBotAssociationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataConnectBotAssociationRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `instance_id` after provisioning.\n"]
    pub fn instance_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `lex_bot` after provisioning.\n"]
    pub fn lex_bot(&self) -> ListRef<DataConnectBotAssociationLexBotElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.lex_bot", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataConnectBotAssociationLexBotEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    lex_region: Option<PrimField<String>>,
    name: PrimField<String>,
}
impl DataConnectBotAssociationLexBotEl {
    #[doc = "Set the field `lex_region`.\n"]
    pub fn set_lex_region(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.lex_region = Some(v.into());
        self
    }
}
impl ToListMappable for DataConnectBotAssociationLexBotEl {
    type O = BlockAssignable<DataConnectBotAssociationLexBotEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataConnectBotAssociationLexBotEl {
    #[doc = ""]
    pub name: PrimField<String>,
}
impl BuildDataConnectBotAssociationLexBotEl {
    pub fn build(self) -> DataConnectBotAssociationLexBotEl {
        DataConnectBotAssociationLexBotEl {
            lex_region: core::default::Default::default(),
            name: self.name,
        }
    }
}
pub struct DataConnectBotAssociationLexBotElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataConnectBotAssociationLexBotElRef {
    fn new(shared: StackShared, base: String) -> DataConnectBotAssociationLexBotElRef {
        DataConnectBotAssociationLexBotElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataConnectBotAssociationLexBotElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `lex_region` after provisioning.\n"]
    pub fn lex_region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.lex_region", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
#[derive(Serialize, Default)]
struct DataConnectBotAssociationDynamic {
    lex_bot: Option<DynamicBlock<DataConnectBotAssociationLexBotEl>>,
}
