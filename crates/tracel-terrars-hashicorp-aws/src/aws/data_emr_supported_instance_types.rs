use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataEmrSupportedInstanceTypesData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    release_label: PrimField<String>,
}

struct DataEmrSupportedInstanceTypes_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataEmrSupportedInstanceTypesData>,
}

#[derive(Clone)]
pub struct DataEmrSupportedInstanceTypes(Rc<DataEmrSupportedInstanceTypes_>);

impl DataEmrSupportedInstanceTypes {
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `release_label` after provisioning.\n"]
    pub fn release_label(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.release_label", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `supported_instance_types` after provisioning.\n"]
    pub fn supported_instance_types(&self) -> ListRef<DataEmrSupportedInstanceTypesSupportedInstanceTypesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.supported_instance_types", self.extract_ref()))
    }
}

impl Referable for DataEmrSupportedInstanceTypes {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataEmrSupportedInstanceTypes { }

impl ToListMappable for DataEmrSupportedInstanceTypes {
    type O = ListRef<DataEmrSupportedInstanceTypesRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataEmrSupportedInstanceTypes_ {
    fn extract_datasource_type(&self) -> String {
        "aws_emr_supported_instance_types".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataEmrSupportedInstanceTypes {
    pub tf_id: String,
    #[doc = ""]
    pub release_label: PrimField<String>,
}

impl BuildDataEmrSupportedInstanceTypes {
    pub fn build(self, stack: &mut Stack) -> DataEmrSupportedInstanceTypes {
        let out = DataEmrSupportedInstanceTypes(Rc::new(DataEmrSupportedInstanceTypes_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataEmrSupportedInstanceTypesData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                region: core::default::Default::default(),
                release_label: self.release_label,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataEmrSupportedInstanceTypesRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEmrSupportedInstanceTypesRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataEmrSupportedInstanceTypesRef {
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

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `release_label` after provisioning.\n"]
    pub fn release_label(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.release_label", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `supported_instance_types` after provisioning.\n"]
    pub fn supported_instance_types(&self) -> ListRef<DataEmrSupportedInstanceTypesSupportedInstanceTypesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.supported_instance_types", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataEmrSupportedInstanceTypesSupportedInstanceTypesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    architecture: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ebs_optimized_available: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ebs_optimized_by_default: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ebs_storage_only: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_family_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_64_bits_only: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memory_gb: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    number_of_disks: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_gb: Option<PrimField<f64>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vcpu: Option<PrimField<f64>>,
}

impl DataEmrSupportedInstanceTypesSupportedInstanceTypesEl {
    #[doc = "Set the field `architecture`.\n"]
    pub fn set_architecture(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.architecture = Some(v.into());
        self
    }

    #[doc = "Set the field `ebs_optimized_available`.\n"]
    pub fn set_ebs_optimized_available(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.ebs_optimized_available = Some(v.into());
        self
    }

    #[doc = "Set the field `ebs_optimized_by_default`.\n"]
    pub fn set_ebs_optimized_by_default(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.ebs_optimized_by_default = Some(v.into());
        self
    }

    #[doc = "Set the field `ebs_storage_only`.\n"]
    pub fn set_ebs_storage_only(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.ebs_storage_only = Some(v.into());
        self
    }

    #[doc = "Set the field `instance_family_id`.\n"]
    pub fn set_instance_family_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance_family_id = Some(v.into());
        self
    }

    #[doc = "Set the field `is_64_bits_only`.\n"]
    pub fn set_is_64_bits_only(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.is_64_bits_only = Some(v.into());
        self
    }

    #[doc = "Set the field `memory_gb`.\n"]
    pub fn set_memory_gb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.memory_gb = Some(v.into());
        self
    }

    #[doc = "Set the field `number_of_disks`.\n"]
    pub fn set_number_of_disks(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.number_of_disks = Some(v.into());
        self
    }

    #[doc = "Set the field `storage_gb`.\n"]
    pub fn set_storage_gb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.storage_gb = Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }

    #[doc = "Set the field `vcpu`.\n"]
    pub fn set_vcpu(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.vcpu = Some(v.into());
        self
    }
}

impl ToListMappable for DataEmrSupportedInstanceTypesSupportedInstanceTypesEl {
    type O = BlockAssignable<DataEmrSupportedInstanceTypesSupportedInstanceTypesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEmrSupportedInstanceTypesSupportedInstanceTypesEl {}

impl BuildDataEmrSupportedInstanceTypesSupportedInstanceTypesEl {
    pub fn build(self) -> DataEmrSupportedInstanceTypesSupportedInstanceTypesEl {
        DataEmrSupportedInstanceTypesSupportedInstanceTypesEl {
            architecture: core::default::Default::default(),
            ebs_optimized_available: core::default::Default::default(),
            ebs_optimized_by_default: core::default::Default::default(),
            ebs_storage_only: core::default::Default::default(),
            instance_family_id: core::default::Default::default(),
            is_64_bits_only: core::default::Default::default(),
            memory_gb: core::default::Default::default(),
            number_of_disks: core::default::Default::default(),
            storage_gb: core::default::Default::default(),
            type_: core::default::Default::default(),
            vcpu: core::default::Default::default(),
        }
    }
}

pub struct DataEmrSupportedInstanceTypesSupportedInstanceTypesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEmrSupportedInstanceTypesSupportedInstanceTypesElRef {
    fn new(shared: StackShared, base: String) -> DataEmrSupportedInstanceTypesSupportedInstanceTypesElRef {
        DataEmrSupportedInstanceTypesSupportedInstanceTypesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEmrSupportedInstanceTypesSupportedInstanceTypesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `architecture` after provisioning.\n"]
    pub fn architecture(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.architecture", self.base))
    }

    #[doc = "Get a reference to the value of field `ebs_optimized_available` after provisioning.\n"]
    pub fn ebs_optimized_available(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.ebs_optimized_available", self.base))
    }

    #[doc = "Get a reference to the value of field `ebs_optimized_by_default` after provisioning.\n"]
    pub fn ebs_optimized_by_default(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.ebs_optimized_by_default", self.base))
    }

    #[doc = "Get a reference to the value of field `ebs_storage_only` after provisioning.\n"]
    pub fn ebs_storage_only(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.ebs_storage_only", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_family_id` after provisioning.\n"]
    pub fn instance_family_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_family_id", self.base))
    }

    #[doc = "Get a reference to the value of field `is_64_bits_only` after provisioning.\n"]
    pub fn is_64_bits_only(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.is_64_bits_only", self.base))
    }

    #[doc = "Get a reference to the value of field `memory_gb` after provisioning.\n"]
    pub fn memory_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.memory_gb", self.base))
    }

    #[doc = "Get a reference to the value of field `number_of_disks` after provisioning.\n"]
    pub fn number_of_disks(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.number_of_disks", self.base))
    }

    #[doc = "Get a reference to the value of field `storage_gb` after provisioning.\n"]
    pub fn storage_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.storage_gb", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `vcpu` after provisioning.\n"]
    pub fn vcpu(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.vcpu", self.base))
    }
}
