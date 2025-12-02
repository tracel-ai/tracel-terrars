use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataSsoadminPrincipalApplicationAssignmentsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    instance_arn: PrimField<String>,
    principal_id: PrimField<String>,
    principal_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    application_assignments:
        Option<Vec<DataSsoadminPrincipalApplicationAssignmentsApplicationAssignmentsEl>>,
    dynamic: DataSsoadminPrincipalApplicationAssignmentsDynamic,
}
struct DataSsoadminPrincipalApplicationAssignments_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataSsoadminPrincipalApplicationAssignmentsData>,
}
#[derive(Clone)]
pub struct DataSsoadminPrincipalApplicationAssignments(
    Rc<DataSsoadminPrincipalApplicationAssignments_>,
);
impl DataSsoadminPrincipalApplicationAssignments {
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
    #[doc = "Set the field `application_assignments`.\n"]
    pub fn set_application_assignments(
        self,
        v: impl Into<
            BlockAssignable<DataSsoadminPrincipalApplicationAssignmentsApplicationAssignmentsEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().application_assignments = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.application_assignments = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `instance_arn` after provisioning.\n"]
    pub fn instance_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `principal_id` after provisioning.\n"]
    pub fn principal_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.principal_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `principal_type` after provisioning.\n"]
    pub fn principal_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.principal_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `application_assignments` after provisioning.\n"]
    pub fn application_assignments(
        &self,
    ) -> ListRef<DataSsoadminPrincipalApplicationAssignmentsApplicationAssignmentsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.application_assignments", self.extract_ref()),
        )
    }
}
impl Referable for DataSsoadminPrincipalApplicationAssignments {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataSsoadminPrincipalApplicationAssignments {}
impl ToListMappable for DataSsoadminPrincipalApplicationAssignments {
    type O = ListRef<DataSsoadminPrincipalApplicationAssignmentsRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataSsoadminPrincipalApplicationAssignments_ {
    fn extract_datasource_type(&self) -> String {
        "aws_ssoadmin_principal_application_assignments".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataSsoadminPrincipalApplicationAssignments {
    pub tf_id: String,
    #[doc = ""]
    pub instance_arn: PrimField<String>,
    #[doc = ""]
    pub principal_id: PrimField<String>,
    #[doc = ""]
    pub principal_type: PrimField<String>,
}
impl BuildDataSsoadminPrincipalApplicationAssignments {
    pub fn build(self, stack: &mut Stack) -> DataSsoadminPrincipalApplicationAssignments {
        let out = DataSsoadminPrincipalApplicationAssignments(Rc::new(
            DataSsoadminPrincipalApplicationAssignments_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(DataSsoadminPrincipalApplicationAssignmentsData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    for_each: None,
                    instance_arn: self.instance_arn,
                    principal_id: self.principal_id,
                    principal_type: self.principal_type,
                    region: core::default::Default::default(),
                    application_assignments: core::default::Default::default(),
                    dynamic: Default::default(),
                }),
            },
        ));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataSsoadminPrincipalApplicationAssignmentsRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataSsoadminPrincipalApplicationAssignmentsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataSsoadminPrincipalApplicationAssignmentsRef {
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
    #[doc = "Get a reference to the value of field `instance_arn` after provisioning.\n"]
    pub fn instance_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `principal_id` after provisioning.\n"]
    pub fn principal_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.principal_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `principal_type` after provisioning.\n"]
    pub fn principal_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.principal_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `application_assignments` after provisioning.\n"]
    pub fn application_assignments(
        &self,
    ) -> ListRef<DataSsoadminPrincipalApplicationAssignmentsApplicationAssignmentsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.application_assignments", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataSsoadminPrincipalApplicationAssignmentsApplicationAssignmentsEl {}
impl DataSsoadminPrincipalApplicationAssignmentsApplicationAssignmentsEl {}
impl ToListMappable for DataSsoadminPrincipalApplicationAssignmentsApplicationAssignmentsEl {
    type O = BlockAssignable<DataSsoadminPrincipalApplicationAssignmentsApplicationAssignmentsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataSsoadminPrincipalApplicationAssignmentsApplicationAssignmentsEl {}
impl BuildDataSsoadminPrincipalApplicationAssignmentsApplicationAssignmentsEl {
    pub fn build(self) -> DataSsoadminPrincipalApplicationAssignmentsApplicationAssignmentsEl {
        DataSsoadminPrincipalApplicationAssignmentsApplicationAssignmentsEl {}
    }
}
pub struct DataSsoadminPrincipalApplicationAssignmentsApplicationAssignmentsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataSsoadminPrincipalApplicationAssignmentsApplicationAssignmentsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataSsoadminPrincipalApplicationAssignmentsApplicationAssignmentsElRef {
        DataSsoadminPrincipalApplicationAssignmentsApplicationAssignmentsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataSsoadminPrincipalApplicationAssignmentsApplicationAssignmentsElRef {
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
#[derive(Serialize, Default)]
struct DataSsoadminPrincipalApplicationAssignmentsDynamic {
    application_assignments:
        Option<DynamicBlock<DataSsoadminPrincipalApplicationAssignmentsApplicationAssignmentsEl>>,
}
