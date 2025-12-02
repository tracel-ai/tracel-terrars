use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct CloudsearchDomainData {
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
    multi_az: Option<PrimField<bool>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    endpoint_options: Option<Vec<CloudsearchDomainEndpointOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    index_field: Option<Vec<CloudsearchDomainIndexFieldEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scaling_parameters: Option<Vec<CloudsearchDomainScalingParametersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<CloudsearchDomainTimeoutsEl>,
    dynamic: CloudsearchDomainDynamic,
}
struct CloudsearchDomain_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CloudsearchDomainData>,
}
#[derive(Clone)]
pub struct CloudsearchDomain(Rc<CloudsearchDomain_>);
impl CloudsearchDomain {
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
    #[doc = "Set the field `multi_az`.\n"]
    pub fn set_multi_az(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().multi_az = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `endpoint_options`.\n"]
    pub fn set_endpoint_options(
        self,
        v: impl Into<BlockAssignable<CloudsearchDomainEndpointOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().endpoint_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.endpoint_options = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `index_field`.\n"]
    pub fn set_index_field(
        self,
        v: impl Into<BlockAssignable<CloudsearchDomainIndexFieldEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().index_field = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.index_field = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `scaling_parameters`.\n"]
    pub fn set_scaling_parameters(
        self,
        v: impl Into<BlockAssignable<CloudsearchDomainScalingParametersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().scaling_parameters = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.scaling_parameters = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<CloudsearchDomainTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `document_service_endpoint` after provisioning.\n"]
    pub fn document_service_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.document_service_endpoint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `domain_id` after provisioning.\n"]
    pub fn domain_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `multi_az` after provisioning.\n"]
    pub fn multi_az(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.multi_az", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `search_service_endpoint` after provisioning.\n"]
    pub fn search_service_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.search_service_endpoint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `endpoint_options` after provisioning.\n"]
    pub fn endpoint_options(&self) -> ListRef<CloudsearchDomainEndpointOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.endpoint_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `scaling_parameters` after provisioning.\n"]
    pub fn scaling_parameters(&self) -> ListRef<CloudsearchDomainScalingParametersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.scaling_parameters", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudsearchDomainTimeoutsElRef {
        CloudsearchDomainTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for CloudsearchDomain {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for CloudsearchDomain {}
impl ToListMappable for CloudsearchDomain {
    type O = ListRef<CloudsearchDomainRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for CloudsearchDomain_ {
    fn extract_resource_type(&self) -> String {
        "aws_cloudsearch_domain".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildCloudsearchDomain {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
}
impl BuildCloudsearchDomain {
    pub fn build(self, stack: &mut Stack) -> CloudsearchDomain {
        let out = CloudsearchDomain(Rc::new(CloudsearchDomain_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(CloudsearchDomainData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                multi_az: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                endpoint_options: core::default::Default::default(),
                index_field: core::default::Default::default(),
                scaling_parameters: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct CloudsearchDomainRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudsearchDomainRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl CloudsearchDomainRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `document_service_endpoint` after provisioning.\n"]
    pub fn document_service_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.document_service_endpoint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `domain_id` after provisioning.\n"]
    pub fn domain_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `multi_az` after provisioning.\n"]
    pub fn multi_az(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.multi_az", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `search_service_endpoint` after provisioning.\n"]
    pub fn search_service_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.search_service_endpoint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `endpoint_options` after provisioning.\n"]
    pub fn endpoint_options(&self) -> ListRef<CloudsearchDomainEndpointOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.endpoint_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `scaling_parameters` after provisioning.\n"]
    pub fn scaling_parameters(&self) -> ListRef<CloudsearchDomainScalingParametersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.scaling_parameters", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudsearchDomainTimeoutsElRef {
        CloudsearchDomainTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct CloudsearchDomainEndpointOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enforce_https: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tls_security_policy: Option<PrimField<String>>,
}
impl CloudsearchDomainEndpointOptionsEl {
    #[doc = "Set the field `enforce_https`.\n"]
    pub fn set_enforce_https(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enforce_https = Some(v.into());
        self
    }
    #[doc = "Set the field `tls_security_policy`.\n"]
    pub fn set_tls_security_policy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tls_security_policy = Some(v.into());
        self
    }
}
impl ToListMappable for CloudsearchDomainEndpointOptionsEl {
    type O = BlockAssignable<CloudsearchDomainEndpointOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudsearchDomainEndpointOptionsEl {}
impl BuildCloudsearchDomainEndpointOptionsEl {
    pub fn build(self) -> CloudsearchDomainEndpointOptionsEl {
        CloudsearchDomainEndpointOptionsEl {
            enforce_https: core::default::Default::default(),
            tls_security_policy: core::default::Default::default(),
        }
    }
}
pub struct CloudsearchDomainEndpointOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudsearchDomainEndpointOptionsElRef {
    fn new(shared: StackShared, base: String) -> CloudsearchDomainEndpointOptionsElRef {
        CloudsearchDomainEndpointOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudsearchDomainEndpointOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enforce_https` after provisioning.\n"]
    pub fn enforce_https(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enforce_https", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `tls_security_policy` after provisioning.\n"]
    pub fn tls_security_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.tls_security_policy", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CloudsearchDomainIndexFieldEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    analysis_scheme: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_value: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    facet: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    highlight: Option<PrimField<bool>>,
    name: PrimField<String>,
    #[serde(rename = "return", skip_serializing_if = "Option::is_none")]
    return_: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    search: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_fields: Option<PrimField<String>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}
impl CloudsearchDomainIndexFieldEl {
    #[doc = "Set the field `analysis_scheme`.\n"]
    pub fn set_analysis_scheme(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.analysis_scheme = Some(v.into());
        self
    }
    #[doc = "Set the field `default_value`.\n"]
    pub fn set_default_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.default_value = Some(v.into());
        self
    }
    #[doc = "Set the field `facet`.\n"]
    pub fn set_facet(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.facet = Some(v.into());
        self
    }
    #[doc = "Set the field `highlight`.\n"]
    pub fn set_highlight(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.highlight = Some(v.into());
        self
    }
    #[doc = "Set the field `return_`.\n"]
    pub fn set_return(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.return_ = Some(v.into());
        self
    }
    #[doc = "Set the field `search`.\n"]
    pub fn set_search(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.search = Some(v.into());
        self
    }
    #[doc = "Set the field `sort`.\n"]
    pub fn set_sort(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.sort = Some(v.into());
        self
    }
    #[doc = "Set the field `source_fields`.\n"]
    pub fn set_source_fields(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_fields = Some(v.into());
        self
    }
}
impl ToListMappable for CloudsearchDomainIndexFieldEl {
    type O = BlockAssignable<CloudsearchDomainIndexFieldEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudsearchDomainIndexFieldEl {
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub type_: PrimField<String>,
}
impl BuildCloudsearchDomainIndexFieldEl {
    pub fn build(self) -> CloudsearchDomainIndexFieldEl {
        CloudsearchDomainIndexFieldEl {
            analysis_scheme: core::default::Default::default(),
            default_value: core::default::Default::default(),
            facet: core::default::Default::default(),
            highlight: core::default::Default::default(),
            name: self.name,
            return_: core::default::Default::default(),
            search: core::default::Default::default(),
            sort: core::default::Default::default(),
            source_fields: core::default::Default::default(),
            type_: self.type_,
        }
    }
}
pub struct CloudsearchDomainIndexFieldElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudsearchDomainIndexFieldElRef {
    fn new(shared: StackShared, base: String) -> CloudsearchDomainIndexFieldElRef {
        CloudsearchDomainIndexFieldElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudsearchDomainIndexFieldElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `analysis_scheme` after provisioning.\n"]
    pub fn analysis_scheme(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.analysis_scheme", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `default_value` after provisioning.\n"]
    pub fn default_value(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_value", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `facet` after provisioning.\n"]
    pub fn facet(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.facet", self.base))
    }
    #[doc = "Get a reference to the value of field `highlight` after provisioning.\n"]
    pub fn highlight(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.highlight", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `return_` after provisioning.\n"]
    pub fn return_(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.return", self.base))
    }
    #[doc = "Get a reference to the value of field `search` after provisioning.\n"]
    pub fn search(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.search", self.base))
    }
    #[doc = "Get a reference to the value of field `sort` after provisioning.\n"]
    pub fn sort(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.sort", self.base))
    }
    #[doc = "Get a reference to the value of field `source_fields` after provisioning.\n"]
    pub fn source_fields(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_fields", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudsearchDomainScalingParametersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    desired_instance_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    desired_partition_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    desired_replication_count: Option<PrimField<f64>>,
}
impl CloudsearchDomainScalingParametersEl {
    #[doc = "Set the field `desired_instance_type`.\n"]
    pub fn set_desired_instance_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.desired_instance_type = Some(v.into());
        self
    }
    #[doc = "Set the field `desired_partition_count`.\n"]
    pub fn set_desired_partition_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.desired_partition_count = Some(v.into());
        self
    }
    #[doc = "Set the field `desired_replication_count`.\n"]
    pub fn set_desired_replication_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.desired_replication_count = Some(v.into());
        self
    }
}
impl ToListMappable for CloudsearchDomainScalingParametersEl {
    type O = BlockAssignable<CloudsearchDomainScalingParametersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudsearchDomainScalingParametersEl {}
impl BuildCloudsearchDomainScalingParametersEl {
    pub fn build(self) -> CloudsearchDomainScalingParametersEl {
        CloudsearchDomainScalingParametersEl {
            desired_instance_type: core::default::Default::default(),
            desired_partition_count: core::default::Default::default(),
            desired_replication_count: core::default::Default::default(),
        }
    }
}
pub struct CloudsearchDomainScalingParametersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudsearchDomainScalingParametersElRef {
    fn new(shared: StackShared, base: String) -> CloudsearchDomainScalingParametersElRef {
        CloudsearchDomainScalingParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudsearchDomainScalingParametersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `desired_instance_type` after provisioning.\n"]
    pub fn desired_instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.desired_instance_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `desired_partition_count` after provisioning.\n"]
    pub fn desired_partition_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.desired_partition_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `desired_replication_count` after provisioning.\n"]
    pub fn desired_replication_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.desired_replication_count", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CloudsearchDomainTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl CloudsearchDomainTimeoutsEl {
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
impl ToListMappable for CloudsearchDomainTimeoutsEl {
    type O = BlockAssignable<CloudsearchDomainTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudsearchDomainTimeoutsEl {}
impl BuildCloudsearchDomainTimeoutsEl {
    pub fn build(self) -> CloudsearchDomainTimeoutsEl {
        CloudsearchDomainTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct CloudsearchDomainTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudsearchDomainTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> CloudsearchDomainTimeoutsElRef {
        CloudsearchDomainTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudsearchDomainTimeoutsElRef {
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
struct CloudsearchDomainDynamic {
    endpoint_options: Option<DynamicBlock<CloudsearchDomainEndpointOptionsEl>>,
    index_field: Option<DynamicBlock<CloudsearchDomainIndexFieldEl>>,
    scaling_parameters: Option<DynamicBlock<CloudsearchDomainScalingParametersEl>>,
}
