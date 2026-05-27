use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct BigqueryDatasetAccessData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    dataset_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    domain: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group_by_email: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iam_member: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    role: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    special_group: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_by_email: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    condition: Option<Vec<BigqueryDatasetAccessConditionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dataset: Option<Vec<BigqueryDatasetAccessDatasetEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    routine: Option<Vec<BigqueryDatasetAccessRoutineEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<BigqueryDatasetAccessTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    view: Option<Vec<BigqueryDatasetAccessViewEl>>,
    dynamic: BigqueryDatasetAccessDynamic,
}
struct BigqueryDatasetAccess_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<BigqueryDatasetAccessData>,
}
#[derive(Clone)]
pub struct BigqueryDatasetAccess(Rc<BigqueryDatasetAccess_>);
impl BigqueryDatasetAccess {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(self, provider: &ProviderGoogle) -> Self {
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
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `domain`.\nA domain to grant access to. Any users signed in with the\ndomain specified will be granted the specified access"]
    pub fn set_domain(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().domain = Some(v.into());
        self
    }
    #[doc = "Set the field `group_by_email`.\nAn email address of a Google Group to grant access to."]
    pub fn set_group_by_email(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().group_by_email = Some(v.into());
        self
    }
    #[doc = "Set the field `iam_member`.\nSome other type of member that appears in the IAM Policy but isn't a user,\ngroup, domain, or special group. For example: 'allUsers'"]
    pub fn set_iam_member(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().iam_member = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `role`.\nDescribes the rights granted to the user specified by the other\nmember of the access object. Basic, predefined, and custom roles are\nsupported. Predefined roles that have equivalent basic roles are\nswapped by the API to their basic counterparts, and will show a diff\npost-create. See\n[official docs](https://cloud.google.com/bigquery/docs/access-control)."]
    pub fn set_role(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().role = Some(v.into());
        self
    }
    #[doc = "Set the field `special_group`.\nA special group to grant access to. Possible values include:\n* 'projectOwners': Owners of the enclosing project.\n* 'projectReaders': Readers of the enclosing project.\n* 'projectWriters': Writers of the enclosing project.\n* 'allAuthenticatedUsers': All authenticated BigQuery users."]
    pub fn set_special_group(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().special_group = Some(v.into());
        self
    }
    #[doc = "Set the field `user_by_email`.\nAn email address of a user to grant access to. For example:\nfred@example.com"]
    pub fn set_user_by_email(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().user_by_email = Some(v.into());
        self
    }
    #[doc = "Set the field `condition`.\n"]
    pub fn set_condition(
        self,
        v: impl Into<BlockAssignable<BigqueryDatasetAccessConditionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().condition = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.condition = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `dataset`.\n"]
    pub fn set_dataset(
        self,
        v: impl Into<BlockAssignable<BigqueryDatasetAccessDatasetEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().dataset = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.dataset = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `routine`.\n"]
    pub fn set_routine(
        self,
        v: impl Into<BlockAssignable<BigqueryDatasetAccessRoutineEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().routine = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.routine = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<BigqueryDatasetAccessTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Set the field `view`.\n"]
    pub fn set_view(self, v: impl Into<BlockAssignable<BigqueryDatasetAccessViewEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().view = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.view = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `api_updated_member` after provisioning.\nIf true, represents that that the iam_member in the config was translated to a different member type by the API, and is stored in state as a different member type"]
    pub fn api_updated_member(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.api_updated_member", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dataset_id` after provisioning.\nA unique ID for this dataset, without the project name. The ID\nmust contain only letters (a-z, A-Z), numbers (0-9), or\nunderscores (_). The maximum length is 1,024 characters."]
    pub fn dataset_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dataset_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `domain` after provisioning.\nA domain to grant access to. Any users signed in with the\ndomain specified will be granted the specified access"]
    pub fn domain(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `group_by_email` after provisioning.\nAn email address of a Google Group to grant access to."]
    pub fn group_by_email(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.group_by_email", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `iam_member` after provisioning.\nSome other type of member that appears in the IAM Policy but isn't a user,\ngroup, domain, or special group. For example: 'allUsers'"]
    pub fn iam_member(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.iam_member", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `role` after provisioning.\nDescribes the rights granted to the user specified by the other\nmember of the access object. Basic, predefined, and custom roles are\nsupported. Predefined roles that have equivalent basic roles are\nswapped by the API to their basic counterparts, and will show a diff\npost-create. See\n[official docs](https://cloud.google.com/bigquery/docs/access-control)."]
    pub fn role(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.role", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `special_group` after provisioning.\nA special group to grant access to. Possible values include:\n* 'projectOwners': Owners of the enclosing project.\n* 'projectReaders': Readers of the enclosing project.\n* 'projectWriters': Writers of the enclosing project.\n* 'allAuthenticatedUsers': All authenticated BigQuery users."]
    pub fn special_group(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.special_group", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `user_by_email` after provisioning.\nAn email address of a user to grant access to. For example:\nfred@example.com"]
    pub fn user_by_email(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_by_email", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `condition` after provisioning.\n"]
    pub fn condition(&self) -> ListRef<BigqueryDatasetAccessConditionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.condition", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dataset` after provisioning.\n"]
    pub fn dataset(&self) -> ListRef<BigqueryDatasetAccessDatasetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dataset", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `routine` after provisioning.\n"]
    pub fn routine(&self) -> ListRef<BigqueryDatasetAccessRoutineElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.routine", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BigqueryDatasetAccessTimeoutsElRef {
        BigqueryDatasetAccessTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `view` after provisioning.\n"]
    pub fn view(&self) -> ListRef<BigqueryDatasetAccessViewElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.view", self.extract_ref()),
        )
    }
}
impl Referable for BigqueryDatasetAccess {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for BigqueryDatasetAccess {}
impl ToListMappable for BigqueryDatasetAccess {
    type O = ListRef<BigqueryDatasetAccessRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for BigqueryDatasetAccess_ {
    fn extract_resource_type(&self) -> String {
        "google_bigquery_dataset_access".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildBigqueryDatasetAccess {
    pub tf_id: String,
    #[doc = "A unique ID for this dataset, without the project name. The ID\nmust contain only letters (a-z, A-Z), numbers (0-9), or\nunderscores (_). The maximum length is 1,024 characters."]
    pub dataset_id: PrimField<String>,
}
impl BuildBigqueryDatasetAccess {
    pub fn build(self, stack: &mut Stack) -> BigqueryDatasetAccess {
        let out = BigqueryDatasetAccess(Rc::new(BigqueryDatasetAccess_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(BigqueryDatasetAccessData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                dataset_id: self.dataset_id,
                deletion_policy: core::default::Default::default(),
                domain: core::default::Default::default(),
                group_by_email: core::default::Default::default(),
                iam_member: core::default::Default::default(),
                id: core::default::Default::default(),
                project: core::default::Default::default(),
                role: core::default::Default::default(),
                special_group: core::default::Default::default(),
                user_by_email: core::default::Default::default(),
                condition: core::default::Default::default(),
                dataset: core::default::Default::default(),
                routine: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                view: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct BigqueryDatasetAccessRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryDatasetAccessRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl BigqueryDatasetAccessRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `api_updated_member` after provisioning.\nIf true, represents that that the iam_member in the config was translated to a different member type by the API, and is stored in state as a different member type"]
    pub fn api_updated_member(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.api_updated_member", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dataset_id` after provisioning.\nA unique ID for this dataset, without the project name. The ID\nmust contain only letters (a-z, A-Z), numbers (0-9), or\nunderscores (_). The maximum length is 1,024 characters."]
    pub fn dataset_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dataset_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `domain` after provisioning.\nA domain to grant access to. Any users signed in with the\ndomain specified will be granted the specified access"]
    pub fn domain(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `group_by_email` after provisioning.\nAn email address of a Google Group to grant access to."]
    pub fn group_by_email(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.group_by_email", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `iam_member` after provisioning.\nSome other type of member that appears in the IAM Policy but isn't a user,\ngroup, domain, or special group. For example: 'allUsers'"]
    pub fn iam_member(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.iam_member", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `role` after provisioning.\nDescribes the rights granted to the user specified by the other\nmember of the access object. Basic, predefined, and custom roles are\nsupported. Predefined roles that have equivalent basic roles are\nswapped by the API to their basic counterparts, and will show a diff\npost-create. See\n[official docs](https://cloud.google.com/bigquery/docs/access-control)."]
    pub fn role(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.role", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `special_group` after provisioning.\nA special group to grant access to. Possible values include:\n* 'projectOwners': Owners of the enclosing project.\n* 'projectReaders': Readers of the enclosing project.\n* 'projectWriters': Writers of the enclosing project.\n* 'allAuthenticatedUsers': All authenticated BigQuery users."]
    pub fn special_group(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.special_group", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `user_by_email` after provisioning.\nAn email address of a user to grant access to. For example:\nfred@example.com"]
    pub fn user_by_email(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_by_email", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `condition` after provisioning.\n"]
    pub fn condition(&self) -> ListRef<BigqueryDatasetAccessConditionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.condition", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dataset` after provisioning.\n"]
    pub fn dataset(&self) -> ListRef<BigqueryDatasetAccessDatasetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dataset", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `routine` after provisioning.\n"]
    pub fn routine(&self) -> ListRef<BigqueryDatasetAccessRoutineElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.routine", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BigqueryDatasetAccessTimeoutsElRef {
        BigqueryDatasetAccessTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `view` after provisioning.\n"]
    pub fn view(&self) -> ListRef<BigqueryDatasetAccessViewElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.view", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct BigqueryDatasetAccessConditionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    expression: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<PrimField<String>>,
}
impl BigqueryDatasetAccessConditionEl {
    #[doc = "Set the field `description`.\nDescription of the expression. This is a longer text which describes the expression,\ne.g. when hovered over it in a UI."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `location`.\nString indicating the location of the expression for error reporting, e.g. a file\nname and a position in the file."]
    pub fn set_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.location = Some(v.into());
        self
    }
    #[doc = "Set the field `title`.\nTitle for the expression, i.e. a short string describing its purpose.\nThis can be used e.g. in UIs which allow to enter the expression."]
    pub fn set_title(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.title = Some(v.into());
        self
    }
}
impl ToListMappable for BigqueryDatasetAccessConditionEl {
    type O = BlockAssignable<BigqueryDatasetAccessConditionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBigqueryDatasetAccessConditionEl {
    #[doc = "Textual representation of an expression in Common Expression Language syntax."]
    pub expression: PrimField<String>,
}
impl BuildBigqueryDatasetAccessConditionEl {
    pub fn build(self) -> BigqueryDatasetAccessConditionEl {
        BigqueryDatasetAccessConditionEl {
            description: core::default::Default::default(),
            expression: self.expression,
            location: core::default::Default::default(),
            title: core::default::Default::default(),
        }
    }
}
pub struct BigqueryDatasetAccessConditionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryDatasetAccessConditionElRef {
    fn new(shared: StackShared, base: String) -> BigqueryDatasetAccessConditionElRef {
        BigqueryDatasetAccessConditionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BigqueryDatasetAccessConditionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription of the expression. This is a longer text which describes the expression,\ne.g. when hovered over it in a UI."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `expression` after provisioning.\nTextual representation of an expression in Common Expression Language syntax."]
    pub fn expression(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nString indicating the location of the expression for error reporting, e.g. a file\nname and a position in the file."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }
    #[doc = "Get a reference to the value of field `title` after provisioning.\nTitle for the expression, i.e. a short string describing its purpose.\nThis can be used e.g. in UIs which allow to enter the expression."]
    pub fn title(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.title", self.base))
    }
}
#[derive(Serialize)]
pub struct BigqueryDatasetAccessDatasetElDatasetEl {
    dataset_id: PrimField<String>,
    project_id: PrimField<String>,
}
impl BigqueryDatasetAccessDatasetElDatasetEl {}
impl ToListMappable for BigqueryDatasetAccessDatasetElDatasetEl {
    type O = BlockAssignable<BigqueryDatasetAccessDatasetElDatasetEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBigqueryDatasetAccessDatasetElDatasetEl {
    #[doc = "The ID of the dataset containing this table."]
    pub dataset_id: PrimField<String>,
    #[doc = "The ID of the project containing this table."]
    pub project_id: PrimField<String>,
}
impl BuildBigqueryDatasetAccessDatasetElDatasetEl {
    pub fn build(self) -> BigqueryDatasetAccessDatasetElDatasetEl {
        BigqueryDatasetAccessDatasetElDatasetEl {
            dataset_id: self.dataset_id,
            project_id: self.project_id,
        }
    }
}
pub struct BigqueryDatasetAccessDatasetElDatasetElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryDatasetAccessDatasetElDatasetElRef {
    fn new(shared: StackShared, base: String) -> BigqueryDatasetAccessDatasetElDatasetElRef {
        BigqueryDatasetAccessDatasetElDatasetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BigqueryDatasetAccessDatasetElDatasetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `dataset_id` after provisioning.\nThe ID of the dataset containing this table."]
    pub fn dataset_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dataset_id", self.base))
    }
    #[doc = "Get a reference to the value of field `project_id` after provisioning.\nThe ID of the project containing this table."]
    pub fn project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }
}
#[derive(Serialize, Default)]
struct BigqueryDatasetAccessDatasetElDynamic {
    dataset: Option<DynamicBlock<BigqueryDatasetAccessDatasetElDatasetEl>>,
}
#[derive(Serialize)]
pub struct BigqueryDatasetAccessDatasetEl {
    target_types: ListField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dataset: Option<Vec<BigqueryDatasetAccessDatasetElDatasetEl>>,
    dynamic: BigqueryDatasetAccessDatasetElDynamic,
}
impl BigqueryDatasetAccessDatasetEl {
    #[doc = "Set the field `dataset`.\n"]
    pub fn set_dataset(
        mut self,
        v: impl Into<BlockAssignable<BigqueryDatasetAccessDatasetElDatasetEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.dataset = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.dataset = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for BigqueryDatasetAccessDatasetEl {
    type O = BlockAssignable<BigqueryDatasetAccessDatasetEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBigqueryDatasetAccessDatasetEl {
    #[doc = "Which resources in the dataset this entry applies to. Currently, only views are supported,\nbut additional target types may be added in the future. Possible values: VIEWS"]
    pub target_types: ListField<PrimField<String>>,
}
impl BuildBigqueryDatasetAccessDatasetEl {
    pub fn build(self) -> BigqueryDatasetAccessDatasetEl {
        BigqueryDatasetAccessDatasetEl {
            target_types: self.target_types,
            dataset: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct BigqueryDatasetAccessDatasetElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryDatasetAccessDatasetElRef {
    fn new(shared: StackShared, base: String) -> BigqueryDatasetAccessDatasetElRef {
        BigqueryDatasetAccessDatasetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BigqueryDatasetAccessDatasetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `target_types` after provisioning.\nWhich resources in the dataset this entry applies to. Currently, only views are supported,\nbut additional target types may be added in the future. Possible values: VIEWS"]
    pub fn target_types(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.target_types", self.base))
    }
    #[doc = "Get a reference to the value of field `dataset` after provisioning.\n"]
    pub fn dataset(&self) -> ListRef<BigqueryDatasetAccessDatasetElDatasetElRef> {
        ListRef::new(self.shared().clone(), format!("{}.dataset", self.base))
    }
}
#[derive(Serialize)]
pub struct BigqueryDatasetAccessRoutineEl {
    dataset_id: PrimField<String>,
    project_id: PrimField<String>,
    routine_id: PrimField<String>,
}
impl BigqueryDatasetAccessRoutineEl {}
impl ToListMappable for BigqueryDatasetAccessRoutineEl {
    type O = BlockAssignable<BigqueryDatasetAccessRoutineEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBigqueryDatasetAccessRoutineEl {
    #[doc = "The ID of the dataset containing this table."]
    pub dataset_id: PrimField<String>,
    #[doc = "The ID of the project containing this table."]
    pub project_id: PrimField<String>,
    #[doc = "The ID of the routine. The ID must contain only letters (a-z,\nA-Z), numbers (0-9), or underscores (_). The maximum length\nis 256 characters."]
    pub routine_id: PrimField<String>,
}
impl BuildBigqueryDatasetAccessRoutineEl {
    pub fn build(self) -> BigqueryDatasetAccessRoutineEl {
        BigqueryDatasetAccessRoutineEl {
            dataset_id: self.dataset_id,
            project_id: self.project_id,
            routine_id: self.routine_id,
        }
    }
}
pub struct BigqueryDatasetAccessRoutineElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryDatasetAccessRoutineElRef {
    fn new(shared: StackShared, base: String) -> BigqueryDatasetAccessRoutineElRef {
        BigqueryDatasetAccessRoutineElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BigqueryDatasetAccessRoutineElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `dataset_id` after provisioning.\nThe ID of the dataset containing this table."]
    pub fn dataset_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dataset_id", self.base))
    }
    #[doc = "Get a reference to the value of field `project_id` after provisioning.\nThe ID of the project containing this table."]
    pub fn project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }
    #[doc = "Get a reference to the value of field `routine_id` after provisioning.\nThe ID of the routine. The ID must contain only letters (a-z,\nA-Z), numbers (0-9), or underscores (_). The maximum length\nis 256 characters."]
    pub fn routine_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.routine_id", self.base))
    }
}
#[derive(Serialize)]
pub struct BigqueryDatasetAccessTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}
impl BigqueryDatasetAccessTimeoutsEl {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }
    #[doc = "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }
}
impl ToListMappable for BigqueryDatasetAccessTimeoutsEl {
    type O = BlockAssignable<BigqueryDatasetAccessTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBigqueryDatasetAccessTimeoutsEl {}
impl BuildBigqueryDatasetAccessTimeoutsEl {
    pub fn build(self) -> BigqueryDatasetAccessTimeoutsEl {
        BigqueryDatasetAccessTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}
pub struct BigqueryDatasetAccessTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryDatasetAccessTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> BigqueryDatasetAccessTimeoutsElRef {
        BigqueryDatasetAccessTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BigqueryDatasetAccessTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }
    #[doc = "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }
}
#[derive(Serialize)]
pub struct BigqueryDatasetAccessViewEl {
    dataset_id: PrimField<String>,
    project_id: PrimField<String>,
    table_id: PrimField<String>,
}
impl BigqueryDatasetAccessViewEl {}
impl ToListMappable for BigqueryDatasetAccessViewEl {
    type O = BlockAssignable<BigqueryDatasetAccessViewEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBigqueryDatasetAccessViewEl {
    #[doc = "The ID of the dataset containing this table."]
    pub dataset_id: PrimField<String>,
    #[doc = "The ID of the project containing this table."]
    pub project_id: PrimField<String>,
    #[doc = "The ID of the table. The ID must contain only letters (a-z,\nA-Z), numbers (0-9), or underscores (_). The maximum length\nis 1,024 characters."]
    pub table_id: PrimField<String>,
}
impl BuildBigqueryDatasetAccessViewEl {
    pub fn build(self) -> BigqueryDatasetAccessViewEl {
        BigqueryDatasetAccessViewEl {
            dataset_id: self.dataset_id,
            project_id: self.project_id,
            table_id: self.table_id,
        }
    }
}
pub struct BigqueryDatasetAccessViewElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryDatasetAccessViewElRef {
    fn new(shared: StackShared, base: String) -> BigqueryDatasetAccessViewElRef {
        BigqueryDatasetAccessViewElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BigqueryDatasetAccessViewElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `dataset_id` after provisioning.\nThe ID of the dataset containing this table."]
    pub fn dataset_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dataset_id", self.base))
    }
    #[doc = "Get a reference to the value of field `project_id` after provisioning.\nThe ID of the project containing this table."]
    pub fn project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }
    #[doc = "Get a reference to the value of field `table_id` after provisioning.\nThe ID of the table. The ID must contain only letters (a-z,\nA-Z), numbers (0-9), or underscores (_). The maximum length\nis 1,024 characters."]
    pub fn table_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.table_id", self.base))
    }
}
#[derive(Serialize, Default)]
struct BigqueryDatasetAccessDynamic {
    condition: Option<DynamicBlock<BigqueryDatasetAccessConditionEl>>,
    dataset: Option<DynamicBlock<BigqueryDatasetAccessDatasetEl>>,
    routine: Option<DynamicBlock<BigqueryDatasetAccessRoutineEl>>,
    view: Option<DynamicBlock<BigqueryDatasetAccessViewEl>>,
}
