use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct ProviderAwsData {
    #[serde(skip_serializing_if = "Option::is_none")]
    alias: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    access_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_account_ids: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_ca_bundle: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ec2_metadata_service_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ec2_metadata_service_endpoint_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    forbidden_account_ids: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http_proxy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    https_proxy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    insecure: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_retries: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    no_proxy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    profile: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retry_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_us_east_1_regional_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_use_path_style: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shared_config_files: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shared_credentials_files: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    skip_credentials_validation: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    skip_metadata_api_check: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    skip_region_validation: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    skip_requesting_account_id: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sts_region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag_policy_compliance: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    token: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    token_bucket_rate_limiter_capacity: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_dualstack_endpoint: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_fips_endpoint: Option<PrimField<bool>>,
}

struct ProviderAws_ {
    data: RefCell<ProviderAwsData>,
}

pub struct ProviderAws(Rc<ProviderAws_>);

impl ProviderAws {
    pub fn provider_ref(&self) -> String {
        let data = self.0.data.borrow();
        if let Some(alias) = &data.alias {
            format!("{}.{}", "aws", alias)
        } else {
            "aws".into()
        }
    }

    pub fn set_alias(self, alias: impl ToString) -> Self {
        self.0.data.borrow_mut().alias = Some(alias.to_string());
        self
    }

    #[doc = "Set the field `access_key`.\nThe access key for API operations. You can retrieve this\nfrom the 'Security & Credentials' section of the AWS console."]
    pub fn set_access_key(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().access_key = Some(v.into());
        self
    }

    #[doc = "Set the field `allowed_account_ids`.\n"]
    pub fn set_allowed_account_ids(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().allowed_account_ids = Some(v.into());
        self
    }

    #[doc = "Set the field `custom_ca_bundle`.\nFile containing custom root and intermediate certificates. Can also be configured using the `AWS_CA_BUNDLE` environment variable. (Setting `ca_bundle` in the shared config file is not supported.)"]
    pub fn set_custom_ca_bundle(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().custom_ca_bundle = Some(v.into());
        self
    }

    #[doc = "Set the field `ec2_metadata_service_endpoint`.\nAddress of the EC2 metadata service endpoint to use. Can also be configured using the `AWS_EC2_METADATA_SERVICE_ENDPOINT` environment variable."]
    pub fn set_ec2_metadata_service_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().ec2_metadata_service_endpoint = Some(v.into());
        self
    }

    #[doc = "Set the field `ec2_metadata_service_endpoint_mode`.\nProtocol to use with EC2 metadata service endpoint.Valid values are `IPv4` and `IPv6`. Can also be configured using the `AWS_EC2_METADATA_SERVICE_ENDPOINT_MODE` environment variable."]
    pub fn set_ec2_metadata_service_endpoint_mode(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().ec2_metadata_service_endpoint_mode = Some(v.into());
        self
    }

    #[doc = "Set the field `forbidden_account_ids`.\n"]
    pub fn set_forbidden_account_ids(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().forbidden_account_ids = Some(v.into());
        self
    }

    #[doc = "Set the field `http_proxy`.\nURL of a proxy to use for HTTP requests when accessing the AWS API. Can also be set using the `HTTP_PROXY` or `http_proxy` environment variables."]
    pub fn set_http_proxy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().http_proxy = Some(v.into());
        self
    }

    #[doc = "Set the field `https_proxy`.\nURL of a proxy to use for HTTPS requests when accessing the AWS API. Can also be set using the `HTTPS_PROXY` or `https_proxy` environment variables."]
    pub fn set_https_proxy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().https_proxy = Some(v.into());
        self
    }

    #[doc = "Set the field `insecure`.\nExplicitly allow the provider to perform \"insecure\" SSL requests. If omitted, default value is `false`"]
    pub fn set_insecure(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().insecure = Some(v.into());
        self
    }

    #[doc = "Set the field `max_retries`.\nThe maximum number of times an AWS API request is\nbeing executed. If the API request still fails, an error is\nthrown."]
    pub fn set_max_retries(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_retries = Some(v.into());
        self
    }

    #[doc = "Set the field `no_proxy`.\nComma-separated list of hosts that should not use HTTP or HTTPS proxies. Can also be set using the `NO_PROXY` or `no_proxy` environment variables."]
    pub fn set_no_proxy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().no_proxy = Some(v.into());
        self
    }

    #[doc = "Set the field `profile`.\nThe profile for API operations. If not set, the default profile\ncreated with `aws configure` will be used."]
    pub fn set_profile(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().profile = Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nThe region where AWS operations will take place. Examples\nare us-east-1, us-west-2, etc."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `retry_mode`.\nSpecifies how retries are attempted. Valid values are `standard` and `adaptive`. Can also be configured using the `AWS_RETRY_MODE` environment variable."]
    pub fn set_retry_mode(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().retry_mode = Some(v.into());
        self
    }

    #[doc = "Set the field `s3_us_east_1_regional_endpoint`.\nSpecifies whether S3 API calls in the `us-east-1` region use the legacy global endpoint or a regional endpoint. Valid values are `legacy` or `regional`. Can also be configured using the `AWS_S3_US_EAST_1_REGIONAL_ENDPOINT` environment variable or the `s3_us_east_1_regional_endpoint` shared config file parameter"]
    pub fn set_s3_us_east_1_regional_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().s3_us_east_1_regional_endpoint = Some(v.into());
        self
    }

    #[doc = "Set the field `s3_use_path_style`.\nSet this to true to enable the request to use path-style addressing,\ni.e., https://s3.amazonaws.com/BUCKET/KEY. By default, the S3 client will\nuse virtual hosted bucket addressing when possible\n(https://BUCKET.s3.amazonaws.com/KEY). Specific to the Amazon S3 service."]
    pub fn set_s3_use_path_style(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().s3_use_path_style = Some(v.into());
        self
    }

    #[doc = "Set the field `secret_key`.\nThe secret key for API operations. You can retrieve this\nfrom the 'Security & Credentials' section of the AWS console."]
    pub fn set_secret_key(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().secret_key = Some(v.into());
        self
    }

    #[doc = "Set the field `shared_config_files`.\nList of paths to shared config files. If not set, defaults to [~/.aws/config]."]
    pub fn set_shared_config_files(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().shared_config_files = Some(v.into());
        self
    }

    #[doc = "Set the field `shared_credentials_files`.\nList of paths to shared credentials files. If not set, defaults to [~/.aws/credentials]."]
    pub fn set_shared_credentials_files(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().shared_credentials_files = Some(v.into());
        self
    }

    #[doc = "Set the field `skip_credentials_validation`.\nSkip the credentials validation via STS API. Used for AWS API implementations that do not have STS available/implemented."]
    pub fn set_skip_credentials_validation(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().skip_credentials_validation = Some(v.into());
        self
    }

    #[doc = "Set the field `skip_metadata_api_check`.\nSkip the AWS Metadata API check. Used for AWS API implementations that do not have a metadata api endpoint."]
    pub fn set_skip_metadata_api_check(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().skip_metadata_api_check = Some(v.into());
        self
    }

    #[doc = "Set the field `skip_region_validation`.\nSkip static validation of region name. Used by users of alternative AWS-like APIs or users w/ access to regions that are not public (yet)."]
    pub fn set_skip_region_validation(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().skip_region_validation = Some(v.into());
        self
    }

    #[doc = "Set the field `skip_requesting_account_id`.\nSkip requesting the account ID. Used for AWS API implementations that do not have IAM/STS API and/or metadata API."]
    pub fn set_skip_requesting_account_id(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().skip_requesting_account_id = Some(v.into());
        self
    }

    #[doc = "Set the field `sts_region`.\nThe region where AWS STS operations will take place. Examples\nare us-east-1 and us-west-2."]
    pub fn set_sts_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().sts_region = Some(v.into());
        self
    }

    #[doc = "Set the field `tag_policy_compliance`.\nThe severity with which to enforce organizational tagging policies on resources managed by this provider instance. At this time this only includes compliance with required tag keys by resource type. Valid values are \"error\", \"warning\", and \"disabled\". When unset or \"disabled\", tag policy compliance will not be enforced by the provider. Can also be configured with the TF_AWS_TAG_POLICY_COMPLIANCE environment variable."]
    pub fn set_tag_policy_compliance(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().tag_policy_compliance = Some(v.into());
        self
    }

    #[doc = "Set the field `token`.\nsession token. A session token is only required if you are\nusing temporary security credentials."]
    pub fn set_token(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().token = Some(v.into());
        self
    }

    #[doc = "Set the field `token_bucket_rate_limiter_capacity`.\nThe capacity of the AWS SDK's token bucket rate limiter."]
    pub fn set_token_bucket_rate_limiter_capacity(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().token_bucket_rate_limiter_capacity = Some(v.into());
        self
    }

    #[doc = "Set the field `use_dualstack_endpoint`.\nResolve an endpoint with DualStack capability"]
    pub fn set_use_dualstack_endpoint(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().use_dualstack_endpoint = Some(v.into());
        self
    }

    #[doc = "Set the field `use_fips_endpoint`.\nResolve an endpoint with FIPS capability"]
    pub fn set_use_fips_endpoint(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().use_fips_endpoint = Some(v.into());
        self
    }
}

