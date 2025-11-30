use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct S3controlObjectLambdaAccessPointData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    account_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    configuration: Option<Vec<S3controlObjectLambdaAccessPointConfigurationEl>>,
    dynamic: S3controlObjectLambdaAccessPointDynamic,
}

struct S3controlObjectLambdaAccessPoint_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<S3controlObjectLambdaAccessPointData>,
}

#[derive(Clone)]
pub struct S3controlObjectLambdaAccessPoint(Rc<S3controlObjectLambdaAccessPoint_>);

impl S3controlObjectLambdaAccessPoint {
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

    #[doc = "Set the field `account_id`.\n"]
    pub fn set_account_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().account_id = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `configuration`.\n"]
    pub fn set_configuration(
        self,
        v: impl Into<BlockAssignable<S3controlObjectLambdaAccessPointConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.configuration = Some(d);
            }
        }
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.account_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `alias` after provisioning.\n"]
    pub fn alias(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.alias", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> ListRef<S3controlObjectLambdaAccessPointConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.configuration", self.extract_ref()),
        )
    }
}

impl Referable for S3controlObjectLambdaAccessPoint {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for S3controlObjectLambdaAccessPoint {}

impl ToListMappable for S3controlObjectLambdaAccessPoint {
    type O = ListRef<S3controlObjectLambdaAccessPointRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for S3controlObjectLambdaAccessPoint_ {
    fn extract_resource_type(&self) -> String {
        "aws_s3control_object_lambda_access_point".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildS3controlObjectLambdaAccessPoint {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildS3controlObjectLambdaAccessPoint {
    pub fn build(self, stack: &mut Stack) -> S3controlObjectLambdaAccessPoint {
        let out = S3controlObjectLambdaAccessPoint(Rc::new(S3controlObjectLambdaAccessPoint_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(S3controlObjectLambdaAccessPointData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                account_id: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                configuration: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct S3controlObjectLambdaAccessPointRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3controlObjectLambdaAccessPointRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl S3controlObjectLambdaAccessPointRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.account_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `alias` after provisioning.\n"]
    pub fn alias(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.alias", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> ListRef<S3controlObjectLambdaAccessPointConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.configuration", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct S3controlObjectLambdaAccessPointConfigurationElTransformationConfigurationElContentTransformationElAwsLambdaEl
{
    function_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    function_payload: Option<PrimField<String>>,
}

impl S3controlObjectLambdaAccessPointConfigurationElTransformationConfigurationElContentTransformationElAwsLambdaEl {
    #[doc = "Set the field `function_payload`.\n"]
    pub fn set_function_payload(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.function_payload = Some(v.into());
        self
    }
}

impl ToListMappable for S3controlObjectLambdaAccessPointConfigurationElTransformationConfigurationElContentTransformationElAwsLambdaEl {
    type O =
        BlockAssignable<
            S3controlObjectLambdaAccessPointConfigurationElTransformationConfigurationElContentTransformationElAwsLambdaEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3controlObjectLambdaAccessPointConfigurationElTransformationConfigurationElContentTransformationElAwsLambdaEl
{
    #[doc = ""]
    pub function_arn: PrimField<String>,
}

impl BuildS3controlObjectLambdaAccessPointConfigurationElTransformationConfigurationElContentTransformationElAwsLambdaEl {
    pub fn build(
        self,
    ) -> S3controlObjectLambdaAccessPointConfigurationElTransformationConfigurationElContentTransformationElAwsLambdaEl {
        S3controlObjectLambdaAccessPointConfigurationElTransformationConfigurationElContentTransformationElAwsLambdaEl {
            function_arn: self.function_arn,
            function_payload: core::default::Default::default(),
        }
    }
}

pub struct S3controlObjectLambdaAccessPointConfigurationElTransformationConfigurationElContentTransformationElAwsLambdaElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for S3controlObjectLambdaAccessPointConfigurationElTransformationConfigurationElContentTransformationElAwsLambdaElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> S3controlObjectLambdaAccessPointConfigurationElTransformationConfigurationElContentTransformationElAwsLambdaElRef {
        S3controlObjectLambdaAccessPointConfigurationElTransformationConfigurationElContentTransformationElAwsLambdaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3controlObjectLambdaAccessPointConfigurationElTransformationConfigurationElContentTransformationElAwsLambdaElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `function_arn` after provisioning.\n"]
    pub fn function_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.function_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `function_payload` after provisioning.\n"]
    pub fn function_payload(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.function_payload", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3controlObjectLambdaAccessPointConfigurationElTransformationConfigurationElContentTransformationElDynamic {
    aws_lambda: Option<
        DynamicBlock<
            S3controlObjectLambdaAccessPointConfigurationElTransformationConfigurationElContentTransformationElAwsLambdaEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct S3controlObjectLambdaAccessPointConfigurationElTransformationConfigurationElContentTransformationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    aws_lambda: Option<
        Vec<
            S3controlObjectLambdaAccessPointConfigurationElTransformationConfigurationElContentTransformationElAwsLambdaEl,
        >,
    >,
    dynamic: S3controlObjectLambdaAccessPointConfigurationElTransformationConfigurationElContentTransformationElDynamic,
}

impl S3controlObjectLambdaAccessPointConfigurationElTransformationConfigurationElContentTransformationEl {
    #[doc = "Set the field `aws_lambda`.\n"]
    pub fn set_aws_lambda(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            S3controlObjectLambdaAccessPointConfigurationElTransformationConfigurationElContentTransformationElAwsLambdaEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.aws_lambda = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.aws_lambda = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for S3controlObjectLambdaAccessPointConfigurationElTransformationConfigurationElContentTransformationEl {
    type O =
        BlockAssignable<
            S3controlObjectLambdaAccessPointConfigurationElTransformationConfigurationElContentTransformationEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3controlObjectLambdaAccessPointConfigurationElTransformationConfigurationElContentTransformationEl
{}

impl BuildS3controlObjectLambdaAccessPointConfigurationElTransformationConfigurationElContentTransformationEl {
    pub fn build(
        self,
    ) -> S3controlObjectLambdaAccessPointConfigurationElTransformationConfigurationElContentTransformationEl {
        S3controlObjectLambdaAccessPointConfigurationElTransformationConfigurationElContentTransformationEl {
            aws_lambda: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct S3controlObjectLambdaAccessPointConfigurationElTransformationConfigurationElContentTransformationElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for S3controlObjectLambdaAccessPointConfigurationElTransformationConfigurationElContentTransformationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> S3controlObjectLambdaAccessPointConfigurationElTransformationConfigurationElContentTransformationElRef {
        S3controlObjectLambdaAccessPointConfigurationElTransformationConfigurationElContentTransformationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3controlObjectLambdaAccessPointConfigurationElTransformationConfigurationElContentTransformationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `aws_lambda` after provisioning.\n"]
    pub fn aws_lambda(
        &self,
    ) -> ListRef<
        S3controlObjectLambdaAccessPointConfigurationElTransformationConfigurationElContentTransformationElAwsLambdaElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.aws_lambda", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3controlObjectLambdaAccessPointConfigurationElTransformationConfigurationElDynamic {
    content_transformation: Option<
        DynamicBlock<
            S3controlObjectLambdaAccessPointConfigurationElTransformationConfigurationElContentTransformationEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct S3controlObjectLambdaAccessPointConfigurationElTransformationConfigurationEl {
    actions: SetField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content_transformation: Option<
        Vec<S3controlObjectLambdaAccessPointConfigurationElTransformationConfigurationElContentTransformationEl>,
    >,
    dynamic: S3controlObjectLambdaAccessPointConfigurationElTransformationConfigurationElDynamic,
}

impl S3controlObjectLambdaAccessPointConfigurationElTransformationConfigurationEl {
    #[doc = "Set the field `content_transformation`.\n"]
    pub fn set_content_transformation(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            S3controlObjectLambdaAccessPointConfigurationElTransformationConfigurationElContentTransformationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.content_transformation = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.content_transformation = Some(d);
            }
        }
        self
    }
}

impl ToListMappable
    for S3controlObjectLambdaAccessPointConfigurationElTransformationConfigurationEl
{
    type O = BlockAssignable<
        S3controlObjectLambdaAccessPointConfigurationElTransformationConfigurationEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3controlObjectLambdaAccessPointConfigurationElTransformationConfigurationEl {
    #[doc = ""]
    pub actions: SetField<PrimField<String>>,
}

impl BuildS3controlObjectLambdaAccessPointConfigurationElTransformationConfigurationEl {
    pub fn build(
        self,
    ) -> S3controlObjectLambdaAccessPointConfigurationElTransformationConfigurationEl {
        S3controlObjectLambdaAccessPointConfigurationElTransformationConfigurationEl {
            actions: self.actions,
            content_transformation: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct S3controlObjectLambdaAccessPointConfigurationElTransformationConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3controlObjectLambdaAccessPointConfigurationElTransformationConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> S3controlObjectLambdaAccessPointConfigurationElTransformationConfigurationElRef {
        S3controlObjectLambdaAccessPointConfigurationElTransformationConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3controlObjectLambdaAccessPointConfigurationElTransformationConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `actions` after provisioning.\n"]
    pub fn actions(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.actions", self.base))
    }

    #[doc = "Get a reference to the value of field `content_transformation` after provisioning.\n"]
    pub fn content_transformation(
        &self,
    ) -> ListRef<
        S3controlObjectLambdaAccessPointConfigurationElTransformationConfigurationElContentTransformationElRef,
    >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.content_transformation", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct S3controlObjectLambdaAccessPointConfigurationElDynamic {
    transformation_configuration: Option<
        DynamicBlock<S3controlObjectLambdaAccessPointConfigurationElTransformationConfigurationEl>,
    >,
}

#[derive(Serialize)]
pub struct S3controlObjectLambdaAccessPointConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_features: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloud_watch_metrics_enabled: Option<PrimField<bool>>,
    supporting_access_point: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transformation_configuration:
        Option<Vec<S3controlObjectLambdaAccessPointConfigurationElTransformationConfigurationEl>>,
    dynamic: S3controlObjectLambdaAccessPointConfigurationElDynamic,
}

impl S3controlObjectLambdaAccessPointConfigurationEl {
    #[doc = "Set the field `allowed_features`.\n"]
    pub fn set_allowed_features(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.allowed_features = Some(v.into());
        self
    }

    #[doc = "Set the field `cloud_watch_metrics_enabled`.\n"]
    pub fn set_cloud_watch_metrics_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.cloud_watch_metrics_enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `transformation_configuration`.\n"]
    pub fn set_transformation_configuration(
        mut self,
        v: impl Into<
            BlockAssignable<
                S3controlObjectLambdaAccessPointConfigurationElTransformationConfigurationEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.transformation_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.transformation_configuration = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for S3controlObjectLambdaAccessPointConfigurationEl {
    type O = BlockAssignable<S3controlObjectLambdaAccessPointConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3controlObjectLambdaAccessPointConfigurationEl {
    #[doc = ""]
    pub supporting_access_point: PrimField<String>,
}

impl BuildS3controlObjectLambdaAccessPointConfigurationEl {
    pub fn build(self) -> S3controlObjectLambdaAccessPointConfigurationEl {
        S3controlObjectLambdaAccessPointConfigurationEl {
            allowed_features: core::default::Default::default(),
            cloud_watch_metrics_enabled: core::default::Default::default(),
            supporting_access_point: self.supporting_access_point,
            transformation_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct S3controlObjectLambdaAccessPointConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3controlObjectLambdaAccessPointConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> S3controlObjectLambdaAccessPointConfigurationElRef {
        S3controlObjectLambdaAccessPointConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3controlObjectLambdaAccessPointConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allowed_features` after provisioning.\n"]
    pub fn allowed_features(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.allowed_features", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `cloud_watch_metrics_enabled` after provisioning.\n"]
    pub fn cloud_watch_metrics_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cloud_watch_metrics_enabled", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `supporting_access_point` after provisioning.\n"]
    pub fn supporting_access_point(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.supporting_access_point", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct S3controlObjectLambdaAccessPointDynamic {
    configuration: Option<DynamicBlock<S3controlObjectLambdaAccessPointConfigurationEl>>,
}
