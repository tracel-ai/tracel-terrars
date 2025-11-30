use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct DataSsoadminApplicationAssignmentsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    application_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}

struct DataSsoadminApplicationAssignments_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataSsoadminApplicationAssignmentsData>,
}

#[derive(Clone)]
pub struct DataSsoadminApplicationAssignments(Rc<DataSsoadminApplicationAssignments_>);

impl DataSsoadminApplicationAssignments {
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

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `application_arn` after provisioning.\n"]
    pub fn application_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.application_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `application_assignments` after provisioning.\n"]
    pub fn application_assignments(
        &self,
    ) -> ListRef<DataSsoadminApplicationAssignmentsApplicationAssignmentsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.application_assignments", self.extract_ref()),
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
}

impl Referable for DataSsoadminApplicationAssignments {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Datasource for DataSsoadminApplicationAssignments {}

impl ToListMappable for DataSsoadminApplicationAssignments {
    type O = ListRef<DataSsoadminApplicationAssignmentsRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataSsoadminApplicationAssignments_ {
    fn extract_datasource_type(&self) -> String {
        "aws_ssoadmin_application_assignments".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataSsoadminApplicationAssignments {
    pub tf_id: String,
    #[doc = ""]
    pub application_arn: PrimField<String>,
}

impl BuildDataSsoadminApplicationAssignments {
    pub fn build(self, stack: &mut Stack) -> DataSsoadminApplicationAssignments {
        let out =
            DataSsoadminApplicationAssignments(Rc::new(DataSsoadminApplicationAssignments_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(DataSsoadminApplicationAssignmentsData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    for_each: None,
                    application_arn: self.application_arn,
                    region: core::default::Default::default(),
                }),
            }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataSsoadminApplicationAssignmentsRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSsoadminApplicationAssignmentsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl DataSsoadminApplicationAssignmentsRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `application_arn` after provisioning.\n"]
    pub fn application_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.application_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `application_assignments` after provisioning.\n"]
    pub fn application_assignments(
        &self,
    ) -> ListRef<DataSsoadminApplicationAssignmentsApplicationAssignmentsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.application_assignments", self.extract_ref()),
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
}

#[derive(Serialize)]
pub struct DataSsoadminApplicationAssignmentsApplicationAssignmentsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    application_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    principal_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    principal_type: Option<PrimField<String>>,
}

impl DataSsoadminApplicationAssignmentsApplicationAssignmentsEl {
    #[doc = "Set the field `application_arn`.\n"]
    pub fn set_application_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.application_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `principal_id`.\n"]
    pub fn set_principal_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.principal_id = Some(v.into());
        self
    }

    #[doc = "Set the field `principal_type`.\n"]
    pub fn set_principal_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.principal_type = Some(v.into());
        self
    }
}

impl ToListMappable for DataSsoadminApplicationAssignmentsApplicationAssignmentsEl {
    type O = BlockAssignable<DataSsoadminApplicationAssignmentsApplicationAssignmentsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSsoadminApplicationAssignmentsApplicationAssignmentsEl {}

impl BuildDataSsoadminApplicationAssignmentsApplicationAssignmentsEl {
    pub fn build(self) -> DataSsoadminApplicationAssignmentsApplicationAssignmentsEl {
        DataSsoadminApplicationAssignmentsApplicationAssignmentsEl {
            application_arn: core::default::Default::default(),
            principal_id: core::default::Default::default(),
            principal_type: core::default::Default::default(),
        }
    }
}

pub struct DataSsoadminApplicationAssignmentsApplicationAssignmentsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSsoadminApplicationAssignmentsApplicationAssignmentsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataSsoadminApplicationAssignmentsApplicationAssignmentsElRef {
        DataSsoadminApplicationAssignmentsApplicationAssignmentsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSsoadminApplicationAssignmentsApplicationAssignmentsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `application_arn` after provisioning.\n"]
    pub fn application_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.application_arn", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `principal_id` after provisioning.\n"]
    pub fn principal_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.principal_id", self.base))
    }

    #[doc = "Get a reference to the value of field `principal_type` after provisioning.\n"]
    pub fn principal_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.principal_type", self.base),
        )
    }
}
