use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataApphubApplicationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    application_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    project: PrimField<String>,
}
struct DataApphubApplication_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataApphubApplicationData>,
}
#[derive(Clone)]
pub struct DataApphubApplication(Rc<DataApphubApplication_>);
impl DataApphubApplication {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(&self, provider: &ProviderGoogle) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `application_id` after provisioning.\nRequired. The Application identifier."]
    pub fn application_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.application_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `attributes` after provisioning.\nConsumer provided attributes."]
    pub fn attributes(&self) -> ListRef<DataApphubApplicationAttributesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.attributes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Create time."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nOptional. User-defined description of an Application."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nOptional. User-defined name for the Application."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nPart of 'parent'. See documentation of 'projectsId'."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The resource name of an Application. Format:\n\"projects/{host-project-id}/locations/{location}/applications/{application-id}\""]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `scope` after provisioning.\nScope of an application."]
    pub fn scope(&self) -> ListRef<DataApphubApplicationScopeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.scope", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nOutput only. Application state. \n Possible values:\n STATE_UNSPECIFIED\nCREATING\nACTIVE\nDELETING"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nOutput only. A universally unique identifier (in UUID4 format) for the 'Application'."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. Update time."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
}
impl Referable for DataApphubApplication {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataApphubApplication {}
impl ToListMappable for DataApphubApplication {
    type O = ListRef<DataApphubApplicationRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataApphubApplication_ {
    fn extract_datasource_type(&self) -> String {
        "google_apphub_application".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataApphubApplication {
    pub tf_id: String,
    #[doc = "Required. The Application identifier."]
    pub application_id: PrimField<String>,
    #[doc = "Part of 'parent'. See documentation of 'projectsId'."]
    pub location: PrimField<String>,
    #[doc = ""]
    pub project: PrimField<String>,
}
impl BuildDataApphubApplication {
    pub fn build(self, stack: &mut Stack) -> DataApphubApplication {
        let out = DataApphubApplication(Rc::new(DataApphubApplication_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataApphubApplicationData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                application_id: self.application_id,
                id: core::default::Default::default(),
                location: self.location,
                project: self.project,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataApphubApplicationRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataApphubApplicationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataApphubApplicationRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `application_id` after provisioning.\nRequired. The Application identifier."]
    pub fn application_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.application_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `attributes` after provisioning.\nConsumer provided attributes."]
    pub fn attributes(&self) -> ListRef<DataApphubApplicationAttributesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.attributes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Create time."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nOptional. User-defined description of an Application."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nOptional. User-defined name for the Application."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nPart of 'parent'. See documentation of 'projectsId'."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The resource name of an Application. Format:\n\"projects/{host-project-id}/locations/{location}/applications/{application-id}\""]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `scope` after provisioning.\nScope of an application."]
    pub fn scope(&self) -> ListRef<DataApphubApplicationScopeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.scope", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nOutput only. Application state. \n Possible values:\n STATE_UNSPECIFIED\nCREATING\nACTIVE\nDELETING"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nOutput only. A universally unique identifier (in UUID4 format) for the 'Application'."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. Update time."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataApphubApplicationAttributesElBusinessOwnersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<PrimField<String>>,
}
impl DataApphubApplicationAttributesElBusinessOwnersEl {
    #[doc = "Set the field `display_name`.\n"]
    pub fn set_display_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `email`.\n"]
    pub fn set_email(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.email = Some(v.into());
        self
    }
}
impl ToListMappable for DataApphubApplicationAttributesElBusinessOwnersEl {
    type O = BlockAssignable<DataApphubApplicationAttributesElBusinessOwnersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataApphubApplicationAttributesElBusinessOwnersEl {}
impl BuildDataApphubApplicationAttributesElBusinessOwnersEl {
    pub fn build(self) -> DataApphubApplicationAttributesElBusinessOwnersEl {
        DataApphubApplicationAttributesElBusinessOwnersEl {
            display_name: core::default::Default::default(),
            email: core::default::Default::default(),
        }
    }
}
pub struct DataApphubApplicationAttributesElBusinessOwnersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataApphubApplicationAttributesElBusinessOwnersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataApphubApplicationAttributesElBusinessOwnersElRef {
        DataApphubApplicationAttributesElBusinessOwnersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataApphubApplicationAttributesElBusinessOwnersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
    #[doc = "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.email", self.base))
    }
}
#[derive(Serialize)]
pub struct DataApphubApplicationAttributesElCriticalityEl {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl DataApphubApplicationAttributesElCriticalityEl {
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable for DataApphubApplicationAttributesElCriticalityEl {
    type O = BlockAssignable<DataApphubApplicationAttributesElCriticalityEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataApphubApplicationAttributesElCriticalityEl {}
impl BuildDataApphubApplicationAttributesElCriticalityEl {
    pub fn build(self) -> DataApphubApplicationAttributesElCriticalityEl {
        DataApphubApplicationAttributesElCriticalityEl {
            type_: core::default::Default::default(),
        }
    }
}
pub struct DataApphubApplicationAttributesElCriticalityElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataApphubApplicationAttributesElCriticalityElRef {
    fn new(shared: StackShared, base: String) -> DataApphubApplicationAttributesElCriticalityElRef {
        DataApphubApplicationAttributesElCriticalityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataApphubApplicationAttributesElCriticalityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct DataApphubApplicationAttributesElDeveloperOwnersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<PrimField<String>>,
}
impl DataApphubApplicationAttributesElDeveloperOwnersEl {
    #[doc = "Set the field `display_name`.\n"]
    pub fn set_display_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `email`.\n"]
    pub fn set_email(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.email = Some(v.into());
        self
    }
}
impl ToListMappable for DataApphubApplicationAttributesElDeveloperOwnersEl {
    type O = BlockAssignable<DataApphubApplicationAttributesElDeveloperOwnersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataApphubApplicationAttributesElDeveloperOwnersEl {}
impl BuildDataApphubApplicationAttributesElDeveloperOwnersEl {
    pub fn build(self) -> DataApphubApplicationAttributesElDeveloperOwnersEl {
        DataApphubApplicationAttributesElDeveloperOwnersEl {
            display_name: core::default::Default::default(),
            email: core::default::Default::default(),
        }
    }
}
pub struct DataApphubApplicationAttributesElDeveloperOwnersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataApphubApplicationAttributesElDeveloperOwnersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataApphubApplicationAttributesElDeveloperOwnersElRef {
        DataApphubApplicationAttributesElDeveloperOwnersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataApphubApplicationAttributesElDeveloperOwnersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
    #[doc = "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.email", self.base))
    }
}
#[derive(Serialize)]
pub struct DataApphubApplicationAttributesElEnvironmentEl {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl DataApphubApplicationAttributesElEnvironmentEl {
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable for DataApphubApplicationAttributesElEnvironmentEl {
    type O = BlockAssignable<DataApphubApplicationAttributesElEnvironmentEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataApphubApplicationAttributesElEnvironmentEl {}
impl BuildDataApphubApplicationAttributesElEnvironmentEl {
    pub fn build(self) -> DataApphubApplicationAttributesElEnvironmentEl {
        DataApphubApplicationAttributesElEnvironmentEl {
            type_: core::default::Default::default(),
        }
    }
}
pub struct DataApphubApplicationAttributesElEnvironmentElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataApphubApplicationAttributesElEnvironmentElRef {
    fn new(shared: StackShared, base: String) -> DataApphubApplicationAttributesElEnvironmentElRef {
        DataApphubApplicationAttributesElEnvironmentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataApphubApplicationAttributesElEnvironmentElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct DataApphubApplicationAttributesElOperatorOwnersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<PrimField<String>>,
}
impl DataApphubApplicationAttributesElOperatorOwnersEl {
    #[doc = "Set the field `display_name`.\n"]
    pub fn set_display_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `email`.\n"]
    pub fn set_email(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.email = Some(v.into());
        self
    }
}
impl ToListMappable for DataApphubApplicationAttributesElOperatorOwnersEl {
    type O = BlockAssignable<DataApphubApplicationAttributesElOperatorOwnersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataApphubApplicationAttributesElOperatorOwnersEl {}
impl BuildDataApphubApplicationAttributesElOperatorOwnersEl {
    pub fn build(self) -> DataApphubApplicationAttributesElOperatorOwnersEl {
        DataApphubApplicationAttributesElOperatorOwnersEl {
            display_name: core::default::Default::default(),
            email: core::default::Default::default(),
        }
    }
}
pub struct DataApphubApplicationAttributesElOperatorOwnersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataApphubApplicationAttributesElOperatorOwnersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataApphubApplicationAttributesElOperatorOwnersElRef {
        DataApphubApplicationAttributesElOperatorOwnersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataApphubApplicationAttributesElOperatorOwnersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
    #[doc = "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.email", self.base))
    }
}
#[derive(Serialize)]
pub struct DataApphubApplicationAttributesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    business_owners: Option<ListField<DataApphubApplicationAttributesElBusinessOwnersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    criticality: Option<ListField<DataApphubApplicationAttributesElCriticalityEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    developer_owners: Option<ListField<DataApphubApplicationAttributesElDeveloperOwnersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    environment: Option<ListField<DataApphubApplicationAttributesElEnvironmentEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    operator_owners: Option<ListField<DataApphubApplicationAttributesElOperatorOwnersEl>>,
}
impl DataApphubApplicationAttributesEl {
    #[doc = "Set the field `business_owners`.\n"]
    pub fn set_business_owners(
        mut self,
        v: impl Into<ListField<DataApphubApplicationAttributesElBusinessOwnersEl>>,
    ) -> Self {
        self.business_owners = Some(v.into());
        self
    }
    #[doc = "Set the field `criticality`.\n"]
    pub fn set_criticality(
        mut self,
        v: impl Into<ListField<DataApphubApplicationAttributesElCriticalityEl>>,
    ) -> Self {
        self.criticality = Some(v.into());
        self
    }
    #[doc = "Set the field `developer_owners`.\n"]
    pub fn set_developer_owners(
        mut self,
        v: impl Into<ListField<DataApphubApplicationAttributesElDeveloperOwnersEl>>,
    ) -> Self {
        self.developer_owners = Some(v.into());
        self
    }
    #[doc = "Set the field `environment`.\n"]
    pub fn set_environment(
        mut self,
        v: impl Into<ListField<DataApphubApplicationAttributesElEnvironmentEl>>,
    ) -> Self {
        self.environment = Some(v.into());
        self
    }
    #[doc = "Set the field `operator_owners`.\n"]
    pub fn set_operator_owners(
        mut self,
        v: impl Into<ListField<DataApphubApplicationAttributesElOperatorOwnersEl>>,
    ) -> Self {
        self.operator_owners = Some(v.into());
        self
    }
}
impl ToListMappable for DataApphubApplicationAttributesEl {
    type O = BlockAssignable<DataApphubApplicationAttributesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataApphubApplicationAttributesEl {}
impl BuildDataApphubApplicationAttributesEl {
    pub fn build(self) -> DataApphubApplicationAttributesEl {
        DataApphubApplicationAttributesEl {
            business_owners: core::default::Default::default(),
            criticality: core::default::Default::default(),
            developer_owners: core::default::Default::default(),
            environment: core::default::Default::default(),
            operator_owners: core::default::Default::default(),
        }
    }
}
pub struct DataApphubApplicationAttributesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataApphubApplicationAttributesElRef {
    fn new(shared: StackShared, base: String) -> DataApphubApplicationAttributesElRef {
        DataApphubApplicationAttributesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataApphubApplicationAttributesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `business_owners` after provisioning.\n"]
    pub fn business_owners(&self) -> ListRef<DataApphubApplicationAttributesElBusinessOwnersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.business_owners", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `criticality` after provisioning.\n"]
    pub fn criticality(&self) -> ListRef<DataApphubApplicationAttributesElCriticalityElRef> {
        ListRef::new(self.shared().clone(), format!("{}.criticality", self.base))
    }
    #[doc = "Get a reference to the value of field `developer_owners` after provisioning.\n"]
    pub fn developer_owners(
        &self,
    ) -> ListRef<DataApphubApplicationAttributesElDeveloperOwnersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.developer_owners", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `environment` after provisioning.\n"]
    pub fn environment(&self) -> ListRef<DataApphubApplicationAttributesElEnvironmentElRef> {
        ListRef::new(self.shared().clone(), format!("{}.environment", self.base))
    }
    #[doc = "Get a reference to the value of field `operator_owners` after provisioning.\n"]
    pub fn operator_owners(&self) -> ListRef<DataApphubApplicationAttributesElOperatorOwnersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.operator_owners", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataApphubApplicationScopeEl {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl DataApphubApplicationScopeEl {
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable for DataApphubApplicationScopeEl {
    type O = BlockAssignable<DataApphubApplicationScopeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataApphubApplicationScopeEl {}
impl BuildDataApphubApplicationScopeEl {
    pub fn build(self) -> DataApphubApplicationScopeEl {
        DataApphubApplicationScopeEl {
            type_: core::default::Default::default(),
        }
    }
}
pub struct DataApphubApplicationScopeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataApphubApplicationScopeElRef {
    fn new(shared: StackShared, base: String) -> DataApphubApplicationScopeElRef {
        DataApphubApplicationScopeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataApphubApplicationScopeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
