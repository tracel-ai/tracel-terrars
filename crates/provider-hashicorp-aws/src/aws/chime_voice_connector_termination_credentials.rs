use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ChimeVoiceConnectorTerminationCredentialsData {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    voice_connector_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    credentials: Option<Vec<ChimeVoiceConnectorTerminationCredentialsCredentialsEl>>,
    dynamic: ChimeVoiceConnectorTerminationCredentialsDynamic,
}
struct ChimeVoiceConnectorTerminationCredentials_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ChimeVoiceConnectorTerminationCredentialsData>,
}
#[derive(Clone)]
pub struct ChimeVoiceConnectorTerminationCredentials(
    Rc<ChimeVoiceConnectorTerminationCredentials_>,
);
impl ChimeVoiceConnectorTerminationCredentials {
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
    #[doc = "Set the field `credentials`.\n"]
    pub fn set_credentials(
        self,
        v: impl Into<BlockAssignable<ChimeVoiceConnectorTerminationCredentialsCredentialsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().credentials = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.credentials = Some(d);
            }
        }
        self
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
    #[doc = "Get a reference to the value of field `voice_connector_id` after provisioning.\n"]
    pub fn voice_connector_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.voice_connector_id", self.extract_ref()),
        )
    }
}
impl Referable for ChimeVoiceConnectorTerminationCredentials {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ChimeVoiceConnectorTerminationCredentials {}
impl ToListMappable for ChimeVoiceConnectorTerminationCredentials {
    type O = ListRef<ChimeVoiceConnectorTerminationCredentialsRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ChimeVoiceConnectorTerminationCredentials_ {
    fn extract_resource_type(&self) -> String {
        "aws_chime_voice_connector_termination_credentials".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildChimeVoiceConnectorTerminationCredentials {
    pub tf_id: String,
    #[doc = ""]
    pub voice_connector_id: PrimField<String>,
}
impl BuildChimeVoiceConnectorTerminationCredentials {
    pub fn build(self, stack: &mut Stack) -> ChimeVoiceConnectorTerminationCredentials {
        let out = ChimeVoiceConnectorTerminationCredentials(Rc::new(
            ChimeVoiceConnectorTerminationCredentials_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(ChimeVoiceConnectorTerminationCredentialsData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    lifecycle: core::default::Default::default(),
                    for_each: None,
                    id: core::default::Default::default(),
                    region: core::default::Default::default(),
                    voice_connector_id: self.voice_connector_id,
                    credentials: core::default::Default::default(),
                    dynamic: Default::default(),
                }),
            },
        ));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ChimeVoiceConnectorTerminationCredentialsRef {
    shared: StackShared,
    base: String,
}
impl Ref for ChimeVoiceConnectorTerminationCredentialsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ChimeVoiceConnectorTerminationCredentialsRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
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
    #[doc = "Get a reference to the value of field `voice_connector_id` after provisioning.\n"]
    pub fn voice_connector_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.voice_connector_id", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ChimeVoiceConnectorTerminationCredentialsCredentialsEl {
    password: PrimField<String>,
    username: PrimField<String>,
}
impl ChimeVoiceConnectorTerminationCredentialsCredentialsEl {}
impl ToListMappable for ChimeVoiceConnectorTerminationCredentialsCredentialsEl {
    type O = BlockAssignable<ChimeVoiceConnectorTerminationCredentialsCredentialsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildChimeVoiceConnectorTerminationCredentialsCredentialsEl {
    #[doc = ""]
    pub password: PrimField<String>,
    #[doc = ""]
    pub username: PrimField<String>,
}
impl BuildChimeVoiceConnectorTerminationCredentialsCredentialsEl {
    pub fn build(self) -> ChimeVoiceConnectorTerminationCredentialsCredentialsEl {
        ChimeVoiceConnectorTerminationCredentialsCredentialsEl {
            password: self.password,
            username: self.username,
        }
    }
}
pub struct ChimeVoiceConnectorTerminationCredentialsCredentialsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ChimeVoiceConnectorTerminationCredentialsCredentialsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ChimeVoiceConnectorTerminationCredentialsCredentialsElRef {
        ChimeVoiceConnectorTerminationCredentialsCredentialsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ChimeVoiceConnectorTerminationCredentialsCredentialsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `password` after provisioning.\n"]
    pub fn password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }
    #[doc = "Get a reference to the value of field `username` after provisioning.\n"]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}
#[derive(Serialize, Default)]
struct ChimeVoiceConnectorTerminationCredentialsDynamic {
    credentials: Option<DynamicBlock<ChimeVoiceConnectorTerminationCredentialsCredentialsEl>>,
}
