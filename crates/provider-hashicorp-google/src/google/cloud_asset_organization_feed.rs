use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct CloudAssetOrganizationFeedData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    asset_names: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    asset_types: Option<ListField<PrimField<String>>>,
    billing_project: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    feed_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    org_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    condition: Option<Vec<CloudAssetOrganizationFeedConditionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    feed_output_config: Option<Vec<CloudAssetOrganizationFeedFeedOutputConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<CloudAssetOrganizationFeedTimeoutsEl>,
    dynamic: CloudAssetOrganizationFeedDynamic,
}
struct CloudAssetOrganizationFeed_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CloudAssetOrganizationFeedData>,
}
#[derive(Clone)]
pub struct CloudAssetOrganizationFeed(Rc<CloudAssetOrganizationFeed_>);
impl CloudAssetOrganizationFeed {
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
    #[doc = "Set the field `asset_names`.\nA list of the full names of the assets to receive updates. You must specify either or both of\nassetNames and assetTypes. Only asset updates matching specified assetNames and assetTypes are\nexported to the feed. For example: //compute.googleapis.com/projects/my_project_123/zones/zone1/instances/instance1.\nSee https://cloud.google.com/apis/design/resourceNames#fullResourceName for more info."]
    pub fn set_asset_names(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().asset_names = Some(v.into());
        self
    }
    #[doc = "Set the field `asset_types`.\nA list of types of the assets to receive updates. You must specify either or both of assetNames\nand assetTypes. Only asset updates matching specified assetNames and assetTypes are exported to\nthe feed. For example: \"compute.googleapis.com/Disk\"\nSee https://cloud.google.com/asset-inventory/docs/supported-asset-types for a list of all\nsupported asset types."]
    pub fn set_asset_types(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().asset_types = Some(v.into());
        self
    }
    #[doc = "Set the field `content_type`.\nAsset content type. If not specified, no content but the asset name and type will be returned. Possible values: [\"CONTENT_TYPE_UNSPECIFIED\", \"RESOURCE\", \"IAM_POLICY\", \"ORG_POLICY\", \"OS_INVENTORY\", \"ACCESS_POLICY\"]"]
    pub fn set_content_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().content_type = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `condition`.\n"]
    pub fn set_condition(
        self,
        v: impl Into<BlockAssignable<CloudAssetOrganizationFeedConditionEl>>,
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
    #[doc = "Set the field `feed_output_config`.\n"]
    pub fn set_feed_output_config(
        self,
        v: impl Into<BlockAssignable<CloudAssetOrganizationFeedFeedOutputConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().feed_output_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.feed_output_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<CloudAssetOrganizationFeedTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `asset_names` after provisioning.\nA list of the full names of the assets to receive updates. You must specify either or both of\nassetNames and assetTypes. Only asset updates matching specified assetNames and assetTypes are\nexported to the feed. For example: //compute.googleapis.com/projects/my_project_123/zones/zone1/instances/instance1.\nSee https://cloud.google.com/apis/design/resourceNames#fullResourceName for more info."]
    pub fn asset_names(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.asset_names", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `asset_types` after provisioning.\nA list of types of the assets to receive updates. You must specify either or both of assetNames\nand assetTypes. Only asset updates matching specified assetNames and assetTypes are exported to\nthe feed. For example: \"compute.googleapis.com/Disk\"\nSee https://cloud.google.com/asset-inventory/docs/supported-asset-types for a list of all\nsupported asset types."]
    pub fn asset_types(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.asset_types", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `billing_project` after provisioning.\nThe project whose identity will be used when sending messages to the\ndestination pubsub topic. It also specifies the project for API\nenablement check, quota, and billing."]
    pub fn billing_project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.billing_project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `content_type` after provisioning.\nAsset content type. If not specified, no content but the asset name and type will be returned. Possible values: [\"CONTENT_TYPE_UNSPECIFIED\", \"RESOURCE\", \"IAM_POLICY\", \"ORG_POLICY\", \"OS_INVENTORY\", \"ACCESS_POLICY\"]"]
    pub fn content_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.content_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `feed_id` after provisioning.\nThis is the client-assigned asset feed identifier and it needs to be unique under a specific parent."]
    pub fn feed_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.feed_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe format will be organizations/{organization_number}/feeds/{client-assigned_feed_identifier}."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `org_id` after provisioning.\nThe organization this feed should be created in."]
    pub fn org_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.org_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `condition` after provisioning.\n"]
    pub fn condition(&self) -> ListRef<CloudAssetOrganizationFeedConditionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.condition", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `feed_output_config` after provisioning.\n"]
    pub fn feed_output_config(&self) -> ListRef<CloudAssetOrganizationFeedFeedOutputConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.feed_output_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudAssetOrganizationFeedTimeoutsElRef {
        CloudAssetOrganizationFeedTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for CloudAssetOrganizationFeed {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for CloudAssetOrganizationFeed {}
impl ToListMappable for CloudAssetOrganizationFeed {
    type O = ListRef<CloudAssetOrganizationFeedRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for CloudAssetOrganizationFeed_ {
    fn extract_resource_type(&self) -> String {
        "google_cloud_asset_organization_feed".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildCloudAssetOrganizationFeed {
    pub tf_id: String,
    #[doc = "The project whose identity will be used when sending messages to the\ndestination pubsub topic. It also specifies the project for API\nenablement check, quota, and billing."]
    pub billing_project: PrimField<String>,
    #[doc = "This is the client-assigned asset feed identifier and it needs to be unique under a specific parent."]
    pub feed_id: PrimField<String>,
    #[doc = "The organization this feed should be created in."]
    pub org_id: PrimField<String>,
}
impl BuildCloudAssetOrganizationFeed {
    pub fn build(self, stack: &mut Stack) -> CloudAssetOrganizationFeed {
        let out = CloudAssetOrganizationFeed(Rc::new(CloudAssetOrganizationFeed_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(CloudAssetOrganizationFeedData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                asset_names: core::default::Default::default(),
                asset_types: core::default::Default::default(),
                billing_project: self.billing_project,
                content_type: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                feed_id: self.feed_id,
                id: core::default::Default::default(),
                org_id: self.org_id,
                condition: core::default::Default::default(),
                feed_output_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct CloudAssetOrganizationFeedRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudAssetOrganizationFeedRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl CloudAssetOrganizationFeedRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `asset_names` after provisioning.\nA list of the full names of the assets to receive updates. You must specify either or both of\nassetNames and assetTypes. Only asset updates matching specified assetNames and assetTypes are\nexported to the feed. For example: //compute.googleapis.com/projects/my_project_123/zones/zone1/instances/instance1.\nSee https://cloud.google.com/apis/design/resourceNames#fullResourceName for more info."]
    pub fn asset_names(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.asset_names", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `asset_types` after provisioning.\nA list of types of the assets to receive updates. You must specify either or both of assetNames\nand assetTypes. Only asset updates matching specified assetNames and assetTypes are exported to\nthe feed. For example: \"compute.googleapis.com/Disk\"\nSee https://cloud.google.com/asset-inventory/docs/supported-asset-types for a list of all\nsupported asset types."]
    pub fn asset_types(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.asset_types", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `billing_project` after provisioning.\nThe project whose identity will be used when sending messages to the\ndestination pubsub topic. It also specifies the project for API\nenablement check, quota, and billing."]
    pub fn billing_project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.billing_project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `content_type` after provisioning.\nAsset content type. If not specified, no content but the asset name and type will be returned. Possible values: [\"CONTENT_TYPE_UNSPECIFIED\", \"RESOURCE\", \"IAM_POLICY\", \"ORG_POLICY\", \"OS_INVENTORY\", \"ACCESS_POLICY\"]"]
    pub fn content_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.content_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `feed_id` after provisioning.\nThis is the client-assigned asset feed identifier and it needs to be unique under a specific parent."]
    pub fn feed_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.feed_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe format will be organizations/{organization_number}/feeds/{client-assigned_feed_identifier}."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `org_id` after provisioning.\nThe organization this feed should be created in."]
    pub fn org_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.org_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `condition` after provisioning.\n"]
    pub fn condition(&self) -> ListRef<CloudAssetOrganizationFeedConditionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.condition", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `feed_output_config` after provisioning.\n"]
    pub fn feed_output_config(&self) -> ListRef<CloudAssetOrganizationFeedFeedOutputConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.feed_output_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudAssetOrganizationFeedTimeoutsElRef {
        CloudAssetOrganizationFeedTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct CloudAssetOrganizationFeedConditionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    expression: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<PrimField<String>>,
}
impl CloudAssetOrganizationFeedConditionEl {
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
impl ToListMappable for CloudAssetOrganizationFeedConditionEl {
    type O = BlockAssignable<CloudAssetOrganizationFeedConditionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudAssetOrganizationFeedConditionEl {
    #[doc = "Textual representation of an expression in Common Expression Language syntax."]
    pub expression: PrimField<String>,
}
impl BuildCloudAssetOrganizationFeedConditionEl {
    pub fn build(self) -> CloudAssetOrganizationFeedConditionEl {
        CloudAssetOrganizationFeedConditionEl {
            description: core::default::Default::default(),
            expression: self.expression,
            location: core::default::Default::default(),
            title: core::default::Default::default(),
        }
    }
}
pub struct CloudAssetOrganizationFeedConditionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudAssetOrganizationFeedConditionElRef {
    fn new(shared: StackShared, base: String) -> CloudAssetOrganizationFeedConditionElRef {
        CloudAssetOrganizationFeedConditionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudAssetOrganizationFeedConditionElRef {
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
pub struct CloudAssetOrganizationFeedFeedOutputConfigElPubsubDestinationEl {
    topic: PrimField<String>,
}
impl CloudAssetOrganizationFeedFeedOutputConfigElPubsubDestinationEl {}
impl ToListMappable for CloudAssetOrganizationFeedFeedOutputConfigElPubsubDestinationEl {
    type O = BlockAssignable<CloudAssetOrganizationFeedFeedOutputConfigElPubsubDestinationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudAssetOrganizationFeedFeedOutputConfigElPubsubDestinationEl {
    #[doc = "Destination on Cloud Pubsub topic."]
    pub topic: PrimField<String>,
}
impl BuildCloudAssetOrganizationFeedFeedOutputConfigElPubsubDestinationEl {
    pub fn build(self) -> CloudAssetOrganizationFeedFeedOutputConfigElPubsubDestinationEl {
        CloudAssetOrganizationFeedFeedOutputConfigElPubsubDestinationEl { topic: self.topic }
    }
}
pub struct CloudAssetOrganizationFeedFeedOutputConfigElPubsubDestinationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudAssetOrganizationFeedFeedOutputConfigElPubsubDestinationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudAssetOrganizationFeedFeedOutputConfigElPubsubDestinationElRef {
        CloudAssetOrganizationFeedFeedOutputConfigElPubsubDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudAssetOrganizationFeedFeedOutputConfigElPubsubDestinationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `topic` after provisioning.\nDestination on Cloud Pubsub topic."]
    pub fn topic(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.topic", self.base))
    }
}
#[derive(Serialize, Default)]
struct CloudAssetOrganizationFeedFeedOutputConfigElDynamic {
    pubsub_destination:
        Option<DynamicBlock<CloudAssetOrganizationFeedFeedOutputConfigElPubsubDestinationEl>>,
}
#[derive(Serialize)]
pub struct CloudAssetOrganizationFeedFeedOutputConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    pubsub_destination:
        Option<Vec<CloudAssetOrganizationFeedFeedOutputConfigElPubsubDestinationEl>>,
    dynamic: CloudAssetOrganizationFeedFeedOutputConfigElDynamic,
}
impl CloudAssetOrganizationFeedFeedOutputConfigEl {
    #[doc = "Set the field `pubsub_destination`.\n"]
    pub fn set_pubsub_destination(
        mut self,
        v: impl Into<BlockAssignable<CloudAssetOrganizationFeedFeedOutputConfigElPubsubDestinationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.pubsub_destination = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.pubsub_destination = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CloudAssetOrganizationFeedFeedOutputConfigEl {
    type O = BlockAssignable<CloudAssetOrganizationFeedFeedOutputConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudAssetOrganizationFeedFeedOutputConfigEl {}
impl BuildCloudAssetOrganizationFeedFeedOutputConfigEl {
    pub fn build(self) -> CloudAssetOrganizationFeedFeedOutputConfigEl {
        CloudAssetOrganizationFeedFeedOutputConfigEl {
            pubsub_destination: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CloudAssetOrganizationFeedFeedOutputConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudAssetOrganizationFeedFeedOutputConfigElRef {
    fn new(shared: StackShared, base: String) -> CloudAssetOrganizationFeedFeedOutputConfigElRef {
        CloudAssetOrganizationFeedFeedOutputConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudAssetOrganizationFeedFeedOutputConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `pubsub_destination` after provisioning.\n"]
    pub fn pubsub_destination(
        &self,
    ) -> ListRef<CloudAssetOrganizationFeedFeedOutputConfigElPubsubDestinationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.pubsub_destination", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CloudAssetOrganizationFeedTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl CloudAssetOrganizationFeedTimeoutsEl {
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
impl ToListMappable for CloudAssetOrganizationFeedTimeoutsEl {
    type O = BlockAssignable<CloudAssetOrganizationFeedTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudAssetOrganizationFeedTimeoutsEl {}
impl BuildCloudAssetOrganizationFeedTimeoutsEl {
    pub fn build(self) -> CloudAssetOrganizationFeedTimeoutsEl {
        CloudAssetOrganizationFeedTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct CloudAssetOrganizationFeedTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudAssetOrganizationFeedTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> CloudAssetOrganizationFeedTimeoutsElRef {
        CloudAssetOrganizationFeedTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudAssetOrganizationFeedTimeoutsElRef {
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
#[derive(Serialize, Default)]
struct CloudAssetOrganizationFeedDynamic {
    condition: Option<DynamicBlock<CloudAssetOrganizationFeedConditionEl>>,
    feed_output_config: Option<DynamicBlock<CloudAssetOrganizationFeedFeedOutputConfigEl>>,
}
