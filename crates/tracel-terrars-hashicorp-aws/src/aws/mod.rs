pub mod provider;

pub use provider::*;

#[cfg(feature = "accessanalyzer_analyzer")]
pub mod accessanalyzer_analyzer;

#[cfg(feature = "accessanalyzer_analyzer")]
pub use accessanalyzer_analyzer::*;

#[cfg(feature = "accessanalyzer_archive_rule")]
pub mod accessanalyzer_archive_rule;

#[cfg(feature = "accessanalyzer_archive_rule")]
pub use accessanalyzer_archive_rule::*;

#[cfg(feature = "account_alternate_contact")]
pub mod account_alternate_contact;

#[cfg(feature = "account_alternate_contact")]
pub use account_alternate_contact::*;

#[cfg(feature = "account_primary_contact")]
pub mod account_primary_contact;

#[cfg(feature = "account_primary_contact")]
pub use account_primary_contact::*;

#[cfg(feature = "account_region")]
pub mod account_region;

#[cfg(feature = "account_region")]
pub use account_region::*;

#[cfg(feature = "acm_certificate")]
pub mod acm_certificate;

#[cfg(feature = "acm_certificate")]
pub use acm_certificate::*;

#[cfg(feature = "acm_certificate_validation")]
pub mod acm_certificate_validation;

#[cfg(feature = "acm_certificate_validation")]
pub use acm_certificate_validation::*;

#[cfg(feature = "acmpca_certificate")]
pub mod acmpca_certificate;

#[cfg(feature = "acmpca_certificate")]
pub use acmpca_certificate::*;

#[cfg(feature = "acmpca_certificate_authority")]
pub mod acmpca_certificate_authority;

#[cfg(feature = "acmpca_certificate_authority")]
pub use acmpca_certificate_authority::*;

#[cfg(feature = "acmpca_certificate_authority_certificate")]
pub mod acmpca_certificate_authority_certificate;

#[cfg(feature = "acmpca_certificate_authority_certificate")]
pub use acmpca_certificate_authority_certificate::*;

#[cfg(feature = "acmpca_permission")]
pub mod acmpca_permission;

#[cfg(feature = "acmpca_permission")]
pub use acmpca_permission::*;

#[cfg(feature = "acmpca_policy")]
pub mod acmpca_policy;

#[cfg(feature = "acmpca_policy")]
pub use acmpca_policy::*;

#[cfg(feature = "alb")]
pub mod alb;

#[cfg(feature = "alb")]
pub use alb::*;

#[cfg(feature = "alb_listener")]
pub mod alb_listener;

#[cfg(feature = "alb_listener")]
pub use alb_listener::*;

#[cfg(feature = "alb_listener_certificate")]
pub mod alb_listener_certificate;

#[cfg(feature = "alb_listener_certificate")]
pub use alb_listener_certificate::*;

#[cfg(feature = "alb_listener_rule")]
pub mod alb_listener_rule;

#[cfg(feature = "alb_listener_rule")]
pub use alb_listener_rule::*;

#[cfg(feature = "alb_target_group")]
pub mod alb_target_group;

#[cfg(feature = "alb_target_group")]
pub use alb_target_group::*;

#[cfg(feature = "alb_target_group_attachment")]
pub mod alb_target_group_attachment;

#[cfg(feature = "alb_target_group_attachment")]
pub use alb_target_group_attachment::*;

#[cfg(feature = "ami")]
pub mod ami;

#[cfg(feature = "ami")]
pub use ami::*;

#[cfg(feature = "ami_copy")]
pub mod ami_copy;

#[cfg(feature = "ami_copy")]
pub use ami_copy::*;

#[cfg(feature = "ami_from_instance")]
pub mod ami_from_instance;

#[cfg(feature = "ami_from_instance")]
pub use ami_from_instance::*;

#[cfg(feature = "ami_launch_permission")]
pub mod ami_launch_permission;

#[cfg(feature = "ami_launch_permission")]
pub use ami_launch_permission::*;

#[cfg(feature = "amplify_app")]
pub mod amplify_app;

#[cfg(feature = "amplify_app")]
pub use amplify_app::*;

#[cfg(feature = "amplify_backend_environment")]
pub mod amplify_backend_environment;

#[cfg(feature = "amplify_backend_environment")]
pub use amplify_backend_environment::*;

#[cfg(feature = "amplify_branch")]
pub mod amplify_branch;

#[cfg(feature = "amplify_branch")]
pub use amplify_branch::*;

#[cfg(feature = "amplify_domain_association")]
pub mod amplify_domain_association;

#[cfg(feature = "amplify_domain_association")]
pub use amplify_domain_association::*;

#[cfg(feature = "amplify_webhook")]
pub mod amplify_webhook;

#[cfg(feature = "amplify_webhook")]
pub use amplify_webhook::*;

#[cfg(feature = "api_gateway_account")]
pub mod api_gateway_account;

#[cfg(feature = "api_gateway_account")]
pub use api_gateway_account::*;

#[cfg(feature = "api_gateway_api_key")]
pub mod api_gateway_api_key;

#[cfg(feature = "api_gateway_api_key")]
pub use api_gateway_api_key::*;

#[cfg(feature = "api_gateway_authorizer")]
pub mod api_gateway_authorizer;

#[cfg(feature = "api_gateway_authorizer")]
pub use api_gateway_authorizer::*;

#[cfg(feature = "api_gateway_base_path_mapping")]
pub mod api_gateway_base_path_mapping;

#[cfg(feature = "api_gateway_base_path_mapping")]
pub use api_gateway_base_path_mapping::*;

#[cfg(feature = "api_gateway_client_certificate")]
pub mod api_gateway_client_certificate;

#[cfg(feature = "api_gateway_client_certificate")]
pub use api_gateway_client_certificate::*;

#[cfg(feature = "api_gateway_deployment")]
pub mod api_gateway_deployment;

#[cfg(feature = "api_gateway_deployment")]
pub use api_gateway_deployment::*;

#[cfg(feature = "api_gateway_documentation_part")]
pub mod api_gateway_documentation_part;

#[cfg(feature = "api_gateway_documentation_part")]
pub use api_gateway_documentation_part::*;

#[cfg(feature = "api_gateway_documentation_version")]
pub mod api_gateway_documentation_version;

#[cfg(feature = "api_gateway_documentation_version")]
pub use api_gateway_documentation_version::*;

#[cfg(feature = "api_gateway_domain_name")]
pub mod api_gateway_domain_name;

#[cfg(feature = "api_gateway_domain_name")]
pub use api_gateway_domain_name::*;

#[cfg(feature = "api_gateway_domain_name_access_association")]
pub mod api_gateway_domain_name_access_association;

#[cfg(feature = "api_gateway_domain_name_access_association")]
pub use api_gateway_domain_name_access_association::*;

#[cfg(feature = "api_gateway_gateway_response")]
pub mod api_gateway_gateway_response;

#[cfg(feature = "api_gateway_gateway_response")]
pub use api_gateway_gateway_response::*;

#[cfg(feature = "api_gateway_integration")]
pub mod api_gateway_integration;

#[cfg(feature = "api_gateway_integration")]
pub use api_gateway_integration::*;

#[cfg(feature = "api_gateway_integration_response")]
pub mod api_gateway_integration_response;

#[cfg(feature = "api_gateway_integration_response")]
pub use api_gateway_integration_response::*;

#[cfg(feature = "api_gateway_method")]
pub mod api_gateway_method;

#[cfg(feature = "api_gateway_method")]
pub use api_gateway_method::*;

#[cfg(feature = "api_gateway_method_response")]
pub mod api_gateway_method_response;

#[cfg(feature = "api_gateway_method_response")]
pub use api_gateway_method_response::*;

#[cfg(feature = "api_gateway_method_settings")]
pub mod api_gateway_method_settings;

#[cfg(feature = "api_gateway_method_settings")]
pub use api_gateway_method_settings::*;

#[cfg(feature = "api_gateway_model")]
pub mod api_gateway_model;

#[cfg(feature = "api_gateway_model")]
pub use api_gateway_model::*;

#[cfg(feature = "api_gateway_request_validator")]
pub mod api_gateway_request_validator;

#[cfg(feature = "api_gateway_request_validator")]
pub use api_gateway_request_validator::*;

#[cfg(feature = "api_gateway_resource")]
pub mod api_gateway_resource;

#[cfg(feature = "api_gateway_resource")]
pub use api_gateway_resource::*;

#[cfg(feature = "api_gateway_rest_api")]
pub mod api_gateway_rest_api;

#[cfg(feature = "api_gateway_rest_api")]
pub use api_gateway_rest_api::*;

#[cfg(feature = "api_gateway_rest_api_policy")]
pub mod api_gateway_rest_api_policy;

#[cfg(feature = "api_gateway_rest_api_policy")]
pub use api_gateway_rest_api_policy::*;

#[cfg(feature = "api_gateway_rest_api_put")]
pub mod api_gateway_rest_api_put;

#[cfg(feature = "api_gateway_rest_api_put")]
pub use api_gateway_rest_api_put::*;

#[cfg(feature = "api_gateway_stage")]
pub mod api_gateway_stage;

#[cfg(feature = "api_gateway_stage")]
pub use api_gateway_stage::*;

#[cfg(feature = "api_gateway_usage_plan")]
pub mod api_gateway_usage_plan;

#[cfg(feature = "api_gateway_usage_plan")]
pub use api_gateway_usage_plan::*;

#[cfg(feature = "api_gateway_usage_plan_key")]
pub mod api_gateway_usage_plan_key;

#[cfg(feature = "api_gateway_usage_plan_key")]
pub use api_gateway_usage_plan_key::*;

#[cfg(feature = "api_gateway_vpc_link")]
pub mod api_gateway_vpc_link;

#[cfg(feature = "api_gateway_vpc_link")]
pub use api_gateway_vpc_link::*;

#[cfg(feature = "apigatewayv2_api")]
pub mod apigatewayv2_api;

#[cfg(feature = "apigatewayv2_api")]
pub use apigatewayv2_api::*;

#[cfg(feature = "apigatewayv2_api_mapping")]
pub mod apigatewayv2_api_mapping;

#[cfg(feature = "apigatewayv2_api_mapping")]
pub use apigatewayv2_api_mapping::*;

#[cfg(feature = "apigatewayv2_authorizer")]
pub mod apigatewayv2_authorizer;

#[cfg(feature = "apigatewayv2_authorizer")]
pub use apigatewayv2_authorizer::*;

#[cfg(feature = "apigatewayv2_deployment")]
pub mod apigatewayv2_deployment;

#[cfg(feature = "apigatewayv2_deployment")]
pub use apigatewayv2_deployment::*;

#[cfg(feature = "apigatewayv2_domain_name")]
pub mod apigatewayv2_domain_name;

#[cfg(feature = "apigatewayv2_domain_name")]
pub use apigatewayv2_domain_name::*;

#[cfg(feature = "apigatewayv2_integration")]
pub mod apigatewayv2_integration;

#[cfg(feature = "apigatewayv2_integration")]
pub use apigatewayv2_integration::*;

#[cfg(feature = "apigatewayv2_integration_response")]
pub mod apigatewayv2_integration_response;

#[cfg(feature = "apigatewayv2_integration_response")]
pub use apigatewayv2_integration_response::*;

#[cfg(feature = "apigatewayv2_model")]
pub mod apigatewayv2_model;

#[cfg(feature = "apigatewayv2_model")]
pub use apigatewayv2_model::*;

#[cfg(feature = "apigatewayv2_route")]
pub mod apigatewayv2_route;

#[cfg(feature = "apigatewayv2_route")]
pub use apigatewayv2_route::*;

#[cfg(feature = "apigatewayv2_route_response")]
pub mod apigatewayv2_route_response;

#[cfg(feature = "apigatewayv2_route_response")]
pub use apigatewayv2_route_response::*;

#[cfg(feature = "apigatewayv2_stage")]
pub mod apigatewayv2_stage;

#[cfg(feature = "apigatewayv2_stage")]
pub use apigatewayv2_stage::*;

#[cfg(feature = "apigatewayv2_vpc_link")]
pub mod apigatewayv2_vpc_link;

#[cfg(feature = "apigatewayv2_vpc_link")]
pub use apigatewayv2_vpc_link::*;

#[cfg(feature = "app_cookie_stickiness_policy")]
pub mod app_cookie_stickiness_policy;

#[cfg(feature = "app_cookie_stickiness_policy")]
pub use app_cookie_stickiness_policy::*;

#[cfg(feature = "appautoscaling_policy")]
pub mod appautoscaling_policy;

#[cfg(feature = "appautoscaling_policy")]
pub use appautoscaling_policy::*;

#[cfg(feature = "appautoscaling_scheduled_action")]
pub mod appautoscaling_scheduled_action;

#[cfg(feature = "appautoscaling_scheduled_action")]
pub use appautoscaling_scheduled_action::*;

#[cfg(feature = "appautoscaling_target")]
pub mod appautoscaling_target;

#[cfg(feature = "appautoscaling_target")]
pub use appautoscaling_target::*;

#[cfg(feature = "appconfig_application")]
pub mod appconfig_application;

#[cfg(feature = "appconfig_application")]
pub use appconfig_application::*;

#[cfg(feature = "appconfig_configuration_profile")]
pub mod appconfig_configuration_profile;

#[cfg(feature = "appconfig_configuration_profile")]
pub use appconfig_configuration_profile::*;

#[cfg(feature = "appconfig_deployment")]
pub mod appconfig_deployment;

#[cfg(feature = "appconfig_deployment")]
pub use appconfig_deployment::*;

#[cfg(feature = "appconfig_deployment_strategy")]
pub mod appconfig_deployment_strategy;

#[cfg(feature = "appconfig_deployment_strategy")]
pub use appconfig_deployment_strategy::*;

#[cfg(feature = "appconfig_environment")]
pub mod appconfig_environment;

#[cfg(feature = "appconfig_environment")]
pub use appconfig_environment::*;

#[cfg(feature = "appconfig_extension")]
pub mod appconfig_extension;

#[cfg(feature = "appconfig_extension")]
pub use appconfig_extension::*;

#[cfg(feature = "appconfig_extension_association")]
pub mod appconfig_extension_association;

#[cfg(feature = "appconfig_extension_association")]
pub use appconfig_extension_association::*;

#[cfg(feature = "appconfig_hosted_configuration_version")]
pub mod appconfig_hosted_configuration_version;

#[cfg(feature = "appconfig_hosted_configuration_version")]
pub use appconfig_hosted_configuration_version::*;

#[cfg(feature = "appfabric_app_authorization")]
pub mod appfabric_app_authorization;

#[cfg(feature = "appfabric_app_authorization")]
pub use appfabric_app_authorization::*;

#[cfg(feature = "appfabric_app_authorization_connection")]
pub mod appfabric_app_authorization_connection;

#[cfg(feature = "appfabric_app_authorization_connection")]
pub use appfabric_app_authorization_connection::*;

#[cfg(feature = "appfabric_app_bundle")]
pub mod appfabric_app_bundle;

#[cfg(feature = "appfabric_app_bundle")]
pub use appfabric_app_bundle::*;

#[cfg(feature = "appfabric_ingestion")]
pub mod appfabric_ingestion;

#[cfg(feature = "appfabric_ingestion")]
pub use appfabric_ingestion::*;

#[cfg(feature = "appfabric_ingestion_destination")]
pub mod appfabric_ingestion_destination;

#[cfg(feature = "appfabric_ingestion_destination")]
pub use appfabric_ingestion_destination::*;

#[cfg(feature = "appflow_connector_profile")]
pub mod appflow_connector_profile;

#[cfg(feature = "appflow_connector_profile")]
pub use appflow_connector_profile::*;

#[cfg(feature = "appflow_flow")]
pub mod appflow_flow;

#[cfg(feature = "appflow_flow")]
pub use appflow_flow::*;

#[cfg(feature = "appintegrations_data_integration")]
pub mod appintegrations_data_integration;

#[cfg(feature = "appintegrations_data_integration")]
pub use appintegrations_data_integration::*;

#[cfg(feature = "appintegrations_event_integration")]
pub mod appintegrations_event_integration;

#[cfg(feature = "appintegrations_event_integration")]
pub use appintegrations_event_integration::*;

#[cfg(feature = "applicationinsights_application")]
pub mod applicationinsights_application;

#[cfg(feature = "applicationinsights_application")]
pub use applicationinsights_application::*;

#[cfg(feature = "appmesh_gateway_route")]
pub mod appmesh_gateway_route;

#[cfg(feature = "appmesh_gateway_route")]
pub use appmesh_gateway_route::*;

#[cfg(feature = "appmesh_mesh")]
pub mod appmesh_mesh;

#[cfg(feature = "appmesh_mesh")]
pub use appmesh_mesh::*;

#[cfg(feature = "appmesh_route")]
pub mod appmesh_route;

#[cfg(feature = "appmesh_route")]
pub use appmesh_route::*;

#[cfg(feature = "appmesh_virtual_gateway")]
pub mod appmesh_virtual_gateway;

#[cfg(feature = "appmesh_virtual_gateway")]
pub use appmesh_virtual_gateway::*;

#[cfg(feature = "appmesh_virtual_node")]
pub mod appmesh_virtual_node;

#[cfg(feature = "appmesh_virtual_node")]
pub use appmesh_virtual_node::*;

#[cfg(feature = "appmesh_virtual_router")]
pub mod appmesh_virtual_router;

#[cfg(feature = "appmesh_virtual_router")]
pub use appmesh_virtual_router::*;

#[cfg(feature = "appmesh_virtual_service")]
pub mod appmesh_virtual_service;

#[cfg(feature = "appmesh_virtual_service")]
pub use appmesh_virtual_service::*;

#[cfg(feature = "apprunner_auto_scaling_configuration_version")]
pub mod apprunner_auto_scaling_configuration_version;

#[cfg(feature = "apprunner_auto_scaling_configuration_version")]
pub use apprunner_auto_scaling_configuration_version::*;

#[cfg(feature = "apprunner_connection")]
pub mod apprunner_connection;

#[cfg(feature = "apprunner_connection")]
pub use apprunner_connection::*;

#[cfg(feature = "apprunner_custom_domain_association")]
pub mod apprunner_custom_domain_association;

#[cfg(feature = "apprunner_custom_domain_association")]
pub use apprunner_custom_domain_association::*;

#[cfg(feature = "apprunner_default_auto_scaling_configuration_version")]
pub mod apprunner_default_auto_scaling_configuration_version;

#[cfg(feature = "apprunner_default_auto_scaling_configuration_version")]
pub use apprunner_default_auto_scaling_configuration_version::*;

#[cfg(feature = "apprunner_deployment")]
pub mod apprunner_deployment;

#[cfg(feature = "apprunner_deployment")]
pub use apprunner_deployment::*;

#[cfg(feature = "apprunner_observability_configuration")]
pub mod apprunner_observability_configuration;

#[cfg(feature = "apprunner_observability_configuration")]
pub use apprunner_observability_configuration::*;

#[cfg(feature = "apprunner_service")]
pub mod apprunner_service;

#[cfg(feature = "apprunner_service")]
pub use apprunner_service::*;

#[cfg(feature = "apprunner_vpc_connector")]
pub mod apprunner_vpc_connector;

#[cfg(feature = "apprunner_vpc_connector")]
pub use apprunner_vpc_connector::*;

#[cfg(feature = "apprunner_vpc_ingress_connection")]
pub mod apprunner_vpc_ingress_connection;

#[cfg(feature = "apprunner_vpc_ingress_connection")]
pub use apprunner_vpc_ingress_connection::*;

#[cfg(feature = "appstream_directory_config")]
pub mod appstream_directory_config;

#[cfg(feature = "appstream_directory_config")]
pub use appstream_directory_config::*;

#[cfg(feature = "appstream_fleet")]
pub mod appstream_fleet;

#[cfg(feature = "appstream_fleet")]
pub use appstream_fleet::*;

#[cfg(feature = "appstream_fleet_stack_association")]
pub mod appstream_fleet_stack_association;

#[cfg(feature = "appstream_fleet_stack_association")]
pub use appstream_fleet_stack_association::*;

#[cfg(feature = "appstream_image_builder")]
pub mod appstream_image_builder;

#[cfg(feature = "appstream_image_builder")]
pub use appstream_image_builder::*;

#[cfg(feature = "appstream_stack")]
pub mod appstream_stack;

#[cfg(feature = "appstream_stack")]
pub use appstream_stack::*;

#[cfg(feature = "appstream_user")]
pub mod appstream_user;

#[cfg(feature = "appstream_user")]
pub use appstream_user::*;

#[cfg(feature = "appstream_user_stack_association")]
pub mod appstream_user_stack_association;

#[cfg(feature = "appstream_user_stack_association")]
pub use appstream_user_stack_association::*;

#[cfg(feature = "appsync_api")]
pub mod appsync_api;

#[cfg(feature = "appsync_api")]
pub use appsync_api::*;

#[cfg(feature = "appsync_api_cache")]
pub mod appsync_api_cache;

#[cfg(feature = "appsync_api_cache")]
pub use appsync_api_cache::*;

#[cfg(feature = "appsync_api_key")]
pub mod appsync_api_key;

#[cfg(feature = "appsync_api_key")]
pub use appsync_api_key::*;

#[cfg(feature = "appsync_channel_namespace")]
pub mod appsync_channel_namespace;

#[cfg(feature = "appsync_channel_namespace")]
pub use appsync_channel_namespace::*;

#[cfg(feature = "appsync_datasource")]
pub mod appsync_datasource;

#[cfg(feature = "appsync_datasource")]
pub use appsync_datasource::*;

#[cfg(feature = "appsync_domain_name")]
pub mod appsync_domain_name;

#[cfg(feature = "appsync_domain_name")]
pub use appsync_domain_name::*;

#[cfg(feature = "appsync_domain_name_api_association")]
pub mod appsync_domain_name_api_association;

#[cfg(feature = "appsync_domain_name_api_association")]
pub use appsync_domain_name_api_association::*;

#[cfg(feature = "appsync_function")]
pub mod appsync_function;

#[cfg(feature = "appsync_function")]
pub use appsync_function::*;

#[cfg(feature = "appsync_graphql_api")]
pub mod appsync_graphql_api;

#[cfg(feature = "appsync_graphql_api")]
pub use appsync_graphql_api::*;

#[cfg(feature = "appsync_resolver")]
pub mod appsync_resolver;

#[cfg(feature = "appsync_resolver")]
pub use appsync_resolver::*;

#[cfg(feature = "appsync_source_api_association")]
pub mod appsync_source_api_association;

#[cfg(feature = "appsync_source_api_association")]
pub use appsync_source_api_association::*;

#[cfg(feature = "appsync_type")]
pub mod appsync_type;

#[cfg(feature = "appsync_type")]
pub use appsync_type::*;

#[cfg(feature = "athena_capacity_reservation")]
pub mod athena_capacity_reservation;

#[cfg(feature = "athena_capacity_reservation")]
pub use athena_capacity_reservation::*;

#[cfg(feature = "athena_data_catalog")]
pub mod athena_data_catalog;

#[cfg(feature = "athena_data_catalog")]
pub use athena_data_catalog::*;

#[cfg(feature = "athena_database")]
pub mod athena_database;

#[cfg(feature = "athena_database")]
pub use athena_database::*;

#[cfg(feature = "athena_named_query")]
pub mod athena_named_query;

#[cfg(feature = "athena_named_query")]
pub use athena_named_query::*;

#[cfg(feature = "athena_prepared_statement")]
pub mod athena_prepared_statement;

#[cfg(feature = "athena_prepared_statement")]
pub use athena_prepared_statement::*;

#[cfg(feature = "athena_workgroup")]
pub mod athena_workgroup;

#[cfg(feature = "athena_workgroup")]
pub use athena_workgroup::*;

#[cfg(feature = "auditmanager_account_registration")]
pub mod auditmanager_account_registration;

#[cfg(feature = "auditmanager_account_registration")]
pub use auditmanager_account_registration::*;

#[cfg(feature = "auditmanager_assessment")]
pub mod auditmanager_assessment;

#[cfg(feature = "auditmanager_assessment")]
pub use auditmanager_assessment::*;

#[cfg(feature = "auditmanager_assessment_delegation")]
pub mod auditmanager_assessment_delegation;

#[cfg(feature = "auditmanager_assessment_delegation")]
pub use auditmanager_assessment_delegation::*;

#[cfg(feature = "auditmanager_assessment_report")]
pub mod auditmanager_assessment_report;

#[cfg(feature = "auditmanager_assessment_report")]
pub use auditmanager_assessment_report::*;

#[cfg(feature = "auditmanager_control")]
pub mod auditmanager_control;

#[cfg(feature = "auditmanager_control")]
pub use auditmanager_control::*;

#[cfg(feature = "auditmanager_framework")]
pub mod auditmanager_framework;

#[cfg(feature = "auditmanager_framework")]
pub use auditmanager_framework::*;

#[cfg(feature = "auditmanager_framework_share")]
pub mod auditmanager_framework_share;

#[cfg(feature = "auditmanager_framework_share")]
pub use auditmanager_framework_share::*;

#[cfg(feature = "auditmanager_organization_admin_account_registration")]
pub mod auditmanager_organization_admin_account_registration;

#[cfg(feature = "auditmanager_organization_admin_account_registration")]
pub use auditmanager_organization_admin_account_registration::*;

#[cfg(feature = "autoscaling_attachment")]
pub mod autoscaling_attachment;

#[cfg(feature = "autoscaling_attachment")]
pub use autoscaling_attachment::*;

#[cfg(feature = "autoscaling_group")]
pub mod autoscaling_group;

#[cfg(feature = "autoscaling_group")]
pub use autoscaling_group::*;

#[cfg(feature = "autoscaling_group_tag")]
pub mod autoscaling_group_tag;

#[cfg(feature = "autoscaling_group_tag")]
pub use autoscaling_group_tag::*;

#[cfg(feature = "autoscaling_lifecycle_hook")]
pub mod autoscaling_lifecycle_hook;

#[cfg(feature = "autoscaling_lifecycle_hook")]
pub use autoscaling_lifecycle_hook::*;

#[cfg(feature = "autoscaling_notification")]
pub mod autoscaling_notification;

#[cfg(feature = "autoscaling_notification")]
pub use autoscaling_notification::*;

#[cfg(feature = "autoscaling_policy")]
pub mod autoscaling_policy;

#[cfg(feature = "autoscaling_policy")]
pub use autoscaling_policy::*;

#[cfg(feature = "autoscaling_schedule")]
pub mod autoscaling_schedule;

#[cfg(feature = "autoscaling_schedule")]
pub use autoscaling_schedule::*;

#[cfg(feature = "autoscaling_traffic_source_attachment")]
pub mod autoscaling_traffic_source_attachment;

#[cfg(feature = "autoscaling_traffic_source_attachment")]
pub use autoscaling_traffic_source_attachment::*;

#[cfg(feature = "autoscalingplans_scaling_plan")]
pub mod autoscalingplans_scaling_plan;

#[cfg(feature = "autoscalingplans_scaling_plan")]
pub use autoscalingplans_scaling_plan::*;

#[cfg(feature = "backup_framework")]
pub mod backup_framework;

#[cfg(feature = "backup_framework")]
pub use backup_framework::*;

#[cfg(feature = "backup_global_settings")]
pub mod backup_global_settings;

#[cfg(feature = "backup_global_settings")]
pub use backup_global_settings::*;

#[cfg(feature = "backup_logically_air_gapped_vault")]
pub mod backup_logically_air_gapped_vault;

#[cfg(feature = "backup_logically_air_gapped_vault")]
pub use backup_logically_air_gapped_vault::*;

#[cfg(feature = "backup_plan")]
pub mod backup_plan;

#[cfg(feature = "backup_plan")]
pub use backup_plan::*;

#[cfg(feature = "backup_region_settings")]
pub mod backup_region_settings;

#[cfg(feature = "backup_region_settings")]
pub use backup_region_settings::*;

#[cfg(feature = "backup_report_plan")]
pub mod backup_report_plan;

#[cfg(feature = "backup_report_plan")]
pub use backup_report_plan::*;

#[cfg(feature = "backup_restore_testing_plan")]
pub mod backup_restore_testing_plan;

#[cfg(feature = "backup_restore_testing_plan")]
pub use backup_restore_testing_plan::*;

#[cfg(feature = "backup_restore_testing_selection")]
pub mod backup_restore_testing_selection;

#[cfg(feature = "backup_restore_testing_selection")]
pub use backup_restore_testing_selection::*;

#[cfg(feature = "backup_selection")]
pub mod backup_selection;

#[cfg(feature = "backup_selection")]
pub use backup_selection::*;

#[cfg(feature = "backup_vault")]
pub mod backup_vault;

#[cfg(feature = "backup_vault")]
pub use backup_vault::*;

#[cfg(feature = "backup_vault_lock_configuration")]
pub mod backup_vault_lock_configuration;

#[cfg(feature = "backup_vault_lock_configuration")]
pub use backup_vault_lock_configuration::*;

#[cfg(feature = "backup_vault_notifications")]
pub mod backup_vault_notifications;

#[cfg(feature = "backup_vault_notifications")]
pub use backup_vault_notifications::*;

#[cfg(feature = "backup_vault_policy")]
pub mod backup_vault_policy;

#[cfg(feature = "backup_vault_policy")]
pub use backup_vault_policy::*;

#[cfg(feature = "batch_compute_environment")]
pub mod batch_compute_environment;

#[cfg(feature = "batch_compute_environment")]
pub use batch_compute_environment::*;

#[cfg(feature = "batch_job_definition")]
pub mod batch_job_definition;

#[cfg(feature = "batch_job_definition")]
pub use batch_job_definition::*;

#[cfg(feature = "batch_job_queue")]
pub mod batch_job_queue;

#[cfg(feature = "batch_job_queue")]
pub use batch_job_queue::*;

#[cfg(feature = "batch_scheduling_policy")]
pub mod batch_scheduling_policy;

#[cfg(feature = "batch_scheduling_policy")]
pub use batch_scheduling_policy::*;

#[cfg(feature = "bedrock_custom_model")]
pub mod bedrock_custom_model;

#[cfg(feature = "bedrock_custom_model")]
pub use bedrock_custom_model::*;

#[cfg(feature = "bedrock_guardrail")]
pub mod bedrock_guardrail;

#[cfg(feature = "bedrock_guardrail")]
pub use bedrock_guardrail::*;

#[cfg(feature = "bedrock_guardrail_version")]
pub mod bedrock_guardrail_version;

#[cfg(feature = "bedrock_guardrail_version")]
pub use bedrock_guardrail_version::*;

#[cfg(feature = "bedrock_inference_profile")]
pub mod bedrock_inference_profile;

#[cfg(feature = "bedrock_inference_profile")]
pub use bedrock_inference_profile::*;

#[cfg(feature = "bedrock_model_invocation_logging_configuration")]
pub mod bedrock_model_invocation_logging_configuration;

#[cfg(feature = "bedrock_model_invocation_logging_configuration")]
pub use bedrock_model_invocation_logging_configuration::*;

#[cfg(feature = "bedrock_provisioned_model_throughput")]
pub mod bedrock_provisioned_model_throughput;

#[cfg(feature = "bedrock_provisioned_model_throughput")]
pub use bedrock_provisioned_model_throughput::*;

#[cfg(feature = "bedrockagent_agent")]
pub mod bedrockagent_agent;

#[cfg(feature = "bedrockagent_agent")]
pub use bedrockagent_agent::*;

#[cfg(feature = "bedrockagent_agent_action_group")]
pub mod bedrockagent_agent_action_group;

#[cfg(feature = "bedrockagent_agent_action_group")]
pub use bedrockagent_agent_action_group::*;

#[cfg(feature = "bedrockagent_agent_alias")]
pub mod bedrockagent_agent_alias;

#[cfg(feature = "bedrockagent_agent_alias")]
pub use bedrockagent_agent_alias::*;

#[cfg(feature = "bedrockagent_agent_collaborator")]
pub mod bedrockagent_agent_collaborator;

#[cfg(feature = "bedrockagent_agent_collaborator")]
pub use bedrockagent_agent_collaborator::*;

#[cfg(feature = "bedrockagent_agent_knowledge_base_association")]
pub mod bedrockagent_agent_knowledge_base_association;

#[cfg(feature = "bedrockagent_agent_knowledge_base_association")]
pub use bedrockagent_agent_knowledge_base_association::*;

#[cfg(feature = "bedrockagent_data_source")]
pub mod bedrockagent_data_source;

#[cfg(feature = "bedrockagent_data_source")]
pub use bedrockagent_data_source::*;

#[cfg(feature = "bedrockagent_flow")]
pub mod bedrockagent_flow;

#[cfg(feature = "bedrockagent_flow")]
pub use bedrockagent_flow::*;

#[cfg(feature = "bedrockagent_knowledge_base")]
pub mod bedrockagent_knowledge_base;

#[cfg(feature = "bedrockagent_knowledge_base")]
pub use bedrockagent_knowledge_base::*;

#[cfg(feature = "bedrockagent_prompt")]
pub mod bedrockagent_prompt;

#[cfg(feature = "bedrockagent_prompt")]
pub use bedrockagent_prompt::*;

#[cfg(feature = "bedrockagentcore_agent_runtime")]
pub mod bedrockagentcore_agent_runtime;

#[cfg(feature = "bedrockagentcore_agent_runtime")]
pub use bedrockagentcore_agent_runtime::*;

#[cfg(feature = "bedrockagentcore_agent_runtime_endpoint")]
pub mod bedrockagentcore_agent_runtime_endpoint;

#[cfg(feature = "bedrockagentcore_agent_runtime_endpoint")]
pub use bedrockagentcore_agent_runtime_endpoint::*;

#[cfg(feature = "bedrockagentcore_api_key_credential_provider")]
pub mod bedrockagentcore_api_key_credential_provider;

#[cfg(feature = "bedrockagentcore_api_key_credential_provider")]
pub use bedrockagentcore_api_key_credential_provider::*;

#[cfg(feature = "bedrockagentcore_browser")]
pub mod bedrockagentcore_browser;

#[cfg(feature = "bedrockagentcore_browser")]
pub use bedrockagentcore_browser::*;

#[cfg(feature = "bedrockagentcore_code_interpreter")]
pub mod bedrockagentcore_code_interpreter;

#[cfg(feature = "bedrockagentcore_code_interpreter")]
pub use bedrockagentcore_code_interpreter::*;

#[cfg(feature = "bedrockagentcore_gateway")]
pub mod bedrockagentcore_gateway;

#[cfg(feature = "bedrockagentcore_gateway")]
pub use bedrockagentcore_gateway::*;

#[cfg(feature = "bedrockagentcore_gateway_target")]
pub mod bedrockagentcore_gateway_target;

#[cfg(feature = "bedrockagentcore_gateway_target")]
pub use bedrockagentcore_gateway_target::*;

#[cfg(feature = "bedrockagentcore_memory")]
pub mod bedrockagentcore_memory;

#[cfg(feature = "bedrockagentcore_memory")]
pub use bedrockagentcore_memory::*;

#[cfg(feature = "bedrockagentcore_memory_strategy")]
pub mod bedrockagentcore_memory_strategy;

#[cfg(feature = "bedrockagentcore_memory_strategy")]
pub use bedrockagentcore_memory_strategy::*;

#[cfg(feature = "bedrockagentcore_oauth2_credential_provider")]
pub mod bedrockagentcore_oauth2_credential_provider;

#[cfg(feature = "bedrockagentcore_oauth2_credential_provider")]
pub use bedrockagentcore_oauth2_credential_provider::*;

#[cfg(feature = "bedrockagentcore_token_vault_cmk")]
pub mod bedrockagentcore_token_vault_cmk;

#[cfg(feature = "bedrockagentcore_token_vault_cmk")]
pub use bedrockagentcore_token_vault_cmk::*;

#[cfg(feature = "bedrockagentcore_workload_identity")]
pub mod bedrockagentcore_workload_identity;

#[cfg(feature = "bedrockagentcore_workload_identity")]
pub use bedrockagentcore_workload_identity::*;

#[cfg(feature = "billing_view")]
pub mod billing_view;

#[cfg(feature = "billing_view")]
pub use billing_view::*;

#[cfg(feature = "budgets_budget")]
pub mod budgets_budget;

#[cfg(feature = "budgets_budget")]
pub use budgets_budget::*;

#[cfg(feature = "budgets_budget_action")]
pub mod budgets_budget_action;

#[cfg(feature = "budgets_budget_action")]
pub use budgets_budget_action::*;

#[cfg(feature = "ce_anomaly_monitor")]
pub mod ce_anomaly_monitor;

#[cfg(feature = "ce_anomaly_monitor")]
pub use ce_anomaly_monitor::*;

#[cfg(feature = "ce_anomaly_subscription")]
pub mod ce_anomaly_subscription;

#[cfg(feature = "ce_anomaly_subscription")]
pub use ce_anomaly_subscription::*;

#[cfg(feature = "ce_cost_allocation_tag")]
pub mod ce_cost_allocation_tag;

#[cfg(feature = "ce_cost_allocation_tag")]
pub use ce_cost_allocation_tag::*;

#[cfg(feature = "ce_cost_category")]
pub mod ce_cost_category;

#[cfg(feature = "ce_cost_category")]
pub use ce_cost_category::*;

#[cfg(feature = "chatbot_slack_channel_configuration")]
pub mod chatbot_slack_channel_configuration;

#[cfg(feature = "chatbot_slack_channel_configuration")]
pub use chatbot_slack_channel_configuration::*;

#[cfg(feature = "chatbot_teams_channel_configuration")]
pub mod chatbot_teams_channel_configuration;

#[cfg(feature = "chatbot_teams_channel_configuration")]
pub use chatbot_teams_channel_configuration::*;

#[cfg(feature = "chime_voice_connector")]
pub mod chime_voice_connector;

#[cfg(feature = "chime_voice_connector")]
pub use chime_voice_connector::*;

#[cfg(feature = "chime_voice_connector_group")]
pub mod chime_voice_connector_group;

#[cfg(feature = "chime_voice_connector_group")]
pub use chime_voice_connector_group::*;

#[cfg(feature = "chime_voice_connector_logging")]
pub mod chime_voice_connector_logging;

#[cfg(feature = "chime_voice_connector_logging")]
pub use chime_voice_connector_logging::*;

#[cfg(feature = "chime_voice_connector_origination")]
pub mod chime_voice_connector_origination;

#[cfg(feature = "chime_voice_connector_origination")]
pub use chime_voice_connector_origination::*;

#[cfg(feature = "chime_voice_connector_streaming")]
pub mod chime_voice_connector_streaming;

#[cfg(feature = "chime_voice_connector_streaming")]
pub use chime_voice_connector_streaming::*;

#[cfg(feature = "chime_voice_connector_termination")]
pub mod chime_voice_connector_termination;

#[cfg(feature = "chime_voice_connector_termination")]
pub use chime_voice_connector_termination::*;

#[cfg(feature = "chime_voice_connector_termination_credentials")]
pub mod chime_voice_connector_termination_credentials;

#[cfg(feature = "chime_voice_connector_termination_credentials")]
pub use chime_voice_connector_termination_credentials::*;

#[cfg(feature = "chimesdkmediapipelines_media_insights_pipeline_configuration")]
pub mod chimesdkmediapipelines_media_insights_pipeline_configuration;

#[cfg(feature = "chimesdkmediapipelines_media_insights_pipeline_configuration")]
pub use chimesdkmediapipelines_media_insights_pipeline_configuration::*;

#[cfg(feature = "chimesdkvoice_global_settings")]
pub mod chimesdkvoice_global_settings;

#[cfg(feature = "chimesdkvoice_global_settings")]
pub use chimesdkvoice_global_settings::*;

#[cfg(feature = "chimesdkvoice_sip_media_application")]
pub mod chimesdkvoice_sip_media_application;

#[cfg(feature = "chimesdkvoice_sip_media_application")]
pub use chimesdkvoice_sip_media_application::*;

#[cfg(feature = "chimesdkvoice_sip_rule")]
pub mod chimesdkvoice_sip_rule;

#[cfg(feature = "chimesdkvoice_sip_rule")]
pub use chimesdkvoice_sip_rule::*;

#[cfg(feature = "chimesdkvoice_voice_profile_domain")]
pub mod chimesdkvoice_voice_profile_domain;

#[cfg(feature = "chimesdkvoice_voice_profile_domain")]
pub use chimesdkvoice_voice_profile_domain::*;

#[cfg(feature = "cleanrooms_collaboration")]
pub mod cleanrooms_collaboration;

#[cfg(feature = "cleanrooms_collaboration")]
pub use cleanrooms_collaboration::*;

#[cfg(feature = "cleanrooms_configured_table")]
pub mod cleanrooms_configured_table;

#[cfg(feature = "cleanrooms_configured_table")]
pub use cleanrooms_configured_table::*;

#[cfg(feature = "cleanrooms_membership")]
pub mod cleanrooms_membership;

#[cfg(feature = "cleanrooms_membership")]
pub use cleanrooms_membership::*;

#[cfg(feature = "cloud9_environment_ec2")]
pub mod cloud9_environment_ec2;

#[cfg(feature = "cloud9_environment_ec2")]
pub use cloud9_environment_ec2::*;

#[cfg(feature = "cloud9_environment_membership")]
pub mod cloud9_environment_membership;

#[cfg(feature = "cloud9_environment_membership")]
pub use cloud9_environment_membership::*;

#[cfg(feature = "cloudcontrolapi_resource")]
pub mod cloudcontrolapi_resource;

#[cfg(feature = "cloudcontrolapi_resource")]
pub use cloudcontrolapi_resource::*;

#[cfg(feature = "cloudformation_stack")]
pub mod cloudformation_stack;

#[cfg(feature = "cloudformation_stack")]
pub use cloudformation_stack::*;

#[cfg(feature = "cloudformation_stack_instances")]
pub mod cloudformation_stack_instances;

#[cfg(feature = "cloudformation_stack_instances")]
pub use cloudformation_stack_instances::*;

#[cfg(feature = "cloudformation_stack_set")]
pub mod cloudformation_stack_set;

#[cfg(feature = "cloudformation_stack_set")]
pub use cloudformation_stack_set::*;

#[cfg(feature = "cloudformation_stack_set_instance")]
pub mod cloudformation_stack_set_instance;

#[cfg(feature = "cloudformation_stack_set_instance")]
pub use cloudformation_stack_set_instance::*;

#[cfg(feature = "cloudformation_type")]
pub mod cloudformation_type;

#[cfg(feature = "cloudformation_type")]
pub use cloudformation_type::*;

#[cfg(feature = "cloudfront_cache_policy")]
pub mod cloudfront_cache_policy;

#[cfg(feature = "cloudfront_cache_policy")]
pub use cloudfront_cache_policy::*;

#[cfg(feature = "cloudfront_continuous_deployment_policy")]
pub mod cloudfront_continuous_deployment_policy;

#[cfg(feature = "cloudfront_continuous_deployment_policy")]
pub use cloudfront_continuous_deployment_policy::*;

#[cfg(feature = "cloudfront_distribution")]
pub mod cloudfront_distribution;

#[cfg(feature = "cloudfront_distribution")]
pub use cloudfront_distribution::*;

#[cfg(feature = "cloudfront_field_level_encryption_config")]
pub mod cloudfront_field_level_encryption_config;

#[cfg(feature = "cloudfront_field_level_encryption_config")]
pub use cloudfront_field_level_encryption_config::*;

#[cfg(feature = "cloudfront_field_level_encryption_profile")]
pub mod cloudfront_field_level_encryption_profile;

#[cfg(feature = "cloudfront_field_level_encryption_profile")]
pub use cloudfront_field_level_encryption_profile::*;

#[cfg(feature = "cloudfront_function")]
pub mod cloudfront_function;

#[cfg(feature = "cloudfront_function")]
pub use cloudfront_function::*;

#[cfg(feature = "cloudfront_key_group")]
pub mod cloudfront_key_group;

#[cfg(feature = "cloudfront_key_group")]
pub use cloudfront_key_group::*;

#[cfg(feature = "cloudfront_key_value_store")]
pub mod cloudfront_key_value_store;

#[cfg(feature = "cloudfront_key_value_store")]
pub use cloudfront_key_value_store::*;

#[cfg(feature = "cloudfront_monitoring_subscription")]
pub mod cloudfront_monitoring_subscription;

#[cfg(feature = "cloudfront_monitoring_subscription")]
pub use cloudfront_monitoring_subscription::*;

#[cfg(feature = "cloudfront_origin_access_control")]
pub mod cloudfront_origin_access_control;

#[cfg(feature = "cloudfront_origin_access_control")]
pub use cloudfront_origin_access_control::*;

#[cfg(feature = "cloudfront_origin_access_identity")]
pub mod cloudfront_origin_access_identity;

#[cfg(feature = "cloudfront_origin_access_identity")]
pub use cloudfront_origin_access_identity::*;

#[cfg(feature = "cloudfront_origin_request_policy")]
pub mod cloudfront_origin_request_policy;

#[cfg(feature = "cloudfront_origin_request_policy")]
pub use cloudfront_origin_request_policy::*;

#[cfg(feature = "cloudfront_public_key")]
pub mod cloudfront_public_key;

#[cfg(feature = "cloudfront_public_key")]
pub use cloudfront_public_key::*;

#[cfg(feature = "cloudfront_realtime_log_config")]
pub mod cloudfront_realtime_log_config;

#[cfg(feature = "cloudfront_realtime_log_config")]
pub use cloudfront_realtime_log_config::*;

#[cfg(feature = "cloudfront_response_headers_policy")]
pub mod cloudfront_response_headers_policy;

#[cfg(feature = "cloudfront_response_headers_policy")]
pub use cloudfront_response_headers_policy::*;

#[cfg(feature = "cloudfront_vpc_origin")]
pub mod cloudfront_vpc_origin;

#[cfg(feature = "cloudfront_vpc_origin")]
pub use cloudfront_vpc_origin::*;

#[cfg(feature = "cloudfrontkeyvaluestore_key")]
pub mod cloudfrontkeyvaluestore_key;

#[cfg(feature = "cloudfrontkeyvaluestore_key")]
pub use cloudfrontkeyvaluestore_key::*;

#[cfg(feature = "cloudfrontkeyvaluestore_keys_exclusive")]
pub mod cloudfrontkeyvaluestore_keys_exclusive;

#[cfg(feature = "cloudfrontkeyvaluestore_keys_exclusive")]
pub use cloudfrontkeyvaluestore_keys_exclusive::*;

#[cfg(feature = "cloudhsm_v2_cluster")]
pub mod cloudhsm_v2_cluster;

#[cfg(feature = "cloudhsm_v2_cluster")]
pub use cloudhsm_v2_cluster::*;

#[cfg(feature = "cloudhsm_v2_hsm")]
pub mod cloudhsm_v2_hsm;

#[cfg(feature = "cloudhsm_v2_hsm")]
pub use cloudhsm_v2_hsm::*;

#[cfg(feature = "cloudsearch_domain")]
pub mod cloudsearch_domain;

#[cfg(feature = "cloudsearch_domain")]
pub use cloudsearch_domain::*;

#[cfg(feature = "cloudsearch_domain_service_access_policy")]
pub mod cloudsearch_domain_service_access_policy;

#[cfg(feature = "cloudsearch_domain_service_access_policy")]
pub use cloudsearch_domain_service_access_policy::*;

#[cfg(feature = "cloudtrail")]
pub mod cloudtrail;

#[cfg(feature = "cloudtrail")]
pub use cloudtrail::*;

#[cfg(feature = "cloudtrail_event_data_store")]
pub mod cloudtrail_event_data_store;

#[cfg(feature = "cloudtrail_event_data_store")]
pub use cloudtrail_event_data_store::*;

#[cfg(feature = "cloudtrail_organization_delegated_admin_account")]
pub mod cloudtrail_organization_delegated_admin_account;

#[cfg(feature = "cloudtrail_organization_delegated_admin_account")]
pub use cloudtrail_organization_delegated_admin_account::*;

#[cfg(feature = "cloudwatch_composite_alarm")]
pub mod cloudwatch_composite_alarm;

#[cfg(feature = "cloudwatch_composite_alarm")]
pub use cloudwatch_composite_alarm::*;

#[cfg(feature = "cloudwatch_contributor_insight_rule")]
pub mod cloudwatch_contributor_insight_rule;

#[cfg(feature = "cloudwatch_contributor_insight_rule")]
pub use cloudwatch_contributor_insight_rule::*;

#[cfg(feature = "cloudwatch_contributor_managed_insight_rule")]
pub mod cloudwatch_contributor_managed_insight_rule;

#[cfg(feature = "cloudwatch_contributor_managed_insight_rule")]
pub use cloudwatch_contributor_managed_insight_rule::*;

#[cfg(feature = "cloudwatch_dashboard")]
pub mod cloudwatch_dashboard;

#[cfg(feature = "cloudwatch_dashboard")]
pub use cloudwatch_dashboard::*;

#[cfg(feature = "cloudwatch_event_api_destination")]
pub mod cloudwatch_event_api_destination;

#[cfg(feature = "cloudwatch_event_api_destination")]
pub use cloudwatch_event_api_destination::*;

#[cfg(feature = "cloudwatch_event_archive")]
pub mod cloudwatch_event_archive;

#[cfg(feature = "cloudwatch_event_archive")]
pub use cloudwatch_event_archive::*;

#[cfg(feature = "cloudwatch_event_bus")]
pub mod cloudwatch_event_bus;

#[cfg(feature = "cloudwatch_event_bus")]
pub use cloudwatch_event_bus::*;

#[cfg(feature = "cloudwatch_event_bus_policy")]
pub mod cloudwatch_event_bus_policy;

#[cfg(feature = "cloudwatch_event_bus_policy")]
pub use cloudwatch_event_bus_policy::*;

#[cfg(feature = "cloudwatch_event_connection")]
pub mod cloudwatch_event_connection;

#[cfg(feature = "cloudwatch_event_connection")]
pub use cloudwatch_event_connection::*;

#[cfg(feature = "cloudwatch_event_endpoint")]
pub mod cloudwatch_event_endpoint;

#[cfg(feature = "cloudwatch_event_endpoint")]
pub use cloudwatch_event_endpoint::*;

#[cfg(feature = "cloudwatch_event_permission")]
pub mod cloudwatch_event_permission;

#[cfg(feature = "cloudwatch_event_permission")]
pub use cloudwatch_event_permission::*;

#[cfg(feature = "cloudwatch_event_rule")]
pub mod cloudwatch_event_rule;

#[cfg(feature = "cloudwatch_event_rule")]
pub use cloudwatch_event_rule::*;

#[cfg(feature = "cloudwatch_event_target")]
pub mod cloudwatch_event_target;

#[cfg(feature = "cloudwatch_event_target")]
pub use cloudwatch_event_target::*;

#[cfg(feature = "cloudwatch_log_account_policy")]
pub mod cloudwatch_log_account_policy;

#[cfg(feature = "cloudwatch_log_account_policy")]
pub use cloudwatch_log_account_policy::*;

#[cfg(feature = "cloudwatch_log_anomaly_detector")]
pub mod cloudwatch_log_anomaly_detector;

#[cfg(feature = "cloudwatch_log_anomaly_detector")]
pub use cloudwatch_log_anomaly_detector::*;

#[cfg(feature = "cloudwatch_log_data_protection_policy")]
pub mod cloudwatch_log_data_protection_policy;

#[cfg(feature = "cloudwatch_log_data_protection_policy")]
pub use cloudwatch_log_data_protection_policy::*;

#[cfg(feature = "cloudwatch_log_delivery")]
pub mod cloudwatch_log_delivery;

#[cfg(feature = "cloudwatch_log_delivery")]
pub use cloudwatch_log_delivery::*;

#[cfg(feature = "cloudwatch_log_delivery_destination")]
pub mod cloudwatch_log_delivery_destination;

#[cfg(feature = "cloudwatch_log_delivery_destination")]
pub use cloudwatch_log_delivery_destination::*;

#[cfg(feature = "cloudwatch_log_delivery_destination_policy")]
pub mod cloudwatch_log_delivery_destination_policy;

#[cfg(feature = "cloudwatch_log_delivery_destination_policy")]
pub use cloudwatch_log_delivery_destination_policy::*;

#[cfg(feature = "cloudwatch_log_delivery_source")]
pub mod cloudwatch_log_delivery_source;

#[cfg(feature = "cloudwatch_log_delivery_source")]
pub use cloudwatch_log_delivery_source::*;

#[cfg(feature = "cloudwatch_log_destination")]
pub mod cloudwatch_log_destination;

#[cfg(feature = "cloudwatch_log_destination")]
pub use cloudwatch_log_destination::*;

#[cfg(feature = "cloudwatch_log_destination_policy")]
pub mod cloudwatch_log_destination_policy;

#[cfg(feature = "cloudwatch_log_destination_policy")]
pub use cloudwatch_log_destination_policy::*;

#[cfg(feature = "cloudwatch_log_group")]
pub mod cloudwatch_log_group;

#[cfg(feature = "cloudwatch_log_group")]
pub use cloudwatch_log_group::*;

#[cfg(feature = "cloudwatch_log_index_policy")]
pub mod cloudwatch_log_index_policy;

#[cfg(feature = "cloudwatch_log_index_policy")]
pub use cloudwatch_log_index_policy::*;

#[cfg(feature = "cloudwatch_log_metric_filter")]
pub mod cloudwatch_log_metric_filter;

#[cfg(feature = "cloudwatch_log_metric_filter")]
pub use cloudwatch_log_metric_filter::*;

#[cfg(feature = "cloudwatch_log_resource_policy")]
pub mod cloudwatch_log_resource_policy;

#[cfg(feature = "cloudwatch_log_resource_policy")]
pub use cloudwatch_log_resource_policy::*;

#[cfg(feature = "cloudwatch_log_stream")]
pub mod cloudwatch_log_stream;

#[cfg(feature = "cloudwatch_log_stream")]
pub use cloudwatch_log_stream::*;

#[cfg(feature = "cloudwatch_log_subscription_filter")]
pub mod cloudwatch_log_subscription_filter;

#[cfg(feature = "cloudwatch_log_subscription_filter")]
pub use cloudwatch_log_subscription_filter::*;

#[cfg(feature = "cloudwatch_metric_alarm")]
pub mod cloudwatch_metric_alarm;

#[cfg(feature = "cloudwatch_metric_alarm")]
pub use cloudwatch_metric_alarm::*;

#[cfg(feature = "cloudwatch_metric_stream")]
pub mod cloudwatch_metric_stream;

#[cfg(feature = "cloudwatch_metric_stream")]
pub use cloudwatch_metric_stream::*;

#[cfg(feature = "cloudwatch_query_definition")]
pub mod cloudwatch_query_definition;

#[cfg(feature = "cloudwatch_query_definition")]
pub use cloudwatch_query_definition::*;

#[cfg(feature = "codeartifact_domain")]
pub mod codeartifact_domain;

#[cfg(feature = "codeartifact_domain")]
pub use codeartifact_domain::*;

#[cfg(feature = "codeartifact_domain_permissions_policy")]
pub mod codeartifact_domain_permissions_policy;

#[cfg(feature = "codeartifact_domain_permissions_policy")]
pub use codeartifact_domain_permissions_policy::*;

#[cfg(feature = "codeartifact_repository")]
pub mod codeartifact_repository;

#[cfg(feature = "codeartifact_repository")]
pub use codeartifact_repository::*;

#[cfg(feature = "codeartifact_repository_permissions_policy")]
pub mod codeartifact_repository_permissions_policy;

#[cfg(feature = "codeartifact_repository_permissions_policy")]
pub use codeartifact_repository_permissions_policy::*;

#[cfg(feature = "codebuild_fleet")]
pub mod codebuild_fleet;

#[cfg(feature = "codebuild_fleet")]
pub use codebuild_fleet::*;

#[cfg(feature = "codebuild_project")]
pub mod codebuild_project;

#[cfg(feature = "codebuild_project")]
pub use codebuild_project::*;

#[cfg(feature = "codebuild_report_group")]
pub mod codebuild_report_group;

#[cfg(feature = "codebuild_report_group")]
pub use codebuild_report_group::*;

#[cfg(feature = "codebuild_resource_policy")]
pub mod codebuild_resource_policy;

#[cfg(feature = "codebuild_resource_policy")]
pub use codebuild_resource_policy::*;

#[cfg(feature = "codebuild_source_credential")]
pub mod codebuild_source_credential;

#[cfg(feature = "codebuild_source_credential")]
pub use codebuild_source_credential::*;

#[cfg(feature = "codebuild_webhook")]
pub mod codebuild_webhook;

#[cfg(feature = "codebuild_webhook")]
pub use codebuild_webhook::*;

#[cfg(feature = "codecatalyst_dev_environment")]
pub mod codecatalyst_dev_environment;

#[cfg(feature = "codecatalyst_dev_environment")]
pub use codecatalyst_dev_environment::*;

#[cfg(feature = "codecatalyst_project")]
pub mod codecatalyst_project;

#[cfg(feature = "codecatalyst_project")]
pub use codecatalyst_project::*;

#[cfg(feature = "codecatalyst_source_repository")]
pub mod codecatalyst_source_repository;

#[cfg(feature = "codecatalyst_source_repository")]
pub use codecatalyst_source_repository::*;

#[cfg(feature = "codecommit_approval_rule_template")]
pub mod codecommit_approval_rule_template;

#[cfg(feature = "codecommit_approval_rule_template")]
pub use codecommit_approval_rule_template::*;

#[cfg(feature = "codecommit_approval_rule_template_association")]
pub mod codecommit_approval_rule_template_association;

#[cfg(feature = "codecommit_approval_rule_template_association")]
pub use codecommit_approval_rule_template_association::*;

#[cfg(feature = "codecommit_repository")]
pub mod codecommit_repository;

#[cfg(feature = "codecommit_repository")]
pub use codecommit_repository::*;

#[cfg(feature = "codecommit_trigger")]
pub mod codecommit_trigger;

#[cfg(feature = "codecommit_trigger")]
pub use codecommit_trigger::*;

#[cfg(feature = "codeconnections_connection")]
pub mod codeconnections_connection;

#[cfg(feature = "codeconnections_connection")]
pub use codeconnections_connection::*;

#[cfg(feature = "codeconnections_host")]
pub mod codeconnections_host;

#[cfg(feature = "codeconnections_host")]
pub use codeconnections_host::*;

#[cfg(feature = "codedeploy_app")]
pub mod codedeploy_app;

#[cfg(feature = "codedeploy_app")]
pub use codedeploy_app::*;

#[cfg(feature = "codedeploy_deployment_config")]
pub mod codedeploy_deployment_config;

#[cfg(feature = "codedeploy_deployment_config")]
pub use codedeploy_deployment_config::*;

#[cfg(feature = "codedeploy_deployment_group")]
pub mod codedeploy_deployment_group;

#[cfg(feature = "codedeploy_deployment_group")]
pub use codedeploy_deployment_group::*;

#[cfg(feature = "codeguruprofiler_profiling_group")]
pub mod codeguruprofiler_profiling_group;

#[cfg(feature = "codeguruprofiler_profiling_group")]
pub use codeguruprofiler_profiling_group::*;

#[cfg(feature = "codegurureviewer_repository_association")]
pub mod codegurureviewer_repository_association;

#[cfg(feature = "codegurureviewer_repository_association")]
pub use codegurureviewer_repository_association::*;

#[cfg(feature = "codepipeline")]
pub mod codepipeline;

#[cfg(feature = "codepipeline")]
pub use codepipeline::*;

#[cfg(feature = "codepipeline_custom_action_type")]
pub mod codepipeline_custom_action_type;

#[cfg(feature = "codepipeline_custom_action_type")]
pub use codepipeline_custom_action_type::*;

#[cfg(feature = "codepipeline_webhook")]
pub mod codepipeline_webhook;

#[cfg(feature = "codepipeline_webhook")]
pub use codepipeline_webhook::*;

#[cfg(feature = "codestarconnections_connection")]
pub mod codestarconnections_connection;

#[cfg(feature = "codestarconnections_connection")]
pub use codestarconnections_connection::*;

#[cfg(feature = "codestarconnections_host")]
pub mod codestarconnections_host;

#[cfg(feature = "codestarconnections_host")]
pub use codestarconnections_host::*;

#[cfg(feature = "codestarnotifications_notification_rule")]
pub mod codestarnotifications_notification_rule;

#[cfg(feature = "codestarnotifications_notification_rule")]
pub use codestarnotifications_notification_rule::*;

#[cfg(feature = "cognito_identity_pool")]
pub mod cognito_identity_pool;

#[cfg(feature = "cognito_identity_pool")]
pub use cognito_identity_pool::*;

#[cfg(feature = "cognito_identity_pool_provider_principal_tag")]
pub mod cognito_identity_pool_provider_principal_tag;

#[cfg(feature = "cognito_identity_pool_provider_principal_tag")]
pub use cognito_identity_pool_provider_principal_tag::*;

#[cfg(feature = "cognito_identity_pool_roles_attachment")]
pub mod cognito_identity_pool_roles_attachment;

#[cfg(feature = "cognito_identity_pool_roles_attachment")]
pub use cognito_identity_pool_roles_attachment::*;

#[cfg(feature = "cognito_identity_provider")]
pub mod cognito_identity_provider;

#[cfg(feature = "cognito_identity_provider")]
pub use cognito_identity_provider::*;

#[cfg(feature = "cognito_log_delivery_configuration")]
pub mod cognito_log_delivery_configuration;

#[cfg(feature = "cognito_log_delivery_configuration")]
pub use cognito_log_delivery_configuration::*;

#[cfg(feature = "cognito_managed_login_branding")]
pub mod cognito_managed_login_branding;

#[cfg(feature = "cognito_managed_login_branding")]
pub use cognito_managed_login_branding::*;

#[cfg(feature = "cognito_managed_user_pool_client")]
pub mod cognito_managed_user_pool_client;

#[cfg(feature = "cognito_managed_user_pool_client")]
pub use cognito_managed_user_pool_client::*;

#[cfg(feature = "cognito_resource_server")]
pub mod cognito_resource_server;

#[cfg(feature = "cognito_resource_server")]
pub use cognito_resource_server::*;

#[cfg(feature = "cognito_risk_configuration")]
pub mod cognito_risk_configuration;

#[cfg(feature = "cognito_risk_configuration")]
pub use cognito_risk_configuration::*;

#[cfg(feature = "cognito_user")]
pub mod cognito_user;

#[cfg(feature = "cognito_user")]
pub use cognito_user::*;

#[cfg(feature = "cognito_user_group")]
pub mod cognito_user_group;

#[cfg(feature = "cognito_user_group")]
pub use cognito_user_group::*;

#[cfg(feature = "cognito_user_in_group")]
pub mod cognito_user_in_group;

#[cfg(feature = "cognito_user_in_group")]
pub use cognito_user_in_group::*;

#[cfg(feature = "cognito_user_pool")]
pub mod cognito_user_pool;

#[cfg(feature = "cognito_user_pool")]
pub use cognito_user_pool::*;

#[cfg(feature = "cognito_user_pool_client")]
pub mod cognito_user_pool_client;

#[cfg(feature = "cognito_user_pool_client")]
pub use cognito_user_pool_client::*;

#[cfg(feature = "cognito_user_pool_domain")]
pub mod cognito_user_pool_domain;

#[cfg(feature = "cognito_user_pool_domain")]
pub use cognito_user_pool_domain::*;

#[cfg(feature = "cognito_user_pool_ui_customization")]
pub mod cognito_user_pool_ui_customization;

#[cfg(feature = "cognito_user_pool_ui_customization")]
pub use cognito_user_pool_ui_customization::*;

#[cfg(feature = "comprehend_document_classifier")]
pub mod comprehend_document_classifier;

#[cfg(feature = "comprehend_document_classifier")]
pub use comprehend_document_classifier::*;

#[cfg(feature = "comprehend_entity_recognizer")]
pub mod comprehend_entity_recognizer;

#[cfg(feature = "comprehend_entity_recognizer")]
pub use comprehend_entity_recognizer::*;

#[cfg(feature = "computeoptimizer_enrollment_status")]
pub mod computeoptimizer_enrollment_status;

#[cfg(feature = "computeoptimizer_enrollment_status")]
pub use computeoptimizer_enrollment_status::*;

#[cfg(feature = "computeoptimizer_recommendation_preferences")]
pub mod computeoptimizer_recommendation_preferences;

#[cfg(feature = "computeoptimizer_recommendation_preferences")]
pub use computeoptimizer_recommendation_preferences::*;

#[cfg(feature = "config_aggregate_authorization")]
pub mod config_aggregate_authorization;

#[cfg(feature = "config_aggregate_authorization")]
pub use config_aggregate_authorization::*;

#[cfg(feature = "config_config_rule")]
pub mod config_config_rule;

#[cfg(feature = "config_config_rule")]
pub use config_config_rule::*;

#[cfg(feature = "config_configuration_aggregator")]
pub mod config_configuration_aggregator;

#[cfg(feature = "config_configuration_aggregator")]
pub use config_configuration_aggregator::*;

#[cfg(feature = "config_configuration_recorder")]
pub mod config_configuration_recorder;

#[cfg(feature = "config_configuration_recorder")]
pub use config_configuration_recorder::*;

#[cfg(feature = "config_configuration_recorder_status")]
pub mod config_configuration_recorder_status;

#[cfg(feature = "config_configuration_recorder_status")]
pub use config_configuration_recorder_status::*;

#[cfg(feature = "config_conformance_pack")]
pub mod config_conformance_pack;

#[cfg(feature = "config_conformance_pack")]
pub use config_conformance_pack::*;

#[cfg(feature = "config_delivery_channel")]
pub mod config_delivery_channel;

#[cfg(feature = "config_delivery_channel")]
pub use config_delivery_channel::*;

#[cfg(feature = "config_organization_conformance_pack")]
pub mod config_organization_conformance_pack;

#[cfg(feature = "config_organization_conformance_pack")]
pub use config_organization_conformance_pack::*;

#[cfg(feature = "config_organization_custom_policy_rule")]
pub mod config_organization_custom_policy_rule;

#[cfg(feature = "config_organization_custom_policy_rule")]
pub use config_organization_custom_policy_rule::*;

#[cfg(feature = "config_organization_custom_rule")]
pub mod config_organization_custom_rule;

#[cfg(feature = "config_organization_custom_rule")]
pub use config_organization_custom_rule::*;

#[cfg(feature = "config_organization_managed_rule")]
pub mod config_organization_managed_rule;

#[cfg(feature = "config_organization_managed_rule")]
pub use config_organization_managed_rule::*;

#[cfg(feature = "config_remediation_configuration")]
pub mod config_remediation_configuration;

#[cfg(feature = "config_remediation_configuration")]
pub use config_remediation_configuration::*;

#[cfg(feature = "config_retention_configuration")]
pub mod config_retention_configuration;

#[cfg(feature = "config_retention_configuration")]
pub use config_retention_configuration::*;

#[cfg(feature = "connect_bot_association")]
pub mod connect_bot_association;

#[cfg(feature = "connect_bot_association")]
pub use connect_bot_association::*;

#[cfg(feature = "connect_contact_flow")]
pub mod connect_contact_flow;

#[cfg(feature = "connect_contact_flow")]
pub use connect_contact_flow::*;

#[cfg(feature = "connect_contact_flow_module")]
pub mod connect_contact_flow_module;

#[cfg(feature = "connect_contact_flow_module")]
pub use connect_contact_flow_module::*;

#[cfg(feature = "connect_hours_of_operation")]
pub mod connect_hours_of_operation;

#[cfg(feature = "connect_hours_of_operation")]
pub use connect_hours_of_operation::*;

#[cfg(feature = "connect_instance")]
pub mod connect_instance;

#[cfg(feature = "connect_instance")]
pub use connect_instance::*;

#[cfg(feature = "connect_instance_storage_config")]
pub mod connect_instance_storage_config;

#[cfg(feature = "connect_instance_storage_config")]
pub use connect_instance_storage_config::*;

#[cfg(feature = "connect_lambda_function_association")]
pub mod connect_lambda_function_association;

#[cfg(feature = "connect_lambda_function_association")]
pub use connect_lambda_function_association::*;

#[cfg(feature = "connect_phone_number")]
pub mod connect_phone_number;

#[cfg(feature = "connect_phone_number")]
pub use connect_phone_number::*;

#[cfg(feature = "connect_phone_number_contact_flow_association")]
pub mod connect_phone_number_contact_flow_association;

#[cfg(feature = "connect_phone_number_contact_flow_association")]
pub use connect_phone_number_contact_flow_association::*;

#[cfg(feature = "connect_queue")]
pub mod connect_queue;

#[cfg(feature = "connect_queue")]
pub use connect_queue::*;

#[cfg(feature = "connect_quick_connect")]
pub mod connect_quick_connect;

#[cfg(feature = "connect_quick_connect")]
pub use connect_quick_connect::*;

#[cfg(feature = "connect_routing_profile")]
pub mod connect_routing_profile;

#[cfg(feature = "connect_routing_profile")]
pub use connect_routing_profile::*;

#[cfg(feature = "connect_security_profile")]
pub mod connect_security_profile;

#[cfg(feature = "connect_security_profile")]
pub use connect_security_profile::*;

#[cfg(feature = "connect_user")]
pub mod connect_user;

#[cfg(feature = "connect_user")]
pub use connect_user::*;

#[cfg(feature = "connect_user_hierarchy_group")]
pub mod connect_user_hierarchy_group;

#[cfg(feature = "connect_user_hierarchy_group")]
pub use connect_user_hierarchy_group::*;

#[cfg(feature = "connect_user_hierarchy_structure")]
pub mod connect_user_hierarchy_structure;

#[cfg(feature = "connect_user_hierarchy_structure")]
pub use connect_user_hierarchy_structure::*;

#[cfg(feature = "connect_vocabulary")]
pub mod connect_vocabulary;

#[cfg(feature = "connect_vocabulary")]
pub use connect_vocabulary::*;

#[cfg(feature = "controltower_baseline")]
pub mod controltower_baseline;

#[cfg(feature = "controltower_baseline")]
pub use controltower_baseline::*;

#[cfg(feature = "controltower_control")]
pub mod controltower_control;

#[cfg(feature = "controltower_control")]
pub use controltower_control::*;

#[cfg(feature = "controltower_landing_zone")]
pub mod controltower_landing_zone;

#[cfg(feature = "controltower_landing_zone")]
pub use controltower_landing_zone::*;

#[cfg(feature = "costoptimizationhub_enrollment_status")]
pub mod costoptimizationhub_enrollment_status;

#[cfg(feature = "costoptimizationhub_enrollment_status")]
pub use costoptimizationhub_enrollment_status::*;

#[cfg(feature = "costoptimizationhub_preferences")]
pub mod costoptimizationhub_preferences;

#[cfg(feature = "costoptimizationhub_preferences")]
pub use costoptimizationhub_preferences::*;

#[cfg(feature = "cur_report_definition")]
pub mod cur_report_definition;

#[cfg(feature = "cur_report_definition")]
pub use cur_report_definition::*;

#[cfg(feature = "customer_gateway")]
pub mod customer_gateway;

#[cfg(feature = "customer_gateway")]
pub use customer_gateway::*;

#[cfg(feature = "customerprofiles_domain")]
pub mod customerprofiles_domain;

#[cfg(feature = "customerprofiles_domain")]
pub use customerprofiles_domain::*;

#[cfg(feature = "customerprofiles_profile")]
pub mod customerprofiles_profile;

#[cfg(feature = "customerprofiles_profile")]
pub use customerprofiles_profile::*;

#[cfg(feature = "dataexchange_data_set")]
pub mod dataexchange_data_set;

#[cfg(feature = "dataexchange_data_set")]
pub use dataexchange_data_set::*;

#[cfg(feature = "dataexchange_event_action")]
pub mod dataexchange_event_action;

#[cfg(feature = "dataexchange_event_action")]
pub use dataexchange_event_action::*;

#[cfg(feature = "dataexchange_revision")]
pub mod dataexchange_revision;

#[cfg(feature = "dataexchange_revision")]
pub use dataexchange_revision::*;

#[cfg(feature = "dataexchange_revision_assets")]
pub mod dataexchange_revision_assets;

#[cfg(feature = "dataexchange_revision_assets")]
pub use dataexchange_revision_assets::*;

#[cfg(feature = "datapipeline_pipeline")]
pub mod datapipeline_pipeline;

#[cfg(feature = "datapipeline_pipeline")]
pub use datapipeline_pipeline::*;

#[cfg(feature = "datapipeline_pipeline_definition")]
pub mod datapipeline_pipeline_definition;

#[cfg(feature = "datapipeline_pipeline_definition")]
pub use datapipeline_pipeline_definition::*;

#[cfg(feature = "datasync_agent")]
pub mod datasync_agent;

#[cfg(feature = "datasync_agent")]
pub use datasync_agent::*;

#[cfg(feature = "datasync_location_azure_blob")]
pub mod datasync_location_azure_blob;

#[cfg(feature = "datasync_location_azure_blob")]
pub use datasync_location_azure_blob::*;

#[cfg(feature = "datasync_location_efs")]
pub mod datasync_location_efs;

#[cfg(feature = "datasync_location_efs")]
pub use datasync_location_efs::*;

#[cfg(feature = "datasync_location_fsx_lustre_file_system")]
pub mod datasync_location_fsx_lustre_file_system;

#[cfg(feature = "datasync_location_fsx_lustre_file_system")]
pub use datasync_location_fsx_lustre_file_system::*;

#[cfg(feature = "datasync_location_fsx_ontap_file_system")]
pub mod datasync_location_fsx_ontap_file_system;

#[cfg(feature = "datasync_location_fsx_ontap_file_system")]
pub use datasync_location_fsx_ontap_file_system::*;

#[cfg(feature = "datasync_location_fsx_openzfs_file_system")]
pub mod datasync_location_fsx_openzfs_file_system;

#[cfg(feature = "datasync_location_fsx_openzfs_file_system")]
pub use datasync_location_fsx_openzfs_file_system::*;

#[cfg(feature = "datasync_location_fsx_windows_file_system")]
pub mod datasync_location_fsx_windows_file_system;

#[cfg(feature = "datasync_location_fsx_windows_file_system")]
pub use datasync_location_fsx_windows_file_system::*;

#[cfg(feature = "datasync_location_hdfs")]
pub mod datasync_location_hdfs;

#[cfg(feature = "datasync_location_hdfs")]
pub use datasync_location_hdfs::*;

#[cfg(feature = "datasync_location_nfs")]
pub mod datasync_location_nfs;

#[cfg(feature = "datasync_location_nfs")]
pub use datasync_location_nfs::*;

#[cfg(feature = "datasync_location_object_storage")]
pub mod datasync_location_object_storage;

#[cfg(feature = "datasync_location_object_storage")]
pub use datasync_location_object_storage::*;

#[cfg(feature = "datasync_location_s3")]
pub mod datasync_location_s3;

#[cfg(feature = "datasync_location_s3")]
pub use datasync_location_s3::*;

#[cfg(feature = "datasync_location_smb")]
pub mod datasync_location_smb;

#[cfg(feature = "datasync_location_smb")]
pub use datasync_location_smb::*;

#[cfg(feature = "datasync_task")]
pub mod datasync_task;

#[cfg(feature = "datasync_task")]
pub use datasync_task::*;

#[cfg(feature = "dax_cluster")]
pub mod dax_cluster;

#[cfg(feature = "dax_cluster")]
pub use dax_cluster::*;

#[cfg(feature = "dax_parameter_group")]
pub mod dax_parameter_group;

#[cfg(feature = "dax_parameter_group")]
pub use dax_parameter_group::*;

#[cfg(feature = "dax_subnet_group")]
pub mod dax_subnet_group;

#[cfg(feature = "dax_subnet_group")]
pub use dax_subnet_group::*;

#[cfg(feature = "db_cluster_snapshot")]
pub mod db_cluster_snapshot;

#[cfg(feature = "db_cluster_snapshot")]
pub use db_cluster_snapshot::*;

#[cfg(feature = "db_event_subscription")]
pub mod db_event_subscription;

#[cfg(feature = "db_event_subscription")]
pub use db_event_subscription::*;

#[cfg(feature = "db_instance")]
pub mod db_instance;

#[cfg(feature = "db_instance")]
pub use db_instance::*;

#[cfg(feature = "db_instance_automated_backups_replication")]
pub mod db_instance_automated_backups_replication;

#[cfg(feature = "db_instance_automated_backups_replication")]
pub use db_instance_automated_backups_replication::*;

#[cfg(feature = "db_instance_role_association")]
pub mod db_instance_role_association;

#[cfg(feature = "db_instance_role_association")]
pub use db_instance_role_association::*;

#[cfg(feature = "db_option_group")]
pub mod db_option_group;

#[cfg(feature = "db_option_group")]
pub use db_option_group::*;

#[cfg(feature = "db_parameter_group")]
pub mod db_parameter_group;

#[cfg(feature = "db_parameter_group")]
pub use db_parameter_group::*;

#[cfg(feature = "db_proxy")]
pub mod db_proxy;

#[cfg(feature = "db_proxy")]
pub use db_proxy::*;

#[cfg(feature = "db_proxy_default_target_group")]
pub mod db_proxy_default_target_group;

#[cfg(feature = "db_proxy_default_target_group")]
pub use db_proxy_default_target_group::*;

#[cfg(feature = "db_proxy_endpoint")]
pub mod db_proxy_endpoint;

#[cfg(feature = "db_proxy_endpoint")]
pub use db_proxy_endpoint::*;

#[cfg(feature = "db_proxy_target")]
pub mod db_proxy_target;

#[cfg(feature = "db_proxy_target")]
pub use db_proxy_target::*;

#[cfg(feature = "db_snapshot")]
pub mod db_snapshot;

#[cfg(feature = "db_snapshot")]
pub use db_snapshot::*;

#[cfg(feature = "db_snapshot_copy")]
pub mod db_snapshot_copy;

#[cfg(feature = "db_snapshot_copy")]
pub use db_snapshot_copy::*;

#[cfg(feature = "db_subnet_group")]
pub mod db_subnet_group;

#[cfg(feature = "db_subnet_group")]
pub use db_subnet_group::*;

#[cfg(feature = "default_network_acl")]
pub mod default_network_acl;

#[cfg(feature = "default_network_acl")]
pub use default_network_acl::*;

#[cfg(feature = "default_route_table")]
pub mod default_route_table;

#[cfg(feature = "default_route_table")]
pub use default_route_table::*;

#[cfg(feature = "default_security_group")]
pub mod default_security_group;

#[cfg(feature = "default_security_group")]
pub use default_security_group::*;

#[cfg(feature = "default_subnet")]
pub mod default_subnet;

#[cfg(feature = "default_subnet")]
pub use default_subnet::*;

#[cfg(feature = "default_vpc")]
pub mod default_vpc;

#[cfg(feature = "default_vpc")]
pub use default_vpc::*;

#[cfg(feature = "default_vpc_dhcp_options")]
pub mod default_vpc_dhcp_options;

#[cfg(feature = "default_vpc_dhcp_options")]
pub use default_vpc_dhcp_options::*;

#[cfg(feature = "detective_graph")]
pub mod detective_graph;

#[cfg(feature = "detective_graph")]
pub use detective_graph::*;

#[cfg(feature = "detective_invitation_accepter")]
pub mod detective_invitation_accepter;

#[cfg(feature = "detective_invitation_accepter")]
pub use detective_invitation_accepter::*;

#[cfg(feature = "detective_member")]
pub mod detective_member;

#[cfg(feature = "detective_member")]
pub use detective_member::*;

#[cfg(feature = "detective_organization_admin_account")]
pub mod detective_organization_admin_account;

#[cfg(feature = "detective_organization_admin_account")]
pub use detective_organization_admin_account::*;

#[cfg(feature = "detective_organization_configuration")]
pub mod detective_organization_configuration;

#[cfg(feature = "detective_organization_configuration")]
pub use detective_organization_configuration::*;

#[cfg(feature = "devicefarm_device_pool")]
pub mod devicefarm_device_pool;

#[cfg(feature = "devicefarm_device_pool")]
pub use devicefarm_device_pool::*;

#[cfg(feature = "devicefarm_instance_profile")]
pub mod devicefarm_instance_profile;

#[cfg(feature = "devicefarm_instance_profile")]
pub use devicefarm_instance_profile::*;

#[cfg(feature = "devicefarm_network_profile")]
pub mod devicefarm_network_profile;

#[cfg(feature = "devicefarm_network_profile")]
pub use devicefarm_network_profile::*;

#[cfg(feature = "devicefarm_project")]
pub mod devicefarm_project;

#[cfg(feature = "devicefarm_project")]
pub use devicefarm_project::*;

#[cfg(feature = "devicefarm_test_grid_project")]
pub mod devicefarm_test_grid_project;

#[cfg(feature = "devicefarm_test_grid_project")]
pub use devicefarm_test_grid_project::*;

#[cfg(feature = "devicefarm_upload")]
pub mod devicefarm_upload;

#[cfg(feature = "devicefarm_upload")]
pub use devicefarm_upload::*;

#[cfg(feature = "devopsguru_event_sources_config")]
pub mod devopsguru_event_sources_config;

#[cfg(feature = "devopsguru_event_sources_config")]
pub use devopsguru_event_sources_config::*;

#[cfg(feature = "devopsguru_notification_channel")]
pub mod devopsguru_notification_channel;

#[cfg(feature = "devopsguru_notification_channel")]
pub use devopsguru_notification_channel::*;

#[cfg(feature = "devopsguru_resource_collection")]
pub mod devopsguru_resource_collection;

#[cfg(feature = "devopsguru_resource_collection")]
pub use devopsguru_resource_collection::*;

#[cfg(feature = "devopsguru_service_integration")]
pub mod devopsguru_service_integration;

#[cfg(feature = "devopsguru_service_integration")]
pub use devopsguru_service_integration::*;

#[cfg(feature = "directory_service_conditional_forwarder")]
pub mod directory_service_conditional_forwarder;

#[cfg(feature = "directory_service_conditional_forwarder")]
pub use directory_service_conditional_forwarder::*;

#[cfg(feature = "directory_service_directory")]
pub mod directory_service_directory;

#[cfg(feature = "directory_service_directory")]
pub use directory_service_directory::*;

#[cfg(feature = "directory_service_log_subscription")]
pub mod directory_service_log_subscription;

#[cfg(feature = "directory_service_log_subscription")]
pub use directory_service_log_subscription::*;

#[cfg(feature = "directory_service_radius_settings")]
pub mod directory_service_radius_settings;

#[cfg(feature = "directory_service_radius_settings")]
pub use directory_service_radius_settings::*;

#[cfg(feature = "directory_service_region")]
pub mod directory_service_region;

#[cfg(feature = "directory_service_region")]
pub use directory_service_region::*;

#[cfg(feature = "directory_service_shared_directory")]
pub mod directory_service_shared_directory;

#[cfg(feature = "directory_service_shared_directory")]
pub use directory_service_shared_directory::*;

#[cfg(feature = "directory_service_shared_directory_accepter")]
pub mod directory_service_shared_directory_accepter;

#[cfg(feature = "directory_service_shared_directory_accepter")]
pub use directory_service_shared_directory_accepter::*;

#[cfg(feature = "directory_service_trust")]
pub mod directory_service_trust;

#[cfg(feature = "directory_service_trust")]
pub use directory_service_trust::*;

#[cfg(feature = "dlm_lifecycle_policy")]
pub mod dlm_lifecycle_policy;

#[cfg(feature = "dlm_lifecycle_policy")]
pub use dlm_lifecycle_policy::*;

#[cfg(feature = "dms_certificate")]
pub mod dms_certificate;

#[cfg(feature = "dms_certificate")]
pub use dms_certificate::*;

#[cfg(feature = "dms_endpoint")]
pub mod dms_endpoint;

#[cfg(feature = "dms_endpoint")]
pub use dms_endpoint::*;

#[cfg(feature = "dms_event_subscription")]
pub mod dms_event_subscription;

#[cfg(feature = "dms_event_subscription")]
pub use dms_event_subscription::*;

#[cfg(feature = "dms_replication_config")]
pub mod dms_replication_config;

#[cfg(feature = "dms_replication_config")]
pub use dms_replication_config::*;

#[cfg(feature = "dms_replication_instance")]
pub mod dms_replication_instance;

#[cfg(feature = "dms_replication_instance")]
pub use dms_replication_instance::*;

#[cfg(feature = "dms_replication_subnet_group")]
pub mod dms_replication_subnet_group;

#[cfg(feature = "dms_replication_subnet_group")]
pub use dms_replication_subnet_group::*;

#[cfg(feature = "dms_replication_task")]
pub mod dms_replication_task;

#[cfg(feature = "dms_replication_task")]
pub use dms_replication_task::*;

#[cfg(feature = "dms_s3_endpoint")]
pub mod dms_s3_endpoint;

#[cfg(feature = "dms_s3_endpoint")]
pub use dms_s3_endpoint::*;

#[cfg(feature = "docdb_cluster")]
pub mod docdb_cluster;

#[cfg(feature = "docdb_cluster")]
pub use docdb_cluster::*;

#[cfg(feature = "docdb_cluster_instance")]
pub mod docdb_cluster_instance;

#[cfg(feature = "docdb_cluster_instance")]
pub use docdb_cluster_instance::*;

#[cfg(feature = "docdb_cluster_parameter_group")]
pub mod docdb_cluster_parameter_group;

#[cfg(feature = "docdb_cluster_parameter_group")]
pub use docdb_cluster_parameter_group::*;

#[cfg(feature = "docdb_cluster_snapshot")]
pub mod docdb_cluster_snapshot;

#[cfg(feature = "docdb_cluster_snapshot")]
pub use docdb_cluster_snapshot::*;

#[cfg(feature = "docdb_event_subscription")]
pub mod docdb_event_subscription;

#[cfg(feature = "docdb_event_subscription")]
pub use docdb_event_subscription::*;

#[cfg(feature = "docdb_global_cluster")]
pub mod docdb_global_cluster;

#[cfg(feature = "docdb_global_cluster")]
pub use docdb_global_cluster::*;

#[cfg(feature = "docdb_subnet_group")]
pub mod docdb_subnet_group;

#[cfg(feature = "docdb_subnet_group")]
pub use docdb_subnet_group::*;

#[cfg(feature = "docdbelastic_cluster")]
pub mod docdbelastic_cluster;

#[cfg(feature = "docdbelastic_cluster")]
pub use docdbelastic_cluster::*;

#[cfg(feature = "drs_replication_configuration_template")]
pub mod drs_replication_configuration_template;

#[cfg(feature = "drs_replication_configuration_template")]
pub use drs_replication_configuration_template::*;

#[cfg(feature = "dsql_cluster")]
pub mod dsql_cluster;

#[cfg(feature = "dsql_cluster")]
pub use dsql_cluster::*;

#[cfg(feature = "dsql_cluster_peering")]
pub mod dsql_cluster_peering;

#[cfg(feature = "dsql_cluster_peering")]
pub use dsql_cluster_peering::*;

#[cfg(feature = "dx_bgp_peer")]
pub mod dx_bgp_peer;

#[cfg(feature = "dx_bgp_peer")]
pub use dx_bgp_peer::*;

#[cfg(feature = "dx_connection")]
pub mod dx_connection;

#[cfg(feature = "dx_connection")]
pub use dx_connection::*;

#[cfg(feature = "dx_connection_association")]
pub mod dx_connection_association;

#[cfg(feature = "dx_connection_association")]
pub use dx_connection_association::*;

#[cfg(feature = "dx_connection_confirmation")]
pub mod dx_connection_confirmation;

#[cfg(feature = "dx_connection_confirmation")]
pub use dx_connection_confirmation::*;

#[cfg(feature = "dx_gateway")]
pub mod dx_gateway;

#[cfg(feature = "dx_gateway")]
pub use dx_gateway::*;

#[cfg(feature = "dx_gateway_association")]
pub mod dx_gateway_association;

#[cfg(feature = "dx_gateway_association")]
pub use dx_gateway_association::*;

#[cfg(feature = "dx_gateway_association_proposal")]
pub mod dx_gateway_association_proposal;

#[cfg(feature = "dx_gateway_association_proposal")]
pub use dx_gateway_association_proposal::*;

#[cfg(feature = "dx_hosted_connection")]
pub mod dx_hosted_connection;

#[cfg(feature = "dx_hosted_connection")]
pub use dx_hosted_connection::*;

#[cfg(feature = "dx_hosted_private_virtual_interface")]
pub mod dx_hosted_private_virtual_interface;

#[cfg(feature = "dx_hosted_private_virtual_interface")]
pub use dx_hosted_private_virtual_interface::*;

#[cfg(feature = "dx_hosted_private_virtual_interface_accepter")]
pub mod dx_hosted_private_virtual_interface_accepter;

#[cfg(feature = "dx_hosted_private_virtual_interface_accepter")]
pub use dx_hosted_private_virtual_interface_accepter::*;

#[cfg(feature = "dx_hosted_public_virtual_interface")]
pub mod dx_hosted_public_virtual_interface;

#[cfg(feature = "dx_hosted_public_virtual_interface")]
pub use dx_hosted_public_virtual_interface::*;

#[cfg(feature = "dx_hosted_public_virtual_interface_accepter")]
pub mod dx_hosted_public_virtual_interface_accepter;

#[cfg(feature = "dx_hosted_public_virtual_interface_accepter")]
pub use dx_hosted_public_virtual_interface_accepter::*;

#[cfg(feature = "dx_hosted_transit_virtual_interface")]
pub mod dx_hosted_transit_virtual_interface;

#[cfg(feature = "dx_hosted_transit_virtual_interface")]
pub use dx_hosted_transit_virtual_interface::*;

#[cfg(feature = "dx_hosted_transit_virtual_interface_accepter")]
pub mod dx_hosted_transit_virtual_interface_accepter;

#[cfg(feature = "dx_hosted_transit_virtual_interface_accepter")]
pub use dx_hosted_transit_virtual_interface_accepter::*;

#[cfg(feature = "dx_lag")]
pub mod dx_lag;

#[cfg(feature = "dx_lag")]
pub use dx_lag::*;

#[cfg(feature = "dx_macsec_key_association")]
pub mod dx_macsec_key_association;

#[cfg(feature = "dx_macsec_key_association")]
pub use dx_macsec_key_association::*;

#[cfg(feature = "dx_private_virtual_interface")]
pub mod dx_private_virtual_interface;

#[cfg(feature = "dx_private_virtual_interface")]
pub use dx_private_virtual_interface::*;

#[cfg(feature = "dx_public_virtual_interface")]
pub mod dx_public_virtual_interface;

#[cfg(feature = "dx_public_virtual_interface")]
pub use dx_public_virtual_interface::*;

#[cfg(feature = "dx_transit_virtual_interface")]
pub mod dx_transit_virtual_interface;

#[cfg(feature = "dx_transit_virtual_interface")]
pub use dx_transit_virtual_interface::*;

#[cfg(feature = "dynamodb_contributor_insights")]
pub mod dynamodb_contributor_insights;

#[cfg(feature = "dynamodb_contributor_insights")]
pub use dynamodb_contributor_insights::*;

#[cfg(feature = "dynamodb_global_table")]
pub mod dynamodb_global_table;

#[cfg(feature = "dynamodb_global_table")]
pub use dynamodb_global_table::*;

#[cfg(feature = "dynamodb_kinesis_streaming_destination")]
pub mod dynamodb_kinesis_streaming_destination;

#[cfg(feature = "dynamodb_kinesis_streaming_destination")]
pub use dynamodb_kinesis_streaming_destination::*;

#[cfg(feature = "dynamodb_resource_policy")]
pub mod dynamodb_resource_policy;

#[cfg(feature = "dynamodb_resource_policy")]
pub use dynamodb_resource_policy::*;

#[cfg(feature = "dynamodb_table")]
pub mod dynamodb_table;

#[cfg(feature = "dynamodb_table")]
pub use dynamodb_table::*;

#[cfg(feature = "dynamodb_table_export")]
pub mod dynamodb_table_export;

#[cfg(feature = "dynamodb_table_export")]
pub use dynamodb_table_export::*;

#[cfg(feature = "dynamodb_table_item")]
pub mod dynamodb_table_item;

#[cfg(feature = "dynamodb_table_item")]
pub use dynamodb_table_item::*;

#[cfg(feature = "dynamodb_table_replica")]
pub mod dynamodb_table_replica;

#[cfg(feature = "dynamodb_table_replica")]
pub use dynamodb_table_replica::*;

#[cfg(feature = "dynamodb_tag")]
pub mod dynamodb_tag;

#[cfg(feature = "dynamodb_tag")]
pub use dynamodb_tag::*;

#[cfg(feature = "ebs_default_kms_key")]
pub mod ebs_default_kms_key;

#[cfg(feature = "ebs_default_kms_key")]
pub use ebs_default_kms_key::*;

#[cfg(feature = "ebs_encryption_by_default")]
pub mod ebs_encryption_by_default;

#[cfg(feature = "ebs_encryption_by_default")]
pub use ebs_encryption_by_default::*;

#[cfg(feature = "ebs_fast_snapshot_restore")]
pub mod ebs_fast_snapshot_restore;

#[cfg(feature = "ebs_fast_snapshot_restore")]
pub use ebs_fast_snapshot_restore::*;

#[cfg(feature = "ebs_snapshot")]
pub mod ebs_snapshot;

#[cfg(feature = "ebs_snapshot")]
pub use ebs_snapshot::*;

#[cfg(feature = "ebs_snapshot_block_public_access")]
pub mod ebs_snapshot_block_public_access;

#[cfg(feature = "ebs_snapshot_block_public_access")]
pub use ebs_snapshot_block_public_access::*;

#[cfg(feature = "ebs_snapshot_copy")]
pub mod ebs_snapshot_copy;

#[cfg(feature = "ebs_snapshot_copy")]
pub use ebs_snapshot_copy::*;

#[cfg(feature = "ebs_snapshot_import")]
pub mod ebs_snapshot_import;

#[cfg(feature = "ebs_snapshot_import")]
pub use ebs_snapshot_import::*;

#[cfg(feature = "ebs_volume")]
pub mod ebs_volume;

#[cfg(feature = "ebs_volume")]
pub use ebs_volume::*;

#[cfg(feature = "ec2_allowed_images_settings")]
pub mod ec2_allowed_images_settings;

#[cfg(feature = "ec2_allowed_images_settings")]
pub use ec2_allowed_images_settings::*;

#[cfg(feature = "ec2_availability_zone_group")]
pub mod ec2_availability_zone_group;

#[cfg(feature = "ec2_availability_zone_group")]
pub use ec2_availability_zone_group::*;

#[cfg(feature = "ec2_capacity_block_reservation")]
pub mod ec2_capacity_block_reservation;

#[cfg(feature = "ec2_capacity_block_reservation")]
pub use ec2_capacity_block_reservation::*;

#[cfg(feature = "ec2_capacity_reservation")]
pub mod ec2_capacity_reservation;

#[cfg(feature = "ec2_capacity_reservation")]
pub use ec2_capacity_reservation::*;

#[cfg(feature = "ec2_carrier_gateway")]
pub mod ec2_carrier_gateway;

#[cfg(feature = "ec2_carrier_gateway")]
pub use ec2_carrier_gateway::*;

#[cfg(feature = "ec2_client_vpn_authorization_rule")]
pub mod ec2_client_vpn_authorization_rule;

#[cfg(feature = "ec2_client_vpn_authorization_rule")]
pub use ec2_client_vpn_authorization_rule::*;

#[cfg(feature = "ec2_client_vpn_endpoint")]
pub mod ec2_client_vpn_endpoint;

#[cfg(feature = "ec2_client_vpn_endpoint")]
pub use ec2_client_vpn_endpoint::*;

#[cfg(feature = "ec2_client_vpn_network_association")]
pub mod ec2_client_vpn_network_association;

#[cfg(feature = "ec2_client_vpn_network_association")]
pub use ec2_client_vpn_network_association::*;

#[cfg(feature = "ec2_client_vpn_route")]
pub mod ec2_client_vpn_route;

#[cfg(feature = "ec2_client_vpn_route")]
pub use ec2_client_vpn_route::*;

#[cfg(feature = "ec2_default_credit_specification")]
pub mod ec2_default_credit_specification;

#[cfg(feature = "ec2_default_credit_specification")]
pub use ec2_default_credit_specification::*;

#[cfg(feature = "ec2_fleet")]
pub mod ec2_fleet;

#[cfg(feature = "ec2_fleet")]
pub use ec2_fleet::*;

#[cfg(feature = "ec2_host")]
pub mod ec2_host;

#[cfg(feature = "ec2_host")]
pub use ec2_host::*;

#[cfg(feature = "ec2_image_block_public_access")]
pub mod ec2_image_block_public_access;

#[cfg(feature = "ec2_image_block_public_access")]
pub use ec2_image_block_public_access::*;

#[cfg(feature = "ec2_instance_connect_endpoint")]
pub mod ec2_instance_connect_endpoint;

#[cfg(feature = "ec2_instance_connect_endpoint")]
pub use ec2_instance_connect_endpoint::*;

#[cfg(feature = "ec2_instance_metadata_defaults")]
pub mod ec2_instance_metadata_defaults;

#[cfg(feature = "ec2_instance_metadata_defaults")]
pub use ec2_instance_metadata_defaults::*;

#[cfg(feature = "ec2_instance_state")]
pub mod ec2_instance_state;

#[cfg(feature = "ec2_instance_state")]
pub use ec2_instance_state::*;

#[cfg(feature = "ec2_local_gateway_route")]
pub mod ec2_local_gateway_route;

#[cfg(feature = "ec2_local_gateway_route")]
pub use ec2_local_gateway_route::*;

#[cfg(feature = "ec2_local_gateway_route_table_vpc_association")]
pub mod ec2_local_gateway_route_table_vpc_association;

#[cfg(feature = "ec2_local_gateway_route_table_vpc_association")]
pub use ec2_local_gateway_route_table_vpc_association::*;

#[cfg(feature = "ec2_managed_prefix_list")]
pub mod ec2_managed_prefix_list;

#[cfg(feature = "ec2_managed_prefix_list")]
pub use ec2_managed_prefix_list::*;

#[cfg(feature = "ec2_managed_prefix_list_entry")]
pub mod ec2_managed_prefix_list_entry;

#[cfg(feature = "ec2_managed_prefix_list_entry")]
pub use ec2_managed_prefix_list_entry::*;

#[cfg(feature = "ec2_network_insights_analysis")]
pub mod ec2_network_insights_analysis;

#[cfg(feature = "ec2_network_insights_analysis")]
pub use ec2_network_insights_analysis::*;

#[cfg(feature = "ec2_network_insights_path")]
pub mod ec2_network_insights_path;

#[cfg(feature = "ec2_network_insights_path")]
pub use ec2_network_insights_path::*;

#[cfg(feature = "ec2_serial_console_access")]
pub mod ec2_serial_console_access;

#[cfg(feature = "ec2_serial_console_access")]
pub use ec2_serial_console_access::*;

#[cfg(feature = "ec2_subnet_cidr_reservation")]
pub mod ec2_subnet_cidr_reservation;

#[cfg(feature = "ec2_subnet_cidr_reservation")]
pub use ec2_subnet_cidr_reservation::*;

#[cfg(feature = "ec2_tag")]
pub mod ec2_tag;

#[cfg(feature = "ec2_tag")]
pub use ec2_tag::*;

#[cfg(feature = "ec2_traffic_mirror_filter")]
pub mod ec2_traffic_mirror_filter;

#[cfg(feature = "ec2_traffic_mirror_filter")]
pub use ec2_traffic_mirror_filter::*;

#[cfg(feature = "ec2_traffic_mirror_filter_rule")]
pub mod ec2_traffic_mirror_filter_rule;

#[cfg(feature = "ec2_traffic_mirror_filter_rule")]
pub use ec2_traffic_mirror_filter_rule::*;

#[cfg(feature = "ec2_traffic_mirror_session")]
pub mod ec2_traffic_mirror_session;

#[cfg(feature = "ec2_traffic_mirror_session")]
pub use ec2_traffic_mirror_session::*;

#[cfg(feature = "ec2_traffic_mirror_target")]
pub mod ec2_traffic_mirror_target;

#[cfg(feature = "ec2_traffic_mirror_target")]
pub use ec2_traffic_mirror_target::*;

#[cfg(feature = "ec2_transit_gateway")]
pub mod ec2_transit_gateway;

#[cfg(feature = "ec2_transit_gateway")]
pub use ec2_transit_gateway::*;

#[cfg(feature = "ec2_transit_gateway_connect")]
pub mod ec2_transit_gateway_connect;

#[cfg(feature = "ec2_transit_gateway_connect")]
pub use ec2_transit_gateway_connect::*;

#[cfg(feature = "ec2_transit_gateway_connect_peer")]
pub mod ec2_transit_gateway_connect_peer;

#[cfg(feature = "ec2_transit_gateway_connect_peer")]
pub use ec2_transit_gateway_connect_peer::*;

#[cfg(feature = "ec2_transit_gateway_default_route_table_association")]
pub mod ec2_transit_gateway_default_route_table_association;

#[cfg(feature = "ec2_transit_gateway_default_route_table_association")]
pub use ec2_transit_gateway_default_route_table_association::*;

#[cfg(feature = "ec2_transit_gateway_default_route_table_propagation")]
pub mod ec2_transit_gateway_default_route_table_propagation;

#[cfg(feature = "ec2_transit_gateway_default_route_table_propagation")]
pub use ec2_transit_gateway_default_route_table_propagation::*;

#[cfg(feature = "ec2_transit_gateway_multicast_domain")]
pub mod ec2_transit_gateway_multicast_domain;

#[cfg(feature = "ec2_transit_gateway_multicast_domain")]
pub use ec2_transit_gateway_multicast_domain::*;

#[cfg(feature = "ec2_transit_gateway_multicast_domain_association")]
pub mod ec2_transit_gateway_multicast_domain_association;

#[cfg(feature = "ec2_transit_gateway_multicast_domain_association")]
pub use ec2_transit_gateway_multicast_domain_association::*;

#[cfg(feature = "ec2_transit_gateway_multicast_group_member")]
pub mod ec2_transit_gateway_multicast_group_member;

#[cfg(feature = "ec2_transit_gateway_multicast_group_member")]
pub use ec2_transit_gateway_multicast_group_member::*;

#[cfg(feature = "ec2_transit_gateway_multicast_group_source")]
pub mod ec2_transit_gateway_multicast_group_source;

#[cfg(feature = "ec2_transit_gateway_multicast_group_source")]
pub use ec2_transit_gateway_multicast_group_source::*;

#[cfg(feature = "ec2_transit_gateway_peering_attachment")]
pub mod ec2_transit_gateway_peering_attachment;

#[cfg(feature = "ec2_transit_gateway_peering_attachment")]
pub use ec2_transit_gateway_peering_attachment::*;

#[cfg(feature = "ec2_transit_gateway_peering_attachment_accepter")]
pub mod ec2_transit_gateway_peering_attachment_accepter;

#[cfg(feature = "ec2_transit_gateway_peering_attachment_accepter")]
pub use ec2_transit_gateway_peering_attachment_accepter::*;

#[cfg(feature = "ec2_transit_gateway_policy_table")]
pub mod ec2_transit_gateway_policy_table;

#[cfg(feature = "ec2_transit_gateway_policy_table")]
pub use ec2_transit_gateway_policy_table::*;

#[cfg(feature = "ec2_transit_gateway_policy_table_association")]
pub mod ec2_transit_gateway_policy_table_association;

#[cfg(feature = "ec2_transit_gateway_policy_table_association")]
pub use ec2_transit_gateway_policy_table_association::*;

#[cfg(feature = "ec2_transit_gateway_prefix_list_reference")]
pub mod ec2_transit_gateway_prefix_list_reference;

#[cfg(feature = "ec2_transit_gateway_prefix_list_reference")]
pub use ec2_transit_gateway_prefix_list_reference::*;

#[cfg(feature = "ec2_transit_gateway_route")]
pub mod ec2_transit_gateway_route;

#[cfg(feature = "ec2_transit_gateway_route")]
pub use ec2_transit_gateway_route::*;

#[cfg(feature = "ec2_transit_gateway_route_table")]
pub mod ec2_transit_gateway_route_table;

#[cfg(feature = "ec2_transit_gateway_route_table")]
pub use ec2_transit_gateway_route_table::*;

#[cfg(feature = "ec2_transit_gateway_route_table_association")]
pub mod ec2_transit_gateway_route_table_association;

#[cfg(feature = "ec2_transit_gateway_route_table_association")]
pub use ec2_transit_gateway_route_table_association::*;

#[cfg(feature = "ec2_transit_gateway_route_table_propagation")]
pub mod ec2_transit_gateway_route_table_propagation;

#[cfg(feature = "ec2_transit_gateway_route_table_propagation")]
pub use ec2_transit_gateway_route_table_propagation::*;

#[cfg(feature = "ec2_transit_gateway_vpc_attachment")]
pub mod ec2_transit_gateway_vpc_attachment;

#[cfg(feature = "ec2_transit_gateway_vpc_attachment")]
pub use ec2_transit_gateway_vpc_attachment::*;

#[cfg(feature = "ec2_transit_gateway_vpc_attachment_accepter")]
pub mod ec2_transit_gateway_vpc_attachment_accepter;

#[cfg(feature = "ec2_transit_gateway_vpc_attachment_accepter")]
pub use ec2_transit_gateway_vpc_attachment_accepter::*;

#[cfg(feature = "ecr_account_setting")]
pub mod ecr_account_setting;

#[cfg(feature = "ecr_account_setting")]
pub use ecr_account_setting::*;

#[cfg(feature = "ecr_lifecycle_policy")]
pub mod ecr_lifecycle_policy;

#[cfg(feature = "ecr_lifecycle_policy")]
pub use ecr_lifecycle_policy::*;

#[cfg(feature = "ecr_pull_through_cache_rule")]
pub mod ecr_pull_through_cache_rule;

#[cfg(feature = "ecr_pull_through_cache_rule")]
pub use ecr_pull_through_cache_rule::*;

#[cfg(feature = "ecr_registry_policy")]
pub mod ecr_registry_policy;

#[cfg(feature = "ecr_registry_policy")]
pub use ecr_registry_policy::*;

#[cfg(feature = "ecr_registry_scanning_configuration")]
pub mod ecr_registry_scanning_configuration;

#[cfg(feature = "ecr_registry_scanning_configuration")]
pub use ecr_registry_scanning_configuration::*;

#[cfg(feature = "ecr_replication_configuration")]
pub mod ecr_replication_configuration;

#[cfg(feature = "ecr_replication_configuration")]
pub use ecr_replication_configuration::*;

#[cfg(feature = "ecr_repository")]
pub mod ecr_repository;

#[cfg(feature = "ecr_repository")]
pub use ecr_repository::*;

#[cfg(feature = "ecr_repository_creation_template")]
pub mod ecr_repository_creation_template;

#[cfg(feature = "ecr_repository_creation_template")]
pub use ecr_repository_creation_template::*;

#[cfg(feature = "ecr_repository_policy")]
pub mod ecr_repository_policy;

#[cfg(feature = "ecr_repository_policy")]
pub use ecr_repository_policy::*;

#[cfg(feature = "ecrpublic_repository")]
pub mod ecrpublic_repository;

#[cfg(feature = "ecrpublic_repository")]
pub use ecrpublic_repository::*;

#[cfg(feature = "ecrpublic_repository_policy")]
pub mod ecrpublic_repository_policy;

#[cfg(feature = "ecrpublic_repository_policy")]
pub use ecrpublic_repository_policy::*;

#[cfg(feature = "ecs_account_setting_default")]
pub mod ecs_account_setting_default;

#[cfg(feature = "ecs_account_setting_default")]
pub use ecs_account_setting_default::*;

#[cfg(feature = "ecs_capacity_provider")]
pub mod ecs_capacity_provider;

#[cfg(feature = "ecs_capacity_provider")]
pub use ecs_capacity_provider::*;

#[cfg(feature = "ecs_cluster")]
pub mod ecs_cluster;

#[cfg(feature = "ecs_cluster")]
pub use ecs_cluster::*;

#[cfg(feature = "ecs_cluster_capacity_providers")]
pub mod ecs_cluster_capacity_providers;

#[cfg(feature = "ecs_cluster_capacity_providers")]
pub use ecs_cluster_capacity_providers::*;

#[cfg(feature = "ecs_service")]
pub mod ecs_service;

#[cfg(feature = "ecs_service")]
pub use ecs_service::*;

#[cfg(feature = "ecs_tag")]
pub mod ecs_tag;

#[cfg(feature = "ecs_tag")]
pub use ecs_tag::*;

#[cfg(feature = "ecs_task_definition")]
pub mod ecs_task_definition;

#[cfg(feature = "ecs_task_definition")]
pub use ecs_task_definition::*;

#[cfg(feature = "ecs_task_set")]
pub mod ecs_task_set;

#[cfg(feature = "ecs_task_set")]
pub use ecs_task_set::*;

#[cfg(feature = "efs_access_point")]
pub mod efs_access_point;

#[cfg(feature = "efs_access_point")]
pub use efs_access_point::*;

#[cfg(feature = "efs_backup_policy")]
pub mod efs_backup_policy;

#[cfg(feature = "efs_backup_policy")]
pub use efs_backup_policy::*;

#[cfg(feature = "efs_file_system")]
pub mod efs_file_system;

#[cfg(feature = "efs_file_system")]
pub use efs_file_system::*;

#[cfg(feature = "efs_file_system_policy")]
pub mod efs_file_system_policy;

#[cfg(feature = "efs_file_system_policy")]
pub use efs_file_system_policy::*;

#[cfg(feature = "efs_mount_target")]
pub mod efs_mount_target;

#[cfg(feature = "efs_mount_target")]
pub use efs_mount_target::*;

#[cfg(feature = "efs_replication_configuration")]
pub mod efs_replication_configuration;

#[cfg(feature = "efs_replication_configuration")]
pub use efs_replication_configuration::*;

#[cfg(feature = "egress_only_internet_gateway")]
pub mod egress_only_internet_gateway;

#[cfg(feature = "egress_only_internet_gateway")]
pub use egress_only_internet_gateway::*;

#[cfg(feature = "eip")]
pub mod eip;

#[cfg(feature = "eip")]
pub use eip::*;

#[cfg(feature = "eip_association")]
pub mod eip_association;

#[cfg(feature = "eip_association")]
pub use eip_association::*;

#[cfg(feature = "eip_domain_name")]
pub mod eip_domain_name;

#[cfg(feature = "eip_domain_name")]
pub use eip_domain_name::*;

#[cfg(feature = "eks_access_entry")]
pub mod eks_access_entry;

#[cfg(feature = "eks_access_entry")]
pub use eks_access_entry::*;

#[cfg(feature = "eks_access_policy_association")]
pub mod eks_access_policy_association;

#[cfg(feature = "eks_access_policy_association")]
pub use eks_access_policy_association::*;

#[cfg(feature = "eks_addon")]
pub mod eks_addon;

#[cfg(feature = "eks_addon")]
pub use eks_addon::*;

#[cfg(feature = "eks_cluster")]
pub mod eks_cluster;

#[cfg(feature = "eks_cluster")]
pub use eks_cluster::*;

#[cfg(feature = "eks_fargate_profile")]
pub mod eks_fargate_profile;

#[cfg(feature = "eks_fargate_profile")]
pub use eks_fargate_profile::*;

#[cfg(feature = "eks_identity_provider_config")]
pub mod eks_identity_provider_config;

#[cfg(feature = "eks_identity_provider_config")]
pub use eks_identity_provider_config::*;

#[cfg(feature = "eks_node_group")]
pub mod eks_node_group;

#[cfg(feature = "eks_node_group")]
pub use eks_node_group::*;

#[cfg(feature = "eks_pod_identity_association")]
pub mod eks_pod_identity_association;

#[cfg(feature = "eks_pod_identity_association")]
pub use eks_pod_identity_association::*;

#[cfg(feature = "elastic_beanstalk_application")]
pub mod elastic_beanstalk_application;

#[cfg(feature = "elastic_beanstalk_application")]
pub use elastic_beanstalk_application::*;

#[cfg(feature = "elastic_beanstalk_application_version")]
pub mod elastic_beanstalk_application_version;

#[cfg(feature = "elastic_beanstalk_application_version")]
pub use elastic_beanstalk_application_version::*;

#[cfg(feature = "elastic_beanstalk_configuration_template")]
pub mod elastic_beanstalk_configuration_template;

#[cfg(feature = "elastic_beanstalk_configuration_template")]
pub use elastic_beanstalk_configuration_template::*;

#[cfg(feature = "elastic_beanstalk_environment")]
pub mod elastic_beanstalk_environment;

#[cfg(feature = "elastic_beanstalk_environment")]
pub use elastic_beanstalk_environment::*;

#[cfg(feature = "elasticache_cluster")]
pub mod elasticache_cluster;

#[cfg(feature = "elasticache_cluster")]
pub use elasticache_cluster::*;

#[cfg(feature = "elasticache_global_replication_group")]
pub mod elasticache_global_replication_group;

#[cfg(feature = "elasticache_global_replication_group")]
pub use elasticache_global_replication_group::*;

#[cfg(feature = "elasticache_parameter_group")]
pub mod elasticache_parameter_group;

#[cfg(feature = "elasticache_parameter_group")]
pub use elasticache_parameter_group::*;

#[cfg(feature = "elasticache_replication_group")]
pub mod elasticache_replication_group;

#[cfg(feature = "elasticache_replication_group")]
pub use elasticache_replication_group::*;

#[cfg(feature = "elasticache_reserved_cache_node")]
pub mod elasticache_reserved_cache_node;

#[cfg(feature = "elasticache_reserved_cache_node")]
pub use elasticache_reserved_cache_node::*;

#[cfg(feature = "elasticache_serverless_cache")]
pub mod elasticache_serverless_cache;

#[cfg(feature = "elasticache_serverless_cache")]
pub use elasticache_serverless_cache::*;

#[cfg(feature = "elasticache_subnet_group")]
pub mod elasticache_subnet_group;

#[cfg(feature = "elasticache_subnet_group")]
pub use elasticache_subnet_group::*;

#[cfg(feature = "elasticache_user")]
pub mod elasticache_user;

#[cfg(feature = "elasticache_user")]
pub use elasticache_user::*;

#[cfg(feature = "elasticache_user_group")]
pub mod elasticache_user_group;

#[cfg(feature = "elasticache_user_group")]
pub use elasticache_user_group::*;

#[cfg(feature = "elasticache_user_group_association")]
pub mod elasticache_user_group_association;

#[cfg(feature = "elasticache_user_group_association")]
pub use elasticache_user_group_association::*;

#[cfg(feature = "elasticsearch_domain")]
pub mod elasticsearch_domain;

#[cfg(feature = "elasticsearch_domain")]
pub use elasticsearch_domain::*;

#[cfg(feature = "elasticsearch_domain_policy")]
pub mod elasticsearch_domain_policy;

#[cfg(feature = "elasticsearch_domain_policy")]
pub use elasticsearch_domain_policy::*;

#[cfg(feature = "elasticsearch_domain_saml_options")]
pub mod elasticsearch_domain_saml_options;

#[cfg(feature = "elasticsearch_domain_saml_options")]
pub use elasticsearch_domain_saml_options::*;

#[cfg(feature = "elasticsearch_vpc_endpoint")]
pub mod elasticsearch_vpc_endpoint;

#[cfg(feature = "elasticsearch_vpc_endpoint")]
pub use elasticsearch_vpc_endpoint::*;

#[cfg(feature = "elastictranscoder_pipeline")]
pub mod elastictranscoder_pipeline;

#[cfg(feature = "elastictranscoder_pipeline")]
pub use elastictranscoder_pipeline::*;

#[cfg(feature = "elastictranscoder_preset")]
pub mod elastictranscoder_preset;

#[cfg(feature = "elastictranscoder_preset")]
pub use elastictranscoder_preset::*;

#[cfg(feature = "elb")]
pub mod elb;

#[cfg(feature = "elb")]
pub use elb::*;

#[cfg(feature = "elb_attachment")]
pub mod elb_attachment;

#[cfg(feature = "elb_attachment")]
pub use elb_attachment::*;

#[cfg(feature = "emr_block_public_access_configuration")]
pub mod emr_block_public_access_configuration;

#[cfg(feature = "emr_block_public_access_configuration")]
pub use emr_block_public_access_configuration::*;

#[cfg(feature = "emr_cluster")]
pub mod emr_cluster;

#[cfg(feature = "emr_cluster")]
pub use emr_cluster::*;

#[cfg(feature = "emr_instance_fleet")]
pub mod emr_instance_fleet;

#[cfg(feature = "emr_instance_fleet")]
pub use emr_instance_fleet::*;

#[cfg(feature = "emr_instance_group")]
pub mod emr_instance_group;

#[cfg(feature = "emr_instance_group")]
pub use emr_instance_group::*;

#[cfg(feature = "emr_managed_scaling_policy")]
pub mod emr_managed_scaling_policy;

#[cfg(feature = "emr_managed_scaling_policy")]
pub use emr_managed_scaling_policy::*;

#[cfg(feature = "emr_security_configuration")]
pub mod emr_security_configuration;

#[cfg(feature = "emr_security_configuration")]
pub use emr_security_configuration::*;

#[cfg(feature = "emr_studio")]
pub mod emr_studio;

#[cfg(feature = "emr_studio")]
pub use emr_studio::*;

#[cfg(feature = "emr_studio_session_mapping")]
pub mod emr_studio_session_mapping;

#[cfg(feature = "emr_studio_session_mapping")]
pub use emr_studio_session_mapping::*;

#[cfg(feature = "emrcontainers_job_template")]
pub mod emrcontainers_job_template;

#[cfg(feature = "emrcontainers_job_template")]
pub use emrcontainers_job_template::*;

#[cfg(feature = "emrcontainers_virtual_cluster")]
pub mod emrcontainers_virtual_cluster;

#[cfg(feature = "emrcontainers_virtual_cluster")]
pub use emrcontainers_virtual_cluster::*;

#[cfg(feature = "emrserverless_application")]
pub mod emrserverless_application;

#[cfg(feature = "emrserverless_application")]
pub use emrserverless_application::*;

#[cfg(feature = "evidently_feature")]
pub mod evidently_feature;

#[cfg(feature = "evidently_feature")]
pub use evidently_feature::*;

#[cfg(feature = "evidently_launch")]
pub mod evidently_launch;

#[cfg(feature = "evidently_launch")]
pub use evidently_launch::*;

#[cfg(feature = "evidently_project")]
pub mod evidently_project;

#[cfg(feature = "evidently_project")]
pub use evidently_project::*;

#[cfg(feature = "evidently_segment")]
pub mod evidently_segment;

#[cfg(feature = "evidently_segment")]
pub use evidently_segment::*;

#[cfg(feature = "finspace_kx_cluster")]
pub mod finspace_kx_cluster;

#[cfg(feature = "finspace_kx_cluster")]
pub use finspace_kx_cluster::*;

#[cfg(feature = "finspace_kx_database")]
pub mod finspace_kx_database;

#[cfg(feature = "finspace_kx_database")]
pub use finspace_kx_database::*;

#[cfg(feature = "finspace_kx_dataview")]
pub mod finspace_kx_dataview;

#[cfg(feature = "finspace_kx_dataview")]
pub use finspace_kx_dataview::*;

#[cfg(feature = "finspace_kx_environment")]
pub mod finspace_kx_environment;

#[cfg(feature = "finspace_kx_environment")]
pub use finspace_kx_environment::*;

#[cfg(feature = "finspace_kx_scaling_group")]
pub mod finspace_kx_scaling_group;

#[cfg(feature = "finspace_kx_scaling_group")]
pub use finspace_kx_scaling_group::*;

#[cfg(feature = "finspace_kx_user")]
pub mod finspace_kx_user;

#[cfg(feature = "finspace_kx_user")]
pub use finspace_kx_user::*;

#[cfg(feature = "finspace_kx_volume")]
pub mod finspace_kx_volume;

#[cfg(feature = "finspace_kx_volume")]
pub use finspace_kx_volume::*;

#[cfg(feature = "fis_experiment_template")]
pub mod fis_experiment_template;

#[cfg(feature = "fis_experiment_template")]
pub use fis_experiment_template::*;

#[cfg(feature = "fis_target_account_configuration")]
pub mod fis_target_account_configuration;

#[cfg(feature = "fis_target_account_configuration")]
pub use fis_target_account_configuration::*;

#[cfg(feature = "flow_log")]
pub mod flow_log;

#[cfg(feature = "flow_log")]
pub use flow_log::*;

#[cfg(feature = "fms_admin_account")]
pub mod fms_admin_account;

#[cfg(feature = "fms_admin_account")]
pub use fms_admin_account::*;

#[cfg(feature = "fms_policy")]
pub mod fms_policy;

#[cfg(feature = "fms_policy")]
pub use fms_policy::*;

#[cfg(feature = "fms_resource_set")]
pub mod fms_resource_set;

#[cfg(feature = "fms_resource_set")]
pub use fms_resource_set::*;

#[cfg(feature = "fsx_backup")]
pub mod fsx_backup;

#[cfg(feature = "fsx_backup")]
pub use fsx_backup::*;

#[cfg(feature = "fsx_data_repository_association")]
pub mod fsx_data_repository_association;

#[cfg(feature = "fsx_data_repository_association")]
pub use fsx_data_repository_association::*;

#[cfg(feature = "fsx_file_cache")]
pub mod fsx_file_cache;

#[cfg(feature = "fsx_file_cache")]
pub use fsx_file_cache::*;

#[cfg(feature = "fsx_lustre_file_system")]
pub mod fsx_lustre_file_system;

#[cfg(feature = "fsx_lustre_file_system")]
pub use fsx_lustre_file_system::*;

#[cfg(feature = "fsx_ontap_file_system")]
pub mod fsx_ontap_file_system;

#[cfg(feature = "fsx_ontap_file_system")]
pub use fsx_ontap_file_system::*;

#[cfg(feature = "fsx_ontap_storage_virtual_machine")]
pub mod fsx_ontap_storage_virtual_machine;

#[cfg(feature = "fsx_ontap_storage_virtual_machine")]
pub use fsx_ontap_storage_virtual_machine::*;

#[cfg(feature = "fsx_ontap_volume")]
pub mod fsx_ontap_volume;

#[cfg(feature = "fsx_ontap_volume")]
pub use fsx_ontap_volume::*;

#[cfg(feature = "fsx_openzfs_file_system")]
pub mod fsx_openzfs_file_system;

#[cfg(feature = "fsx_openzfs_file_system")]
pub use fsx_openzfs_file_system::*;

#[cfg(feature = "fsx_openzfs_snapshot")]
pub mod fsx_openzfs_snapshot;

#[cfg(feature = "fsx_openzfs_snapshot")]
pub use fsx_openzfs_snapshot::*;

#[cfg(feature = "fsx_openzfs_volume")]
pub mod fsx_openzfs_volume;

#[cfg(feature = "fsx_openzfs_volume")]
pub use fsx_openzfs_volume::*;

#[cfg(feature = "fsx_s3_access_point_attachment")]
pub mod fsx_s3_access_point_attachment;

#[cfg(feature = "fsx_s3_access_point_attachment")]
pub use fsx_s3_access_point_attachment::*;

#[cfg(feature = "fsx_windows_file_system")]
pub mod fsx_windows_file_system;

#[cfg(feature = "fsx_windows_file_system")]
pub use fsx_windows_file_system::*;

#[cfg(feature = "gamelift_alias")]
pub mod gamelift_alias;

#[cfg(feature = "gamelift_alias")]
pub use gamelift_alias::*;

#[cfg(feature = "gamelift_build")]
pub mod gamelift_build;

#[cfg(feature = "gamelift_build")]
pub use gamelift_build::*;

#[cfg(feature = "gamelift_fleet")]
pub mod gamelift_fleet;

#[cfg(feature = "gamelift_fleet")]
pub use gamelift_fleet::*;

#[cfg(feature = "gamelift_game_server_group")]
pub mod gamelift_game_server_group;

#[cfg(feature = "gamelift_game_server_group")]
pub use gamelift_game_server_group::*;

#[cfg(feature = "gamelift_game_session_queue")]
pub mod gamelift_game_session_queue;

#[cfg(feature = "gamelift_game_session_queue")]
pub use gamelift_game_session_queue::*;

#[cfg(feature = "gamelift_script")]
pub mod gamelift_script;

#[cfg(feature = "gamelift_script")]
pub use gamelift_script::*;

#[cfg(feature = "glacier_vault")]
pub mod glacier_vault;

#[cfg(feature = "glacier_vault")]
pub use glacier_vault::*;

#[cfg(feature = "glacier_vault_lock")]
pub mod glacier_vault_lock;

#[cfg(feature = "glacier_vault_lock")]
pub use glacier_vault_lock::*;

#[cfg(feature = "globalaccelerator_accelerator")]
pub mod globalaccelerator_accelerator;

#[cfg(feature = "globalaccelerator_accelerator")]
pub use globalaccelerator_accelerator::*;

#[cfg(feature = "globalaccelerator_cross_account_attachment")]
pub mod globalaccelerator_cross_account_attachment;

#[cfg(feature = "globalaccelerator_cross_account_attachment")]
pub use globalaccelerator_cross_account_attachment::*;

#[cfg(feature = "globalaccelerator_custom_routing_accelerator")]
pub mod globalaccelerator_custom_routing_accelerator;

#[cfg(feature = "globalaccelerator_custom_routing_accelerator")]
pub use globalaccelerator_custom_routing_accelerator::*;

#[cfg(feature = "globalaccelerator_custom_routing_endpoint_group")]
pub mod globalaccelerator_custom_routing_endpoint_group;

#[cfg(feature = "globalaccelerator_custom_routing_endpoint_group")]
pub use globalaccelerator_custom_routing_endpoint_group::*;

#[cfg(feature = "globalaccelerator_custom_routing_listener")]
pub mod globalaccelerator_custom_routing_listener;

#[cfg(feature = "globalaccelerator_custom_routing_listener")]
pub use globalaccelerator_custom_routing_listener::*;

#[cfg(feature = "globalaccelerator_endpoint_group")]
pub mod globalaccelerator_endpoint_group;

#[cfg(feature = "globalaccelerator_endpoint_group")]
pub use globalaccelerator_endpoint_group::*;

#[cfg(feature = "globalaccelerator_listener")]
pub mod globalaccelerator_listener;

#[cfg(feature = "globalaccelerator_listener")]
pub use globalaccelerator_listener::*;

#[cfg(feature = "glue_catalog_database")]
pub mod glue_catalog_database;

#[cfg(feature = "glue_catalog_database")]
pub use glue_catalog_database::*;

#[cfg(feature = "glue_catalog_table")]
pub mod glue_catalog_table;

#[cfg(feature = "glue_catalog_table")]
pub use glue_catalog_table::*;

#[cfg(feature = "glue_catalog_table_optimizer")]
pub mod glue_catalog_table_optimizer;

#[cfg(feature = "glue_catalog_table_optimizer")]
pub use glue_catalog_table_optimizer::*;

#[cfg(feature = "glue_classifier")]
pub mod glue_classifier;

#[cfg(feature = "glue_classifier")]
pub use glue_classifier::*;

#[cfg(feature = "glue_connection")]
pub mod glue_connection;

#[cfg(feature = "glue_connection")]
pub use glue_connection::*;

#[cfg(feature = "glue_crawler")]
pub mod glue_crawler;

#[cfg(feature = "glue_crawler")]
pub use glue_crawler::*;

#[cfg(feature = "glue_data_catalog_encryption_settings")]
pub mod glue_data_catalog_encryption_settings;

#[cfg(feature = "glue_data_catalog_encryption_settings")]
pub use glue_data_catalog_encryption_settings::*;

#[cfg(feature = "glue_data_quality_ruleset")]
pub mod glue_data_quality_ruleset;

#[cfg(feature = "glue_data_quality_ruleset")]
pub use glue_data_quality_ruleset::*;

#[cfg(feature = "glue_dev_endpoint")]
pub mod glue_dev_endpoint;

#[cfg(feature = "glue_dev_endpoint")]
pub use glue_dev_endpoint::*;

#[cfg(feature = "glue_job")]
pub mod glue_job;

#[cfg(feature = "glue_job")]
pub use glue_job::*;

#[cfg(feature = "glue_ml_transform")]
pub mod glue_ml_transform;

#[cfg(feature = "glue_ml_transform")]
pub use glue_ml_transform::*;

#[cfg(feature = "glue_partition")]
pub mod glue_partition;

#[cfg(feature = "glue_partition")]
pub use glue_partition::*;

#[cfg(feature = "glue_partition_index")]
pub mod glue_partition_index;

#[cfg(feature = "glue_partition_index")]
pub use glue_partition_index::*;

#[cfg(feature = "glue_registry")]
pub mod glue_registry;

#[cfg(feature = "glue_registry")]
pub use glue_registry::*;

#[cfg(feature = "glue_resource_policy")]
pub mod glue_resource_policy;

#[cfg(feature = "glue_resource_policy")]
pub use glue_resource_policy::*;

#[cfg(feature = "glue_schema")]
pub mod glue_schema;

#[cfg(feature = "glue_schema")]
pub use glue_schema::*;

#[cfg(feature = "glue_security_configuration")]
pub mod glue_security_configuration;

#[cfg(feature = "glue_security_configuration")]
pub use glue_security_configuration::*;

#[cfg(feature = "glue_trigger")]
pub mod glue_trigger;

#[cfg(feature = "glue_trigger")]
pub use glue_trigger::*;

#[cfg(feature = "glue_user_defined_function")]
pub mod glue_user_defined_function;

#[cfg(feature = "glue_user_defined_function")]
pub use glue_user_defined_function::*;

#[cfg(feature = "glue_workflow")]
pub mod glue_workflow;

#[cfg(feature = "glue_workflow")]
pub use glue_workflow::*;

#[cfg(feature = "grafana_license_association")]
pub mod grafana_license_association;

#[cfg(feature = "grafana_license_association")]
pub use grafana_license_association::*;

#[cfg(feature = "grafana_role_association")]
pub mod grafana_role_association;

#[cfg(feature = "grafana_role_association")]
pub use grafana_role_association::*;

#[cfg(feature = "grafana_workspace")]
pub mod grafana_workspace;

#[cfg(feature = "grafana_workspace")]
pub use grafana_workspace::*;

#[cfg(feature = "grafana_workspace_api_key")]
pub mod grafana_workspace_api_key;

#[cfg(feature = "grafana_workspace_api_key")]
pub use grafana_workspace_api_key::*;

#[cfg(feature = "grafana_workspace_saml_configuration")]
pub mod grafana_workspace_saml_configuration;

#[cfg(feature = "grafana_workspace_saml_configuration")]
pub use grafana_workspace_saml_configuration::*;

#[cfg(feature = "grafana_workspace_service_account")]
pub mod grafana_workspace_service_account;

#[cfg(feature = "grafana_workspace_service_account")]
pub use grafana_workspace_service_account::*;

#[cfg(feature = "grafana_workspace_service_account_token")]
pub mod grafana_workspace_service_account_token;

#[cfg(feature = "grafana_workspace_service_account_token")]
pub use grafana_workspace_service_account_token::*;

#[cfg(feature = "guardduty_detector")]
pub mod guardduty_detector;

#[cfg(feature = "guardduty_detector")]
pub use guardduty_detector::*;

#[cfg(feature = "guardduty_detector_feature")]
pub mod guardduty_detector_feature;

#[cfg(feature = "guardduty_detector_feature")]
pub use guardduty_detector_feature::*;

#[cfg(feature = "guardduty_filter")]
pub mod guardduty_filter;

#[cfg(feature = "guardduty_filter")]
pub use guardduty_filter::*;

#[cfg(feature = "guardduty_invite_accepter")]
pub mod guardduty_invite_accepter;

#[cfg(feature = "guardduty_invite_accepter")]
pub use guardduty_invite_accepter::*;

#[cfg(feature = "guardduty_ipset")]
pub mod guardduty_ipset;

#[cfg(feature = "guardduty_ipset")]
pub use guardduty_ipset::*;

#[cfg(feature = "guardduty_malware_protection_plan")]
pub mod guardduty_malware_protection_plan;

#[cfg(feature = "guardduty_malware_protection_plan")]
pub use guardduty_malware_protection_plan::*;

#[cfg(feature = "guardduty_member")]
pub mod guardduty_member;

#[cfg(feature = "guardduty_member")]
pub use guardduty_member::*;

#[cfg(feature = "guardduty_member_detector_feature")]
pub mod guardduty_member_detector_feature;

#[cfg(feature = "guardduty_member_detector_feature")]
pub use guardduty_member_detector_feature::*;

#[cfg(feature = "guardduty_organization_admin_account")]
pub mod guardduty_organization_admin_account;

#[cfg(feature = "guardduty_organization_admin_account")]
pub use guardduty_organization_admin_account::*;

#[cfg(feature = "guardduty_organization_configuration")]
pub mod guardduty_organization_configuration;

#[cfg(feature = "guardduty_organization_configuration")]
pub use guardduty_organization_configuration::*;

#[cfg(feature = "guardduty_organization_configuration_feature")]
pub mod guardduty_organization_configuration_feature;

#[cfg(feature = "guardduty_organization_configuration_feature")]
pub use guardduty_organization_configuration_feature::*;

#[cfg(feature = "guardduty_publishing_destination")]
pub mod guardduty_publishing_destination;

#[cfg(feature = "guardduty_publishing_destination")]
pub use guardduty_publishing_destination::*;

#[cfg(feature = "guardduty_threatintelset")]
pub mod guardduty_threatintelset;

#[cfg(feature = "guardduty_threatintelset")]
pub use guardduty_threatintelset::*;

#[cfg(feature = "iam_access_key")]
pub mod iam_access_key;

#[cfg(feature = "iam_access_key")]
pub use iam_access_key::*;

#[cfg(feature = "iam_account_alias")]
pub mod iam_account_alias;

#[cfg(feature = "iam_account_alias")]
pub use iam_account_alias::*;

#[cfg(feature = "iam_account_password_policy")]
pub mod iam_account_password_policy;

#[cfg(feature = "iam_account_password_policy")]
pub use iam_account_password_policy::*;

#[cfg(feature = "iam_group")]
pub mod iam_group;

#[cfg(feature = "iam_group")]
pub use iam_group::*;

#[cfg(feature = "iam_group_membership")]
pub mod iam_group_membership;

#[cfg(feature = "iam_group_membership")]
pub use iam_group_membership::*;

#[cfg(feature = "iam_group_policies_exclusive")]
pub mod iam_group_policies_exclusive;

#[cfg(feature = "iam_group_policies_exclusive")]
pub use iam_group_policies_exclusive::*;

#[cfg(feature = "iam_group_policy")]
pub mod iam_group_policy;

#[cfg(feature = "iam_group_policy")]
pub use iam_group_policy::*;

#[cfg(feature = "iam_group_policy_attachment")]
pub mod iam_group_policy_attachment;

#[cfg(feature = "iam_group_policy_attachment")]
pub use iam_group_policy_attachment::*;

#[cfg(feature = "iam_group_policy_attachments_exclusive")]
pub mod iam_group_policy_attachments_exclusive;

#[cfg(feature = "iam_group_policy_attachments_exclusive")]
pub use iam_group_policy_attachments_exclusive::*;

#[cfg(feature = "iam_instance_profile")]
pub mod iam_instance_profile;

#[cfg(feature = "iam_instance_profile")]
pub use iam_instance_profile::*;

#[cfg(feature = "iam_openid_connect_provider")]
pub mod iam_openid_connect_provider;

#[cfg(feature = "iam_openid_connect_provider")]
pub use iam_openid_connect_provider::*;

#[cfg(feature = "iam_organizations_features")]
pub mod iam_organizations_features;

#[cfg(feature = "iam_organizations_features")]
pub use iam_organizations_features::*;

#[cfg(feature = "iam_policy")]
pub mod iam_policy;

#[cfg(feature = "iam_policy")]
pub use iam_policy::*;

#[cfg(feature = "iam_policy_attachment")]
pub mod iam_policy_attachment;

#[cfg(feature = "iam_policy_attachment")]
pub use iam_policy_attachment::*;

#[cfg(feature = "iam_role")]
pub mod iam_role;

#[cfg(feature = "iam_role")]
pub use iam_role::*;

#[cfg(feature = "iam_role_policies_exclusive")]
pub mod iam_role_policies_exclusive;

#[cfg(feature = "iam_role_policies_exclusive")]
pub use iam_role_policies_exclusive::*;

#[cfg(feature = "iam_role_policy")]
pub mod iam_role_policy;

#[cfg(feature = "iam_role_policy")]
pub use iam_role_policy::*;

#[cfg(feature = "iam_role_policy_attachment")]
pub mod iam_role_policy_attachment;

#[cfg(feature = "iam_role_policy_attachment")]
pub use iam_role_policy_attachment::*;

#[cfg(feature = "iam_role_policy_attachments_exclusive")]
pub mod iam_role_policy_attachments_exclusive;

#[cfg(feature = "iam_role_policy_attachments_exclusive")]
pub use iam_role_policy_attachments_exclusive::*;

#[cfg(feature = "iam_saml_provider")]
pub mod iam_saml_provider;

#[cfg(feature = "iam_saml_provider")]
pub use iam_saml_provider::*;

#[cfg(feature = "iam_security_token_service_preferences")]
pub mod iam_security_token_service_preferences;

#[cfg(feature = "iam_security_token_service_preferences")]
pub use iam_security_token_service_preferences::*;

#[cfg(feature = "iam_server_certificate")]
pub mod iam_server_certificate;

#[cfg(feature = "iam_server_certificate")]
pub use iam_server_certificate::*;

#[cfg(feature = "iam_service_linked_role")]
pub mod iam_service_linked_role;

#[cfg(feature = "iam_service_linked_role")]
pub use iam_service_linked_role::*;

#[cfg(feature = "iam_service_specific_credential")]
pub mod iam_service_specific_credential;

#[cfg(feature = "iam_service_specific_credential")]
pub use iam_service_specific_credential::*;

#[cfg(feature = "iam_signing_certificate")]
pub mod iam_signing_certificate;

#[cfg(feature = "iam_signing_certificate")]
pub use iam_signing_certificate::*;

#[cfg(feature = "iam_user")]
pub mod iam_user;

#[cfg(feature = "iam_user")]
pub use iam_user::*;

#[cfg(feature = "iam_user_group_membership")]
pub mod iam_user_group_membership;

#[cfg(feature = "iam_user_group_membership")]
pub use iam_user_group_membership::*;

#[cfg(feature = "iam_user_login_profile")]
pub mod iam_user_login_profile;

#[cfg(feature = "iam_user_login_profile")]
pub use iam_user_login_profile::*;

#[cfg(feature = "iam_user_policies_exclusive")]
pub mod iam_user_policies_exclusive;

#[cfg(feature = "iam_user_policies_exclusive")]
pub use iam_user_policies_exclusive::*;

#[cfg(feature = "iam_user_policy")]
pub mod iam_user_policy;

#[cfg(feature = "iam_user_policy")]
pub use iam_user_policy::*;

#[cfg(feature = "iam_user_policy_attachment")]
pub mod iam_user_policy_attachment;

#[cfg(feature = "iam_user_policy_attachment")]
pub use iam_user_policy_attachment::*;

#[cfg(feature = "iam_user_policy_attachments_exclusive")]
pub mod iam_user_policy_attachments_exclusive;

#[cfg(feature = "iam_user_policy_attachments_exclusive")]
pub use iam_user_policy_attachments_exclusive::*;

#[cfg(feature = "iam_user_ssh_key")]
pub mod iam_user_ssh_key;

#[cfg(feature = "iam_user_ssh_key")]
pub use iam_user_ssh_key::*;

#[cfg(feature = "iam_virtual_mfa_device")]
pub mod iam_virtual_mfa_device;

#[cfg(feature = "iam_virtual_mfa_device")]
pub use iam_virtual_mfa_device::*;

#[cfg(feature = "identitystore_group")]
pub mod identitystore_group;

#[cfg(feature = "identitystore_group")]
pub use identitystore_group::*;

#[cfg(feature = "identitystore_group_membership")]
pub mod identitystore_group_membership;

#[cfg(feature = "identitystore_group_membership")]
pub use identitystore_group_membership::*;

#[cfg(feature = "identitystore_user")]
pub mod identitystore_user;

#[cfg(feature = "identitystore_user")]
pub use identitystore_user::*;

#[cfg(feature = "imagebuilder_component")]
pub mod imagebuilder_component;

#[cfg(feature = "imagebuilder_component")]
pub use imagebuilder_component::*;

#[cfg(feature = "imagebuilder_container_recipe")]
pub mod imagebuilder_container_recipe;

#[cfg(feature = "imagebuilder_container_recipe")]
pub use imagebuilder_container_recipe::*;

#[cfg(feature = "imagebuilder_distribution_configuration")]
pub mod imagebuilder_distribution_configuration;

#[cfg(feature = "imagebuilder_distribution_configuration")]
pub use imagebuilder_distribution_configuration::*;

#[cfg(feature = "imagebuilder_image")]
pub mod imagebuilder_image;

#[cfg(feature = "imagebuilder_image")]
pub use imagebuilder_image::*;

#[cfg(feature = "imagebuilder_image_pipeline")]
pub mod imagebuilder_image_pipeline;

#[cfg(feature = "imagebuilder_image_pipeline")]
pub use imagebuilder_image_pipeline::*;

#[cfg(feature = "imagebuilder_image_recipe")]
pub mod imagebuilder_image_recipe;

#[cfg(feature = "imagebuilder_image_recipe")]
pub use imagebuilder_image_recipe::*;

#[cfg(feature = "imagebuilder_infrastructure_configuration")]
pub mod imagebuilder_infrastructure_configuration;

#[cfg(feature = "imagebuilder_infrastructure_configuration")]
pub use imagebuilder_infrastructure_configuration::*;

#[cfg(feature = "imagebuilder_lifecycle_policy")]
pub mod imagebuilder_lifecycle_policy;

#[cfg(feature = "imagebuilder_lifecycle_policy")]
pub use imagebuilder_lifecycle_policy::*;

#[cfg(feature = "imagebuilder_workflow")]
pub mod imagebuilder_workflow;

#[cfg(feature = "imagebuilder_workflow")]
pub use imagebuilder_workflow::*;

#[cfg(feature = "inspector2_delegated_admin_account")]
pub mod inspector2_delegated_admin_account;

#[cfg(feature = "inspector2_delegated_admin_account")]
pub use inspector2_delegated_admin_account::*;

#[cfg(feature = "inspector2_enabler")]
pub mod inspector2_enabler;

#[cfg(feature = "inspector2_enabler")]
pub use inspector2_enabler::*;

#[cfg(feature = "inspector2_filter")]
pub mod inspector2_filter;

#[cfg(feature = "inspector2_filter")]
pub use inspector2_filter::*;

#[cfg(feature = "inspector2_member_association")]
pub mod inspector2_member_association;

#[cfg(feature = "inspector2_member_association")]
pub use inspector2_member_association::*;

#[cfg(feature = "inspector2_organization_configuration")]
pub mod inspector2_organization_configuration;

#[cfg(feature = "inspector2_organization_configuration")]
pub use inspector2_organization_configuration::*;

#[cfg(feature = "inspector_assessment_target")]
pub mod inspector_assessment_target;

#[cfg(feature = "inspector_assessment_target")]
pub use inspector_assessment_target::*;

#[cfg(feature = "inspector_assessment_template")]
pub mod inspector_assessment_template;

#[cfg(feature = "inspector_assessment_template")]
pub use inspector_assessment_template::*;

#[cfg(feature = "inspector_resource_group")]
pub mod inspector_resource_group;

#[cfg(feature = "inspector_resource_group")]
pub use inspector_resource_group::*;

#[cfg(feature = "instance")]
pub mod instance;

#[cfg(feature = "instance")]
pub use instance::*;

#[cfg(feature = "internet_gateway")]
pub mod internet_gateway;

#[cfg(feature = "internet_gateway")]
pub use internet_gateway::*;

#[cfg(feature = "internet_gateway_attachment")]
pub mod internet_gateway_attachment;

#[cfg(feature = "internet_gateway_attachment")]
pub use internet_gateway_attachment::*;

#[cfg(feature = "internetmonitor_monitor")]
pub mod internetmonitor_monitor;

#[cfg(feature = "internetmonitor_monitor")]
pub use internetmonitor_monitor::*;

#[cfg(feature = "invoicing_invoice_unit")]
pub mod invoicing_invoice_unit;

#[cfg(feature = "invoicing_invoice_unit")]
pub use invoicing_invoice_unit::*;

#[cfg(feature = "iot_authorizer")]
pub mod iot_authorizer;

#[cfg(feature = "iot_authorizer")]
pub use iot_authorizer::*;

#[cfg(feature = "iot_billing_group")]
pub mod iot_billing_group;

#[cfg(feature = "iot_billing_group")]
pub use iot_billing_group::*;

#[cfg(feature = "iot_ca_certificate")]
pub mod iot_ca_certificate;

#[cfg(feature = "iot_ca_certificate")]
pub use iot_ca_certificate::*;

#[cfg(feature = "iot_certificate")]
pub mod iot_certificate;

#[cfg(feature = "iot_certificate")]
pub use iot_certificate::*;

#[cfg(feature = "iot_domain_configuration")]
pub mod iot_domain_configuration;

#[cfg(feature = "iot_domain_configuration")]
pub use iot_domain_configuration::*;

#[cfg(feature = "iot_event_configurations")]
pub mod iot_event_configurations;

#[cfg(feature = "iot_event_configurations")]
pub use iot_event_configurations::*;

#[cfg(feature = "iot_indexing_configuration")]
pub mod iot_indexing_configuration;

#[cfg(feature = "iot_indexing_configuration")]
pub use iot_indexing_configuration::*;

#[cfg(feature = "iot_logging_options")]
pub mod iot_logging_options;

#[cfg(feature = "iot_logging_options")]
pub use iot_logging_options::*;

#[cfg(feature = "iot_policy")]
pub mod iot_policy;

#[cfg(feature = "iot_policy")]
pub use iot_policy::*;

#[cfg(feature = "iot_policy_attachment")]
pub mod iot_policy_attachment;

#[cfg(feature = "iot_policy_attachment")]
pub use iot_policy_attachment::*;

#[cfg(feature = "iot_provisioning_template")]
pub mod iot_provisioning_template;

#[cfg(feature = "iot_provisioning_template")]
pub use iot_provisioning_template::*;

#[cfg(feature = "iot_role_alias")]
pub mod iot_role_alias;

#[cfg(feature = "iot_role_alias")]
pub use iot_role_alias::*;

#[cfg(feature = "iot_thing")]
pub mod iot_thing;

#[cfg(feature = "iot_thing")]
pub use iot_thing::*;

#[cfg(feature = "iot_thing_group")]
pub mod iot_thing_group;

#[cfg(feature = "iot_thing_group")]
pub use iot_thing_group::*;

#[cfg(feature = "iot_thing_group_membership")]
pub mod iot_thing_group_membership;

#[cfg(feature = "iot_thing_group_membership")]
pub use iot_thing_group_membership::*;

#[cfg(feature = "iot_thing_principal_attachment")]
pub mod iot_thing_principal_attachment;

#[cfg(feature = "iot_thing_principal_attachment")]
pub use iot_thing_principal_attachment::*;

#[cfg(feature = "iot_thing_type")]
pub mod iot_thing_type;

#[cfg(feature = "iot_thing_type")]
pub use iot_thing_type::*;

#[cfg(feature = "iot_topic_rule")]
pub mod iot_topic_rule;

#[cfg(feature = "iot_topic_rule")]
pub use iot_topic_rule::*;

#[cfg(feature = "iot_topic_rule_destination")]
pub mod iot_topic_rule_destination;

#[cfg(feature = "iot_topic_rule_destination")]
pub use iot_topic_rule_destination::*;

#[cfg(feature = "ivs_channel")]
pub mod ivs_channel;

#[cfg(feature = "ivs_channel")]
pub use ivs_channel::*;

#[cfg(feature = "ivs_playback_key_pair")]
pub mod ivs_playback_key_pair;

#[cfg(feature = "ivs_playback_key_pair")]
pub use ivs_playback_key_pair::*;

#[cfg(feature = "ivs_recording_configuration")]
pub mod ivs_recording_configuration;

#[cfg(feature = "ivs_recording_configuration")]
pub use ivs_recording_configuration::*;

#[cfg(feature = "ivschat_logging_configuration")]
pub mod ivschat_logging_configuration;

#[cfg(feature = "ivschat_logging_configuration")]
pub use ivschat_logging_configuration::*;

#[cfg(feature = "ivschat_room")]
pub mod ivschat_room;

#[cfg(feature = "ivschat_room")]
pub use ivschat_room::*;

#[cfg(feature = "kendra_data_source")]
pub mod kendra_data_source;

#[cfg(feature = "kendra_data_source")]
pub use kendra_data_source::*;

#[cfg(feature = "kendra_experience")]
pub mod kendra_experience;

#[cfg(feature = "kendra_experience")]
pub use kendra_experience::*;

#[cfg(feature = "kendra_faq")]
pub mod kendra_faq;

#[cfg(feature = "kendra_faq")]
pub use kendra_faq::*;

#[cfg(feature = "kendra_index")]
pub mod kendra_index;

#[cfg(feature = "kendra_index")]
pub use kendra_index::*;

#[cfg(feature = "kendra_query_suggestions_block_list")]
pub mod kendra_query_suggestions_block_list;

#[cfg(feature = "kendra_query_suggestions_block_list")]
pub use kendra_query_suggestions_block_list::*;

#[cfg(feature = "kendra_thesaurus")]
pub mod kendra_thesaurus;

#[cfg(feature = "kendra_thesaurus")]
pub use kendra_thesaurus::*;

#[cfg(feature = "key_pair")]
pub mod key_pair;

#[cfg(feature = "key_pair")]
pub use key_pair::*;

#[cfg(feature = "keyspaces_keyspace")]
pub mod keyspaces_keyspace;

#[cfg(feature = "keyspaces_keyspace")]
pub use keyspaces_keyspace::*;

#[cfg(feature = "keyspaces_table")]
pub mod keyspaces_table;

#[cfg(feature = "keyspaces_table")]
pub use keyspaces_table::*;

#[cfg(feature = "kinesis_analytics_application")]
pub mod kinesis_analytics_application;

#[cfg(feature = "kinesis_analytics_application")]
pub use kinesis_analytics_application::*;

#[cfg(feature = "kinesis_resource_policy")]
pub mod kinesis_resource_policy;

#[cfg(feature = "kinesis_resource_policy")]
pub use kinesis_resource_policy::*;

#[cfg(feature = "kinesis_stream")]
pub mod kinesis_stream;

#[cfg(feature = "kinesis_stream")]
pub use kinesis_stream::*;

#[cfg(feature = "kinesis_stream_consumer")]
pub mod kinesis_stream_consumer;

#[cfg(feature = "kinesis_stream_consumer")]
pub use kinesis_stream_consumer::*;

#[cfg(feature = "kinesis_video_stream")]
pub mod kinesis_video_stream;

#[cfg(feature = "kinesis_video_stream")]
pub use kinesis_video_stream::*;

#[cfg(feature = "kinesisanalyticsv2_application")]
pub mod kinesisanalyticsv2_application;

#[cfg(feature = "kinesisanalyticsv2_application")]
pub use kinesisanalyticsv2_application::*;

#[cfg(feature = "kinesisanalyticsv2_application_snapshot")]
pub mod kinesisanalyticsv2_application_snapshot;

#[cfg(feature = "kinesisanalyticsv2_application_snapshot")]
pub use kinesisanalyticsv2_application_snapshot::*;

#[cfg(feature = "kms_alias")]
pub mod kms_alias;

#[cfg(feature = "kms_alias")]
pub use kms_alias::*;

#[cfg(feature = "kms_ciphertext")]
pub mod kms_ciphertext;

#[cfg(feature = "kms_ciphertext")]
pub use kms_ciphertext::*;

#[cfg(feature = "kms_custom_key_store")]
pub mod kms_custom_key_store;

#[cfg(feature = "kms_custom_key_store")]
pub use kms_custom_key_store::*;

#[cfg(feature = "kms_external_key")]
pub mod kms_external_key;

#[cfg(feature = "kms_external_key")]
pub use kms_external_key::*;

#[cfg(feature = "kms_grant")]
pub mod kms_grant;

#[cfg(feature = "kms_grant")]
pub use kms_grant::*;

#[cfg(feature = "kms_key")]
pub mod kms_key;

#[cfg(feature = "kms_key")]
pub use kms_key::*;

#[cfg(feature = "kms_key_policy")]
pub mod kms_key_policy;

#[cfg(feature = "kms_key_policy")]
pub use kms_key_policy::*;

#[cfg(feature = "kms_replica_external_key")]
pub mod kms_replica_external_key;

#[cfg(feature = "kms_replica_external_key")]
pub use kms_replica_external_key::*;

#[cfg(feature = "kms_replica_key")]
pub mod kms_replica_key;

#[cfg(feature = "kms_replica_key")]
pub use kms_replica_key::*;

#[cfg(feature = "lakeformation_data_cells_filter")]
pub mod lakeformation_data_cells_filter;

#[cfg(feature = "lakeformation_data_cells_filter")]
pub use lakeformation_data_cells_filter::*;

#[cfg(feature = "lakeformation_data_lake_settings")]
pub mod lakeformation_data_lake_settings;

#[cfg(feature = "lakeformation_data_lake_settings")]
pub use lakeformation_data_lake_settings::*;

#[cfg(feature = "lakeformation_identity_center_configuration")]
pub mod lakeformation_identity_center_configuration;

#[cfg(feature = "lakeformation_identity_center_configuration")]
pub use lakeformation_identity_center_configuration::*;

#[cfg(feature = "lakeformation_lf_tag")]
pub mod lakeformation_lf_tag;

#[cfg(feature = "lakeformation_lf_tag")]
pub use lakeformation_lf_tag::*;

#[cfg(feature = "lakeformation_lf_tag_expression")]
pub mod lakeformation_lf_tag_expression;

#[cfg(feature = "lakeformation_lf_tag_expression")]
pub use lakeformation_lf_tag_expression::*;

#[cfg(feature = "lakeformation_opt_in")]
pub mod lakeformation_opt_in;

#[cfg(feature = "lakeformation_opt_in")]
pub use lakeformation_opt_in::*;

#[cfg(feature = "lakeformation_permissions")]
pub mod lakeformation_permissions;

#[cfg(feature = "lakeformation_permissions")]
pub use lakeformation_permissions::*;

#[cfg(feature = "lakeformation_resource")]
pub mod lakeformation_resource;

#[cfg(feature = "lakeformation_resource")]
pub use lakeformation_resource::*;

#[cfg(feature = "lakeformation_resource_lf_tag")]
pub mod lakeformation_resource_lf_tag;

#[cfg(feature = "lakeformation_resource_lf_tag")]
pub use lakeformation_resource_lf_tag::*;

#[cfg(feature = "lakeformation_resource_lf_tags")]
pub mod lakeformation_resource_lf_tags;

#[cfg(feature = "lakeformation_resource_lf_tags")]
pub use lakeformation_resource_lf_tags::*;

#[cfg(feature = "lambda_alias")]
pub mod lambda_alias;

#[cfg(feature = "lambda_alias")]
pub use lambda_alias::*;

#[cfg(feature = "lambda_code_signing_config")]
pub mod lambda_code_signing_config;

#[cfg(feature = "lambda_code_signing_config")]
pub use lambda_code_signing_config::*;

#[cfg(feature = "lambda_event_source_mapping")]
pub mod lambda_event_source_mapping;

#[cfg(feature = "lambda_event_source_mapping")]
pub use lambda_event_source_mapping::*;

#[cfg(feature = "lambda_function")]
pub mod lambda_function;

#[cfg(feature = "lambda_function")]
pub use lambda_function::*;

#[cfg(feature = "lambda_function_event_invoke_config")]
pub mod lambda_function_event_invoke_config;

#[cfg(feature = "lambda_function_event_invoke_config")]
pub use lambda_function_event_invoke_config::*;

#[cfg(feature = "lambda_function_recursion_config")]
pub mod lambda_function_recursion_config;

#[cfg(feature = "lambda_function_recursion_config")]
pub use lambda_function_recursion_config::*;

#[cfg(feature = "lambda_function_url")]
pub mod lambda_function_url;

#[cfg(feature = "lambda_function_url")]
pub use lambda_function_url::*;

#[cfg(feature = "lambda_invocation")]
pub mod lambda_invocation;

#[cfg(feature = "lambda_invocation")]
pub use lambda_invocation::*;

#[cfg(feature = "lambda_layer_version")]
pub mod lambda_layer_version;

#[cfg(feature = "lambda_layer_version")]
pub use lambda_layer_version::*;

#[cfg(feature = "lambda_layer_version_permission")]
pub mod lambda_layer_version_permission;

#[cfg(feature = "lambda_layer_version_permission")]
pub use lambda_layer_version_permission::*;

#[cfg(feature = "lambda_permission")]
pub mod lambda_permission;

#[cfg(feature = "lambda_permission")]
pub use lambda_permission::*;

#[cfg(feature = "lambda_provisioned_concurrency_config")]
pub mod lambda_provisioned_concurrency_config;

#[cfg(feature = "lambda_provisioned_concurrency_config")]
pub use lambda_provisioned_concurrency_config::*;

#[cfg(feature = "lambda_runtime_management_config")]
pub mod lambda_runtime_management_config;

#[cfg(feature = "lambda_runtime_management_config")]
pub use lambda_runtime_management_config::*;

#[cfg(feature = "launch_configuration")]
pub mod launch_configuration;

#[cfg(feature = "launch_configuration")]
pub use launch_configuration::*;

#[cfg(feature = "launch_template")]
pub mod launch_template;

#[cfg(feature = "launch_template")]
pub use launch_template::*;

#[cfg(feature = "lb")]
pub mod lb;

#[cfg(feature = "lb")]
pub use lb::*;

#[cfg(feature = "lb_cookie_stickiness_policy")]
pub mod lb_cookie_stickiness_policy;

#[cfg(feature = "lb_cookie_stickiness_policy")]
pub use lb_cookie_stickiness_policy::*;

#[cfg(feature = "lb_listener")]
pub mod lb_listener;

#[cfg(feature = "lb_listener")]
pub use lb_listener::*;

#[cfg(feature = "lb_listener_certificate")]
pub mod lb_listener_certificate;

#[cfg(feature = "lb_listener_certificate")]
pub use lb_listener_certificate::*;

#[cfg(feature = "lb_listener_rule")]
pub mod lb_listener_rule;

#[cfg(feature = "lb_listener_rule")]
pub use lb_listener_rule::*;

#[cfg(feature = "lb_ssl_negotiation_policy")]
pub mod lb_ssl_negotiation_policy;

#[cfg(feature = "lb_ssl_negotiation_policy")]
pub use lb_ssl_negotiation_policy::*;

#[cfg(feature = "lb_target_group")]
pub mod lb_target_group;

#[cfg(feature = "lb_target_group")]
pub use lb_target_group::*;

#[cfg(feature = "lb_target_group_attachment")]
pub mod lb_target_group_attachment;

#[cfg(feature = "lb_target_group_attachment")]
pub use lb_target_group_attachment::*;

#[cfg(feature = "lb_trust_store")]
pub mod lb_trust_store;

#[cfg(feature = "lb_trust_store")]
pub use lb_trust_store::*;

#[cfg(feature = "lb_trust_store_revocation")]
pub mod lb_trust_store_revocation;

#[cfg(feature = "lb_trust_store_revocation")]
pub use lb_trust_store_revocation::*;

#[cfg(feature = "lex_bot")]
pub mod lex_bot;

#[cfg(feature = "lex_bot")]
pub use lex_bot::*;

#[cfg(feature = "lex_bot_alias")]
pub mod lex_bot_alias;

#[cfg(feature = "lex_bot_alias")]
pub use lex_bot_alias::*;

#[cfg(feature = "lex_intent")]
pub mod lex_intent;

#[cfg(feature = "lex_intent")]
pub use lex_intent::*;

#[cfg(feature = "lex_slot_type")]
pub mod lex_slot_type;

#[cfg(feature = "lex_slot_type")]
pub use lex_slot_type::*;

#[cfg(feature = "licensemanager_association")]
pub mod licensemanager_association;

#[cfg(feature = "licensemanager_association")]
pub use licensemanager_association::*;

#[cfg(feature = "licensemanager_grant")]
pub mod licensemanager_grant;

#[cfg(feature = "licensemanager_grant")]
pub use licensemanager_grant::*;

#[cfg(feature = "licensemanager_grant_accepter")]
pub mod licensemanager_grant_accepter;

#[cfg(feature = "licensemanager_grant_accepter")]
pub use licensemanager_grant_accepter::*;

#[cfg(feature = "licensemanager_license_configuration")]
pub mod licensemanager_license_configuration;

#[cfg(feature = "licensemanager_license_configuration")]
pub use licensemanager_license_configuration::*;

#[cfg(feature = "lightsail_bucket")]
pub mod lightsail_bucket;

#[cfg(feature = "lightsail_bucket")]
pub use lightsail_bucket::*;

#[cfg(feature = "lightsail_bucket_access_key")]
pub mod lightsail_bucket_access_key;

#[cfg(feature = "lightsail_bucket_access_key")]
pub use lightsail_bucket_access_key::*;

#[cfg(feature = "lightsail_bucket_resource_access")]
pub mod lightsail_bucket_resource_access;

#[cfg(feature = "lightsail_bucket_resource_access")]
pub use lightsail_bucket_resource_access::*;

#[cfg(feature = "lightsail_certificate")]
pub mod lightsail_certificate;

#[cfg(feature = "lightsail_certificate")]
pub use lightsail_certificate::*;

#[cfg(feature = "lightsail_container_service")]
pub mod lightsail_container_service;

#[cfg(feature = "lightsail_container_service")]
pub use lightsail_container_service::*;

#[cfg(feature = "lightsail_container_service_deployment_version")]
pub mod lightsail_container_service_deployment_version;

#[cfg(feature = "lightsail_container_service_deployment_version")]
pub use lightsail_container_service_deployment_version::*;

#[cfg(feature = "lightsail_database")]
pub mod lightsail_database;

#[cfg(feature = "lightsail_database")]
pub use lightsail_database::*;

#[cfg(feature = "lightsail_disk")]
pub mod lightsail_disk;

#[cfg(feature = "lightsail_disk")]
pub use lightsail_disk::*;

#[cfg(feature = "lightsail_disk_attachment")]
pub mod lightsail_disk_attachment;

#[cfg(feature = "lightsail_disk_attachment")]
pub use lightsail_disk_attachment::*;

#[cfg(feature = "lightsail_distribution")]
pub mod lightsail_distribution;

#[cfg(feature = "lightsail_distribution")]
pub use lightsail_distribution::*;

#[cfg(feature = "lightsail_domain")]
pub mod lightsail_domain;

#[cfg(feature = "lightsail_domain")]
pub use lightsail_domain::*;

#[cfg(feature = "lightsail_domain_entry")]
pub mod lightsail_domain_entry;

#[cfg(feature = "lightsail_domain_entry")]
pub use lightsail_domain_entry::*;

#[cfg(feature = "lightsail_instance")]
pub mod lightsail_instance;

#[cfg(feature = "lightsail_instance")]
pub use lightsail_instance::*;

#[cfg(feature = "lightsail_instance_public_ports")]
pub mod lightsail_instance_public_ports;

#[cfg(feature = "lightsail_instance_public_ports")]
pub use lightsail_instance_public_ports::*;

#[cfg(feature = "lightsail_key_pair")]
pub mod lightsail_key_pair;

#[cfg(feature = "lightsail_key_pair")]
pub use lightsail_key_pair::*;

#[cfg(feature = "lightsail_lb")]
pub mod lightsail_lb;

#[cfg(feature = "lightsail_lb")]
pub use lightsail_lb::*;

#[cfg(feature = "lightsail_lb_attachment")]
pub mod lightsail_lb_attachment;

#[cfg(feature = "lightsail_lb_attachment")]
pub use lightsail_lb_attachment::*;

#[cfg(feature = "lightsail_lb_certificate")]
pub mod lightsail_lb_certificate;

#[cfg(feature = "lightsail_lb_certificate")]
pub use lightsail_lb_certificate::*;

#[cfg(feature = "lightsail_lb_certificate_attachment")]
pub mod lightsail_lb_certificate_attachment;

#[cfg(feature = "lightsail_lb_certificate_attachment")]
pub use lightsail_lb_certificate_attachment::*;

#[cfg(feature = "lightsail_lb_https_redirection_policy")]
pub mod lightsail_lb_https_redirection_policy;

#[cfg(feature = "lightsail_lb_https_redirection_policy")]
pub use lightsail_lb_https_redirection_policy::*;

#[cfg(feature = "lightsail_lb_stickiness_policy")]
pub mod lightsail_lb_stickiness_policy;

#[cfg(feature = "lightsail_lb_stickiness_policy")]
pub use lightsail_lb_stickiness_policy::*;

#[cfg(feature = "lightsail_static_ip")]
pub mod lightsail_static_ip;

#[cfg(feature = "lightsail_static_ip")]
pub use lightsail_static_ip::*;

#[cfg(feature = "lightsail_static_ip_attachment")]
pub mod lightsail_static_ip_attachment;

#[cfg(feature = "lightsail_static_ip_attachment")]
pub use lightsail_static_ip_attachment::*;

#[cfg(feature = "load_balancer_backend_server_policy")]
pub mod load_balancer_backend_server_policy;

#[cfg(feature = "load_balancer_backend_server_policy")]
pub use load_balancer_backend_server_policy::*;

#[cfg(feature = "load_balancer_listener_policy")]
pub mod load_balancer_listener_policy;

#[cfg(feature = "load_balancer_listener_policy")]
pub use load_balancer_listener_policy::*;

#[cfg(feature = "load_balancer_policy")]
pub mod load_balancer_policy;

#[cfg(feature = "load_balancer_policy")]
pub use load_balancer_policy::*;

#[cfg(feature = "location_geofence_collection")]
pub mod location_geofence_collection;

#[cfg(feature = "location_geofence_collection")]
pub use location_geofence_collection::*;

#[cfg(feature = "location_map")]
pub mod location_map;

#[cfg(feature = "location_map")]
pub use location_map::*;

#[cfg(feature = "location_place_index")]
pub mod location_place_index;

#[cfg(feature = "location_place_index")]
pub use location_place_index::*;

#[cfg(feature = "location_route_calculator")]
pub mod location_route_calculator;

#[cfg(feature = "location_route_calculator")]
pub use location_route_calculator::*;

#[cfg(feature = "location_tracker")]
pub mod location_tracker;

#[cfg(feature = "location_tracker")]
pub use location_tracker::*;

#[cfg(feature = "location_tracker_association")]
pub mod location_tracker_association;

#[cfg(feature = "location_tracker_association")]
pub use location_tracker_association::*;

#[cfg(feature = "m2_application")]
pub mod m2_application;

#[cfg(feature = "m2_application")]
pub use m2_application::*;

#[cfg(feature = "m2_deployment")]
pub mod m2_deployment;

#[cfg(feature = "m2_deployment")]
pub use m2_deployment::*;

#[cfg(feature = "m2_environment")]
pub mod m2_environment;

#[cfg(feature = "m2_environment")]
pub use m2_environment::*;

#[cfg(feature = "macie2_account")]
pub mod macie2_account;

#[cfg(feature = "macie2_account")]
pub use macie2_account::*;

#[cfg(feature = "macie2_classification_export_configuration")]
pub mod macie2_classification_export_configuration;

#[cfg(feature = "macie2_classification_export_configuration")]
pub use macie2_classification_export_configuration::*;

#[cfg(feature = "macie2_classification_job")]
pub mod macie2_classification_job;

#[cfg(feature = "macie2_classification_job")]
pub use macie2_classification_job::*;

#[cfg(feature = "macie2_custom_data_identifier")]
pub mod macie2_custom_data_identifier;

#[cfg(feature = "macie2_custom_data_identifier")]
pub use macie2_custom_data_identifier::*;

#[cfg(feature = "macie2_findings_filter")]
pub mod macie2_findings_filter;

#[cfg(feature = "macie2_findings_filter")]
pub use macie2_findings_filter::*;

#[cfg(feature = "macie2_invitation_accepter")]
pub mod macie2_invitation_accepter;

#[cfg(feature = "macie2_invitation_accepter")]
pub use macie2_invitation_accepter::*;

#[cfg(feature = "macie2_member")]
pub mod macie2_member;

#[cfg(feature = "macie2_member")]
pub use macie2_member::*;

#[cfg(feature = "macie2_organization_admin_account")]
pub mod macie2_organization_admin_account;

#[cfg(feature = "macie2_organization_admin_account")]
pub use macie2_organization_admin_account::*;

#[cfg(feature = "macie2_organization_configuration")]
pub mod macie2_organization_configuration;

#[cfg(feature = "macie2_organization_configuration")]
pub use macie2_organization_configuration::*;

#[cfg(feature = "main_route_table_association")]
pub mod main_route_table_association;

#[cfg(feature = "main_route_table_association")]
pub use main_route_table_association::*;

#[cfg(feature = "media_convert_queue")]
pub mod media_convert_queue;

#[cfg(feature = "media_convert_queue")]
pub use media_convert_queue::*;

#[cfg(feature = "media_package_channel")]
pub mod media_package_channel;

#[cfg(feature = "media_package_channel")]
pub use media_package_channel::*;

#[cfg(feature = "media_packagev2_channel_group")]
pub mod media_packagev2_channel_group;

#[cfg(feature = "media_packagev2_channel_group")]
pub use media_packagev2_channel_group::*;

#[cfg(feature = "media_store_container")]
pub mod media_store_container;

#[cfg(feature = "media_store_container")]
pub use media_store_container::*;

#[cfg(feature = "media_store_container_policy")]
pub mod media_store_container_policy;

#[cfg(feature = "media_store_container_policy")]
pub use media_store_container_policy::*;

#[cfg(feature = "memorydb_acl")]
pub mod memorydb_acl;

#[cfg(feature = "memorydb_acl")]
pub use memorydb_acl::*;

#[cfg(feature = "memorydb_cluster")]
pub mod memorydb_cluster;

#[cfg(feature = "memorydb_cluster")]
pub use memorydb_cluster::*;

#[cfg(feature = "memorydb_multi_region_cluster")]
pub mod memorydb_multi_region_cluster;

#[cfg(feature = "memorydb_multi_region_cluster")]
pub use memorydb_multi_region_cluster::*;

#[cfg(feature = "memorydb_parameter_group")]
pub mod memorydb_parameter_group;

#[cfg(feature = "memorydb_parameter_group")]
pub use memorydb_parameter_group::*;

#[cfg(feature = "memorydb_snapshot")]
pub mod memorydb_snapshot;

#[cfg(feature = "memorydb_snapshot")]
pub use memorydb_snapshot::*;

#[cfg(feature = "memorydb_subnet_group")]
pub mod memorydb_subnet_group;

#[cfg(feature = "memorydb_subnet_group")]
pub use memorydb_subnet_group::*;

#[cfg(feature = "memorydb_user")]
pub mod memorydb_user;

#[cfg(feature = "memorydb_user")]
pub use memorydb_user::*;

#[cfg(feature = "mq_broker")]
pub mod mq_broker;

#[cfg(feature = "mq_broker")]
pub use mq_broker::*;

#[cfg(feature = "mq_configuration")]
pub mod mq_configuration;

#[cfg(feature = "mq_configuration")]
pub use mq_configuration::*;

#[cfg(feature = "msk_cluster")]
pub mod msk_cluster;

#[cfg(feature = "msk_cluster")]
pub use msk_cluster::*;

#[cfg(feature = "msk_cluster_policy")]
pub mod msk_cluster_policy;

#[cfg(feature = "msk_cluster_policy")]
pub use msk_cluster_policy::*;

#[cfg(feature = "msk_configuration")]
pub mod msk_configuration;

#[cfg(feature = "msk_configuration")]
pub use msk_configuration::*;

#[cfg(feature = "msk_replicator")]
pub mod msk_replicator;

#[cfg(feature = "msk_replicator")]
pub use msk_replicator::*;

#[cfg(feature = "msk_scram_secret_association")]
pub mod msk_scram_secret_association;

#[cfg(feature = "msk_scram_secret_association")]
pub use msk_scram_secret_association::*;

#[cfg(feature = "msk_serverless_cluster")]
pub mod msk_serverless_cluster;

#[cfg(feature = "msk_serverless_cluster")]
pub use msk_serverless_cluster::*;

#[cfg(feature = "msk_single_scram_secret_association")]
pub mod msk_single_scram_secret_association;

#[cfg(feature = "msk_single_scram_secret_association")]
pub use msk_single_scram_secret_association::*;

#[cfg(feature = "msk_vpc_connection")]
pub mod msk_vpc_connection;

#[cfg(feature = "msk_vpc_connection")]
pub use msk_vpc_connection::*;

#[cfg(feature = "mskconnect_connector")]
pub mod mskconnect_connector;

#[cfg(feature = "mskconnect_connector")]
pub use mskconnect_connector::*;

#[cfg(feature = "mskconnect_custom_plugin")]
pub mod mskconnect_custom_plugin;

#[cfg(feature = "mskconnect_custom_plugin")]
pub use mskconnect_custom_plugin::*;

#[cfg(feature = "mskconnect_worker_configuration")]
pub mod mskconnect_worker_configuration;

#[cfg(feature = "mskconnect_worker_configuration")]
pub use mskconnect_worker_configuration::*;

#[cfg(feature = "mwaa_environment")]
pub mod mwaa_environment;

#[cfg(feature = "mwaa_environment")]
pub use mwaa_environment::*;

#[cfg(feature = "nat_gateway")]
pub mod nat_gateway;

#[cfg(feature = "nat_gateway")]
pub use nat_gateway::*;

#[cfg(feature = "nat_gateway_eip_association")]
pub mod nat_gateway_eip_association;

#[cfg(feature = "nat_gateway_eip_association")]
pub use nat_gateway_eip_association::*;

#[cfg(feature = "neptune_cluster")]
pub mod neptune_cluster;

#[cfg(feature = "neptune_cluster")]
pub use neptune_cluster::*;

#[cfg(feature = "neptune_cluster_endpoint")]
pub mod neptune_cluster_endpoint;

#[cfg(feature = "neptune_cluster_endpoint")]
pub use neptune_cluster_endpoint::*;

#[cfg(feature = "neptune_cluster_instance")]
pub mod neptune_cluster_instance;

#[cfg(feature = "neptune_cluster_instance")]
pub use neptune_cluster_instance::*;

#[cfg(feature = "neptune_cluster_parameter_group")]
pub mod neptune_cluster_parameter_group;

#[cfg(feature = "neptune_cluster_parameter_group")]
pub use neptune_cluster_parameter_group::*;

#[cfg(feature = "neptune_cluster_snapshot")]
pub mod neptune_cluster_snapshot;

#[cfg(feature = "neptune_cluster_snapshot")]
pub use neptune_cluster_snapshot::*;

#[cfg(feature = "neptune_event_subscription")]
pub mod neptune_event_subscription;

#[cfg(feature = "neptune_event_subscription")]
pub use neptune_event_subscription::*;

#[cfg(feature = "neptune_global_cluster")]
pub mod neptune_global_cluster;

#[cfg(feature = "neptune_global_cluster")]
pub use neptune_global_cluster::*;

#[cfg(feature = "neptune_parameter_group")]
pub mod neptune_parameter_group;

#[cfg(feature = "neptune_parameter_group")]
pub use neptune_parameter_group::*;

#[cfg(feature = "neptune_subnet_group")]
pub mod neptune_subnet_group;

#[cfg(feature = "neptune_subnet_group")]
pub use neptune_subnet_group::*;

#[cfg(feature = "neptunegraph_graph")]
pub mod neptunegraph_graph;

#[cfg(feature = "neptunegraph_graph")]
pub use neptunegraph_graph::*;

#[cfg(feature = "network_acl")]
pub mod network_acl;

#[cfg(feature = "network_acl")]
pub use network_acl::*;

#[cfg(feature = "network_acl_association")]
pub mod network_acl_association;

#[cfg(feature = "network_acl_association")]
pub use network_acl_association::*;

#[cfg(feature = "network_acl_rule")]
pub mod network_acl_rule;

#[cfg(feature = "network_acl_rule")]
pub use network_acl_rule::*;

#[cfg(feature = "network_interface")]
pub mod network_interface;

#[cfg(feature = "network_interface")]
pub use network_interface::*;

#[cfg(feature = "network_interface_attachment")]
pub mod network_interface_attachment;

#[cfg(feature = "network_interface_attachment")]
pub use network_interface_attachment::*;

#[cfg(feature = "network_interface_permission")]
pub mod network_interface_permission;

#[cfg(feature = "network_interface_permission")]
pub use network_interface_permission::*;

#[cfg(feature = "network_interface_sg_attachment")]
pub mod network_interface_sg_attachment;

#[cfg(feature = "network_interface_sg_attachment")]
pub use network_interface_sg_attachment::*;

#[cfg(feature = "networkfirewall_firewall")]
pub mod networkfirewall_firewall;

#[cfg(feature = "networkfirewall_firewall")]
pub use networkfirewall_firewall::*;

#[cfg(feature = "networkfirewall_firewall_policy")]
pub mod networkfirewall_firewall_policy;

#[cfg(feature = "networkfirewall_firewall_policy")]
pub use networkfirewall_firewall_policy::*;

#[cfg(feature = "networkfirewall_firewall_transit_gateway_attachment_accepter")]
pub mod networkfirewall_firewall_transit_gateway_attachment_accepter;

#[cfg(feature = "networkfirewall_firewall_transit_gateway_attachment_accepter")]
pub use networkfirewall_firewall_transit_gateway_attachment_accepter::*;

#[cfg(feature = "networkfirewall_logging_configuration")]
pub mod networkfirewall_logging_configuration;

#[cfg(feature = "networkfirewall_logging_configuration")]
pub use networkfirewall_logging_configuration::*;

#[cfg(feature = "networkfirewall_resource_policy")]
pub mod networkfirewall_resource_policy;

#[cfg(feature = "networkfirewall_resource_policy")]
pub use networkfirewall_resource_policy::*;

#[cfg(feature = "networkfirewall_rule_group")]
pub mod networkfirewall_rule_group;

#[cfg(feature = "networkfirewall_rule_group")]
pub use networkfirewall_rule_group::*;

#[cfg(feature = "networkfirewall_tls_inspection_configuration")]
pub mod networkfirewall_tls_inspection_configuration;

#[cfg(feature = "networkfirewall_tls_inspection_configuration")]
pub use networkfirewall_tls_inspection_configuration::*;

#[cfg(feature = "networkfirewall_vpc_endpoint_association")]
pub mod networkfirewall_vpc_endpoint_association;

#[cfg(feature = "networkfirewall_vpc_endpoint_association")]
pub use networkfirewall_vpc_endpoint_association::*;

#[cfg(feature = "networkflowmonitor_monitor")]
pub mod networkflowmonitor_monitor;

#[cfg(feature = "networkflowmonitor_monitor")]
pub use networkflowmonitor_monitor::*;

#[cfg(feature = "networkflowmonitor_scope")]
pub mod networkflowmonitor_scope;

#[cfg(feature = "networkflowmonitor_scope")]
pub use networkflowmonitor_scope::*;

#[cfg(feature = "networkmanager_attachment_accepter")]
pub mod networkmanager_attachment_accepter;

#[cfg(feature = "networkmanager_attachment_accepter")]
pub use networkmanager_attachment_accepter::*;

#[cfg(feature = "networkmanager_connect_attachment")]
pub mod networkmanager_connect_attachment;

#[cfg(feature = "networkmanager_connect_attachment")]
pub use networkmanager_connect_attachment::*;

#[cfg(feature = "networkmanager_connect_peer")]
pub mod networkmanager_connect_peer;

#[cfg(feature = "networkmanager_connect_peer")]
pub use networkmanager_connect_peer::*;

#[cfg(feature = "networkmanager_connection")]
pub mod networkmanager_connection;

#[cfg(feature = "networkmanager_connection")]
pub use networkmanager_connection::*;

#[cfg(feature = "networkmanager_core_network")]
pub mod networkmanager_core_network;

#[cfg(feature = "networkmanager_core_network")]
pub use networkmanager_core_network::*;

#[cfg(feature = "networkmanager_core_network_policy_attachment")]
pub mod networkmanager_core_network_policy_attachment;

#[cfg(feature = "networkmanager_core_network_policy_attachment")]
pub use networkmanager_core_network_policy_attachment::*;

#[cfg(feature = "networkmanager_customer_gateway_association")]
pub mod networkmanager_customer_gateway_association;

#[cfg(feature = "networkmanager_customer_gateway_association")]
pub use networkmanager_customer_gateway_association::*;

#[cfg(feature = "networkmanager_device")]
pub mod networkmanager_device;

#[cfg(feature = "networkmanager_device")]
pub use networkmanager_device::*;

#[cfg(feature = "networkmanager_dx_gateway_attachment")]
pub mod networkmanager_dx_gateway_attachment;

#[cfg(feature = "networkmanager_dx_gateway_attachment")]
pub use networkmanager_dx_gateway_attachment::*;

#[cfg(feature = "networkmanager_global_network")]
pub mod networkmanager_global_network;

#[cfg(feature = "networkmanager_global_network")]
pub use networkmanager_global_network::*;

#[cfg(feature = "networkmanager_link")]
pub mod networkmanager_link;

#[cfg(feature = "networkmanager_link")]
pub use networkmanager_link::*;

#[cfg(feature = "networkmanager_link_association")]
pub mod networkmanager_link_association;

#[cfg(feature = "networkmanager_link_association")]
pub use networkmanager_link_association::*;

#[cfg(feature = "networkmanager_site")]
pub mod networkmanager_site;

#[cfg(feature = "networkmanager_site")]
pub use networkmanager_site::*;

#[cfg(feature = "networkmanager_site_to_site_vpn_attachment")]
pub mod networkmanager_site_to_site_vpn_attachment;

#[cfg(feature = "networkmanager_site_to_site_vpn_attachment")]
pub use networkmanager_site_to_site_vpn_attachment::*;

#[cfg(feature = "networkmanager_transit_gateway_connect_peer_association")]
pub mod networkmanager_transit_gateway_connect_peer_association;

#[cfg(feature = "networkmanager_transit_gateway_connect_peer_association")]
pub use networkmanager_transit_gateway_connect_peer_association::*;

#[cfg(feature = "networkmanager_transit_gateway_peering")]
pub mod networkmanager_transit_gateway_peering;

#[cfg(feature = "networkmanager_transit_gateway_peering")]
pub use networkmanager_transit_gateway_peering::*;

#[cfg(feature = "networkmanager_transit_gateway_registration")]
pub mod networkmanager_transit_gateway_registration;

#[cfg(feature = "networkmanager_transit_gateway_registration")]
pub use networkmanager_transit_gateway_registration::*;

#[cfg(feature = "networkmanager_transit_gateway_route_table_attachment")]
pub mod networkmanager_transit_gateway_route_table_attachment;

#[cfg(feature = "networkmanager_transit_gateway_route_table_attachment")]
pub use networkmanager_transit_gateway_route_table_attachment::*;

#[cfg(feature = "networkmanager_vpc_attachment")]
pub mod networkmanager_vpc_attachment;

#[cfg(feature = "networkmanager_vpc_attachment")]
pub use networkmanager_vpc_attachment::*;

#[cfg(feature = "networkmonitor_monitor")]
pub mod networkmonitor_monitor;

#[cfg(feature = "networkmonitor_monitor")]
pub use networkmonitor_monitor::*;

#[cfg(feature = "networkmonitor_probe")]
pub mod networkmonitor_probe;

#[cfg(feature = "networkmonitor_probe")]
pub use networkmonitor_probe::*;

#[cfg(feature = "notifications_channel_association")]
pub mod notifications_channel_association;

#[cfg(feature = "notifications_channel_association")]
pub use notifications_channel_association::*;

#[cfg(feature = "notifications_event_rule")]
pub mod notifications_event_rule;

#[cfg(feature = "notifications_event_rule")]
pub use notifications_event_rule::*;

#[cfg(feature = "notifications_notification_configuration")]
pub mod notifications_notification_configuration;

#[cfg(feature = "notifications_notification_configuration")]
pub use notifications_notification_configuration::*;

#[cfg(feature = "notifications_notification_hub")]
pub mod notifications_notification_hub;

#[cfg(feature = "notifications_notification_hub")]
pub use notifications_notification_hub::*;

#[cfg(feature = "notificationscontacts_email_contact")]
pub mod notificationscontacts_email_contact;

#[cfg(feature = "notificationscontacts_email_contact")]
pub use notificationscontacts_email_contact::*;

#[cfg(feature = "oam_link")]
pub mod oam_link;

#[cfg(feature = "oam_link")]
pub use oam_link::*;

#[cfg(feature = "oam_sink")]
pub mod oam_sink;

#[cfg(feature = "oam_sink")]
pub use oam_sink::*;

#[cfg(feature = "oam_sink_policy")]
pub mod oam_sink_policy;

#[cfg(feature = "oam_sink_policy")]
pub use oam_sink_policy::*;

#[cfg(feature = "observabilityadmin_centralization_rule_for_organization")]
pub mod observabilityadmin_centralization_rule_for_organization;

#[cfg(feature = "observabilityadmin_centralization_rule_for_organization")]
pub use observabilityadmin_centralization_rule_for_organization::*;

#[cfg(feature = "odb_cloud_autonomous_vm_cluster")]
pub mod odb_cloud_autonomous_vm_cluster;

#[cfg(feature = "odb_cloud_autonomous_vm_cluster")]
pub use odb_cloud_autonomous_vm_cluster::*;

#[cfg(feature = "odb_cloud_exadata_infrastructure")]
pub mod odb_cloud_exadata_infrastructure;

#[cfg(feature = "odb_cloud_exadata_infrastructure")]
pub use odb_cloud_exadata_infrastructure::*;

#[cfg(feature = "odb_cloud_vm_cluster")]
pub mod odb_cloud_vm_cluster;

#[cfg(feature = "odb_cloud_vm_cluster")]
pub use odb_cloud_vm_cluster::*;

#[cfg(feature = "odb_network")]
pub mod odb_network;

#[cfg(feature = "odb_network")]
pub use odb_network::*;

#[cfg(feature = "odb_network_peering_connection")]
pub mod odb_network_peering_connection;

#[cfg(feature = "odb_network_peering_connection")]
pub use odb_network_peering_connection::*;

#[cfg(feature = "opensearch_authorize_vpc_endpoint_access")]
pub mod opensearch_authorize_vpc_endpoint_access;

#[cfg(feature = "opensearch_authorize_vpc_endpoint_access")]
pub use opensearch_authorize_vpc_endpoint_access::*;

#[cfg(feature = "opensearch_domain")]
pub mod opensearch_domain;

#[cfg(feature = "opensearch_domain")]
pub use opensearch_domain::*;

#[cfg(feature = "opensearch_domain_policy")]
pub mod opensearch_domain_policy;

#[cfg(feature = "opensearch_domain_policy")]
pub use opensearch_domain_policy::*;

#[cfg(feature = "opensearch_domain_saml_options")]
pub mod opensearch_domain_saml_options;

#[cfg(feature = "opensearch_domain_saml_options")]
pub use opensearch_domain_saml_options::*;

#[cfg(feature = "opensearch_inbound_connection_accepter")]
pub mod opensearch_inbound_connection_accepter;

#[cfg(feature = "opensearch_inbound_connection_accepter")]
pub use opensearch_inbound_connection_accepter::*;

#[cfg(feature = "opensearch_outbound_connection")]
pub mod opensearch_outbound_connection;

#[cfg(feature = "opensearch_outbound_connection")]
pub use opensearch_outbound_connection::*;

#[cfg(feature = "opensearch_package")]
pub mod opensearch_package;

#[cfg(feature = "opensearch_package")]
pub use opensearch_package::*;

#[cfg(feature = "opensearch_package_association")]
pub mod opensearch_package_association;

#[cfg(feature = "opensearch_package_association")]
pub use opensearch_package_association::*;

#[cfg(feature = "opensearch_vpc_endpoint")]
pub mod opensearch_vpc_endpoint;

#[cfg(feature = "opensearch_vpc_endpoint")]
pub use opensearch_vpc_endpoint::*;

#[cfg(feature = "opensearchserverless_access_policy")]
pub mod opensearchserverless_access_policy;

#[cfg(feature = "opensearchserverless_access_policy")]
pub use opensearchserverless_access_policy::*;

#[cfg(feature = "opensearchserverless_collection")]
pub mod opensearchserverless_collection;

#[cfg(feature = "opensearchserverless_collection")]
pub use opensearchserverless_collection::*;

#[cfg(feature = "opensearchserverless_lifecycle_policy")]
pub mod opensearchserverless_lifecycle_policy;

#[cfg(feature = "opensearchserverless_lifecycle_policy")]
pub use opensearchserverless_lifecycle_policy::*;

#[cfg(feature = "opensearchserverless_security_config")]
pub mod opensearchserverless_security_config;

#[cfg(feature = "opensearchserverless_security_config")]
pub use opensearchserverless_security_config::*;

#[cfg(feature = "opensearchserverless_security_policy")]
pub mod opensearchserverless_security_policy;

#[cfg(feature = "opensearchserverless_security_policy")]
pub use opensearchserverless_security_policy::*;

#[cfg(feature = "opensearchserverless_vpc_endpoint")]
pub mod opensearchserverless_vpc_endpoint;

#[cfg(feature = "opensearchserverless_vpc_endpoint")]
pub use opensearchserverless_vpc_endpoint::*;

#[cfg(feature = "organizations_account")]
pub mod organizations_account;

#[cfg(feature = "organizations_account")]
pub use organizations_account::*;

#[cfg(feature = "organizations_delegated_administrator")]
pub mod organizations_delegated_administrator;

#[cfg(feature = "organizations_delegated_administrator")]
pub use organizations_delegated_administrator::*;

#[cfg(feature = "organizations_organization")]
pub mod organizations_organization;

#[cfg(feature = "organizations_organization")]
pub use organizations_organization::*;

#[cfg(feature = "organizations_organizational_unit")]
pub mod organizations_organizational_unit;

#[cfg(feature = "organizations_organizational_unit")]
pub use organizations_organizational_unit::*;

#[cfg(feature = "organizations_policy")]
pub mod organizations_policy;

#[cfg(feature = "organizations_policy")]
pub use organizations_policy::*;

#[cfg(feature = "organizations_policy_attachment")]
pub mod organizations_policy_attachment;

#[cfg(feature = "organizations_policy_attachment")]
pub use organizations_policy_attachment::*;

#[cfg(feature = "organizations_resource_policy")]
pub mod organizations_resource_policy;

#[cfg(feature = "organizations_resource_policy")]
pub use organizations_resource_policy::*;

#[cfg(feature = "osis_pipeline")]
pub mod osis_pipeline;

#[cfg(feature = "osis_pipeline")]
pub use osis_pipeline::*;

#[cfg(feature = "paymentcryptography_key")]
pub mod paymentcryptography_key;

#[cfg(feature = "paymentcryptography_key")]
pub use paymentcryptography_key::*;

#[cfg(feature = "paymentcryptography_key_alias")]
pub mod paymentcryptography_key_alias;

#[cfg(feature = "paymentcryptography_key_alias")]
pub use paymentcryptography_key_alias::*;

#[cfg(feature = "pinpoint_adm_channel")]
pub mod pinpoint_adm_channel;

#[cfg(feature = "pinpoint_adm_channel")]
pub use pinpoint_adm_channel::*;

#[cfg(feature = "pinpoint_apns_channel")]
pub mod pinpoint_apns_channel;

#[cfg(feature = "pinpoint_apns_channel")]
pub use pinpoint_apns_channel::*;

#[cfg(feature = "pinpoint_apns_sandbox_channel")]
pub mod pinpoint_apns_sandbox_channel;

#[cfg(feature = "pinpoint_apns_sandbox_channel")]
pub use pinpoint_apns_sandbox_channel::*;

#[cfg(feature = "pinpoint_apns_voip_channel")]
pub mod pinpoint_apns_voip_channel;

#[cfg(feature = "pinpoint_apns_voip_channel")]
pub use pinpoint_apns_voip_channel::*;

#[cfg(feature = "pinpoint_apns_voip_sandbox_channel")]
pub mod pinpoint_apns_voip_sandbox_channel;

#[cfg(feature = "pinpoint_apns_voip_sandbox_channel")]
pub use pinpoint_apns_voip_sandbox_channel::*;

#[cfg(feature = "pinpoint_app")]
pub mod pinpoint_app;

#[cfg(feature = "pinpoint_app")]
pub use pinpoint_app::*;

#[cfg(feature = "pinpoint_baidu_channel")]
pub mod pinpoint_baidu_channel;

#[cfg(feature = "pinpoint_baidu_channel")]
pub use pinpoint_baidu_channel::*;

#[cfg(feature = "pinpoint_email_channel")]
pub mod pinpoint_email_channel;

#[cfg(feature = "pinpoint_email_channel")]
pub use pinpoint_email_channel::*;

#[cfg(feature = "pinpoint_email_template")]
pub mod pinpoint_email_template;

#[cfg(feature = "pinpoint_email_template")]
pub use pinpoint_email_template::*;

#[cfg(feature = "pinpoint_event_stream")]
pub mod pinpoint_event_stream;

#[cfg(feature = "pinpoint_event_stream")]
pub use pinpoint_event_stream::*;

#[cfg(feature = "pinpoint_gcm_channel")]
pub mod pinpoint_gcm_channel;

#[cfg(feature = "pinpoint_gcm_channel")]
pub use pinpoint_gcm_channel::*;

#[cfg(feature = "pinpoint_sms_channel")]
pub mod pinpoint_sms_channel;

#[cfg(feature = "pinpoint_sms_channel")]
pub use pinpoint_sms_channel::*;

#[cfg(feature = "pinpointsmsvoicev2_configuration_set")]
pub mod pinpointsmsvoicev2_configuration_set;

#[cfg(feature = "pinpointsmsvoicev2_configuration_set")]
pub use pinpointsmsvoicev2_configuration_set::*;

#[cfg(feature = "pinpointsmsvoicev2_opt_out_list")]
pub mod pinpointsmsvoicev2_opt_out_list;

#[cfg(feature = "pinpointsmsvoicev2_opt_out_list")]
pub use pinpointsmsvoicev2_opt_out_list::*;

#[cfg(feature = "pinpointsmsvoicev2_phone_number")]
pub mod pinpointsmsvoicev2_phone_number;

#[cfg(feature = "pinpointsmsvoicev2_phone_number")]
pub use pinpointsmsvoicev2_phone_number::*;

#[cfg(feature = "pipes_pipe")]
pub mod pipes_pipe;

#[cfg(feature = "pipes_pipe")]
pub use pipes_pipe::*;

#[cfg(feature = "placement_group")]
pub mod placement_group;

#[cfg(feature = "placement_group")]
pub use placement_group::*;

#[cfg(feature = "prometheus_alert_manager_definition")]
pub mod prometheus_alert_manager_definition;

#[cfg(feature = "prometheus_alert_manager_definition")]
pub use prometheus_alert_manager_definition::*;

#[cfg(feature = "prometheus_query_logging_configuration")]
pub mod prometheus_query_logging_configuration;

#[cfg(feature = "prometheus_query_logging_configuration")]
pub use prometheus_query_logging_configuration::*;

#[cfg(feature = "prometheus_resource_policy")]
pub mod prometheus_resource_policy;

#[cfg(feature = "prometheus_resource_policy")]
pub use prometheus_resource_policy::*;

#[cfg(feature = "prometheus_rule_group_namespace")]
pub mod prometheus_rule_group_namespace;

#[cfg(feature = "prometheus_rule_group_namespace")]
pub use prometheus_rule_group_namespace::*;

#[cfg(feature = "prometheus_scraper")]
pub mod prometheus_scraper;

#[cfg(feature = "prometheus_scraper")]
pub use prometheus_scraper::*;

#[cfg(feature = "prometheus_workspace")]
pub mod prometheus_workspace;

#[cfg(feature = "prometheus_workspace")]
pub use prometheus_workspace::*;

#[cfg(feature = "prometheus_workspace_configuration")]
pub mod prometheus_workspace_configuration;

#[cfg(feature = "prometheus_workspace_configuration")]
pub use prometheus_workspace_configuration::*;

#[cfg(feature = "proxy_protocol_policy")]
pub mod proxy_protocol_policy;

#[cfg(feature = "proxy_protocol_policy")]
pub use proxy_protocol_policy::*;

#[cfg(feature = "qbusiness_application")]
pub mod qbusiness_application;

#[cfg(feature = "qbusiness_application")]
pub use qbusiness_application::*;

#[cfg(feature = "qldb_ledger")]
pub mod qldb_ledger;

#[cfg(feature = "qldb_ledger")]
pub use qldb_ledger::*;

#[cfg(feature = "qldb_stream")]
pub mod qldb_stream;

#[cfg(feature = "qldb_stream")]
pub use qldb_stream::*;

#[cfg(feature = "ram_principal_association")]
pub mod ram_principal_association;

#[cfg(feature = "ram_principal_association")]
pub use ram_principal_association::*;

#[cfg(feature = "ram_resource_association")]
pub mod ram_resource_association;

#[cfg(feature = "ram_resource_association")]
pub use ram_resource_association::*;

#[cfg(feature = "ram_resource_share")]
pub mod ram_resource_share;

#[cfg(feature = "ram_resource_share")]
pub use ram_resource_share::*;

#[cfg(feature = "ram_resource_share_accepter")]
pub mod ram_resource_share_accepter;

#[cfg(feature = "ram_resource_share_accepter")]
pub use ram_resource_share_accepter::*;

#[cfg(feature = "ram_sharing_with_organization")]
pub mod ram_sharing_with_organization;

#[cfg(feature = "ram_sharing_with_organization")]
pub use ram_sharing_with_organization::*;

#[cfg(feature = "rbin_rule")]
pub mod rbin_rule;

#[cfg(feature = "rbin_rule")]
pub use rbin_rule::*;

#[cfg(feature = "rds_certificate")]
pub mod rds_certificate;

#[cfg(feature = "rds_certificate")]
pub use rds_certificate::*;

#[cfg(feature = "rds_cluster")]
pub mod rds_cluster;

#[cfg(feature = "rds_cluster")]
pub use rds_cluster::*;

#[cfg(feature = "rds_cluster_activity_stream")]
pub mod rds_cluster_activity_stream;

#[cfg(feature = "rds_cluster_activity_stream")]
pub use rds_cluster_activity_stream::*;

#[cfg(feature = "rds_cluster_endpoint")]
pub mod rds_cluster_endpoint;

#[cfg(feature = "rds_cluster_endpoint")]
pub use rds_cluster_endpoint::*;

#[cfg(feature = "rds_cluster_instance")]
pub mod rds_cluster_instance;

#[cfg(feature = "rds_cluster_instance")]
pub use rds_cluster_instance::*;

#[cfg(feature = "rds_cluster_parameter_group")]
pub mod rds_cluster_parameter_group;

#[cfg(feature = "rds_cluster_parameter_group")]
pub use rds_cluster_parameter_group::*;

#[cfg(feature = "rds_cluster_role_association")]
pub mod rds_cluster_role_association;

#[cfg(feature = "rds_cluster_role_association")]
pub use rds_cluster_role_association::*;

#[cfg(feature = "rds_cluster_snapshot_copy")]
pub mod rds_cluster_snapshot_copy;

#[cfg(feature = "rds_cluster_snapshot_copy")]
pub use rds_cluster_snapshot_copy::*;

#[cfg(feature = "rds_custom_db_engine_version")]
pub mod rds_custom_db_engine_version;

#[cfg(feature = "rds_custom_db_engine_version")]
pub use rds_custom_db_engine_version::*;

#[cfg(feature = "rds_export_task")]
pub mod rds_export_task;

#[cfg(feature = "rds_export_task")]
pub use rds_export_task::*;

#[cfg(feature = "rds_global_cluster")]
pub mod rds_global_cluster;

#[cfg(feature = "rds_global_cluster")]
pub use rds_global_cluster::*;

#[cfg(feature = "rds_instance_state")]
pub mod rds_instance_state;

#[cfg(feature = "rds_instance_state")]
pub use rds_instance_state::*;

#[cfg(feature = "rds_integration")]
pub mod rds_integration;

#[cfg(feature = "rds_integration")]
pub use rds_integration::*;

#[cfg(feature = "rds_reserved_instance")]
pub mod rds_reserved_instance;

#[cfg(feature = "rds_reserved_instance")]
pub use rds_reserved_instance::*;

#[cfg(feature = "rds_shard_group")]
pub mod rds_shard_group;

#[cfg(feature = "rds_shard_group")]
pub use rds_shard_group::*;

#[cfg(feature = "redshift_authentication_profile")]
pub mod redshift_authentication_profile;

#[cfg(feature = "redshift_authentication_profile")]
pub use redshift_authentication_profile::*;

#[cfg(feature = "redshift_cluster")]
pub mod redshift_cluster;

#[cfg(feature = "redshift_cluster")]
pub use redshift_cluster::*;

#[cfg(feature = "redshift_cluster_iam_roles")]
pub mod redshift_cluster_iam_roles;

#[cfg(feature = "redshift_cluster_iam_roles")]
pub use redshift_cluster_iam_roles::*;

#[cfg(feature = "redshift_cluster_snapshot")]
pub mod redshift_cluster_snapshot;

#[cfg(feature = "redshift_cluster_snapshot")]
pub use redshift_cluster_snapshot::*;

#[cfg(feature = "redshift_data_share_authorization")]
pub mod redshift_data_share_authorization;

#[cfg(feature = "redshift_data_share_authorization")]
pub use redshift_data_share_authorization::*;

#[cfg(feature = "redshift_data_share_consumer_association")]
pub mod redshift_data_share_consumer_association;

#[cfg(feature = "redshift_data_share_consumer_association")]
pub use redshift_data_share_consumer_association::*;

#[cfg(feature = "redshift_endpoint_access")]
pub mod redshift_endpoint_access;

#[cfg(feature = "redshift_endpoint_access")]
pub use redshift_endpoint_access::*;

#[cfg(feature = "redshift_endpoint_authorization")]
pub mod redshift_endpoint_authorization;

#[cfg(feature = "redshift_endpoint_authorization")]
pub use redshift_endpoint_authorization::*;

#[cfg(feature = "redshift_event_subscription")]
pub mod redshift_event_subscription;

#[cfg(feature = "redshift_event_subscription")]
pub use redshift_event_subscription::*;

#[cfg(feature = "redshift_hsm_client_certificate")]
pub mod redshift_hsm_client_certificate;

#[cfg(feature = "redshift_hsm_client_certificate")]
pub use redshift_hsm_client_certificate::*;

#[cfg(feature = "redshift_hsm_configuration")]
pub mod redshift_hsm_configuration;

#[cfg(feature = "redshift_hsm_configuration")]
pub use redshift_hsm_configuration::*;

#[cfg(feature = "redshift_integration")]
pub mod redshift_integration;

#[cfg(feature = "redshift_integration")]
pub use redshift_integration::*;

#[cfg(feature = "redshift_logging")]
pub mod redshift_logging;

#[cfg(feature = "redshift_logging")]
pub use redshift_logging::*;

#[cfg(feature = "redshift_parameter_group")]
pub mod redshift_parameter_group;

#[cfg(feature = "redshift_parameter_group")]
pub use redshift_parameter_group::*;

#[cfg(feature = "redshift_partner")]
pub mod redshift_partner;

#[cfg(feature = "redshift_partner")]
pub use redshift_partner::*;

#[cfg(feature = "redshift_resource_policy")]
pub mod redshift_resource_policy;

#[cfg(feature = "redshift_resource_policy")]
pub use redshift_resource_policy::*;

#[cfg(feature = "redshift_scheduled_action")]
pub mod redshift_scheduled_action;

#[cfg(feature = "redshift_scheduled_action")]
pub use redshift_scheduled_action::*;

#[cfg(feature = "redshift_snapshot_copy")]
pub mod redshift_snapshot_copy;

#[cfg(feature = "redshift_snapshot_copy")]
pub use redshift_snapshot_copy::*;

#[cfg(feature = "redshift_snapshot_copy_grant")]
pub mod redshift_snapshot_copy_grant;

#[cfg(feature = "redshift_snapshot_copy_grant")]
pub use redshift_snapshot_copy_grant::*;

#[cfg(feature = "redshift_snapshot_schedule")]
pub mod redshift_snapshot_schedule;

#[cfg(feature = "redshift_snapshot_schedule")]
pub use redshift_snapshot_schedule::*;

#[cfg(feature = "redshift_snapshot_schedule_association")]
pub mod redshift_snapshot_schedule_association;

#[cfg(feature = "redshift_snapshot_schedule_association")]
pub use redshift_snapshot_schedule_association::*;

#[cfg(feature = "redshift_subnet_group")]
pub mod redshift_subnet_group;

#[cfg(feature = "redshift_subnet_group")]
pub use redshift_subnet_group::*;

#[cfg(feature = "redshift_usage_limit")]
pub mod redshift_usage_limit;

#[cfg(feature = "redshift_usage_limit")]
pub use redshift_usage_limit::*;

#[cfg(feature = "redshiftdata_statement")]
pub mod redshiftdata_statement;

#[cfg(feature = "redshiftdata_statement")]
pub use redshiftdata_statement::*;

#[cfg(feature = "redshiftserverless_custom_domain_association")]
pub mod redshiftserverless_custom_domain_association;

#[cfg(feature = "redshiftserverless_custom_domain_association")]
pub use redshiftserverless_custom_domain_association::*;

#[cfg(feature = "redshiftserverless_endpoint_access")]
pub mod redshiftserverless_endpoint_access;

#[cfg(feature = "redshiftserverless_endpoint_access")]
pub use redshiftserverless_endpoint_access::*;

#[cfg(feature = "redshiftserverless_namespace")]
pub mod redshiftserverless_namespace;

#[cfg(feature = "redshiftserverless_namespace")]
pub use redshiftserverless_namespace::*;

#[cfg(feature = "redshiftserverless_resource_policy")]
pub mod redshiftserverless_resource_policy;

#[cfg(feature = "redshiftserverless_resource_policy")]
pub use redshiftserverless_resource_policy::*;

#[cfg(feature = "redshiftserverless_snapshot")]
pub mod redshiftserverless_snapshot;

#[cfg(feature = "redshiftserverless_snapshot")]
pub use redshiftserverless_snapshot::*;

#[cfg(feature = "redshiftserverless_usage_limit")]
pub mod redshiftserverless_usage_limit;

#[cfg(feature = "redshiftserverless_usage_limit")]
pub use redshiftserverless_usage_limit::*;

#[cfg(feature = "redshiftserverless_workgroup")]
pub mod redshiftserverless_workgroup;

#[cfg(feature = "redshiftserverless_workgroup")]
pub use redshiftserverless_workgroup::*;

#[cfg(feature = "rekognition_collection")]
pub mod rekognition_collection;

#[cfg(feature = "rekognition_collection")]
pub use rekognition_collection::*;

#[cfg(feature = "rekognition_project")]
pub mod rekognition_project;

#[cfg(feature = "rekognition_project")]
pub use rekognition_project::*;

#[cfg(feature = "rekognition_stream_processor")]
pub mod rekognition_stream_processor;

#[cfg(feature = "rekognition_stream_processor")]
pub use rekognition_stream_processor::*;

#[cfg(feature = "resiliencehub_resiliency_policy")]
pub mod resiliencehub_resiliency_policy;

#[cfg(feature = "resiliencehub_resiliency_policy")]
pub use resiliencehub_resiliency_policy::*;

#[cfg(feature = "resourceexplorer2_index")]
pub mod resourceexplorer2_index;

#[cfg(feature = "resourceexplorer2_index")]
pub use resourceexplorer2_index::*;

#[cfg(feature = "resourceexplorer2_view")]
pub mod resourceexplorer2_view;

#[cfg(feature = "resourceexplorer2_view")]
pub use resourceexplorer2_view::*;

#[cfg(feature = "resourcegroups_group")]
pub mod resourcegroups_group;

#[cfg(feature = "resourcegroups_group")]
pub use resourcegroups_group::*;

#[cfg(feature = "resourcegroups_resource")]
pub mod resourcegroups_resource;

#[cfg(feature = "resourcegroups_resource")]
pub use resourcegroups_resource::*;

#[cfg(feature = "rolesanywhere_profile")]
pub mod rolesanywhere_profile;

#[cfg(feature = "rolesanywhere_profile")]
pub use rolesanywhere_profile::*;

#[cfg(feature = "rolesanywhere_trust_anchor")]
pub mod rolesanywhere_trust_anchor;

#[cfg(feature = "rolesanywhere_trust_anchor")]
pub use rolesanywhere_trust_anchor::*;

#[cfg(feature = "route")]
pub mod route;

#[cfg(feature = "route")]
pub use route::*;

#[cfg(feature = "route53_cidr_collection")]
pub mod route53_cidr_collection;

#[cfg(feature = "route53_cidr_collection")]
pub use route53_cidr_collection::*;

#[cfg(feature = "route53_cidr_location")]
pub mod route53_cidr_location;

#[cfg(feature = "route53_cidr_location")]
pub use route53_cidr_location::*;

#[cfg(feature = "route53_delegation_set")]
pub mod route53_delegation_set;

#[cfg(feature = "route53_delegation_set")]
pub use route53_delegation_set::*;

#[cfg(feature = "route53_health_check")]
pub mod route53_health_check;

#[cfg(feature = "route53_health_check")]
pub use route53_health_check::*;

#[cfg(feature = "route53_hosted_zone_dnssec")]
pub mod route53_hosted_zone_dnssec;

#[cfg(feature = "route53_hosted_zone_dnssec")]
pub use route53_hosted_zone_dnssec::*;

#[cfg(feature = "route53_key_signing_key")]
pub mod route53_key_signing_key;

#[cfg(feature = "route53_key_signing_key")]
pub use route53_key_signing_key::*;

#[cfg(feature = "route53_query_log")]
pub mod route53_query_log;

#[cfg(feature = "route53_query_log")]
pub use route53_query_log::*;

#[cfg(feature = "route53_record")]
pub mod route53_record;

#[cfg(feature = "route53_record")]
pub use route53_record::*;

#[cfg(feature = "route53_records_exclusive")]
pub mod route53_records_exclusive;

#[cfg(feature = "route53_records_exclusive")]
pub use route53_records_exclusive::*;

#[cfg(feature = "route53_resolver_config")]
pub mod route53_resolver_config;

#[cfg(feature = "route53_resolver_config")]
pub use route53_resolver_config::*;

#[cfg(feature = "route53_resolver_dnssec_config")]
pub mod route53_resolver_dnssec_config;

#[cfg(feature = "route53_resolver_dnssec_config")]
pub use route53_resolver_dnssec_config::*;

#[cfg(feature = "route53_resolver_endpoint")]
pub mod route53_resolver_endpoint;

#[cfg(feature = "route53_resolver_endpoint")]
pub use route53_resolver_endpoint::*;

#[cfg(feature = "route53_resolver_firewall_config")]
pub mod route53_resolver_firewall_config;

#[cfg(feature = "route53_resolver_firewall_config")]
pub use route53_resolver_firewall_config::*;

#[cfg(feature = "route53_resolver_firewall_domain_list")]
pub mod route53_resolver_firewall_domain_list;

#[cfg(feature = "route53_resolver_firewall_domain_list")]
pub use route53_resolver_firewall_domain_list::*;

#[cfg(feature = "route53_resolver_firewall_rule")]
pub mod route53_resolver_firewall_rule;

#[cfg(feature = "route53_resolver_firewall_rule")]
pub use route53_resolver_firewall_rule::*;

#[cfg(feature = "route53_resolver_firewall_rule_group")]
pub mod route53_resolver_firewall_rule_group;

#[cfg(feature = "route53_resolver_firewall_rule_group")]
pub use route53_resolver_firewall_rule_group::*;

#[cfg(feature = "route53_resolver_firewall_rule_group_association")]
pub mod route53_resolver_firewall_rule_group_association;

#[cfg(feature = "route53_resolver_firewall_rule_group_association")]
pub use route53_resolver_firewall_rule_group_association::*;

#[cfg(feature = "route53_resolver_query_log_config")]
pub mod route53_resolver_query_log_config;

#[cfg(feature = "route53_resolver_query_log_config")]
pub use route53_resolver_query_log_config::*;

#[cfg(feature = "route53_resolver_query_log_config_association")]
pub mod route53_resolver_query_log_config_association;

#[cfg(feature = "route53_resolver_query_log_config_association")]
pub use route53_resolver_query_log_config_association::*;

#[cfg(feature = "route53_resolver_rule")]
pub mod route53_resolver_rule;

#[cfg(feature = "route53_resolver_rule")]
pub use route53_resolver_rule::*;

#[cfg(feature = "route53_resolver_rule_association")]
pub mod route53_resolver_rule_association;

#[cfg(feature = "route53_resolver_rule_association")]
pub use route53_resolver_rule_association::*;

#[cfg(feature = "route53_traffic_policy")]
pub mod route53_traffic_policy;

#[cfg(feature = "route53_traffic_policy")]
pub use route53_traffic_policy::*;

#[cfg(feature = "route53_traffic_policy_instance")]
pub mod route53_traffic_policy_instance;

#[cfg(feature = "route53_traffic_policy_instance")]
pub use route53_traffic_policy_instance::*;

#[cfg(feature = "route53_vpc_association_authorization")]
pub mod route53_vpc_association_authorization;

#[cfg(feature = "route53_vpc_association_authorization")]
pub use route53_vpc_association_authorization::*;

#[cfg(feature = "route53_zone")]
pub mod route53_zone;

#[cfg(feature = "route53_zone")]
pub use route53_zone::*;

#[cfg(feature = "route53_zone_association")]
pub mod route53_zone_association;

#[cfg(feature = "route53_zone_association")]
pub use route53_zone_association::*;

#[cfg(feature = "route53domains_delegation_signer_record")]
pub mod route53domains_delegation_signer_record;

#[cfg(feature = "route53domains_delegation_signer_record")]
pub use route53domains_delegation_signer_record::*;

#[cfg(feature = "route53domains_domain")]
pub mod route53domains_domain;

#[cfg(feature = "route53domains_domain")]
pub use route53domains_domain::*;

#[cfg(feature = "route53domains_registered_domain")]
pub mod route53domains_registered_domain;

#[cfg(feature = "route53domains_registered_domain")]
pub use route53domains_registered_domain::*;

#[cfg(feature = "route53profiles_association")]
pub mod route53profiles_association;

#[cfg(feature = "route53profiles_association")]
pub use route53profiles_association::*;

#[cfg(feature = "route53profiles_profile")]
pub mod route53profiles_profile;

#[cfg(feature = "route53profiles_profile")]
pub use route53profiles_profile::*;

#[cfg(feature = "route53profiles_resource_association")]
pub mod route53profiles_resource_association;

#[cfg(feature = "route53profiles_resource_association")]
pub use route53profiles_resource_association::*;

#[cfg(feature = "route53recoverycontrolconfig_cluster")]
pub mod route53recoverycontrolconfig_cluster;

#[cfg(feature = "route53recoverycontrolconfig_cluster")]
pub use route53recoverycontrolconfig_cluster::*;

#[cfg(feature = "route53recoverycontrolconfig_control_panel")]
pub mod route53recoverycontrolconfig_control_panel;

#[cfg(feature = "route53recoverycontrolconfig_control_panel")]
pub use route53recoverycontrolconfig_control_panel::*;

#[cfg(feature = "route53recoverycontrolconfig_routing_control")]
pub mod route53recoverycontrolconfig_routing_control;

#[cfg(feature = "route53recoverycontrolconfig_routing_control")]
pub use route53recoverycontrolconfig_routing_control::*;

#[cfg(feature = "route53recoverycontrolconfig_safety_rule")]
pub mod route53recoverycontrolconfig_safety_rule;

#[cfg(feature = "route53recoverycontrolconfig_safety_rule")]
pub use route53recoverycontrolconfig_safety_rule::*;

#[cfg(feature = "route53recoveryreadiness_cell")]
pub mod route53recoveryreadiness_cell;

#[cfg(feature = "route53recoveryreadiness_cell")]
pub use route53recoveryreadiness_cell::*;

#[cfg(feature = "route53recoveryreadiness_readiness_check")]
pub mod route53recoveryreadiness_readiness_check;

#[cfg(feature = "route53recoveryreadiness_readiness_check")]
pub use route53recoveryreadiness_readiness_check::*;

#[cfg(feature = "route53recoveryreadiness_recovery_group")]
pub mod route53recoveryreadiness_recovery_group;

#[cfg(feature = "route53recoveryreadiness_recovery_group")]
pub use route53recoveryreadiness_recovery_group::*;

#[cfg(feature = "route53recoveryreadiness_resource_set")]
pub mod route53recoveryreadiness_resource_set;

#[cfg(feature = "route53recoveryreadiness_resource_set")]
pub use route53recoveryreadiness_resource_set::*;

#[cfg(feature = "route_table")]
pub mod route_table;

#[cfg(feature = "route_table")]
pub use route_table::*;

#[cfg(feature = "route_table_association")]
pub mod route_table_association;

#[cfg(feature = "route_table_association")]
pub use route_table_association::*;

#[cfg(feature = "rum_app_monitor")]
pub mod rum_app_monitor;

#[cfg(feature = "rum_app_monitor")]
pub use rum_app_monitor::*;

#[cfg(feature = "rum_metrics_destination")]
pub mod rum_metrics_destination;

#[cfg(feature = "rum_metrics_destination")]
pub use rum_metrics_destination::*;

#[cfg(feature = "s3_access_point")]
pub mod s3_access_point;

#[cfg(feature = "s3_access_point")]
pub use s3_access_point::*;

#[cfg(feature = "s3_account_public_access_block")]
pub mod s3_account_public_access_block;

#[cfg(feature = "s3_account_public_access_block")]
pub use s3_account_public_access_block::*;

#[cfg(feature = "s3_bucket")]
pub mod s3_bucket;

#[cfg(feature = "s3_bucket")]
pub use s3_bucket::*;

#[cfg(feature = "s3_bucket_accelerate_configuration")]
pub mod s3_bucket_accelerate_configuration;

#[cfg(feature = "s3_bucket_accelerate_configuration")]
pub use s3_bucket_accelerate_configuration::*;

#[cfg(feature = "s3_bucket_acl")]
pub mod s3_bucket_acl;

#[cfg(feature = "s3_bucket_acl")]
pub use s3_bucket_acl::*;

#[cfg(feature = "s3_bucket_analytics_configuration")]
pub mod s3_bucket_analytics_configuration;

#[cfg(feature = "s3_bucket_analytics_configuration")]
pub use s3_bucket_analytics_configuration::*;

#[cfg(feature = "s3_bucket_cors_configuration")]
pub mod s3_bucket_cors_configuration;

#[cfg(feature = "s3_bucket_cors_configuration")]
pub use s3_bucket_cors_configuration::*;

#[cfg(feature = "s3_bucket_intelligent_tiering_configuration")]
pub mod s3_bucket_intelligent_tiering_configuration;

#[cfg(feature = "s3_bucket_intelligent_tiering_configuration")]
pub use s3_bucket_intelligent_tiering_configuration::*;

#[cfg(feature = "s3_bucket_inventory")]
pub mod s3_bucket_inventory;

#[cfg(feature = "s3_bucket_inventory")]
pub use s3_bucket_inventory::*;

#[cfg(feature = "s3_bucket_lifecycle_configuration")]
pub mod s3_bucket_lifecycle_configuration;

#[cfg(feature = "s3_bucket_lifecycle_configuration")]
pub use s3_bucket_lifecycle_configuration::*;

#[cfg(feature = "s3_bucket_logging")]
pub mod s3_bucket_logging;

#[cfg(feature = "s3_bucket_logging")]
pub use s3_bucket_logging::*;

#[cfg(feature = "s3_bucket_metadata_configuration")]
pub mod s3_bucket_metadata_configuration;

#[cfg(feature = "s3_bucket_metadata_configuration")]
pub use s3_bucket_metadata_configuration::*;

#[cfg(feature = "s3_bucket_metric")]
pub mod s3_bucket_metric;

#[cfg(feature = "s3_bucket_metric")]
pub use s3_bucket_metric::*;

#[cfg(feature = "s3_bucket_notification")]
pub mod s3_bucket_notification;

#[cfg(feature = "s3_bucket_notification")]
pub use s3_bucket_notification::*;

#[cfg(feature = "s3_bucket_object")]
pub mod s3_bucket_object;

#[cfg(feature = "s3_bucket_object")]
pub use s3_bucket_object::*;

#[cfg(feature = "s3_bucket_object_lock_configuration")]
pub mod s3_bucket_object_lock_configuration;

#[cfg(feature = "s3_bucket_object_lock_configuration")]
pub use s3_bucket_object_lock_configuration::*;

#[cfg(feature = "s3_bucket_ownership_controls")]
pub mod s3_bucket_ownership_controls;

#[cfg(feature = "s3_bucket_ownership_controls")]
pub use s3_bucket_ownership_controls::*;

#[cfg(feature = "s3_bucket_policy")]
pub mod s3_bucket_policy;

#[cfg(feature = "s3_bucket_policy")]
pub use s3_bucket_policy::*;

#[cfg(feature = "s3_bucket_public_access_block")]
pub mod s3_bucket_public_access_block;

#[cfg(feature = "s3_bucket_public_access_block")]
pub use s3_bucket_public_access_block::*;

#[cfg(feature = "s3_bucket_replication_configuration")]
pub mod s3_bucket_replication_configuration;

#[cfg(feature = "s3_bucket_replication_configuration")]
pub use s3_bucket_replication_configuration::*;

#[cfg(feature = "s3_bucket_request_payment_configuration")]
pub mod s3_bucket_request_payment_configuration;

#[cfg(feature = "s3_bucket_request_payment_configuration")]
pub use s3_bucket_request_payment_configuration::*;

#[cfg(feature = "s3_bucket_server_side_encryption_configuration")]
pub mod s3_bucket_server_side_encryption_configuration;

#[cfg(feature = "s3_bucket_server_side_encryption_configuration")]
pub use s3_bucket_server_side_encryption_configuration::*;

#[cfg(feature = "s3_bucket_versioning")]
pub mod s3_bucket_versioning;

#[cfg(feature = "s3_bucket_versioning")]
pub use s3_bucket_versioning::*;

#[cfg(feature = "s3_bucket_website_configuration")]
pub mod s3_bucket_website_configuration;

#[cfg(feature = "s3_bucket_website_configuration")]
pub use s3_bucket_website_configuration::*;

#[cfg(feature = "s3_directory_bucket")]
pub mod s3_directory_bucket;

#[cfg(feature = "s3_directory_bucket")]
pub use s3_directory_bucket::*;

#[cfg(feature = "s3_object")]
pub mod s3_object;

#[cfg(feature = "s3_object")]
pub use s3_object::*;

#[cfg(feature = "s3_object_copy")]
pub mod s3_object_copy;

#[cfg(feature = "s3_object_copy")]
pub use s3_object_copy::*;

#[cfg(feature = "s3control_access_grant")]
pub mod s3control_access_grant;

#[cfg(feature = "s3control_access_grant")]
pub use s3control_access_grant::*;

#[cfg(feature = "s3control_access_grants_instance")]
pub mod s3control_access_grants_instance;

#[cfg(feature = "s3control_access_grants_instance")]
pub use s3control_access_grants_instance::*;

#[cfg(feature = "s3control_access_grants_instance_resource_policy")]
pub mod s3control_access_grants_instance_resource_policy;

#[cfg(feature = "s3control_access_grants_instance_resource_policy")]
pub use s3control_access_grants_instance_resource_policy::*;

#[cfg(feature = "s3control_access_grants_location")]
pub mod s3control_access_grants_location;

#[cfg(feature = "s3control_access_grants_location")]
pub use s3control_access_grants_location::*;

#[cfg(feature = "s3control_access_point_policy")]
pub mod s3control_access_point_policy;

#[cfg(feature = "s3control_access_point_policy")]
pub use s3control_access_point_policy::*;

#[cfg(feature = "s3control_bucket")]
pub mod s3control_bucket;

#[cfg(feature = "s3control_bucket")]
pub use s3control_bucket::*;

#[cfg(feature = "s3control_bucket_lifecycle_configuration")]
pub mod s3control_bucket_lifecycle_configuration;

#[cfg(feature = "s3control_bucket_lifecycle_configuration")]
pub use s3control_bucket_lifecycle_configuration::*;

#[cfg(feature = "s3control_bucket_policy")]
pub mod s3control_bucket_policy;

#[cfg(feature = "s3control_bucket_policy")]
pub use s3control_bucket_policy::*;

#[cfg(feature = "s3control_directory_bucket_access_point_scope")]
pub mod s3control_directory_bucket_access_point_scope;

#[cfg(feature = "s3control_directory_bucket_access_point_scope")]
pub use s3control_directory_bucket_access_point_scope::*;

#[cfg(feature = "s3control_multi_region_access_point")]
pub mod s3control_multi_region_access_point;

#[cfg(feature = "s3control_multi_region_access_point")]
pub use s3control_multi_region_access_point::*;

#[cfg(feature = "s3control_multi_region_access_point_policy")]
pub mod s3control_multi_region_access_point_policy;

#[cfg(feature = "s3control_multi_region_access_point_policy")]
pub use s3control_multi_region_access_point_policy::*;

#[cfg(feature = "s3control_object_lambda_access_point")]
pub mod s3control_object_lambda_access_point;

#[cfg(feature = "s3control_object_lambda_access_point")]
pub use s3control_object_lambda_access_point::*;

#[cfg(feature = "s3control_object_lambda_access_point_policy")]
pub mod s3control_object_lambda_access_point_policy;

#[cfg(feature = "s3control_object_lambda_access_point_policy")]
pub use s3control_object_lambda_access_point_policy::*;

#[cfg(feature = "s3control_storage_lens_configuration")]
pub mod s3control_storage_lens_configuration;

#[cfg(feature = "s3control_storage_lens_configuration")]
pub use s3control_storage_lens_configuration::*;

#[cfg(feature = "s3outposts_endpoint")]
pub mod s3outposts_endpoint;

#[cfg(feature = "s3outposts_endpoint")]
pub use s3outposts_endpoint::*;

#[cfg(feature = "s3tables_namespace")]
pub mod s3tables_namespace;

#[cfg(feature = "s3tables_namespace")]
pub use s3tables_namespace::*;

#[cfg(feature = "s3tables_table")]
pub mod s3tables_table;

#[cfg(feature = "s3tables_table")]
pub use s3tables_table::*;

#[cfg(feature = "s3tables_table_bucket")]
pub mod s3tables_table_bucket;

#[cfg(feature = "s3tables_table_bucket")]
pub use s3tables_table_bucket::*;

#[cfg(feature = "s3tables_table_bucket_policy")]
pub mod s3tables_table_bucket_policy;

#[cfg(feature = "s3tables_table_bucket_policy")]
pub use s3tables_table_bucket_policy::*;

#[cfg(feature = "s3tables_table_policy")]
pub mod s3tables_table_policy;

#[cfg(feature = "s3tables_table_policy")]
pub use s3tables_table_policy::*;

#[cfg(feature = "sagemaker_app")]
pub mod sagemaker_app;

#[cfg(feature = "sagemaker_app")]
pub use sagemaker_app::*;

#[cfg(feature = "sagemaker_app_image_config")]
pub mod sagemaker_app_image_config;

#[cfg(feature = "sagemaker_app_image_config")]
pub use sagemaker_app_image_config::*;

#[cfg(feature = "sagemaker_code_repository")]
pub mod sagemaker_code_repository;

#[cfg(feature = "sagemaker_code_repository")]
pub use sagemaker_code_repository::*;

#[cfg(feature = "sagemaker_data_quality_job_definition")]
pub mod sagemaker_data_quality_job_definition;

#[cfg(feature = "sagemaker_data_quality_job_definition")]
pub use sagemaker_data_quality_job_definition::*;

#[cfg(feature = "sagemaker_device")]
pub mod sagemaker_device;

#[cfg(feature = "sagemaker_device")]
pub use sagemaker_device::*;

#[cfg(feature = "sagemaker_device_fleet")]
pub mod sagemaker_device_fleet;

#[cfg(feature = "sagemaker_device_fleet")]
pub use sagemaker_device_fleet::*;

#[cfg(feature = "sagemaker_domain")]
pub mod sagemaker_domain;

#[cfg(feature = "sagemaker_domain")]
pub use sagemaker_domain::*;

#[cfg(feature = "sagemaker_endpoint")]
pub mod sagemaker_endpoint;

#[cfg(feature = "sagemaker_endpoint")]
pub use sagemaker_endpoint::*;

#[cfg(feature = "sagemaker_endpoint_configuration")]
pub mod sagemaker_endpoint_configuration;

#[cfg(feature = "sagemaker_endpoint_configuration")]
pub use sagemaker_endpoint_configuration::*;

#[cfg(feature = "sagemaker_feature_group")]
pub mod sagemaker_feature_group;

#[cfg(feature = "sagemaker_feature_group")]
pub use sagemaker_feature_group::*;

#[cfg(feature = "sagemaker_flow_definition")]
pub mod sagemaker_flow_definition;

#[cfg(feature = "sagemaker_flow_definition")]
pub use sagemaker_flow_definition::*;

#[cfg(feature = "sagemaker_hub")]
pub mod sagemaker_hub;

#[cfg(feature = "sagemaker_hub")]
pub use sagemaker_hub::*;

#[cfg(feature = "sagemaker_human_task_ui")]
pub mod sagemaker_human_task_ui;

#[cfg(feature = "sagemaker_human_task_ui")]
pub use sagemaker_human_task_ui::*;

#[cfg(feature = "sagemaker_image")]
pub mod sagemaker_image;

#[cfg(feature = "sagemaker_image")]
pub use sagemaker_image::*;

#[cfg(feature = "sagemaker_image_version")]
pub mod sagemaker_image_version;

#[cfg(feature = "sagemaker_image_version")]
pub use sagemaker_image_version::*;

#[cfg(feature = "sagemaker_mlflow_tracking_server")]
pub mod sagemaker_mlflow_tracking_server;

#[cfg(feature = "sagemaker_mlflow_tracking_server")]
pub use sagemaker_mlflow_tracking_server::*;

#[cfg(feature = "sagemaker_model")]
pub mod sagemaker_model;

#[cfg(feature = "sagemaker_model")]
pub use sagemaker_model::*;

#[cfg(feature = "sagemaker_model_package_group")]
pub mod sagemaker_model_package_group;

#[cfg(feature = "sagemaker_model_package_group")]
pub use sagemaker_model_package_group::*;

#[cfg(feature = "sagemaker_model_package_group_policy")]
pub mod sagemaker_model_package_group_policy;

#[cfg(feature = "sagemaker_model_package_group_policy")]
pub use sagemaker_model_package_group_policy::*;

#[cfg(feature = "sagemaker_monitoring_schedule")]
pub mod sagemaker_monitoring_schedule;

#[cfg(feature = "sagemaker_monitoring_schedule")]
pub use sagemaker_monitoring_schedule::*;

#[cfg(feature = "sagemaker_notebook_instance")]
pub mod sagemaker_notebook_instance;

#[cfg(feature = "sagemaker_notebook_instance")]
pub use sagemaker_notebook_instance::*;

#[cfg(feature = "sagemaker_notebook_instance_lifecycle_configuration")]
pub mod sagemaker_notebook_instance_lifecycle_configuration;

#[cfg(feature = "sagemaker_notebook_instance_lifecycle_configuration")]
pub use sagemaker_notebook_instance_lifecycle_configuration::*;

#[cfg(feature = "sagemaker_pipeline")]
pub mod sagemaker_pipeline;

#[cfg(feature = "sagemaker_pipeline")]
pub use sagemaker_pipeline::*;

#[cfg(feature = "sagemaker_project")]
pub mod sagemaker_project;

#[cfg(feature = "sagemaker_project")]
pub use sagemaker_project::*;

#[cfg(feature = "sagemaker_servicecatalog_portfolio_status")]
pub mod sagemaker_servicecatalog_portfolio_status;

#[cfg(feature = "sagemaker_servicecatalog_portfolio_status")]
pub use sagemaker_servicecatalog_portfolio_status::*;

#[cfg(feature = "sagemaker_space")]
pub mod sagemaker_space;

#[cfg(feature = "sagemaker_space")]
pub use sagemaker_space::*;

#[cfg(feature = "sagemaker_studio_lifecycle_config")]
pub mod sagemaker_studio_lifecycle_config;

#[cfg(feature = "sagemaker_studio_lifecycle_config")]
pub use sagemaker_studio_lifecycle_config::*;

#[cfg(feature = "sagemaker_user_profile")]
pub mod sagemaker_user_profile;

#[cfg(feature = "sagemaker_user_profile")]
pub use sagemaker_user_profile::*;

#[cfg(feature = "sagemaker_workforce")]
pub mod sagemaker_workforce;

#[cfg(feature = "sagemaker_workforce")]
pub use sagemaker_workforce::*;

#[cfg(feature = "sagemaker_workteam")]
pub mod sagemaker_workteam;

#[cfg(feature = "sagemaker_workteam")]
pub use sagemaker_workteam::*;

#[cfg(feature = "scheduler_schedule")]
pub mod scheduler_schedule;

#[cfg(feature = "scheduler_schedule")]
pub use scheduler_schedule::*;

#[cfg(feature = "scheduler_schedule_group")]
pub mod scheduler_schedule_group;

#[cfg(feature = "scheduler_schedule_group")]
pub use scheduler_schedule_group::*;

#[cfg(feature = "schemas_discoverer")]
pub mod schemas_discoverer;

#[cfg(feature = "schemas_discoverer")]
pub use schemas_discoverer::*;

#[cfg(feature = "schemas_registry")]
pub mod schemas_registry;

#[cfg(feature = "schemas_registry")]
pub use schemas_registry::*;

#[cfg(feature = "schemas_registry_policy")]
pub mod schemas_registry_policy;

#[cfg(feature = "schemas_registry_policy")]
pub use schemas_registry_policy::*;

#[cfg(feature = "schemas_schema")]
pub mod schemas_schema;

#[cfg(feature = "schemas_schema")]
pub use schemas_schema::*;

#[cfg(feature = "secretsmanager_secret")]
pub mod secretsmanager_secret;

#[cfg(feature = "secretsmanager_secret")]
pub use secretsmanager_secret::*;

#[cfg(feature = "secretsmanager_secret_policy")]
pub mod secretsmanager_secret_policy;

#[cfg(feature = "secretsmanager_secret_policy")]
pub use secretsmanager_secret_policy::*;

#[cfg(feature = "secretsmanager_secret_rotation")]
pub mod secretsmanager_secret_rotation;

#[cfg(feature = "secretsmanager_secret_rotation")]
pub use secretsmanager_secret_rotation::*;

#[cfg(feature = "secretsmanager_secret_version")]
pub mod secretsmanager_secret_version;

#[cfg(feature = "secretsmanager_secret_version")]
pub use secretsmanager_secret_version::*;

#[cfg(feature = "security_group")]
pub mod security_group;

#[cfg(feature = "security_group")]
pub use security_group::*;

#[cfg(feature = "security_group_rule")]
pub mod security_group_rule;

#[cfg(feature = "security_group_rule")]
pub use security_group_rule::*;

#[cfg(feature = "securityhub_account")]
pub mod securityhub_account;

#[cfg(feature = "securityhub_account")]
pub use securityhub_account::*;

#[cfg(feature = "securityhub_action_target")]
pub mod securityhub_action_target;

#[cfg(feature = "securityhub_action_target")]
pub use securityhub_action_target::*;

#[cfg(feature = "securityhub_automation_rule")]
pub mod securityhub_automation_rule;

#[cfg(feature = "securityhub_automation_rule")]
pub use securityhub_automation_rule::*;

#[cfg(feature = "securityhub_configuration_policy")]
pub mod securityhub_configuration_policy;

#[cfg(feature = "securityhub_configuration_policy")]
pub use securityhub_configuration_policy::*;

#[cfg(feature = "securityhub_configuration_policy_association")]
pub mod securityhub_configuration_policy_association;

#[cfg(feature = "securityhub_configuration_policy_association")]
pub use securityhub_configuration_policy_association::*;

#[cfg(feature = "securityhub_finding_aggregator")]
pub mod securityhub_finding_aggregator;

#[cfg(feature = "securityhub_finding_aggregator")]
pub use securityhub_finding_aggregator::*;

#[cfg(feature = "securityhub_insight")]
pub mod securityhub_insight;

#[cfg(feature = "securityhub_insight")]
pub use securityhub_insight::*;

#[cfg(feature = "securityhub_invite_accepter")]
pub mod securityhub_invite_accepter;

#[cfg(feature = "securityhub_invite_accepter")]
pub use securityhub_invite_accepter::*;

#[cfg(feature = "securityhub_member")]
pub mod securityhub_member;

#[cfg(feature = "securityhub_member")]
pub use securityhub_member::*;

#[cfg(feature = "securityhub_organization_admin_account")]
pub mod securityhub_organization_admin_account;

#[cfg(feature = "securityhub_organization_admin_account")]
pub use securityhub_organization_admin_account::*;

#[cfg(feature = "securityhub_organization_configuration")]
pub mod securityhub_organization_configuration;

#[cfg(feature = "securityhub_organization_configuration")]
pub use securityhub_organization_configuration::*;

#[cfg(feature = "securityhub_product_subscription")]
pub mod securityhub_product_subscription;

#[cfg(feature = "securityhub_product_subscription")]
pub use securityhub_product_subscription::*;

#[cfg(feature = "securityhub_standards_control")]
pub mod securityhub_standards_control;

#[cfg(feature = "securityhub_standards_control")]
pub use securityhub_standards_control::*;

#[cfg(feature = "securityhub_standards_control_association")]
pub mod securityhub_standards_control_association;

#[cfg(feature = "securityhub_standards_control_association")]
pub use securityhub_standards_control_association::*;

#[cfg(feature = "securityhub_standards_subscription")]
pub mod securityhub_standards_subscription;

#[cfg(feature = "securityhub_standards_subscription")]
pub use securityhub_standards_subscription::*;

#[cfg(feature = "securitylake_aws_log_source")]
pub mod securitylake_aws_log_source;

#[cfg(feature = "securitylake_aws_log_source")]
pub use securitylake_aws_log_source::*;

#[cfg(feature = "securitylake_custom_log_source")]
pub mod securitylake_custom_log_source;

#[cfg(feature = "securitylake_custom_log_source")]
pub use securitylake_custom_log_source::*;

#[cfg(feature = "securitylake_data_lake")]
pub mod securitylake_data_lake;

#[cfg(feature = "securitylake_data_lake")]
pub use securitylake_data_lake::*;

#[cfg(feature = "securitylake_subscriber")]
pub mod securitylake_subscriber;

#[cfg(feature = "securitylake_subscriber")]
pub use securitylake_subscriber::*;

#[cfg(feature = "securitylake_subscriber_notification")]
pub mod securitylake_subscriber_notification;

#[cfg(feature = "securitylake_subscriber_notification")]
pub use securitylake_subscriber_notification::*;

#[cfg(feature = "serverlessapplicationrepository_cloudformation_stack")]
pub mod serverlessapplicationrepository_cloudformation_stack;

#[cfg(feature = "serverlessapplicationrepository_cloudformation_stack")]
pub use serverlessapplicationrepository_cloudformation_stack::*;

#[cfg(feature = "service_discovery_http_namespace")]
pub mod service_discovery_http_namespace;

#[cfg(feature = "service_discovery_http_namespace")]
pub use service_discovery_http_namespace::*;

#[cfg(feature = "service_discovery_instance")]
pub mod service_discovery_instance;

#[cfg(feature = "service_discovery_instance")]
pub use service_discovery_instance::*;

#[cfg(feature = "service_discovery_private_dns_namespace")]
pub mod service_discovery_private_dns_namespace;

#[cfg(feature = "service_discovery_private_dns_namespace")]
pub use service_discovery_private_dns_namespace::*;

#[cfg(feature = "service_discovery_public_dns_namespace")]
pub mod service_discovery_public_dns_namespace;

#[cfg(feature = "service_discovery_public_dns_namespace")]
pub use service_discovery_public_dns_namespace::*;

#[cfg(feature = "service_discovery_service")]
pub mod service_discovery_service;

#[cfg(feature = "service_discovery_service")]
pub use service_discovery_service::*;

#[cfg(feature = "servicecatalog_budget_resource_association")]
pub mod servicecatalog_budget_resource_association;

#[cfg(feature = "servicecatalog_budget_resource_association")]
pub use servicecatalog_budget_resource_association::*;

#[cfg(feature = "servicecatalog_constraint")]
pub mod servicecatalog_constraint;

#[cfg(feature = "servicecatalog_constraint")]
pub use servicecatalog_constraint::*;

#[cfg(feature = "servicecatalog_organizations_access")]
pub mod servicecatalog_organizations_access;

#[cfg(feature = "servicecatalog_organizations_access")]
pub use servicecatalog_organizations_access::*;

#[cfg(feature = "servicecatalog_portfolio")]
pub mod servicecatalog_portfolio;

#[cfg(feature = "servicecatalog_portfolio")]
pub use servicecatalog_portfolio::*;

#[cfg(feature = "servicecatalog_portfolio_share")]
pub mod servicecatalog_portfolio_share;

#[cfg(feature = "servicecatalog_portfolio_share")]
pub use servicecatalog_portfolio_share::*;

#[cfg(feature = "servicecatalog_principal_portfolio_association")]
pub mod servicecatalog_principal_portfolio_association;

#[cfg(feature = "servicecatalog_principal_portfolio_association")]
pub use servicecatalog_principal_portfolio_association::*;

#[cfg(feature = "servicecatalog_product")]
pub mod servicecatalog_product;

#[cfg(feature = "servicecatalog_product")]
pub use servicecatalog_product::*;

#[cfg(feature = "servicecatalog_product_portfolio_association")]
pub mod servicecatalog_product_portfolio_association;

#[cfg(feature = "servicecatalog_product_portfolio_association")]
pub use servicecatalog_product_portfolio_association::*;

#[cfg(feature = "servicecatalog_provisioned_product")]
pub mod servicecatalog_provisioned_product;

#[cfg(feature = "servicecatalog_provisioned_product")]
pub use servicecatalog_provisioned_product::*;

#[cfg(feature = "servicecatalog_provisioning_artifact")]
pub mod servicecatalog_provisioning_artifact;

#[cfg(feature = "servicecatalog_provisioning_artifact")]
pub use servicecatalog_provisioning_artifact::*;

#[cfg(feature = "servicecatalog_service_action")]
pub mod servicecatalog_service_action;

#[cfg(feature = "servicecatalog_service_action")]
pub use servicecatalog_service_action::*;

#[cfg(feature = "servicecatalog_tag_option")]
pub mod servicecatalog_tag_option;

#[cfg(feature = "servicecatalog_tag_option")]
pub use servicecatalog_tag_option::*;

#[cfg(feature = "servicecatalog_tag_option_resource_association")]
pub mod servicecatalog_tag_option_resource_association;

#[cfg(feature = "servicecatalog_tag_option_resource_association")]
pub use servicecatalog_tag_option_resource_association::*;

#[cfg(feature = "servicecatalogappregistry_application")]
pub mod servicecatalogappregistry_application;

#[cfg(feature = "servicecatalogappregistry_application")]
pub use servicecatalogappregistry_application::*;

#[cfg(feature = "servicecatalogappregistry_attribute_group")]
pub mod servicecatalogappregistry_attribute_group;

#[cfg(feature = "servicecatalogappregistry_attribute_group")]
pub use servicecatalogappregistry_attribute_group::*;

#[cfg(feature = "servicecatalogappregistry_attribute_group_association")]
pub mod servicecatalogappregistry_attribute_group_association;

#[cfg(feature = "servicecatalogappregistry_attribute_group_association")]
pub use servicecatalogappregistry_attribute_group_association::*;

#[cfg(feature = "servicequotas_service_quota")]
pub mod servicequotas_service_quota;

#[cfg(feature = "servicequotas_service_quota")]
pub use servicequotas_service_quota::*;

#[cfg(feature = "servicequotas_template")]
pub mod servicequotas_template;

#[cfg(feature = "servicequotas_template")]
pub use servicequotas_template::*;

#[cfg(feature = "servicequotas_template_association")]
pub mod servicequotas_template_association;

#[cfg(feature = "servicequotas_template_association")]
pub use servicequotas_template_association::*;

#[cfg(feature = "ses_active_receipt_rule_set")]
pub mod ses_active_receipt_rule_set;

#[cfg(feature = "ses_active_receipt_rule_set")]
pub use ses_active_receipt_rule_set::*;

#[cfg(feature = "ses_configuration_set")]
pub mod ses_configuration_set;

#[cfg(feature = "ses_configuration_set")]
pub use ses_configuration_set::*;

#[cfg(feature = "ses_domain_dkim")]
pub mod ses_domain_dkim;

#[cfg(feature = "ses_domain_dkim")]
pub use ses_domain_dkim::*;

#[cfg(feature = "ses_domain_identity")]
pub mod ses_domain_identity;

#[cfg(feature = "ses_domain_identity")]
pub use ses_domain_identity::*;

#[cfg(feature = "ses_domain_identity_verification")]
pub mod ses_domain_identity_verification;

#[cfg(feature = "ses_domain_identity_verification")]
pub use ses_domain_identity_verification::*;

#[cfg(feature = "ses_domain_mail_from")]
pub mod ses_domain_mail_from;

#[cfg(feature = "ses_domain_mail_from")]
pub use ses_domain_mail_from::*;

#[cfg(feature = "ses_email_identity")]
pub mod ses_email_identity;

#[cfg(feature = "ses_email_identity")]
pub use ses_email_identity::*;

#[cfg(feature = "ses_event_destination")]
pub mod ses_event_destination;

#[cfg(feature = "ses_event_destination")]
pub use ses_event_destination::*;

#[cfg(feature = "ses_identity_notification_topic")]
pub mod ses_identity_notification_topic;

#[cfg(feature = "ses_identity_notification_topic")]
pub use ses_identity_notification_topic::*;

#[cfg(feature = "ses_identity_policy")]
pub mod ses_identity_policy;

#[cfg(feature = "ses_identity_policy")]
pub use ses_identity_policy::*;

#[cfg(feature = "ses_receipt_filter")]
pub mod ses_receipt_filter;

#[cfg(feature = "ses_receipt_filter")]
pub use ses_receipt_filter::*;

#[cfg(feature = "ses_receipt_rule")]
pub mod ses_receipt_rule;

#[cfg(feature = "ses_receipt_rule")]
pub use ses_receipt_rule::*;

#[cfg(feature = "ses_receipt_rule_set")]
pub mod ses_receipt_rule_set;

#[cfg(feature = "ses_receipt_rule_set")]
pub use ses_receipt_rule_set::*;

#[cfg(feature = "ses_template")]
pub mod ses_template;

#[cfg(feature = "ses_template")]
pub use ses_template::*;

#[cfg(feature = "sesv2_account_suppression_attributes")]
pub mod sesv2_account_suppression_attributes;

#[cfg(feature = "sesv2_account_suppression_attributes")]
pub use sesv2_account_suppression_attributes::*;

#[cfg(feature = "sesv2_account_vdm_attributes")]
pub mod sesv2_account_vdm_attributes;

#[cfg(feature = "sesv2_account_vdm_attributes")]
pub use sesv2_account_vdm_attributes::*;

#[cfg(feature = "sesv2_configuration_set")]
pub mod sesv2_configuration_set;

#[cfg(feature = "sesv2_configuration_set")]
pub use sesv2_configuration_set::*;

#[cfg(feature = "sesv2_configuration_set_event_destination")]
pub mod sesv2_configuration_set_event_destination;

#[cfg(feature = "sesv2_configuration_set_event_destination")]
pub use sesv2_configuration_set_event_destination::*;

#[cfg(feature = "sesv2_contact_list")]
pub mod sesv2_contact_list;

#[cfg(feature = "sesv2_contact_list")]
pub use sesv2_contact_list::*;

#[cfg(feature = "sesv2_dedicated_ip_assignment")]
pub mod sesv2_dedicated_ip_assignment;

#[cfg(feature = "sesv2_dedicated_ip_assignment")]
pub use sesv2_dedicated_ip_assignment::*;

#[cfg(feature = "sesv2_dedicated_ip_pool")]
pub mod sesv2_dedicated_ip_pool;

#[cfg(feature = "sesv2_dedicated_ip_pool")]
pub use sesv2_dedicated_ip_pool::*;

#[cfg(feature = "sesv2_email_identity")]
pub mod sesv2_email_identity;

#[cfg(feature = "sesv2_email_identity")]
pub use sesv2_email_identity::*;

#[cfg(feature = "sesv2_email_identity_feedback_attributes")]
pub mod sesv2_email_identity_feedback_attributes;

#[cfg(feature = "sesv2_email_identity_feedback_attributes")]
pub use sesv2_email_identity_feedback_attributes::*;

#[cfg(feature = "sesv2_email_identity_mail_from_attributes")]
pub mod sesv2_email_identity_mail_from_attributes;

#[cfg(feature = "sesv2_email_identity_mail_from_attributes")]
pub use sesv2_email_identity_mail_from_attributes::*;

#[cfg(feature = "sesv2_email_identity_policy")]
pub mod sesv2_email_identity_policy;

#[cfg(feature = "sesv2_email_identity_policy")]
pub use sesv2_email_identity_policy::*;

#[cfg(feature = "sfn_activity")]
pub mod sfn_activity;

#[cfg(feature = "sfn_activity")]
pub use sfn_activity::*;

#[cfg(feature = "sfn_alias")]
pub mod sfn_alias;

#[cfg(feature = "sfn_alias")]
pub use sfn_alias::*;

#[cfg(feature = "sfn_state_machine")]
pub mod sfn_state_machine;

#[cfg(feature = "sfn_state_machine")]
pub use sfn_state_machine::*;

#[cfg(feature = "shield_application_layer_automatic_response")]
pub mod shield_application_layer_automatic_response;

#[cfg(feature = "shield_application_layer_automatic_response")]
pub use shield_application_layer_automatic_response::*;

#[cfg(feature = "shield_drt_access_log_bucket_association")]
pub mod shield_drt_access_log_bucket_association;

#[cfg(feature = "shield_drt_access_log_bucket_association")]
pub use shield_drt_access_log_bucket_association::*;

#[cfg(feature = "shield_drt_access_role_arn_association")]
pub mod shield_drt_access_role_arn_association;

#[cfg(feature = "shield_drt_access_role_arn_association")]
pub use shield_drt_access_role_arn_association::*;

#[cfg(feature = "shield_proactive_engagement")]
pub mod shield_proactive_engagement;

#[cfg(feature = "shield_proactive_engagement")]
pub use shield_proactive_engagement::*;

#[cfg(feature = "shield_protection")]
pub mod shield_protection;

#[cfg(feature = "shield_protection")]
pub use shield_protection::*;

#[cfg(feature = "shield_protection_group")]
pub mod shield_protection_group;

#[cfg(feature = "shield_protection_group")]
pub use shield_protection_group::*;

#[cfg(feature = "shield_protection_health_check_association")]
pub mod shield_protection_health_check_association;

#[cfg(feature = "shield_protection_health_check_association")]
pub use shield_protection_health_check_association::*;

#[cfg(feature = "shield_subscription")]
pub mod shield_subscription;

#[cfg(feature = "shield_subscription")]
pub use shield_subscription::*;

#[cfg(feature = "signer_signing_job")]
pub mod signer_signing_job;

#[cfg(feature = "signer_signing_job")]
pub use signer_signing_job::*;

#[cfg(feature = "signer_signing_profile")]
pub mod signer_signing_profile;

#[cfg(feature = "signer_signing_profile")]
pub use signer_signing_profile::*;

#[cfg(feature = "signer_signing_profile_permission")]
pub mod signer_signing_profile_permission;

#[cfg(feature = "signer_signing_profile_permission")]
pub use signer_signing_profile_permission::*;

#[cfg(feature = "snapshot_create_volume_permission")]
pub mod snapshot_create_volume_permission;

#[cfg(feature = "snapshot_create_volume_permission")]
pub use snapshot_create_volume_permission::*;

#[cfg(feature = "sns_platform_application")]
pub mod sns_platform_application;

#[cfg(feature = "sns_platform_application")]
pub use sns_platform_application::*;

#[cfg(feature = "sns_sms_preferences")]
pub mod sns_sms_preferences;

#[cfg(feature = "sns_sms_preferences")]
pub use sns_sms_preferences::*;

#[cfg(feature = "sns_topic")]
pub mod sns_topic;

#[cfg(feature = "sns_topic")]
pub use sns_topic::*;

#[cfg(feature = "sns_topic_data_protection_policy")]
pub mod sns_topic_data_protection_policy;

#[cfg(feature = "sns_topic_data_protection_policy")]
pub use sns_topic_data_protection_policy::*;

#[cfg(feature = "sns_topic_policy")]
pub mod sns_topic_policy;

#[cfg(feature = "sns_topic_policy")]
pub use sns_topic_policy::*;

#[cfg(feature = "sns_topic_subscription")]
pub mod sns_topic_subscription;

#[cfg(feature = "sns_topic_subscription")]
pub use sns_topic_subscription::*;

#[cfg(feature = "spot_datafeed_subscription")]
pub mod spot_datafeed_subscription;

#[cfg(feature = "spot_datafeed_subscription")]
pub use spot_datafeed_subscription::*;

#[cfg(feature = "spot_fleet_request")]
pub mod spot_fleet_request;

#[cfg(feature = "spot_fleet_request")]
pub use spot_fleet_request::*;

#[cfg(feature = "spot_instance_request")]
pub mod spot_instance_request;

#[cfg(feature = "spot_instance_request")]
pub use spot_instance_request::*;

#[cfg(feature = "sqs_queue")]
pub mod sqs_queue;

#[cfg(feature = "sqs_queue")]
pub use sqs_queue::*;

#[cfg(feature = "sqs_queue_policy")]
pub mod sqs_queue_policy;

#[cfg(feature = "sqs_queue_policy")]
pub use sqs_queue_policy::*;

#[cfg(feature = "sqs_queue_redrive_allow_policy")]
pub mod sqs_queue_redrive_allow_policy;

#[cfg(feature = "sqs_queue_redrive_allow_policy")]
pub use sqs_queue_redrive_allow_policy::*;

#[cfg(feature = "sqs_queue_redrive_policy")]
pub mod sqs_queue_redrive_policy;

#[cfg(feature = "sqs_queue_redrive_policy")]
pub use sqs_queue_redrive_policy::*;

#[cfg(feature = "ssm_activation")]
pub mod ssm_activation;

#[cfg(feature = "ssm_activation")]
pub use ssm_activation::*;

#[cfg(feature = "ssm_association")]
pub mod ssm_association;

#[cfg(feature = "ssm_association")]
pub use ssm_association::*;

#[cfg(feature = "ssm_default_patch_baseline")]
pub mod ssm_default_patch_baseline;

#[cfg(feature = "ssm_default_patch_baseline")]
pub use ssm_default_patch_baseline::*;

#[cfg(feature = "ssm_document")]
pub mod ssm_document;

#[cfg(feature = "ssm_document")]
pub use ssm_document::*;

#[cfg(feature = "ssm_maintenance_window")]
pub mod ssm_maintenance_window;

#[cfg(feature = "ssm_maintenance_window")]
pub use ssm_maintenance_window::*;

#[cfg(feature = "ssm_maintenance_window_target")]
pub mod ssm_maintenance_window_target;

#[cfg(feature = "ssm_maintenance_window_target")]
pub use ssm_maintenance_window_target::*;

#[cfg(feature = "ssm_maintenance_window_task")]
pub mod ssm_maintenance_window_task;

#[cfg(feature = "ssm_maintenance_window_task")]
pub use ssm_maintenance_window_task::*;

#[cfg(feature = "ssm_parameter")]
pub mod ssm_parameter;

#[cfg(feature = "ssm_parameter")]
pub use ssm_parameter::*;

#[cfg(feature = "ssm_patch_baseline")]
pub mod ssm_patch_baseline;

#[cfg(feature = "ssm_patch_baseline")]
pub use ssm_patch_baseline::*;

#[cfg(feature = "ssm_patch_group")]
pub mod ssm_patch_group;

#[cfg(feature = "ssm_patch_group")]
pub use ssm_patch_group::*;

#[cfg(feature = "ssm_resource_data_sync")]
pub mod ssm_resource_data_sync;

#[cfg(feature = "ssm_resource_data_sync")]
pub use ssm_resource_data_sync::*;

#[cfg(feature = "ssm_service_setting")]
pub mod ssm_service_setting;

#[cfg(feature = "ssm_service_setting")]
pub use ssm_service_setting::*;

#[cfg(feature = "ssmcontacts_contact")]
pub mod ssmcontacts_contact;

#[cfg(feature = "ssmcontacts_contact")]
pub use ssmcontacts_contact::*;

#[cfg(feature = "ssmcontacts_contact_channel")]
pub mod ssmcontacts_contact_channel;

#[cfg(feature = "ssmcontacts_contact_channel")]
pub use ssmcontacts_contact_channel::*;

#[cfg(feature = "ssmcontacts_plan")]
pub mod ssmcontacts_plan;

#[cfg(feature = "ssmcontacts_plan")]
pub use ssmcontacts_plan::*;

#[cfg(feature = "ssmcontacts_rotation")]
pub mod ssmcontacts_rotation;

#[cfg(feature = "ssmcontacts_rotation")]
pub use ssmcontacts_rotation::*;

#[cfg(feature = "ssmincidents_replication_set")]
pub mod ssmincidents_replication_set;

#[cfg(feature = "ssmincidents_replication_set")]
pub use ssmincidents_replication_set::*;

#[cfg(feature = "ssmincidents_response_plan")]
pub mod ssmincidents_response_plan;

#[cfg(feature = "ssmincidents_response_plan")]
pub use ssmincidents_response_plan::*;

#[cfg(feature = "ssmquicksetup_configuration_manager")]
pub mod ssmquicksetup_configuration_manager;

#[cfg(feature = "ssmquicksetup_configuration_manager")]
pub use ssmquicksetup_configuration_manager::*;

#[cfg(feature = "ssoadmin_account_assignment")]
pub mod ssoadmin_account_assignment;

#[cfg(feature = "ssoadmin_account_assignment")]
pub use ssoadmin_account_assignment::*;

#[cfg(feature = "ssoadmin_application")]
pub mod ssoadmin_application;

#[cfg(feature = "ssoadmin_application")]
pub use ssoadmin_application::*;

#[cfg(feature = "ssoadmin_application_access_scope")]
pub mod ssoadmin_application_access_scope;

#[cfg(feature = "ssoadmin_application_access_scope")]
pub use ssoadmin_application_access_scope::*;

#[cfg(feature = "ssoadmin_application_assignment")]
pub mod ssoadmin_application_assignment;

#[cfg(feature = "ssoadmin_application_assignment")]
pub use ssoadmin_application_assignment::*;

#[cfg(feature = "ssoadmin_application_assignment_configuration")]
pub mod ssoadmin_application_assignment_configuration;

#[cfg(feature = "ssoadmin_application_assignment_configuration")]
pub use ssoadmin_application_assignment_configuration::*;

#[cfg(feature = "ssoadmin_customer_managed_policy_attachment")]
pub mod ssoadmin_customer_managed_policy_attachment;

#[cfg(feature = "ssoadmin_customer_managed_policy_attachment")]
pub use ssoadmin_customer_managed_policy_attachment::*;

#[cfg(feature = "ssoadmin_instance_access_control_attributes")]
pub mod ssoadmin_instance_access_control_attributes;

#[cfg(feature = "ssoadmin_instance_access_control_attributes")]
pub use ssoadmin_instance_access_control_attributes::*;

#[cfg(feature = "ssoadmin_managed_policy_attachment")]
pub mod ssoadmin_managed_policy_attachment;

#[cfg(feature = "ssoadmin_managed_policy_attachment")]
pub use ssoadmin_managed_policy_attachment::*;

#[cfg(feature = "ssoadmin_permission_set")]
pub mod ssoadmin_permission_set;

#[cfg(feature = "ssoadmin_permission_set")]
pub use ssoadmin_permission_set::*;

#[cfg(feature = "ssoadmin_permission_set_inline_policy")]
pub mod ssoadmin_permission_set_inline_policy;

#[cfg(feature = "ssoadmin_permission_set_inline_policy")]
pub use ssoadmin_permission_set_inline_policy::*;

#[cfg(feature = "ssoadmin_permissions_boundary_attachment")]
pub mod ssoadmin_permissions_boundary_attachment;

#[cfg(feature = "ssoadmin_permissions_boundary_attachment")]
pub use ssoadmin_permissions_boundary_attachment::*;

#[cfg(feature = "ssoadmin_trusted_token_issuer")]
pub mod ssoadmin_trusted_token_issuer;

#[cfg(feature = "ssoadmin_trusted_token_issuer")]
pub use ssoadmin_trusted_token_issuer::*;

#[cfg(feature = "storagegateway_cache")]
pub mod storagegateway_cache;

#[cfg(feature = "storagegateway_cache")]
pub use storagegateway_cache::*;

#[cfg(feature = "storagegateway_cached_iscsi_volume")]
pub mod storagegateway_cached_iscsi_volume;

#[cfg(feature = "storagegateway_cached_iscsi_volume")]
pub use storagegateway_cached_iscsi_volume::*;

#[cfg(feature = "storagegateway_file_system_association")]
pub mod storagegateway_file_system_association;

#[cfg(feature = "storagegateway_file_system_association")]
pub use storagegateway_file_system_association::*;

#[cfg(feature = "storagegateway_gateway")]
pub mod storagegateway_gateway;

#[cfg(feature = "storagegateway_gateway")]
pub use storagegateway_gateway::*;

#[cfg(feature = "storagegateway_nfs_file_share")]
pub mod storagegateway_nfs_file_share;

#[cfg(feature = "storagegateway_nfs_file_share")]
pub use storagegateway_nfs_file_share::*;

#[cfg(feature = "storagegateway_smb_file_share")]
pub mod storagegateway_smb_file_share;

#[cfg(feature = "storagegateway_smb_file_share")]
pub use storagegateway_smb_file_share::*;

#[cfg(feature = "storagegateway_stored_iscsi_volume")]
pub mod storagegateway_stored_iscsi_volume;

#[cfg(feature = "storagegateway_stored_iscsi_volume")]
pub use storagegateway_stored_iscsi_volume::*;

#[cfg(feature = "storagegateway_tape_pool")]
pub mod storagegateway_tape_pool;

#[cfg(feature = "storagegateway_tape_pool")]
pub use storagegateway_tape_pool::*;

#[cfg(feature = "storagegateway_upload_buffer")]
pub mod storagegateway_upload_buffer;

#[cfg(feature = "storagegateway_upload_buffer")]
pub use storagegateway_upload_buffer::*;

#[cfg(feature = "storagegateway_working_storage")]
pub mod storagegateway_working_storage;

#[cfg(feature = "storagegateway_working_storage")]
pub use storagegateway_working_storage::*;

#[cfg(feature = "subnet")]
pub mod subnet;

#[cfg(feature = "subnet")]
pub use subnet::*;

#[cfg(feature = "swf_domain")]
pub mod swf_domain;

#[cfg(feature = "swf_domain")]
pub use swf_domain::*;

#[cfg(feature = "synthetics_canary")]
pub mod synthetics_canary;

#[cfg(feature = "synthetics_canary")]
pub use synthetics_canary::*;

#[cfg(feature = "synthetics_group")]
pub mod synthetics_group;

#[cfg(feature = "synthetics_group")]
pub use synthetics_group::*;

#[cfg(feature = "synthetics_group_association")]
pub mod synthetics_group_association;

#[cfg(feature = "synthetics_group_association")]
pub use synthetics_group_association::*;

#[cfg(feature = "timestreaminfluxdb_db_cluster")]
pub mod timestreaminfluxdb_db_cluster;

#[cfg(feature = "timestreaminfluxdb_db_cluster")]
pub use timestreaminfluxdb_db_cluster::*;

#[cfg(feature = "timestreaminfluxdb_db_instance")]
pub mod timestreaminfluxdb_db_instance;

#[cfg(feature = "timestreaminfluxdb_db_instance")]
pub use timestreaminfluxdb_db_instance::*;

#[cfg(feature = "timestreamquery_scheduled_query")]
pub mod timestreamquery_scheduled_query;

#[cfg(feature = "timestreamquery_scheduled_query")]
pub use timestreamquery_scheduled_query::*;

#[cfg(feature = "timestreamwrite_database")]
pub mod timestreamwrite_database;

#[cfg(feature = "timestreamwrite_database")]
pub use timestreamwrite_database::*;

#[cfg(feature = "timestreamwrite_table")]
pub mod timestreamwrite_table;

#[cfg(feature = "timestreamwrite_table")]
pub use timestreamwrite_table::*;

#[cfg(feature = "transcribe_language_model")]
pub mod transcribe_language_model;

#[cfg(feature = "transcribe_language_model")]
pub use transcribe_language_model::*;

#[cfg(feature = "transcribe_medical_vocabulary")]
pub mod transcribe_medical_vocabulary;

#[cfg(feature = "transcribe_medical_vocabulary")]
pub use transcribe_medical_vocabulary::*;

#[cfg(feature = "transcribe_vocabulary")]
pub mod transcribe_vocabulary;

#[cfg(feature = "transcribe_vocabulary")]
pub use transcribe_vocabulary::*;

#[cfg(feature = "transcribe_vocabulary_filter")]
pub mod transcribe_vocabulary_filter;

#[cfg(feature = "transcribe_vocabulary_filter")]
pub use transcribe_vocabulary_filter::*;

#[cfg(feature = "transfer_access")]
pub mod transfer_access;

#[cfg(feature = "transfer_access")]
pub use transfer_access::*;

#[cfg(feature = "transfer_agreement")]
pub mod transfer_agreement;

#[cfg(feature = "transfer_agreement")]
pub use transfer_agreement::*;

#[cfg(feature = "transfer_certificate")]
pub mod transfer_certificate;

#[cfg(feature = "transfer_certificate")]
pub use transfer_certificate::*;

#[cfg(feature = "transfer_connector")]
pub mod transfer_connector;

#[cfg(feature = "transfer_connector")]
pub use transfer_connector::*;

#[cfg(feature = "transfer_host_key")]
pub mod transfer_host_key;

#[cfg(feature = "transfer_host_key")]
pub use transfer_host_key::*;

#[cfg(feature = "transfer_profile")]
pub mod transfer_profile;

#[cfg(feature = "transfer_profile")]
pub use transfer_profile::*;

#[cfg(feature = "transfer_server")]
pub mod transfer_server;

#[cfg(feature = "transfer_server")]
pub use transfer_server::*;

#[cfg(feature = "transfer_ssh_key")]
pub mod transfer_ssh_key;

#[cfg(feature = "transfer_ssh_key")]
pub use transfer_ssh_key::*;

#[cfg(feature = "transfer_tag")]
pub mod transfer_tag;

#[cfg(feature = "transfer_tag")]
pub use transfer_tag::*;

#[cfg(feature = "transfer_user")]
pub mod transfer_user;

#[cfg(feature = "transfer_user")]
pub use transfer_user::*;

#[cfg(feature = "transfer_web_app")]
pub mod transfer_web_app;

#[cfg(feature = "transfer_web_app")]
pub use transfer_web_app::*;

#[cfg(feature = "transfer_web_app_customization")]
pub mod transfer_web_app_customization;

#[cfg(feature = "transfer_web_app_customization")]
pub use transfer_web_app_customization::*;

#[cfg(feature = "transfer_workflow")]
pub mod transfer_workflow;

#[cfg(feature = "transfer_workflow")]
pub use transfer_workflow::*;

#[cfg(feature = "verifiedaccess_endpoint")]
pub mod verifiedaccess_endpoint;

#[cfg(feature = "verifiedaccess_endpoint")]
pub use verifiedaccess_endpoint::*;

#[cfg(feature = "verifiedaccess_group")]
pub mod verifiedaccess_group;

#[cfg(feature = "verifiedaccess_group")]
pub use verifiedaccess_group::*;

#[cfg(feature = "verifiedaccess_instance")]
pub mod verifiedaccess_instance;

#[cfg(feature = "verifiedaccess_instance")]
pub use verifiedaccess_instance::*;

#[cfg(feature = "verifiedaccess_instance_logging_configuration")]
pub mod verifiedaccess_instance_logging_configuration;

#[cfg(feature = "verifiedaccess_instance_logging_configuration")]
pub use verifiedaccess_instance_logging_configuration::*;

#[cfg(feature = "verifiedaccess_instance_trust_provider_attachment")]
pub mod verifiedaccess_instance_trust_provider_attachment;

#[cfg(feature = "verifiedaccess_instance_trust_provider_attachment")]
pub use verifiedaccess_instance_trust_provider_attachment::*;

#[cfg(feature = "verifiedaccess_trust_provider")]
pub mod verifiedaccess_trust_provider;

#[cfg(feature = "verifiedaccess_trust_provider")]
pub use verifiedaccess_trust_provider::*;

#[cfg(feature = "verifiedpermissions_identity_source")]
pub mod verifiedpermissions_identity_source;

#[cfg(feature = "verifiedpermissions_identity_source")]
pub use verifiedpermissions_identity_source::*;

#[cfg(feature = "verifiedpermissions_policy")]
pub mod verifiedpermissions_policy;

#[cfg(feature = "verifiedpermissions_policy")]
pub use verifiedpermissions_policy::*;

#[cfg(feature = "verifiedpermissions_policy_store")]
pub mod verifiedpermissions_policy_store;

#[cfg(feature = "verifiedpermissions_policy_store")]
pub use verifiedpermissions_policy_store::*;

#[cfg(feature = "verifiedpermissions_policy_template")]
pub mod verifiedpermissions_policy_template;

#[cfg(feature = "verifiedpermissions_policy_template")]
pub use verifiedpermissions_policy_template::*;

#[cfg(feature = "verifiedpermissions_schema")]
pub mod verifiedpermissions_schema;

#[cfg(feature = "verifiedpermissions_schema")]
pub use verifiedpermissions_schema::*;

#[cfg(feature = "volume_attachment")]
pub mod volume_attachment;

#[cfg(feature = "volume_attachment")]
pub use volume_attachment::*;

#[cfg(feature = "vpc")]
pub mod vpc;

#[cfg(feature = "vpc")]
pub use vpc::*;

#[cfg(feature = "vpc_block_public_access_exclusion")]
pub mod vpc_block_public_access_exclusion;

#[cfg(feature = "vpc_block_public_access_exclusion")]
pub use vpc_block_public_access_exclusion::*;

#[cfg(feature = "vpc_block_public_access_options")]
pub mod vpc_block_public_access_options;

#[cfg(feature = "vpc_block_public_access_options")]
pub use vpc_block_public_access_options::*;

#[cfg(feature = "vpc_dhcp_options")]
pub mod vpc_dhcp_options;

#[cfg(feature = "vpc_dhcp_options")]
pub use vpc_dhcp_options::*;

#[cfg(feature = "vpc_dhcp_options_association")]
pub mod vpc_dhcp_options_association;

#[cfg(feature = "vpc_dhcp_options_association")]
pub use vpc_dhcp_options_association::*;

#[cfg(feature = "vpc_endpoint")]
pub mod vpc_endpoint;

#[cfg(feature = "vpc_endpoint")]
pub use vpc_endpoint::*;

#[cfg(feature = "vpc_endpoint_connection_accepter")]
pub mod vpc_endpoint_connection_accepter;

#[cfg(feature = "vpc_endpoint_connection_accepter")]
pub use vpc_endpoint_connection_accepter::*;

#[cfg(feature = "vpc_endpoint_connection_notification")]
pub mod vpc_endpoint_connection_notification;

#[cfg(feature = "vpc_endpoint_connection_notification")]
pub use vpc_endpoint_connection_notification::*;

#[cfg(feature = "vpc_endpoint_policy")]
pub mod vpc_endpoint_policy;

#[cfg(feature = "vpc_endpoint_policy")]
pub use vpc_endpoint_policy::*;

#[cfg(feature = "vpc_endpoint_private_dns")]
pub mod vpc_endpoint_private_dns;

#[cfg(feature = "vpc_endpoint_private_dns")]
pub use vpc_endpoint_private_dns::*;

#[cfg(feature = "vpc_endpoint_route_table_association")]
pub mod vpc_endpoint_route_table_association;

#[cfg(feature = "vpc_endpoint_route_table_association")]
pub use vpc_endpoint_route_table_association::*;

#[cfg(feature = "vpc_endpoint_security_group_association")]
pub mod vpc_endpoint_security_group_association;

#[cfg(feature = "vpc_endpoint_security_group_association")]
pub use vpc_endpoint_security_group_association::*;

#[cfg(feature = "vpc_endpoint_service")]
pub mod vpc_endpoint_service;

#[cfg(feature = "vpc_endpoint_service")]
pub use vpc_endpoint_service::*;

#[cfg(feature = "vpc_endpoint_service_allowed_principal")]
pub mod vpc_endpoint_service_allowed_principal;

#[cfg(feature = "vpc_endpoint_service_allowed_principal")]
pub use vpc_endpoint_service_allowed_principal::*;

#[cfg(feature = "vpc_endpoint_service_private_dns_verification")]
pub mod vpc_endpoint_service_private_dns_verification;

#[cfg(feature = "vpc_endpoint_service_private_dns_verification")]
pub use vpc_endpoint_service_private_dns_verification::*;

#[cfg(feature = "vpc_endpoint_subnet_association")]
pub mod vpc_endpoint_subnet_association;

#[cfg(feature = "vpc_endpoint_subnet_association")]
pub use vpc_endpoint_subnet_association::*;

#[cfg(feature = "vpc_ipam")]
pub mod vpc_ipam;

#[cfg(feature = "vpc_ipam")]
pub use vpc_ipam::*;

#[cfg(feature = "vpc_ipam_organization_admin_account")]
pub mod vpc_ipam_organization_admin_account;

#[cfg(feature = "vpc_ipam_organization_admin_account")]
pub use vpc_ipam_organization_admin_account::*;

#[cfg(feature = "vpc_ipam_pool")]
pub mod vpc_ipam_pool;

#[cfg(feature = "vpc_ipam_pool")]
pub use vpc_ipam_pool::*;

#[cfg(feature = "vpc_ipam_pool_cidr")]
pub mod vpc_ipam_pool_cidr;

#[cfg(feature = "vpc_ipam_pool_cidr")]
pub use vpc_ipam_pool_cidr::*;

#[cfg(feature = "vpc_ipam_pool_cidr_allocation")]
pub mod vpc_ipam_pool_cidr_allocation;

#[cfg(feature = "vpc_ipam_pool_cidr_allocation")]
pub use vpc_ipam_pool_cidr_allocation::*;

#[cfg(feature = "vpc_ipam_preview_next_cidr")]
pub mod vpc_ipam_preview_next_cidr;

#[cfg(feature = "vpc_ipam_preview_next_cidr")]
pub use vpc_ipam_preview_next_cidr::*;

#[cfg(feature = "vpc_ipam_resource_discovery")]
pub mod vpc_ipam_resource_discovery;

#[cfg(feature = "vpc_ipam_resource_discovery")]
pub use vpc_ipam_resource_discovery::*;

#[cfg(feature = "vpc_ipam_resource_discovery_association")]
pub mod vpc_ipam_resource_discovery_association;

#[cfg(feature = "vpc_ipam_resource_discovery_association")]
pub use vpc_ipam_resource_discovery_association::*;

#[cfg(feature = "vpc_ipam_scope")]
pub mod vpc_ipam_scope;

#[cfg(feature = "vpc_ipam_scope")]
pub use vpc_ipam_scope::*;

#[cfg(feature = "vpc_ipv4_cidr_block_association")]
pub mod vpc_ipv4_cidr_block_association;

#[cfg(feature = "vpc_ipv4_cidr_block_association")]
pub use vpc_ipv4_cidr_block_association::*;

#[cfg(feature = "vpc_ipv6_cidr_block_association")]
pub mod vpc_ipv6_cidr_block_association;

#[cfg(feature = "vpc_ipv6_cidr_block_association")]
pub use vpc_ipv6_cidr_block_association::*;

#[cfg(feature = "vpc_network_performance_metric_subscription")]
pub mod vpc_network_performance_metric_subscription;

#[cfg(feature = "vpc_network_performance_metric_subscription")]
pub use vpc_network_performance_metric_subscription::*;

#[cfg(feature = "vpc_peering_connection")]
pub mod vpc_peering_connection;

#[cfg(feature = "vpc_peering_connection")]
pub use vpc_peering_connection::*;

#[cfg(feature = "vpc_peering_connection_accepter")]
pub mod vpc_peering_connection_accepter;

#[cfg(feature = "vpc_peering_connection_accepter")]
pub use vpc_peering_connection_accepter::*;

#[cfg(feature = "vpc_peering_connection_options")]
pub mod vpc_peering_connection_options;

#[cfg(feature = "vpc_peering_connection_options")]
pub use vpc_peering_connection_options::*;

#[cfg(feature = "vpc_route_server")]
pub mod vpc_route_server;

#[cfg(feature = "vpc_route_server")]
pub use vpc_route_server::*;

#[cfg(feature = "vpc_route_server_endpoint")]
pub mod vpc_route_server_endpoint;

#[cfg(feature = "vpc_route_server_endpoint")]
pub use vpc_route_server_endpoint::*;

#[cfg(feature = "vpc_route_server_peer")]
pub mod vpc_route_server_peer;

#[cfg(feature = "vpc_route_server_peer")]
pub use vpc_route_server_peer::*;

#[cfg(feature = "vpc_route_server_propagation")]
pub mod vpc_route_server_propagation;

#[cfg(feature = "vpc_route_server_propagation")]
pub use vpc_route_server_propagation::*;

#[cfg(feature = "vpc_route_server_vpc_association")]
pub mod vpc_route_server_vpc_association;

#[cfg(feature = "vpc_route_server_vpc_association")]
pub use vpc_route_server_vpc_association::*;

#[cfg(feature = "vpc_security_group_egress_rule")]
pub mod vpc_security_group_egress_rule;

#[cfg(feature = "vpc_security_group_egress_rule")]
pub use vpc_security_group_egress_rule::*;

#[cfg(feature = "vpc_security_group_ingress_rule")]
pub mod vpc_security_group_ingress_rule;

#[cfg(feature = "vpc_security_group_ingress_rule")]
pub use vpc_security_group_ingress_rule::*;

#[cfg(feature = "vpc_security_group_vpc_association")]
pub mod vpc_security_group_vpc_association;

#[cfg(feature = "vpc_security_group_vpc_association")]
pub use vpc_security_group_vpc_association::*;

#[cfg(feature = "vpclattice_access_log_subscription")]
pub mod vpclattice_access_log_subscription;

#[cfg(feature = "vpclattice_access_log_subscription")]
pub use vpclattice_access_log_subscription::*;

#[cfg(feature = "vpclattice_auth_policy")]
pub mod vpclattice_auth_policy;

#[cfg(feature = "vpclattice_auth_policy")]
pub use vpclattice_auth_policy::*;

#[cfg(feature = "vpclattice_domain_verification")]
pub mod vpclattice_domain_verification;

#[cfg(feature = "vpclattice_domain_verification")]
pub use vpclattice_domain_verification::*;

#[cfg(feature = "vpclattice_listener")]
pub mod vpclattice_listener;

#[cfg(feature = "vpclattice_listener")]
pub use vpclattice_listener::*;

#[cfg(feature = "vpclattice_listener_rule")]
pub mod vpclattice_listener_rule;

#[cfg(feature = "vpclattice_listener_rule")]
pub use vpclattice_listener_rule::*;

#[cfg(feature = "vpclattice_resource_configuration")]
pub mod vpclattice_resource_configuration;

#[cfg(feature = "vpclattice_resource_configuration")]
pub use vpclattice_resource_configuration::*;

#[cfg(feature = "vpclattice_resource_gateway")]
pub mod vpclattice_resource_gateway;

#[cfg(feature = "vpclattice_resource_gateway")]
pub use vpclattice_resource_gateway::*;

#[cfg(feature = "vpclattice_resource_policy")]
pub mod vpclattice_resource_policy;

#[cfg(feature = "vpclattice_resource_policy")]
pub use vpclattice_resource_policy::*;

#[cfg(feature = "vpclattice_service")]
pub mod vpclattice_service;

#[cfg(feature = "vpclattice_service")]
pub use vpclattice_service::*;

#[cfg(feature = "vpclattice_service_network")]
pub mod vpclattice_service_network;

#[cfg(feature = "vpclattice_service_network")]
pub use vpclattice_service_network::*;

#[cfg(feature = "vpclattice_service_network_resource_association")]
pub mod vpclattice_service_network_resource_association;

#[cfg(feature = "vpclattice_service_network_resource_association")]
pub use vpclattice_service_network_resource_association::*;

#[cfg(feature = "vpclattice_service_network_service_association")]
pub mod vpclattice_service_network_service_association;

#[cfg(feature = "vpclattice_service_network_service_association")]
pub use vpclattice_service_network_service_association::*;

#[cfg(feature = "vpclattice_service_network_vpc_association")]
pub mod vpclattice_service_network_vpc_association;

#[cfg(feature = "vpclattice_service_network_vpc_association")]
pub use vpclattice_service_network_vpc_association::*;

#[cfg(feature = "vpclattice_target_group")]
pub mod vpclattice_target_group;

#[cfg(feature = "vpclattice_target_group")]
pub use vpclattice_target_group::*;

#[cfg(feature = "vpclattice_target_group_attachment")]
pub mod vpclattice_target_group_attachment;

#[cfg(feature = "vpclattice_target_group_attachment")]
pub use vpclattice_target_group_attachment::*;

#[cfg(feature = "vpn_connection")]
pub mod vpn_connection;

#[cfg(feature = "vpn_connection")]
pub use vpn_connection::*;

#[cfg(feature = "vpn_connection_route")]
pub mod vpn_connection_route;

#[cfg(feature = "vpn_connection_route")]
pub use vpn_connection_route::*;

#[cfg(feature = "vpn_gateway")]
pub mod vpn_gateway;

#[cfg(feature = "vpn_gateway")]
pub use vpn_gateway::*;

#[cfg(feature = "vpn_gateway_attachment")]
pub mod vpn_gateway_attachment;

#[cfg(feature = "vpn_gateway_attachment")]
pub use vpn_gateway_attachment::*;

#[cfg(feature = "vpn_gateway_route_propagation")]
pub mod vpn_gateway_route_propagation;

#[cfg(feature = "vpn_gateway_route_propagation")]
pub use vpn_gateway_route_propagation::*;

#[cfg(feature = "waf_byte_match_set")]
pub mod waf_byte_match_set;

#[cfg(feature = "waf_byte_match_set")]
pub use waf_byte_match_set::*;

#[cfg(feature = "waf_geo_match_set")]
pub mod waf_geo_match_set;

#[cfg(feature = "waf_geo_match_set")]
pub use waf_geo_match_set::*;

#[cfg(feature = "waf_ipset")]
pub mod waf_ipset;

#[cfg(feature = "waf_ipset")]
pub use waf_ipset::*;

#[cfg(feature = "waf_rate_based_rule")]
pub mod waf_rate_based_rule;

#[cfg(feature = "waf_rate_based_rule")]
pub use waf_rate_based_rule::*;

#[cfg(feature = "waf_regex_match_set")]
pub mod waf_regex_match_set;

#[cfg(feature = "waf_regex_match_set")]
pub use waf_regex_match_set::*;

#[cfg(feature = "waf_regex_pattern_set")]
pub mod waf_regex_pattern_set;

#[cfg(feature = "waf_regex_pattern_set")]
pub use waf_regex_pattern_set::*;

#[cfg(feature = "waf_rule")]
pub mod waf_rule;

#[cfg(feature = "waf_rule")]
pub use waf_rule::*;

#[cfg(feature = "waf_rule_group")]
pub mod waf_rule_group;

#[cfg(feature = "waf_rule_group")]
pub use waf_rule_group::*;

#[cfg(feature = "waf_size_constraint_set")]
pub mod waf_size_constraint_set;

#[cfg(feature = "waf_size_constraint_set")]
pub use waf_size_constraint_set::*;

#[cfg(feature = "waf_sql_injection_match_set")]
pub mod waf_sql_injection_match_set;

#[cfg(feature = "waf_sql_injection_match_set")]
pub use waf_sql_injection_match_set::*;

#[cfg(feature = "waf_web_acl")]
pub mod waf_web_acl;

#[cfg(feature = "waf_web_acl")]
pub use waf_web_acl::*;

#[cfg(feature = "waf_xss_match_set")]
pub mod waf_xss_match_set;

#[cfg(feature = "waf_xss_match_set")]
pub use waf_xss_match_set::*;

#[cfg(feature = "wafregional_byte_match_set")]
pub mod wafregional_byte_match_set;

#[cfg(feature = "wafregional_byte_match_set")]
pub use wafregional_byte_match_set::*;

#[cfg(feature = "wafregional_geo_match_set")]
pub mod wafregional_geo_match_set;

#[cfg(feature = "wafregional_geo_match_set")]
pub use wafregional_geo_match_set::*;

#[cfg(feature = "wafregional_ipset")]
pub mod wafregional_ipset;

#[cfg(feature = "wafregional_ipset")]
pub use wafregional_ipset::*;

#[cfg(feature = "wafregional_rate_based_rule")]
pub mod wafregional_rate_based_rule;

#[cfg(feature = "wafregional_rate_based_rule")]
pub use wafregional_rate_based_rule::*;

#[cfg(feature = "wafregional_regex_match_set")]
pub mod wafregional_regex_match_set;

#[cfg(feature = "wafregional_regex_match_set")]
pub use wafregional_regex_match_set::*;

#[cfg(feature = "wafregional_regex_pattern_set")]
pub mod wafregional_regex_pattern_set;

#[cfg(feature = "wafregional_regex_pattern_set")]
pub use wafregional_regex_pattern_set::*;

#[cfg(feature = "wafregional_rule")]
pub mod wafregional_rule;

#[cfg(feature = "wafregional_rule")]
pub use wafregional_rule::*;

#[cfg(feature = "wafregional_rule_group")]
pub mod wafregional_rule_group;

#[cfg(feature = "wafregional_rule_group")]
pub use wafregional_rule_group::*;

#[cfg(feature = "wafregional_size_constraint_set")]
pub mod wafregional_size_constraint_set;

#[cfg(feature = "wafregional_size_constraint_set")]
pub use wafregional_size_constraint_set::*;

#[cfg(feature = "wafregional_sql_injection_match_set")]
pub mod wafregional_sql_injection_match_set;

#[cfg(feature = "wafregional_sql_injection_match_set")]
pub use wafregional_sql_injection_match_set::*;

#[cfg(feature = "wafregional_web_acl")]
pub mod wafregional_web_acl;

#[cfg(feature = "wafregional_web_acl")]
pub use wafregional_web_acl::*;

#[cfg(feature = "wafregional_web_acl_association")]
pub mod wafregional_web_acl_association;

#[cfg(feature = "wafregional_web_acl_association")]
pub use wafregional_web_acl_association::*;

#[cfg(feature = "wafregional_xss_match_set")]
pub mod wafregional_xss_match_set;

#[cfg(feature = "wafregional_xss_match_set")]
pub use wafregional_xss_match_set::*;

#[cfg(feature = "workspaces_connection_alias")]
pub mod workspaces_connection_alias;

#[cfg(feature = "workspaces_connection_alias")]
pub use workspaces_connection_alias::*;

#[cfg(feature = "workspaces_directory")]
pub mod workspaces_directory;

#[cfg(feature = "workspaces_directory")]
pub use workspaces_directory::*;

#[cfg(feature = "workspaces_ip_group")]
pub mod workspaces_ip_group;

#[cfg(feature = "workspaces_ip_group")]
pub use workspaces_ip_group::*;

#[cfg(feature = "workspaces_workspace")]
pub mod workspaces_workspace;

#[cfg(feature = "workspaces_workspace")]
pub use workspaces_workspace::*;

#[cfg(feature = "workspacesweb_browser_settings")]
pub mod workspacesweb_browser_settings;

#[cfg(feature = "workspacesweb_browser_settings")]
pub use workspacesweb_browser_settings::*;

#[cfg(feature = "workspacesweb_browser_settings_association")]
pub mod workspacesweb_browser_settings_association;

#[cfg(feature = "workspacesweb_browser_settings_association")]
pub use workspacesweb_browser_settings_association::*;

#[cfg(feature = "workspacesweb_data_protection_settings")]
pub mod workspacesweb_data_protection_settings;

#[cfg(feature = "workspacesweb_data_protection_settings")]
pub use workspacesweb_data_protection_settings::*;

#[cfg(feature = "workspacesweb_data_protection_settings_association")]
pub mod workspacesweb_data_protection_settings_association;

#[cfg(feature = "workspacesweb_data_protection_settings_association")]
pub use workspacesweb_data_protection_settings_association::*;

#[cfg(feature = "workspacesweb_identity_provider")]
pub mod workspacesweb_identity_provider;

#[cfg(feature = "workspacesweb_identity_provider")]
pub use workspacesweb_identity_provider::*;

#[cfg(feature = "workspacesweb_ip_access_settings")]
pub mod workspacesweb_ip_access_settings;

#[cfg(feature = "workspacesweb_ip_access_settings")]
pub use workspacesweb_ip_access_settings::*;

#[cfg(feature = "workspacesweb_ip_access_settings_association")]
pub mod workspacesweb_ip_access_settings_association;

#[cfg(feature = "workspacesweb_ip_access_settings_association")]
pub use workspacesweb_ip_access_settings_association::*;

#[cfg(feature = "workspacesweb_network_settings")]
pub mod workspacesweb_network_settings;

#[cfg(feature = "workspacesweb_network_settings")]
pub use workspacesweb_network_settings::*;

#[cfg(feature = "workspacesweb_network_settings_association")]
pub mod workspacesweb_network_settings_association;

#[cfg(feature = "workspacesweb_network_settings_association")]
pub use workspacesweb_network_settings_association::*;

#[cfg(feature = "workspacesweb_portal")]
pub mod workspacesweb_portal;

#[cfg(feature = "workspacesweb_portal")]
pub use workspacesweb_portal::*;

#[cfg(feature = "workspacesweb_session_logger")]
pub mod workspacesweb_session_logger;

#[cfg(feature = "workspacesweb_session_logger")]
pub use workspacesweb_session_logger::*;

#[cfg(feature = "workspacesweb_session_logger_association")]
pub mod workspacesweb_session_logger_association;

#[cfg(feature = "workspacesweb_session_logger_association")]
pub use workspacesweb_session_logger_association::*;

#[cfg(feature = "workspacesweb_trust_store")]
pub mod workspacesweb_trust_store;

#[cfg(feature = "workspacesweb_trust_store")]
pub use workspacesweb_trust_store::*;

#[cfg(feature = "workspacesweb_trust_store_association")]
pub mod workspacesweb_trust_store_association;

#[cfg(feature = "workspacesweb_trust_store_association")]
pub use workspacesweb_trust_store_association::*;

#[cfg(feature = "workspacesweb_user_access_logging_settings")]
pub mod workspacesweb_user_access_logging_settings;

#[cfg(feature = "workspacesweb_user_access_logging_settings")]
pub use workspacesweb_user_access_logging_settings::*;

#[cfg(feature = "workspacesweb_user_access_logging_settings_association")]
pub mod workspacesweb_user_access_logging_settings_association;

#[cfg(feature = "workspacesweb_user_access_logging_settings_association")]
pub use workspacesweb_user_access_logging_settings_association::*;

#[cfg(feature = "workspacesweb_user_settings")]
pub mod workspacesweb_user_settings;

#[cfg(feature = "workspacesweb_user_settings")]
pub use workspacesweb_user_settings::*;

#[cfg(feature = "workspacesweb_user_settings_association")]
pub mod workspacesweb_user_settings_association;

#[cfg(feature = "workspacesweb_user_settings_association")]
pub use workspacesweb_user_settings_association::*;

#[cfg(feature = "xray_encryption_config")]
pub mod xray_encryption_config;

#[cfg(feature = "xray_encryption_config")]
pub use xray_encryption_config::*;

#[cfg(feature = "xray_group")]
pub mod xray_group;

#[cfg(feature = "xray_group")]
pub use xray_group::*;

#[cfg(feature = "xray_resource_policy")]
pub mod xray_resource_policy;

#[cfg(feature = "xray_resource_policy")]
pub use xray_resource_policy::*;

#[cfg(feature = "xray_sampling_rule")]
pub mod xray_sampling_rule;

#[cfg(feature = "xray_sampling_rule")]
pub use xray_sampling_rule::*;

#[cfg(feature = "data_account_primary_contact")]
pub mod data_account_primary_contact;

#[cfg(feature = "data_account_primary_contact")]
pub use data_account_primary_contact::*;

#[cfg(feature = "data_acm_certificate")]
pub mod data_acm_certificate;

#[cfg(feature = "data_acm_certificate")]
pub use data_acm_certificate::*;

#[cfg(feature = "data_acmpca_certificate")]
pub mod data_acmpca_certificate;

#[cfg(feature = "data_acmpca_certificate")]
pub use data_acmpca_certificate::*;

#[cfg(feature = "data_acmpca_certificate_authority")]
pub mod data_acmpca_certificate_authority;

#[cfg(feature = "data_acmpca_certificate_authority")]
pub use data_acmpca_certificate_authority::*;

#[cfg(feature = "data_alb")]
pub mod data_alb;

#[cfg(feature = "data_alb")]
pub use data_alb::*;

#[cfg(feature = "data_alb_listener")]
pub mod data_alb_listener;

#[cfg(feature = "data_alb_listener")]
pub use data_alb_listener::*;

#[cfg(feature = "data_alb_target_group")]
pub mod data_alb_target_group;

#[cfg(feature = "data_alb_target_group")]
pub use data_alb_target_group::*;

#[cfg(feature = "data_ami")]
pub mod data_ami;

#[cfg(feature = "data_ami")]
pub use data_ami::*;

#[cfg(feature = "data_ami_ids")]
pub mod data_ami_ids;

#[cfg(feature = "data_ami_ids")]
pub use data_ami_ids::*;

#[cfg(feature = "data_api_gateway_api_key")]
pub mod data_api_gateway_api_key;

#[cfg(feature = "data_api_gateway_api_key")]
pub use data_api_gateway_api_key::*;

#[cfg(feature = "data_api_gateway_api_keys")]
pub mod data_api_gateway_api_keys;

#[cfg(feature = "data_api_gateway_api_keys")]
pub use data_api_gateway_api_keys::*;

#[cfg(feature = "data_api_gateway_authorizer")]
pub mod data_api_gateway_authorizer;

#[cfg(feature = "data_api_gateway_authorizer")]
pub use data_api_gateway_authorizer::*;

#[cfg(feature = "data_api_gateway_authorizers")]
pub mod data_api_gateway_authorizers;

#[cfg(feature = "data_api_gateway_authorizers")]
pub use data_api_gateway_authorizers::*;

#[cfg(feature = "data_api_gateway_domain_name")]
pub mod data_api_gateway_domain_name;

#[cfg(feature = "data_api_gateway_domain_name")]
pub use data_api_gateway_domain_name::*;

#[cfg(feature = "data_api_gateway_export")]
pub mod data_api_gateway_export;

#[cfg(feature = "data_api_gateway_export")]
pub use data_api_gateway_export::*;

#[cfg(feature = "data_api_gateway_resource")]
pub mod data_api_gateway_resource;

#[cfg(feature = "data_api_gateway_resource")]
pub use data_api_gateway_resource::*;

#[cfg(feature = "data_api_gateway_rest_api")]
pub mod data_api_gateway_rest_api;

#[cfg(feature = "data_api_gateway_rest_api")]
pub use data_api_gateway_rest_api::*;

#[cfg(feature = "data_api_gateway_sdk")]
pub mod data_api_gateway_sdk;

#[cfg(feature = "data_api_gateway_sdk")]
pub use data_api_gateway_sdk::*;

#[cfg(feature = "data_api_gateway_vpc_link")]
pub mod data_api_gateway_vpc_link;

#[cfg(feature = "data_api_gateway_vpc_link")]
pub use data_api_gateway_vpc_link::*;

#[cfg(feature = "data_apigatewayv2_api")]
pub mod data_apigatewayv2_api;

#[cfg(feature = "data_apigatewayv2_api")]
pub use data_apigatewayv2_api::*;

#[cfg(feature = "data_apigatewayv2_apis")]
pub mod data_apigatewayv2_apis;

#[cfg(feature = "data_apigatewayv2_apis")]
pub use data_apigatewayv2_apis::*;

#[cfg(feature = "data_apigatewayv2_export")]
pub mod data_apigatewayv2_export;

#[cfg(feature = "data_apigatewayv2_export")]
pub use data_apigatewayv2_export::*;

#[cfg(feature = "data_apigatewayv2_vpc_link")]
pub mod data_apigatewayv2_vpc_link;

#[cfg(feature = "data_apigatewayv2_vpc_link")]
pub use data_apigatewayv2_vpc_link::*;

#[cfg(feature = "data_appconfig_application")]
pub mod data_appconfig_application;

#[cfg(feature = "data_appconfig_application")]
pub use data_appconfig_application::*;

#[cfg(feature = "data_appconfig_configuration_profile")]
pub mod data_appconfig_configuration_profile;

#[cfg(feature = "data_appconfig_configuration_profile")]
pub use data_appconfig_configuration_profile::*;

#[cfg(feature = "data_appconfig_configuration_profiles")]
pub mod data_appconfig_configuration_profiles;

#[cfg(feature = "data_appconfig_configuration_profiles")]
pub use data_appconfig_configuration_profiles::*;

#[cfg(feature = "data_appconfig_environment")]
pub mod data_appconfig_environment;

#[cfg(feature = "data_appconfig_environment")]
pub use data_appconfig_environment::*;

#[cfg(feature = "data_appconfig_environments")]
pub mod data_appconfig_environments;

#[cfg(feature = "data_appconfig_environments")]
pub use data_appconfig_environments::*;

#[cfg(feature = "data_appintegrations_event_integration")]
pub mod data_appintegrations_event_integration;

#[cfg(feature = "data_appintegrations_event_integration")]
pub use data_appintegrations_event_integration::*;

#[cfg(feature = "data_appmesh_gateway_route")]
pub mod data_appmesh_gateway_route;

#[cfg(feature = "data_appmesh_gateway_route")]
pub use data_appmesh_gateway_route::*;

#[cfg(feature = "data_appmesh_mesh")]
pub mod data_appmesh_mesh;

#[cfg(feature = "data_appmesh_mesh")]
pub use data_appmesh_mesh::*;

#[cfg(feature = "data_appmesh_route")]
pub mod data_appmesh_route;

#[cfg(feature = "data_appmesh_route")]
pub use data_appmesh_route::*;

#[cfg(feature = "data_appmesh_virtual_gateway")]
pub mod data_appmesh_virtual_gateway;

#[cfg(feature = "data_appmesh_virtual_gateway")]
pub use data_appmesh_virtual_gateway::*;

#[cfg(feature = "data_appmesh_virtual_node")]
pub mod data_appmesh_virtual_node;

#[cfg(feature = "data_appmesh_virtual_node")]
pub use data_appmesh_virtual_node::*;

#[cfg(feature = "data_appmesh_virtual_router")]
pub mod data_appmesh_virtual_router;

#[cfg(feature = "data_appmesh_virtual_router")]
pub use data_appmesh_virtual_router::*;

#[cfg(feature = "data_appmesh_virtual_service")]
pub mod data_appmesh_virtual_service;

#[cfg(feature = "data_appmesh_virtual_service")]
pub use data_appmesh_virtual_service::*;

#[cfg(feature = "data_apprunner_hosted_zone_id")]
pub mod data_apprunner_hosted_zone_id;

#[cfg(feature = "data_apprunner_hosted_zone_id")]
pub use data_apprunner_hosted_zone_id::*;

#[cfg(feature = "data_appstream_image")]
pub mod data_appstream_image;

#[cfg(feature = "data_appstream_image")]
pub use data_appstream_image::*;

#[cfg(feature = "data_arn")]
pub mod data_arn;

#[cfg(feature = "data_arn")]
pub use data_arn::*;

#[cfg(feature = "data_athena_named_query")]
pub mod data_athena_named_query;

#[cfg(feature = "data_athena_named_query")]
pub use data_athena_named_query::*;

#[cfg(feature = "data_auditmanager_control")]
pub mod data_auditmanager_control;

#[cfg(feature = "data_auditmanager_control")]
pub use data_auditmanager_control::*;

#[cfg(feature = "data_auditmanager_framework")]
pub mod data_auditmanager_framework;

#[cfg(feature = "data_auditmanager_framework")]
pub use data_auditmanager_framework::*;

#[cfg(feature = "data_autoscaling_group")]
pub mod data_autoscaling_group;

#[cfg(feature = "data_autoscaling_group")]
pub use data_autoscaling_group::*;

#[cfg(feature = "data_autoscaling_groups")]
pub mod data_autoscaling_groups;

#[cfg(feature = "data_autoscaling_groups")]
pub use data_autoscaling_groups::*;

#[cfg(feature = "data_availability_zone")]
pub mod data_availability_zone;

#[cfg(feature = "data_availability_zone")]
pub use data_availability_zone::*;

#[cfg(feature = "data_availability_zones")]
pub mod data_availability_zones;

#[cfg(feature = "data_availability_zones")]
pub use data_availability_zones::*;

#[cfg(feature = "data_backup_framework")]
pub mod data_backup_framework;

#[cfg(feature = "data_backup_framework")]
pub use data_backup_framework::*;

#[cfg(feature = "data_backup_plan")]
pub mod data_backup_plan;

#[cfg(feature = "data_backup_plan")]
pub use data_backup_plan::*;

#[cfg(feature = "data_backup_report_plan")]
pub mod data_backup_report_plan;

#[cfg(feature = "data_backup_report_plan")]
pub use data_backup_report_plan::*;

#[cfg(feature = "data_backup_selection")]
pub mod data_backup_selection;

#[cfg(feature = "data_backup_selection")]
pub use data_backup_selection::*;

#[cfg(feature = "data_backup_vault")]
pub mod data_backup_vault;

#[cfg(feature = "data_backup_vault")]
pub use data_backup_vault::*;

#[cfg(feature = "data_batch_compute_environment")]
pub mod data_batch_compute_environment;

#[cfg(feature = "data_batch_compute_environment")]
pub use data_batch_compute_environment::*;

#[cfg(feature = "data_batch_job_definition")]
pub mod data_batch_job_definition;

#[cfg(feature = "data_batch_job_definition")]
pub use data_batch_job_definition::*;

#[cfg(feature = "data_batch_job_queue")]
pub mod data_batch_job_queue;

#[cfg(feature = "data_batch_job_queue")]
pub use data_batch_job_queue::*;

#[cfg(feature = "data_batch_scheduling_policy")]
pub mod data_batch_scheduling_policy;

#[cfg(feature = "data_batch_scheduling_policy")]
pub use data_batch_scheduling_policy::*;

#[cfg(feature = "data_bedrock_custom_model")]
pub mod data_bedrock_custom_model;

#[cfg(feature = "data_bedrock_custom_model")]
pub use data_bedrock_custom_model::*;

#[cfg(feature = "data_bedrock_custom_models")]
pub mod data_bedrock_custom_models;

#[cfg(feature = "data_bedrock_custom_models")]
pub use data_bedrock_custom_models::*;

#[cfg(feature = "data_bedrock_foundation_model")]
pub mod data_bedrock_foundation_model;

#[cfg(feature = "data_bedrock_foundation_model")]
pub use data_bedrock_foundation_model::*;

#[cfg(feature = "data_bedrock_foundation_models")]
pub mod data_bedrock_foundation_models;

#[cfg(feature = "data_bedrock_foundation_models")]
pub use data_bedrock_foundation_models::*;

#[cfg(feature = "data_bedrock_inference_profile")]
pub mod data_bedrock_inference_profile;

#[cfg(feature = "data_bedrock_inference_profile")]
pub use data_bedrock_inference_profile::*;

#[cfg(feature = "data_bedrock_inference_profiles")]
pub mod data_bedrock_inference_profiles;

#[cfg(feature = "data_bedrock_inference_profiles")]
pub use data_bedrock_inference_profiles::*;

#[cfg(feature = "data_bedrockagent_agent_versions")]
pub mod data_bedrockagent_agent_versions;

#[cfg(feature = "data_bedrockagent_agent_versions")]
pub use data_bedrockagent_agent_versions::*;

#[cfg(feature = "data_billing_service_account")]
pub mod data_billing_service_account;

#[cfg(feature = "data_billing_service_account")]
pub use data_billing_service_account::*;

#[cfg(feature = "data_billing_views")]
pub mod data_billing_views;

#[cfg(feature = "data_billing_views")]
pub use data_billing_views::*;

#[cfg(feature = "data_budgets_budget")]
pub mod data_budgets_budget;

#[cfg(feature = "data_budgets_budget")]
pub use data_budgets_budget::*;

#[cfg(feature = "data_caller_identity")]
pub mod data_caller_identity;

#[cfg(feature = "data_caller_identity")]
pub use data_caller_identity::*;

#[cfg(feature = "data_canonical_user_id")]
pub mod data_canonical_user_id;

#[cfg(feature = "data_canonical_user_id")]
pub use data_canonical_user_id::*;

#[cfg(feature = "data_ce_cost_category")]
pub mod data_ce_cost_category;

#[cfg(feature = "data_ce_cost_category")]
pub use data_ce_cost_category::*;

#[cfg(feature = "data_ce_tags")]
pub mod data_ce_tags;

#[cfg(feature = "data_ce_tags")]
pub use data_ce_tags::*;

#[cfg(feature = "data_chatbot_slack_workspace")]
pub mod data_chatbot_slack_workspace;

#[cfg(feature = "data_chatbot_slack_workspace")]
pub use data_chatbot_slack_workspace::*;

#[cfg(feature = "data_cloudcontrolapi_resource")]
pub mod data_cloudcontrolapi_resource;

#[cfg(feature = "data_cloudcontrolapi_resource")]
pub use data_cloudcontrolapi_resource::*;

#[cfg(feature = "data_cloudformation_export")]
pub mod data_cloudformation_export;

#[cfg(feature = "data_cloudformation_export")]
pub use data_cloudformation_export::*;

#[cfg(feature = "data_cloudformation_stack")]
pub mod data_cloudformation_stack;

#[cfg(feature = "data_cloudformation_stack")]
pub use data_cloudformation_stack::*;

#[cfg(feature = "data_cloudformation_type")]
pub mod data_cloudformation_type;

#[cfg(feature = "data_cloudformation_type")]
pub use data_cloudformation_type::*;

#[cfg(feature = "data_cloudfront_cache_policy")]
pub mod data_cloudfront_cache_policy;

#[cfg(feature = "data_cloudfront_cache_policy")]
pub use data_cloudfront_cache_policy::*;

#[cfg(feature = "data_cloudfront_distribution")]
pub mod data_cloudfront_distribution;

#[cfg(feature = "data_cloudfront_distribution")]
pub use data_cloudfront_distribution::*;

#[cfg(feature = "data_cloudfront_function")]
pub mod data_cloudfront_function;

#[cfg(feature = "data_cloudfront_function")]
pub use data_cloudfront_function::*;

#[cfg(feature = "data_cloudfront_log_delivery_canonical_user_id")]
pub mod data_cloudfront_log_delivery_canonical_user_id;

#[cfg(feature = "data_cloudfront_log_delivery_canonical_user_id")]
pub use data_cloudfront_log_delivery_canonical_user_id::*;

#[cfg(feature = "data_cloudfront_origin_access_control")]
pub mod data_cloudfront_origin_access_control;

#[cfg(feature = "data_cloudfront_origin_access_control")]
pub use data_cloudfront_origin_access_control::*;

#[cfg(feature = "data_cloudfront_origin_access_identities")]
pub mod data_cloudfront_origin_access_identities;

#[cfg(feature = "data_cloudfront_origin_access_identities")]
pub use data_cloudfront_origin_access_identities::*;

#[cfg(feature = "data_cloudfront_origin_access_identity")]
pub mod data_cloudfront_origin_access_identity;

#[cfg(feature = "data_cloudfront_origin_access_identity")]
pub use data_cloudfront_origin_access_identity::*;

#[cfg(feature = "data_cloudfront_origin_request_policy")]
pub mod data_cloudfront_origin_request_policy;

#[cfg(feature = "data_cloudfront_origin_request_policy")]
pub use data_cloudfront_origin_request_policy::*;

#[cfg(feature = "data_cloudfront_realtime_log_config")]
pub mod data_cloudfront_realtime_log_config;

#[cfg(feature = "data_cloudfront_realtime_log_config")]
pub use data_cloudfront_realtime_log_config::*;

#[cfg(feature = "data_cloudfront_response_headers_policy")]
pub mod data_cloudfront_response_headers_policy;

#[cfg(feature = "data_cloudfront_response_headers_policy")]
pub use data_cloudfront_response_headers_policy::*;

#[cfg(feature = "data_cloudhsm_v2_cluster")]
pub mod data_cloudhsm_v2_cluster;

#[cfg(feature = "data_cloudhsm_v2_cluster")]
pub use data_cloudhsm_v2_cluster::*;

#[cfg(feature = "data_cloudtrail_service_account")]
pub mod data_cloudtrail_service_account;

#[cfg(feature = "data_cloudtrail_service_account")]
pub use data_cloudtrail_service_account::*;

#[cfg(feature = "data_cloudwatch_contributor_managed_insight_rules")]
pub mod data_cloudwatch_contributor_managed_insight_rules;

#[cfg(feature = "data_cloudwatch_contributor_managed_insight_rules")]
pub use data_cloudwatch_contributor_managed_insight_rules::*;

#[cfg(feature = "data_cloudwatch_event_bus")]
pub mod data_cloudwatch_event_bus;

#[cfg(feature = "data_cloudwatch_event_bus")]
pub use data_cloudwatch_event_bus::*;

#[cfg(feature = "data_cloudwatch_event_buses")]
pub mod data_cloudwatch_event_buses;

#[cfg(feature = "data_cloudwatch_event_buses")]
pub use data_cloudwatch_event_buses::*;

#[cfg(feature = "data_cloudwatch_event_connection")]
pub mod data_cloudwatch_event_connection;

#[cfg(feature = "data_cloudwatch_event_connection")]
pub use data_cloudwatch_event_connection::*;

#[cfg(feature = "data_cloudwatch_event_source")]
pub mod data_cloudwatch_event_source;

#[cfg(feature = "data_cloudwatch_event_source")]
pub use data_cloudwatch_event_source::*;

#[cfg(feature = "data_cloudwatch_log_data_protection_policy_document")]
pub mod data_cloudwatch_log_data_protection_policy_document;

#[cfg(feature = "data_cloudwatch_log_data_protection_policy_document")]
pub use data_cloudwatch_log_data_protection_policy_document::*;

#[cfg(feature = "data_cloudwatch_log_group")]
pub mod data_cloudwatch_log_group;

#[cfg(feature = "data_cloudwatch_log_group")]
pub use data_cloudwatch_log_group::*;

#[cfg(feature = "data_cloudwatch_log_groups")]
pub mod data_cloudwatch_log_groups;

#[cfg(feature = "data_cloudwatch_log_groups")]
pub use data_cloudwatch_log_groups::*;

#[cfg(feature = "data_codeartifact_authorization_token")]
pub mod data_codeartifact_authorization_token;

#[cfg(feature = "data_codeartifact_authorization_token")]
pub use data_codeartifact_authorization_token::*;

#[cfg(feature = "data_codeartifact_repository_endpoint")]
pub mod data_codeartifact_repository_endpoint;

#[cfg(feature = "data_codeartifact_repository_endpoint")]
pub use data_codeartifact_repository_endpoint::*;

#[cfg(feature = "data_codebuild_fleet")]
pub mod data_codebuild_fleet;

#[cfg(feature = "data_codebuild_fleet")]
pub use data_codebuild_fleet::*;

#[cfg(feature = "data_codecatalyst_dev_environment")]
pub mod data_codecatalyst_dev_environment;

#[cfg(feature = "data_codecatalyst_dev_environment")]
pub use data_codecatalyst_dev_environment::*;

#[cfg(feature = "data_codecommit_approval_rule_template")]
pub mod data_codecommit_approval_rule_template;

#[cfg(feature = "data_codecommit_approval_rule_template")]
pub use data_codecommit_approval_rule_template::*;

#[cfg(feature = "data_codecommit_repository")]
pub mod data_codecommit_repository;

#[cfg(feature = "data_codecommit_repository")]
pub use data_codecommit_repository::*;

#[cfg(feature = "data_codeguruprofiler_profiling_group")]
pub mod data_codeguruprofiler_profiling_group;

#[cfg(feature = "data_codeguruprofiler_profiling_group")]
pub use data_codeguruprofiler_profiling_group::*;

#[cfg(feature = "data_codestarconnections_connection")]
pub mod data_codestarconnections_connection;

#[cfg(feature = "data_codestarconnections_connection")]
pub use data_codestarconnections_connection::*;

#[cfg(feature = "data_cognito_identity_pool")]
pub mod data_cognito_identity_pool;

#[cfg(feature = "data_cognito_identity_pool")]
pub use data_cognito_identity_pool::*;

#[cfg(feature = "data_cognito_user_group")]
pub mod data_cognito_user_group;

#[cfg(feature = "data_cognito_user_group")]
pub use data_cognito_user_group::*;

#[cfg(feature = "data_cognito_user_groups")]
pub mod data_cognito_user_groups;

#[cfg(feature = "data_cognito_user_groups")]
pub use data_cognito_user_groups::*;

#[cfg(feature = "data_cognito_user_pool")]
pub mod data_cognito_user_pool;

#[cfg(feature = "data_cognito_user_pool")]
pub use data_cognito_user_pool::*;

#[cfg(feature = "data_cognito_user_pool_client")]
pub mod data_cognito_user_pool_client;

#[cfg(feature = "data_cognito_user_pool_client")]
pub use data_cognito_user_pool_client::*;

#[cfg(feature = "data_cognito_user_pool_clients")]
pub mod data_cognito_user_pool_clients;

#[cfg(feature = "data_cognito_user_pool_clients")]
pub use data_cognito_user_pool_clients::*;

#[cfg(feature = "data_cognito_user_pool_signing_certificate")]
pub mod data_cognito_user_pool_signing_certificate;

#[cfg(feature = "data_cognito_user_pool_signing_certificate")]
pub use data_cognito_user_pool_signing_certificate::*;

#[cfg(feature = "data_cognito_user_pools")]
pub mod data_cognito_user_pools;

#[cfg(feature = "data_cognito_user_pools")]
pub use data_cognito_user_pools::*;

#[cfg(feature = "data_connect_bot_association")]
pub mod data_connect_bot_association;

#[cfg(feature = "data_connect_bot_association")]
pub use data_connect_bot_association::*;

#[cfg(feature = "data_connect_contact_flow")]
pub mod data_connect_contact_flow;

#[cfg(feature = "data_connect_contact_flow")]
pub use data_connect_contact_flow::*;

#[cfg(feature = "data_connect_contact_flow_module")]
pub mod data_connect_contact_flow_module;

#[cfg(feature = "data_connect_contact_flow_module")]
pub use data_connect_contact_flow_module::*;

#[cfg(feature = "data_connect_hours_of_operation")]
pub mod data_connect_hours_of_operation;

#[cfg(feature = "data_connect_hours_of_operation")]
pub use data_connect_hours_of_operation::*;

#[cfg(feature = "data_connect_instance")]
pub mod data_connect_instance;

#[cfg(feature = "data_connect_instance")]
pub use data_connect_instance::*;

#[cfg(feature = "data_connect_instance_storage_config")]
pub mod data_connect_instance_storage_config;

#[cfg(feature = "data_connect_instance_storage_config")]
pub use data_connect_instance_storage_config::*;

#[cfg(feature = "data_connect_lambda_function_association")]
pub mod data_connect_lambda_function_association;

#[cfg(feature = "data_connect_lambda_function_association")]
pub use data_connect_lambda_function_association::*;

#[cfg(feature = "data_connect_prompt")]
pub mod data_connect_prompt;

#[cfg(feature = "data_connect_prompt")]
pub use data_connect_prompt::*;

#[cfg(feature = "data_connect_queue")]
pub mod data_connect_queue;

#[cfg(feature = "data_connect_queue")]
pub use data_connect_queue::*;

#[cfg(feature = "data_connect_quick_connect")]
pub mod data_connect_quick_connect;

#[cfg(feature = "data_connect_quick_connect")]
pub use data_connect_quick_connect::*;

#[cfg(feature = "data_connect_routing_profile")]
pub mod data_connect_routing_profile;

#[cfg(feature = "data_connect_routing_profile")]
pub use data_connect_routing_profile::*;

#[cfg(feature = "data_connect_security_profile")]
pub mod data_connect_security_profile;

#[cfg(feature = "data_connect_security_profile")]
pub use data_connect_security_profile::*;

#[cfg(feature = "data_connect_user")]
pub mod data_connect_user;

#[cfg(feature = "data_connect_user")]
pub use data_connect_user::*;

#[cfg(feature = "data_connect_user_hierarchy_group")]
pub mod data_connect_user_hierarchy_group;

#[cfg(feature = "data_connect_user_hierarchy_group")]
pub use data_connect_user_hierarchy_group::*;

#[cfg(feature = "data_connect_user_hierarchy_structure")]
pub mod data_connect_user_hierarchy_structure;

#[cfg(feature = "data_connect_user_hierarchy_structure")]
pub use data_connect_user_hierarchy_structure::*;

#[cfg(feature = "data_connect_vocabulary")]
pub mod data_connect_vocabulary;

#[cfg(feature = "data_connect_vocabulary")]
pub use data_connect_vocabulary::*;

#[cfg(feature = "data_controltower_controls")]
pub mod data_controltower_controls;

#[cfg(feature = "data_controltower_controls")]
pub use data_controltower_controls::*;

#[cfg(feature = "data_cur_report_definition")]
pub mod data_cur_report_definition;

#[cfg(feature = "data_cur_report_definition")]
pub use data_cur_report_definition::*;

#[cfg(feature = "data_customer_gateway")]
pub mod data_customer_gateway;

#[cfg(feature = "data_customer_gateway")]
pub use data_customer_gateway::*;

#[cfg(feature = "data_datapipeline_pipeline")]
pub mod data_datapipeline_pipeline;

#[cfg(feature = "data_datapipeline_pipeline")]
pub use data_datapipeline_pipeline::*;

#[cfg(feature = "data_datapipeline_pipeline_definition")]
pub mod data_datapipeline_pipeline_definition;

#[cfg(feature = "data_datapipeline_pipeline_definition")]
pub use data_datapipeline_pipeline_definition::*;

#[cfg(feature = "data_db_cluster_snapshot")]
pub mod data_db_cluster_snapshot;

#[cfg(feature = "data_db_cluster_snapshot")]
pub use data_db_cluster_snapshot::*;

#[cfg(feature = "data_db_event_categories")]
pub mod data_db_event_categories;

#[cfg(feature = "data_db_event_categories")]
pub use data_db_event_categories::*;

#[cfg(feature = "data_db_instance")]
pub mod data_db_instance;

#[cfg(feature = "data_db_instance")]
pub use data_db_instance::*;

#[cfg(feature = "data_db_instances")]
pub mod data_db_instances;

#[cfg(feature = "data_db_instances")]
pub use data_db_instances::*;

#[cfg(feature = "data_db_parameter_group")]
pub mod data_db_parameter_group;

#[cfg(feature = "data_db_parameter_group")]
pub use data_db_parameter_group::*;

#[cfg(feature = "data_db_proxy")]
pub mod data_db_proxy;

#[cfg(feature = "data_db_proxy")]
pub use data_db_proxy::*;

#[cfg(feature = "data_db_snapshot")]
pub mod data_db_snapshot;

#[cfg(feature = "data_db_snapshot")]
pub use data_db_snapshot::*;

#[cfg(feature = "data_db_subnet_group")]
pub mod data_db_subnet_group;

#[cfg(feature = "data_db_subnet_group")]
pub use data_db_subnet_group::*;

#[cfg(feature = "data_default_tags")]
pub mod data_default_tags;

#[cfg(feature = "data_default_tags")]
pub use data_default_tags::*;

#[cfg(feature = "data_devopsguru_notification_channel")]
pub mod data_devopsguru_notification_channel;

#[cfg(feature = "data_devopsguru_notification_channel")]
pub use data_devopsguru_notification_channel::*;

#[cfg(feature = "data_devopsguru_resource_collection")]
pub mod data_devopsguru_resource_collection;

#[cfg(feature = "data_devopsguru_resource_collection")]
pub use data_devopsguru_resource_collection::*;

#[cfg(feature = "data_directory_service_directory")]
pub mod data_directory_service_directory;

#[cfg(feature = "data_directory_service_directory")]
pub use data_directory_service_directory::*;

#[cfg(feature = "data_dms_certificate")]
pub mod data_dms_certificate;

#[cfg(feature = "data_dms_certificate")]
pub use data_dms_certificate::*;

#[cfg(feature = "data_dms_endpoint")]
pub mod data_dms_endpoint;

#[cfg(feature = "data_dms_endpoint")]
pub use data_dms_endpoint::*;

#[cfg(feature = "data_dms_replication_instance")]
pub mod data_dms_replication_instance;

#[cfg(feature = "data_dms_replication_instance")]
pub use data_dms_replication_instance::*;

#[cfg(feature = "data_dms_replication_subnet_group")]
pub mod data_dms_replication_subnet_group;

#[cfg(feature = "data_dms_replication_subnet_group")]
pub use data_dms_replication_subnet_group::*;

#[cfg(feature = "data_dms_replication_task")]
pub mod data_dms_replication_task;

#[cfg(feature = "data_dms_replication_task")]
pub use data_dms_replication_task::*;

#[cfg(feature = "data_docdb_engine_version")]
pub mod data_docdb_engine_version;

#[cfg(feature = "data_docdb_engine_version")]
pub use data_docdb_engine_version::*;

#[cfg(feature = "data_docdb_orderable_db_instance")]
pub mod data_docdb_orderable_db_instance;

#[cfg(feature = "data_docdb_orderable_db_instance")]
pub use data_docdb_orderable_db_instance::*;

#[cfg(feature = "data_dx_connection")]
pub mod data_dx_connection;

#[cfg(feature = "data_dx_connection")]
pub use data_dx_connection::*;

#[cfg(feature = "data_dx_gateway")]
pub mod data_dx_gateway;

#[cfg(feature = "data_dx_gateway")]
pub use data_dx_gateway::*;

#[cfg(feature = "data_dx_location")]
pub mod data_dx_location;

#[cfg(feature = "data_dx_location")]
pub use data_dx_location::*;

#[cfg(feature = "data_dx_locations")]
pub mod data_dx_locations;

#[cfg(feature = "data_dx_locations")]
pub use data_dx_locations::*;

#[cfg(feature = "data_dx_router_configuration")]
pub mod data_dx_router_configuration;

#[cfg(feature = "data_dx_router_configuration")]
pub use data_dx_router_configuration::*;

#[cfg(feature = "data_dynamodb_table")]
pub mod data_dynamodb_table;

#[cfg(feature = "data_dynamodb_table")]
pub use data_dynamodb_table::*;

#[cfg(feature = "data_dynamodb_table_item")]
pub mod data_dynamodb_table_item;

#[cfg(feature = "data_dynamodb_table_item")]
pub use data_dynamodb_table_item::*;

#[cfg(feature = "data_dynamodb_tables")]
pub mod data_dynamodb_tables;

#[cfg(feature = "data_dynamodb_tables")]
pub use data_dynamodb_tables::*;

#[cfg(feature = "data_ebs_default_kms_key")]
pub mod data_ebs_default_kms_key;

#[cfg(feature = "data_ebs_default_kms_key")]
pub use data_ebs_default_kms_key::*;

#[cfg(feature = "data_ebs_encryption_by_default")]
pub mod data_ebs_encryption_by_default;

#[cfg(feature = "data_ebs_encryption_by_default")]
pub use data_ebs_encryption_by_default::*;

#[cfg(feature = "data_ebs_snapshot")]
pub mod data_ebs_snapshot;

#[cfg(feature = "data_ebs_snapshot")]
pub use data_ebs_snapshot::*;

#[cfg(feature = "data_ebs_snapshot_ids")]
pub mod data_ebs_snapshot_ids;

#[cfg(feature = "data_ebs_snapshot_ids")]
pub use data_ebs_snapshot_ids::*;

#[cfg(feature = "data_ebs_volume")]
pub mod data_ebs_volume;

#[cfg(feature = "data_ebs_volume")]
pub use data_ebs_volume::*;

#[cfg(feature = "data_ebs_volumes")]
pub mod data_ebs_volumes;

#[cfg(feature = "data_ebs_volumes")]
pub use data_ebs_volumes::*;

#[cfg(feature = "data_ec2_capacity_block_offering")]
pub mod data_ec2_capacity_block_offering;

#[cfg(feature = "data_ec2_capacity_block_offering")]
pub use data_ec2_capacity_block_offering::*;

#[cfg(feature = "data_ec2_client_vpn_endpoint")]
pub mod data_ec2_client_vpn_endpoint;

#[cfg(feature = "data_ec2_client_vpn_endpoint")]
pub use data_ec2_client_vpn_endpoint::*;

#[cfg(feature = "data_ec2_coip_pool")]
pub mod data_ec2_coip_pool;

#[cfg(feature = "data_ec2_coip_pool")]
pub use data_ec2_coip_pool::*;

#[cfg(feature = "data_ec2_coip_pools")]
pub mod data_ec2_coip_pools;

#[cfg(feature = "data_ec2_coip_pools")]
pub use data_ec2_coip_pools::*;

#[cfg(feature = "data_ec2_host")]
pub mod data_ec2_host;

#[cfg(feature = "data_ec2_host")]
pub use data_ec2_host::*;

#[cfg(feature = "data_ec2_instance_type")]
pub mod data_ec2_instance_type;

#[cfg(feature = "data_ec2_instance_type")]
pub use data_ec2_instance_type::*;

#[cfg(feature = "data_ec2_instance_type_offering")]
pub mod data_ec2_instance_type_offering;

#[cfg(feature = "data_ec2_instance_type_offering")]
pub use data_ec2_instance_type_offering::*;

#[cfg(feature = "data_ec2_instance_type_offerings")]
pub mod data_ec2_instance_type_offerings;

#[cfg(feature = "data_ec2_instance_type_offerings")]
pub use data_ec2_instance_type_offerings::*;

#[cfg(feature = "data_ec2_instance_types")]
pub mod data_ec2_instance_types;

#[cfg(feature = "data_ec2_instance_types")]
pub use data_ec2_instance_types::*;

#[cfg(feature = "data_ec2_local_gateway")]
pub mod data_ec2_local_gateway;

#[cfg(feature = "data_ec2_local_gateway")]
pub use data_ec2_local_gateway::*;

#[cfg(feature = "data_ec2_local_gateway_route_table")]
pub mod data_ec2_local_gateway_route_table;

#[cfg(feature = "data_ec2_local_gateway_route_table")]
pub use data_ec2_local_gateway_route_table::*;

#[cfg(feature = "data_ec2_local_gateway_route_tables")]
pub mod data_ec2_local_gateway_route_tables;

#[cfg(feature = "data_ec2_local_gateway_route_tables")]
pub use data_ec2_local_gateway_route_tables::*;

#[cfg(feature = "data_ec2_local_gateway_virtual_interface")]
pub mod data_ec2_local_gateway_virtual_interface;

#[cfg(feature = "data_ec2_local_gateway_virtual_interface")]
pub use data_ec2_local_gateway_virtual_interface::*;

#[cfg(feature = "data_ec2_local_gateway_virtual_interface_group")]
pub mod data_ec2_local_gateway_virtual_interface_group;

#[cfg(feature = "data_ec2_local_gateway_virtual_interface_group")]
pub use data_ec2_local_gateway_virtual_interface_group::*;

#[cfg(feature = "data_ec2_local_gateway_virtual_interface_groups")]
pub mod data_ec2_local_gateway_virtual_interface_groups;

#[cfg(feature = "data_ec2_local_gateway_virtual_interface_groups")]
pub use data_ec2_local_gateway_virtual_interface_groups::*;

#[cfg(feature = "data_ec2_local_gateways")]
pub mod data_ec2_local_gateways;

#[cfg(feature = "data_ec2_local_gateways")]
pub use data_ec2_local_gateways::*;

#[cfg(feature = "data_ec2_managed_prefix_list")]
pub mod data_ec2_managed_prefix_list;

#[cfg(feature = "data_ec2_managed_prefix_list")]
pub use data_ec2_managed_prefix_list::*;

#[cfg(feature = "data_ec2_managed_prefix_lists")]
pub mod data_ec2_managed_prefix_lists;

#[cfg(feature = "data_ec2_managed_prefix_lists")]
pub use data_ec2_managed_prefix_lists::*;

#[cfg(feature = "data_ec2_network_insights_analysis")]
pub mod data_ec2_network_insights_analysis;

#[cfg(feature = "data_ec2_network_insights_analysis")]
pub use data_ec2_network_insights_analysis::*;

#[cfg(feature = "data_ec2_network_insights_path")]
pub mod data_ec2_network_insights_path;

#[cfg(feature = "data_ec2_network_insights_path")]
pub use data_ec2_network_insights_path::*;

#[cfg(feature = "data_ec2_public_ipv4_pool")]
pub mod data_ec2_public_ipv4_pool;

#[cfg(feature = "data_ec2_public_ipv4_pool")]
pub use data_ec2_public_ipv4_pool::*;

#[cfg(feature = "data_ec2_public_ipv4_pools")]
pub mod data_ec2_public_ipv4_pools;

#[cfg(feature = "data_ec2_public_ipv4_pools")]
pub use data_ec2_public_ipv4_pools::*;

#[cfg(feature = "data_ec2_serial_console_access")]
pub mod data_ec2_serial_console_access;

#[cfg(feature = "data_ec2_serial_console_access")]
pub use data_ec2_serial_console_access::*;

#[cfg(feature = "data_ec2_spot_price")]
pub mod data_ec2_spot_price;

#[cfg(feature = "data_ec2_spot_price")]
pub use data_ec2_spot_price::*;

#[cfg(feature = "data_ec2_transit_gateway")]
pub mod data_ec2_transit_gateway;

#[cfg(feature = "data_ec2_transit_gateway")]
pub use data_ec2_transit_gateway::*;

#[cfg(feature = "data_ec2_transit_gateway_attachment")]
pub mod data_ec2_transit_gateway_attachment;

#[cfg(feature = "data_ec2_transit_gateway_attachment")]
pub use data_ec2_transit_gateway_attachment::*;

#[cfg(feature = "data_ec2_transit_gateway_attachments")]
pub mod data_ec2_transit_gateway_attachments;

#[cfg(feature = "data_ec2_transit_gateway_attachments")]
pub use data_ec2_transit_gateway_attachments::*;

#[cfg(feature = "data_ec2_transit_gateway_connect")]
pub mod data_ec2_transit_gateway_connect;

#[cfg(feature = "data_ec2_transit_gateway_connect")]
pub use data_ec2_transit_gateway_connect::*;

#[cfg(feature = "data_ec2_transit_gateway_connect_peer")]
pub mod data_ec2_transit_gateway_connect_peer;

#[cfg(feature = "data_ec2_transit_gateway_connect_peer")]
pub use data_ec2_transit_gateway_connect_peer::*;

#[cfg(feature = "data_ec2_transit_gateway_dx_gateway_attachment")]
pub mod data_ec2_transit_gateway_dx_gateway_attachment;

#[cfg(feature = "data_ec2_transit_gateway_dx_gateway_attachment")]
pub use data_ec2_transit_gateway_dx_gateway_attachment::*;

#[cfg(feature = "data_ec2_transit_gateway_multicast_domain")]
pub mod data_ec2_transit_gateway_multicast_domain;

#[cfg(feature = "data_ec2_transit_gateway_multicast_domain")]
pub use data_ec2_transit_gateway_multicast_domain::*;

#[cfg(feature = "data_ec2_transit_gateway_peering_attachment")]
pub mod data_ec2_transit_gateway_peering_attachment;

#[cfg(feature = "data_ec2_transit_gateway_peering_attachment")]
pub use data_ec2_transit_gateway_peering_attachment::*;

#[cfg(feature = "data_ec2_transit_gateway_peering_attachments")]
pub mod data_ec2_transit_gateway_peering_attachments;

#[cfg(feature = "data_ec2_transit_gateway_peering_attachments")]
pub use data_ec2_transit_gateway_peering_attachments::*;

#[cfg(feature = "data_ec2_transit_gateway_route_table")]
pub mod data_ec2_transit_gateway_route_table;

#[cfg(feature = "data_ec2_transit_gateway_route_table")]
pub use data_ec2_transit_gateway_route_table::*;

#[cfg(feature = "data_ec2_transit_gateway_route_table_associations")]
pub mod data_ec2_transit_gateway_route_table_associations;

#[cfg(feature = "data_ec2_transit_gateway_route_table_associations")]
pub use data_ec2_transit_gateway_route_table_associations::*;

#[cfg(feature = "data_ec2_transit_gateway_route_table_propagations")]
pub mod data_ec2_transit_gateway_route_table_propagations;

#[cfg(feature = "data_ec2_transit_gateway_route_table_propagations")]
pub use data_ec2_transit_gateway_route_table_propagations::*;

#[cfg(feature = "data_ec2_transit_gateway_route_table_routes")]
pub mod data_ec2_transit_gateway_route_table_routes;

#[cfg(feature = "data_ec2_transit_gateway_route_table_routes")]
pub use data_ec2_transit_gateway_route_table_routes::*;

#[cfg(feature = "data_ec2_transit_gateway_route_tables")]
pub mod data_ec2_transit_gateway_route_tables;

#[cfg(feature = "data_ec2_transit_gateway_route_tables")]
pub use data_ec2_transit_gateway_route_tables::*;

#[cfg(feature = "data_ec2_transit_gateway_vpc_attachment")]
pub mod data_ec2_transit_gateway_vpc_attachment;

#[cfg(feature = "data_ec2_transit_gateway_vpc_attachment")]
pub use data_ec2_transit_gateway_vpc_attachment::*;

#[cfg(feature = "data_ec2_transit_gateway_vpc_attachments")]
pub mod data_ec2_transit_gateway_vpc_attachments;

#[cfg(feature = "data_ec2_transit_gateway_vpc_attachments")]
pub use data_ec2_transit_gateway_vpc_attachments::*;

#[cfg(feature = "data_ec2_transit_gateway_vpn_attachment")]
pub mod data_ec2_transit_gateway_vpn_attachment;

#[cfg(feature = "data_ec2_transit_gateway_vpn_attachment")]
pub use data_ec2_transit_gateway_vpn_attachment::*;

#[cfg(feature = "data_ecr_authorization_token")]
pub mod data_ecr_authorization_token;

#[cfg(feature = "data_ecr_authorization_token")]
pub use data_ecr_authorization_token::*;

#[cfg(feature = "data_ecr_image")]
pub mod data_ecr_image;

#[cfg(feature = "data_ecr_image")]
pub use data_ecr_image::*;

#[cfg(feature = "data_ecr_images")]
pub mod data_ecr_images;

#[cfg(feature = "data_ecr_images")]
pub use data_ecr_images::*;

#[cfg(feature = "data_ecr_lifecycle_policy_document")]
pub mod data_ecr_lifecycle_policy_document;

#[cfg(feature = "data_ecr_lifecycle_policy_document")]
pub use data_ecr_lifecycle_policy_document::*;

#[cfg(feature = "data_ecr_pull_through_cache_rule")]
pub mod data_ecr_pull_through_cache_rule;

#[cfg(feature = "data_ecr_pull_through_cache_rule")]
pub use data_ecr_pull_through_cache_rule::*;

#[cfg(feature = "data_ecr_repositories")]
pub mod data_ecr_repositories;

#[cfg(feature = "data_ecr_repositories")]
pub use data_ecr_repositories::*;

#[cfg(feature = "data_ecr_repository")]
pub mod data_ecr_repository;

#[cfg(feature = "data_ecr_repository")]
pub use data_ecr_repository::*;

#[cfg(feature = "data_ecr_repository_creation_template")]
pub mod data_ecr_repository_creation_template;

#[cfg(feature = "data_ecr_repository_creation_template")]
pub use data_ecr_repository_creation_template::*;

#[cfg(feature = "data_ecrpublic_authorization_token")]
pub mod data_ecrpublic_authorization_token;

#[cfg(feature = "data_ecrpublic_authorization_token")]
pub use data_ecrpublic_authorization_token::*;

#[cfg(feature = "data_ecrpublic_images")]
pub mod data_ecrpublic_images;

#[cfg(feature = "data_ecrpublic_images")]
pub use data_ecrpublic_images::*;

#[cfg(feature = "data_ecs_cluster")]
pub mod data_ecs_cluster;

#[cfg(feature = "data_ecs_cluster")]
pub use data_ecs_cluster::*;

#[cfg(feature = "data_ecs_clusters")]
pub mod data_ecs_clusters;

#[cfg(feature = "data_ecs_clusters")]
pub use data_ecs_clusters::*;

#[cfg(feature = "data_ecs_container_definition")]
pub mod data_ecs_container_definition;

#[cfg(feature = "data_ecs_container_definition")]
pub use data_ecs_container_definition::*;

#[cfg(feature = "data_ecs_service")]
pub mod data_ecs_service;

#[cfg(feature = "data_ecs_service")]
pub use data_ecs_service::*;

#[cfg(feature = "data_ecs_task_definition")]
pub mod data_ecs_task_definition;

#[cfg(feature = "data_ecs_task_definition")]
pub use data_ecs_task_definition::*;

#[cfg(feature = "data_ecs_task_execution")]
pub mod data_ecs_task_execution;

#[cfg(feature = "data_ecs_task_execution")]
pub use data_ecs_task_execution::*;

#[cfg(feature = "data_efs_access_point")]
pub mod data_efs_access_point;

#[cfg(feature = "data_efs_access_point")]
pub use data_efs_access_point::*;

#[cfg(feature = "data_efs_access_points")]
pub mod data_efs_access_points;

#[cfg(feature = "data_efs_access_points")]
pub use data_efs_access_points::*;

#[cfg(feature = "data_efs_file_system")]
pub mod data_efs_file_system;

#[cfg(feature = "data_efs_file_system")]
pub use data_efs_file_system::*;

#[cfg(feature = "data_efs_mount_target")]
pub mod data_efs_mount_target;

#[cfg(feature = "data_efs_mount_target")]
pub use data_efs_mount_target::*;

#[cfg(feature = "data_eip")]
pub mod data_eip;

#[cfg(feature = "data_eip")]
pub use data_eip::*;

#[cfg(feature = "data_eips")]
pub mod data_eips;

#[cfg(feature = "data_eips")]
pub use data_eips::*;

#[cfg(feature = "data_eks_access_entry")]
pub mod data_eks_access_entry;

#[cfg(feature = "data_eks_access_entry")]
pub use data_eks_access_entry::*;

#[cfg(feature = "data_eks_addon")]
pub mod data_eks_addon;

#[cfg(feature = "data_eks_addon")]
pub use data_eks_addon::*;

#[cfg(feature = "data_eks_addon_version")]
pub mod data_eks_addon_version;

#[cfg(feature = "data_eks_addon_version")]
pub use data_eks_addon_version::*;

#[cfg(feature = "data_eks_cluster")]
pub mod data_eks_cluster;

#[cfg(feature = "data_eks_cluster")]
pub use data_eks_cluster::*;

#[cfg(feature = "data_eks_cluster_auth")]
pub mod data_eks_cluster_auth;

#[cfg(feature = "data_eks_cluster_auth")]
pub use data_eks_cluster_auth::*;

#[cfg(feature = "data_eks_cluster_versions")]
pub mod data_eks_cluster_versions;

#[cfg(feature = "data_eks_cluster_versions")]
pub use data_eks_cluster_versions::*;

#[cfg(feature = "data_eks_clusters")]
pub mod data_eks_clusters;

#[cfg(feature = "data_eks_clusters")]
pub use data_eks_clusters::*;

#[cfg(feature = "data_eks_node_group")]
pub mod data_eks_node_group;

#[cfg(feature = "data_eks_node_group")]
pub use data_eks_node_group::*;

#[cfg(feature = "data_eks_node_groups")]
pub mod data_eks_node_groups;

#[cfg(feature = "data_eks_node_groups")]
pub use data_eks_node_groups::*;

#[cfg(feature = "data_elastic_beanstalk_application")]
pub mod data_elastic_beanstalk_application;

#[cfg(feature = "data_elastic_beanstalk_application")]
pub use data_elastic_beanstalk_application::*;

#[cfg(feature = "data_elastic_beanstalk_hosted_zone")]
pub mod data_elastic_beanstalk_hosted_zone;

#[cfg(feature = "data_elastic_beanstalk_hosted_zone")]
pub use data_elastic_beanstalk_hosted_zone::*;

#[cfg(feature = "data_elastic_beanstalk_solution_stack")]
pub mod data_elastic_beanstalk_solution_stack;

#[cfg(feature = "data_elastic_beanstalk_solution_stack")]
pub use data_elastic_beanstalk_solution_stack::*;

#[cfg(feature = "data_elasticache_cluster")]
pub mod data_elasticache_cluster;

#[cfg(feature = "data_elasticache_cluster")]
pub use data_elasticache_cluster::*;

#[cfg(feature = "data_elasticache_replication_group")]
pub mod data_elasticache_replication_group;

#[cfg(feature = "data_elasticache_replication_group")]
pub use data_elasticache_replication_group::*;

#[cfg(feature = "data_elasticache_reserved_cache_node_offering")]
pub mod data_elasticache_reserved_cache_node_offering;

#[cfg(feature = "data_elasticache_reserved_cache_node_offering")]
pub use data_elasticache_reserved_cache_node_offering::*;

#[cfg(feature = "data_elasticache_serverless_cache")]
pub mod data_elasticache_serverless_cache;

#[cfg(feature = "data_elasticache_serverless_cache")]
pub use data_elasticache_serverless_cache::*;

#[cfg(feature = "data_elasticache_subnet_group")]
pub mod data_elasticache_subnet_group;

#[cfg(feature = "data_elasticache_subnet_group")]
pub use data_elasticache_subnet_group::*;

#[cfg(feature = "data_elasticache_user")]
pub mod data_elasticache_user;

#[cfg(feature = "data_elasticache_user")]
pub use data_elasticache_user::*;

#[cfg(feature = "data_elasticsearch_domain")]
pub mod data_elasticsearch_domain;

#[cfg(feature = "data_elasticsearch_domain")]
pub use data_elasticsearch_domain::*;

#[cfg(feature = "data_elb")]
pub mod data_elb;

#[cfg(feature = "data_elb")]
pub use data_elb::*;

#[cfg(feature = "data_elb_hosted_zone_id")]
pub mod data_elb_hosted_zone_id;

#[cfg(feature = "data_elb_hosted_zone_id")]
pub use data_elb_hosted_zone_id::*;

#[cfg(feature = "data_elb_service_account")]
pub mod data_elb_service_account;

#[cfg(feature = "data_elb_service_account")]
pub use data_elb_service_account::*;

#[cfg(feature = "data_emr_release_labels")]
pub mod data_emr_release_labels;

#[cfg(feature = "data_emr_release_labels")]
pub use data_emr_release_labels::*;

#[cfg(feature = "data_emr_supported_instance_types")]
pub mod data_emr_supported_instance_types;

#[cfg(feature = "data_emr_supported_instance_types")]
pub use data_emr_supported_instance_types::*;

#[cfg(feature = "data_emrcontainers_virtual_cluster")]
pub mod data_emrcontainers_virtual_cluster;

#[cfg(feature = "data_emrcontainers_virtual_cluster")]
pub use data_emrcontainers_virtual_cluster::*;

#[cfg(feature = "data_fis_experiment_templates")]
pub mod data_fis_experiment_templates;

#[cfg(feature = "data_fis_experiment_templates")]
pub use data_fis_experiment_templates::*;

#[cfg(feature = "data_fsx_ontap_file_system")]
pub mod data_fsx_ontap_file_system;

#[cfg(feature = "data_fsx_ontap_file_system")]
pub use data_fsx_ontap_file_system::*;

#[cfg(feature = "data_fsx_ontap_storage_virtual_machine")]
pub mod data_fsx_ontap_storage_virtual_machine;

#[cfg(feature = "data_fsx_ontap_storage_virtual_machine")]
pub use data_fsx_ontap_storage_virtual_machine::*;

#[cfg(feature = "data_fsx_ontap_storage_virtual_machines")]
pub mod data_fsx_ontap_storage_virtual_machines;

#[cfg(feature = "data_fsx_ontap_storage_virtual_machines")]
pub use data_fsx_ontap_storage_virtual_machines::*;

#[cfg(feature = "data_fsx_openzfs_snapshot")]
pub mod data_fsx_openzfs_snapshot;

#[cfg(feature = "data_fsx_openzfs_snapshot")]
pub use data_fsx_openzfs_snapshot::*;

#[cfg(feature = "data_fsx_windows_file_system")]
pub mod data_fsx_windows_file_system;

#[cfg(feature = "data_fsx_windows_file_system")]
pub use data_fsx_windows_file_system::*;

#[cfg(feature = "data_globalaccelerator_accelerator")]
pub mod data_globalaccelerator_accelerator;

#[cfg(feature = "data_globalaccelerator_accelerator")]
pub use data_globalaccelerator_accelerator::*;

#[cfg(feature = "data_globalaccelerator_custom_routing_accelerator")]
pub mod data_globalaccelerator_custom_routing_accelerator;

#[cfg(feature = "data_globalaccelerator_custom_routing_accelerator")]
pub use data_globalaccelerator_custom_routing_accelerator::*;

#[cfg(feature = "data_glue_catalog_table")]
pub mod data_glue_catalog_table;

#[cfg(feature = "data_glue_catalog_table")]
pub use data_glue_catalog_table::*;

#[cfg(feature = "data_glue_connection")]
pub mod data_glue_connection;

#[cfg(feature = "data_glue_connection")]
pub use data_glue_connection::*;

#[cfg(feature = "data_glue_data_catalog_encryption_settings")]
pub mod data_glue_data_catalog_encryption_settings;

#[cfg(feature = "data_glue_data_catalog_encryption_settings")]
pub use data_glue_data_catalog_encryption_settings::*;

#[cfg(feature = "data_glue_registry")]
pub mod data_glue_registry;

#[cfg(feature = "data_glue_registry")]
pub use data_glue_registry::*;

#[cfg(feature = "data_glue_script")]
pub mod data_glue_script;

#[cfg(feature = "data_glue_script")]
pub use data_glue_script::*;

#[cfg(feature = "data_grafana_workspace")]
pub mod data_grafana_workspace;

#[cfg(feature = "data_grafana_workspace")]
pub use data_grafana_workspace::*;

#[cfg(feature = "data_guardduty_detector")]
pub mod data_guardduty_detector;

#[cfg(feature = "data_guardduty_detector")]
pub use data_guardduty_detector::*;

#[cfg(feature = "data_guardduty_finding_ids")]
pub mod data_guardduty_finding_ids;

#[cfg(feature = "data_guardduty_finding_ids")]
pub use data_guardduty_finding_ids::*;

#[cfg(feature = "data_iam_access_keys")]
pub mod data_iam_access_keys;

#[cfg(feature = "data_iam_access_keys")]
pub use data_iam_access_keys::*;

#[cfg(feature = "data_iam_account_alias")]
pub mod data_iam_account_alias;

#[cfg(feature = "data_iam_account_alias")]
pub use data_iam_account_alias::*;

#[cfg(feature = "data_iam_group")]
pub mod data_iam_group;

#[cfg(feature = "data_iam_group")]
pub use data_iam_group::*;

#[cfg(feature = "data_iam_instance_profile")]
pub mod data_iam_instance_profile;

#[cfg(feature = "data_iam_instance_profile")]
pub use data_iam_instance_profile::*;

#[cfg(feature = "data_iam_instance_profiles")]
pub mod data_iam_instance_profiles;

#[cfg(feature = "data_iam_instance_profiles")]
pub use data_iam_instance_profiles::*;

#[cfg(feature = "data_iam_openid_connect_provider")]
pub mod data_iam_openid_connect_provider;

#[cfg(feature = "data_iam_openid_connect_provider")]
pub use data_iam_openid_connect_provider::*;

#[cfg(feature = "data_iam_policy")]
pub mod data_iam_policy;

#[cfg(feature = "data_iam_policy")]
pub use data_iam_policy::*;

#[cfg(feature = "data_iam_policy_document")]
pub mod data_iam_policy_document;

#[cfg(feature = "data_iam_policy_document")]
pub use data_iam_policy_document::*;

#[cfg(feature = "data_iam_principal_policy_simulation")]
pub mod data_iam_principal_policy_simulation;

#[cfg(feature = "data_iam_principal_policy_simulation")]
pub use data_iam_principal_policy_simulation::*;

#[cfg(feature = "data_iam_role")]
pub mod data_iam_role;

#[cfg(feature = "data_iam_role")]
pub use data_iam_role::*;

#[cfg(feature = "data_iam_roles")]
pub mod data_iam_roles;

#[cfg(feature = "data_iam_roles")]
pub use data_iam_roles::*;

#[cfg(feature = "data_iam_saml_provider")]
pub mod data_iam_saml_provider;

#[cfg(feature = "data_iam_saml_provider")]
pub use data_iam_saml_provider::*;

#[cfg(feature = "data_iam_server_certificate")]
pub mod data_iam_server_certificate;

#[cfg(feature = "data_iam_server_certificate")]
pub use data_iam_server_certificate::*;

#[cfg(feature = "data_iam_session_context")]
pub mod data_iam_session_context;

#[cfg(feature = "data_iam_session_context")]
pub use data_iam_session_context::*;

#[cfg(feature = "data_iam_user")]
pub mod data_iam_user;

#[cfg(feature = "data_iam_user")]
pub use data_iam_user::*;

#[cfg(feature = "data_iam_user_ssh_key")]
pub mod data_iam_user_ssh_key;

#[cfg(feature = "data_iam_user_ssh_key")]
pub use data_iam_user_ssh_key::*;

#[cfg(feature = "data_iam_users")]
pub mod data_iam_users;

#[cfg(feature = "data_iam_users")]
pub use data_iam_users::*;

#[cfg(feature = "data_identitystore_group")]
pub mod data_identitystore_group;

#[cfg(feature = "data_identitystore_group")]
pub use data_identitystore_group::*;

#[cfg(feature = "data_identitystore_group_memberships")]
pub mod data_identitystore_group_memberships;

#[cfg(feature = "data_identitystore_group_memberships")]
pub use data_identitystore_group_memberships::*;

#[cfg(feature = "data_identitystore_groups")]
pub mod data_identitystore_groups;

#[cfg(feature = "data_identitystore_groups")]
pub use data_identitystore_groups::*;

#[cfg(feature = "data_identitystore_user")]
pub mod data_identitystore_user;

#[cfg(feature = "data_identitystore_user")]
pub use data_identitystore_user::*;

#[cfg(feature = "data_identitystore_users")]
pub mod data_identitystore_users;

#[cfg(feature = "data_identitystore_users")]
pub use data_identitystore_users::*;

#[cfg(feature = "data_imagebuilder_component")]
pub mod data_imagebuilder_component;

#[cfg(feature = "data_imagebuilder_component")]
pub use data_imagebuilder_component::*;

#[cfg(feature = "data_imagebuilder_components")]
pub mod data_imagebuilder_components;

#[cfg(feature = "data_imagebuilder_components")]
pub use data_imagebuilder_components::*;

#[cfg(feature = "data_imagebuilder_container_recipe")]
pub mod data_imagebuilder_container_recipe;

#[cfg(feature = "data_imagebuilder_container_recipe")]
pub use data_imagebuilder_container_recipe::*;

#[cfg(feature = "data_imagebuilder_container_recipes")]
pub mod data_imagebuilder_container_recipes;

#[cfg(feature = "data_imagebuilder_container_recipes")]
pub use data_imagebuilder_container_recipes::*;

#[cfg(feature = "data_imagebuilder_distribution_configuration")]
pub mod data_imagebuilder_distribution_configuration;

#[cfg(feature = "data_imagebuilder_distribution_configuration")]
pub use data_imagebuilder_distribution_configuration::*;

#[cfg(feature = "data_imagebuilder_distribution_configurations")]
pub mod data_imagebuilder_distribution_configurations;

#[cfg(feature = "data_imagebuilder_distribution_configurations")]
pub use data_imagebuilder_distribution_configurations::*;

#[cfg(feature = "data_imagebuilder_image")]
pub mod data_imagebuilder_image;

#[cfg(feature = "data_imagebuilder_image")]
pub use data_imagebuilder_image::*;

#[cfg(feature = "data_imagebuilder_image_pipeline")]
pub mod data_imagebuilder_image_pipeline;

#[cfg(feature = "data_imagebuilder_image_pipeline")]
pub use data_imagebuilder_image_pipeline::*;

#[cfg(feature = "data_imagebuilder_image_pipelines")]
pub mod data_imagebuilder_image_pipelines;

#[cfg(feature = "data_imagebuilder_image_pipelines")]
pub use data_imagebuilder_image_pipelines::*;

#[cfg(feature = "data_imagebuilder_image_recipe")]
pub mod data_imagebuilder_image_recipe;

#[cfg(feature = "data_imagebuilder_image_recipe")]
pub use data_imagebuilder_image_recipe::*;

#[cfg(feature = "data_imagebuilder_image_recipes")]
pub mod data_imagebuilder_image_recipes;

#[cfg(feature = "data_imagebuilder_image_recipes")]
pub use data_imagebuilder_image_recipes::*;

#[cfg(feature = "data_imagebuilder_infrastructure_configuration")]
pub mod data_imagebuilder_infrastructure_configuration;

#[cfg(feature = "data_imagebuilder_infrastructure_configuration")]
pub use data_imagebuilder_infrastructure_configuration::*;

#[cfg(feature = "data_imagebuilder_infrastructure_configurations")]
pub mod data_imagebuilder_infrastructure_configurations;

#[cfg(feature = "data_imagebuilder_infrastructure_configurations")]
pub use data_imagebuilder_infrastructure_configurations::*;

#[cfg(feature = "data_inspector_rules_packages")]
pub mod data_inspector_rules_packages;

#[cfg(feature = "data_inspector_rules_packages")]
pub use data_inspector_rules_packages::*;

#[cfg(feature = "data_instance")]
pub mod data_instance;

#[cfg(feature = "data_instance")]
pub use data_instance::*;

#[cfg(feature = "data_instances")]
pub mod data_instances;

#[cfg(feature = "data_instances")]
pub use data_instances::*;

#[cfg(feature = "data_internet_gateway")]
pub mod data_internet_gateway;

#[cfg(feature = "data_internet_gateway")]
pub use data_internet_gateway::*;

#[cfg(feature = "data_iot_endpoint")]
pub mod data_iot_endpoint;

#[cfg(feature = "data_iot_endpoint")]
pub use data_iot_endpoint::*;

#[cfg(feature = "data_iot_registration_code")]
pub mod data_iot_registration_code;

#[cfg(feature = "data_iot_registration_code")]
pub use data_iot_registration_code::*;

#[cfg(feature = "data_ip_ranges")]
pub mod data_ip_ranges;

#[cfg(feature = "data_ip_ranges")]
pub use data_ip_ranges::*;

#[cfg(feature = "data_ivs_stream_key")]
pub mod data_ivs_stream_key;

#[cfg(feature = "data_ivs_stream_key")]
pub use data_ivs_stream_key::*;

#[cfg(feature = "data_kendra_experience")]
pub mod data_kendra_experience;

#[cfg(feature = "data_kendra_experience")]
pub use data_kendra_experience::*;

#[cfg(feature = "data_kendra_faq")]
pub mod data_kendra_faq;

#[cfg(feature = "data_kendra_faq")]
pub use data_kendra_faq::*;

#[cfg(feature = "data_kendra_index")]
pub mod data_kendra_index;

#[cfg(feature = "data_kendra_index")]
pub use data_kendra_index::*;

#[cfg(feature = "data_kendra_query_suggestions_block_list")]
pub mod data_kendra_query_suggestions_block_list;

#[cfg(feature = "data_kendra_query_suggestions_block_list")]
pub use data_kendra_query_suggestions_block_list::*;

#[cfg(feature = "data_kendra_thesaurus")]
pub mod data_kendra_thesaurus;

#[cfg(feature = "data_kendra_thesaurus")]
pub use data_kendra_thesaurus::*;

#[cfg(feature = "data_key_pair")]
pub mod data_key_pair;

#[cfg(feature = "data_key_pair")]
pub use data_key_pair::*;

#[cfg(feature = "data_kms_alias")]
pub mod data_kms_alias;

#[cfg(feature = "data_kms_alias")]
pub use data_kms_alias::*;

#[cfg(feature = "data_kms_ciphertext")]
pub mod data_kms_ciphertext;

#[cfg(feature = "data_kms_ciphertext")]
pub use data_kms_ciphertext::*;

#[cfg(feature = "data_kms_custom_key_store")]
pub mod data_kms_custom_key_store;

#[cfg(feature = "data_kms_custom_key_store")]
pub use data_kms_custom_key_store::*;

#[cfg(feature = "data_kms_key")]
pub mod data_kms_key;

#[cfg(feature = "data_kms_key")]
pub use data_kms_key::*;

#[cfg(feature = "data_kms_public_key")]
pub mod data_kms_public_key;

#[cfg(feature = "data_kms_public_key")]
pub use data_kms_public_key::*;

#[cfg(feature = "data_kms_secret")]
pub mod data_kms_secret;

#[cfg(feature = "data_kms_secret")]
pub use data_kms_secret::*;

#[cfg(feature = "data_kms_secrets")]
pub mod data_kms_secrets;

#[cfg(feature = "data_kms_secrets")]
pub use data_kms_secrets::*;

#[cfg(feature = "data_lakeformation_data_lake_settings")]
pub mod data_lakeformation_data_lake_settings;

#[cfg(feature = "data_lakeformation_data_lake_settings")]
pub use data_lakeformation_data_lake_settings::*;

#[cfg(feature = "data_lakeformation_permissions")]
pub mod data_lakeformation_permissions;

#[cfg(feature = "data_lakeformation_permissions")]
pub use data_lakeformation_permissions::*;

#[cfg(feature = "data_lakeformation_resource")]
pub mod data_lakeformation_resource;

#[cfg(feature = "data_lakeformation_resource")]
pub use data_lakeformation_resource::*;

#[cfg(feature = "data_lambda_alias")]
pub mod data_lambda_alias;

#[cfg(feature = "data_lambda_alias")]
pub use data_lambda_alias::*;

#[cfg(feature = "data_lambda_code_signing_config")]
pub mod data_lambda_code_signing_config;

#[cfg(feature = "data_lambda_code_signing_config")]
pub use data_lambda_code_signing_config::*;

#[cfg(feature = "data_lambda_function")]
pub mod data_lambda_function;

#[cfg(feature = "data_lambda_function")]
pub use data_lambda_function::*;

#[cfg(feature = "data_lambda_function_url")]
pub mod data_lambda_function_url;

#[cfg(feature = "data_lambda_function_url")]
pub use data_lambda_function_url::*;

#[cfg(feature = "data_lambda_functions")]
pub mod data_lambda_functions;

#[cfg(feature = "data_lambda_functions")]
pub use data_lambda_functions::*;

#[cfg(feature = "data_lambda_invocation")]
pub mod data_lambda_invocation;

#[cfg(feature = "data_lambda_invocation")]
pub use data_lambda_invocation::*;

#[cfg(feature = "data_lambda_layer_version")]
pub mod data_lambda_layer_version;

#[cfg(feature = "data_lambda_layer_version")]
pub use data_lambda_layer_version::*;

#[cfg(feature = "data_launch_configuration")]
pub mod data_launch_configuration;

#[cfg(feature = "data_launch_configuration")]
pub use data_launch_configuration::*;

#[cfg(feature = "data_launch_template")]
pub mod data_launch_template;

#[cfg(feature = "data_launch_template")]
pub use data_launch_template::*;

#[cfg(feature = "data_lb")]
pub mod data_lb;

#[cfg(feature = "data_lb")]
pub use data_lb::*;

#[cfg(feature = "data_lb_hosted_zone_id")]
pub mod data_lb_hosted_zone_id;

#[cfg(feature = "data_lb_hosted_zone_id")]
pub use data_lb_hosted_zone_id::*;

#[cfg(feature = "data_lb_listener")]
pub mod data_lb_listener;

#[cfg(feature = "data_lb_listener")]
pub use data_lb_listener::*;

#[cfg(feature = "data_lb_listener_rule")]
pub mod data_lb_listener_rule;

#[cfg(feature = "data_lb_listener_rule")]
pub use data_lb_listener_rule::*;

#[cfg(feature = "data_lb_target_group")]
pub mod data_lb_target_group;

#[cfg(feature = "data_lb_target_group")]
pub use data_lb_target_group::*;

#[cfg(feature = "data_lb_trust_store")]
pub mod data_lb_trust_store;

#[cfg(feature = "data_lb_trust_store")]
pub use data_lb_trust_store::*;

#[cfg(feature = "data_lbs")]
pub mod data_lbs;

#[cfg(feature = "data_lbs")]
pub use data_lbs::*;

#[cfg(feature = "data_lex_bot")]
pub mod data_lex_bot;

#[cfg(feature = "data_lex_bot")]
pub use data_lex_bot::*;

#[cfg(feature = "data_lex_bot_alias")]
pub mod data_lex_bot_alias;

#[cfg(feature = "data_lex_bot_alias")]
pub use data_lex_bot_alias::*;

#[cfg(feature = "data_lex_intent")]
pub mod data_lex_intent;

#[cfg(feature = "data_lex_intent")]
pub use data_lex_intent::*;

#[cfg(feature = "data_lex_slot_type")]
pub mod data_lex_slot_type;

#[cfg(feature = "data_lex_slot_type")]
pub use data_lex_slot_type::*;

#[cfg(feature = "data_licensemanager_grants")]
pub mod data_licensemanager_grants;

#[cfg(feature = "data_licensemanager_grants")]
pub use data_licensemanager_grants::*;

#[cfg(feature = "data_licensemanager_received_license")]
pub mod data_licensemanager_received_license;

#[cfg(feature = "data_licensemanager_received_license")]
pub use data_licensemanager_received_license::*;

#[cfg(feature = "data_licensemanager_received_licenses")]
pub mod data_licensemanager_received_licenses;

#[cfg(feature = "data_licensemanager_received_licenses")]
pub use data_licensemanager_received_licenses::*;

#[cfg(feature = "data_location_geofence_collection")]
pub mod data_location_geofence_collection;

#[cfg(feature = "data_location_geofence_collection")]
pub use data_location_geofence_collection::*;

#[cfg(feature = "data_location_map")]
pub mod data_location_map;

#[cfg(feature = "data_location_map")]
pub use data_location_map::*;

#[cfg(feature = "data_location_place_index")]
pub mod data_location_place_index;

#[cfg(feature = "data_location_place_index")]
pub use data_location_place_index::*;

#[cfg(feature = "data_location_route_calculator")]
pub mod data_location_route_calculator;

#[cfg(feature = "data_location_route_calculator")]
pub use data_location_route_calculator::*;

#[cfg(feature = "data_location_tracker")]
pub mod data_location_tracker;

#[cfg(feature = "data_location_tracker")]
pub use data_location_tracker::*;

#[cfg(feature = "data_location_tracker_association")]
pub mod data_location_tracker_association;

#[cfg(feature = "data_location_tracker_association")]
pub use data_location_tracker_association::*;

#[cfg(feature = "data_location_tracker_associations")]
pub mod data_location_tracker_associations;

#[cfg(feature = "data_location_tracker_associations")]
pub use data_location_tracker_associations::*;

#[cfg(feature = "data_media_convert_queue")]
pub mod data_media_convert_queue;

#[cfg(feature = "data_media_convert_queue")]
pub use data_media_convert_queue::*;

#[cfg(feature = "data_memorydb_acl")]
pub mod data_memorydb_acl;

#[cfg(feature = "data_memorydb_acl")]
pub use data_memorydb_acl::*;

#[cfg(feature = "data_memorydb_cluster")]
pub mod data_memorydb_cluster;

#[cfg(feature = "data_memorydb_cluster")]
pub use data_memorydb_cluster::*;

#[cfg(feature = "data_memorydb_parameter_group")]
pub mod data_memorydb_parameter_group;

#[cfg(feature = "data_memorydb_parameter_group")]
pub use data_memorydb_parameter_group::*;

#[cfg(feature = "data_memorydb_snapshot")]
pub mod data_memorydb_snapshot;

#[cfg(feature = "data_memorydb_snapshot")]
pub use data_memorydb_snapshot::*;

#[cfg(feature = "data_memorydb_subnet_group")]
pub mod data_memorydb_subnet_group;

#[cfg(feature = "data_memorydb_subnet_group")]
pub use data_memorydb_subnet_group::*;

#[cfg(feature = "data_memorydb_user")]
pub mod data_memorydb_user;

#[cfg(feature = "data_memorydb_user")]
pub use data_memorydb_user::*;

#[cfg(feature = "data_mq_broker")]
pub mod data_mq_broker;

#[cfg(feature = "data_mq_broker")]
pub use data_mq_broker::*;

#[cfg(feature = "data_mq_broker_engine_types")]
pub mod data_mq_broker_engine_types;

#[cfg(feature = "data_mq_broker_engine_types")]
pub use data_mq_broker_engine_types::*;

#[cfg(feature = "data_mq_broker_instance_type_offerings")]
pub mod data_mq_broker_instance_type_offerings;

#[cfg(feature = "data_mq_broker_instance_type_offerings")]
pub use data_mq_broker_instance_type_offerings::*;

#[cfg(feature = "data_msk_bootstrap_brokers")]
pub mod data_msk_bootstrap_brokers;

#[cfg(feature = "data_msk_bootstrap_brokers")]
pub use data_msk_bootstrap_brokers::*;

#[cfg(feature = "data_msk_broker_nodes")]
pub mod data_msk_broker_nodes;

#[cfg(feature = "data_msk_broker_nodes")]
pub use data_msk_broker_nodes::*;

#[cfg(feature = "data_msk_cluster")]
pub mod data_msk_cluster;

#[cfg(feature = "data_msk_cluster")]
pub use data_msk_cluster::*;

#[cfg(feature = "data_msk_configuration")]
pub mod data_msk_configuration;

#[cfg(feature = "data_msk_configuration")]
pub use data_msk_configuration::*;

#[cfg(feature = "data_msk_kafka_version")]
pub mod data_msk_kafka_version;

#[cfg(feature = "data_msk_kafka_version")]
pub use data_msk_kafka_version::*;

#[cfg(feature = "data_msk_vpc_connection")]
pub mod data_msk_vpc_connection;

#[cfg(feature = "data_msk_vpc_connection")]
pub use data_msk_vpc_connection::*;

#[cfg(feature = "data_mskconnect_connector")]
pub mod data_mskconnect_connector;

#[cfg(feature = "data_mskconnect_connector")]
pub use data_mskconnect_connector::*;

#[cfg(feature = "data_mskconnect_custom_plugin")]
pub mod data_mskconnect_custom_plugin;

#[cfg(feature = "data_mskconnect_custom_plugin")]
pub use data_mskconnect_custom_plugin::*;

#[cfg(feature = "data_mskconnect_worker_configuration")]
pub mod data_mskconnect_worker_configuration;

#[cfg(feature = "data_mskconnect_worker_configuration")]
pub use data_mskconnect_worker_configuration::*;

#[cfg(feature = "data_nat_gateway")]
pub mod data_nat_gateway;

#[cfg(feature = "data_nat_gateway")]
pub use data_nat_gateway::*;

#[cfg(feature = "data_nat_gateways")]
pub mod data_nat_gateways;

#[cfg(feature = "data_nat_gateways")]
pub use data_nat_gateways::*;

#[cfg(feature = "data_neptune_engine_version")]
pub mod data_neptune_engine_version;

#[cfg(feature = "data_neptune_engine_version")]
pub use data_neptune_engine_version::*;

#[cfg(feature = "data_neptune_orderable_db_instance")]
pub mod data_neptune_orderable_db_instance;

#[cfg(feature = "data_neptune_orderable_db_instance")]
pub use data_neptune_orderable_db_instance::*;

#[cfg(feature = "data_network_acls")]
pub mod data_network_acls;

#[cfg(feature = "data_network_acls")]
pub use data_network_acls::*;

#[cfg(feature = "data_network_interface")]
pub mod data_network_interface;

#[cfg(feature = "data_network_interface")]
pub use data_network_interface::*;

#[cfg(feature = "data_network_interfaces")]
pub mod data_network_interfaces;

#[cfg(feature = "data_network_interfaces")]
pub use data_network_interfaces::*;

#[cfg(feature = "data_networkfirewall_firewall")]
pub mod data_networkfirewall_firewall;

#[cfg(feature = "data_networkfirewall_firewall")]
pub use data_networkfirewall_firewall::*;

#[cfg(feature = "data_networkfirewall_firewall_policy")]
pub mod data_networkfirewall_firewall_policy;

#[cfg(feature = "data_networkfirewall_firewall_policy")]
pub use data_networkfirewall_firewall_policy::*;

#[cfg(feature = "data_networkfirewall_resource_policy")]
pub mod data_networkfirewall_resource_policy;

#[cfg(feature = "data_networkfirewall_resource_policy")]
pub use data_networkfirewall_resource_policy::*;

#[cfg(feature = "data_networkmanager_connection")]
pub mod data_networkmanager_connection;

#[cfg(feature = "data_networkmanager_connection")]
pub use data_networkmanager_connection::*;

#[cfg(feature = "data_networkmanager_connections")]
pub mod data_networkmanager_connections;

#[cfg(feature = "data_networkmanager_connections")]
pub use data_networkmanager_connections::*;

#[cfg(feature = "data_networkmanager_core_network_policy_document")]
pub mod data_networkmanager_core_network_policy_document;

#[cfg(feature = "data_networkmanager_core_network_policy_document")]
pub use data_networkmanager_core_network_policy_document::*;

#[cfg(feature = "data_networkmanager_device")]
pub mod data_networkmanager_device;

#[cfg(feature = "data_networkmanager_device")]
pub use data_networkmanager_device::*;

#[cfg(feature = "data_networkmanager_devices")]
pub mod data_networkmanager_devices;

#[cfg(feature = "data_networkmanager_devices")]
pub use data_networkmanager_devices::*;

#[cfg(feature = "data_networkmanager_global_network")]
pub mod data_networkmanager_global_network;

#[cfg(feature = "data_networkmanager_global_network")]
pub use data_networkmanager_global_network::*;

#[cfg(feature = "data_networkmanager_global_networks")]
pub mod data_networkmanager_global_networks;

#[cfg(feature = "data_networkmanager_global_networks")]
pub use data_networkmanager_global_networks::*;

#[cfg(feature = "data_networkmanager_link")]
pub mod data_networkmanager_link;

#[cfg(feature = "data_networkmanager_link")]
pub use data_networkmanager_link::*;

#[cfg(feature = "data_networkmanager_links")]
pub mod data_networkmanager_links;

#[cfg(feature = "data_networkmanager_links")]
pub use data_networkmanager_links::*;

#[cfg(feature = "data_networkmanager_site")]
pub mod data_networkmanager_site;

#[cfg(feature = "data_networkmanager_site")]
pub use data_networkmanager_site::*;

#[cfg(feature = "data_networkmanager_sites")]
pub mod data_networkmanager_sites;

#[cfg(feature = "data_networkmanager_sites")]
pub use data_networkmanager_sites::*;

#[cfg(feature = "data_oam_link")]
pub mod data_oam_link;

#[cfg(feature = "data_oam_link")]
pub use data_oam_link::*;

#[cfg(feature = "data_oam_links")]
pub mod data_oam_links;

#[cfg(feature = "data_oam_links")]
pub use data_oam_links::*;

#[cfg(feature = "data_oam_sink")]
pub mod data_oam_sink;

#[cfg(feature = "data_oam_sink")]
pub use data_oam_sink::*;

#[cfg(feature = "data_oam_sinks")]
pub mod data_oam_sinks;

#[cfg(feature = "data_oam_sinks")]
pub use data_oam_sinks::*;

#[cfg(feature = "data_odb_cloud_autonomous_vm_cluster")]
pub mod data_odb_cloud_autonomous_vm_cluster;

#[cfg(feature = "data_odb_cloud_autonomous_vm_cluster")]
pub use data_odb_cloud_autonomous_vm_cluster::*;

#[cfg(feature = "data_odb_cloud_autonomous_vm_clusters")]
pub mod data_odb_cloud_autonomous_vm_clusters;

#[cfg(feature = "data_odb_cloud_autonomous_vm_clusters")]
pub use data_odb_cloud_autonomous_vm_clusters::*;

#[cfg(feature = "data_odb_cloud_exadata_infrastructure")]
pub mod data_odb_cloud_exadata_infrastructure;

#[cfg(feature = "data_odb_cloud_exadata_infrastructure")]
pub use data_odb_cloud_exadata_infrastructure::*;

#[cfg(feature = "data_odb_cloud_exadata_infrastructures")]
pub mod data_odb_cloud_exadata_infrastructures;

#[cfg(feature = "data_odb_cloud_exadata_infrastructures")]
pub use data_odb_cloud_exadata_infrastructures::*;

#[cfg(feature = "data_odb_cloud_vm_cluster")]
pub mod data_odb_cloud_vm_cluster;

#[cfg(feature = "data_odb_cloud_vm_cluster")]
pub use data_odb_cloud_vm_cluster::*;

#[cfg(feature = "data_odb_cloud_vm_clusters")]
pub mod data_odb_cloud_vm_clusters;

#[cfg(feature = "data_odb_cloud_vm_clusters")]
pub use data_odb_cloud_vm_clusters::*;

#[cfg(feature = "data_odb_db_node")]
pub mod data_odb_db_node;

#[cfg(feature = "data_odb_db_node")]
pub use data_odb_db_node::*;

#[cfg(feature = "data_odb_db_nodes")]
pub mod data_odb_db_nodes;

#[cfg(feature = "data_odb_db_nodes")]
pub use data_odb_db_nodes::*;

#[cfg(feature = "data_odb_db_server")]
pub mod data_odb_db_server;

#[cfg(feature = "data_odb_db_server")]
pub use data_odb_db_server::*;

#[cfg(feature = "data_odb_db_servers")]
pub mod data_odb_db_servers;

#[cfg(feature = "data_odb_db_servers")]
pub use data_odb_db_servers::*;

#[cfg(feature = "data_odb_db_system_shapes")]
pub mod data_odb_db_system_shapes;

#[cfg(feature = "data_odb_db_system_shapes")]
pub use data_odb_db_system_shapes::*;

#[cfg(feature = "data_odb_gi_versions")]
pub mod data_odb_gi_versions;

#[cfg(feature = "data_odb_gi_versions")]
pub use data_odb_gi_versions::*;

#[cfg(feature = "data_odb_network")]
pub mod data_odb_network;

#[cfg(feature = "data_odb_network")]
pub use data_odb_network::*;

#[cfg(feature = "data_odb_network_peering_connection")]
pub mod data_odb_network_peering_connection;

#[cfg(feature = "data_odb_network_peering_connection")]
pub use data_odb_network_peering_connection::*;

#[cfg(feature = "data_odb_network_peering_connections")]
pub mod data_odb_network_peering_connections;

#[cfg(feature = "data_odb_network_peering_connections")]
pub use data_odb_network_peering_connections::*;

#[cfg(feature = "data_odb_networks")]
pub mod data_odb_networks;

#[cfg(feature = "data_odb_networks")]
pub use data_odb_networks::*;

#[cfg(feature = "data_opensearch_domain")]
pub mod data_opensearch_domain;

#[cfg(feature = "data_opensearch_domain")]
pub use data_opensearch_domain::*;

#[cfg(feature = "data_opensearchserverless_access_policy")]
pub mod data_opensearchserverless_access_policy;

#[cfg(feature = "data_opensearchserverless_access_policy")]
pub use data_opensearchserverless_access_policy::*;

#[cfg(feature = "data_opensearchserverless_collection")]
pub mod data_opensearchserverless_collection;

#[cfg(feature = "data_opensearchserverless_collection")]
pub use data_opensearchserverless_collection::*;

#[cfg(feature = "data_opensearchserverless_lifecycle_policy")]
pub mod data_opensearchserverless_lifecycle_policy;

#[cfg(feature = "data_opensearchserverless_lifecycle_policy")]
pub use data_opensearchserverless_lifecycle_policy::*;

#[cfg(feature = "data_opensearchserverless_security_config")]
pub mod data_opensearchserverless_security_config;

#[cfg(feature = "data_opensearchserverless_security_config")]
pub use data_opensearchserverless_security_config::*;

#[cfg(feature = "data_opensearchserverless_security_policy")]
pub mod data_opensearchserverless_security_policy;

#[cfg(feature = "data_opensearchserverless_security_policy")]
pub use data_opensearchserverless_security_policy::*;

#[cfg(feature = "data_opensearchserverless_vpc_endpoint")]
pub mod data_opensearchserverless_vpc_endpoint;

#[cfg(feature = "data_opensearchserverless_vpc_endpoint")]
pub use data_opensearchserverless_vpc_endpoint::*;

#[cfg(feature = "data_organizations_delegated_administrators")]
pub mod data_organizations_delegated_administrators;

#[cfg(feature = "data_organizations_delegated_administrators")]
pub use data_organizations_delegated_administrators::*;

#[cfg(feature = "data_organizations_delegated_services")]
pub mod data_organizations_delegated_services;

#[cfg(feature = "data_organizations_delegated_services")]
pub use data_organizations_delegated_services::*;

#[cfg(feature = "data_organizations_organization")]
pub mod data_organizations_organization;

#[cfg(feature = "data_organizations_organization")]
pub use data_organizations_organization::*;

#[cfg(feature = "data_organizations_organizational_unit")]
pub mod data_organizations_organizational_unit;

#[cfg(feature = "data_organizations_organizational_unit")]
pub use data_organizations_organizational_unit::*;

#[cfg(feature = "data_organizations_organizational_unit_child_accounts")]
pub mod data_organizations_organizational_unit_child_accounts;

#[cfg(feature = "data_organizations_organizational_unit_child_accounts")]
pub use data_organizations_organizational_unit_child_accounts::*;

#[cfg(feature = "data_organizations_organizational_unit_descendant_accounts")]
pub mod data_organizations_organizational_unit_descendant_accounts;

#[cfg(feature = "data_organizations_organizational_unit_descendant_accounts")]
pub use data_organizations_organizational_unit_descendant_accounts::*;

#[cfg(feature = "data_organizations_organizational_unit_descendant_organizational_units")]
pub mod data_organizations_organizational_unit_descendant_organizational_units;

#[cfg(feature = "data_organizations_organizational_unit_descendant_organizational_units")]
pub use data_organizations_organizational_unit_descendant_organizational_units::*;

#[cfg(feature = "data_organizations_organizational_units")]
pub mod data_organizations_organizational_units;

#[cfg(feature = "data_organizations_organizational_units")]
pub use data_organizations_organizational_units::*;

#[cfg(feature = "data_organizations_policies")]
pub mod data_organizations_policies;

#[cfg(feature = "data_organizations_policies")]
pub use data_organizations_policies::*;

#[cfg(feature = "data_organizations_policies_for_target")]
pub mod data_organizations_policies_for_target;

#[cfg(feature = "data_organizations_policies_for_target")]
pub use data_organizations_policies_for_target::*;

#[cfg(feature = "data_organizations_policy")]
pub mod data_organizations_policy;

#[cfg(feature = "data_organizations_policy")]
pub use data_organizations_policy::*;

#[cfg(feature = "data_organizations_resource_tags")]
pub mod data_organizations_resource_tags;

#[cfg(feature = "data_organizations_resource_tags")]
pub use data_organizations_resource_tags::*;

#[cfg(feature = "data_outposts_asset")]
pub mod data_outposts_asset;

#[cfg(feature = "data_outposts_asset")]
pub use data_outposts_asset::*;

#[cfg(feature = "data_outposts_assets")]
pub mod data_outposts_assets;

#[cfg(feature = "data_outposts_assets")]
pub use data_outposts_assets::*;

#[cfg(feature = "data_outposts_outpost")]
pub mod data_outposts_outpost;

#[cfg(feature = "data_outposts_outpost")]
pub use data_outposts_outpost::*;

#[cfg(feature = "data_outposts_outpost_instance_type")]
pub mod data_outposts_outpost_instance_type;

#[cfg(feature = "data_outposts_outpost_instance_type")]
pub use data_outposts_outpost_instance_type::*;

#[cfg(feature = "data_outposts_outpost_instance_types")]
pub mod data_outposts_outpost_instance_types;

#[cfg(feature = "data_outposts_outpost_instance_types")]
pub use data_outposts_outpost_instance_types::*;

#[cfg(feature = "data_outposts_outposts")]
pub mod data_outposts_outposts;

#[cfg(feature = "data_outposts_outposts")]
pub use data_outposts_outposts::*;

#[cfg(feature = "data_outposts_site")]
pub mod data_outposts_site;

#[cfg(feature = "data_outposts_site")]
pub use data_outposts_site::*;

#[cfg(feature = "data_outposts_sites")]
pub mod data_outposts_sites;

#[cfg(feature = "data_outposts_sites")]
pub use data_outposts_sites::*;

#[cfg(feature = "data_partition")]
pub mod data_partition;

#[cfg(feature = "data_partition")]
pub use data_partition::*;

#[cfg(feature = "data_polly_voices")]
pub mod data_polly_voices;

#[cfg(feature = "data_polly_voices")]
pub use data_polly_voices::*;

#[cfg(feature = "data_prefix_list")]
pub mod data_prefix_list;

#[cfg(feature = "data_prefix_list")]
pub use data_prefix_list::*;

#[cfg(feature = "data_pricing_product")]
pub mod data_pricing_product;

#[cfg(feature = "data_pricing_product")]
pub use data_pricing_product::*;

#[cfg(feature = "data_prometheus_default_scraper_configuration")]
pub mod data_prometheus_default_scraper_configuration;

#[cfg(feature = "data_prometheus_default_scraper_configuration")]
pub use data_prometheus_default_scraper_configuration::*;

#[cfg(feature = "data_prometheus_workspace")]
pub mod data_prometheus_workspace;

#[cfg(feature = "data_prometheus_workspace")]
pub use data_prometheus_workspace::*;

#[cfg(feature = "data_prometheus_workspaces")]
pub mod data_prometheus_workspaces;

#[cfg(feature = "data_prometheus_workspaces")]
pub use data_prometheus_workspaces::*;

#[cfg(feature = "data_qldb_ledger")]
pub mod data_qldb_ledger;

#[cfg(feature = "data_qldb_ledger")]
pub use data_qldb_ledger::*;

#[cfg(feature = "data_ram_resource_share")]
pub mod data_ram_resource_share;

#[cfg(feature = "data_ram_resource_share")]
pub use data_ram_resource_share::*;

#[cfg(feature = "data_rds_certificate")]
pub mod data_rds_certificate;

#[cfg(feature = "data_rds_certificate")]
pub use data_rds_certificate::*;

#[cfg(feature = "data_rds_cluster")]
pub mod data_rds_cluster;

#[cfg(feature = "data_rds_cluster")]
pub use data_rds_cluster::*;

#[cfg(feature = "data_rds_cluster_parameter_group")]
pub mod data_rds_cluster_parameter_group;

#[cfg(feature = "data_rds_cluster_parameter_group")]
pub use data_rds_cluster_parameter_group::*;

#[cfg(feature = "data_rds_clusters")]
pub mod data_rds_clusters;

#[cfg(feature = "data_rds_clusters")]
pub use data_rds_clusters::*;

#[cfg(feature = "data_rds_engine_version")]
pub mod data_rds_engine_version;

#[cfg(feature = "data_rds_engine_version")]
pub use data_rds_engine_version::*;

#[cfg(feature = "data_rds_global_cluster")]
pub mod data_rds_global_cluster;

#[cfg(feature = "data_rds_global_cluster")]
pub use data_rds_global_cluster::*;

#[cfg(feature = "data_rds_orderable_db_instance")]
pub mod data_rds_orderable_db_instance;

#[cfg(feature = "data_rds_orderable_db_instance")]
pub use data_rds_orderable_db_instance::*;

#[cfg(feature = "data_rds_reserved_instance_offering")]
pub mod data_rds_reserved_instance_offering;

#[cfg(feature = "data_rds_reserved_instance_offering")]
pub use data_rds_reserved_instance_offering::*;

#[cfg(feature = "data_redshift_cluster")]
pub mod data_redshift_cluster;

#[cfg(feature = "data_redshift_cluster")]
pub use data_redshift_cluster::*;

#[cfg(feature = "data_redshift_cluster_credentials")]
pub mod data_redshift_cluster_credentials;

#[cfg(feature = "data_redshift_cluster_credentials")]
pub use data_redshift_cluster_credentials::*;

#[cfg(feature = "data_redshift_data_shares")]
pub mod data_redshift_data_shares;

#[cfg(feature = "data_redshift_data_shares")]
pub use data_redshift_data_shares::*;

#[cfg(feature = "data_redshift_orderable_cluster")]
pub mod data_redshift_orderable_cluster;

#[cfg(feature = "data_redshift_orderable_cluster")]
pub use data_redshift_orderable_cluster::*;

#[cfg(feature = "data_redshift_producer_data_shares")]
pub mod data_redshift_producer_data_shares;

#[cfg(feature = "data_redshift_producer_data_shares")]
pub use data_redshift_producer_data_shares::*;

#[cfg(feature = "data_redshift_subnet_group")]
pub mod data_redshift_subnet_group;

#[cfg(feature = "data_redshift_subnet_group")]
pub use data_redshift_subnet_group::*;

#[cfg(feature = "data_redshiftserverless_credentials")]
pub mod data_redshiftserverless_credentials;

#[cfg(feature = "data_redshiftserverless_credentials")]
pub use data_redshiftserverless_credentials::*;

#[cfg(feature = "data_redshiftserverless_namespace")]
pub mod data_redshiftserverless_namespace;

#[cfg(feature = "data_redshiftserverless_namespace")]
pub use data_redshiftserverless_namespace::*;

#[cfg(feature = "data_redshiftserverless_workgroup")]
pub mod data_redshiftserverless_workgroup;

#[cfg(feature = "data_redshiftserverless_workgroup")]
pub use data_redshiftserverless_workgroup::*;

#[cfg(feature = "data_region")]
pub mod data_region;

#[cfg(feature = "data_region")]
pub use data_region::*;

#[cfg(feature = "data_regions")]
pub mod data_regions;

#[cfg(feature = "data_regions")]
pub use data_regions::*;

#[cfg(feature = "data_resourceexplorer2_search")]
pub mod data_resourceexplorer2_search;

#[cfg(feature = "data_resourceexplorer2_search")]
pub use data_resourceexplorer2_search::*;

#[cfg(feature = "data_resourcegroupstaggingapi_resources")]
pub mod data_resourcegroupstaggingapi_resources;

#[cfg(feature = "data_resourcegroupstaggingapi_resources")]
pub use data_resourcegroupstaggingapi_resources::*;

#[cfg(feature = "data_route")]
pub mod data_route;

#[cfg(feature = "data_route")]
pub use data_route::*;

#[cfg(feature = "data_route53_delegation_set")]
pub mod data_route53_delegation_set;

#[cfg(feature = "data_route53_delegation_set")]
pub use data_route53_delegation_set::*;

#[cfg(feature = "data_route53_records")]
pub mod data_route53_records;

#[cfg(feature = "data_route53_records")]
pub use data_route53_records::*;

#[cfg(feature = "data_route53_resolver_endpoint")]
pub mod data_route53_resolver_endpoint;

#[cfg(feature = "data_route53_resolver_endpoint")]
pub use data_route53_resolver_endpoint::*;

#[cfg(feature = "data_route53_resolver_firewall_config")]
pub mod data_route53_resolver_firewall_config;

#[cfg(feature = "data_route53_resolver_firewall_config")]
pub use data_route53_resolver_firewall_config::*;

#[cfg(feature = "data_route53_resolver_firewall_domain_list")]
pub mod data_route53_resolver_firewall_domain_list;

#[cfg(feature = "data_route53_resolver_firewall_domain_list")]
pub use data_route53_resolver_firewall_domain_list::*;

#[cfg(feature = "data_route53_resolver_firewall_rule_group")]
pub mod data_route53_resolver_firewall_rule_group;

#[cfg(feature = "data_route53_resolver_firewall_rule_group")]
pub use data_route53_resolver_firewall_rule_group::*;

#[cfg(feature = "data_route53_resolver_firewall_rule_group_association")]
pub mod data_route53_resolver_firewall_rule_group_association;

#[cfg(feature = "data_route53_resolver_firewall_rule_group_association")]
pub use data_route53_resolver_firewall_rule_group_association::*;

#[cfg(feature = "data_route53_resolver_firewall_rules")]
pub mod data_route53_resolver_firewall_rules;

#[cfg(feature = "data_route53_resolver_firewall_rules")]
pub use data_route53_resolver_firewall_rules::*;

#[cfg(feature = "data_route53_resolver_query_log_config")]
pub mod data_route53_resolver_query_log_config;

#[cfg(feature = "data_route53_resolver_query_log_config")]
pub use data_route53_resolver_query_log_config::*;

#[cfg(feature = "data_route53_resolver_rule")]
pub mod data_route53_resolver_rule;

#[cfg(feature = "data_route53_resolver_rule")]
pub use data_route53_resolver_rule::*;

#[cfg(feature = "data_route53_resolver_rules")]
pub mod data_route53_resolver_rules;

#[cfg(feature = "data_route53_resolver_rules")]
pub use data_route53_resolver_rules::*;

#[cfg(feature = "data_route53_traffic_policy_document")]
pub mod data_route53_traffic_policy_document;

#[cfg(feature = "data_route53_traffic_policy_document")]
pub use data_route53_traffic_policy_document::*;

#[cfg(feature = "data_route53_zone")]
pub mod data_route53_zone;

#[cfg(feature = "data_route53_zone")]
pub use data_route53_zone::*;

#[cfg(feature = "data_route53_zones")]
pub mod data_route53_zones;

#[cfg(feature = "data_route53_zones")]
pub use data_route53_zones::*;

#[cfg(feature = "data_route53profiles_profiles")]
pub mod data_route53profiles_profiles;

#[cfg(feature = "data_route53profiles_profiles")]
pub use data_route53profiles_profiles::*;

#[cfg(feature = "data_route_table")]
pub mod data_route_table;

#[cfg(feature = "data_route_table")]
pub use data_route_table::*;

#[cfg(feature = "data_route_tables")]
pub mod data_route_tables;

#[cfg(feature = "data_route_tables")]
pub use data_route_tables::*;

#[cfg(feature = "data_s3_access_point")]
pub mod data_s3_access_point;

#[cfg(feature = "data_s3_access_point")]
pub use data_s3_access_point::*;

#[cfg(feature = "data_s3_account_public_access_block")]
pub mod data_s3_account_public_access_block;

#[cfg(feature = "data_s3_account_public_access_block")]
pub use data_s3_account_public_access_block::*;

#[cfg(feature = "data_s3_bucket")]
pub mod data_s3_bucket;

#[cfg(feature = "data_s3_bucket")]
pub use data_s3_bucket::*;

#[cfg(feature = "data_s3_bucket_object")]
pub mod data_s3_bucket_object;

#[cfg(feature = "data_s3_bucket_object")]
pub use data_s3_bucket_object::*;

#[cfg(feature = "data_s3_bucket_objects")]
pub mod data_s3_bucket_objects;

#[cfg(feature = "data_s3_bucket_objects")]
pub use data_s3_bucket_objects::*;

#[cfg(feature = "data_s3_bucket_policy")]
pub mod data_s3_bucket_policy;

#[cfg(feature = "data_s3_bucket_policy")]
pub use data_s3_bucket_policy::*;

#[cfg(feature = "data_s3_directory_buckets")]
pub mod data_s3_directory_buckets;

#[cfg(feature = "data_s3_directory_buckets")]
pub use data_s3_directory_buckets::*;

#[cfg(feature = "data_s3_object")]
pub mod data_s3_object;

#[cfg(feature = "data_s3_object")]
pub use data_s3_object::*;

#[cfg(feature = "data_s3_objects")]
pub mod data_s3_objects;

#[cfg(feature = "data_s3_objects")]
pub use data_s3_objects::*;

#[cfg(feature = "data_s3control_multi_region_access_point")]
pub mod data_s3control_multi_region_access_point;

#[cfg(feature = "data_s3control_multi_region_access_point")]
pub use data_s3control_multi_region_access_point::*;

#[cfg(feature = "data_sagemaker_prebuilt_ecr_image")]
pub mod data_sagemaker_prebuilt_ecr_image;

#[cfg(feature = "data_sagemaker_prebuilt_ecr_image")]
pub use data_sagemaker_prebuilt_ecr_image::*;

#[cfg(feature = "data_secretsmanager_random_password")]
pub mod data_secretsmanager_random_password;

#[cfg(feature = "data_secretsmanager_random_password")]
pub use data_secretsmanager_random_password::*;

#[cfg(feature = "data_secretsmanager_secret")]
pub mod data_secretsmanager_secret;

#[cfg(feature = "data_secretsmanager_secret")]
pub use data_secretsmanager_secret::*;

#[cfg(feature = "data_secretsmanager_secret_rotation")]
pub mod data_secretsmanager_secret_rotation;

#[cfg(feature = "data_secretsmanager_secret_rotation")]
pub use data_secretsmanager_secret_rotation::*;

#[cfg(feature = "data_secretsmanager_secret_version")]
pub mod data_secretsmanager_secret_version;

#[cfg(feature = "data_secretsmanager_secret_version")]
pub use data_secretsmanager_secret_version::*;

#[cfg(feature = "data_secretsmanager_secret_versions")]
pub mod data_secretsmanager_secret_versions;

#[cfg(feature = "data_secretsmanager_secret_versions")]
pub use data_secretsmanager_secret_versions::*;

#[cfg(feature = "data_secretsmanager_secrets")]
pub mod data_secretsmanager_secrets;

#[cfg(feature = "data_secretsmanager_secrets")]
pub use data_secretsmanager_secrets::*;

#[cfg(feature = "data_security_group")]
pub mod data_security_group;

#[cfg(feature = "data_security_group")]
pub use data_security_group::*;

#[cfg(feature = "data_security_groups")]
pub mod data_security_groups;

#[cfg(feature = "data_security_groups")]
pub use data_security_groups::*;

#[cfg(feature = "data_securityhub_standards_control_associations")]
pub mod data_securityhub_standards_control_associations;

#[cfg(feature = "data_securityhub_standards_control_associations")]
pub use data_securityhub_standards_control_associations::*;

#[cfg(feature = "data_serverlessapplicationrepository_application")]
pub mod data_serverlessapplicationrepository_application;

#[cfg(feature = "data_serverlessapplicationrepository_application")]
pub use data_serverlessapplicationrepository_application::*;

#[cfg(feature = "data_service")]
pub mod data_service;

#[cfg(feature = "data_service")]
pub use data_service::*;

#[cfg(feature = "data_service_discovery_dns_namespace")]
pub mod data_service_discovery_dns_namespace;

#[cfg(feature = "data_service_discovery_dns_namespace")]
pub use data_service_discovery_dns_namespace::*;

#[cfg(feature = "data_service_discovery_http_namespace")]
pub mod data_service_discovery_http_namespace;

#[cfg(feature = "data_service_discovery_http_namespace")]
pub use data_service_discovery_http_namespace::*;

#[cfg(feature = "data_service_discovery_service")]
pub mod data_service_discovery_service;

#[cfg(feature = "data_service_discovery_service")]
pub use data_service_discovery_service::*;

#[cfg(feature = "data_service_principal")]
pub mod data_service_principal;

#[cfg(feature = "data_service_principal")]
pub use data_service_principal::*;

#[cfg(feature = "data_servicecatalog_constraint")]
pub mod data_servicecatalog_constraint;

#[cfg(feature = "data_servicecatalog_constraint")]
pub use data_servicecatalog_constraint::*;

#[cfg(feature = "data_servicecatalog_launch_paths")]
pub mod data_servicecatalog_launch_paths;

#[cfg(feature = "data_servicecatalog_launch_paths")]
pub use data_servicecatalog_launch_paths::*;

#[cfg(feature = "data_servicecatalog_portfolio")]
pub mod data_servicecatalog_portfolio;

#[cfg(feature = "data_servicecatalog_portfolio")]
pub use data_servicecatalog_portfolio::*;

#[cfg(feature = "data_servicecatalog_portfolio_constraints")]
pub mod data_servicecatalog_portfolio_constraints;

#[cfg(feature = "data_servicecatalog_portfolio_constraints")]
pub use data_servicecatalog_portfolio_constraints::*;

#[cfg(feature = "data_servicecatalog_product")]
pub mod data_servicecatalog_product;

#[cfg(feature = "data_servicecatalog_product")]
pub use data_servicecatalog_product::*;

#[cfg(feature = "data_servicecatalog_provisioning_artifacts")]
pub mod data_servicecatalog_provisioning_artifacts;

#[cfg(feature = "data_servicecatalog_provisioning_artifacts")]
pub use data_servicecatalog_provisioning_artifacts::*;

#[cfg(feature = "data_servicecatalogappregistry_application")]
pub mod data_servicecatalogappregistry_application;

#[cfg(feature = "data_servicecatalogappregistry_application")]
pub use data_servicecatalogappregistry_application::*;

#[cfg(feature = "data_servicecatalogappregistry_attribute_group")]
pub mod data_servicecatalogappregistry_attribute_group;

#[cfg(feature = "data_servicecatalogappregistry_attribute_group")]
pub use data_servicecatalogappregistry_attribute_group::*;

#[cfg(feature = "data_servicecatalogappregistry_attribute_group_associations")]
pub mod data_servicecatalogappregistry_attribute_group_associations;

#[cfg(feature = "data_servicecatalogappregistry_attribute_group_associations")]
pub use data_servicecatalogappregistry_attribute_group_associations::*;

#[cfg(feature = "data_servicequotas_service")]
pub mod data_servicequotas_service;

#[cfg(feature = "data_servicequotas_service")]
pub use data_servicequotas_service::*;

#[cfg(feature = "data_servicequotas_service_quota")]
pub mod data_servicequotas_service_quota;

#[cfg(feature = "data_servicequotas_service_quota")]
pub use data_servicequotas_service_quota::*;

#[cfg(feature = "data_servicequotas_templates")]
pub mod data_servicequotas_templates;

#[cfg(feature = "data_servicequotas_templates")]
pub use data_servicequotas_templates::*;

#[cfg(feature = "data_ses_active_receipt_rule_set")]
pub mod data_ses_active_receipt_rule_set;

#[cfg(feature = "data_ses_active_receipt_rule_set")]
pub use data_ses_active_receipt_rule_set::*;

#[cfg(feature = "data_ses_domain_identity")]
pub mod data_ses_domain_identity;

#[cfg(feature = "data_ses_domain_identity")]
pub use data_ses_domain_identity::*;

#[cfg(feature = "data_ses_email_identity")]
pub mod data_ses_email_identity;

#[cfg(feature = "data_ses_email_identity")]
pub use data_ses_email_identity::*;

#[cfg(feature = "data_sesv2_configuration_set")]
pub mod data_sesv2_configuration_set;

#[cfg(feature = "data_sesv2_configuration_set")]
pub use data_sesv2_configuration_set::*;

#[cfg(feature = "data_sesv2_dedicated_ip_pool")]
pub mod data_sesv2_dedicated_ip_pool;

#[cfg(feature = "data_sesv2_dedicated_ip_pool")]
pub use data_sesv2_dedicated_ip_pool::*;

#[cfg(feature = "data_sesv2_email_identity")]
pub mod data_sesv2_email_identity;

#[cfg(feature = "data_sesv2_email_identity")]
pub use data_sesv2_email_identity::*;

#[cfg(feature = "data_sesv2_email_identity_mail_from_attributes")]
pub mod data_sesv2_email_identity_mail_from_attributes;

#[cfg(feature = "data_sesv2_email_identity_mail_from_attributes")]
pub use data_sesv2_email_identity_mail_from_attributes::*;

#[cfg(feature = "data_sfn_activity")]
pub mod data_sfn_activity;

#[cfg(feature = "data_sfn_activity")]
pub use data_sfn_activity::*;

#[cfg(feature = "data_sfn_alias")]
pub mod data_sfn_alias;

#[cfg(feature = "data_sfn_alias")]
pub use data_sfn_alias::*;

#[cfg(feature = "data_sfn_state_machine")]
pub mod data_sfn_state_machine;

#[cfg(feature = "data_sfn_state_machine")]
pub use data_sfn_state_machine::*;

#[cfg(feature = "data_sfn_state_machine_versions")]
pub mod data_sfn_state_machine_versions;

#[cfg(feature = "data_sfn_state_machine_versions")]
pub use data_sfn_state_machine_versions::*;

#[cfg(feature = "data_shield_protection")]
pub mod data_shield_protection;

#[cfg(feature = "data_shield_protection")]
pub use data_shield_protection::*;

#[cfg(feature = "data_signer_signing_job")]
pub mod data_signer_signing_job;

#[cfg(feature = "data_signer_signing_job")]
pub use data_signer_signing_job::*;

#[cfg(feature = "data_signer_signing_profile")]
pub mod data_signer_signing_profile;

#[cfg(feature = "data_signer_signing_profile")]
pub use data_signer_signing_profile::*;

#[cfg(feature = "data_sns_topic")]
pub mod data_sns_topic;

#[cfg(feature = "data_sns_topic")]
pub use data_sns_topic::*;

#[cfg(feature = "data_spot_datafeed_subscription")]
pub mod data_spot_datafeed_subscription;

#[cfg(feature = "data_spot_datafeed_subscription")]
pub use data_spot_datafeed_subscription::*;

#[cfg(feature = "data_sqs_queue")]
pub mod data_sqs_queue;

#[cfg(feature = "data_sqs_queue")]
pub use data_sqs_queue::*;

#[cfg(feature = "data_sqs_queues")]
pub mod data_sqs_queues;

#[cfg(feature = "data_sqs_queues")]
pub use data_sqs_queues::*;

#[cfg(feature = "data_ssm_document")]
pub mod data_ssm_document;

#[cfg(feature = "data_ssm_document")]
pub use data_ssm_document::*;

#[cfg(feature = "data_ssm_instances")]
pub mod data_ssm_instances;

#[cfg(feature = "data_ssm_instances")]
pub use data_ssm_instances::*;

#[cfg(feature = "data_ssm_maintenance_windows")]
pub mod data_ssm_maintenance_windows;

#[cfg(feature = "data_ssm_maintenance_windows")]
pub use data_ssm_maintenance_windows::*;

#[cfg(feature = "data_ssm_parameter")]
pub mod data_ssm_parameter;

#[cfg(feature = "data_ssm_parameter")]
pub use data_ssm_parameter::*;

#[cfg(feature = "data_ssm_parameters_by_path")]
pub mod data_ssm_parameters_by_path;

#[cfg(feature = "data_ssm_parameters_by_path")]
pub use data_ssm_parameters_by_path::*;

#[cfg(feature = "data_ssm_patch_baseline")]
pub mod data_ssm_patch_baseline;

#[cfg(feature = "data_ssm_patch_baseline")]
pub use data_ssm_patch_baseline::*;

#[cfg(feature = "data_ssm_patch_baselines")]
pub mod data_ssm_patch_baselines;

#[cfg(feature = "data_ssm_patch_baselines")]
pub use data_ssm_patch_baselines::*;

#[cfg(feature = "data_ssmcontacts_contact")]
pub mod data_ssmcontacts_contact;

#[cfg(feature = "data_ssmcontacts_contact")]
pub use data_ssmcontacts_contact::*;

#[cfg(feature = "data_ssmcontacts_contact_channel")]
pub mod data_ssmcontacts_contact_channel;

#[cfg(feature = "data_ssmcontacts_contact_channel")]
pub use data_ssmcontacts_contact_channel::*;

#[cfg(feature = "data_ssmcontacts_plan")]
pub mod data_ssmcontacts_plan;

#[cfg(feature = "data_ssmcontacts_plan")]
pub use data_ssmcontacts_plan::*;

#[cfg(feature = "data_ssmcontacts_rotation")]
pub mod data_ssmcontacts_rotation;

#[cfg(feature = "data_ssmcontacts_rotation")]
pub use data_ssmcontacts_rotation::*;

#[cfg(feature = "data_ssmincidents_replication_set")]
pub mod data_ssmincidents_replication_set;

#[cfg(feature = "data_ssmincidents_replication_set")]
pub use data_ssmincidents_replication_set::*;

#[cfg(feature = "data_ssmincidents_response_plan")]
pub mod data_ssmincidents_response_plan;

#[cfg(feature = "data_ssmincidents_response_plan")]
pub use data_ssmincidents_response_plan::*;

#[cfg(feature = "data_ssoadmin_application")]
pub mod data_ssoadmin_application;

#[cfg(feature = "data_ssoadmin_application")]
pub use data_ssoadmin_application::*;

#[cfg(feature = "data_ssoadmin_application_assignments")]
pub mod data_ssoadmin_application_assignments;

#[cfg(feature = "data_ssoadmin_application_assignments")]
pub use data_ssoadmin_application_assignments::*;

#[cfg(feature = "data_ssoadmin_application_providers")]
pub mod data_ssoadmin_application_providers;

#[cfg(feature = "data_ssoadmin_application_providers")]
pub use data_ssoadmin_application_providers::*;

#[cfg(feature = "data_ssoadmin_instances")]
pub mod data_ssoadmin_instances;

#[cfg(feature = "data_ssoadmin_instances")]
pub use data_ssoadmin_instances::*;

#[cfg(feature = "data_ssoadmin_permission_set")]
pub mod data_ssoadmin_permission_set;

#[cfg(feature = "data_ssoadmin_permission_set")]
pub use data_ssoadmin_permission_set::*;

#[cfg(feature = "data_ssoadmin_permission_sets")]
pub mod data_ssoadmin_permission_sets;

#[cfg(feature = "data_ssoadmin_permission_sets")]
pub use data_ssoadmin_permission_sets::*;

#[cfg(feature = "data_ssoadmin_principal_application_assignments")]
pub mod data_ssoadmin_principal_application_assignments;

#[cfg(feature = "data_ssoadmin_principal_application_assignments")]
pub use data_ssoadmin_principal_application_assignments::*;

#[cfg(feature = "data_storagegateway_local_disk")]
pub mod data_storagegateway_local_disk;

#[cfg(feature = "data_storagegateway_local_disk")]
pub use data_storagegateway_local_disk::*;

#[cfg(feature = "data_subnet")]
pub mod data_subnet;

#[cfg(feature = "data_subnet")]
pub use data_subnet::*;

#[cfg(feature = "data_subnets")]
pub mod data_subnets;

#[cfg(feature = "data_subnets")]
pub use data_subnets::*;

#[cfg(feature = "data_synthetics_runtime_version")]
pub mod data_synthetics_runtime_version;

#[cfg(feature = "data_synthetics_runtime_version")]
pub use data_synthetics_runtime_version::*;

#[cfg(feature = "data_synthetics_runtime_versions")]
pub mod data_synthetics_runtime_versions;

#[cfg(feature = "data_synthetics_runtime_versions")]
pub use data_synthetics_runtime_versions::*;

#[cfg(feature = "data_timestreamwrite_database")]
pub mod data_timestreamwrite_database;

#[cfg(feature = "data_timestreamwrite_database")]
pub use data_timestreamwrite_database::*;

#[cfg(feature = "data_timestreamwrite_table")]
pub mod data_timestreamwrite_table;

#[cfg(feature = "data_timestreamwrite_table")]
pub use data_timestreamwrite_table::*;

#[cfg(feature = "data_transfer_connector")]
pub mod data_transfer_connector;

#[cfg(feature = "data_transfer_connector")]
pub use data_transfer_connector::*;

#[cfg(feature = "data_transfer_server")]
pub mod data_transfer_server;

#[cfg(feature = "data_transfer_server")]
pub use data_transfer_server::*;

#[cfg(feature = "data_verifiedpermissions_policy_store")]
pub mod data_verifiedpermissions_policy_store;

#[cfg(feature = "data_verifiedpermissions_policy_store")]
pub use data_verifiedpermissions_policy_store::*;

#[cfg(feature = "data_vpc")]
pub mod data_vpc;

#[cfg(feature = "data_vpc")]
pub use data_vpc::*;

#[cfg(feature = "data_vpc_dhcp_options")]
pub mod data_vpc_dhcp_options;

#[cfg(feature = "data_vpc_dhcp_options")]
pub use data_vpc_dhcp_options::*;

#[cfg(feature = "data_vpc_endpoint")]
pub mod data_vpc_endpoint;

#[cfg(feature = "data_vpc_endpoint")]
pub use data_vpc_endpoint::*;

#[cfg(feature = "data_vpc_endpoint_associations")]
pub mod data_vpc_endpoint_associations;

#[cfg(feature = "data_vpc_endpoint_associations")]
pub use data_vpc_endpoint_associations::*;

#[cfg(feature = "data_vpc_endpoint_service")]
pub mod data_vpc_endpoint_service;

#[cfg(feature = "data_vpc_endpoint_service")]
pub use data_vpc_endpoint_service::*;

#[cfg(feature = "data_vpc_ipam")]
pub mod data_vpc_ipam;

#[cfg(feature = "data_vpc_ipam")]
pub use data_vpc_ipam::*;

#[cfg(feature = "data_vpc_ipam_pool")]
pub mod data_vpc_ipam_pool;

#[cfg(feature = "data_vpc_ipam_pool")]
pub use data_vpc_ipam_pool::*;

#[cfg(feature = "data_vpc_ipam_pool_cidrs")]
pub mod data_vpc_ipam_pool_cidrs;

#[cfg(feature = "data_vpc_ipam_pool_cidrs")]
pub use data_vpc_ipam_pool_cidrs::*;

#[cfg(feature = "data_vpc_ipam_pools")]
pub mod data_vpc_ipam_pools;

#[cfg(feature = "data_vpc_ipam_pools")]
pub use data_vpc_ipam_pools::*;

#[cfg(feature = "data_vpc_ipam_preview_next_cidr")]
pub mod data_vpc_ipam_preview_next_cidr;

#[cfg(feature = "data_vpc_ipam_preview_next_cidr")]
pub use data_vpc_ipam_preview_next_cidr::*;

#[cfg(feature = "data_vpc_ipams")]
pub mod data_vpc_ipams;

#[cfg(feature = "data_vpc_ipams")]
pub use data_vpc_ipams::*;

#[cfg(feature = "data_vpc_peering_connection")]
pub mod data_vpc_peering_connection;

#[cfg(feature = "data_vpc_peering_connection")]
pub use data_vpc_peering_connection::*;

#[cfg(feature = "data_vpc_peering_connections")]
pub mod data_vpc_peering_connections;

#[cfg(feature = "data_vpc_peering_connections")]
pub use data_vpc_peering_connections::*;

#[cfg(feature = "data_vpc_security_group_rule")]
pub mod data_vpc_security_group_rule;

#[cfg(feature = "data_vpc_security_group_rule")]
pub use data_vpc_security_group_rule::*;

#[cfg(feature = "data_vpc_security_group_rules")]
pub mod data_vpc_security_group_rules;

#[cfg(feature = "data_vpc_security_group_rules")]
pub use data_vpc_security_group_rules::*;

#[cfg(feature = "data_vpclattice_auth_policy")]
pub mod data_vpclattice_auth_policy;

#[cfg(feature = "data_vpclattice_auth_policy")]
pub use data_vpclattice_auth_policy::*;

#[cfg(feature = "data_vpclattice_listener")]
pub mod data_vpclattice_listener;

#[cfg(feature = "data_vpclattice_listener")]
pub use data_vpclattice_listener::*;

#[cfg(feature = "data_vpclattice_resource_policy")]
pub mod data_vpclattice_resource_policy;

#[cfg(feature = "data_vpclattice_resource_policy")]
pub use data_vpclattice_resource_policy::*;

#[cfg(feature = "data_vpclattice_service")]
pub mod data_vpclattice_service;

#[cfg(feature = "data_vpclattice_service")]
pub use data_vpclattice_service::*;

#[cfg(feature = "data_vpclattice_service_network")]
pub mod data_vpclattice_service_network;

#[cfg(feature = "data_vpclattice_service_network")]
pub use data_vpclattice_service_network::*;

#[cfg(feature = "data_vpcs")]
pub mod data_vpcs;

#[cfg(feature = "data_vpcs")]
pub use data_vpcs::*;

#[cfg(feature = "data_vpn_connection")]
pub mod data_vpn_connection;

#[cfg(feature = "data_vpn_connection")]
pub use data_vpn_connection::*;

#[cfg(feature = "data_vpn_gateway")]
pub mod data_vpn_gateway;

#[cfg(feature = "data_vpn_gateway")]
pub use data_vpn_gateway::*;

#[cfg(feature = "data_waf_ipset")]
pub mod data_waf_ipset;

#[cfg(feature = "data_waf_ipset")]
pub use data_waf_ipset::*;

#[cfg(feature = "data_waf_rate_based_rule")]
pub mod data_waf_rate_based_rule;

#[cfg(feature = "data_waf_rate_based_rule")]
pub use data_waf_rate_based_rule::*;

#[cfg(feature = "data_waf_rule")]
pub mod data_waf_rule;

#[cfg(feature = "data_waf_rule")]
pub use data_waf_rule::*;

#[cfg(feature = "data_waf_subscribed_rule_group")]
pub mod data_waf_subscribed_rule_group;

#[cfg(feature = "data_waf_subscribed_rule_group")]
pub use data_waf_subscribed_rule_group::*;

#[cfg(feature = "data_waf_web_acl")]
pub mod data_waf_web_acl;

#[cfg(feature = "data_waf_web_acl")]
pub use data_waf_web_acl::*;

#[cfg(feature = "data_wafregional_ipset")]
pub mod data_wafregional_ipset;

#[cfg(feature = "data_wafregional_ipset")]
pub use data_wafregional_ipset::*;

#[cfg(feature = "data_wafregional_rate_based_rule")]
pub mod data_wafregional_rate_based_rule;

#[cfg(feature = "data_wafregional_rate_based_rule")]
pub use data_wafregional_rate_based_rule::*;

#[cfg(feature = "data_wafregional_rule")]
pub mod data_wafregional_rule;

#[cfg(feature = "data_wafregional_rule")]
pub use data_wafregional_rule::*;

#[cfg(feature = "data_wafregional_subscribed_rule_group")]
pub mod data_wafregional_subscribed_rule_group;

#[cfg(feature = "data_wafregional_subscribed_rule_group")]
pub use data_wafregional_subscribed_rule_group::*;

#[cfg(feature = "data_wafregional_web_acl")]
pub mod data_wafregional_web_acl;

#[cfg(feature = "data_wafregional_web_acl")]
pub use data_wafregional_web_acl::*;

#[cfg(feature = "data_workspaces_bundle")]
pub mod data_workspaces_bundle;

#[cfg(feature = "data_workspaces_bundle")]
pub use data_workspaces_bundle::*;

#[cfg(feature = "data_workspaces_directory")]
pub mod data_workspaces_directory;

#[cfg(feature = "data_workspaces_directory")]
pub use data_workspaces_directory::*;

#[cfg(feature = "data_workspaces_image")]
pub mod data_workspaces_image;

#[cfg(feature = "data_workspaces_image")]
pub use data_workspaces_image::*;

#[cfg(feature = "data_workspaces_workspace")]
pub mod data_workspaces_workspace;

#[cfg(feature = "data_workspaces_workspace")]
pub use data_workspaces_workspace::*;
