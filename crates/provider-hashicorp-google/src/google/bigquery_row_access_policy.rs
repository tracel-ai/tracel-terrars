use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct BigqueryRowAccessPolicyData {
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
    filter_predicate: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    grantees: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    policy_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    table_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<BigqueryRowAccessPolicyTimeoutsEl>,
}
struct BigqueryRowAccessPolicy_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<BigqueryRowAccessPolicyData>,
}
#[derive(Clone)]
pub struct BigqueryRowAccessPolicy(Rc<BigqueryRowAccessPolicy_>);
impl BigqueryRowAccessPolicy {
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
    #[doc = "Set the field `grantees`.\nInput only. The optional list of iam_member users or groups that specifies the initial\nmembers that the row-level access policy should be created with.\n\ngrantees types:\n- \"user:alice@example.com\": An email address that represents a specific\nGoogle account.\n- \"serviceAccount:my-other-app@appspot.gserviceaccount.com\": An email\naddress that represents a service account.\n- \"group:admins@example.com\": An email address that represents a Google\ngroup.\n- \"domain:example.com\":The Google Workspace domain (primary) that\nrepresents all the users of that domain.\n- \"allAuthenticatedUsers\": A special identifier that represents all service\naccounts and all users on the internet who have authenticated with a Google\nAccount. This identifier includes accounts that aren't connected to a\nGoogle Workspace or Cloud Identity domain, such as personal Gmail accounts.\nUsers who aren't authenticated, such as anonymous visitors, aren't\nincluded.\n- \"allUsers\":A special identifier that represents anyone who is on\nthe internet, including authenticated and unauthenticated users. Because\nBigQuery requires authentication before a user can access the service,\nallUsers includes only authenticated users."]
    pub fn set_grantees(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().grantees = Some(v.into());
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
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<BigqueryRowAccessPolicyTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `creation_time` after provisioning.\nThe time when this row access policy was created, in milliseconds since\nthe epoch."]
    pub fn creation_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creation_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dataset_id` after provisioning.\nThe ID of the dataset containing this row access policy."]
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
    #[doc = "Get a reference to the value of field `filter_predicate` after provisioning.\nA SQL boolean expression that represents the rows defined by this row\naccess policy, similar to the boolean expression in a WHERE clause of a\nSELECT query on a table.\nReferences to other tables, routines, and temporary functions are not\nsupported.\n\nExamples: region=\"EU\"\ndate_field = CAST('2019-9-27' as DATE)\nnullable_field is not NULL\nnumeric_field BETWEEN 1.0 AND 5.0"]
    pub fn filter_predicate(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.filter_predicate", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `grantees` after provisioning.\nInput only. The optional list of iam_member users or groups that specifies the initial\nmembers that the row-level access policy should be created with.\n\ngrantees types:\n- \"user:alice@example.com\": An email address that represents a specific\nGoogle account.\n- \"serviceAccount:my-other-app@appspot.gserviceaccount.com\": An email\naddress that represents a service account.\n- \"group:admins@example.com\": An email address that represents a Google\ngroup.\n- \"domain:example.com\":The Google Workspace domain (primary) that\nrepresents all the users of that domain.\n- \"allAuthenticatedUsers\": A special identifier that represents all service\naccounts and all users on the internet who have authenticated with a Google\nAccount. This identifier includes accounts that aren't connected to a\nGoogle Workspace or Cloud Identity domain, such as personal Gmail accounts.\nUsers who aren't authenticated, such as anonymous visitors, aren't\nincluded.\n- \"allUsers\":A special identifier that represents anyone who is on\nthe internet, including authenticated and unauthenticated users. Because\nBigQuery requires authentication before a user can access the service,\nallUsers includes only authenticated users."]
    pub fn grantees(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.grantees", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `last_modified_time` after provisioning.\nThe time when this row access policy was last modified, in milliseconds\nsince the epoch."]
    pub fn last_modified_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_modified_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `policy_id` after provisioning.\nThe ID of the row access policy. The ID must contain only\nletters (a-z, A-Z), numbers (0-9), or underscores (_). The maximum\nlength is 256 characters."]
    pub fn policy_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.policy_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `table_id` after provisioning.\nThe ID of the table containing this row access policy."]
    pub fn table_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.table_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BigqueryRowAccessPolicyTimeoutsElRef {
        BigqueryRowAccessPolicyTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for BigqueryRowAccessPolicy {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for BigqueryRowAccessPolicy {}
impl ToListMappable for BigqueryRowAccessPolicy {
    type O = ListRef<BigqueryRowAccessPolicyRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for BigqueryRowAccessPolicy_ {
    fn extract_resource_type(&self) -> String {
        "google_bigquery_row_access_policy".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildBigqueryRowAccessPolicy {
    pub tf_id: String,
    #[doc = "The ID of the dataset containing this row access policy."]
    pub dataset_id: PrimField<String>,
    #[doc = "A SQL boolean expression that represents the rows defined by this row\naccess policy, similar to the boolean expression in a WHERE clause of a\nSELECT query on a table.\nReferences to other tables, routines, and temporary functions are not\nsupported.\n\nExamples: region=\"EU\"\ndate_field = CAST('2019-9-27' as DATE)\nnullable_field is not NULL\nnumeric_field BETWEEN 1.0 AND 5.0"]
    pub filter_predicate: PrimField<String>,
    #[doc = "The ID of the row access policy. The ID must contain only\nletters (a-z, A-Z), numbers (0-9), or underscores (_). The maximum\nlength is 256 characters."]
    pub policy_id: PrimField<String>,
    #[doc = "The ID of the table containing this row access policy."]
    pub table_id: PrimField<String>,
}
impl BuildBigqueryRowAccessPolicy {
    pub fn build(self, stack: &mut Stack) -> BigqueryRowAccessPolicy {
        let out = BigqueryRowAccessPolicy(Rc::new(BigqueryRowAccessPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(BigqueryRowAccessPolicyData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                dataset_id: self.dataset_id,
                deletion_policy: core::default::Default::default(),
                filter_predicate: self.filter_predicate,
                grantees: core::default::Default::default(),
                id: core::default::Default::default(),
                policy_id: self.policy_id,
                project: core::default::Default::default(),
                table_id: self.table_id,
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct BigqueryRowAccessPolicyRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryRowAccessPolicyRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl BigqueryRowAccessPolicyRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `creation_time` after provisioning.\nThe time when this row access policy was created, in milliseconds since\nthe epoch."]
    pub fn creation_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creation_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dataset_id` after provisioning.\nThe ID of the dataset containing this row access policy."]
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
    #[doc = "Get a reference to the value of field `filter_predicate` after provisioning.\nA SQL boolean expression that represents the rows defined by this row\naccess policy, similar to the boolean expression in a WHERE clause of a\nSELECT query on a table.\nReferences to other tables, routines, and temporary functions are not\nsupported.\n\nExamples: region=\"EU\"\ndate_field = CAST('2019-9-27' as DATE)\nnullable_field is not NULL\nnumeric_field BETWEEN 1.0 AND 5.0"]
    pub fn filter_predicate(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.filter_predicate", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `grantees` after provisioning.\nInput only. The optional list of iam_member users or groups that specifies the initial\nmembers that the row-level access policy should be created with.\n\ngrantees types:\n- \"user:alice@example.com\": An email address that represents a specific\nGoogle account.\n- \"serviceAccount:my-other-app@appspot.gserviceaccount.com\": An email\naddress that represents a service account.\n- \"group:admins@example.com\": An email address that represents a Google\ngroup.\n- \"domain:example.com\":The Google Workspace domain (primary) that\nrepresents all the users of that domain.\n- \"allAuthenticatedUsers\": A special identifier that represents all service\naccounts and all users on the internet who have authenticated with a Google\nAccount. This identifier includes accounts that aren't connected to a\nGoogle Workspace or Cloud Identity domain, such as personal Gmail accounts.\nUsers who aren't authenticated, such as anonymous visitors, aren't\nincluded.\n- \"allUsers\":A special identifier that represents anyone who is on\nthe internet, including authenticated and unauthenticated users. Because\nBigQuery requires authentication before a user can access the service,\nallUsers includes only authenticated users."]
    pub fn grantees(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.grantees", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `last_modified_time` after provisioning.\nThe time when this row access policy was last modified, in milliseconds\nsince the epoch."]
    pub fn last_modified_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_modified_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `policy_id` after provisioning.\nThe ID of the row access policy. The ID must contain only\nletters (a-z, A-Z), numbers (0-9), or underscores (_). The maximum\nlength is 256 characters."]
    pub fn policy_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.policy_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `table_id` after provisioning.\nThe ID of the table containing this row access policy."]
    pub fn table_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.table_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BigqueryRowAccessPolicyTimeoutsElRef {
        BigqueryRowAccessPolicyTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct BigqueryRowAccessPolicyTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl BigqueryRowAccessPolicyTimeoutsEl {
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
    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}
impl ToListMappable for BigqueryRowAccessPolicyTimeoutsEl {
    type O = BlockAssignable<BigqueryRowAccessPolicyTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBigqueryRowAccessPolicyTimeoutsEl {}
impl BuildBigqueryRowAccessPolicyTimeoutsEl {
    pub fn build(self) -> BigqueryRowAccessPolicyTimeoutsEl {
        BigqueryRowAccessPolicyTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct BigqueryRowAccessPolicyTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryRowAccessPolicyTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> BigqueryRowAccessPolicyTimeoutsElRef {
        BigqueryRowAccessPolicyTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BigqueryRowAccessPolicyTimeoutsElRef {
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
    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
