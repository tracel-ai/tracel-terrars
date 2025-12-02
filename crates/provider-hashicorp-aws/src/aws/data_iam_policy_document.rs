use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataIamPolicyDocumentData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    override_json: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    override_policy_documents: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    policy_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_json: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_policy_documents: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    statement: Option<Vec<DataIamPolicyDocumentStatementEl>>,
    dynamic: DataIamPolicyDocumentDynamic,
}
struct DataIamPolicyDocument_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataIamPolicyDocumentData>,
}
#[derive(Clone)]
pub struct DataIamPolicyDocument(Rc<DataIamPolicyDocument_>);
impl DataIamPolicyDocument {
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
    #[doc = "Set the field `override_json`.\n"]
    pub fn set_override_json(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().override_json = Some(v.into());
        self
    }
    #[doc = "Set the field `override_policy_documents`.\n"]
    pub fn set_override_policy_documents(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().override_policy_documents = Some(v.into());
        self
    }
    #[doc = "Set the field `policy_id`.\n"]
    pub fn set_policy_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().policy_id = Some(v.into());
        self
    }
    #[doc = "Set the field `source_json`.\n"]
    pub fn set_source_json(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().source_json = Some(v.into());
        self
    }
    #[doc = "Set the field `source_policy_documents`.\n"]
    pub fn set_source_policy_documents(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().source_policy_documents = Some(v.into());
        self
    }
    #[doc = "Set the field `version`.\n"]
    pub fn set_version(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().version = Some(v.into());
        self
    }
    #[doc = "Set the field `statement`.\n"]
    pub fn set_statement(
        self,
        v: impl Into<BlockAssignable<DataIamPolicyDocumentStatementEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().statement = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.statement = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `json` after provisioning.\n"]
    pub fn json(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.json", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `minified_json` after provisioning.\n"]
    pub fn minified_json(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.minified_json", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `override_json` after provisioning.\n"]
    pub fn override_json(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.override_json", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `override_policy_documents` after provisioning.\n"]
    pub fn override_policy_documents(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.override_policy_documents", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `policy_id` after provisioning.\n"]
    pub fn policy_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.policy_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_json` after provisioning.\n"]
    pub fn source_json(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_json", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_policy_documents` after provisioning.\n"]
    pub fn source_policy_documents(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source_policy_documents", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `statement` after provisioning.\n"]
    pub fn statement(&self) -> ListRef<DataIamPolicyDocumentStatementElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.statement", self.extract_ref()),
        )
    }
}
impl Referable for DataIamPolicyDocument {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataIamPolicyDocument {}
impl ToListMappable for DataIamPolicyDocument {
    type O = ListRef<DataIamPolicyDocumentRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataIamPolicyDocument_ {
    fn extract_datasource_type(&self) -> String {
        "aws_iam_policy_document".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataIamPolicyDocument {
    pub tf_id: String,
}
impl BuildDataIamPolicyDocument {
    pub fn build(self, stack: &mut Stack) -> DataIamPolicyDocument {
        let out = DataIamPolicyDocument(Rc::new(DataIamPolicyDocument_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataIamPolicyDocumentData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                override_json: core::default::Default::default(),
                override_policy_documents: core::default::Default::default(),
                policy_id: core::default::Default::default(),
                source_json: core::default::Default::default(),
                source_policy_documents: core::default::Default::default(),
                version: core::default::Default::default(),
                statement: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataIamPolicyDocumentRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataIamPolicyDocumentRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataIamPolicyDocumentRef {
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
    #[doc = "Get a reference to the value of field `json` after provisioning.\n"]
    pub fn json(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.json", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `minified_json` after provisioning.\n"]
    pub fn minified_json(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.minified_json", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `override_json` after provisioning.\n"]
    pub fn override_json(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.override_json", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `override_policy_documents` after provisioning.\n"]
    pub fn override_policy_documents(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.override_policy_documents", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `policy_id` after provisioning.\n"]
    pub fn policy_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.policy_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_json` after provisioning.\n"]
    pub fn source_json(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_json", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_policy_documents` after provisioning.\n"]
    pub fn source_policy_documents(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source_policy_documents", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `statement` after provisioning.\n"]
    pub fn statement(&self) -> ListRef<DataIamPolicyDocumentStatementElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.statement", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataIamPolicyDocumentStatementElConditionEl {
    test: PrimField<String>,
    values: ListField<PrimField<String>>,
    variable: PrimField<String>,
}
impl DataIamPolicyDocumentStatementElConditionEl {}
impl ToListMappable for DataIamPolicyDocumentStatementElConditionEl {
    type O = BlockAssignable<DataIamPolicyDocumentStatementElConditionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataIamPolicyDocumentStatementElConditionEl {
    #[doc = ""]
    pub test: PrimField<String>,
    #[doc = ""]
    pub values: ListField<PrimField<String>>,
    #[doc = ""]
    pub variable: PrimField<String>,
}
impl BuildDataIamPolicyDocumentStatementElConditionEl {
    pub fn build(self) -> DataIamPolicyDocumentStatementElConditionEl {
        DataIamPolicyDocumentStatementElConditionEl {
            test: self.test,
            values: self.values,
            variable: self.variable,
        }
    }
}
pub struct DataIamPolicyDocumentStatementElConditionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataIamPolicyDocumentStatementElConditionElRef {
    fn new(shared: StackShared, base: String) -> DataIamPolicyDocumentStatementElConditionElRef {
        DataIamPolicyDocumentStatementElConditionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataIamPolicyDocumentStatementElConditionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `test` after provisioning.\n"]
    pub fn test(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.test", self.base))
    }
    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
    #[doc = "Get a reference to the value of field `variable` after provisioning.\n"]
    pub fn variable(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.variable", self.base))
    }
}
#[derive(Serialize)]
pub struct DataIamPolicyDocumentStatementElNotPrincipalsEl {
    identifiers: SetField<PrimField<String>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}
impl DataIamPolicyDocumentStatementElNotPrincipalsEl {}
impl ToListMappable for DataIamPolicyDocumentStatementElNotPrincipalsEl {
    type O = BlockAssignable<DataIamPolicyDocumentStatementElNotPrincipalsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataIamPolicyDocumentStatementElNotPrincipalsEl {
    #[doc = ""]
    pub identifiers: SetField<PrimField<String>>,
    #[doc = ""]
    pub type_: PrimField<String>,
}
impl BuildDataIamPolicyDocumentStatementElNotPrincipalsEl {
    pub fn build(self) -> DataIamPolicyDocumentStatementElNotPrincipalsEl {
        DataIamPolicyDocumentStatementElNotPrincipalsEl {
            identifiers: self.identifiers,
            type_: self.type_,
        }
    }
}
pub struct DataIamPolicyDocumentStatementElNotPrincipalsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataIamPolicyDocumentStatementElNotPrincipalsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataIamPolicyDocumentStatementElNotPrincipalsElRef {
        DataIamPolicyDocumentStatementElNotPrincipalsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataIamPolicyDocumentStatementElNotPrincipalsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `identifiers` after provisioning.\n"]
    pub fn identifiers(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.identifiers", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct DataIamPolicyDocumentStatementElPrincipalsEl {
    identifiers: SetField<PrimField<String>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}
impl DataIamPolicyDocumentStatementElPrincipalsEl {}
impl ToListMappable for DataIamPolicyDocumentStatementElPrincipalsEl {
    type O = BlockAssignable<DataIamPolicyDocumentStatementElPrincipalsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataIamPolicyDocumentStatementElPrincipalsEl {
    #[doc = ""]
    pub identifiers: SetField<PrimField<String>>,
    #[doc = ""]
    pub type_: PrimField<String>,
}
impl BuildDataIamPolicyDocumentStatementElPrincipalsEl {
    pub fn build(self) -> DataIamPolicyDocumentStatementElPrincipalsEl {
        DataIamPolicyDocumentStatementElPrincipalsEl {
            identifiers: self.identifiers,
            type_: self.type_,
        }
    }
}
pub struct DataIamPolicyDocumentStatementElPrincipalsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataIamPolicyDocumentStatementElPrincipalsElRef {
    fn new(shared: StackShared, base: String) -> DataIamPolicyDocumentStatementElPrincipalsElRef {
        DataIamPolicyDocumentStatementElPrincipalsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataIamPolicyDocumentStatementElPrincipalsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `identifiers` after provisioning.\n"]
    pub fn identifiers(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.identifiers", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize, Default)]
struct DataIamPolicyDocumentStatementElDynamic {
    condition: Option<DynamicBlock<DataIamPolicyDocumentStatementElConditionEl>>,
    not_principals: Option<DynamicBlock<DataIamPolicyDocumentStatementElNotPrincipalsEl>>,
    principals: Option<DynamicBlock<DataIamPolicyDocumentStatementElPrincipalsEl>>,
}
#[derive(Serialize)]
pub struct DataIamPolicyDocumentStatementEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    actions: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    effect: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    not_actions: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    not_resources: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resources: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sid: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    condition: Option<Vec<DataIamPolicyDocumentStatementElConditionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    not_principals: Option<Vec<DataIamPolicyDocumentStatementElNotPrincipalsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    principals: Option<Vec<DataIamPolicyDocumentStatementElPrincipalsEl>>,
    dynamic: DataIamPolicyDocumentStatementElDynamic,
}
impl DataIamPolicyDocumentStatementEl {
    #[doc = "Set the field `actions`.\n"]
    pub fn set_actions(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.actions = Some(v.into());
        self
    }
    #[doc = "Set the field `effect`.\n"]
    pub fn set_effect(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.effect = Some(v.into());
        self
    }
    #[doc = "Set the field `not_actions`.\n"]
    pub fn set_not_actions(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.not_actions = Some(v.into());
        self
    }
    #[doc = "Set the field `not_resources`.\n"]
    pub fn set_not_resources(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.not_resources = Some(v.into());
        self
    }
    #[doc = "Set the field `resources`.\n"]
    pub fn set_resources(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.resources = Some(v.into());
        self
    }
    #[doc = "Set the field `sid`.\n"]
    pub fn set_sid(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sid = Some(v.into());
        self
    }
    #[doc = "Set the field `condition`.\n"]
    pub fn set_condition(
        mut self,
        v: impl Into<BlockAssignable<DataIamPolicyDocumentStatementElConditionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.condition = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.condition = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `not_principals`.\n"]
    pub fn set_not_principals(
        mut self,
        v: impl Into<BlockAssignable<DataIamPolicyDocumentStatementElNotPrincipalsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.not_principals = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.not_principals = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `principals`.\n"]
    pub fn set_principals(
        mut self,
        v: impl Into<BlockAssignable<DataIamPolicyDocumentStatementElPrincipalsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.principals = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.principals = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataIamPolicyDocumentStatementEl {
    type O = BlockAssignable<DataIamPolicyDocumentStatementEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataIamPolicyDocumentStatementEl {}
impl BuildDataIamPolicyDocumentStatementEl {
    pub fn build(self) -> DataIamPolicyDocumentStatementEl {
        DataIamPolicyDocumentStatementEl {
            actions: core::default::Default::default(),
            effect: core::default::Default::default(),
            not_actions: core::default::Default::default(),
            not_resources: core::default::Default::default(),
            resources: core::default::Default::default(),
            sid: core::default::Default::default(),
            condition: core::default::Default::default(),
            not_principals: core::default::Default::default(),
            principals: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataIamPolicyDocumentStatementElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataIamPolicyDocumentStatementElRef {
    fn new(shared: StackShared, base: String) -> DataIamPolicyDocumentStatementElRef {
        DataIamPolicyDocumentStatementElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataIamPolicyDocumentStatementElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `actions` after provisioning.\n"]
    pub fn actions(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.actions", self.base))
    }
    #[doc = "Get a reference to the value of field `effect` after provisioning.\n"]
    pub fn effect(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.effect", self.base))
    }
    #[doc = "Get a reference to the value of field `not_actions` after provisioning.\n"]
    pub fn not_actions(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.not_actions", self.base))
    }
    #[doc = "Get a reference to the value of field `not_resources` after provisioning.\n"]
    pub fn not_resources(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.not_resources", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `resources` after provisioning.\n"]
    pub fn resources(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.resources", self.base))
    }
    #[doc = "Get a reference to the value of field `sid` after provisioning.\n"]
    pub fn sid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sid", self.base))
    }
}
#[derive(Serialize, Default)]
struct DataIamPolicyDocumentDynamic {
    statement: Option<DynamicBlock<DataIamPolicyDocumentStatementEl>>,
}
