use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataResourcegroupstaggingapiResourcesData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_compliant_resources: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_compliance_details: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_arn_list: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_type_filters: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag_filter: Option<Vec<DataResourcegroupstaggingapiResourcesTagFilterEl>>,
    dynamic: DataResourcegroupstaggingapiResourcesDynamic,
}

struct DataResourcegroupstaggingapiResources_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataResourcegroupstaggingapiResourcesData>,
}

#[derive(Clone)]
pub struct DataResourcegroupstaggingapiResources(Rc<DataResourcegroupstaggingapiResources_>);

impl DataResourcegroupstaggingapiResources {
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

    #[doc = "Set the field `exclude_compliant_resources`.\n"]
    pub fn set_exclude_compliant_resources(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().exclude_compliant_resources = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `include_compliance_details`.\n"]
    pub fn set_include_compliance_details(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().include_compliance_details = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `resource_arn_list`.\n"]
    pub fn set_resource_arn_list(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().resource_arn_list = Some(v.into());
        self
    }

    #[doc = "Set the field `resource_type_filters`.\n"]
    pub fn set_resource_type_filters(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().resource_type_filters = Some(v.into());
        self
    }

    #[doc = "Set the field `tag_filter`.\n"]
    pub fn set_tag_filter(
        self,
        v: impl Into<BlockAssignable<DataResourcegroupstaggingapiResourcesTagFilterEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().tag_filter = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.tag_filter = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `exclude_compliant_resources` after provisioning.\n"]
    pub fn exclude_compliant_resources(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.exclude_compliant_resources", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `include_compliance_details` after provisioning.\n"]
    pub fn include_compliance_details(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.include_compliance_details", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_arn_list` after provisioning.\n"]
    pub fn resource_arn_list(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.resource_arn_list", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_tag_mapping_list` after provisioning.\n"]
    pub fn resource_tag_mapping_list(&self) -> ListRef<DataResourcegroupstaggingapiResourcesResourceTagMappingListElRef> {
        ListRef::new(self.shared().clone(), format!("{}.resource_tag_mapping_list", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_type_filters` after provisioning.\n"]
    pub fn resource_type_filters(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.resource_type_filters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tag_filter` after provisioning.\n"]
    pub fn tag_filter(&self) -> ListRef<DataResourcegroupstaggingapiResourcesTagFilterElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tag_filter", self.extract_ref()))
    }
}

impl Referable for DataResourcegroupstaggingapiResources {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataResourcegroupstaggingapiResources { }

impl ToListMappable for DataResourcegroupstaggingapiResources {
    type O = ListRef<DataResourcegroupstaggingapiResourcesRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataResourcegroupstaggingapiResources_ {
    fn extract_datasource_type(&self) -> String {
        "aws_resourcegroupstaggingapi_resources".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataResourcegroupstaggingapiResources {
    pub tf_id: String,
}

impl BuildDataResourcegroupstaggingapiResources {
    pub fn build(self, stack: &mut Stack) -> DataResourcegroupstaggingapiResources {
        let out = DataResourcegroupstaggingapiResources(Rc::new(DataResourcegroupstaggingapiResources_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataResourcegroupstaggingapiResourcesData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                exclude_compliant_resources: core::default::Default::default(),
                id: core::default::Default::default(),
                include_compliance_details: core::default::Default::default(),
                region: core::default::Default::default(),
                resource_arn_list: core::default::Default::default(),
                resource_type_filters: core::default::Default::default(),
                tag_filter: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataResourcegroupstaggingapiResourcesRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataResourcegroupstaggingapiResourcesRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataResourcegroupstaggingapiResourcesRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `exclude_compliant_resources` after provisioning.\n"]
    pub fn exclude_compliant_resources(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.exclude_compliant_resources", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `include_compliance_details` after provisioning.\n"]
    pub fn include_compliance_details(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.include_compliance_details", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_arn_list` after provisioning.\n"]
    pub fn resource_arn_list(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.resource_arn_list", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_tag_mapping_list` after provisioning.\n"]
    pub fn resource_tag_mapping_list(&self) -> ListRef<DataResourcegroupstaggingapiResourcesResourceTagMappingListElRef> {
        ListRef::new(self.shared().clone(), format!("{}.resource_tag_mapping_list", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_type_filters` after provisioning.\n"]
    pub fn resource_type_filters(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.resource_type_filters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tag_filter` after provisioning.\n"]
    pub fn tag_filter(&self) -> ListRef<DataResourcegroupstaggingapiResourcesTagFilterElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tag_filter", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataResourcegroupstaggingapiResourcesResourceTagMappingListElComplianceDetailsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    compliance_status: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    keys_with_noncompliant_values: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    non_compliant_keys: Option<SetField<PrimField<String>>>,
}

impl DataResourcegroupstaggingapiResourcesResourceTagMappingListElComplianceDetailsEl {
    #[doc = "Set the field `compliance_status`.\n"]
    pub fn set_compliance_status(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.compliance_status = Some(v.into());
        self
    }

    #[doc = "Set the field `keys_with_noncompliant_values`.\n"]
    pub fn set_keys_with_noncompliant_values(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.keys_with_noncompliant_values = Some(v.into());
        self
    }

    #[doc = "Set the field `non_compliant_keys`.\n"]
    pub fn set_non_compliant_keys(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.non_compliant_keys = Some(v.into());
        self
    }
}

impl ToListMappable for DataResourcegroupstaggingapiResourcesResourceTagMappingListElComplianceDetailsEl {
    type O = BlockAssignable<DataResourcegroupstaggingapiResourcesResourceTagMappingListElComplianceDetailsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataResourcegroupstaggingapiResourcesResourceTagMappingListElComplianceDetailsEl {}

impl BuildDataResourcegroupstaggingapiResourcesResourceTagMappingListElComplianceDetailsEl {
    pub fn build(self) -> DataResourcegroupstaggingapiResourcesResourceTagMappingListElComplianceDetailsEl {
        DataResourcegroupstaggingapiResourcesResourceTagMappingListElComplianceDetailsEl {
            compliance_status: core::default::Default::default(),
            keys_with_noncompliant_values: core::default::Default::default(),
            non_compliant_keys: core::default::Default::default(),
        }
    }
}

pub struct DataResourcegroupstaggingapiResourcesResourceTagMappingListElComplianceDetailsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataResourcegroupstaggingapiResourcesResourceTagMappingListElComplianceDetailsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataResourcegroupstaggingapiResourcesResourceTagMappingListElComplianceDetailsElRef {
        DataResourcegroupstaggingapiResourcesResourceTagMappingListElComplianceDetailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataResourcegroupstaggingapiResourcesResourceTagMappingListElComplianceDetailsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `compliance_status` after provisioning.\n"]
    pub fn compliance_status(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.compliance_status", self.base))
    }

    #[doc = "Get a reference to the value of field `keys_with_noncompliant_values` after provisioning.\n"]
    pub fn keys_with_noncompliant_values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.keys_with_noncompliant_values", self.base))
    }

    #[doc = "Get a reference to the value of field `non_compliant_keys` after provisioning.\n"]
    pub fn non_compliant_keys(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.non_compliant_keys", self.base))
    }
}

#[derive(Serialize)]
pub struct DataResourcegroupstaggingapiResourcesResourceTagMappingListEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    compliance_details: Option<
        ListField<DataResourcegroupstaggingapiResourcesResourceTagMappingListElComplianceDetailsEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
}

impl DataResourcegroupstaggingapiResourcesResourceTagMappingListEl {
    #[doc = "Set the field `compliance_details`.\n"]
    pub fn set_compliance_details(
        mut self,
        v: impl Into<ListField<DataResourcegroupstaggingapiResourcesResourceTagMappingListElComplianceDetailsEl>>,
    ) -> Self {
        self.compliance_details = Some(v.into());
        self
    }

    #[doc = "Set the field `resource_arn`.\n"]
    pub fn set_resource_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.resource_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.tags = Some(v.into());
        self
    }
}

impl ToListMappable for DataResourcegroupstaggingapiResourcesResourceTagMappingListEl {
    type O = BlockAssignable<DataResourcegroupstaggingapiResourcesResourceTagMappingListEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataResourcegroupstaggingapiResourcesResourceTagMappingListEl {}

impl BuildDataResourcegroupstaggingapiResourcesResourceTagMappingListEl {
    pub fn build(self) -> DataResourcegroupstaggingapiResourcesResourceTagMappingListEl {
        DataResourcegroupstaggingapiResourcesResourceTagMappingListEl {
            compliance_details: core::default::Default::default(),
            resource_arn: core::default::Default::default(),
            tags: core::default::Default::default(),
        }
    }
}

pub struct DataResourcegroupstaggingapiResourcesResourceTagMappingListElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataResourcegroupstaggingapiResourcesResourceTagMappingListElRef {
    fn new(shared: StackShared, base: String) -> DataResourcegroupstaggingapiResourcesResourceTagMappingListElRef {
        DataResourcegroupstaggingapiResourcesResourceTagMappingListElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataResourcegroupstaggingapiResourcesResourceTagMappingListElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `compliance_details` after provisioning.\n"]
    pub fn compliance_details(
        &self,
    ) -> ListRef<DataResourcegroupstaggingapiResourcesResourceTagMappingListElComplianceDetailsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.compliance_details", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_arn` after provisioning.\n"]
    pub fn resource_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize)]
pub struct DataResourcegroupstaggingapiResourcesTagFilterEl {
    key: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl DataResourcegroupstaggingapiResourcesTagFilterEl {
    #[doc = "Set the field `values`.\n"]
    pub fn set_values(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}

impl ToListMappable for DataResourcegroupstaggingapiResourcesTagFilterEl {
    type O = BlockAssignable<DataResourcegroupstaggingapiResourcesTagFilterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataResourcegroupstaggingapiResourcesTagFilterEl {
    #[doc = ""]
    pub key: PrimField<String>,
}

impl BuildDataResourcegroupstaggingapiResourcesTagFilterEl {
    pub fn build(self) -> DataResourcegroupstaggingapiResourcesTagFilterEl {
        DataResourcegroupstaggingapiResourcesTagFilterEl {
            key: self.key,
            values: core::default::Default::default(),
        }
    }
}

pub struct DataResourcegroupstaggingapiResourcesTagFilterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataResourcegroupstaggingapiResourcesTagFilterElRef {
    fn new(shared: StackShared, base: String) -> DataResourcegroupstaggingapiResourcesTagFilterElRef {
        DataResourcegroupstaggingapiResourcesTagFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataResourcegroupstaggingapiResourcesTagFilterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataResourcegroupstaggingapiResourcesDynamic {
    tag_filter: Option<DynamicBlock<DataResourcegroupstaggingapiResourcesTagFilterEl>>,
}
