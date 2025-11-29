use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataRoute53profilesProfilesData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}

struct DataRoute53profilesProfiles_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataRoute53profilesProfilesData>,
}

#[derive(Clone)]
pub struct DataRoute53profilesProfiles(Rc<DataRoute53profilesProfiles_>);

impl DataRoute53profilesProfiles {
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

    #[doc = "Get a reference to the value of field `profiles` after provisioning.\n"]
    pub fn profiles(&self) -> ListRef<DataRoute53profilesProfilesProfilesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.profiles", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}

impl Referable for DataRoute53profilesProfiles {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataRoute53profilesProfiles { }

impl ToListMappable for DataRoute53profilesProfiles {
    type O = ListRef<DataRoute53profilesProfilesRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataRoute53profilesProfiles_ {
    fn extract_datasource_type(&self) -> String {
        "aws_route53profiles_profiles".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataRoute53profilesProfiles {
    pub tf_id: String,
}

impl BuildDataRoute53profilesProfiles {
    pub fn build(self, stack: &mut Stack) -> DataRoute53profilesProfiles {
        let out = DataRoute53profilesProfiles(Rc::new(DataRoute53profilesProfiles_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataRoute53profilesProfilesData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataRoute53profilesProfilesRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataRoute53profilesProfilesRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataRoute53profilesProfilesRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `profiles` after provisioning.\n"]
    pub fn profiles(&self) -> ListRef<DataRoute53profilesProfilesProfilesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.profiles", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataRoute53profilesProfilesProfilesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    share_status: Option<PrimField<String>>,
}

impl DataRoute53profilesProfilesProfilesEl {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.arn = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc = "Set the field `share_status`.\n"]
    pub fn set_share_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.share_status = Some(v.into());
        self
    }
}

impl ToListMappable for DataRoute53profilesProfilesProfilesEl {
    type O = BlockAssignable<DataRoute53profilesProfilesProfilesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRoute53profilesProfilesProfilesEl {}

impl BuildDataRoute53profilesProfilesProfilesEl {
    pub fn build(self) -> DataRoute53profilesProfilesProfilesEl {
        DataRoute53profilesProfilesProfilesEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
            share_status: core::default::Default::default(),
        }
    }
}

pub struct DataRoute53profilesProfilesProfilesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataRoute53profilesProfilesProfilesElRef {
    fn new(shared: StackShared, base: String) -> DataRoute53profilesProfilesProfilesElRef {
        DataRoute53profilesProfilesProfilesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRoute53profilesProfilesProfilesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `share_status` after provisioning.\n"]
    pub fn share_status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.share_status", self.base))
    }
}