impl Provider for ProviderAws_ {
    fn extract_type_tf_id(&self) -> String {
        "aws".into()
    }

    fn extract_provider_type(&self) -> serde_json::Value {
        serde_json::json!({
            "source": "hashicorp/aws",
            "version": "6.22.1",
        })
    }

    fn extract_provider(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildProviderAws {}

impl BuildProviderAws {
    pub fn build(self, stack: &mut Stack) -> ProviderAws {
        let out = ProviderAws(Rc::new(ProviderAws_ {
            data: RefCell::new(ProviderAwsData {
                alias: None,
                access_key: core::default::Default::default(),
                allowed_account_ids: core::default::Default::default(),
                custom_ca_bundle: core::default::Default::default(),
                ec2_metadata_service_endpoint: core::default::Default::default(),
                ec2_metadata_service_endpoint_mode: core::default::Default::default(),
                forbidden_account_ids: core::default::Default::default(),
                http_proxy: core::default::Default::default(),
                https_proxy: core::default::Default::default(),
                insecure: core::default::Default::default(),
                max_retries: core::default::Default::default(),
                no_proxy: core::default::Default::default(),
                profile: core::default::Default::default(),
                region: core::default::Default::default(),
                retry_mode: core::default::Default::default(),
                s3_us_east_1_regional_endpoint: core::default::Default::default(),
                s3_use_path_style: core::default::Default::default(),
                secret_key: core::default::Default::default(),
                shared_config_files: core::default::Default::default(),
                shared_credentials_files: core::default::Default::default(),
                skip_credentials_validation: core::default::Default::default(),
                skip_metadata_api_check: core::default::Default::default(),
                skip_region_validation: core::default::Default::default(),
                skip_requesting_account_id: core::default::Default::default(),
                sts_region: core::default::Default::default(),
                tag_policy_compliance: core::default::Default::default(),
                token: core::default::Default::default(),
                token_bucket_rate_limiter_capacity: core::default::Default::default(),
                use_dualstack_endpoint: core::default::Default::default(),
                use_fips_endpoint: core::default::Default::default(),
            }),
        }));
        stack.add_provider(out.0.clone());
        out
    }
}
