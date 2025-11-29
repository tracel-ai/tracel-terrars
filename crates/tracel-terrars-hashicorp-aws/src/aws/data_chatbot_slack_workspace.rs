use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataChatbotSlackWorkspaceData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    slack_team_name: PrimField<String>,
}

struct DataChatbotSlackWorkspace_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataChatbotSlackWorkspaceData>,
}

#[derive(Clone)]
pub struct DataChatbotSlackWorkspace(Rc<DataChatbotSlackWorkspace_>);

impl DataChatbotSlackWorkspace {
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `slack_team_id` after provisioning.\n"]
    pub fn slack_team_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.slack_team_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `slack_team_name` after provisioning.\n"]
    pub fn slack_team_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.slack_team_name", self.extract_ref()))
    }
}

impl Referable for DataChatbotSlackWorkspace {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataChatbotSlackWorkspace { }

impl ToListMappable for DataChatbotSlackWorkspace {
    type O = ListRef<DataChatbotSlackWorkspaceRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataChatbotSlackWorkspace_ {
    fn extract_datasource_type(&self) -> String {
        "aws_chatbot_slack_workspace".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataChatbotSlackWorkspace {
    pub tf_id: String,
    #[doc = ""]
    pub slack_team_name: PrimField<String>,
}

impl BuildDataChatbotSlackWorkspace {
    pub fn build(self, stack: &mut Stack) -> DataChatbotSlackWorkspace {
        let out = DataChatbotSlackWorkspace(Rc::new(DataChatbotSlackWorkspace_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataChatbotSlackWorkspaceData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                region: core::default::Default::default(),
                slack_team_name: self.slack_team_name,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataChatbotSlackWorkspaceRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataChatbotSlackWorkspaceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataChatbotSlackWorkspaceRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `slack_team_id` after provisioning.\n"]
    pub fn slack_team_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.slack_team_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `slack_team_name` after provisioning.\n"]
    pub fn slack_team_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.slack_team_name", self.extract_ref()))
    }
}
